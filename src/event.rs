use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    source_id: u64,
    // TODO: should be an enum
    r#type: String,
    // TODO: should be a timestamp
    timestamp: String,
    data: Option<Value>,
}
