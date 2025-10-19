//! Bias feature extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core BiasFeature.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{FeatureExtractor, ScalarFeatureExtractor};

/// Bias feature that always returns 1.0.
///
/// This corresponds to the BiasFeature class in Java GDS.
/// Used to add a bias term to feature vectors in machine learning models.
pub struct BiasFeature;

impl FeatureExtractor for BiasFeature {
    fn dimension(&self) -> usize {
        1
    }
}

impl ScalarFeatureExtractor for BiasFeature {
    fn extract(&self, _node_id: u64) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bias_feature_always_returns_one() {
        let bias = BiasFeature;
        assert_eq!(bias.extract(0), 1.0);
        assert_eq!(bias.extract(42), 1.0);
        assert_eq!(bias.extract(u64::MAX), 1.0);
    }

    #[test]
    fn test_bias_feature_dimension() {
        let bias = BiasFeature;
        assert_eq!(FeatureExtractor::dimension(&bias), 1);
    }
}
