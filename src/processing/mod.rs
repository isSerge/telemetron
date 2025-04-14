pub mod storage;

use crate::event::Event;

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Failed to store event: {0}")]
    EventProcessingError(String),
}

#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync {
    /// Process an event.
    async fn process_event(&self, event: &Event) -> Result<(), ProcessingError>;

    /// Processor name (for logging purposes).
    fn name(&self) -> &'static str;
}
