use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::event::Event;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Debug, Clone)]
pub struct AppState {
    storage: Arc<DashMap<u64, Event>>,
    sender: EventSender,
}

impl AppState {
    pub fn new(sender: EventSender) -> Self {
        AppState { storage: Arc::new(DashMap::new()), sender }
    }
}
