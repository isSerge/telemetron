use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::event::{Event, EventType};

#[derive(Debug, Clone)]
pub struct SourceTelemetry {
    pub total_events: u64,
    pub first_timestamp: DateTime<Utc>,
    pub last_timestamp: DateTime<Utc>,
    pub events_by_type: HashMap<EventType, u64>,
}

impl SourceTelemetry {
    pub fn new(event: &Event) -> Self {
        let mut events_by_type = HashMap::new();
        events_by_type.insert(event.r#type.clone(), 1);

        Self {
            total_events: 1,
            first_timestamp: event.timestamp,
            last_timestamp: event.timestamp,
            events_by_type,
        }
    }

    pub fn update(&mut self, event: &Event) {
        self.total_events += 1;
        self.first_timestamp = self.first_timestamp.min(event.timestamp);
        self.last_timestamp = self.last_timestamp.max(event.timestamp);
        *self.events_by_type.entry(event.r#type.clone()).or_insert(0) += 1;
    }
}
