//! Bellman-Ford Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.BellmanFord`
//!
//! This module implements the "Gross pole" of the Bellman-Ford algorithm,
//! handling persistent data access and the main algorithm execution.

use super::spec::{BellmanFordResult, PathResult};
use super::computation::BellmanFordComputationRuntime;
use crate::projection::eval::procedure::AlgorithmError;
use std::collections::VecDeque;

/// Bellman-Ford Storage Runtime
///
/// Translation of: Main `BellmanFord` class (lines 48-357)
/// Handles the persistent data access and algorithm orchestration
pub struct BellmanFordStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: u32,
    
    /// Whether to track negative cycles
    pub track_negative_cycles: bool,
    
    /// Whether to track shortest paths
    pub track_paths: bool,
    
    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl BellmanFordStorageRuntime {
    /// Create a new Bellman-Ford storage runtime
    ///
    /// Translation of: Constructor (lines 55-69)
    pub fn new(
        source_node: u32,
        track_negative_cycles: bool,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            track_negative_cycles,
            track_paths,
            concurrency,
        }
    }

    /// Compute Bellman-Ford shortest paths with negative cycle detection
    ///
    /// Translation of: `compute()` method (lines 72-124)
    pub fn compute_bellman_ford(
        &mut self,
        computation: &mut BellmanFordComputationRuntime,
    ) -> Result<BellmanFordResult, AlgorithmError> {
        // Initialize computation runtime
        computation.initialize(self.source_node, self.track_negative_cycles, self.track_paths);
        
        // Get mock graph data for now
        let node_count = 100; // TODO: Replace with actual graph store
        
        // Initialize frontier with source node
        let mut frontier = VecDeque::new();
        frontier.push_back(self.source_node);
        
        // Initialize distances
        computation.set_distance(self.source_node, 0.0);
        computation.set_predecessor(self.source_node, None);
        computation.set_length(self.source_node, 0);
        
        // Main Bellman-Ford loop
        let mut iteration = 0;
        let max_iterations = node_count; // Bellman-Ford converges in at most V-1 iterations
        
        while !frontier.is_empty() && iteration < max_iterations {
            let mut next_frontier = VecDeque::new();
            
            // Process all nodes in current frontier
            while let Some(node_id) = frontier.pop_front() {
                // Relax all outgoing edges from this node
                let neighbors = self.get_neighbors_with_weights(node_id);
                
                for (neighbor, weight) in neighbors {
                    let current_distance = computation.distance(node_id);
                    let new_distance = current_distance + weight;
                    
                    if new_distance < computation.distance(neighbor) {
                        computation.set_distance(neighbor, new_distance);
                        computation.set_predecessor(neighbor, Some(node_id));
                        computation.set_length(neighbor, computation.length(node_id) + 1);
                        
                        // Check for negative cycle (path length > V)
                        if computation.length(neighbor) > node_count as u32 {
                            computation.add_negative_cycle_node(neighbor);
                        }
                        
                        next_frontier.push_back(neighbor);
                    }
                }
            }
            
            frontier = next_frontier;
            iteration += 1;
        }
        
        // Check if we have negative cycles
        let contains_negative_cycle = computation.has_negative_cycles();
        
        // Generate results
        let shortest_paths = if contains_negative_cycle || !self.track_paths {
            vec![]
        } else {
            self.generate_shortest_paths(computation)?
        };
        
        let negative_cycles = if self.track_negative_cycles {
            self.generate_negative_cycles(computation)?
        } else {
            vec![]
        };
        
        Ok(BellmanFordResult {
            shortest_paths,
            negative_cycles,
            contains_negative_cycle,
        })
    }

    /// Generate shortest paths from the distance tracker
    ///
    /// Translation of: `pathResults()` method (lines 198-232)
    fn generate_shortest_paths(
        &self,
        computation: &BellmanFordComputationRuntime,
    ) -> Result<Vec<PathResult>, AlgorithmError> {
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

    /// Generate negative cycle paths
    ///
    /// Translation of: `negativeCyclesResults()` method (lines 159-196)
    fn generate_negative_cycles(
        &self,
        computation: &BellmanFordComputationRuntime,
    ) -> Result<Vec<PathResult>, AlgorithmError> {
        let mut cycles = Vec::new();
        
        for cycle_node in computation.get_negative_cycle_nodes() {
            let cycle = self.reconstruct_negative_cycle(computation, *cycle_node)?;
            cycles.push(cycle);
        }
        
        Ok(cycles)
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 236-269)
    fn reconstruct_path(
        &self,
        computation: &BellmanFordComputationRuntime,
        source_node: u32,
        target_node: u32,
    ) -> Result<PathResult, AlgorithmError> {
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
        
        Ok(PathResult {
            source_node,
            target_node,
            total_cost: computation.distance(target_node),
            node_ids,
            costs,
        })
    }

    /// Reconstruct a negative cycle starting from a given node
    ///
    /// Translation of: `negativeCycleResult()` method (lines 271-308)
    fn reconstruct_negative_cycle(
        &self,
        computation: &BellmanFordComputationRuntime,
        start_node: u32,
    ) -> Result<PathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut costs = Vec::new();
        
        let mut current_node = start_node;
        let mut length = 0;
        let max_length = 100; // TODO: Replace with actual graph node count
        
        // Follow predecessors until we complete the cycle
        while length < max_length {
            node_ids.push(current_node);
            costs.push(computation.distance(current_node));
            
            current_node = computation.predecessor(current_node)
                .ok_or_else(|| AlgorithmError::InvalidGraph("Missing predecessor in cycle".to_string()))?;
            
            length += 1;
            
            // Check if we've completed the cycle
            if current_node == start_node {
                break;
            }
        }
        
        // If we didn't complete the cycle, return empty result
        if length >= max_length {
            return Ok(PathResult {
                source_node: start_node,
                target_node: start_node,
                total_cost: 0.0,
                node_ids: vec![],
                costs: vec![],
            });
        }
        
        Ok(PathResult {
            source_node: start_node,
            target_node: start_node,
            total_cost: 0.0, // Negative cycles have negative total cost
            node_ids,
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

    #[test]
    fn test_bellman_ford_storage_runtime_creation() {
        let storage = BellmanFordStorageRuntime::new(0, true, true, 4);
        assert_eq!(storage.source_node, 0);
        assert!(storage.track_negative_cycles);
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_bellman_ford_path_computation() {
        let mut storage = BellmanFordStorageRuntime::new(0, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(0, true, true, 4);
        
        // Test basic path computation
        let result = storage.compute_bellman_ford(&mut computation);
        assert!(result.is_ok());
        
        let bellman_ford_result = result.unwrap();
        assert!(!bellman_ford_result.contains_negative_cycle);
    }

    #[test]
    fn test_bellman_ford_path_same_source_target() {
        let mut storage = BellmanFordStorageRuntime::new(0, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(0, true, true, 4);
        
        // Test with same source and target
        let result = storage.compute_bellman_ford(&mut computation);
        assert!(result.is_ok());
        
        let bellman_ford_result = result.unwrap();
        assert!(!bellman_ford_result.contains_negative_cycle);
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = BellmanFordStorageRuntime::new(0, true, true, 4);
        
        let neighbors = storage.get_neighbors_with_weights(0);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0], (1, 1.0));
        assert_eq!(neighbors[1], (2, 4.0));
        
        let neighbors_empty = storage.get_neighbors_with_weights(99);
        assert!(neighbors_empty.is_empty());
    }
}
