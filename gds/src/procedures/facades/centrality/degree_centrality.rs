//! Degree Centrality Facade
//!
//! **What is it?**: Counts the number of connections (edges) each node has
//! **Why care?**: Identifies highly-connected nodes (hubs) in the network
//! **Complexity**: O(V + E) - linear in graph size
//! **Best for**: Quick identification of important nodes by connectivity
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! // Get degree scores for all nodes
//! let results = graph.degree_centrality()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! // Get statistics
//! let stats = graph.degree_centrality().stats()?;
//! println!("Average degree: {}", stats.mean);
//!
//! // Store as node property for use in other algorithms
//! graph.degree_centrality().mutate("degree")?;
//! ```

use crate::procedures::facades::traits::{Result, CentralityScore};
use crate::procedures::facades::builder_base::{MutationResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about degree distribution in the graph
#[derive(Debug, Clone)]
pub struct DegreeCentralityStats {
    /// Minimum degree found
    pub min: f64,
    /// Maximum degree found
    pub max: f64,
    /// Average degree
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median degree (50th percentile)
    pub p50: f64,
    /// 90th percentile degree
    pub p90: f64,
    /// 99th percentile degree
    pub p99: f64,
    /// Number of nodes with degree 0
    pub isolated_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Facade Type
// ============================================================================

/// DegreeCentrality algorithm facade - no configuration needed!
///
/// This is the simplest centrality algorithm: just count edges per node.
/// Use this for quick identification of hub nodes.
pub struct DegreeCentralityFacade<'a> {
    // Note: We'll add graph reference later when we integrate with Graph type
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> DegreeCentralityFacade<'a> {
    /// Create a new DegreeCentrality facade
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Stream mode: Get degree for each node
    ///
    /// Returns an iterator over (node_id, degree) tuples.
    /// Degrees are raw counts by default (not normalized).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// for (node_id, degree) in facade.stream()? {
    ///     if degree > 100.0 {
    ///         println!("Hub: node {} has {} connections", node_id, degree);
    ///     }
    /// }
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        // TODO: Call actual algorithm spec and return stream of results
        // For now, return empty iterator
        Ok(Box::new(std::iter::empty()))
    }

    /// Stats mode: Get aggregated statistics about degree distribution
    ///
    /// Returns min, max, mean, stddev, and percentiles of the degree distribution.
    /// This is useful for understanding the overall graph structure without
    /// needing to process individual scores.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let stats = facade.stats()?;
    /// println!("Graph has average degree: {}", stats.mean);
    /// println!("Max degree (highest hub): {}", stats.max);
    /// println!("Isolated nodes: {}", stats.isolated_nodes);
    /// ```
    pub fn stats(&self) -> Result<DegreeCentralityStats> {
        // TODO: Call actual algorithm spec and compute statistics
        // For now, return placeholder
        Ok(DegreeCentralityStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            stddev: 0.0,
            p50: 0.0,
            p90: 0.0,
            p99: 0.0,
            isolated_nodes: 0,
            execution_time_ms: 0,
        })
    }

    /// Mutate mode: Compute and store degree as a node property
    ///
    /// Stores the degree of each node as a property in the graph.
    /// This allows other algorithms to use the degree as input.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let result = facade.mutate("degree")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(&self, property_name: &str) -> Result<MutationResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Call actual algorithm spec and store results
        // For now, return placeholder
        Ok(MutationResult::new(
            0,
            property_name.to_string(),
            std::time::Duration::from_millis(0),
        ))
    }
}

impl<'a> Default for DegreeCentralityFacade<'a> {
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
    fn test_facade_creation() {
        let facade = DegreeCentralityFacade::new();
        // Smoke test - just verify it creates without panic
        assert!(true);
    }

    #[test]
    fn test_stream_returns_iterator() {
        let facade = DegreeCentralityFacade::new();
        let result = facade.stream();
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats_returns_valid_structure() {
        let facade = DegreeCentralityFacade::new();
        let stats = facade.stats().unwrap();
        assert_eq!(stats.min, 0.0);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = DegreeCentralityFacade::new();
        let result = facade.mutate("");
        assert!(result.is_err()); // Empty property name should fail
    }

    #[test]
    fn test_mutate_accepts_valid_property_name() {
        let facade = DegreeCentralityFacade::new();
        let result = facade.mutate("degree");
        assert!(result.is_ok());
    }
}
