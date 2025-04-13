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
    // Initialize the logger
    env_logger::init();

    let config = Config::new();

    if let Err(err) = run_server(config).await {
        log::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
