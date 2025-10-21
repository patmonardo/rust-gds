//! Spanning Tree Storage Runtime - Graph Data Access and Algorithm Orchestration
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.Prim` (algorithm orchestration)
//!
//! This module implements the "Gross pole" for spanning tree algorithms,
//! handling persistent data access and orchestrating the Prim's algorithm execution.

use super::computation::{SpanningTreeComputationRuntime, SpanningTree};
use crate::projection::eval::procedure::AlgorithmError;
use std::collections::HashMap;

/// Spanning Tree Storage Runtime
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.Prim`
///
/// Implements the "Gross pole" for spanning tree algorithms, handling:
/// - Graph data access and neighbor iteration
/// - Algorithm orchestration and execution
/// - Result construction and validation
pub struct SpanningTreeStorageRuntime {
    /// Start node for the spanning tree
    pub start_node_id: u32,
    
    /// Whether to compute minimum (true) or maximum (false) spanning tree
    pub compute_minimum: bool,
    
    /// Concurrency level
    pub concurrency: usize,
}

impl SpanningTreeStorageRuntime {
    /// Creates a new SpanningTreeStorageRuntime.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - Starting node for the spanning tree
    /// * `compute_minimum` - Whether to compute minimum (true) or maximum (false) spanning tree
    /// * `concurrency` - Concurrency level
    ///
    /// # Returns
    ///
    /// A new `SpanningTreeStorageRuntime` instance.
    pub fn new(start_node_id: u32, compute_minimum: bool, concurrency: usize) -> Self {
        Self {
            start_node_id,
            compute_minimum,
            concurrency,
        }
    }
    
    /// Compute the spanning tree using Prim's algorithm.
    ///
    /// **Translation Source**: `org.neo4j.gds.spanningtree.Prim.compute()`
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    /// * `get_neighbors` - Function to get neighbors of a node
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree<F>(
        &self,
        node_count: u32,
        get_neighbors: F,
    ) -> Result<SpanningTree, AlgorithmError>
    where
        F: Fn(u32) -> Vec<(u32, f64)>,
    {
        // Create computation runtime
        let mut computation = SpanningTreeComputationRuntime::new(
            self.start_node_id,
            self.compute_minimum,
            node_count,
            self.concurrency,
        );
        
        // Initialize computation
        computation.initialize(self.start_node_id);
        
        // Main Prim's algorithm loop
        while !computation.is_queue_empty() {
            // Get next node from priority queue
            let (current_node, current_cost) = match computation.pop_from_queue() {
                Some((node, cost)) => (node, cost),
                None => break,
            };
            
            // Skip if already visited
            if computation.is_visited(current_node) {
                continue;
            }
            
            // Mark as visited
            computation.mark_visited(current_node, current_cost);
            
            // Process neighbors
            let neighbors = get_neighbors(current_node);
            for (neighbor, weight) in neighbors {
                // Skip if neighbor already visited
                if computation.is_visited(neighbor) {
                    continue;
                }
                
                // Transform weight for min/max spanning tree
                let transformed_weight = computation.transform_weight(weight);
                
                // Check if neighbor is already in queue
                let current_parent = computation.parent(neighbor);
                let current_cost_to_parent = computation.cost_to_parent(neighbor);
                
                if current_parent == -1 {
                    // Neighbor not in queue, add it
                    computation.add_to_queue(neighbor, transformed_weight, current_node);
                } else if transformed_weight < current_cost_to_parent {
                    // Better path found, update
                    computation.update_cost(neighbor, transformed_weight, current_node);
                }
            }
        }
        
        // Build and return result
        Ok(computation.build_result(node_count))
    }
    
    /// Get neighbors of a node (mock implementation for testing).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to get neighbors for
    ///
    /// # Returns
    ///
    /// A vector of (neighbor_id, weight) pairs.
    pub fn get_neighbors(&self, node_id: u32) -> Vec<(u32, f64)> {
        // Mock implementation for testing
        // In a real implementation, this would access the graph store
        match node_id {
            0 => vec![(1, 1.0), (2, 2.0), (3, 1.5)],
            1 => vec![(0, 1.0), (2, 1.5), (3, 2.5)],
            2 => vec![(0, 2.0), (1, 1.5), (3, 1.0)],
            3 => vec![(0, 1.5), (1, 2.5), (2, 1.0)],
            _ => vec![],
        }
    }
    
    /// Compute spanning tree with mock graph data.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree_mock(&self, node_count: u32) -> Result<SpanningTree, AlgorithmError> {
        self.compute_spanning_tree(node_count, |node_id| self.get_neighbors(node_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_runtime_creation() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        
        assert_eq!(runtime.start_node_id, 0);
        assert!(runtime.compute_minimum);
        assert_eq!(runtime.concurrency, 1);
    }
    
    #[test]
    fn test_storage_runtime_minimum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let result = runtime.compute_spanning_tree_mock(4).unwrap();
        
        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert!(result.total_weight() > 0.0);
        
        // Verify tree structure (all nodes should be connected)
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }
    
    #[test]
    fn test_storage_runtime_maximum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, false, 1);
        let result = runtime.compute_spanning_tree_mock(4).unwrap();
        
        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert!(result.total_weight() > 0.0);
        
        // Verify tree structure
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }
    
    #[test]
    fn test_storage_runtime_different_start_nodes() {
        let runtime1 = SpanningTreeStorageRuntime::new(0, true, 1);
        let runtime2 = SpanningTreeStorageRuntime::new(1, true, 1);
        
        let result1 = runtime1.compute_spanning_tree_mock(4).unwrap();
        let result2 = runtime2.compute_spanning_tree_mock(4).unwrap();
        
        // Both should produce valid spanning trees
        assert_eq!(result1.effective_node_count(), 4);
        assert_eq!(result2.effective_node_count(), 4);
        
        // Different start nodes should produce different trees
        assert_eq!(result1.head(0), 0);
        assert_eq!(result2.head(1), 1);
    }
    
    #[test]
    fn test_storage_runtime_edge_iteration() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let result = runtime.compute_spanning_tree_mock(4).unwrap();
        
        let mut edges = Vec::new();
        result.for_each_edge(|source, target, cost| {
            edges.push((source, target, cost));
            true
        });
        
        // Should have exactly 3 edges for a 4-node spanning tree
        assert_eq!(edges.len(), 3);
        
        // All edges should have positive costs
        for (_, _, cost) in &edges {
            assert!(*cost > 0.0);
        }
    }
    
    #[test]
    fn test_storage_runtime_empty_graph() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        
        // Mock empty graph
        let result = runtime.compute_spanning_tree(0, |_| vec![]).unwrap();
        
        assert_eq!(result.effective_node_count(), 0);
        assert_eq!(result.total_weight(), 0.0);
    }
    
    #[test]
    fn test_storage_runtime_single_node() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        
        // Mock single node graph
        let result = runtime.compute_spanning_tree(1, |_| vec![]).unwrap();
        
        assert_eq!(result.effective_node_count(), 1);
        assert_eq!(result.total_weight(), 0.0);
        assert_eq!(result.parent(0), -1); // Root has no parent
    }
}
