pub mod storage;

use crate::{common_types::EventsMap, event::Event};

// TODO: add more specific errors
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Failed to store event: {0}")]
    EventProcessingError(String),
}

#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync {
    /// Process an event.
    async fn process_event(
        &self,
        events_map: &EventsMap,
        events: &[Event],
    ) -> Result<(), ProcessingError>;

    /// Processor name (for logging purposes).
    fn name(&self) -> &'static str;
}
