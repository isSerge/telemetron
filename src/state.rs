use std::sync::Arc;

use crate::{
    common_types::{EventSender, EventsMap},
    config::Config,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub events_map: EventsMap,
    pub sender: EventSender,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(sender: EventSender, events_map: EventsMap, config: Arc<Config>) -> Self {
        AppState { events_map, sender, config }
    }
}
