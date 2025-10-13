//! Feature extractor interface for ML in GDS.
//!
//! Translated from Java GDS ml-core FeatureExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Marker interface for feature extractors.
///
/// This corresponds to the FeatureExtractor interface in Java GDS.
/// Implementations specify how many feature dimensions they produce.
pub trait FeatureExtractor {
    /// Get the dimension (number of features) this extractor produces.
    fn dimension(&self) -> usize;
}
