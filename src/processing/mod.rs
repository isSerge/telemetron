pub mod error;
pub mod source_telemetry;
pub mod storage;

use error::ProcessingError;

use crate::{common_types::TelemetryMap, event::Event};

#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync {
    /// Process an event.
    async fn process_event(
        &self,
        telemetry_map: &TelemetryMap,
        events: &[Event],
    ) -> Result<(), ProcessingError>;

    /// Processor name (for logging purposes).
    fn name(&self) -> &'static str;
}
