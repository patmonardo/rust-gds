//! **Traversal Infrastructure**
//!
//! **Translation Source**: `org.neo4j.gds.paths.traverse.*`
//!
//! This module implements the core traversal infrastructure from Java GDS,
//! including ExitPredicate, Aggregator, and related utilities.

use serde::{Deserialize, Serialize};

/// Exit predicate result for traversal control
///
/// Translation of: `ExitPredicate.Result` (lines 26-40)
/// Controls the behavior of graph traversal algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitPredicateResult {
    /// Add current node to result set and visit all neighbors
    Follow,
    /// Add current node to result set and terminate traversal
    Break,
    /// Don't add node to result set, don't follow neighbors, continue with next element
    Continue,
}

/// Exit predicate for controlling traversal behavior
///
/// Translation of: `ExitPredicate.java` (lines 22-52)
/// Called once for each accepted node during traversal
pub trait ExitPredicate {
    /// Test whether to continue traversal from current node
    ///
    /// # Arguments
    /// * `source_node` - The source node
    /// * `current_node` - The current node being processed
    /// * `weight_at_source` - Total weight collected by Aggregator during traversal
    ///
    /// # Returns
    /// `ExitPredicateResult` indicating how to proceed
    fn test(&self, source_node: u32, current_node: u32, weight_at_source: f64) -> ExitPredicateResult;
}

/// Default exit predicate that follows all nodes
///
/// Translation of: `ExitPredicate.FOLLOW` (line 24)
pub struct FollowExitPredicate;

impl ExitPredicate for FollowExitPredicate {
    fn test(&self, _source_node: u32, _current_node: u32, _weight_at_source: f64) -> ExitPredicateResult {
        ExitPredicateResult::Follow
    }
}

/// Target-based exit predicate
///
/// Translation of: `TargetExitPredicate.java` (lines 24-33)
/// Terminates traversal when target nodes are reached
pub struct TargetExitPredicate {
    targets: Vec<u32>,
}

impl TargetExitPredicate {
    /// Create new target exit predicate
    pub fn new(targets: Vec<u32>) -> Self {
        Self { targets }
    }
}

impl ExitPredicate for TargetExitPredicate {
    fn test(&self, _source_node: u32, current_node: u32, _weight_at_source: f64) -> ExitPredicateResult {
        if self.targets.contains(&current_node) {
            ExitPredicateResult::Break
        } else {
            ExitPredicateResult::Follow
        }
    }
}

/// Aggregator function for computing weights during traversal
///
/// Translation of: `Aggregator.java` (lines 22-35)
/// Aggregates weight between source and current node
pub trait Aggregator {
    /// Apply aggregation function
    ///
    /// # Arguments
    /// * `source_node` - Source node
    /// * `current_node` - Current node
    /// * `weight_at_source` - Weight that has been aggregated for current node so far
    ///
    /// # Returns
    /// New weight (e.g., weight_at_source + 1.0)
    fn apply(&self, source_node: u32, current_node: u32, weight_at_source: f64) -> f64;
}

/// No aggregation aggregator
///
/// Translation of: `Aggregator.NO_AGGREGATION` (line 24)
pub struct NoAggregator;

impl Aggregator for NoAggregator {
    fn apply(&self, _source_node: u32, _current_node: u32, _weight_at_source: f64) -> f64 {
        0.0
    }
}

/// One-hop aggregator that increments weight by 1
///
/// Translation of: `OneHopAggregator.java` (lines 22-27)
pub struct OneHopAggregator;

impl Aggregator for OneHopAggregator {
    fn apply(&self, _source_node: u32, _current_node: u32, weight_at_source: f64) -> f64 {
        weight_at_source + 1.0
    }
}

/// Weight-based aggregator that adds edge weights
pub struct WeightAggregator;

impl Aggregator for WeightAggregator {
    fn apply(&self, _source_node: u32, _current_node: u32, weight_at_source: f64) -> f64 {
        // TODO: Implement actual edge weight lookup
        // For now, just increment by 1.0
        weight_at_source + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_exit_predicate() {
        let predicate = FollowExitPredicate;
        assert_eq!(predicate.test(0, 1, 1.0), ExitPredicateResult::Follow);
        assert_eq!(predicate.test(1, 2, 2.0), ExitPredicateResult::Follow);
    }

    #[test]
    fn test_target_exit_predicate() {
        let predicate = TargetExitPredicate::new(vec![3, 5]);
        
        assert_eq!(predicate.test(0, 1, 1.0), ExitPredicateResult::Follow);
        assert_eq!(predicate.test(0, 3, 2.0), ExitPredicateResult::Break);
        assert_eq!(predicate.test(0, 5, 3.0), ExitPredicateResult::Break);
    }

    #[test]
    fn test_no_aggregator() {
        let aggregator = NoAggregator;
        assert_eq!(aggregator.apply(0, 1, 5.0), 0.0);
    }

    #[test]
    fn test_one_hop_aggregator() {
        let aggregator = OneHopAggregator;
        assert_eq!(aggregator.apply(0, 1, 0.0), 1.0);
        assert_eq!(aggregator.apply(1, 2, 1.0), 2.0);
    }

    #[test]
    fn test_weight_aggregator() {
        let aggregator = WeightAggregator;
        assert_eq!(aggregator.apply(0, 1, 0.0), 1.0);
        assert_eq!(aggregator.apply(1, 2, 1.0), 2.0);
    }
}
