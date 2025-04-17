use metrics_exporter_prometheus::PrometheusHandle;

use crate::common_types::{EventSender, EventValidators, TelemetryMap};

#[derive(Debug, Clone)]
pub struct AppState {
    pub telemetry_map: TelemetryMap,
    pub sender: EventSender,
    pub validators: EventValidators,
    pub prometheus_handle: PrometheusHandle,
}

impl AppState {
    pub fn new(
        sender: EventSender,
        telemetry_map: TelemetryMap,
        validators: EventValidators,
        prometheus_handle: PrometheusHandle,
    ) -> Self {
        AppState { telemetry_map, sender, validators, prometheus_handle }
    }
}
