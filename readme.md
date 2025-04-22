# Telemetron

Telemetron is a lightweight, extensible telemetry event ingestion and processing service written in Rust. It's designed to receive events via HTTP, validate them using a configurable plugin pipeline, and process them asynchronously using another set of processing plugins.

Default event processing focuses on in-memory aggregation of basic statistics per event source, but the plugin architecture allows for easy extension to support additional validation rules, storage options, alerts, etc.

## Features

*   **HTTP API:** Simple endpoints for event ingestion (`/ingest`), aggregated statistics (`/stats`, `/stats/{source_id}`).
*   **Plugin Architecture:**
    *   **Validators:** Chainable plugins to validate incoming events before processing (e.g., by Source ID or by Event Type).
    *   **Processors:** Chainable plugins to process batches of validated events asynchronously (e.g., In-memory statistics aggregation).
    *   Uses the `inventory` crate for automatic plugin discovery.
*   **Asynchronous Processing:** Uses Tokio and MPSC channels for non-blocking event handling and processing.
*   **Configurable:**
    *   Uses a `config.toml` file for server, processor, and plugin configuration.
*   **Extensible Event Types:** Supports core, predefined event types (`Heartbeat`) and custom, string-based types (`Custom(String)`).
*   **Observability:**
    *   Structured logging via `tracing`.
    *   Prometheus metrics exposed on `/metrics`.
    *   Liveness health check endpoint `/healthz`.
*   **Error Handling:** Defined error types and DLQ (Dead Letter Queue) logging for processing failures.

## Prerequisites

*   **Rust:** Install via [rustup](https://rustup.rs/).

## Getting Started

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/isserge/telemetron.git
    cd telemetron
    ```
2.  **Build the project:**
    ```bash
    # For development
    cargo build
    # For release
    cargo build --release
    ```

## Configuration

Telemetron requires config file in the directory where it's run. You can use existing [`config.toml`](config.toml) for reference.

### Environment Variable Overrides:

Configuration values can be overridden using environment variables with the prefix `TELEMETRON_` and `__` as a separator for nested values. For example:

`TELEMETRON_HTTP__PORT=8081` overrides `http.port`.

`TELEMETRON_PROCESSOR__BATCH_SIZE=50` overrides `processor.batch_size`.

## Running the Application

1. Ensure config.toml is present in the current directory.
2. Run the compiled binary:
```
./target/release/telemetron
# Or for development builds:
# ./target/debug/telemetron
```
3. Control logging verbosity with the RUST_LOG environment variable:
```
# Example: Show info logs for telemetron, debug for storage plugin
RUST_LOG=telemetron=info,telemetron::processing::storage=debug ./target/release/telemetron
```
