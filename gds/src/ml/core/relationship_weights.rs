//! Relationship weights interface for ML-Core in GDS.
//!
//! Translated from Java GDS ml-core RelationshipWeights.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Default relationship weight value.
pub const DEFAULT_VALUE: f64 = 1.0;

/// Trait for looking up relationship weights between nodes.
///
/// This provides an abstraction for accessing edge weights in a graph,
/// allowing different implementations (graph-backed, cached, computed, etc.)
///
/// Java equivalent:
/// ```java
/// public interface RelationshipWeights {
///     double DEFAULT_VALUE = 1.0D;
///     RelationshipWeights UNWEIGHTED = (source, target, defaultValue) -> DEFAULT_VALUE;
///     
///     default double weight(long source, long target) {
///         return weight(source, target, DEFAULT_VALUE);
///     }
///     
///     double weight(long source, long target, double defaultValue);
/// }
/// ```
pub trait RelationshipWeights {
    /// Get the weight of the relationship from source to target.
    ///
    /// # Arguments
    /// * `source` - Source node ID
    /// * `target` - Target node ID
    ///
    /// # Returns
    /// The relationship weight, or DEFAULT_VALUE if not found.
    fn weight(&self, source: u64, target: u64) -> f64 {
        self.weight_with_default(source, target, DEFAULT_VALUE)
    }

    /// Get the weight of the relationship with a custom default value.
    ///
    /// # Arguments
    /// * `source` - Source node ID
    /// * `target` - Target node ID
    /// * `default_value` - Value to return if relationship doesn't exist or has no weight
    ///
    /// # Returns
    /// The relationship weight, or default_value if not found.
    fn weight_with_default(&self, source: u64, target: u64, default_value: f64) -> f64;
}

/// Unweighted graph implementation - always returns DEFAULT_VALUE.
///
/// This is the equivalent of Java's `RelationshipWeights.UNWEIGHTED`.
#[derive(Debug, Clone, Copy)]
pub struct UnweightedRelationships;

impl RelationshipWeights for UnweightedRelationships {
    fn weight_with_default(&self, _source: u64, _target: u64, _default_value: f64) -> f64 {
        DEFAULT_VALUE
    }
}

/// Constant for unweighted graphs.
pub const UNWEIGHTED: UnweightedRelationships = UnweightedRelationships;

/// Implementation of RelationshipWeights using a closure.
///
/// This allows easy creation of weight functions from lambdas.
pub struct ClosureRelationshipWeights<F>
where
    F: Fn(u64, u64, f64) -> f64,
{
    func: F,
}

impl<F> ClosureRelationshipWeights<F>
where
    F: Fn(u64, u64, f64) -> f64,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> RelationshipWeights for ClosureRelationshipWeights<F>
where
    F: Fn(u64, u64, f64) -> f64,
{
    fn weight_with_default(&self, source: u64, target: u64, default_value: f64) -> f64 {
        (self.func)(source, target, default_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unweighted() {
        let weights = UNWEIGHTED;
        assert_eq!(weights.weight(0, 1), DEFAULT_VALUE);
        assert_eq!(weights.weight(100, 200), DEFAULT_VALUE);
        assert_eq!(weights.weight_with_default(0, 1, 5.0), DEFAULT_VALUE);
    }

    #[test]
    fn test_default_weight() {
        let weights = UNWEIGHTED;
        assert_eq!(weights.weight(0, 1), 1.0);
    }

    struct MockWeights {
        multiplier: f64,
    }

    impl RelationshipWeights for MockWeights {
        fn weight_with_default(&self, source: u64, target: u64, default_value: f64) -> f64 {
            if source == target {
                default_value
            } else {
                (source + target) as f64 * self.multiplier
            }
        }
    }

    #[test]
    fn test_custom_weights() {
        let weights = MockWeights { multiplier: 2.0 };

        assert_eq!(weights.weight(1, 2), 6.0); // (1 + 2) * 2.0
        assert_eq!(weights.weight(3, 5), 16.0); // (3 + 5) * 2.0
        assert_eq!(weights.weight(5, 5), DEFAULT_VALUE); // self-loop
    }

    #[test]
    fn test_custom_default() {
        let weights = MockWeights { multiplier: 1.0 };

        assert_eq!(weights.weight_with_default(1, 1, 42.0), 42.0);
        assert_eq!(weights.weight_with_default(1, 2, 99.0), 3.0);
    }

    #[test]
    fn test_closure_weights() {
        let weights = ClosureRelationshipWeights::new(|src, tgt, _def| (src * 100 + tgt) as f64);

        assert_eq!(weights.weight(1, 2), 102.0);
        assert_eq!(weights.weight(5, 3), 503.0);
    }
}
