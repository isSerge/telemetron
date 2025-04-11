use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::{error::Error, event::Event};

async fn ingest_handler(body: Json<Event>) -> impl IntoResponse {
    log::info!("Ingesting data: {:?}", body);

    // TODO: implement the ingestion logic
    // TODO: validate the event
    // TODO: store the event
    // TODO: return a 200 response
    // TODO: return a 400 response if the event is invalid
    // TODO: return a 500 response if the event is valid but the ingestion fails

    "Ingested data"
}

async fn stats_handler() -> impl IntoResponse {
    log::info!("Stats");

    "Stats"
}

async fn stats_by_source_id_handler() -> impl IntoResponse {
    log::info!("Stats by source id");

    "Stats by source id"
}

async fn not_found_handler() -> impl IntoResponse {
    log::info!("Not found");

    "Not found"
}

pub async fn run_server() -> Result<(), Error> {
    log::info!("Starting Telemetron");

    let routes = Router::new()
        .route("/ingest", post(ingest_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/{source_id}", get(stats_by_source_id_handler))
        .fallback(not_found_handler);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    log::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
