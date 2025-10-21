//! Memory Estimation - Algorithm memory requirements and estimation
//!
//! **Translation Source**: `org.neo4j.gds.mem.MemoryEstimation` and related classes
//! **Key Features**: Memory estimation, graph dimension analysis, concurrency scaling
//!
//! This module provides memory estimation capabilities for algorithms, allowing
//! users to understand memory requirements before execution.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEstimation {
    /// Estimated memory usage in bytes
    pub estimated_bytes: u64,
    /// Memory usage breakdown by component
    pub breakdown: HashMap<String, u64>,
    /// Whether this is a conservative estimate
    pub is_conservative: bool,
    /// Additional notes about the estimation
    pub notes: Vec<String>,
}

/// Graph dimensions for memory estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDimensions {
    /// Number of nodes
    pub node_count: u64,
    /// Number of relationships
    pub relationship_count: u64,
    /// Number of node labels
    pub node_label_count: u64,
    /// Number of relationship types
    pub relationship_type_count: u64,
    /// Average node property count
    pub avg_node_property_count: f64,
    /// Average relationship property count
    pub avg_relationship_property_count: f64,
}

impl GraphDimensions {
    /// Create new graph dimensions
    pub fn new(node_count: u64, relationship_count: u64) -> Self {
        Self {
            node_count,
            relationship_count,
            node_label_count: 1,
            relationship_type_count: 1,
            avg_node_property_count: 0.0,
            avg_relationship_property_count: 0.0,
        }
    }

    /// Estimate memory for a dense array of f64 values
    pub fn dense_f64_array_memory(&self) -> u64 {
        self.node_count * std::mem::size_of::<f64>() as u64
    }

    /// Estimate memory for a dense array of u32 values
    pub fn dense_u32_array_memory(&self) -> u64 {
        self.node_count * std::mem::size_of::<u32>() as u64
    }

    /// Estimate memory for a CSR (Compressed Sparse Row) representation
    pub fn csr_memory(&self) -> u64 {
        // CSR: node_count * 4 bytes (offsets) + relationship_count * 4 bytes (targets)
        self.node_count * 4 + self.relationship_count * 4
    }

    /// Estimate memory for adjacency list representation
    pub fn adjacency_list_memory(&self) -> u64 {
        // Adjacency list: relationship_count * 8 bytes (source + target)
        self.relationship_count * 8
    }
}

/// Concurrency configuration for memory estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Number of parallel threads
    pub thread_count: usize,
    /// Whether to use parallel processing
    pub use_parallel: bool,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            thread_count: num_cpus::get(),
            use_parallel: true,
        }
    }
}

/// Memory estimation engine
pub struct MemoryEstimationEngine;

impl MemoryEstimationEngine {
    /// Estimate memory for PageRank algorithm
    ///
    /// **Translation of**: `PageRankMemoryEstimation.java`
    pub fn estimate_pagerank_memory(
        dimensions: &GraphDimensions,
        concurrency: &ConcurrencyConfig,
    ) -> MemoryEstimation {
        let mut breakdown = HashMap::new();
        let mut notes = Vec::new();

        // Node values array (f64)
        let node_values_memory = dimensions.dense_f64_array_memory();
        breakdown.insert("node_values".to_string(), node_values_memory);

        // Previous node values array (f64)
        let prev_values_memory = dimensions.dense_f64_array_memory();
        breakdown.insert("prev_node_values".to_string(), prev_values_memory);

        // Graph representation (CSR)
        let graph_memory = dimensions.csr_memory();
        breakdown.insert("graph_csr".to_string(), graph_memory);

        // Thread-local accumulators
        let thread_accumulators = concurrency.thread_count as u64 * dimensions.node_count * 8; // f64
        breakdown.insert("thread_accumulators".to_string(), thread_accumulators);

        // Convergence tracking
        let convergence_memory = dimensions.node_count * 8; // f64 per node
        breakdown.insert("convergence_tracking".to_string(), convergence_memory);

        let total_memory = breakdown.values().sum();

        notes.push("Memory estimation includes graph representation".to_string());
        notes.push("Thread-local accumulators scale with concurrency".to_string());
        notes.push("Convergence tracking adds overhead".to_string());

        MemoryEstimation {
            estimated_bytes: total_memory,
            breakdown,
            is_conservative: true,
            notes,
        }
    }

