use std::collections::HashSet;

use super::{EventValidationError, EventValidator};
use crate::{
    config::EventTypeValidationConfig,
    event::{Event, EventType},
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
