use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::{event::Event, processing::EventProcessor, validation::EventValidator};

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;
pub type TelemetryMap = Arc<DashMap<u64, SourceTelemetry>>;
pub type SourceTelemetry = Vec<Event>;
pub type EventValidators = Arc<Vec<Box<dyn EventValidator + Send + Sync>>>;
pub type EventProcessors = Arc<Vec<Box<dyn EventProcessor + Send + Sync>>>;
