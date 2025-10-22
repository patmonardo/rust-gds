//! Dijkstra Facade
//!
//! **What is it?**: Shortest path algorithm using priority queue (Dijkstra's algorithm)
//! **Why care?**: Finds optimal paths by weight, guarantees shortest path in non-negative graphs
//! **Complexity**: O((V + E) log V) with binary heap, O(V²) with simple implementation
//! **Best for**: Weighted graphs with non-negative edge weights
//!
//! ## How Dijkstra Works
//!
//! Dijkstra finds shortest paths from a source node by:
//! 1. Starting with source node distance = 0, others = infinity
//! 2. Using priority queue to always expand lowest-distance node
//! 3. Relaxing edges: `distance[v] = min(distance[v], distance[u] + weight(u,v))`
//! 4. Continuing until target reached or all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let paths = graph
//!     .dijkstra()
//!     .source(42)
//!     .target(99)
//!     .weight_property("cost")
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::procedures::facades::traits::{Result, PathResult};
use crate::procedures::facades::builder_base::{MutationResult, WriteResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about Dijkstra computation
#[derive(Debug, Clone)]
pub struct DijkstraStats {
    /// Number of paths found
    pub paths_found: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of nodes expanded during search
    pub nodes_expanded: u64,
    /// Number of edges considered
    pub edges_considered: u64,
    /// Maximum queue size during execution
    pub max_queue_size: u64,
    /// Whether search reached target(s)
    pub target_reached: bool,
}

// ============================================================================
// Builder Type
// ============================================================================

/// Dijkstra algorithm builder - fluent configuration
///
/// Use this to configure and run Dijkstra with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: 0 (must be set explicitly)
/// - targets: empty (compute all reachable paths)
/// - weight_property: "weight"
/// - direction: "outgoing"
/// - track_relationships: false
/// - concurrency: 4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::pathfinding::DijkstraBuilder;
/// let builder = DijkstraBuilder::new()
///     .source(42)
///     .target(99)
///     .weight_property("cost")
///     .direction("outgoing");
/// ```
pub struct DijkstraBuilder {
    /// Source node for path computation
    source: Option<u64>,
    /// Target nodes (empty = all targets, single = specific target)
    targets: Vec<u64>,
    /// Property name for edge weights
    weight_property: String,
    /// Direction of traversal ("incoming", "outgoing", "both")
    direction: String,
    /// Whether to track relationship IDs in results
    track_relationships: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
}

impl DijkstraBuilder {
    /// Create a new Dijkstra builder with defaults
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (compute all reachable paths)
    /// - weight_property: "weight"
    /// - direction: "outgoing"
    /// - track_relationships: false
    /// - concurrency: 4
    pub fn new() -> Self {
        Self {
            source: None,
            targets: vec![],
            weight_property: "weight".to_string(),
            direction: "outgoing".to_string(),
            track_relationships: false,
            concurrency: 4,
        }
    }

    /// Set source node
    ///
    /// The algorithm starts path computation from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    /// Set single target node
    ///
    /// If specified, algorithm stops when target is reached.
    /// If not specified, computes paths to all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.targets = vec![target];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes shortest paths to all specified targets.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.targets = targets;
        self
    }

    /// Set weight property name
    ///
    /// Property must exist on relationships and contain numeric values.
    /// Default: "weight"
    pub fn weight_property(mut self, property: &str) -> Self {
        self.weight_property = property.to_string();
        self
    }

    /// Set traversal direction
    ///
    /// Options: "incoming", "outgoing", "both"
    /// Default: "outgoing"
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Enable relationship tracking in results
    ///
    /// When true, results include relationship IDs along paths.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_relationships(mut self, track: bool) -> Self {
        self.track_relationships = track;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Higher values = faster on large graphs, but more memory.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Validate configuration before execution
    fn validate(&self) -> Result<()> {
        match self.source {
            None => return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "source node must be specified".to_string()
            )),
            Some(id) if id == u64::MAX => return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "source node ID cannot be u64::MAX".to_string()
            )),
            _ => {}
        }

        if self.concurrency == 0 {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                "concurrency must be > 0".to_string()
            ));
        }

        if !["incoming", "outgoing", "both"].contains(&self.direction.as_str()) {
            return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                format!("direction must be 'incoming', 'outgoing', or 'both', got '{}'", self.direction)
            ));
        }

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }


    /// Execute the algorithm and return iterator over path results
    ///
    /// Returns paths from source to target(s) in order of discovery.
    ///
    /// Use this when you want individual path results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DijkstraBuilder;
    /// let builder = DijkstraBuilder::new().source(0).target(5);
    /// for path in builder.stream()? {
    ///     println!("Path: {:?}, Cost: {}", path.path, path.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return a dummy path for demonstration
        let dummy_path = PathResult {
            source: self.source.unwrap(),
            target: self.targets.first().copied().unwrap_or(0),
            path: vec![self.source.unwrap(), self.targets.first().copied().unwrap_or(0)],
            cost: 42.0,
        };

        Ok(Box::new(std::iter::once(dummy_path)))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns computation statistics without individual paths.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DijkstraBuilder;
    /// let builder = DijkstraBuilder::new().source(0);
    /// let stats = builder.stats()?;
    /// println!("Found {} paths in {}ms", stats.paths_found, stats.execution_time_ms);
    /// ```
    pub fn stats(self) -> Result<DijkstraStats> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy stats for demonstration
        Ok(DijkstraStats {
            paths_found: 1,
            execution_time_ms: 42,
            nodes_expanded: 10,
            edges_considered: 25,
            max_queue_size: 8,
            target_reached: !self.targets.is_empty(),
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores shortest path distances as a node property.
    /// Property contains distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DijkstraBuilder;
    /// let builder = DijkstraBuilder::new().source(0);
    /// let result = builder.mutate("distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(MutationResult::new(
            10, // Dummy count of nodes updated
            property_name.to_string(),
            std::time::Duration::from_millis(42),
        ))
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists shortest paths and distances to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DijkstraBuilder;
    /// let builder = DijkstraBuilder::new().source(0);
    /// let result = builder.write("paths")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(WriteResult::new(
            5, // Dummy count of nodes written
            property_name.to_string(),
            std::time::Duration::from_millis(84),
        ))
    }
}

