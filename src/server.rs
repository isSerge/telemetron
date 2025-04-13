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

use crate::{config::Config, error::Error, event::Event, processor::Processor, state::AppState};

async fn ingest_handler(
    State(state): State<AppState>,
    event: Json<Event>,
) -> Result<impl IntoResponse, Error> {
    log::info!("Ingesting data: {:?}", event);

    // TODO: validate the event
    // TODO: return a 400 response if the event is invalid

    match state.sender.send(event.0).await {
        Ok(_) => {
            log::info!("Event sent to channel");
            Ok((StatusCode::ACCEPTED, "Success"))
        }
        Err(err) => {
            log::error!("Failed to send event to channel: {}", err);
            Err(Error::InternalServerError("Failed to send event to channel".into()))
        }
    }
}

async fn stats_handler(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    log::info!("Stats");

    let sources_count = state.events_map.len();
    let events_count: usize = state.events_map.iter().map(|x| x.len()).sum();

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
    log::info!("Stats by source id: {}", source_id);

    let events_count = state.events_map.get(&source_id).map_or(0, |v| v.len());
    let stats = Json(serde_json::json!({
        "source_id": source_id,
        "events_count": events_count,
    }));

    Ok(stats)
}

async fn not_found_handler() -> impl IntoResponse {
    log::info!("Not found");

    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn run_server(config: Config) -> Result<(), Error> {
    log::info!("Starting Telemetron");

    // Create a channel for sending events
    let (sender, receiver) = mpsc::channel::<Event>(config.channel_capacity);

    // Create a map to store events by source id
    let events_map = Arc::new(DashMap::new());
    // Initialize the application state
    let app_state = AppState::new(sender, events_map.clone());

    // Spawn the processor
    tokio::spawn(async move {
        let mut processor = Processor::new(receiver, events_map);
        processor.run().await;
    });

    let routes = Router::new()
        .route("/ingest", post(ingest_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/{source_id}", get(stats_by_source_id_handler))
        .fallback(not_found_handler)
        .with_state(app_state);

    let listener = TcpListener::bind(format!("{}:{}", config.http_host, config.http_port)).await?;

    log::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
