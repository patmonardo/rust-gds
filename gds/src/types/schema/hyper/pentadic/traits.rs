//! Pentadic-level schema traits and markers

use super::types::PentadicSchema;

/// Marker trait for Pentadic HyperSchema
pub trait PentadicHyperSchema: Send + Sync {
    /// Get the pentadic schema
    fn schema(&self) -> PentadicSchema;
}

/// Default implementation of PentadicHyperSchema
impl PentadicHyperSchema for PentadicSchema {
    fn schema(&self) -> PentadicSchema {
        self.clone()
    }
}

