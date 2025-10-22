//! Local Clustering Coefficient Storage Runtime

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;
use crate::types::graph::Degrees;

/// Storage Runtime for Local Clustering Coefficient
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure for computing clustering coefficients.
pub struct LocalClusteringCoefficientStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
}

impl<'a, G: GraphStore> LocalClusteringCoefficientStorageRuntime<'a, G> {
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

    /// Get degree of a node
    pub fn degree(&self, node_id: usize) -> i32 {
        let graph = self.graph_store.get_graph();
        graph.degree(node_id as u64) as i32
    }
}
