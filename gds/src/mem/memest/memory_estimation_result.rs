//! Memory estimation result builder with formatting utilities.
//!
//! Provides structured results from memory estimation with human-readable output.

use crate::core::graph_dimensions::{ConcreteGraphDimensions, GraphDimensions};
use crate::mem::{Estimate, MemoryRange, MemoryTree};
use std::collections::HashMap;

/// Result of a memory estimation containing a hierarchical breakdown.
#[derive(Debug, Clone)]
pub struct MemoryEstimationResult {
    dimensions: ConcreteGraphDimensions,
    memory_tree: MemoryTree,
}

impl MemoryEstimationResult {
    /// Creates a new memory estimation result.
    pub fn new(dimensions: ConcreteGraphDimensions, memory_tree: MemoryTree) -> Self {
        Self {
            dimensions,
            memory_tree,
        }
    }

    /// Returns the graph dimensions.
    pub fn dimensions(&self) -> &ConcreteGraphDimensions {
        &self.dimensions
    }

    /// Returns the minimum memory required for this estimation.
    pub fn memory_usage(&self) -> usize {
        self.memory_tree.memory_usage().min()
    }

    /// Returns the memory range (min to max) for this estimation.
    pub fn memory_range(&self) -> &MemoryRange {
        self.memory_tree.memory_usage()
    }

    /// Returns the memory tree for this estimation.
    pub fn tree(&self) -> &MemoryTree {
        &self.memory_tree
    }

    /// Formats the memory usage in human-readable form.
    pub fn format_memory_usage(&self) -> String {
        Estimate::human_readable(self.memory_usage())
    }

    /// Converts this result to a map structure for serialization.
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("requiredMemory".to_string(), self.format_memory_usage());
        map.insert(
            "nodeCount".to_string(),
            self.dimensions().node_count().to_string(),
        );
        map.insert(
            "relationshipCount".to_string(),
            self.dimensions().rel_count_upper_bound().to_string(),
        );
        map.insert("treeView".to_string(), self.memory_tree.render());
        map
    }
}

/// Builder for creating memory estimation results.
#[derive(Default)]
pub struct MemoryEstimationResultBuilder {
    dimensions: Option<ConcreteGraphDimensions>,
    memory_tree: Option<MemoryTree>,
}

impl MemoryEstimationResultBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the graph dimensions for the result.
    pub fn with_dimensions(mut self, dimensions: ConcreteGraphDimensions) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Sets the memory tree for the result.
    pub fn with_memory_tree(mut self, memory_tree: MemoryTree) -> Self {
        self.memory_tree = Some(memory_tree);
        self
    }

    /// Builds the memory estimation result.
    ///
    /// # Panics
    ///
    /// Panics if dimensions or memory tree are not set.
    pub fn build(self) -> MemoryEstimationResult {
        MemoryEstimationResult::new(
            self.dimensions.expect("Graph dimensions must be set"),
            self.memory_tree.expect("Memory tree must be set"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::{ConcreteGraphDimensions, GraphDimensions};

    #[test]
    fn test_builder_pattern() {
        let dimensions = ConcreteGraphDimensions::of(1000, 5000);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(1024 * 1024));

        let result = MemoryEstimationResultBuilder::new()
            .with_dimensions(dimensions)
            .with_memory_tree(tree)
            .build();

        assert_eq!(result.memory_usage(), 1024 * 1024);
        assert_eq!(result.dimensions().node_count(), 1000);
    }

    #[test]
    fn test_format_memory_usage() {
        let dimensions = ConcreteGraphDimensions::of(100, 200);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(1024 * 1024));

        let result = MemoryEstimationResult::new(dimensions, tree);

        let formatted = result.format_memory_usage();
        assert!(formatted.contains("MiB") || formatted.contains("KiB"));
    }

    #[test]
    fn test_to_map() {
        let dimensions = ConcreteGraphDimensions::of(1000, 5000);
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(2048));

        let result = MemoryEstimationResult::new(dimensions, tree);
        let map = result.to_map();

        assert!(map.contains_key("requiredMemory"));
        assert!(map.contains_key("nodeCount"));
        assert!(map.contains_key("relationshipCount"));
        assert!(map.contains_key("treeView"));
        assert_eq!(map.get("nodeCount").unwrap(), "1000");
    }

    #[test]
    #[should_panic(expected = "Graph dimensions must be set")]
    fn test_builder_missing_dimensions() {
        let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(1024));

        MemoryEstimationResultBuilder::new()
            .with_memory_tree(tree)
            .build();
    }

    #[test]
    #[should_panic(expected = "Memory tree must be set")]
    fn test_builder_missing_tree() {
        let dimensions = ConcreteGraphDimensions::of(100, 200);

        MemoryEstimationResultBuilder::new()
            .with_dimensions(dimensions)
            .build();
    }
}
