use std::sync::Arc;

use crate::{
    common_types::{EventProcessors, EventValidators},
    config::Config,
    processing::EventProcessor,
    validation::EventValidator,
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

pub struct ValidationPluginFactory {
    pub name: &'static str,
    pub constructor: fn(toml::Value) -> Result<Box<dyn EventValidator + Send + Sync>, PluginError>,
}

pub struct ProcessingPluginFactory {
    pub name: &'static str,
    pub constructor: fn(toml::Value) -> Result<Box<dyn EventProcessor + Send + Sync>, PluginError>,
}

// Register the plugin factories
inventory::collect!(ValidationPluginFactory);
inventory::collect!(ProcessingPluginFactory);

pub fn build_validators(config: &Config) -> Result<EventValidators, PluginError> {
    let mut validators = Vec::new();
    let config_plugins = &config.validation.plugins;

    for factory in inventory::iter::<ValidationPluginFactory> {
        let name = factory.name;
        tracing::debug!(plugin_name = name, "Loading validator plugin");
        // Check if the plugin is in the config
        // and if so, call the constructor with the parameters
        // provided in the config
        if let Some(params) = config_plugins.get(name) {
            tracing::debug!(plugin_name = name, "Loading validator plugin");

            let plugin_box = (factory.constructor)(params.clone())?;
            validators.push(plugin_box);
            tracing::info!(plugin_name = name, "Validator plugin loaded successfully");
        } else {
            tracing::warn!(plugin_name = name, "Validator plugin not found in config");
        }
    }

    Ok(Arc::new(validators))
}

pub fn build_processors(config: &Config) -> Result<EventProcessors, PluginError> {
    let mut processors = Vec::new();
    let config_plugins = &config.processing.plugins;

    for factory in inventory::iter::<ProcessingPluginFactory> {
        let name = factory.name;
        tracing::debug!(plugin_name = name, "Loading processor plugin");
        // Check if the plugin is in the config
        // and if so, call the constructor with the parameters
        // provided in the config
        if let Some(params) = config_plugins.get(name) {
            tracing::debug!(plugin_name = name, "Loading processor plugin");

            let plugin_box = (factory.constructor)(params.clone())?;
            processors.push(plugin_box);
            tracing::info!(plugin_name = name, "Processor plugin loaded successfully");
        } else {
            tracing::warn!(plugin_name = name, "Processor plugin not found in config");
        }
    }

    Ok(Arc::new(processors))
}
