//! DFS Facade
//!
//! **What is it?**: Depth-First Search - deep exploration before breadth expansion
//! **Why care?**: Explores entire branches before backtracking, great for topological analysis
//! **Complexity**: O(V + E) - visits each node and edge once
//! **Best for**: Topological sorting, cycle detection, connected component analysis
//!
//! ## How DFS Works
//!
//! DFS explores a graph by going as deep as possible:
//! 1. Start with source node at depth 0
//! 2. Choose one unvisited neighbor and go deeper (depth + 1)
//! 3. Continue until dead end, then backtrack to previous node
//! 4. Try other unvisited neighbors from each backtracked node
//! 5. Continue until all reachable nodes visited
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let traversal = graph
//!     .dfs()
//!     .source(42)
//!     .max_depth(10)
//!     .track_paths(true)
//!     .targets(vec![99, 100])
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::procedures::facades::traits::{Result, PathResult};
use crate::procedures::facades::builder_base::{MutationResult, WriteResult, ConfigValidator};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about DFS computation
#[derive(Debug, Clone)]
pub struct DfsStats {
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
    /// Number of backtracking operations performed
    pub backtrack_operations: u64,
    /// Average branch depth before backtracking
    pub avg_branch_depth: f64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// DFS algorithm builder - fluent configuration
///
/// Use this to configure and run DFS with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (find all reachable nodes)
/// - max_depth: None (unlimited traversal)
/// - track_paths: false (only discovery order, not full paths)
/// - concurrency: 1 (DFS is typically single-threaded)
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::pathfinding::DfsBuilder;
/// let builder = DfsBuilder::new()
///     .source(42)
///     .max_depth(10)
///     .track_paths(true)
///     .targets(vec![99, 100]);
/// ```
pub struct DfsBuilder {
    /// Source node for DFS traversal
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Maximum depth to traverse (None = unlimited)
    max_depth: Option<u32>,
    /// Whether to track full paths or just discovery order
    track_paths: bool,
    /// Concurrency level for parallel processing
    concurrency: usize,
}

impl DfsBuilder {
    /// Create a new DFS builder with defaults
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (find all reachable nodes)
    /// - max_depth: None (unlimited traversal)
    /// - track_paths: false (only discovery order, not full paths)
    /// - concurrency: 1 (DFS is typically single-threaded)
    pub fn new() -> Self {
        Self {
            source: None,
            targets: vec![],
            max_depth: None,
            track_paths: false,
            concurrency: 1,
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
    /// DFS is typically single-threaded but can benefit from parallelism
    /// for large disconnected components.
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
    /// Returns nodes in depth-first order with their discovery depths.
    ///
    /// Use this when you want individual traversal results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = DfsBuilder::new().source(0).max_depth(5);
    /// for result in builder.stream()? {
    ///     println!("Node {} at depth {}", result.target, result.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy results for demonstration
        let dummy_results = if self.targets.is_empty() {
            // No specific targets - return all reachable nodes in DFS order
            vec![
                PathResult {
                    source: self.source.unwrap(),
                    target: 1,
                    path: vec![self.source.unwrap(), 1],
                    cost: 1.0, // Depth from source
                },
                PathResult {
                    source: self.source.unwrap(),
                    target: 3,
                    path: vec![self.source.unwrap(), 1, 3],
                    cost: 2.0,
                },
                PathResult {
                    source: self.source.unwrap(),
                    target: 2,
                    path: vec![self.source.unwrap(), 2],
                    cost: 1.0,
                },
            ]
        } else {
            // Specific targets - return paths to targets in DFS order
            self.targets
                .into_iter()
                .enumerate()
                .map(|(i, target)| {
                    // Simulate DFS path discovery order
                    let path = match i {
                        0 => vec![self.source.unwrap(), target],
                        1 => vec![self.source.unwrap(), 1, target],
                        _ => vec![self.source.unwrap(), target],
                    };
                    let cost = path.len() as f64 - 1.0; // Edge count = depth
                    PathResult {
                        source: self.source.unwrap(),
                        target,
                        path,
                        cost,
                    }
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
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = DfsBuilder::new().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes, backtracked {} times", stats.nodes_visited, stats.backtrack_operations);
    /// ```
    pub fn stats(self) -> Result<DfsStats> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy stats for demonstration
        Ok(DfsStats {
            nodes_visited: 8,
            max_depth_reached: 3,
            execution_time_ms: 18,
            targets_found: self.targets.len() as u64,
            all_targets_reached: !self.targets.is_empty(),
            backtrack_operations: 5,
            avg_branch_depth: 2.2,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores DFS discovery order or depths as a node property.
    /// Property contains discovery order (1, 2, 3...) or depth from source.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = DfsBuilder::new().source(0);
    /// let result = builder.mutate("dfs_order")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(MutationResult::new(
            6, // Dummy count of nodes updated
            property_name.to_string(),
            std::time::Duration::from_millis(25),
        ))
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists DFS traversal results and discovery order to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::DfsBuilder;
    /// let builder = DfsBuilder::new().source(0);
    /// let result = builder.write("dfs_results")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(WriteResult::new(
            4, // Dummy count of nodes written
            property_name.to_string(),
            std::time::Duration::from_millis(45),
        ))
    }
}

impl Default for DfsBuilder {
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
        let builder = DfsBuilder::new();
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert!(builder.max_depth.is_none());
        assert!(!builder.track_paths);
        assert_eq!(builder.concurrency, 1);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let builder = DfsBuilder::new()
            .source(42)
            .targets(vec![99, 100])
            .max_depth(10)
            .track_paths(true)
            .concurrency(4);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.max_depth, Some(10));
        assert!(builder.track_paths);
        assert_eq!(builder.concurrency, 4);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = DfsBuilder::new();
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = DfsBuilder::new().source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_max_depth() {
        let builder = DfsBuilder::new().source(0).max_depth(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = DfsBuilder::new()
            .source(0)
            .max_depth(10)
            .track_paths(true);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = DfsBuilder::new(); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = DfsBuilder::new().concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = DfsBuilder::new().source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = DfsBuilder::new().source(0);
        assert!(builder.mutate("dfs_order").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = DfsBuilder::new().source(0);
        assert!(builder.write("dfs_results").is_ok());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = DfsBuilder::new().source(0).targets(vec![2, 3]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 2);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 3);
    }

    #[test]
    fn test_stream_returns_all_reachable() {
        let builder = DfsBuilder::new().source(0); // No targets specified
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 3); // Dummy implementation returns 3 nodes in DFS order
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 1);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 3); // DFS goes deep first
        assert_eq!(results[2].source, 0);
        assert_eq!(results[2].target, 2);
    }

    #[test]
    fn test_stats_returns_aggregated_info() {
        let builder = DfsBuilder::new().source(0).targets(vec![1, 2, 3]);
        let stats = builder.stats().unwrap();

        assert_eq!(stats.nodes_visited, 8);
        assert_eq!(stats.targets_found, 3);
        assert!(stats.all_targets_reached);
        assert_eq!(stats.backtrack_operations, 5);
        assert_eq!(stats.avg_branch_depth, 2.2);
    }
}
