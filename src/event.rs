use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
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
