use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::{error::Error, event::Event, state::AppState};

async fn ingest_handler(
    State(state): State<AppState>,
    body: Json<Event>,
) -> Result<impl IntoResponse, Error> {
    log::info!("Ingesting data: {:?}", body);

    // TODO: implement the ingestion logic
    // TODO: validate the event
    // TODO: store the event
    // TODO: return a 200 response
    // TODO: return a 400 response if the event is invalid
    // TODO: return a 500 response if the event is valid but the ingestion fails

    Ok((StatusCode::CREATED, "Success"))
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

    // Initialize the application state
    let app_state = AppState::new();

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
