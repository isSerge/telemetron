use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use dashmap::DashMap;
use tokio::{net::TcpListener, sync::mpsc};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    common_types::{EventProcessors, EventValidators},
    config::Config,
    error::Error,
    event::Event,
    processor::EventProcessorManager,
    state::AppState,
};

#[tracing::instrument(skip(state), fields(source_id = event.source_id))]
async fn ingest_handler(
    State(state): State<AppState>,
    event: Json<Event>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Ingest request");

    let event = event.0;

    for validator in state.validators.iter() {
        tracing::info!("Validating event with {}", validator.name());
        if let Err(err) = validator.validate(&event) {
            tracing::warn!("Event validation failed: {}", err);
            return Err(Error::InvalidEvent(err));
        }
    }
    tracing::info!("Event validated successfully");

    match state.sender.send(event).await {
        Ok(_) => {
            tracing::info!("Event sent to channel");
            Ok((StatusCode::ACCEPTED, "Success"))
        }
        Err(err) => {
            tracing::error!("Failed to send event to channel: {}", err);
            Err(Error::InternalServerError("Failed to send event to channel".into()))
        }
    }
}

async fn stats_handler(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    tracing::info!("Stats");

    let sources_count = state.telemetry_map.len();
    let events_count: u64 =
        state.telemetry_map.iter().map(|entry| entry.value().total_events).sum();

    // TODO: add more stats

    let stats = Json(serde_json::json!({
        "sources_count": sources_count,
        "events_count": events_count,
    }));

    Ok(stats)
}

async fn stats_by_source_id_handler(
    State(state): State<AppState>,
    Path(source_id): Path<u64>,
) -> Result<impl IntoResponse, Error> {
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
            Ok(stats)
        }
        None => {
            tracing::warn!("Source id {} not found", source_id);
            Err(Error::NotFound(format!("Source id {} not found", source_id)))
        }
    }
}

async fn not_found_handler() -> impl IntoResponse {
    tracing::info!("Not found");

    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn run_server(
    config: Arc<Config>,
    validators: EventValidators,
    processors: EventProcessors,
) -> Result<(), Error> {
    tracing::info!("Starting Telemetron");

    // Create a channel for sending events
    let (sender, receiver) = mpsc::channel::<Event>(config.processor.channel_capacity);

    // Create a map to store events by source id
    let telemetry_map = Arc::new(DashMap::new());

    // Initialize the application state
    let app_state = AppState::new(sender, telemetry_map.clone(), validators);

    // Create another config clone - to be moved into the processor
    let config_clone = config.clone();
    // Spawn the processor
    tokio::spawn(async move {
        let processor = EventProcessorManager::new(telemetry_map, processors, config_clone);
        processor.run(receiver).await;
    });

    let routes = Router::new()
        .route("/ingest", post(ingest_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/{source_id}", get(stats_by_source_id_handler))
        .fallback(not_found_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = TcpListener::bind(format!("{}:{}", config.http.host, config.http.port)).await?;

    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
