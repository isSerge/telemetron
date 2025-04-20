use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse event type: {0}")]
pub struct ParseEventTypeError(String);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum EventType {
    /// Heartbeat event
    Heartbeat,
    /// Custom event type
    /// This is used for custom event types that are not predefined in the
    /// system.
    Custom(String),
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Heartbeat => write!(f, "Heartbeat"),
            EventType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Parses a string into an EventType.
/// If the string is "Heartbeat", it returns EventType::Heartbeat.
impl EventType {
    pub fn from_str(s: &str) -> Result<Self, ParseEventTypeError> {
        match s {
            "Heartbeat" => Ok(EventType::Heartbeat),
            _ => {
                if s.is_empty() {
                    return Err(ParseEventTypeError("Event type cannot be empty".to_string()));
                }
                Ok(EventType::Custom(s.to_string()))
            }
        }
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EventType::Heartbeat => serializer.serialize_str("Heartbeat"),
            EventType::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EventType::from_str(&s).map_err(serde::de::Error::custom)
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
