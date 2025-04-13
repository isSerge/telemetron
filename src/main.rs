#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

mod common_types;
mod config;
mod error;
mod event;
mod processor;
mod server;
mod state;

use std::error::Error;

use config::Config;
use server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt().init();

    let config = Config::new();

    tracing::info!("Config: {:?}", config);

    if let Err(err) = run_server(config).await {
        tracing::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
