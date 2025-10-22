//! Common traits for algorithm facades
//!
//! Defines the contract that all algorithm facades must implement,
//! ensuring consistent API across all algorithms while allowing
//! algorithms to customize their behavior.

use crate::projection::eval::procedure::AlgorithmError;

/// Result type for facade operations
pub type Result<T> = std::result::Result<T, AlgorithmError>;

/// Alias for Result with common usage
pub type FacadeResult<T> = std::result::Result<T, AlgorithmError>;

/// Core trait for any algorithm facade
pub trait AlgorithmRunner {
    /// Algorithm name (e.g., "pagerank", "louvain")
    fn algorithm_name(&self) -> &'static str;

    /// Human-readable description
    fn description(&self) -> &'static str;
}

/// Trait for algorithms that support streaming results
/// 
/// Stream mode returns individual results for each node/component,
/// allowing processing of results as they complete or are ready.
pub trait StreamResults<T> {
    /// Execute algorithm and return an iterator over results
    fn stream(&self) -> Result<Box<dyn Iterator<Item = T>>>;
}

/// Trait for algorithms that support statistics computation
///
/// Stats mode returns aggregated statistics about the algorithm's results,
/// useful for understanding overall graph properties without individual values.
pub trait StatsResults {
    type Stats;

    /// Execute algorithm and return aggregated statistics
    fn stats(&self) -> Result<Self::Stats>;
}

/// Trait for algorithms that support mutating node properties
///
/// Mutate mode computes results and stores them as node properties in the graph,
/// enabling subsequent algorithms to use these properties as inputs.
pub trait MutateResults {
    /// Execute algorithm and store results as a node property
    fn mutate(&self, property_name: &str) -> Result<MutationStats>;
}

/// Trait for algorithms that support writing results to storage
///
/// Write mode computes results and persists them to the storage backend,
/// making them available for long-term queries and analysis.
pub trait WriteResults {
    /// Execute algorithm and persist results to storage
    fn write(&self, property_name: &str) -> Result<WriteStats>;
}

// ============================================================================
// Common Result Types
// ============================================================================

/// Statistics about a mutation operation
#[derive(Debug, Clone)]
pub struct MutationStats {
    /// Number of nodes that received the property
    pub nodes_updated: u64,
    /// Property name that was created/updated
    pub property_name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Statistics about a write operation
#[derive(Debug, Clone)]
pub struct WriteStats {
    /// Number of nodes written
    pub nodes_written: u64,
    /// Property name that was written
    pub property_name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Centrality-Specific Traits
// ============================================================================

/// Result type for centrality algorithms: (node_id, score)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CentralityScore {
    pub node_id: u64,
    pub score: f64,
}

/// Statistics aggregated from centrality algorithm results
#[derive(Debug, Clone)]
pub struct CentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
}

// ============================================================================
// Community-Specific Traits
// ============================================================================

/// Result type for community algorithms: (node_id, community_id)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommunityAssignment {
    pub node_id: u64,
    pub community_id: u64,
}

/// Statistics aggregated from community detection results
#[derive(Debug, Clone)]
pub struct CommunityStats {
    pub community_count: u64,
    pub largest_community_size: u64,
    pub modularityx1000: i64, // Modularith stored as i64 to avoid f64 issues
}

// ============================================================================
// Path Finding-Specific Traits
// ============================================================================

/// Result type for path finding algorithms
#[derive(Debug, Clone)]
pub struct PathResult {
    pub source: u64,
    pub target: u64,
    pub path: Vec<u64>,
    pub cost: f64,
}

/// Statistics about path finding execution
#[derive(Debug, Clone)]
pub struct PathStats {
    pub paths_found: u64,
    pub total_path_length: f64,
    pub avg_path_length: f64,
    pub longest_path: u64,
}
