use crate::{
    common_types::{EventSender, EventsMap},
    config::Config,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub events_map: EventsMap,
    pub sender: EventSender,
    pub config: Config,
}

impl AppState {
    pub fn new(sender: EventSender, events_map: EventsMap, config: Config) -> Self {
        AppState { events_map, sender, config }
    }
}
