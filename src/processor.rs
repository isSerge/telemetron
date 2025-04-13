
use crate::common_types::{EventReceiver, EventsMap};

pub struct Processor {
    receiver: EventReceiver,
    events_map: EventsMap,
}

impl Processor {
    pub fn new(receiver: EventReceiver, events_map: EventsMap) -> Self {
        Processor { receiver, events_map }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            log::info!("Processing event: {:?}", event);
            let mut source_events = self.events_map.entry(event.source_id).or_default();
            source_events.push(event);

            // TODO: consider batching
        }
    }
}