    /// Estimate memory for Degree Centrality algorithm
    pub fn estimate_degree_centrality_memory(
        dimensions: &GraphDimensions,
        concurrency: &ConcurrencyConfig,
    ) -> MemoryEstimation {
        let mut breakdown = HashMap::new();
        let mut notes = Vec::new();

        // Degree values array (f64)
        let degree_values_memory = dimensions.dense_f64_array_memory();
        breakdown.insert("degree_values".to_string(), degree_values_memory);

        // Graph representation (CSR)
        let graph_memory = dimensions.csr_memory();
        breakdown.insert("graph_csr".to_string(), graph_memory);

        // Statistics tracking
        let stats_memory = 64; // Small constant for min/max/average
        breakdown.insert("statistics".to_string(), stats_memory);

        let total_memory = breakdown.values().sum();

        notes.push("Degree Centrality is memory-efficient".to_string());
        notes.push("Only requires single pass over graph".to_string());

        MemoryEstimation {
            estimated_bytes: total_memory,
            breakdown,
            is_conservative: false,
            notes,
        }
    }

    /// Estimate memory for All Shortest Paths algorithm
    pub fn estimate_all_shortest_paths_memory(
        dimensions: &GraphDimensions,
        concurrency: &ConcurrencyConfig,
    ) -> MemoryEstimation {
        let mut breakdown = HashMap::new();
        let mut notes = Vec::new();

        // Distance matrix (O(V²) - this is the killer!)
        let distance_matrix_memory = dimensions.node_count * dimensions.node_count * 8; // f64
        breakdown.insert("distance_matrix".to_string(), distance_matrix_memory);

        // Graph representation (CSR)
        let graph_memory = dimensions.csr_memory();
        breakdown.insert("graph_csr".to_string(), graph_memory);

        // Priority queue per thread (for Dijkstra)
        let priority_queue_memory = concurrency.thread_count as u64 * dimensions.node_count * 16; // Node + distance
        breakdown.insert("priority_queues".to_string(), priority_queue_memory);

        let total_memory = breakdown.values().sum();

        notes.push("⚠️ WARNING: O(V²) memory complexity!".to_string());
        notes.push("Consider streaming results for large graphs".to_string());
        notes.push("Memory scales quadratically with node count".to_string());

        MemoryEstimation {
            estimated_bytes: total_memory,
            breakdown,
            is_conservative: true,
            notes,
        }
    }

    /// Estimate memory for Sum aggregation algorithm
    pub fn estimate_sum_memory(
        dimensions: &GraphDimensions,
        concurrency: &ConcurrencyConfig,
    ) -> MemoryEstimation {
        let mut breakdown = HashMap::new();
        let mut notes = Vec::new();

        // Property values access (no storage needed)
        let property_access_memory = 0;
        breakdown.insert("property_access".to_string(), property_access_memory);

        // Accumulator variables
        let accumulator_memory = 16; // sum (f64) + count (usize)
        breakdown.insert("accumulator".to_string(), accumulator_memory);

        // Thread-local accumulators
        let thread_accumulators = concurrency.thread_count as u64 * 16;
        breakdown.insert("thread_accumulators".to_string(), thread_accumulators);

        let total_memory = breakdown.values().sum();

        notes.push("Sum is extremely memory-efficient".to_string());
        notes.push("Only requires constant memory".to_string());

        MemoryEstimation {
            estimated_bytes: total_memory,
            breakdown,
            is_conservative: false,
            notes,
        }
    }

    /// Format memory size in human-readable format
    pub fn format_memory_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;

