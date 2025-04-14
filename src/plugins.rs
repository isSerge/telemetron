use std::sync::Arc;

use crate::{
    common_types::{EventProcessors, EventValidators},
    config::{
        Config, EventTypeValidationConfig, NoParamsValidationConfig, SourceIdValidationConfig,
    },
    processing::{EventProcessor, storage::StorageProcessor},
    validation::{EventValidator, event_type::EventTypeValidator, source_id::SourceIdValidator},
};

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Failed to parse parameters for plugin '{plugin_name}': {source}")]
    ParameterDeserialization {
        plugin_name: String,
        #[source]
        source: toml::de::Error,
    },
}

pub fn build_validators(config: &Config) -> Result<EventValidators, PluginError> {
    let mut validators = Vec::new();

    for (name, params) in &config.validation.plugins {
        tracing::debug!(plugin_name = name, "Loading validator plugin");

        let plugin_box: Box<dyn EventValidator + Send + Sync> = match name.as_str() {
            "EventTypeValidator" => {
                let config: EventTypeValidationConfig = params.clone().try_into().map_err(|e| {
                    PluginError::ParameterDeserialization { plugin_name: name.clone(), source: e }
                })?;
                let validator = EventTypeValidator::new(config);
                Box::new(validator)
            }
            "SourceIdValidator" => {
                let config: SourceIdValidationConfig = params.clone().try_into().map_err(|e| {
                    PluginError::ParameterDeserialization { plugin_name: name.clone(), source: e }
                })?;
                let validator = SourceIdValidator::new(config);
                Box::new(validator)
            }
            _ => {
                tracing::error!(plugin_name = name, "Unknown plugin name");
                continue;
            }
        };

        validators.push(plugin_box);
        tracing::info!(plugin_name = name, "Validator plugin loaded successfully");
    }

    Ok(Arc::new(validators))
}

pub fn build_processors(config: &Config) -> Result<EventProcessors, PluginError> {
    let mut processors = Vec::new();

    for (name, params) in &config.processing.plugins {
        tracing::debug!(plugin_name = name, "Loading processor plugin");

        let plugin_box: Box<dyn EventProcessor + Send + Sync> = match name.as_str() {
            "StorageProcessor" => {
                // Deserialize into NoParams to ensure config format is valid ({})
                let _config: NoParamsValidationConfig = params.clone().try_into().map_err(|e| {
                    PluginError::ParameterDeserialization { plugin_name: name.clone(), source: e }
                })?;
                Box::new(StorageProcessor)
            }
            _ => {
                tracing::error!(plugin_name = name, "Unknown plugin name");
                continue;
            }
        };

        processors.push(plugin_box);
        tracing::info!(plugin_name = name, "Processor plugin loaded successfully");
    }

    Ok(Arc::new(processors))
}
