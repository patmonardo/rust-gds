//! **DFS Computation Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the "Subtle pole" for DFS algorithm - ephemeral computation state.

use std::collections::HashMap;

/// DFS Computation Runtime - handles ephemeral computation state
///
/// Translation of: `DFSComputation.java` (lines 32-75)
/// This implements the "Subtle pole" for accumulating traversal state
pub struct DfsComputationRuntime {
    /// Source node for traversal
    pub source_node: u32,
    /// Whether to track paths
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Visited nodes with their discovery order
    visited_nodes: HashMap<u32, u32>,
    /// Maximum depth constraint
    max_depth: Option<u32>,
}

impl DfsComputationRuntime {
    /// Create new DFS computation runtime
    pub fn new(source_node: u32, track_paths: bool, concurrency: usize) -> Self {
        Self {
            source_node,
            track_paths,
            concurrency,
            visited_nodes: HashMap::new(),
            max_depth: None,
        }
    }

    /// Initialize computation state
    ///
    /// Translation of: `DFSComputation.initialize()` (lines 76-100)
    /// This resets the internal state for a new traversal
    pub fn initialize(&mut self, source_node: u32, max_depth: Option<u32>) {
        self.source_node = source_node;
        self.max_depth = max_depth;
        self.visited_nodes.clear();
        
        // Add source node at discovery order 0
        self.visited_nodes.insert(source_node, 0);
    }

    /// Add a visited node with its discovery order
    ///
    /// Translation of: `DFSComputation.addVisitedNode()` (lines 101-125)
    /// This tracks nodes as they are discovered during traversal
    pub fn add_visited_node(&mut self, node: u32, _depth: u32) {
        // Discovery order is the current size of visited nodes
        let discovery_order = self.visited_nodes.len() as u32;
        self.visited_nodes.insert(node, discovery_order);
    }

    /// Check if a node has been visited
    ///
    /// Translation of: `DFSComputation.isVisited()` (lines 126-140)
    /// This checks the visited state of a node
    pub fn is_visited(&self, node: u32) -> bool {
        self.visited_nodes.contains_key(&node)
    }

    /// Get discovery order of a visited node
    ///
    /// Translation of: `DFSComputation.getDiscoveryOrder()` (lines 141-155)
    /// This retrieves the discovery order of a previously visited node
    pub fn get_discovery_order(&self, node: u32) -> Option<u32> {
        self.visited_nodes.get(&node).copied()
    }

    /// Get total number of visited nodes
    ///
    /// Translation of: `DFSComputation.getVisitedCount()` (lines 156-170)
    /// This returns the count of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited_nodes.len()
    }

    /// Get all visited nodes with their discovery orders
    ///
    /// Translation of: `DFSComputation.getVisitedNodes()` (lines 171-185)
    /// This returns all visited nodes as a vector of (node, discovery_order) pairs
    pub fn get_visited_nodes(&self) -> Vec<(u32, u32)> {
        self.visited_nodes.iter().map(|(&node, &order)| (node, order)).collect()
    }

    /// Check if max depth constraint is satisfied
    ///
    /// Translation of: `DFSComputation.checkMaxDepth()` (lines 186-200)
    /// This validates depth constraints during traversal
    pub fn check_max_depth(&self, current_depth: u32) -> bool {
        match self.max_depth {
            Some(max_depth) => current_depth <= max_depth,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_computation_runtime_creation() {
        let runtime = DfsComputationRuntime::new(0, true, 4);
        assert_eq!(runtime.source_node, 0);
        assert!(runtime.track_paths);
        assert_eq!(runtime.concurrency, 4);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_dfs_computation_runtime_initialization() {
        let mut runtime = DfsComputationRuntime::new(0, true, 1);
        runtime.initialize(5, Some(10));
        
        assert_eq!(runtime.source_node, 5);
        assert_eq!(runtime.max_depth, Some(10));
        assert_eq!(runtime.visited_count(), 1);
        assert!(runtime.is_visited(5));
        assert_eq!(runtime.get_discovery_order(5), Some(0));
    }

    #[test]
    fn test_dfs_computation_runtime_visited_operations() {
        let mut runtime = DfsComputationRuntime::new(0, false, 1);
        runtime.initialize(0, None);
        
        assert!(!runtime.is_visited(1));
        assert_eq!(runtime.get_discovery_order(1), None);
        
        runtime.add_visited_node(1, 1);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.get_discovery_order(1), Some(1));
        assert_eq!(runtime.visited_count(), 2);
    }

    #[test]
    fn test_dfs_computation_runtime_max_depth_check() {
        let mut runtime = DfsComputationRuntime::new(0, false, 1);
        runtime.initialize(0, Some(3));
        
        assert!(runtime.check_max_depth(0));
        assert!(runtime.check_max_depth(1));
        assert!(runtime.check_max_depth(3));
        assert!(!runtime.check_max_depth(4));
        
        runtime.initialize(0, None);
        assert!(runtime.check_max_depth(100)); // No limit
    }

    #[test]
    fn test_dfs_computation_runtime_get_visited_nodes() {
        let mut runtime = DfsComputationRuntime::new(0, false, 1);
        runtime.initialize(0, None);
        
        runtime.add_visited_node(1, 1);
        runtime.add_visited_node(2, 2);
        
        let visited = runtime.get_visited_nodes();
        assert_eq!(visited.len(), 3);
        
        // Check that all expected nodes are present
        let nodes: Vec<u32> = visited.iter().map(|(node, _)| *node).collect();
        assert!(nodes.contains(&0));
        assert!(nodes.contains(&1));
        assert!(nodes.contains(&2));
        
        // Check discovery order is sequential
        let mut orders: Vec<u32> = visited.iter().map(|(_, order)| *order).collect();
        orders.sort();
        assert_eq!(orders, vec![0, 1, 2]);
    }
}
