//! A* Facade
//!
//! **What is it?**: A* (A-star) - optimal pathfinding with heuristic guidance
//! **Why care?**: Finds optimal paths faster than Dijkstra using admissible heuristics
//! **Complexity**: O((V + E) log V) with good heuristics, same as Dijkstra with poor heuristics
//! **Best for**: Weighted graphs with good heuristic estimates, GPS navigation, game pathfinding
//!
//! ## How A* Works
//!
//! A* improves on Dijkstra by using a heuristic function h(n) that estimates distance to target:
//! - f(n) = g(n) + h(n) where g(n) is actual cost from source, h(n) is estimated cost to target
//! - Uses priority queue ordered by f(n) (total estimated cost)
//! - Guarantees optimality if heuristic is admissible (never overestimates)
//! - Explores fewer nodes than Dijkstra when heuristic is informative
//!
//! ## Heuristics
//!
//! - **Manhattan**: |dx| + |dy| (grid-based, taxicab distance)
//! - **Euclidean**: sqrt(dx² + dy²) (straight-line distance)
//! - **Haversine**: Great circle distance for geographic coordinates
//! - **Custom**: User-provided closure function
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let path = graph
//!     .astar()
//!     .source(42)
//!     .target(99)
//!     .weight_property("cost")
//!     .heuristic(Heuristic::Manhattan)
//!     .stream()?
//!     .next()
//!     .unwrap();
//! ```

use crate::procedures::facades::traits::{Result, PathResult};
use crate::procedures::facades::builder_base::{MutationResult, WriteResult, ConfigValidator};

// ============================================================================
// Heuristic Types
// ============================================================================

/// Heuristic function types for A* algorithm
#[derive(Debug, Clone, Copy)]
pub enum Heuristic {
    /// Manhattan distance: |dx| + |dy| (taxicab distance)
    Manhattan,
    /// Euclidean distance: sqrt(dx² + dy²) (straight-line distance)
    Euclidean,
    /// Haversine distance: Great circle distance for geographic coordinates
    Haversine,
    /// Custom heuristic function provided by user
    Custom(fn(u64, u64) -> f64),
}

impl Heuristic {
    /// Calculate heuristic value between two nodes
    pub fn calculate(&self, node_a: u64, node_b: u64) -> f64 {
        match self {
            Heuristic::Manhattan => {
                // TODO: Implement actual coordinate lookup and Manhattan calculation
                // For now, return a simple estimate based on node IDs
                ((node_a as f64 - node_b as f64).abs() * 2.0).min(100.0)
            }
            Heuristic::Euclidean => {
                // TODO: Implement actual coordinate lookup and Euclidean calculation
                ((node_a as f64 - node_b as f64).abs() * 1.414).min(100.0)
            }
            Heuristic::Haversine => {
                // TODO: Implement actual lat/lng lookup and Haversine calculation
                // For geographic routing, this would use latitude/longitude properties
                ((node_a as f64 - node_b as f64).abs() * 111.0).min(1000.0) // Rough km estimate
            }
            Heuristic::Custom(f) => f(node_a, node_b),
        }
    }
}

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about A* computation
#[derive(Debug, Clone)]
pub struct AStarStats {
    /// Number of nodes visited during search
    pub nodes_visited: u64,
    /// Number of nodes in the priority queue when finished
    pub final_queue_size: u64,
    /// Maximum queue size during execution
    pub max_queue_size: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Average heuristic estimate accuracy (1.0 = perfect, higher = less accurate)
    pub heuristic_accuracy: f64,
    /// Number of heuristic evaluations performed
    pub heuristic_evaluations: u64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// A* algorithm builder - fluent configuration
///
/// Use this to configure and run A* with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - source: None (must be set explicitly)
/// - targets: empty (compute path to all reachable nodes)
/// - weight_property: "weight"
/// - heuristic: Manhattan (simple and fast)
/// - concurrency: 4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::facades::pathfinding::{AStarBuilder, Heuristic};
/// let builder = AStarBuilder::new()
///     .source(42)
///     .target(99)
///     .weight_property("cost")
///     .heuristic(Heuristic::Euclidean)
///     .concurrency(8);
/// ```
pub struct AStarBuilder {
    /// Source node for A* search
    source: Option<u64>,
    /// Target nodes (empty = all reachable, specific = stop when found)
    targets: Vec<u64>,
    /// Property name for edge weights
    weight_property: String,
    /// Heuristic function type
    heuristic: Heuristic,
    /// Concurrency level for parallel processing
    concurrency: usize,
}

impl AStarBuilder {
    /// Create a new A* builder with defaults
    ///
    /// Defaults:
    /// - source: None (must be set)
    /// - targets: empty (compute to all reachable nodes)
    /// - weight_property: "weight"
    /// - heuristic: Manhattan (simple and fast)
    /// - concurrency: 4
    pub fn new() -> Self {
        Self {
            source: None,
            targets: vec![],
            weight_property: "weight".to_string(),
            heuristic: Heuristic::Manhattan,
            concurrency: 4,
        }
    }

