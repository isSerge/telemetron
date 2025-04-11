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
    source_id: u64,
    // TODO: should be an enum
    r#type: EventType,
    // TODO: should be a timestamp
    timestamp: DateTime<Utc>,
    data: Option<Value>,
}
