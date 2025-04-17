use std::{sync::Arc, time::Duration};

use futures::Future;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tracing::Instrument;

use crate::{
    common_types::{EventProcessors, EventReceiver, TelemetryMap},
    config::Config,
    metrics::{
        EVENTS_PROCESSED_TOTAL, PROCESSOR_PLUGIN_DURATION_SECONDS, PROCESSOR_PLUGIN_ERRORS_TOTAL,
    },
    processing::error::ProcessingError,
};

// Retry helper
async fn execute_with_retries<F, Fut>(
    plugin_name: &str,
    retry_attempts: u32,
    retry_delay: u64,
    mut operation: F,
) -> Result<(), ProcessingError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(), ProcessingError>>,
{
    let mut attempts = 0;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) if attempts < retry_attempts => {
                tracing::warn!(
                    "Plugin {} failed to process event: {}. Retrying in {} ms (attempt {}/{})",
                    plugin_name,
                    err,
                    retry_delay,
                    attempts,
                    retry_attempts
                );
                tokio::time::sleep(Duration::from_millis(retry_delay)).await;
            }
            Err(err) => {
                tracing::error!(
                    target: "dlq_log",
                    plugin = plugin_name,
                    attempts = attempts,
                    error = %err,
                    "DLQ: Operation failed permanently after {} attempts. Logging failed batch summary.", attempts
                );
                metrics::counter!(PROCESSOR_PLUGIN_ERRORS_TOTAL, "plugin" => plugin_name.to_owned())
                    .increment(1);
                return Err(err);
            }
        }
    }
}

pub struct EventProcessorManager {
    telemetry_map: TelemetryMap,
    plugins: EventProcessors,
    config: Arc<Config>,
}

impl EventProcessorManager {
    pub fn new(telemetry_map: TelemetryMap, plugins: EventProcessors, config: Arc<Config>) -> Self {
        EventProcessorManager { telemetry_map, plugins, config }
    }

    #[tracing::instrument(skip_all)]
    pub async fn run(&self, receiver: EventReceiver) {
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

                let mut batch_processed = true; // Flag to track if batch was processed successfully

                for plugin in self.plugins.iter() {
                    let start = std::time::Instant::now();
                    let name = plugin.name();
                    let retry_attempts = self.config.processor.retry_attempts;
                    let retry_delay = self.config.processor.retry_delay;

                    tracing::info!("Processing with plugin: {}", name);

                    let operation = || plugin.process_event(&self.telemetry_map, &events_batch);

                    let result =
                        execute_with_retries(name, retry_attempts, retry_delay, operation).await;

                    match result {
                        Ok(_) => {
                            metrics::histogram!(
                              PROCESSOR_PLUGIN_DURATION_SECONDS,
                              "plugin" => name.to_owned(),
                              "status" => "success"
                            )
                            .record(start.elapsed());

                            tracing::info!("Plugin {} processed events successfully", name);
                        }
                        Err(_) => {
                            // Mark batch as not processed successfully
                            batch_processed = false;
                            metrics::histogram!(
                              PROCESSOR_PLUGIN_DURATION_SECONDS,
                              "plugin" => name.to_owned(),
                              "status" => "error"
                            )
                            .record(start.elapsed());

                            tracing::error!(
                                "Plugin failed processing batch after all retries (see DLQ log)."
                            )
                        }
                    }
                }

                if batch_processed {
                    metrics::counter!(EVENTS_PROCESSED_TOTAL).increment(events_batch.len() as u64);
                    tracing::info!("Batch of {} processed successfully", events_batch.len());
                } else {
                    tracing::warn!("Failed to process batch of {} events", events_batch.len());
                }
            }
            .instrument(process_span)
            .await
        }

        tracing::info!(
            "Event receiver channel closed and all batches processed. Exiting processor loop."
        );
    }
}
