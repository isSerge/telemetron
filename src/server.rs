use std::{sync::Arc, time::Instant};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use dashmap::DashMap;
use metrics_exporter_prometheus::PrometheusHandle;
use tokio::{net::TcpListener, sync::mpsc};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    common_types::{EventProcessors, EventValidators},
    config::Config,
    error::Error,
    event::Event,
    metrics as metrics_consts,
    processor::EventProcessorManager,
    state::AppState,
};

#[tracing::instrument(skip(state), fields(source_id = event.source_id))]
async fn ingest_handler(
    State(state): State<AppState>,
    event: Json<Event>,
) -> Result<impl IntoResponse, Error> {
    let start = Instant::now();
    tracing::info!("Ingest request");

    // Increment the total events counter
    metrics::counter!(metrics_consts::HTTP_REQUESTS_TOTAL, "endpoint" => "/ingest").increment(1);
    let event = event.0;

    for validator in state.validators.iter() {
        tracing::info!("Validating event with {}", validator.name());
        if let Err(err) = validator.validate(&event) {
            tracing::warn!("Event validation failed: {}", err);
            metrics::histogram!(metrics_consts::HTTP_REQUESTS_DURATION_SECONDS, "endpoint" => "/ingest", "status" => "4xx").record(start.elapsed());
            return Err(Error::InvalidEvent(err));
        }
    }
    tracing::info!("Event validated successfully");

    match state.sender.send(event).await {
        Ok(_) => {
            tracing::info!("Event sent to channel");
            metrics::histogram!(metrics_consts::HTTP_REQUESTS_DURATION_SECONDS, "endpoint" => "/ingest", "status" => "2xx").record(start.elapsed());
            Ok((StatusCode::ACCEPTED, "Success"))
        }
        Err(err) => {
            tracing::error!("Failed to send event to channel: {}", err);
            metrics::histogram!(metrics_consts::HTTP_REQUESTS_DURATION_SECONDS, "endpoint" => "/ingest", "status" => "5xx").record(start.elapsed());
            Err(Error::InternalServerError("Failed to send event to channel".into()))
        }
    }
}

async fn stats_handler(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let start = Instant::now();
    metrics::counter!(metrics_consts::HTTP_REQUESTS_TOTAL, "endpoint" => "/stats").increment(1);

    tracing::info!("Stats");

    let sources_count = state.telemetry_map.len();
    let events_count: u64 =
        state.telemetry_map.iter().map(|entry| entry.value().total_events).sum();

    // TODO: add more stats
    let stats = Json(serde_json::json!({
        "sources_count": sources_count,
        "events_count": events_count,
    }));

    metrics::histogram!(metrics_consts::HTTP_REQUESTS_DURATION_SECONDS, "endpoint" => "/stats", "status" => "2xx").record(start.elapsed());

    Ok(stats)
}

async fn stats_by_source_id_handler(
    State(state): State<AppState>,
    Path(source_id): Path<u64>,
) -> Result<impl IntoResponse, Error> {
    let start = Instant::now();
    metrics::counter!(metrics_consts::HTTP_REQUESTS_TOTAL, "endpoint" => "/stats/{source_id}")
        .increment(1);
    tracing::info!("Stats by source id: {}", source_id);

    let entry = state.telemetry_map.get(&source_id);

    match entry {
        Some(entry) => {
            let telemetry = entry.value();
            let stats = Json(serde_json::json!({
                "source_id": source_id,
                "total_events": telemetry.total_events,
                "first_event": telemetry.first_timestamp,
                "last_event": telemetry.last_timestamp,
                "event_types": telemetry.events_by_type,
            }));
            metrics::histogram!(
                metrics_consts::HTTP_REQUESTS_DURATION_SECONDS,
                "endpoint" => "/stats/{source_id}",
                "status" => "2xx"
            )
            .record(start.elapsed());
            Ok(stats)
        }
        None => {
            tracing::warn!("Source id {} not found", source_id);
            metrics::histogram!(
                metrics_consts::HTTP_REQUESTS_DURATION_SECONDS,
                "endpoint" => "/stats/{source_id}",
                "status" => "4xx"
            )
            .record(start.elapsed());
            Err(Error::NotFound(format!("Source id {} not found", source_id)))
        }
    }
}

async fn not_found_handler() -> impl IntoResponse {
    metrics::counter!(metrics_consts::HTTP_REQUESTS_TOTAL, "endpoint" => "/404").increment(1);
    tracing::info!("Not found");

    (StatusCode::NOT_FOUND, "Not found")
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    metrics::counter!(metrics_consts::HTTP_REQUESTS_TOTAL, "endpoint" => "/metrics").increment(1);

    let body = state.prometheus_handle.render();
    let headers = [(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/openmetrics-text; version=1.0.0; charset=utf-8"),
    )];
    (StatusCode::OK, headers, body)
}

async fn wait_for_shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C signal handler");
        tracing::info!("Ctrl+C received, shutting down...");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
        tracing::info!("SIGTERM received, shutting down...");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending(); // No-op for non-Unix systems

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Termination signal received, shutting down...");
}

pub async fn run_server(
    config: Arc<Config>,
    validators: EventValidators,
    processors: EventProcessors,
    prometheus_handle: PrometheusHandle,
) -> Result<(), Error> {
    tracing::info!("Starting Telemetron");

    // Create a channel for sending events
    let (sender, receiver) = mpsc::channel::<Event>(config.processor.channel_capacity);

    // Create a map to store events by source id
    let telemetry_map = Arc::new(DashMap::new());

    // Initialize the application state
    let app_state =
        AppState::new(sender.clone(), telemetry_map.clone(), validators, prometheus_handle);

    // Create another config clone - to be moved into the processor
    let config_clone = config.clone();
    // Spawn the processor
    let processor_handle = tokio::spawn(async move {
        let processor = EventProcessorManager::new(telemetry_map, processors, config_clone);
        processor.run(receiver).await;
    });

    let routes = Router::new()
        .route("/ingest", post(ingest_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/{source_id}", get(stats_by_source_id_handler))
        .route("/metrics", get(metrics_handler))
        .fallback(not_found_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = TcpListener::bind(format!("{}:{}", config.http.host, config.http.port)).await?;

    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(wait_for_shutdown())
        .await?;

    // Close the sender channel
    tracing::info!("Closing event sender channel");
    drop(sender);

    // Wait for the processor to finish
    if let Err(err) = processor_handle.await {
        tracing::error!("Processor task failed: {}", err);
        return Err(Error::InternalServerError("Processor task failed".into()));
    }
    tracing::info!("Processor task finished successfully");
    tracing::info!("Telemetron shutdown complete");

    Ok(())
}
