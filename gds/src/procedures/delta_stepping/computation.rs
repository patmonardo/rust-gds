//! Delta Stepping Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.TentativeDistances`
//!
//! This module implements the "Subtle pole" of the Delta Stepping algorithm,
//! handling ephemeral computation state and the sophisticated binning strategy
//! for efficient frontier management.

use std::collections::{HashMap, VecDeque};

/// Delta Stepping Computation Runtime
///
/// Translation of: `TentativeDistances` interface and implementations (lines 32-218)
/// Handles ephemeral computation state for the Delta Stepping algorithm
pub struct DeltaSteppingComputationRuntime {
    /// Distances from source to each node
    distances: HashMap<u32, f64>,
    
    /// Predecessor for each node in shortest path (if storing predecessors)
    predecessors: HashMap<u32, Option<u32>>,
    
    /// Bins for organizing nodes by distance ranges
    bins: Vec<VecDeque<u32>>,
    
    /// Source node
    source_node: u32,
    
    /// Delta parameter for binning strategy
    delta: f64,
    
    /// Whether to store predecessors for path reconstruction
    store_predecessors: bool,
    
    /// Concurrency level
    concurrency: usize,
}

impl DeltaSteppingComputationRuntime {
    /// Create a new Delta Stepping computation runtime
    ///
    /// Translation of: `TentativeDistances.distanceAndPredecessors()` (lines 76-100)
    pub fn new(
        source_node: u32,
        delta: f64,
        concurrency: usize,
        store_predecessors: bool,
    ) -> Self {
        Self {
            distances: HashMap::new(),
            predecessors: HashMap::new(),
            bins: Vec::new(),
            source_node,
            delta,
            store_predecessors,
            concurrency,
        }
    }

    /// Initialize the computation runtime
    ///
    /// Translation of: Initialization in `compute()` method (lines 124-125)
    pub fn initialize(&mut self, source_node: u32, delta: f64, store_predecessors: bool) {
        self.source_node = source_node;
        self.delta = delta;
        self.store_predecessors = store_predecessors;
        
        // Clear previous state
        self.distances.clear();
        self.predecessors.clear();
        self.bins.clear();
        
        // Initialize with infinite distances
        for node_id in 0..100 { // TODO: Replace with actual graph node count
            self.distances.insert(node_id, f64::INFINITY);
            if self.store_predecessors {
                self.predecessors.insert(node_id, None);
            }
        }
    }

    /// Get distance to a node
    ///
    /// Translation of: `distance()` method (lines 40, 109, 154)
    pub fn distance(&self, node_id: u32) -> f64 {
        self.distances.get(&node_id).copied().unwrap_or(f64::INFINITY)
    }

    /// Set distance to a node
    ///
    /// Translation of: `set()` method (lines 50, 119, 173)
    pub fn set_distance(&mut self, node_id: u32, distance: f64) {
        self.distances.insert(node_id, distance);
    }

    /// Get predecessor of a node
    ///
    /// Translation of: `predecessor()` method (lines 45, 114, 158)
    pub fn predecessor(&self, node_id: u32) -> Option<u32> {
        if self.store_predecessors {
            self.predecessors.get(&node_id).copied().flatten()
        } else {
            None
        }
    }

    /// Set predecessor of a node
    ///
    /// Translation of: `set()` method (lines 50, 119, 173)
    pub fn set_predecessor(&mut self, node_id: u32, predecessor: Option<u32>) {
        if self.store_predecessors {
            self.predecessors.insert(node_id, predecessor);
        }
    }

    /// Add a node to a specific bin
    ///
    /// Translation of: Bin management in `DeltaSteppingTask.relaxNode()` (lines 270-279)
    pub fn add_to_bin(&mut self, node_id: u32, bin_index: usize) {
        // Ensure we have enough bins
        while self.bins.len() <= bin_index {
            self.bins.push(VecDeque::new());
        }
        
        self.bins[bin_index].push_back(node_id);
    }

    /// Find the next non-empty bin starting from the given index
    ///
    /// Translation of: `minNonEmptyBin()` method (lines 227-234)
    pub fn find_next_non_empty_bin(&self, start_index: usize) -> usize {
        for i in start_index..self.bins.len() {
            if !self.bins[i].is_empty() {
                return i;
            }
        }
        usize::MAX // No more bins
    }

    /// Get all nodes in a specific bin
    ///
    /// Translation of: Bin access in `DeltaSteppingTask.updateFrontier()` (lines 291-303)
    pub fn get_bin_nodes(&mut self, bin_index: usize) -> Vec<u32> {
        if bin_index < self.bins.len() {
            let nodes: Vec<u32> = self.bins[bin_index].drain(..).collect();
            nodes
        } else {
            vec![]
        }
    }