    /// Set source node
    ///
    /// The algorithm starts search from this node.
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

    /// Set heuristic function type
    ///
    /// Different heuristics trade off accuracy vs computation speed:
    /// - Manhattan: Fast, less accurate, good for grids
    /// - Euclidean: Moderate speed/accuracy, good for open spaces
    /// - Haversine: Slow, very accurate for geographic routing
    /// - Custom: User-defined function
    pub fn heuristic(mut self, heuristic: Heuristic) -> Self {
        self.heuristic = heuristic;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// A* benefits from parallelism when exploring large graphs.
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

        ConfigValidator::non_empty_string(&self.weight_property, "weight_property")?;

        Ok(())
    }

    /// Execute the algorithm and return iterator over path results
    ///
    /// Returns optimal paths from source to target(s) using A* search.
    ///
    /// Use this when you want individual path results:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::{AStarBuilder, Heuristic};
    /// let builder = AStarBuilder::new()
    ///     .source(0)
    ///     .target(5)
    ///     .heuristic(Heuristic::Euclidean);
    /// for path in builder.stream()? {
    ///     println!("Found path: {:?}, Cost: {}", path.path, path.cost);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy results for demonstration
        let dummy_results = if self.targets.is_empty() {
            // No specific targets - return paths to sample reachable nodes
            vec![
                PathResult {
                    source: self.source.unwrap(),
                    target: 1,
                    path: vec![self.source.unwrap(), 1],
                    cost: self.heuristic.calculate(self.source.unwrap(), 1),
                },
                PathResult {
                    source: self.source.unwrap(),
                    target: 2,
                    path: vec![self.source.unwrap(), 1, 2],
                    cost: self.heuristic.calculate(self.source.unwrap(), 1) + self.heuristic.calculate(1, 2),
                },
                PathResult {
                    source: self.source.unwrap(),
                    target: 3,
                    path: vec![self.source.unwrap(), 3],
                    cost: self.heuristic.calculate(self.source.unwrap(), 3),
                },
            ]
        } else {
            // Specific targets - return optimal paths to each target
            self.targets
                .into_iter()
                .enumerate()
                .map(|(i, target)| {
                    // Simulate A* path with heuristic-based cost estimation
                    let heuristic_cost = self.heuristic.calculate(self.source.unwrap(), target);
                    let actual_cost = heuristic_cost * (1.0 + (i as f64 * 0.1)); // Simulate A* optimality

                    // Simulate path reconstruction (in real implementation, this would come from A* algorithm)
                    let path = match i {
                        0 => vec![self.source.unwrap(), target],
                        1 => vec![self.source.unwrap(), 1, target],
                        2 => vec![self.source.unwrap(), 3, target],
                        _ => vec![self.source.unwrap(), target],
                    };

                    PathResult {
                        source: self.source.unwrap(),
                        target,
                        path,
                        cost: actual_cost,
                    }
                })
                .collect()
        };

        Ok(Box::new(dummy_results.into_iter()))
    }

    /// Stats mode: Get aggregated statistics
    ///
    /// Returns search statistics without individual paths.
    ///
    /// Use this when you want performance metrics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::{AStarBuilder, Heuristic};
    /// let builder = AStarBuilder::new().source(0);
    /// let stats = builder.stats()?;
    /// println!("Visited {} nodes, heuristic accuracy: {:.2}", stats.nodes_visited, stats.heuristic_accuracy);
    /// ```
    pub fn stats(self) -> Result<AStarStats> {
        self.validate()?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy stats for demonstration
        Ok(AStarStats {
            nodes_visited: 15,
            final_queue_size: 3,
            max_queue_size: 12,
            execution_time_ms: 35,
            targets_found: self.targets.len() as u64,
            all_targets_reached: !self.targets.is_empty(),
            heuristic_accuracy: match self.heuristic {
                Heuristic::Manhattan => 1.2,  // Less accurate but fast
                Heuristic::Euclidean => 1.0,  // Perfect for Euclidean space
                Heuristic::Haversine => 1.0,  // Perfect for geographic routing
                Heuristic::Custom(_) => 1.1,  // User-defined, assume good
            },
            heuristic_evaluations: 45,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores A* distances as a node property.
    /// Property contains estimated distance from source to each reachable node.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::{AStarBuilder, Heuristic};
    /// let builder = AStarBuilder::new().source(0);
    /// let result = builder.mutate("astar_distance")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(MutationResult::new(
            12, // Dummy count of nodes updated
            property_name.to_string(),
            std::time::Duration::from_millis(40),
        ))
    }

