use tracing::Instrument;

use super::EventProcessor;
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

impl EventProcessor for StorageProcessor {
    async fn process_event(&self, event: Event) {
        let process_span = tracing::info_span!("process_event", source_id = event.source_id);

        async {
            tracing::info!("Storing event");
            let mut source_events = self.events_map.entry(event.source_id).or_default();
            source_events.push(event);
            tracing::info!("Event stored");
            // TODO: consider batching
        }
        .instrument(process_span)
        .await
    }

    fn name(&self) -> &'static str {
        "StorageProcessor"
    }
}
