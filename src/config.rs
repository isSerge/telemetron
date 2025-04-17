use std::{
    collections::{HashMap, HashSet},
    env, io,
};

use config::Environment;
use serde::Deserialize;

use crate::{event::EventType, plugins};

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessorConfig {
    pub channel_capacity: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_batch_timeout")]
    pub batch_timeout: u64,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay: u64,
}

fn default_batch_size() -> usize {
    100
}

fn default_batch_timeout() -> u64 {
    1000
}

fn default_retry_attempts() -> u32 {
    3
}

fn default_retry_delay() -> u64 {
    1000
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
    #[error("Unknown validation plugin(s): {0:?}")]
    UnknownValidationPlugin(HashSet<String>),
    #[error("Unknown processing plugin(s): {0:?}")]
    UnknownProcessingPlugin(HashSet<String>),
}

impl Config {
    /// Load the config file and environment variables, deserialize and validate
    /// the config file into the Config struct
    pub fn try_load() -> Result<Self, ConfigError> {
        let settings = Self::load_base_config()?;

        let config = settings.try_deserialize::<Config>()?;

        config.validate_plugins()?;

        Ok(config)
    }

    /// Load the base config file and environment variables
    fn load_base_config() -> Result<config::Config, ConfigError> {
        let config_path = env::current_dir()?.join("config.toml");
        tracing::info!("Loading config from: {:?}", config_path);

        let config_builder = config::Config::builder()
            // Load the default config file
            .add_source(config::File::from(config_path).required(true))
            // Load environment variables
            .add_source(
                Environment::with_prefix("TELEMETRON").prefix_separator("_").separator("__"),
            )
            .build()?;

        Ok(config_builder)
    }

    fn validate_plugins(&self) -> Result<(), ConfigError> {
        // Collect known plugins from inventory
        let known_validation_plugins: HashSet<String> =
            inventory::iter::<plugins::ValidationPluginFactory>
                .into_iter()
                .map(|p| p.name.to_string())
                .collect();
        let known_processing_plugins: HashSet<String> =
            inventory::iter::<plugins::ProcessingPluginFactory>
                .into_iter()
                .map(|p| p.name.to_string())
                .collect();

        // Check validation plugins
        let configured_validation_plugins: HashSet<String> =
            self.validation.plugins.keys().cloned().collect();
        let unknown_validation_plugins = configured_validation_plugins
            .difference(&known_validation_plugins)
            .cloned()
            .collect::<HashSet<_>>();

        if !unknown_validation_plugins.is_empty() {
            return Err(ConfigError::UnknownValidationPlugin(unknown_validation_plugins));
        }

        // Check processing plugins
        let configured_processing_plugins: HashSet<String> =
            self.processing.plugins.keys().cloned().collect();
        let unknown_processing_plugins = configured_processing_plugins
            .difference(&known_processing_plugins)
            .cloned()
            .collect::<HashSet<_>>();

        if !unknown_processing_plugins.is_empty() {
            return Err(ConfigError::UnknownProcessingPlugin(unknown_processing_plugins));
        }

        // TODO: warn about plugins that are not configured

        Ok(())
    }
}
