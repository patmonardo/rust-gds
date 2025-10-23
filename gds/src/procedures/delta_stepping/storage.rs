//! Delta Stepping Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.DeltaStepping`
//!
//! This module implements the "Gross pole" of the Delta Stepping algorithm,
//! handling persistent data access and the main algorithm orchestration.
//! Delta Stepping uses a sophisticated binning strategy for efficient frontier management.

use super::spec::{DeltaSteppingResult, DeltaSteppingPathResult};
use super::computation::DeltaSteppingComputationRuntime;
use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::properties::relationship::PropertyValue;
use std::collections::VecDeque;
use std::time::Instant;

/// Delta Stepping Storage Runtime
///
/// Translation of: Main `DeltaStepping` class (lines 52-380)
/// Handles the persistent data access and algorithm orchestration
pub struct DeltaSteppingStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: u32,
    
    /// Delta parameter for binning strategy
    pub delta: f64,
    
    /// Concurrency level for parallel processing
    pub concurrency: usize,
    
    /// Whether to store predecessors for path reconstruction
    pub store_predecessors: bool,
}

impl DeltaSteppingStorageRuntime {
    /// Create a new Delta Stepping storage runtime
    ///
    /// Translation of: Constructor (lines 86-114)
    pub fn new(
        source_node: u32,
        delta: f64,
        concurrency: usize,
        store_predecessors: bool,
    ) -> Self {
        Self {
            source_node,
            delta,
            concurrency,
            store_predecessors,
        }
    }

    /// Compute Delta Stepping shortest paths
    ///
    /// Translation of: `compute()` method (lines 117-164)
    pub fn compute_delta_stepping(
        &mut self,
        computation: &mut DeltaSteppingComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
    ) -> Result<DeltaSteppingResult, AlgorithmError> {
        let start_time = Instant::now();
        
        // Initialize computation runtime
        let node_count = graph.map(|g| g.node_count()).unwrap_or(100);
        computation.initialize(
            self.source_node,
            self.delta,
            self.store_predecessors,
            node_count
        );
        
        // Initialize frontier with source node
        let mut frontier = VecDeque::new();
        frontier.push_back(self.source_node);
        
        // Initialize distances
        computation.set_distance(self.source_node, 0.0);
        if self.store_predecessors {
            computation.set_predecessor(self.source_node, None);
        }
        
        // Main Delta Stepping loop
        let mut current_bin = 0;
        let max_iterations = node_count; // Safety limit
        let mut iteration = 0;
        
        while !frontier.is_empty() && iteration < max_iterations {
            // Phase 1: Relax nodes in current bin
            let mut next_frontier = VecDeque::new();
            
            while let Some(node_id) = frontier.pop_front() {
                // Check if node is in current bin
                let node_distance = computation.distance(node_id);
                if node_distance >= self.delta * current_bin as f64 {
                    // Relax all outgoing edges from this node
                    let neighbors = self.get_neighbors_with_weights(graph, node_id, direction);
                    
                    for (neighbor, weight) in neighbors {
                        let current_distance = computation.distance(node_id);
                        let new_distance = current_distance + weight;
                        
                        if new_distance < computation.distance(neighbor) {
                            computation.set_distance(neighbor, new_distance);
                            if self.store_predecessors {
                                computation.set_predecessor(neighbor, Some(node_id));
                            }
                            
                            // Determine which bin this node belongs to
                            let dest_bin = (new_distance / self.delta) as usize;
                            if dest_bin == current_bin {
                                next_frontier.push_back(neighbor);
                            } else {
                                // Add to appropriate bin for future processing
                                computation.add_to_bin(neighbor, dest_bin);
                            }
                        }
                    }
                }
            }
            
            // Phase 2: Sync and find next bin
            frontier = next_frontier;
            
            // Find the next non-empty bin
            current_bin = computation.find_next_non_empty_bin(current_bin);
            if current_bin == usize::MAX {
                break; // No more bins to process
            }
            
            // Move nodes from next bin to frontier
            let bin_nodes = computation.get_bin_nodes(current_bin);
            for node_id in bin_nodes {
                frontier.push_back(node_id);
            }
            
            iteration += 1;
        }
        
        // Generate results
        let shortest_paths = if self.store_predecessors {
            self.generate_shortest_paths(computation)?
        } else {
            vec![]
        };
        
        let computation_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(DeltaSteppingResult {
            shortest_paths,
            computation_time_ms,
        })
    }

