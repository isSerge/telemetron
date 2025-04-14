#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

mod common_types;
mod config;
mod error;
mod event;
mod processing;
mod processor;
mod server;
mod state;
mod validation;

use std::{error::Error, sync::Arc};

use config::Config;
use server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt().init();

    // Load the configuration
    let config = match Config::try_load() {
        Ok(config) => {
            tracing::info!("Config loaded successfully");
            config
        }
        Err(err) => {
            tracing::error!("Failed to load config: {}", err);
            std::process::exit(1);
        }
    };

    let config = Arc::new(config);

    if let Err(err) = run_server(config).await {
        tracing::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
