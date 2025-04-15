/// This module defines a custom error type for handling errors that occur
/// during the processing of data in a plugin.
/// Example:
/// ```rust
/// Err(ProcessingError::new(
///     self.name(),
///     format!("DB write failed: {}", db_err),
///     Some(Box::new(db_err)),
/// ))
#[derive(Debug, thiserror::Error)]
#[error("Processing error in plugin {plugin_name}: {details}")]
pub struct ProcessingError {
    pub plugin_name: &'static str,
    pub details: String,
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ProcessingError {
    pub fn new(
        plugin_name: &'static str,
        details: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self { plugin_name, details: details.into(), source }
    }
}
