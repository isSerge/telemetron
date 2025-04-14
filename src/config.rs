use std::{collections::HashSet, env, io};

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

#[derive(Debug, Deserialize, Clone)]
pub struct EventValidationConfig {
    #[serde(default)]
    allowed_source_ids: HashSet<u64>,
    #[serde(default)]
    allowed_event_types: HashSet<EventType>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub http: HttpConfig,
    pub processor: ProcessorConfig,
    pub event_validation: EventValidationConfig,
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

    // helpers
    pub fn is_source_id_allowed(&self, source_id: u64) -> bool {
        if self.event_validation.allowed_source_ids.is_empty() {
            return true;
        }
        self.event_validation.allowed_source_ids.contains(&source_id)
    }

    pub fn is_event_type_allowed(&self, event_type: &EventType) -> bool {
        if self.event_validation.allowed_event_types.is_empty() {
            return true;
        }
        self.event_validation.allowed_event_types.contains(event_type)
    }
}
