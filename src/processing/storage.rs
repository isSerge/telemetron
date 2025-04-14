use super::{EventProcessor, ProcessingError};
use crate::{common_types::EventsMap, event::Event};

#[derive(Debug, Default)]
pub struct StorageProcessor;

#[async_trait::async_trait]
impl EventProcessor for StorageProcessor {
    async fn process_event(
        &self,
        events_map: &EventsMap,
        event: &Event,
    ) -> Result<(), ProcessingError> {
        let mut source_events = events_map.entry(event.source_id).or_default();
        source_events.push(event.clone());

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StorageProcessor"
    }
}
