//! HITS Storage Runtime

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;

/// Storage Runtime for HITS
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure for computing HITS.
pub struct HitsStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
}

impl<'a, G: GraphStore> HitsStorageRuntime<'a, G> {
    /// Create a new storage runtime
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        Ok(Self { graph_store })
    }

    /// Get reference to graph store
    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.graph_store.node_count() as usize
    }
}
