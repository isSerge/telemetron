pub mod event_type;
pub mod source_id;

use crate::event::{Event, EventValidationError};

pub trait EventValidator: Send + Sync {
    /// Validate an event.
    fn validate(&self, event: &Event) -> Result<(), EventValidationError>;

    /// Validator name (for logging purposes).
    fn name(&self) -> &'static str;
}
