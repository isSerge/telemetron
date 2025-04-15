use super::{EventProcessor, ProcessingError};
use crate::{
    common_types::TelemetryMap,
    config::NoParamsValidationConfig,
    event::Event,
    plugins::{PluginError, ProcessingPluginFactory},
    processing::source_telemetry::SourceTelemetry,
};

#[derive(Debug, Default)]
pub struct StorageProcessor;

impl StorageProcessor {
    pub fn new(_config: NoParamsValidationConfig) -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl EventProcessor for StorageProcessor {
    async fn process_event(
        &self,
        telemetry_map: &TelemetryMap,
        events: &[Event],
    ) -> Result<(), ProcessingError> {
        if events.is_empty() {
            return Ok(());
        }

        tracing::debug!("Processing batch of events: {:?}", events.len());

        // iterate over each group and store the events
        for event in events {
            if events.is_empty() {
                continue;
            }
            let source_entry = telemetry_map.entry(event.source_id);

            source_entry
                // if the telemetry already exists, update it
                .and_modify(|telemetry| {
                    tracing::debug!("Updating telemetry for source_id: {}", event.source_id);
                    telemetry.update(event)
                })
                // if the telemetry does not exist, create it
                .or_insert_with(|| {
                    tracing::debug!("Creating new telemetry for source_id: {}", event.source_id);
                    
                    SourceTelemetry::new(event)
                });
        }

        tracing::debug!("Finish processing event batch");

        Ok(())
    }

    fn name(&self) -> &'static str {
        "StorageProcessor"
    }
}

/// Constructs a StorageProcessor plugin.
/// This function is called by the plugin factory to create a new instance of
/// the plugin.
fn construct_storage_processor(
    config_params: toml::Value,
) -> Result<Box<dyn EventProcessor + Send + Sync>, PluginError> {
    // StorageProcessor does not require any parameters, but keep deserialization
    // for consistency with other plugins
    let config: NoParamsValidationConfig =
        config_params.try_into().map_err(|e| PluginError::ParameterDeserialization {
            plugin_name: "StorageProcessor".to_string(),
            source: e,
        })?;
    Ok(Box::new(StorageProcessor::new(config)))
}

// Submit plugin to an inventory
inventory::submit! {
  ProcessingPluginFactory {
        name: "StorageProcessor",
        constructor: construct_storage_processor,
    }
}
