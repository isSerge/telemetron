use crate::common_types::{EventSender, EventsMap};

#[derive(Debug, Clone)]
pub struct AppState {
    events_map: EventsMap,
    pub sender: EventSender,
}

impl AppState {
    pub fn new(sender: EventSender, events_map: EventsMap) -> Self {
        AppState { events_map, sender }
    }
}
