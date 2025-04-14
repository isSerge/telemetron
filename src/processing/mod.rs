pub mod storage;

use crate::event::Event;

pub trait EventProcessor: Send + Sync {
    /// Process an event.
    async fn process_event(&self, event: Event);

    /// Processor name (for logging purposes).
    fn name(&self) -> &'static str;
}
