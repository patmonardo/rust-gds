//! Monadic-level schema traits and markers

use super::types::MonadicSchema;

/// Marker trait for Monadic HyperSchema
pub trait MonadicHyperSchema: Send + Sync {
    /// Get the monadic schema
    fn schema(&self) -> MonadicSchema;
}

/// Default implementation of MonadicHyperSchema
impl MonadicHyperSchema for MonadicSchema {
    fn schema(&self) -> MonadicSchema {
        self.clone()
    }
}

