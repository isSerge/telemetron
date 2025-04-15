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
