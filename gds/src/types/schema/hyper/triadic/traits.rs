//! Triadic-level schema traits and markers

use super::types::TriadicSchema;

/// Marker trait for Triadic HyperSchema
pub trait TriadicHyperSchema: Send + Sync {
    /// Get the triadic schema
    fn schema(&self) -> TriadicSchema;
}

/// Default implementation of TriadicHyperSchema
impl TriadicHyperSchema for TriadicSchema {
    fn schema(&self) -> TriadicSchema {
        self.clone()
    }
}