impl Default for DijkstraBuilder {
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
        let builder = DijkstraBuilder::new();
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert_eq!(builder.weight_property, "weight");
        assert_eq!(builder.direction, "outgoing");
        assert!(!builder.track_relationships);
        assert_eq!(builder.concurrency, 4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = DijkstraBuilder::new()
            .source(42)
            .target(99)
            .weight_property("cost")
            .direction("incoming")
            .track_relationships(true)
            .concurrency(8);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99]);
        assert_eq!(builder.weight_property, "cost");
        assert_eq!(builder.direction, "incoming");
        assert!(builder.track_relationships);
        assert_eq!(builder.concurrency, 8);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DijkstraBuilder::new();
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DijkstraBuilder::new().source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_direction() {
        let builder = DijkstraBuilder::new().source(0).direction("invalid");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = DijkstraBuilder::new().source(0).weight_property("");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DijkstraBuilder::new()
            .source(0)
            .target(5)
            .weight_property("cost")
            .direction("outgoing");
        assert!(builder.validate().is_ok());
    }


    #[test]
    fn test_stream_requires_validation() {
        let builder = DijkstraBuilder::new(); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = DijkstraBuilder::new().direction("invalid");
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = DijkstraBuilder::new().source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = DijkstraBuilder::new().source(0);
        assert!(builder.mutate("distance").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DijkstraBuilder::new().source(0);
        assert!(builder.write("paths").is_ok());
    }
}