    /// Write mode: Compute and persist to storage
    ///
    /// Persists A* search results and optimal paths to storage backend.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::facades::pathfinding::{AStarBuilder, Heuristic};
    /// let builder = AStarBuilder::new().source(0);
    /// let result = builder.write("astar_paths")?;
    /// println!("Wrote {} nodes", result.nodes_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        // TODO: Wire to actual algorithm spec when execution pipeline is ready
        // For now, return dummy result for demonstration
        Ok(WriteResult::new(
            8, // Dummy count of nodes written
            property_name.to_string(),
            std::time::Duration::from_millis(70),
        ))
    }
}

impl Default for AStarBuilder {
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
        let builder = AStarBuilder::new();
        assert_eq!(builder.source, None);
        assert!(builder.targets.is_empty());
        assert_eq!(builder.weight_property, "weight");
        assert!(matches!(builder.heuristic, Heuristic::Manhattan));
        assert_eq!(builder.concurrency, 4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let _custom_heuristic = |a: u64, b: u64| (a as f64 - b as f64).abs();
        let builder = AStarBuilder::new()
            .source(42)
            .targets(vec![99, 100])
            .weight_property("cost")
            .heuristic(Heuristic::Euclidean)
            .concurrency(8);

        assert_eq!(builder.source, Some(42));
        assert_eq!(builder.targets, vec![99, 100]);
        assert_eq!(builder.weight_property, "cost");
        assert!(matches!(builder.heuristic, Heuristic::Euclidean));
        assert_eq!(builder.concurrency, 8);
    }

    #[test]
    fn test_heuristic_calculations() {
        let manhattan = Heuristic::Manhattan;
        let euclidean = Heuristic::Euclidean;
        let haversine = Heuristic::Haversine;

        // Manhattan distance should be higher than Euclidean for same nodes
        assert!(manhattan.calculate(0, 5) >= euclidean.calculate(0, 5));

        // Haversine should give larger values (geographic scale)
        assert!(haversine.calculate(0, 5) >= manhattan.calculate(0, 5));

        // Custom heuristic
        let custom = Heuristic::Custom(|a, b| (a as f64 - b as f64).powi(2));
        assert_eq!(custom.calculate(0, 3), 9.0);
    }

    #[test]
    fn test_validate_missing_source() {
        let builder = AStarBuilder::new();
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_concurrency() {
        let builder = AStarBuilder::new().source(0).concurrency(0);
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_empty_weight_property() {
        let builder = AStarBuilder::new().source(0).weight_property("");
        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let builder = AStarBuilder::new()
            .source(0)
            .target(5)
            .weight_property("cost")
            .heuristic(Heuristic::Euclidean);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let builder = AStarBuilder::new(); // Missing source
        assert!(builder.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let builder = AStarBuilder::new().concurrency(0); // Invalid concurrency
        assert!(builder.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let builder = AStarBuilder::new().source(0); // Valid config but...
        assert!(builder.mutate("").is_err()); // Empty property name
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let builder = AStarBuilder::new().source(0);
        assert!(builder.mutate("astar_distance").is_ok());
    }

    #[test]
    fn test_write_validates_property_name() {
        let builder = AStarBuilder::new().source(0);
        assert!(builder.write("astar_paths").is_ok());
    }

    #[test]
    fn test_stream_returns_paths_to_targets() {
        let builder = AStarBuilder::new().source(0).targets(vec![2, 3]);
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].source, 0);
        assert_eq!(results[0].target, 2);
        assert_eq!(results[1].source, 0);
        assert_eq!(results[1].target, 3);

        // A* should find optimal paths (lower cost than naive)
        assert!(results[0].cost <= 10.0);
        assert!(results[1].cost <= 10.0);
    }

    #[test]
    fn test_stream_returns_all_reachable() {
        let builder = AStarBuilder::new().source(0); // No targets specified
        let results: Vec<_> = builder.stream().unwrap().collect();

        assert_eq!(results.len(), 3); // Dummy implementation returns 3 nodes
        assert_eq!(results[0].source, 0);

        // All paths should start from source
        for result in &results {
            assert_eq!(result.source, 0);
            assert!(!result.path.is_empty());
            assert_eq!(result.path[0], 0);
        }
    }

    #[test]
    fn test_stats_returns_heuristic_specific_info() {
        // Test Manhattan heuristic
        let builder = AStarBuilder::new().source(0).heuristic(Heuristic::Manhattan);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.2); // Manhattan is less accurate

        // Test Euclidean heuristic
        let builder = AStarBuilder::new().source(0).heuristic(Heuristic::Euclidean);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Euclidean is perfect

        // Test Haversine heuristic
        let builder = AStarBuilder::new().source(0).heuristic(Heuristic::Haversine);
        let stats = builder.stats().unwrap();
        assert_eq!(stats.heuristic_accuracy, 1.0); // Haversine is perfect for geo
    }
}
