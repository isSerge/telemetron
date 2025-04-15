use crate::common_types::{EventSender, EventValidators, EventsMap};

#[derive(Debug, Clone)]
pub struct AppState {
    pub events_map: EventsMap,
    pub sender: EventSender,
    pub validators: EventValidators,
}

impl AppState {
    pub fn new(sender: EventSender, events_map: EventsMap, validators: EventValidators) -> Self {
        AppState { events_map, sender, validators }
    }
}
