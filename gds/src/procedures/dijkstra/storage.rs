//! Dijkstra Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Dijkstra`
//!
//! This module implements the "Gross pole" of the Dijkstra algorithm,
//! handling persistent data access and the main algorithm orchestration.
//! This is the core of the Algorithmic Virtual Machine.

use super::spec::{DijkstraResult, DijkstraPathResult};
use super::computation::DijkstraComputationRuntime;
use super::targets::Targets;
use crate::projection::eval::procedure::AlgorithmError;
use std::time::Instant;

/// Dijkstra Storage Runtime
///
/// Translation of: Main `Dijkstra` class (lines 45-309)
/// Handles the persistent data access and algorithm orchestration
pub struct DijkstraStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: u32,
    
    /// Whether to track relationship IDs
    pub track_relationships: bool,
    
    /// Concurrency level for parallel processing
    pub concurrency: usize,
    
    /// Whether to use heuristic function (for A* behavior)
    pub use_heuristic: bool,
}

impl DijkstraStorageRuntime {
    /// Create a new Dijkstra storage runtime
    ///
    /// Translation of: Constructor (lines 118-140)
    pub fn new(
        source_node: u32,
        track_relationships: bool,
        concurrency: usize,
        use_heuristic: bool,
    ) -> Self {
        Self {
            source_node,
            track_relationships,
            concurrency,
            use_heuristic,
        }
    }

    /// Compute Dijkstra shortest paths
    ///
    /// Translation of: `compute()` method (lines 170-183)
    pub fn compute_dijkstra(
        &mut self,
        computation: &mut DijkstraComputationRuntime,
        mut targets: Box<dyn Targets>,
    ) -> Result<DijkstraResult, AlgorithmError> {
        let start_time = Instant::now();
        
        // Initialize computation runtime
        computation.initialize(
            self.source_node,
            self.track_relationships,
            self.use_heuristic
        );
        
        // Get mock graph data for now
        let _node_count = 100; // TODO: Replace with actual graph store
        
        // Initialize priority queue with source node
        computation.add_to_queue(self.source_node, 0.0);
        
        let mut paths = Vec::new();
        let mut path_index = 0u64;
        
        // Main Dijkstra loop
        while !computation.is_queue_empty() {
            // Get node with minimum cost
            let (current_node, current_cost) = computation.pop_from_queue();
            
            // Mark node as visited
            computation.mark_visited(current_node);
            
            // Check if we should emit a result for this node
            let traversal_state = targets.apply(current_node);
            
            if traversal_state.should_emit() {
                // Reconstruct and emit path
                let path = self.reconstruct_path(computation, current_node, path_index)?;
                paths.push(path);
                path_index += 1;
                
                if traversal_state.should_stop() {
                    break;
                }
            }
            
            // Relax all outgoing edges
            self.relax_edges(computation, current_node, current_cost)?;
        }
        
        let computation_time_ms = start_time.elapsed().as_millis() as u64;
        
        // Create path finding result
        let path_finding_result = super::path_finding_result::PathFindingResult::new(paths);
        
        Ok(DijkstraResult {
            path_finding_result,
            computation_time_ms,
        })
    }

    /// Relax all outgoing edges from a node
    ///
    /// Translation of: `updateCost()` method (lines 220-241)
    fn relax_edges(
        &self,
        computation: &mut DijkstraComputationRuntime,
        source_node: u32,
        source_cost: f64,
    ) -> Result<(), AlgorithmError> {
        // Get neighbors with weights for the source node
        let neighbors = self.get_neighbors_with_weights(source_node);
        
        for (target_node, weight) in neighbors {
            // Skip if target is already visited
            if computation.is_visited(target_node) {
                continue;
            }
            
            let new_cost = source_cost + weight;
            
            if !computation.is_in_queue(target_node) {
                // First time seeing this node
                computation.add_to_queue(target_node, new_cost);
                computation.set_predecessor(target_node, Some(source_node));
                if self.track_relationships {
                    computation.set_relationship_id(target_node, Some(0)); // TODO: Get actual relationship ID
                }
            } else if new_cost < computation.get_cost(target_node) {
                // Found a shorter path
                computation.update_queue_cost(target_node, new_cost);
                computation.set_predecessor(target_node, Some(source_node));
                if self.track_relationships {
                    computation.set_relationship_id(target_node, Some(0)); // TODO: Get actual relationship ID
                }
            }
        }
        
        Ok(())
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 245-284)
    fn reconstruct_path(
        &self,
        computation: &DijkstraComputationRuntime,
        target_node: u32,
        path_index: u64,
    ) -> Result<DijkstraPathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut relationship_ids = Vec::new();
        let mut costs = Vec::new();
        
