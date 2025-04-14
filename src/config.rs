use std::{
    collections::{HashMap, HashSet},
    env, io,
};

use serde::Deserialize;

use crate::event::EventType;

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessorConfig {
    pub channel_capacity: usize,
}

// Plugin specific config
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct SourceIdValidationConfig {
    #[serde(default)]
    pub allowed: HashSet<u64>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct EventTypeValidationConfig {
    #[serde(default)]
    pub allowed: HashSet<EventType>,
}

/// Config struct for plugins that do not require any parameters
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct NoParamsValidationConfig {}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct EventValidationConfig {
    #[serde(default)]
    pub plugins: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ProcessingConfig {
    #[serde(default)]
    pub plugins: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub http: HttpConfig,
    pub processor: ProcessorConfig,
    pub validation: EventValidationConfig,
    pub processing: ProcessingConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to load config file: {0}")]
    LoadError(#[from] config::ConfigError),

    #[error("Failed to get current working dir: {0}")]
    IoError(#[from] io::Error),
}

impl Config {
    pub fn try_load() -> Result<Self, ConfigError> {
        let config_path = env::current_dir()?.join("config.toml");

        tracing::info!("Loading config from: {:?}", config_path);

        // TODO: add source for env variables
        let config_builder =
            config::Config::builder().add_source(config::File::from(config_path).required(true));

        let settings = config_builder.build()?;

        settings.try_deserialize::<Config>().map_err(ConfigError::LoadError)
    }
}
