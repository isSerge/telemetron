use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::event::Event;

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;
pub type EventsMap = Arc<DashMap<u64, Vec<Event>>>;
