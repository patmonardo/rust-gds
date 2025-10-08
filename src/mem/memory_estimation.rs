//! Memory estimation trait and types
//!
//! Provides interfaces for components that can estimate their memory usage
//! based on graph dimensions and concurrency settings.

use super::memory_tree::MemoryTree;
use crate::core::graph_dimensions::GraphDimensions;

/// A component that can estimate its memory usage
///
/// Implementations provide hierarchical memory estimations that can be
/// composed to estimate total memory requirements for complex operations.
pub trait MemoryEstimation {
    /// Returns a textual description for this component
    fn description(&self) -> String;

    /// Computes an actual memory estimation based on graph dimensions and concurrency
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The graph dimensions (node count, relationship count, etc.)
    /// * `concurrency` - The concurrency level (number of threads)
    ///
    /// # Returns
    ///
    /// A memory tree representing the hierarchical estimation
    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree;

    /// Returns nested components of this estimation
    ///
    /// Default implementation returns an empty vector. Override to provide
    /// hierarchical estimations.
    fn components(&self) -> Vec<Box<dyn MemoryEstimation>> {
        Vec::new()
    }
}

/// A value class pairing a memory estimation with graph dimensions
pub struct MemoryEstimationWithDimensions {
    memory_estimation: Box<dyn MemoryEstimation>,
    node_count: usize,
    relationship_count: usize,
}

impl MemoryEstimationWithDimensions {
    /// Creates a new pairing of estimation and dimensions
    pub fn new(
        memory_estimation: Box<dyn MemoryEstimation>,
        node_count: usize,
        relationship_count: usize,
    ) -> Self {
        Self {
            memory_estimation,
            node_count,
            relationship_count,
        }
    }

    /// Returns the memory estimation
    pub fn memory_estimation(&self) -> &dyn MemoryEstimation {
        &*self.memory_estimation
    }

    /// Returns the node count
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the relationship count
    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::ConcreteGraphDimensions;
    use crate::mem::MemoryRange;

    struct SimpleEstimation {
        desc: String,
        fixed_size: usize,
    }

    impl MemoryEstimation for SimpleEstimation {
        fn description(&self) -> String {
            self.desc.clone()
        }

        fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
            MemoryTree::leaf(self.desc.clone(), MemoryRange::of(self.fixed_size))
        }
    }

    #[test]
    fn test_simple_estimation() {
        let estimation = SimpleEstimation {
            desc: "test".to_string(),
            fixed_size: 1024,
        };

        assert_eq!(estimation.description(), "test");
        assert_eq!(estimation.components().len(), 0);

        let dims = ConcreteGraphDimensions::of(1000, 5000);
        let tree = estimation.estimate(&dims, 4);
        assert_eq!(tree.memory_usage().min(), 1024);
    }
}
