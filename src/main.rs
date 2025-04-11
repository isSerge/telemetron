#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

mod error;
mod event;
mod http;

use std::error::Error;

use http::run_server;

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
