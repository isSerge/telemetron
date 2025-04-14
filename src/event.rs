use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::config::Config;

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum EventType {
    Heartbeat,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub source_id: u64,
    pub r#type: EventType,
    pub timestamp: DateTime<Utc>,
    pub data: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum EventValidationError {
    #[error("Invalid source_id: {0}")]
    InvalidSourceId(u64),
    #[error("Invalid event type")]
    InvalidEventType,
}

impl From<EventValidationError> for Event {
    fn from(err: EventValidationError) -> Self {
        match err {
            EventValidationError::InvalidSourceId(source_id) =>
                Event { source_id, r#type: EventType::Heartbeat, timestamp: Utc::now(), data: None },
            EventValidationError::InvalidEventType => Event {
                source_id: 0,
                r#type: EventType::Heartbeat,
                timestamp: Utc::now(),
                data: None,
            },
        }
    }
}

impl Event {
    pub fn validate(&self, config: &Config) -> Result<(), EventValidationError> {
        if !config.is_source_id_allowed(self.source_id) {
            return Err(EventValidationError::InvalidSourceId(self.source_id));
        }

        if !config.is_event_type_allowed(&self.r#type) {
            return Err(EventValidationError::InvalidEventType);
        }

        Ok(())
    }
}
