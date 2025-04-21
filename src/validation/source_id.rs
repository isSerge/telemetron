use std::collections::HashSet;

use super::{EventValidationError, EventValidator};
use crate::{
    config::SourceIdValidationConfig,
    event::Event,
    plugins::{PluginError, ValidationPluginFactory},
};

#[derive(Debug)]
pub struct SourceIdValidator {
    pub allowed_ids: HashSet<u64>,
}

impl SourceIdValidator {
    pub fn new(config: SourceIdValidationConfig) -> Self {
        if config.allowed.is_empty() {
            tracing::warn!(
                "SourceIdValidator initialized with no allowed IDs. This will allow all source \
                 IDs."
            );
        }
        Self { allowed_ids: config.allowed }
    }
}

impl EventValidator for SourceIdValidator {
    fn name(&self) -> &'static str {
        "SourceIdValidator"
    }

    fn validate(&self, event: &Event) -> Result<(), EventValidationError> {
        if self.allowed_ids.is_empty() || self.allowed_ids.contains(&event.source_id) {
            Ok(())
        } else {
            Err(EventValidationError::DisallowedSourceId(event.source_id))
        }
    }
}

/// Constructs a SourceIdValidator from the given parameters.
/// This function is called by the plugin factory to create a new instance
/// of the plugin.
/// It deserializes the parameters from TOML format and creates a new
/// SourceIdValidator instance.
fn construct_source_id_validator(
    config_params: toml::Value,
) -> Result<Box<dyn EventValidator + Send + Sync>, PluginError> {
    let config: SourceIdValidationConfig =
        config_params.try_into().map_err(|e| PluginError::ParameterDeserialization {
            plugin_name: "SourceIdValidator".to_string(),
            source: e,
        })?;
    Ok(Box::new(SourceIdValidator::new(config)))
}

// Submit plugin to an inventory
inventory::submit! {
  ValidationPluginFactory {
        name: "SourceIdValidator",
        constructor: construct_source_id_validator,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::event::{Event, EventType};

    /// Creates a new event for testing purposes.
    fn create_event(source_id: u64, event_type: EventType) -> Event {
        Event { source_id, r#type: event_type, timestamp: Utc::now(), data: None }
    }

    fn get_allowed_ids() -> HashSet<u64> {
        vec![1, 2, 3].into_iter().collect()
    }

    #[test]
    fn test_validates_allowed_source_id() {
        let validator =
            SourceIdValidator::new(SourceIdValidationConfig { allowed: get_allowed_ids() });

        let event = create_event(1, EventType::Heartbeat);
        assert!(validator.validate(&event).is_ok());
    }

    #[test]
    fn test_validates_disallowed_source_id() {
        let validator =
            SourceIdValidator::new(SourceIdValidationConfig { allowed: get_allowed_ids() });

        let event = create_event(4, EventType::Heartbeat);
        assert!(validator.validate(&event).is_err());
    }

    #[test]
    fn test_validates_empty_allowed_source_ids() {
        let validator =
            SourceIdValidator::new(SourceIdValidationConfig { allowed: HashSet::new() });

        let event = create_event(4, EventType::Heartbeat);
        assert!(validator.validate(&event).is_ok());
    }
}
