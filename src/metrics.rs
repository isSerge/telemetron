use metrics::{Unit, describe_counter, describe_histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics() -> PrometheusHandle {
    PrometheusBuilder::new().install_recorder().expect("Failed to install Prometheus recorder") // Use expect here as it's critical for startup
}

// -------- HTTP Server Metrics --------
pub const HTTP_REQUESTS_TOTAL: &str = "telemetron_http_requests_total";
pub const HTTP_REQUESTS_DURATION_SECONDS: &str = "telemetron_http_requests_duration_seconds";

// -------- Processor Metrics --------
// Renamed for clarity: focuses on plugin errors leading to DLQ
pub const PROCESSOR_PLUGIN_ERRORS_TOTAL: &str = "telemetron_processor_plugin_errors_total";
// Renamed for clarity: focuses on plugin execution time
pub const PROCESSOR_PLUGIN_DURATION_SECONDS: &str = "telemetron_processor_plugin_duration_seconds";
// Overall successful event count
pub const EVENTS_PROCESSED_TOTAL: &str = "telemetron_events_processed_total";

/// Call this function once at startup to describe metrics to the exporter.
pub fn describe_metrics() {
    // --- HTTP ---
    describe_counter!(
        HTTP_REQUESTS_TOTAL,
        Unit::Count,
        "Total number of HTTP requests received, partitioned by endpoint."
    );
    describe_histogram!(
        HTTP_REQUESTS_DURATION_SECONDS,
        Unit::Seconds,
        "HTTP request latency, partitioned by endpoint and status code."
    );

    // --- Processor ---
    describe_counter!(
        PROCESSOR_PLUGIN_ERRORS_TOTAL,
        Unit::Count,
        "Total number of permanent processing errors for each plugin (leading to DLQ)."
    );
    describe_histogram!(
        PROCESSOR_PLUGIN_DURATION_SECONDS,
        Unit::Seconds,
        "Duration taken by each plugin to process an event batch, partitioned by plugin and \
         status (success/error)."
    );
    describe_counter!(
        EVENTS_PROCESSED_TOTAL,
        Unit::Count,
        "Total number of events successfully processed by all plugins in the pipeline."
    );
}
