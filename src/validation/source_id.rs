use std::collections::HashSet;

use super::{EventValidationError, EventValidator};
use crate::{config::SourceIdValidationConfig, event::Event};

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
