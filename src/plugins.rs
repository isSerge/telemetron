use std::sync::Arc;

use crate::{
    common_types::{EventProcessors, EventValidators},
    config::Config,
};

#[derive(Debug, thiserror::Error)]
pub enum PluginError {}

pub fn build_validators(config: &Config) -> Result<EventValidators, PluginError> {
    let validators = vec![];

    Ok(Arc::new(validators))
}

pub fn build_processors(config: &Config) -> Result<EventProcessors, PluginError> {
    let processors = vec![];

    Ok(Arc::new(processors))
}
