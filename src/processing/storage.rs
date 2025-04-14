use super::{EventProcessor, ProcessingError};
use crate::{common_types::EventsMap, event::Event};

#[derive(Debug)]
pub struct StorageProcessor {
    events_map: EventsMap,
}

impl StorageProcessor {
    pub fn new(events_map: EventsMap) -> Self {
        StorageProcessor { events_map: events_map.clone() }
    }
}

#[async_trait::async_trait]
impl EventProcessor for StorageProcessor {
    async fn process_event(&self, event: &Event) -> Result<(), ProcessingError> {
        let mut source_events = self.events_map.entry(event.source_id).or_default();
        source_events.push(event.clone());

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StorageProcessor"
    }
}
