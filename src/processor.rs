use tracing::Instrument;

use crate::common_types::{EventReceiver, EventsMap};

pub struct Processor {
    receiver: EventReceiver,
    events_map: EventsMap,
}

impl Processor {
    pub fn new(receiver: EventReceiver, events_map: EventsMap) -> Self {
        Processor { receiver, events_map }
    }

    #[tracing::instrument(skip(self))]
    pub async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
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
    }
}
