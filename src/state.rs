use crate::common_types::{EventSender, EventValidators, TelemetryMap};

#[derive(Debug, Clone)]
pub struct AppState {
    pub telemetry_map: TelemetryMap,
    pub sender: EventSender,
    pub validators: EventValidators,
}

impl AppState {
    pub fn new(
        sender: EventSender,
        telemetry_map: TelemetryMap,
        validators: EventValidators,
    ) -> Self {
        AppState { telemetry_map, sender, validators }
    }
}
