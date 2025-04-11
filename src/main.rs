#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

mod config;
mod error;
mod event;
mod http;
mod state;

use std::error::Error;

use config::Config;
use http::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    let config = Config::new();

    if let Err(err) = run_server(&config.http_host, config.http_port).await {
        log::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