        if bytes == 0 {
            return "0 B".to_string();
        }

        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Check if memory estimation is feasible for given constraints
    pub fn is_feasible(
        estimation: &MemoryEstimation,
        available_memory: u64,
        safety_factor: f64,
    ) -> (bool, String) {
        let required_memory = (estimation.estimated_bytes as f64 * safety_factor) as u64;
        
        if required_memory <= available_memory {
            (
                true,
                format!(
                    "✅ Feasible: {} required, {} available",
                    Self::format_memory_size(required_memory),
                    Self::format_memory_size(available_memory)
                ),
            )
        } else {
            (
                false,
                format!(
                    "❌ Not feasible: {} required, {} available",
                    Self::format_memory_size(required_memory),
                    Self::format_memory_size(available_memory)
                ),
            )
        }
    }
}

/// Memory estimation error
#[derive(Debug, thiserror::Error)]
pub enum MemoryEstimationError {
    #[error("Invalid graph dimensions: {0}")]
    InvalidDimensions(String),
    
    #[error("Invalid concurrency configuration: {0}")]
    InvalidConcurrency(String),
    
    #[error("Estimation failed: {0}")]
    EstimationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_memory_estimation() {
        let dimensions = GraphDimensions::new(1000, 5000);
        let concurrency = ConcurrencyConfig::default();
        
        let estimation = MemoryEstimationEngine::estimate_pagerank_memory(&dimensions, &concurrency);
        
        assert!(estimation.estimated_bytes > 0);
        assert!(estimation.breakdown.contains_key("node_values"));
        assert!(estimation.breakdown.contains_key("graph_csr"));
        assert!(estimation.is_conservative);
    }

    #[test]
    fn test_degree_centrality_memory_estimation() {
        let dimensions = GraphDimensions::new(1000, 5000);
        let concurrency = ConcurrencyConfig::default();
        
        let estimation = MemoryEstimationEngine::estimate_degree_centrality_memory(&dimensions, &concurrency);
        
        assert!(estimation.estimated_bytes > 0);
        assert!(estimation.breakdown.contains_key("degree_values"));
        assert!(!estimation.is_conservative); // Degree centrality is efficient
    }

    #[test]
    fn test_all_shortest_paths_memory_estimation() {
        let dimensions = GraphDimensions::new(100, 500); // Small graph
        let concurrency = ConcurrencyConfig::default();
        
        let estimation = MemoryEstimationEngine::estimate_all_shortest_paths_memory(&dimensions, &concurrency);
        
        assert!(estimation.estimated_bytes > 0);
        assert!(estimation.breakdown.contains_key("distance_matrix"));
        assert!(estimation.is_conservative);
        
        // Check that distance matrix dominates memory usage
        let distance_memory = estimation.breakdown.get("distance_matrix").unwrap();
        assert!(*distance_memory > estimation.estimated_bytes / 2);
    }

    #[test]
    fn test_memory_formatting() {
        assert_eq!(MemoryEstimationEngine::format_memory_size(0), "0 B");
        assert_eq!(MemoryEstimationEngine::format_memory_size(1024), "1.0 KB");
        assert_eq!(MemoryEstimationEngine::format_memory_size(1024 * 1024), "1.0 MB");
        assert_eq!(MemoryEstimationEngine::format_memory_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_feasibility_check() {
        let dimensions = GraphDimensions::new(1000, 5000);
        let concurrency = ConcurrencyConfig::default();
        let estimation = MemoryEstimationEngine::estimate_pagerank_memory(&dimensions, &concurrency);
        
        // Test with sufficient memory
        let (feasible, message) = MemoryEstimationEngine::is_feasible(&estimation, estimation.estimated_bytes * 2, 1.0);
        assert!(feasible);
        assert!(message.contains("✅ Feasible"));
        
        // Test with insufficient memory
        let (feasible, message) = MemoryEstimationEngine::is_feasible(&estimation, estimation.estimated_bytes / 2, 1.0);
        assert!(!feasible);
        assert!(message.contains("❌ Not feasible"));
    }
}
