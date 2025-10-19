//! Graph memory estimation result container.
//!
//! Pairs graph dimensions with memory usage tree for complete estimation.

use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::mem::{MemoryRange, MemoryTree};

/// Memory estimation result for a graph.
///
/// Contains both the graph dimensions and the hierarchical memory tree
/// describing memory usage breakdown.
///
/// # Example
///
/// ```rust,ignore
/// use gds::mem::memest::GraphMemoryEstimation;
///
/// let estimation = GraphMemoryEstimation::new(dimensions, memory_tree);
/// println!("Node count: {}", estimation.dimensions().node_count());
/// println!("Memory: {}", estimation.memory_tree().render());
/// ```
#[derive(Debug, Clone)]
pub struct GraphMemoryEstimation {
    dimensions: ConcreteGraphDimensions,
    memory_tree: MemoryTree,
}

impl GraphMemoryEstimation {
    /// Creates a new graph memory estimation.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - The graph dimensions
    /// * `memory_tree` - The memory usage tree
    pub fn new(dimensions: ConcreteGraphDimensions, memory_tree: MemoryTree) -> Self {
        Self {
            dimensions,
            memory_tree,
        }
    }

    /// Returns a reference to the graph dimensions.
    pub fn dimensions(&self) -> &ConcreteGraphDimensions {
        &self.dimensions
    }

    /// Returns the memory tree.
    pub fn memory_tree(&self) -> &MemoryTree {
        &self.memory_tree
    }

    /// Returns the memory range (min to max).
    pub fn memory_range(&self) -> &MemoryRange {
        self.memory_tree.memory_usage()
    }

    /// Returns the minimum memory required.
    pub fn min_memory(&self) -> usize {
        self.memory_tree.memory_usage().min()
    }

    /// Returns the maximum memory that might be required.
    pub fn max_memory(&self) -> usize {
        self.memory_tree.memory_usage().max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::GraphDimensions;
    use crate::mem::MemoryRange;

    #[test]
    fn test_graph_memory_estimation_creation() {
        let dimensions = ConcreteGraphDimensions::of(1000, 5000);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(1024));

        let estimation = GraphMemoryEstimation::new(dimensions, tree);

        assert_eq!(estimation.dimensions().node_count(), 1000);
        assert_eq!(estimation.min_memory(), 1024);
    }

    #[test]
    fn test_memory_range_access() {
        let dimensions = ConcreteGraphDimensions::of(100, 200);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of_range(512, 1024));

        let estimation = GraphMemoryEstimation::new(dimensions, tree);

        assert_eq!(estimation.min_memory(), 512);
        assert_eq!(estimation.max_memory(), 1024);
    }
}
