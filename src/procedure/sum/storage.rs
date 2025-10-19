//! Storage Runtime for Sum Aggregation
//!
//! This module implements the **Gross pole** of the Functor machinery.
//! It represents persistent data structures (PropertyValues in storage).

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;

/// Storage Runtime for Sum Aggregation
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to extract values from PropertyValues (the storage layer).
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent PropertyValues in storage
/// - **Computation Runtime** (Subtle) = ephemeral accumulation values
/// - **Functor** = the mapping between them
pub struct SumStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
    /// Property key to aggregate
    property_key: String,
}

impl<'a, G: GraphStore> SumStorageRuntime<'a, G> {
    /// Create a new storage runtime
    ///
    /// **Parameters**:
    /// - `graph_store`: The graph to access
    /// - `property_key`: Which property to sum
    pub fn new(graph_store: &'a G, property_key: &str) -> Result<Self, AlgorithmError> {
        // TODO: Validate that property_key exists on the graph
        // For now, just accept any property key

        Ok(Self {
            graph_store,
            property_key: property_key.to_string(),
        })
    }

    /// Get node value from storage
    ///
    /// This projects from PropertyValues (Gross - persistent storage)
    /// to f64 (Subtle - computation value).
    ///
    /// **This is where the Functor machinery actually works**:
    /// PropertyValues (Gross) → f64 (Subtle)
    ///
    /// ## Implementation Notes
    ///
    /// Currently returns 1.0 for all nodes (placeholder).
    /// Future implementation would:
    /// 1. Look up PropertyValues by property_key
    /// 2. Extract value for node_id
    /// 3. Convert to f64 (type projection)
    /// 4. Handle missing values (default behavior)
    pub fn get_node_value(&self, _node_id: u32) -> Result<f64, AlgorithmError> {
        // TODO: Actually read from PropertyValues
        // For now: placeholder implementation returns 1.0
        // This simulates the Functor: PropertyValues → f64

        Ok(1.0)
    }

    /// Get the property key
    pub fn property_key(&self) -> &str {
        &self.property_key
    }

    /// Get reference to graph store
    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }
}

#[cfg(test)]
mod tests {
    // Note: Full tests would require a real GraphStore
    // For now, we just test creation

    #[test]
    fn test_storage_runtime_property_key() {
        // This test would need a mock GraphStore
        // Skipping implementation for now
    }
}
