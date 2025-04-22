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

## API Endpoints

*   **`POST /ingest`**
    *   **Description:** Submits a single telemetry event for processing.
    *   **Request Body:** JSON object representing an `Event`.
        ```json
        {
          "sourceId": 123,
          "type": "Heartbeat",
          "timestamp": "2023-10-27T10:00:00Z",
          "data": { "key": "value" } // Optional data payload
        }
        ```
    *   **Responses:**
        *   `202 Accepted`: Event was successfully validated and queued for processing.
        *   `400 Bad Request`: Event failed validation (invalid format, disallowed source ID/type). Error details in JSON body.
        *   `500 Internal Server Error`: Server-side error occurred.
*   **`GET /stats`**
    *   **Description:** Returns aggregated statistics across all sources.
    *   **Response Body:** JSON object.
        ```json
        {
          "sources_count": 5,
          "events_count": 1053
        }
        ```
*   **`GET /stats/{source_id}`**
    *   **Description:** Returns detailed statistics for a specific `source_id`.
    *   **URL Parameter:** `source_id` (u64).
    *   **Responses:**
        *   `200 OK`: JSON object with stats for the source.
            ```json
            {
              "source_id": 123,
              "total_events": 55,
              "first_event": "2023-10-27T09:30:00Z",
              "last_event": "2023-10-27T10:00:00Z",
              "event_types": {
                "Heartbeat": 50,
                "Login": 5
              }
            }
            ```
*   **`GET /metrics`**
    *   **Description:** Exposes application metrics in Prometheus/OpenMetrics format.
    *   **Response Body:** Text-based metrics scrape data.
*   **`GET /healthz`**
    *   **Description:** Simple liveness check endpoint.
    *   **Responses:**
        *   `200 OK`: Server is running and responding. Body: `OK`.

## Metrics

Key metrics exposed via `/metrics`:

*   `telemetron_http_requests_total`: Counter of HTTP requests (labels: `endpoint`).
*   `telemetron_http_requests_duration_seconds`: Histogram of HTTP request latency (labels: `endpoint`, `status`).
*   `telemetron_processor_plugin_errors_total`: Counter of permanent errors per processor plugin (label: `plugin`).
*   `telemetron_processor_plugin_duration_seconds`: Histogram of plugin batch processing time (labels: `plugin`, `status`).
*   `telemetron_events_processed_total`: Counter of events successfully processed by all plugins.

## Plugins

Telemetron uses a plugin system for validation and processing, discovered at startup using `inventory` crate.

*   **Validators (`src/validation/mod.rs::EventValidator`)**: Implement the `validate` method. Return `Ok(())` if valid, or `Err(EventValidationError)` if invalid.
*   **Processors (`src/processing/mod.rs::EventProcessor`)**: Implement the `process_event` method to handle batches of events. Return `Ok(())` on success or `Err(ProcessingError)` on failure.

**Adding a New Plugin:**
1.  Implement the appropriate trait (`EventValidator` or `EventProcessor`).
2.  Create a constructor function (`fn(toml::Value) -> Result<Box<dyn Trait...>, PluginError>`) that takes TOML parameters and returns an instance of your plugin.
3.  Register the plugin using `inventory::submit!`:
    ```rust
    // In your plugin's module (e.g., src/validation/my_validator.rs)
    inventory::submit! {
      ValidationPluginFactory { // Or ProcessingPluginFactory
            name: "MyValidator", // Name used in config.toml
            constructor: construct_my_validator,
        }
    }
    ```
4.  Add configuration for your plugin under the relevant section (`[validation.plugins]` or `[processing.plugins]`) in `config.toml`.

See existing plugins ([`src/validation/source_id.rs`](src/validation/source_id.rs), [`src/processing/storage.rs`](src/processing/storage.rs)) for examples.

## Testing

Run the test suite using Cargo:

```bash
cargo test
# Run with output capture disabled to see logs/println
cargo test -- --nocapture
# Enable specific tracing logs during tests
RUST_LOG=telemetron::processing=trace cargo test -- --nocapture
```

