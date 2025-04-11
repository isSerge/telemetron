#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

use std::error::Error;

use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::Value;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    if let Err(err) = run_server().await {
        log::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    source_id: u64,
    // TODO: should be an enum
    r#type: String,
    // TODO: should be a timestamp
    timestamp: String,
    data: Option<Value>,
}

async fn ingest_handler(body: Json<Event>) -> impl IntoResponse {
    log::info!("Ingesting data: {:?}", body);

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

async fn run_server() -> Result<(), Box<dyn Error>> {
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
