//! Dyadic-level schema traits and markers

use super::types::DyadicSchema;

/// Marker trait for Dyadic HyperSchema
pub trait DyadicHyperSchema: Send + Sync {
    /// Get the dyadic schema
    fn schema(&self) -> DyadicSchema;
}

/// Default implementation of DyadicHyperSchema
impl DyadicHyperSchema for DyadicSchema {
    fn schema(&self) -> DyadicSchema {
        self.clone()
    }
}

