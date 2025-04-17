use metrics::{Unit, describe_counter, describe_histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics() -> PrometheusHandle {
    PrometheusBuilder::new().install_recorder().expect("Failed to install Prometheus recorder") // Use expect here as it's critical for startup
}

/// Call this function once at startup to describe metrics to the exporter.
pub fn describe_metrics() {
    describe_counter!(HTTP_REQUESTS_TOTAL, Unit::Count, "Total number of HTTP requests received");
    // Optional: Add default buckets or leave to exporter defaults
    describe_histogram!(HTTP_REQUESTS_DURATION_SECONDS, Unit::Seconds, "HTTP request latency");
    describe_counter!(
        EVENTS_SENT_TO_CHANNEL_TOTAL,
        Unit::Count,
        "Total number of events successfully sent to the processing channel"
    );
    describe_counter!(
        PROCESSOR_RECEIVED_EVENTS_TOTAL,
        Unit::Count,
        "Total number of events received from the channel by the processor"
    );
    describe_counter!(
        PROCESSOR_PROCESSED_BATCHES_TOTAL,
        Unit::Count,
        "Total number of batches processed by event processor plugins"
    );
    describe_counter!(
        PROCESSOR_PROCESSED_EVENTS_TOTAL,
        Unit::Count,
        "Total number of events processed by event processor plugins"
    );
    describe_counter!(
        PROCESSOR_ERRORS_TOTAL,
        Unit::Count,
        "Total number of errors encountered during event processing by plugins (before DLQ)"
    );
    describe_histogram!(
        PROCESSOR_BATCH_DURATION_SECONDS,
        Unit::Seconds,
        "Duration taken to process a batch of events by a plugin"
    );
    describe_counter!(
        DLQ_EVENTS_TOTAL,
        Unit::Count,
        "Total number of events/batches logged to DLQ after retries"
    );
    describe_counter!(
        STORAGE_EVENTS_UPDATED_TOTAL,
        Unit::Count,
        "Total number of events that updated existing telemetry in StorageProcessor"
    );
    describe_counter!(
        STORAGE_SOURCES_CREATED_TOTAL,
        Unit::Count,
        "Total number of new sources created in StorageProcessor"
    );
}

// -------- HTTP Server Metrics --------
pub const HTTP_REQUESTS_TOTAL: &str = "telemetron_http_requests_total";
pub const HTTP_REQUESTS_DURATION_SECONDS: &str = "telemetron_http_requests_duration_seconds";

// -------- Channel Metrics --------
pub const EVENTS_SENT_TO_CHANNEL_TOTAL: &str = "telemetron_events_sent_to_channel_total";

// -------- Processor Metrics --------
pub const PROCESSOR_RECEIVED_EVENTS_TOTAL: &str = "telemetron_processor_received_events_total";
pub const PROCESSOR_PROCESSED_BATCHES_TOTAL: &str = "telemetron_processor_processed_batches_total";
pub const PROCESSOR_PROCESSED_EVENTS_TOTAL: &str = "telemetron_processor_processed_events_total";
pub const PROCESSOR_ERRORS_TOTAL: &str = "telemetron_processor_errors_total";
pub const PROCESSOR_BATCH_DURATION_SECONDS: &str = "telemetron_processor_batch_duration_seconds";
pub const DLQ_EVENTS_TOTAL: &str = "telemetron_dlq_events_total";

// -------- Storage Plugin Specific --------
pub const STORAGE_EVENTS_UPDATED_TOTAL: &str = "telemetron_storage_events_updated_total";
pub const STORAGE_SOURCES_CREATED_TOTAL: &str = "telemetron_storage_sources_created_total";
