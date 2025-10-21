//! Bellman-Ford Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.DistanceTracker`
//!
//! This module implements the "Subtle pole" of the Bellman-Ford algorithm,
//! handling ephemeral computation state and distance tracking.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Bellman-Ford Computation Runtime
///
/// Translation of: `DistanceTracker` class (lines 30-155)
/// Handles ephemeral computation state for the Bellman-Ford algorithm
pub struct BellmanFordComputationRuntime {
    /// Distances from source to each node
    distances: HashMap<u32, f64>,
    
    /// Predecessor for each node in shortest path
    predecessors: HashMap<u32, Option<u32>>,
    
    /// Path length for each node
    lengths: HashMap<u32, u32>,
    
    /// Nodes involved in negative cycles
    negative_cycle_nodes: Vec<u32>,
    
    /// Source node
    source_node: u32,
    
    /// Whether to track negative cycles
    track_negative_cycles: bool,
    
    /// Whether to track shortest paths
    track_paths: bool,
    
    /// Concurrency level
    concurrency: usize,
}

impl BellmanFordComputationRuntime {
    /// Create a new Bellman-Ford computation runtime
    ///
    /// Translation of: `DistanceTracker.create()` method (lines 36-54)
    pub fn new(
        source_node: u32,
        track_negative_cycles: bool,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            distances: HashMap::new(),
            predecessors: HashMap::new(),
            lengths: HashMap::new(),
            negative_cycle_nodes: Vec::new(),
            source_node,
            track_negative_cycles,
            track_paths,
            concurrency,
        }
    }

    /// Initialize the computation runtime
    ///
    /// Translation of: Initialization in `compute()` method (lines 100-103)
    pub fn initialize(&mut self, source_node: u32, track_negative_cycles: bool, track_paths: bool) {
        self.source_node = source_node;
        self.track_negative_cycles = track_negative_cycles;
        self.track_paths = track_paths;
        
        // Clear previous state
        self.distances.clear();
        self.predecessors.clear();
        self.lengths.clear();
        self.negative_cycle_nodes.clear();
        
        // Initialize with infinite distances
        for node_id in 0..100 { // TODO: Replace with actual graph node count
            self.distances.insert(node_id, f64::INFINITY);
            self.predecessors.insert(node_id, None);
            self.lengths.insert(node_id, u32::MAX);
        }
    }

    /// Get distance to a node
    ///
    /// Translation of: `distance()` method (lines 83-85)
    pub fn distance(&self, node_id: u32) -> f64 {
        self.distances.get(&node_id).copied().unwrap_or(f64::INFINITY)
    }

    /// Set distance to a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_distance(&mut self, node_id: u32, distance: f64) {
        self.distances.insert(node_id, distance);
    }

    /// Get predecessor of a node
    ///
    /// Translation of: `predecessor()` method (lines 87-89)
    pub fn predecessor(&self, node_id: u32) -> Option<u32> {
        self.predecessors.get(&node_id).copied().flatten()
    }

    /// Set predecessor of a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_predecessor(&mut self, node_id: u32, predecessor: Option<u32>) {
        self.predecessors.insert(node_id, predecessor);
    }

    /// Get path length to a node
    ///
    /// Translation of: `length()` method (lines 91-91)
    pub fn length(&self, node_id: u32) -> u32 {
        self.lengths.get(&node_id).copied().unwrap_or(u32::MAX)
    }

    /// Set path length to a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_length(&mut self, node_id: u32, length: u32) {
        self.lengths.insert(node_id, length);
    }

    /// Add a node to negative cycles
    ///
    /// Translation of: `processNegativeCycle()` method (lines 152-162)
    pub fn add_negative_cycle_node(&mut self, node_id: u32) {
        if self.track_negative_cycles && !self.negative_cycle_nodes.contains(&node_id) {
            self.negative_cycle_nodes.push(node_id);
        }
    }

    /// Check if there are negative cycles
    ///
    /// Translation of: `containsNegativeCycle` check (line 122)
    pub fn has_negative_cycles(&self) -> bool {
        !self.negative_cycle_nodes.is_empty()
    }

    /// Get all negative cycle nodes
    ///
    /// Translation of: `negativeCyclesVertices` usage (lines 83, 122)
    pub fn get_negative_cycle_nodes(&self) -> &[u32] {
        &self.negative_cycle_nodes
    }

    /// Compare and exchange distance (atomic operation)
    ///
    /// Translation of: `compareAndExchange()` method (lines 107-154)
    /// Simplified version without atomic operations for now
    pub fn compare_and_exchange(
        &mut self,
        node_id: u32,
        expected_distance: f64,
        new_distance: f64,
        predecessor: u32,
        length: u32,
    ) -> f64 {
        let current_distance = self.distance(node_id);
        
        if current_distance > new_distance {
            self.set_distance(node_id, new_distance);
            self.set_predecessor(node_id, Some(predecessor));
            self.set_length(node_id, length);
            expected_distance
        } else {
            // Signal unsuccessful update
            if expected_distance == 0.0 { -1.0 } else { -expected_distance }
        }
    }

    /// Get total number of nodes processed
    pub fn node_count(&self) -> usize {
        self.distances.len()
    }

    /// Get source node
    pub fn source_node(&self) -> u32 {
        self.source_node
    }

    /// Check if tracking negative cycles
    pub fn track_negative_cycles(&self) -> bool {
        self.track_negative_cycles
    }

    /// Check if tracking paths
    pub fn track_paths(&self) -> bool {
        self.track_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford_computation_runtime_initialization() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        assert_eq!(runtime.source_node(), 0);
        assert!(runtime.track_negative_cycles());
        assert!(runtime.track_paths());
        assert_eq!(runtime.distance(0), f64::INFINITY);
        assert_eq!(runtime.predecessor(0), None);
        assert_eq!(runtime.length(0), u32::MAX);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_empty_negative_cycles() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        assert!(!runtime.has_negative_cycles());
        assert!(runtime.get_negative_cycle_nodes().is_empty());
    }

    #[test]
    fn test_bellman_ford_computation_runtime_nodes_explored() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Set some distances
        runtime.set_distance(1, 5.0);
        runtime.set_distance(2, 10.0);
        
        assert_eq!(runtime.distance(1), 5.0);
        assert_eq!(runtime.distance(2), 10.0);
        assert_eq!(runtime.distance(3), f64::INFINITY);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_total_cost() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Set source distance
        runtime.set_distance(0, 0.0);
        runtime.set_predecessor(0, None);
        runtime.set_length(0, 0);
        
        assert_eq!(runtime.distance(0), 0.0);
        assert_eq!(runtime.predecessor(0), None);
        assert_eq!(runtime.length(0), 0);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_operations() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Test distance operations
        runtime.set_distance(1, 5.0);
        assert_eq!(runtime.distance(1), 5.0);
        
        // Test predecessor operations
        runtime.set_predecessor(1, Some(0));
        assert_eq!(runtime.predecessor(1), Some(0));
        
        // Test length operations
        runtime.set_length(1, 1);
        assert_eq!(runtime.length(1), 1);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_path_reconstruction() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Set up a simple path: 0 -> 1 -> 2
        runtime.set_distance(0, 0.0);
        runtime.set_predecessor(0, None);
        runtime.set_length(0, 0);
        
        runtime.set_distance(1, 5.0);
        runtime.set_predecessor(1, Some(0));
        runtime.set_length(1, 1);
        
        runtime.set_distance(2, 10.0);
        runtime.set_predecessor(2, Some(1));
        runtime.set_length(2, 2);
        
        // Test path reconstruction
        assert_eq!(runtime.predecessor(2), Some(1));
        assert_eq!(runtime.predecessor(1), Some(0));
        assert_eq!(runtime.predecessor(0), None);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_lowest_f_cost() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Set different distances
        runtime.set_distance(1, 10.0);
        runtime.set_distance(2, 5.0);
        runtime.set_distance(3, 15.0);
        
        // Find node with minimum distance
        let mut min_node = None;
        let mut min_distance = f64::INFINITY;
        
        for node_id in 1..=3 {
            let distance = runtime.distance(node_id);
            if distance < min_distance {
                min_distance = distance;
                min_node = Some(node_id);
            }
        }
        
        assert_eq!(min_node, Some(2));
        assert_eq!(min_distance, 5.0);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_negative_cycles() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Add negative cycle nodes
        runtime.add_negative_cycle_node(5);
        runtime.add_negative_cycle_node(6);
        
        assert!(runtime.has_negative_cycles());
        assert_eq!(runtime.get_negative_cycle_nodes().len(), 2);
        assert!(runtime.get_negative_cycle_nodes().contains(&5));
        assert!(runtime.get_negative_cycle_nodes().contains(&6));
    }

    #[test]
    fn test_bellman_ford_computation_runtime_compare_and_exchange() {
        let mut runtime = BellmanFordComputationRuntime::new(0, true, true, 4);
        runtime.initialize(0, true, true);
        
        // Set initial distance
        runtime.set_distance(1, 10.0);
        
        // Try to update with better distance
        let result = runtime.compare_and_exchange(1, 10.0, 5.0, 0, 1);
        assert_eq!(result, 10.0); // Should return expected distance on success
        assert_eq!(runtime.distance(1), 5.0);
        
        // Try to update with worse distance
        let result = runtime.compare_and_exchange(1, 5.0, 8.0, 0, 1);
        assert_eq!(result, -5.0); // Should return negative expected distance on failure
        assert_eq!(runtime.distance(1), 5.0); // Distance should remain unchanged
    }
}
