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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use dashmap::DashMap;

    use super::*;
    use crate::{
        common_types::TelemetryMap,
        event::{Event, EventType},
    };

    /// Creates a new telemetry map for testing purposes.
    fn create_map() -> TelemetryMap {
        Arc::new(DashMap::new())
    }

    /// Creates a new event for testing purposes.
    fn create_event(source_id: u64, event_type: EventType) -> Event {
        Event { source_id, r#type: event_type, timestamp: Utc::now(), data: None }
    }

    #[tokio::test]
    async fn test_empty_batch() {
        let processor = StorageProcessor::new(NoParamsValidationConfig::default());
        let telemetry_map = create_map();
        let events: Vec<Event> = vec![];

        // Process an empty batch of events
        let result = processor.process_event(&telemetry_map, &events).await;

        assert!(result.is_ok());
        assert!(telemetry_map.is_empty());
    }

    #[tokio::test]
    async fn test_single_event() {
        let processor = StorageProcessor::new(NoParamsValidationConfig::default());
        let telemetry_map = create_map();
        let events = vec![create_event(1, EventType::Heartbeat)];

        // Process a single event
        let result = processor.process_event(&telemetry_map, &events).await;

        assert!(result.is_ok());
        assert_eq!(telemetry_map.len(), 1);
        assert!(telemetry_map.contains_key(&1));
    }

    #[tokio::test]
    async fn test_multiple_events() {
        let processor = StorageProcessor::new(NoParamsValidationConfig::default());
        let telemetry_map = create_map();
        let events = vec![
            create_event(1, EventType::Heartbeat),
            create_event(2, EventType::Custom("CustomEvent".to_string())),
            create_event(1, EventType::Heartbeat),
        ];

        // Process multiple events
        let result = processor.process_event(&telemetry_map, &events).await;

        assert!(result.is_ok());
        assert_eq!(telemetry_map.len(), 2);
        assert!(telemetry_map.contains_key(&1));
        assert!(telemetry_map.contains_key(&2));
    }

    #[tokio::test]
    async fn test_update_same_source_id() {
        let processor = StorageProcessor::new(NoParamsValidationConfig::default());
        let telemetry_map = create_map();
        let events = vec![
            create_event(1, EventType::Heartbeat),
            create_event(1, EventType::Custom("CustomEvent".to_string())),
        ];

        // Process events that update the same source_id
        let result = processor.process_event(&telemetry_map, &events).await;

        assert!(result.is_ok());
        assert_eq!(telemetry_map.len(), 1);
        assert!(telemetry_map.contains_key(&1));

        let entry = telemetry_map.get(&1);

        assert!(entry.is_some());

        match entry {
            Some(telemetry) => {
                assert_eq!(telemetry.value().total_events, 2);
                assert_eq!(telemetry.value().events_by_type.len(), 2);
                assert_eq!(telemetry.value().last_timestamp, events[1].timestamp);
            }
            None => panic!("Telemetry entry should exist"),
        }
    }
}
