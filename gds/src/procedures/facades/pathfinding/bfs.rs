//! BFS Facade
//!
//! **What is it?**: Breadth-First Search - level-by-level graph traversal
//! **Why care?**: Finds shortest paths in unweighted graphs, explores systematically
//! **Complexity**: O(V + E) - visits each node and edge once
//! **Best for**: Unweighted graphs, shortest paths by edge count, connectivity analysis
//!
//! ## How BFS Works
//!
//! BFS explores a graph level by level:
//! 1. Start with source node at distance 0
//! 2. Visit all neighbors at distance 1
//! 3. Visit all neighbors of those at distance 2
//! 4. Continue until target found or all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let traversal = graph
//!     .bfs()
//!     .source(42)
//!     .max_depth(5)
//!     .track_paths(true)
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::procedures::facades::traits::{Result, PathResult};
use crate::procedures::facades::builder_base::{MutationResult, WriteResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about BFS computation
#[derive(Debug, Clone)]
pub struct BfsStats {
    /// Number of nodes visited during traversal
    pub nodes_visited: u64,
    /// Maximum depth reached during traversal
    pub max_depth_reached: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Average branching factor (neighbors per node)
    pub avg_branching_factor: f64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// BFS algorithm builder - fluent configuration
///
/// Use this to configure and run BFS with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (find all reachable nodes)
/// - max_depth: None (unlimited traversal)
/// - track_paths: false (only distances, not full paths)
/// - concurrency: 1 (BFS is typically single-threaded)
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::pathfinding::BfsBuilder;
/// let builder = BfsBuilder::new()
///     .source(42)
///     .max_depth(5)
///     .track_paths(true)
///     .targets(vec![99, 100]);
/// ```
pub struct BfsBuilder {
    /// Source node for BFS traversal
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Maximum depth to traverse (None = unlimited)
    max_depth: Option<u32>,
    /// Whether to track full paths or just distances
    track_paths: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
    /// Delta parameter for chunking (affects performance)
    delta: usize,
}

impl BfsBuilder {
    /// Create a new BFS builder with defaults
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (find all reachable nodes)
    /// - max_depth: None (unlimited traversal)
    /// - track_paths: false (only distances, not full paths)
    /// - concurrency: 1 (BFS is typically single-threaded)
    /// - delta: 64 (chunking parameter)
    pub fn new() -> Self {
        Self {
            source: None,
            targets: vec![],
            max_depth: None,
            track_paths: false,
            concurrency: 1,
            delta: 64,
        }
    }

    /// Set source node
    ///
    /// The algorithm starts traversal from this node.
    /// Must be a valid node ID in the graph.
    pub fn source(mut self, source: u64) -> Self {
        self.source = Some(source);
        self
    }

    /// Set single target node
    ///
    /// If specified, traversal stops when target is reached.
    /// If not specified, traverses all reachable nodes.
    pub fn target(mut self, target: u64) -> Self {
        self.targets = vec![target];
        self
    }

    /// Set multiple target nodes
    ///
    /// Algorithm computes traversal until all targets are found or max depth reached.
    pub fn targets(mut self, targets: Vec<u64>) -> Self {
        self.targets = targets;
        self
    }

    /// Set maximum depth to traverse
    ///
    /// Limits how far from source to explore.
    /// Useful for neighborhood analysis or performance control.
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Enable path tracking
    ///
    /// When true, results include full node sequences for each path.
    /// Slightly more memory usage but enables path reconstruction.
    pub fn track_paths(mut self, track: bool) -> Self {
        self.track_paths = track;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// BFS is typically single-threaded but can benefit from parallelism.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set delta parameter for chunking
    ///
    /// Affects internal chunking strategy.
    /// Higher values = larger chunks, better for large graphs.
    pub fn delta(mut self, delta: usize) -> Self {
        self.delta = delta;
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

        if let Some(depth) = self.max_depth {
            if depth == 0 {
                return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                    "max_depth must be > 0 or None".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Execute the algorithm and return iterator over traversal results
    ///
    /// Returns nodes in breadth-first order with their distances from source.
    ///
    /// Use this when you want individual traversal results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::BfsBuilder;
    /// let builder = BfsBuilder::new().source(0).max_depth(3);
    /// for result in builder.stream()? {
    ///     println!("Node {} at distance {}", result.target, result.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy results for demonstration
        let dummy_results = if self.targets.is_empty() {
            // No specific targets - return all reachable nodes
            vec![
                PathResult {
                    source: self.source.unwrap(),
                    target: 1,
                    path: vec![self.source.unwrap(), 1],
                    cost: 1.0,
                },
                PathResult {
                    source: self.source.unwrap(),
                    target: 2,
                    path: vec![self.source.unwrap(), 2],
                    cost: 1.0,
                },
            ]
        } else {
            // Specific targets - return paths to targets
            self.targets
                .into_iter()
                .enumerate()
                .map(|(i, target)| PathResult {
                    source: self.source.unwrap(),
                    target,
                    path: vec![self.source.unwrap(), target],
                    cost: (i + 1) as f64,
                })
                .collect()
        };

        Ok(Box::new(dummy_results.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns traversal statistics without individual nodes.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::BfsBuilder;
    /// let builder = BfsBuilder::new().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes in {}ms", stats.nodes_visited, stats.execution_time_ms);
    /// ```
    pub fn stats(self) -> Result<BfsStats> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy stats for demonstration
        Ok(BfsStats {
            nodes_visited: 10,
            max_depth_reached: 3,
            execution_time_ms: 25,
            targets_found: self.targets.len() as u64,
            all_targets_reached: !self.targets.is_empty(),
            avg_branching_factor: 2.5,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores BFS distances as a node property.
    /// Property contains distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::BfsBuilder;
    /// let builder = BfsBuilder::new().source(0);
    /// let result = builder.mutate("distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(MutationResult::new(
            8, // Dummy count of nodes updated
            property_name.to_string(),
            std::time::Duration::from_millis(30),
        ))
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists BFS traversal results and distances to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::BfsBuilder;
    /// let builder = BfsBuilder::new().source(0);
    /// let result = builder.write("bfs_results")?;
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
            std::time::Duration::from_millis(60),
        ))
    }
}

impl Default for BfsBuilder {
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
        let builder = BfsBuilder::new();
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert!(builder.max_depth.is_none());
        assert!(!builder.track_paths);
        assert_eq!(builder.concurrency, 1);
        assert_eq!(builder.delta, 64);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = BfsBuilder::new()
            .source(42)
            .targets(vec![99, 100])
            .max_depth(5)
            .track_paths(true)
            .concurrency(4)
            .delta(128);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.max_depth, Some(5));
        assert!(builder.track_paths);
        assert_eq!(builder.concurrency, 4);
        assert_eq!(builder.delta, 128);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = BfsBuilder::new();
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = BfsBuilder::new().source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_max_depth() {
        let builder = BfsBuilder::new().source(0).max_depth(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = BfsBuilder::new()
            .source(0)
            .max_depth(5)
            .track_paths(true);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = BfsBuilder::new(); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = BfsBuilder::new().concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = BfsBuilder::new().source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = BfsBuilder::new().source(0);
        assert!(builder.mutate("distance").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = BfsBuilder::new().source(0);
        assert!(builder.write("bfs_results").is_ok());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = BfsBuilder::new().source(0).targets(vec![3, 5]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 3);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 5);
    }

    #[test]
    fn test_stream_returns_all_reachable() {
        let builder = BfsBuilder::new().source(0); // No targets specified
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 2); // Dummy implementation returns 2 nodes
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 1);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 2);
    }

    #[test]
    fn test_stats_returns_aggregated_info() {
        let builder = BfsBuilder::new().source(0).targets(vec![1, 2, 3]);
        let stats = builder.stats().unwrap();

        assert_eq!(stats.nodes_visited, 10);
        assert_eq!(stats.targets_found, 3);
        assert!(stats.all_targets_reached);
        assert_eq!(stats.avg_branching_factor, 2.5);
    }
}
