#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

mod common_types;
mod config;
mod error;
mod event;
mod metrics;
mod plugins;
mod processing;
mod processor;
mod server;
mod state;
mod validation;

use std::{error::Error, sync::Arc};

use config::Config;
use server::run_server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(format!("{}={}", module_path!(), "info")))
                .add_directive(format!("dlq_log={}", "error").parse()?),
        )
        .init();

    // Setup metrics
    let prometheus_handle = metrics::setup_metrics();
    metrics::describe_metrics();
    tracing::info!("Prometheus recorder installed.");

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

    // Build validator plugins
    let validators = match plugins::build_validators(&config) {
        Ok(validators) => {
            tracing::info!("{} validators loaded successfully", validators.len());
            validators
        }
        Err(err) => {
            tracing::error!("Failed to load validators: {}", err);
            std::process::exit(1);
        }
    };

    // Build processor plugins
    let processors = match plugins::build_processors(&config) {
        Ok(processors) => {
            tracing::info!("{} processors loaded successfully", processors.len());
            processors
        }
        Err(err) => {
            tracing::error!("Failed to load processors: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = run_server(config, validators, processors, prometheus_handle).await {
        tracing::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
