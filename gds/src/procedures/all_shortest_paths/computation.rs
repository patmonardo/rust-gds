//! All Shortest Paths Computation Runtime
//!
//! This module implements the **Subtle pole** of the Functor machinery for All Shortest Paths.
//! It represents ephemeral computation state (shortest path results and statistics).

use super::storage::ShortestPathResult;

/// Computation Runtime for All Shortest Paths
///
/// This is the **Subtle pole** - ephemeral computation state.
/// It manages the algorithm's shortest path computation and statistics.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral shortest path results and statistics
/// - **Functor** = the mapping between them via shortest path computation
#[derive(Debug, Clone)]
pub struct AllShortestPathsComputationRuntime {
    /// All shortest path results
    results: Vec<ShortestPathResult>,
    /// Number of nodes processed
    node_count: usize,
    /// Maximum distance found
    max_distance: f64,
    /// Minimum distance found
    min_distance: f64,
    /// Number of infinite distances (unreachable pairs)
    infinite_distances: usize,
}

impl AllShortestPathsComputationRuntime {
    /// Create a new computation runtime
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            node_count: 0,
            max_distance: 0.0,
            min_distance: f64::INFINITY,
            infinite_distances: 0,
        }
    }

    /// Add shortest path results for a source node
    ///
    /// This is the core operation of the Subtle pole.
    /// Results coming from GraphStore (Gross) are accumulated here.
    pub fn add_shortest_paths(&mut self, source_results: Vec<ShortestPathResult>) {
        for result in source_results {
            self.results.push(result.clone());
            
            // Update statistics
            if result.distance == f64::INFINITY {
                self.infinite_distances += 1;
            } else {
                if result.distance > self.max_distance {
                    self.max_distance = result.distance;
                }
                if result.distance < self.min_distance {
                    self.min_distance = result.distance;
                }
            }
        }
        
        self.node_count += 1;
    }

    /// Add a single shortest path result
    pub fn add_result(&mut self, result: ShortestPathResult) {
        self.results.push(result.clone());
        
        // Update statistics
        if result.distance == f64::INFINITY {
            self.infinite_distances += 1;
        } else {
            if result.distance > self.max_distance {
                self.max_distance = result.distance;
            }
            if result.distance < self.min_distance {
                self.min_distance = result.distance;
            }
        }
    }

    /// Get all results
    pub fn get_results(&self) -> &Vec<ShortestPathResult> {
        &self.results
    }

    /// Get results for a specific source node
    pub fn get_results_for_source(&self, source: u32) -> Vec<ShortestPathResult> {
        self.results
            .iter()
            .filter(|result| result.source == source)
            .cloned()
            .collect()
    }

    /// Get results for a specific target node
    pub fn get_results_for_target(&self, target: u32) -> Vec<ShortestPathResult> {
        self.results
            .iter()
            .filter(|result| result.target == target)
            .cloned()
            .collect()
    }

    /// Get maximum distance found
    pub fn max_distance(&self) -> f64 {
        self.max_distance
    }

    /// Get minimum distance found
    pub fn min_distance(&self) -> f64 {
        if self.min_distance == f64::INFINITY {
            0.0
        } else {
            self.min_distance
        }
    }

    /// Get number of infinite distances (unreachable pairs)
    pub fn infinite_distances(&self) -> usize {
        self.infinite_distances
    }

    /// Get number of finite distances
    pub fn finite_distances(&self) -> usize {
        self.results.len() - self.infinite_distances
    }

    /// Get total number of results
    pub fn total_results(&self) -> usize {
        self.results.len()
    }

    /// Get number of source nodes processed
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Check if graph is connected (no infinite distances)
    pub fn is_connected(&self) -> bool {
        self.infinite_distances == 0
    }

    /// Get average distance (excluding infinite distances)
    pub fn average_distance(&self) -> f64 {
        let finite_results: Vec<&ShortestPathResult> = self.results
            .iter()
            .filter(|result| result.distance != f64::INFINITY)
            .collect();
            
        if finite_results.is_empty() {
            0.0
        } else {
            let sum: f64 = finite_results.iter().map(|r| r.distance).sum();
            sum / finite_results.len() as f64
        }
    }

    /// Clear all results
    pub fn clear(&mut self) {
        self.results.clear();
        self.node_count = 0;
        self.max_distance = 0.0;
        self.min_distance = f64::INFINITY;
        self.infinite_distances = 0;
    }
}

impl Default for AllShortestPathsComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
