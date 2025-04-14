use tracing::Instrument;

use crate::common_types::{EventProcessors, EventReceiver, EventsMap};

pub struct Processor {
    receiver: EventReceiver,
    events_map: EventsMap,
    plugins: EventProcessors,
}

impl Processor {
    pub fn new(receiver: EventReceiver, events_map: EventsMap, plugins: EventProcessors) -> Self {
        Processor { receiver, events_map, plugins }
    }

    #[tracing::instrument(skip(self))]
    pub async fn run(&mut self) {
        tracing::info!("Starting processor");
        while let Some(event) = self.receiver.recv().await {
            let process_span = tracing::info_span!("process_event", source_id = event.source_id, event_type = ?event.r#type);

            async {
                tracing::info!("Processing event");
                for plugin in self.plugins.iter() {
                    tracing::info!("Processing with plugin: {}", plugin.name());
                    match plugin.process_event(&self.events_map, &event).await {
                        Ok(_) => tracing::info!("Plugin {} processed event", plugin.name()),
                        Err(err) => {
                            tracing::error!(
                                "Plugin {} failed to process event: {}",
                                plugin.name(),
                                err
                            );
                        } /* TODO: add retry logic
                           * TODO: send failed events to a dead letter queue
                           */
                    }
                }
            }
            .instrument(process_span)
            .await
        }
    }
}
