use std::collections::HashSet;

use super::{EventValidationError, EventValidator};
use crate::{
    config::EventTypeValidationConfig,
    event::{Event, EventType},
    plugins::{PluginError, ValidationPluginFactory},
};

#[derive(Debug)]
pub struct EventTypeValidator {
    pub allowed_types: HashSet<EventType>,
}

impl EventTypeValidator {
    pub fn new(config: EventTypeValidationConfig) -> Self {
        if config.allowed.is_empty() {
            tracing::warn!(
                "EventTypeValidator initialized with no allowed event types. This will allow all \
                 event types."
            );
        } else {
            tracing::info!(
                "EventTypeValidator initialized with allowed event types: {:?}",
                config.allowed
            );
        }
        Self { allowed_types: config.allowed }
    }
}

impl EventValidator for EventTypeValidator {
    fn name(&self) -> &'static str {
        "EventTypeValidator"
    }

    fn validate(&self, event: &Event) -> Result<(), EventValidationError> {
        if self.allowed_types.is_empty() || self.allowed_types.contains(&event.r#type) {
            Ok(())
        } else {
            Err(EventValidationError::DisallowedEventType(event.r#type.clone()))
        }
    }
}

/// Constructs an EventTypeValidator from the given parameters.
/// This function is called by the plugin factory to create a new instance of
/// the plugin.
/// It deserializes the parameters from TOML format and creates a new
/// EventTypeValidator instance.
fn construct_event_type_validator(
    config_params: toml::Value,
) -> Result<Box<dyn EventValidator + Send + Sync>, PluginError> {
    let config: EventTypeValidationConfig =
        config_params.try_into().map_err(|e| PluginError::ParameterDeserialization {
            plugin_name: "EventTypeValidator".to_string(),
            source: e,
        })?;
    Ok(Box::new(EventTypeValidator::new(config)))
}

// Submit plugin to an inventory
inventory::submit! {
  ValidationPluginFactory {
        name: "EventTypeValidator",
        constructor: construct_event_type_validator,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::event::EventType;

    #[test]
    fn test_validates_allowed_event_type() {
        let allowed = HashSet::from([EventType::Heartbeat, EventType::Custom("Test".to_string())]);
        let config = EventTypeValidationConfig { allowed };
        let validator = EventTypeValidator::new(config);
        let event =
            Event { source_id: 1, r#type: EventType::Heartbeat, timestamp: Utc::now(), data: None };

        assert!(validator.validate(&event).is_ok());
    }

    #[test]
    fn test_validates_disallowed_event_type() {
        let allowed = HashSet::from([EventType::Custom("Test".to_string())]);
        let config = EventTypeValidationConfig { allowed };
        let validator = EventTypeValidator::new(config);
        let event =
            Event { source_id: 1, r#type: EventType::Heartbeat, timestamp: Utc::now(), data: None };

        assert!(validator.validate(&event).is_err());
    }

    #[test]
    fn test_validates_empty_allowed_event_types() {
        let allowed = HashSet::new();
        let config = EventTypeValidationConfig { allowed };
        let validator = EventTypeValidator::new(config);
        let event =
            Event { source_id: 1, r#type: EventType::Heartbeat, timestamp: Utc::now(), data: None };

        assert!(validator.validate(&event).is_ok());
    }
}
