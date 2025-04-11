#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]

//! TODO: Add a description

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    if let Err(err) = run().await {
        log::error!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    log::info!("Starting Telemetron");
    Ok(())
}