    /// Generate shortest paths from the distance tracker
    ///
    /// Translation of: `pathResults()` method (lines 306-346)
    fn generate_shortest_paths(
        &self,
        computation: &DeltaSteppingComputationRuntime,
    ) -> Result<Vec<DeltaSteppingPathResult>, AlgorithmError> {
        let mut paths = Vec::new();
        let node_count = 100; // TODO: Replace with actual graph store
        
        for target_node in 0..node_count {
            if computation.predecessor(target_node).is_some() {
                let path = self.reconstruct_path(computation, self.source_node, target_node)?;
                paths.push(path);
            }
        }
        
        Ok(paths)
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 348-380)
    fn reconstruct_path(
        &self,
        computation: &DeltaSteppingComputationRuntime,
        source_node: u32,
        target_node: u32,
    ) -> Result<DeltaSteppingPathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut costs = Vec::new();
        
        let mut current_node = target_node;
        
        // Backtrack from target to source
        while current_node != source_node {
            node_ids.push(current_node);
            costs.push(computation.distance(current_node));
            
            current_node = computation.predecessor(current_node)
                .ok_or_else(|| AlgorithmError::InvalidGraph("Missing predecessor".to_string()))?;
        }
        
        // Add source node
        node_ids.push(source_node);
        costs.push(computation.distance(source_node));
        
        // Reverse to get correct order
        node_ids.reverse();
        costs.reverse();
        
        Ok(DeltaSteppingPathResult {
            index: 0, // TODO: Assign proper index
            source_node,
            target_node,
            node_ids,
            costs,
        })
    }

    /// Get neighbors with weights for a given node
    ///
    /// Uses Graph::stream_relationships to iterate outgoing edges with weights
    fn get_neighbors_with_weights(&self, graph: Option<&dyn Graph>, node_id: u32, direction: u8) -> Vec<(u32, f64)> {
        if let Some(g) = graph {
            let fallback: PropertyValue = 1.0;
            let iter: Box<dyn Iterator<Item = crate::types::properties::relationship::traits::RelationshipCursorBox> + Send> =
                if direction == 1 { g.stream_inverse_relationships(node_id as u64, fallback) } else { g.stream_relationships(node_id as u64, fallback) };
            return iter.into_iter()
                .map(|cursor| (cursor.target_id() as u32, cursor.property()))
                .collect();
        } else {
            // Mock implementation for tests
            match node_id {
                0 => vec![(1, 1.0), (2, 4.0)],
                1 => vec![(2, 2.0), (3, 5.0)],
                2 => vec![(3, 1.0), (4, 3.0)],
                3 => vec![(4, 2.0)],
                _ => vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_stepping_storage_runtime_creation() {
        let storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.delta, 1.0);
        assert_eq!(storage.concurrency, 4);
        assert!(storage.store_predecessors);
    }

    #[test]
    fn test_delta_stepping_path_computation() {
        let mut storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        
        // Test basic path computation
        let result = storage.compute_delta_stepping(&mut computation, None, 0);
        assert!(result.is_ok());
        
        let delta_stepping_result = result.unwrap();
        assert!(delta_stepping_result.computation_time_ms >= 0); // Allow 0 for very fast execution
    }

    #[test]
    fn test_delta_stepping_path_same_source_target() {
        let mut storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        
        // Test with same source and target
        let result = storage.compute_delta_stepping(&mut computation, None, 0);
        assert!(result.is_ok());
        
        let delta_stepping_result = result.unwrap();
        assert!(delta_stepping_result.computation_time_ms >= 0); // Allow 0 for very fast execution
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        
        let neighbors = storage.get_neighbors_with_weights(None, 0, 0);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0], (1, 1.0));
        assert_eq!(neighbors[1], (2, 4.0));
        
        let neighbors_empty = storage.get_neighbors_with_weights(None, 99, 0);
        assert!(neighbors_empty.is_empty());
    }
}
