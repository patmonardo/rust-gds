//! Scalar feature extractor interface for ML in GDS.
//!
//! Translated from Java GDS ml-core ScalarFeatureExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::FeatureExtractor;

/// Interface for extractors that produce scalar (single f64) features.
///
/// This corresponds to the ScalarFeatureExtractor interface in Java GDS.
/// Scalar extractors always have dimension = 1.
pub trait ScalarFeatureExtractor: FeatureExtractor {
    /// Extract a scalar feature value from a node.
    fn extract(&self, node_id: u64) -> f64;
}
