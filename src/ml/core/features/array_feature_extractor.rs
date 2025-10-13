//! Array feature extractor interface for ML in GDS.
//!
//! Translated from Java GDS ml-core ArrayFeatureExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::FeatureExtractor;

/// Interface for array feature extractors.
pub trait ArrayFeatureExtractor: FeatureExtractor {
    /// Extract an array feature for the given node.
    fn extract(&self, node_id: u64) -> Vec<f64>;
}
