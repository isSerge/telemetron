use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::{net::TcpListener, sync::mpsc};

use crate::{error::Error, event::Event, state::AppState};

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

    Ok((StatusCode::OK, "Stats"))
}

async fn stats_by_source_id_handler(
    State(state): State<AppState>,
    Path(source_id): Path<u64>,
) -> Result<impl IntoResponse, Error> {
    log::info!("Stats by source id: {}", source_id);

    Ok((StatusCode::OK, format!("Stats by source id: {}", source_id)))
}

async fn not_found_handler() -> impl IntoResponse {
    log::info!("Not found");

    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn run_server(host: &str, port: u16) -> Result<(), Error> {
    log::info!("Starting Telemetron");

    // TODO: move to config
    const CAPACITY: usize = 100;

    // Create a channel for sending events
    let (sender, _) = mpsc::channel::<Event>(CAPACITY);
    // Initialize the application state
    let app_state = AppState::new(sender);

    // TODO: spawn a task for event processor (using channel receiver)

    let routes = Router::new()
        .route("/ingest", post(ingest_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/{source_id}", get(stats_by_source_id_handler))
        .fallback(not_found_handler)
        .with_state(app_state);

    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    log::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
