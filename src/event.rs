use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum EventType {
    Heartbeat,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Heartbeat => write!(f, "Heartbeat"),
        }
    }
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
    #[error("Disallowed source_id: {0}")]
    DisallowedSourceId(u64),
    #[error("Disallowed event type: {0}")]
    DisallowedEventType(EventType),
}

impl From<EventValidationError> for Event {
    fn from(err: EventValidationError) -> Self {
        match err {
            EventValidationError::DisallowedSourceId(source_id) =>
                Event { source_id, r#type: EventType::Heartbeat, timestamp: Utc::now(), data: None },
            EventValidationError::DisallowedEventType(event_type) =>
                Event { source_id: 0, r#type: event_type, timestamp: Utc::now(), data: None },
        }
    }
}