    /// Compare and exchange distance (atomic operation)
    ///
    /// Translation of: `compareAndExchange()` method (lines 59, 124, 179)
    /// Simplified version without atomic operations for now
    pub fn compare_and_exchange(
        &mut self,
        node_id: u32,
        expected_distance: f64,
        new_distance: f64,
        predecessor: u32,
    ) -> f64 {
        let current_distance = self.distance(node_id);
        
        if current_distance > new_distance {
            self.set_distance(node_id, new_distance);
            if self.store_predecessors {
                self.set_predecessor(node_id, Some(predecessor));
            }
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

    /// Get delta parameter
    pub fn delta(&self) -> f64 {
        self.delta
    }

    /// Check if storing predecessors
    pub fn store_predecessors(&self) -> bool {
        self.store_predecessors
    }

    /// Get number of bins
    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }

    /// Get nodes in a specific bin (without removing them)
    pub fn peek_bin_nodes(&self, bin_index: usize) -> &VecDeque<u32> {
        if bin_index < self.bins.len() {
            &self.bins[bin_index]
        } else {
            static EMPTY: VecDeque<u32> = VecDeque::new();
            &EMPTY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_stepping_computation_runtime_initialization() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        assert_eq!(runtime.source_node(), 0);
        assert_eq!(runtime.delta(), 1.0);
        assert!(runtime.store_predecessors());
        assert_eq!(runtime.distance(0), f64::INFINITY);
        assert_eq!(runtime.predecessor(0), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_empty_bins() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        assert_eq!(runtime.bin_count(), 0);
        assert_eq!(runtime.find_next_non_empty_bin(0), usize::MAX);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_nodes_explored() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Set some distances
        runtime.set_distance(1, 5.0);
        runtime.set_distance(2, 10.0);
        
        assert_eq!(runtime.distance(1), 5.0);
        assert_eq!(runtime.distance(2), 10.0);
        assert_eq!(runtime.distance(3), f64::INFINITY);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_total_cost() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Set source distance
        runtime.set_distance(0, 0.0);
        runtime.set_predecessor(0, None);
        
        assert_eq!(runtime.distance(0), 0.0);
        assert_eq!(runtime.predecessor(0), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_operations() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Test distance operations
        runtime.set_distance(1, 5.0);
        assert_eq!(runtime.distance(1), 5.0);
        
        // Test predecessor operations
        runtime.set_predecessor(1, Some(0));
        assert_eq!(runtime.predecessor(1), Some(0));
    }

    #[test]
    fn test_delta_stepping_computation_runtime_path_reconstruction() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Set up a simple path: 0 -> 1 -> 2
        runtime.set_distance(0, 0.0);
        runtime.set_predecessor(0, None);
        
        runtime.set_distance(1, 5.0);
        runtime.set_predecessor(1, Some(0));
        
        runtime.set_distance(2, 10.0);
        runtime.set_predecessor(2, Some(1));
        
        // Test path reconstruction
        assert_eq!(runtime.predecessor(2), Some(1));
        assert_eq!(runtime.predecessor(1), Some(0));
        assert_eq!(runtime.predecessor(0), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_lowest_f_cost() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
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
    fn test_delta_stepping_computation_runtime_binning() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Add nodes to different bins
        runtime.add_to_bin(1, 0);
        runtime.add_to_bin(2, 1);
        runtime.add_to_bin(3, 0);
        
        assert_eq!(runtime.bin_count(), 2);
        assert_eq!(runtime.find_next_non_empty_bin(0), 0);
        assert_eq!(runtime.find_next_non_empty_bin(1), 1);
        assert_eq!(runtime.find_next_non_empty_bin(2), usize::MAX);
        
        // Test getting bin nodes
        let bin_0_nodes = runtime.get_bin_nodes(0);
        assert_eq!(bin_0_nodes.len(), 2);
        assert!(bin_0_nodes.contains(&1));
        assert!(bin_0_nodes.contains(&3));
        
        let bin_1_nodes = runtime.get_bin_nodes(1);
        assert_eq!(bin_1_nodes.len(), 1);
        assert_eq!(bin_1_nodes[0], 2);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_compare_and_exchange() {
        let mut runtime = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        runtime.initialize(0, 1.0, true);
        
        // Set initial distance
        runtime.set_distance(1, 10.0);
        
        // Try to update with better distance
        let result = runtime.compare_and_exchange(1, 10.0, 5.0, 0);
        assert_eq!(result, 10.0); // Should return expected distance on success
        assert_eq!(runtime.distance(1), 5.0);
        
        // Try to update with worse distance
        let result = runtime.compare_and_exchange(1, 5.0, 8.0, 0);
        assert_eq!(result, -5.0); // Should return negative expected distance on failure
        assert_eq!(runtime.distance(1), 5.0); // Distance should remain unchanged
    }
}
