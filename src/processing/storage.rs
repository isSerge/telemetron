use std::collections::HashMap;

use super::{EventProcessor, ProcessingError};
use crate::{common_types::EventsMap, event::Event};

#[derive(Debug, Default)]
pub struct StorageProcessor;

#[async_trait::async_trait]
impl EventProcessor for StorageProcessor {
    async fn process_event(
        &self,
        events_map: &EventsMap,
        events: &[Event],
    ) -> Result<(), ProcessingError> {
        if events.is_empty() {
            return Ok(());
        }

        tracing::debug!("Processing batch of events: {:?}", events.len());

        // group events by source_id
        let mut grouped_events: HashMap<u64, Vec<&Event>> = HashMap::new();
        for event in events {
            grouped_events.entry(event.source_id).or_default().push(event);
        }

        // iterate over each group and store the events
        for (source_id, events) in grouped_events {
            if events.is_empty() {
                continue;
            }
            tracing::debug!("Storing events for source_id: {}", source_id);
            let mut source_events = events_map.entry(source_id).or_default();
            source_events.extend(events.into_iter().cloned());
        }

        tracing::debug!("Finish processing event batch");

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StorageProcessor"
    }
}
