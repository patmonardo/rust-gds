//! Tetradic-level schema traits and markers

use super::types::TetradicSchema;

/// Marker trait for Tetradic HyperSchema
pub trait TetradicHyperSchema: Send + Sync {
    /// Get the tetradic schema
    fn schema(&self) -> TetradicSchema;
}

/// Default implementation of TetradicHyperSchema
impl TetradicHyperSchema for TetradicSchema {
    fn schema(&self) -> TetradicSchema {
        self.clone()
    }
}