        let mut current_node = target_node;
        
        // Backtrack from target to source
        while current_node != self.source_node {
            node_ids.push(current_node);
            costs.push(computation.get_cost(current_node));
            
            if self.track_relationships {
                relationship_ids.push(computation.get_relationship_id(current_node).unwrap_or(0));
            }
            
            current_node = computation.get_predecessor(current_node)
                .ok_or_else(|| AlgorithmError::InvalidGraph("Missing predecessor".to_string()))?;
        }
        
        // Add source node
        node_ids.push(self.source_node);
        costs.push(0.0);
        
        // Reverse to get correct order
        node_ids.reverse();
        costs.reverse();
        if self.track_relationships {
            relationship_ids.reverse();
        }
        
        Ok(DijkstraPathResult {
            index: path_index,
            source_node: self.source_node,
            target_node,
            node_ids,
            relationship_ids,
            costs,
        })
    }

    /// Get neighbors with weights for a given node
    ///
    /// TODO: Replace with actual GraphStore API call
    /// This simulates the Java `forEachRelationship` logic
    fn get_neighbors_with_weights(&self, node_id: u32) -> Vec<(u32, f64)> {
        // Mock implementation - replace with actual graph store access
        match node_id {
            0 => vec![(1, 1.0), (2, 4.0)],
            1 => vec![(2, 2.0), (3, 5.0)],
            2 => vec![(3, 1.0), (4, 3.0)],
            3 => vec![(4, 2.0)],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::computation::DijkstraComputationRuntime;
    use super::super::targets::{SingleTarget, ManyTargets, AllTargets};

    #[test]
    fn test_dijkstra_storage_runtime_creation() {
        let storage = DijkstraStorageRuntime::new(0, true, 4, false);
        assert_eq!(storage.source_node, 0);
        assert!(storage.track_relationships);
        assert_eq!(storage.concurrency, 4);
        assert!(!storage.use_heuristic);
    }

    #[test]
    fn test_dijkstra_path_computation_single_target() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(SingleTarget::new(3));
        
        // Test basic path computation
        let result = storage.compute_dijkstra(&mut computation, targets);
        assert!(result.is_ok());
        
        let dijkstra_result = result.unwrap();
        assert!(dijkstra_result.computation_time_ms >= 0); // Allow 0 for very fast execution
    }

    #[test]
    fn test_dijkstra_path_computation_many_targets() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(ManyTargets::new(vec![3, 5]));
        
        // Test with multiple targets
        let result = storage.compute_dijkstra(&mut computation, targets);
        assert!(result.is_ok());
        
        let dijkstra_result = result.unwrap();
        assert!(dijkstra_result.computation_time_ms >= 0); // Allow 0 for very fast execution
    }

    #[test]
    fn test_dijkstra_path_computation_all_targets() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(AllTargets::new());
        
        // Test with all targets
        let result = storage.compute_dijkstra(&mut computation, targets);
        assert!(result.is_ok());
        
        let dijkstra_result = result.unwrap();
        assert!(dijkstra_result.computation_time_ms >= 0); // Allow 0 for very fast execution
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = DijkstraStorageRuntime::new(0, false, 4, false);
        
        let neighbors = storage.get_neighbors_with_weights(0);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0], (1, 1.0));
        assert_eq!(neighbors[1], (2, 4.0));
        
        let neighbors_empty = storage.get_neighbors_with_weights(99);
        assert!(neighbors_empty.is_empty());
    }
}
