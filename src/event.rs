use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Debug, Deserialize, Clone)]
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
