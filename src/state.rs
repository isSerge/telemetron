use std::sync::Arc;

use dashmap::DashMap;

use crate::event::Event;

#[derive(Debug, Clone)]
pub struct AppState {
    storage: Arc<DashMap<u64, Event>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { storage: Arc::new(DashMap::new()) }
    }
}
