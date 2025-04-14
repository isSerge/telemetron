use std::{sync::Arc, time::Duration};

use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tracing::Instrument;

use crate::{
    common_types::{EventProcessors, EventReceiver, EventsMap},
    config::Config,
};

pub struct Processor {
    events_map: EventsMap,
    plugins: EventProcessors,
    config: Arc<Config>,
}

impl Processor {
    pub fn new(events_map: EventsMap, plugins: EventProcessors, config: Arc<Config>) -> Self {
        Processor { events_map, plugins, config }
    }

    #[tracing::instrument(skip(self, receiver))]
    pub async fn run(&mut self, receiver: EventReceiver) {
        let batch_size = self.config.processor.batch_size;
        let batch_timeout = self.config.processor.batch_timeout;

        tracing::info!(
            batch_size = batch_size,
            batch_timeout = batch_timeout,
            "Starting processor"
        );

        let receiver_stream = ReceiverStream::new(receiver)
            .chunks_timeout(batch_size, Duration::from_millis(batch_timeout));

        tokio::pin!(receiver_stream);

        while let Some(events_batch) = receiver_stream.next().await {
            if events_batch.is_empty() {
                tracing::debug!("Received empty batch of events");
                continue;
            }

            let process_span =
                tracing::info_span!("process_event_batch", batch_size = events_batch.len());

            async {
                tracing::info!("Processing batch of events");
                for plugin in self.plugins.iter() {
                    tracing::info!("Processing with plugin: {}", plugin.name());
                    match plugin.process_event(&self.events_map, &events_batch).await {
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
