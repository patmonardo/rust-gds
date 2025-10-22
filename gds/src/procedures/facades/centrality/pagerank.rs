//! PageRank Facade
//!
//! **What is it?**: Iterative algorithm computing node importance via link structure
//! **Why care?**: Models "random surfer" - nodes that many nodes link to are important
//! **Complexity**: O(k*(V + E)) where k is iterations
//! **Best for**: Finding important/authoritative nodes in networks
//!
//! ## The Random Surfer Model
//!
//! PageRank imagines a random person surfing the web:
//! - With probability `damping_factor`, they follow a random outgoing link
//! - With probability `1 - damping_factor`, they jump to a random page
//!
//! Nodes with more incoming links (and links from important nodes) get higher scores.
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .pagerank()
//!     .iterations(20)
//!     .damping_factor(0.85)
//!     .tolerance(1e-4)
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::procedures::facades::traits::{Result, CentralityScore};
use crate::procedures::facades::builder_base::{MutationResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about PageRank computation
#[derive(Debug, Clone)]
pub struct PageRankStats {
    /// Minimum PageRank score
    pub min: f64,
    /// Maximum PageRank score
    pub max: f64,
    /// Average PageRank score
    pub mean: f64,
    /// Standard deviation of scores
    pub stddev: f64,
    /// Median score (50th percentile)
    pub p50: f64,
    /// 90th percentile score
    pub p90: f64,
    /// 99th percentile score
    pub p99: f64,
    /// How many iterations actually ran
    pub iterations_ran: u32,
    /// Did algorithm converge to tolerance?
    pub converged: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// PageRank algorithm builder - fluent configuration
///
/// Use this to configure and run PageRank with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - iterations: 20
/// - damping_factor: 0.85 (traditional value from Google)
/// - tolerance: 1e-4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::centrality::PageRankBuilder;
/// let builder = PageRankBuilder::new()
///     .iterations(30)
///     .damping_factor(0.85)
///     .tolerance(1e-5);
/// ```
pub struct PageRankBuilder {
    /// Maximum iterations to run
    iterations: u32,
    /// Probability of teleporting to random node (1 - damping_factor = teleport_prob)
    damping_factor: f64,
    /// Convergence threshold
    tolerance: f64,
}

impl PageRankBuilder {
    /// Create a new PageRank builder with defaults
    ///
    /// Defaults:
    /// - iterations: 20
    /// - damping_factor: 0.85
    /// - tolerance: 1e-4
    pub fn new() -> Self {
        Self {
            iterations: 20,
            damping_factor: 0.85,
            tolerance: 1e-4,
        }
    }

    /// Set maximum iterations
    ///
    /// The algorithm will stop after this many iterations or when converged,
    /// whichever comes first.
    ///
    /// Higher values = more accurate but slower.
    /// Typical: 10-50 iterations
    pub fn iterations(mut self, n: u32) -> Self {
        self.iterations = n;
        self
    }

    /// Set damping factor (probability of following a link)
    ///
    /// Range: (0.0, 1.0)
    ///
    /// - 0.85 (default): Traditional Google PageRank value
    /// - Higher (0.95): Edges matter more, random nodes less
    /// - Lower (0.5): Random teleportation matters more
    pub fn damping_factor(mut self, d: f64) -> Self {
        self.damping_factor = d;
        self
    }

    /// Set convergence tolerance
    ///
    /// The algorithm converges when max delta between iterations < tolerance.
    ///
    /// - 1e-4 (default): Good balance
    /// - 1e-6: Very tight, slower
    /// - 1e-3: Loose, faster
    pub fn tolerance(mut self, t: f64) -> Self {
        self.tolerance = t;
        self
    }

    /// Validate configuration before execution
    fn validate(&self) -> Result<()> {
        ConfigValidator::iterations(self.iterations, "iterations")?;
        ConfigValidator::in_range(self.damping_factor, 0.01, 0.99, "damping_factor")?;
        ConfigValidator::positive(self.tolerance, "tolerance")?;
        Ok(())
    }

    /// Stream mode: Get PageRank score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// Use this when you want individual results, e.g.:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::PageRankBuilder;
    /// let builder = PageRankBuilder::new();
    /// for score in builder.stream()? {
    ///     println!("Node {} has score {}", score.node_id, score.score);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        // TODO: Call actual algorithm spec and return stream
        Ok(Box::new(std::iter::empty()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns min, max, mean, stddev, percentiles, and convergence info.
    ///
    /// Use this when you want overview statistics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::PageRankBuilder;
    /// let builder = PageRankBuilder::new();
    /// let stats = builder.stats()?;
    /// println!("Converged: {}, Iterations: {}", stats.converged, stats.iterations_ran);
    /// ```
    pub fn stats(self) -> Result<PageRankStats> {
        self.validate()?;
        // TODO: Call actual algorithm spec and compute statistics
        Ok(PageRankStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            stddev: 0.0,
            p50: 0.0,
            p90: 0.0,
            p99: 0.0,
            iterations_ran: 0,
            converged: false,
            execution_time_ms: 0,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores PageRank scores as a node property for use by other algorithms.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::PageRankBuilder;
    /// let builder = PageRankBuilder::new().damping_factor(0.85);
    /// let result = builder.mutate("pagerank")?;
    /// println!("Updated {} nodes", result.nodes_updated);
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

impl Default for PageRankBuilder {
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
    fn test_builder_defaults() {
        let builder = PageRankBuilder::new();
        assert_eq!(builder.iterations, 20);
        assert_eq!(builder.damping_factor, 0.85);
        assert_eq!(builder.tolerance, 1e-4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = PageRankBuilder::new()
            .iterations(30)
            .damping_factor(0.90)
            .tolerance(1e-5);

        assert_eq!(builder.iterations, 30);
        assert_eq!(builder.damping_factor, 0.90);
        assert_eq!(builder.tolerance, 1e-5);
    }

    #[test]
    fn test_validate_iterations() {
        let builder = PageRankBuilder::new().iterations(0);
        assert!(builder.validate().is_err()); // 0 is invalid

        let builder = PageRankBuilder::new().iterations(2_000_000);
        assert!(builder.validate().is_err()); // Too large is invalid

        let builder = PageRankBuilder::new().iterations(50);
        assert!(builder.validate().is_ok()); // 50 is valid
    }

    #[test]
    fn test_validate_damping_factor() {
        let builder = PageRankBuilder::new().damping_factor(0.0);
        assert!(builder.validate().is_err()); // 0.0 is invalid

        let builder = PageRankBuilder::new().damping_factor(1.0);
        assert!(builder.validate().is_err()); // 1.0 is invalid

        let builder = PageRankBuilder::new().damping_factor(0.85);
        assert!(builder.validate().is_ok()); // 0.85 is valid
    }

    #[test]
    fn test_validate_tolerance() {
        let builder = PageRankBuilder::new().tolerance(0.0);
        assert!(builder.validate().is_err()); // 0.0 is invalid (not positive)

        let builder = PageRankBuilder::new().tolerance(1e-4);
        assert!(builder.validate().is_ok()); // positive is valid
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = PageRankBuilder::new().iterations(0); // Invalid
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = PageRankBuilder::new().damping_factor(0.0); // Invalid
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = PageRankBuilder::new().tolerance(0.0); // Invalid
        assert!(builder.mutate("pr").is_err());
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = PageRankBuilder::new(); // Valid config
        assert!(builder.mutate("").is_err()); // But empty property name
    }

    #[test]
    fn test_mutate_accepts_valid_property() {
        let builder = PageRankBuilder::new();
        assert!(builder.mutate("pagerank").is_ok());
    }
}
