//! Betweenness Centrality Facade
//!
//! **What is it?**: Fraction of shortest paths that pass through each node
//! **Why care?**: Identifies "bridge" nodes that connect different network regions
//! **Complexity**: O(V*(V+E)) using Brandes' algorithm - more expensive!
//! **Best for**: Finding bottlenecks and critical connectors in networks
//!
//! ## What Betweenness Means
//!
//! For each node N:
//! - For every pair of other nodes (S, T), find shortest path from S to T
//! - Count how many of those shortest paths pass through N
//! - Betweenness = (# paths through N) / (# shortest paths total)
//!
//! High betweenness = critical for network flow/communication
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .betweenness()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! let stats = graph.betweenness().stats()?;
//! println!("Max betweenness: {} (bottleneck identified)", stats.max);
//! ```

use crate::procedures::facades::traits::{Result, CentralityScore};

use crate::procedures::facades::builder_base::{MutationResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about betweenness centrality in the graph
#[derive(Debug, Clone)]
pub struct BetweennessStats {
    /// Minimum betweenness score
    pub min: f64,
    /// Maximum betweenness score
    pub max: f64,
    /// Average betweenness
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median (50th percentile)
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 99th percentile
    pub p99: f64,
    /// Number of "bridge" nodes (high betweenness > mean + stddev)
    pub bridge_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// Betweenness Centrality algorithm builder
///
/// This algorithm is more expensive than PageRank (O(V*E) vs O(V+E)).
/// For very large graphs, consider sampling strategies.
///
/// ## Note on Performance
/// - Small graphs (<1000 nodes): Should be fast
/// - Medium graphs (1K-100K nodes): May take seconds
/// - Large graphs (>100K nodes): May take minutes
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::centrality::BetweenessBuilder;
/// let builder = BetweenessBuilder::new();
/// let stats = builder.stats()?;
/// ```
pub struct BetweenessBuilder {
    // No configuration currently - algorithm is deterministic
    // Could add: sampling factor, normalization flag, etc.
}

impl BetweenessBuilder {
    /// Create a new Betweenness builder
    pub fn new() -> Self {
        Self {}
    }

    /// Validate configuration
    fn validate(&self) -> Result<()> {
        // Currently no configuration to validate
        Ok(())
    }

    /// Stream mode: Get betweenness score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// **Warning**: This algorithm is O(V*E), so streaming on large graphs
    /// may take a while. Consider computing stats instead for overview.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// for score in builder.stream()? {
    ///     if score.score > 0.1 {
    ///         println!("Bridge node: {} (betweenness: {:.4})", score.node_id, score.score);
    ///     }
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        // TODO: Call actual algorithm spec and return stream
        Ok(Box::new(std::iter::empty()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// This is the recommended way to analyze betweenness on large graphs.
    /// Returns min, max, mean, stddev, percentiles, and identifies "bridge" nodes.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// let stats = builder.stats()?;
    /// println!("Found {} bridge nodes", stats.bridge_nodes);
    /// println!("Execution took {}ms", stats.execution_time_ms);
    /// ```
    pub fn stats(self) -> Result<BetweennessStats> {
        self.validate()?;
        // TODO: Call actual algorithm spec and compute statistics
        Ok(BetweennessStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            stddev: 0.0,
            p50: 0.0,
            p90: 0.0,
            p99: 0.0,
            bridge_nodes: 0,
            execution_time_ms: 0,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores betweenness scores as a node property.
    /// Useful for follow-up analysis like identifying connectors.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// let result = builder.mutate("betweenness")?;
    /// println!("Computed and stored for {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Call actual algorithm spec and store results
        Ok(MutationResult::new(
            0,
            property_name.to_string(),
            std::time::Duration::from_millis(0),
        ))
    }
}

impl Default for BetweenessBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = BetweenessBuilder::new();
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_ok() {
        let builder = BetweenessBuilder::new();
        let result = builder.stream();
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats_ok() {
        let builder = BetweenessBuilder::new();
        let result = builder.stats();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = BetweenessBuilder::new();
        assert!(builder.mutate("").is_err());
    }

    #[test]
    fn test_mutate_accepts_valid_name() {
        let builder = BetweenessBuilder::new();
        assert!(builder.mutate("betweenness").is_ok());
    }

    #[test]
    fn test_default_builder() {
        let _builder = BetweenessBuilder::default();
        // Just verify it creates without panic
    }
}
