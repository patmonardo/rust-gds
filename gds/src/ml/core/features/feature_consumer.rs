//! Feature consumer interface for ML in GDS.
//!
//! Translated from Java GDS ml-core FeatureConsumer.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Interface for consuming extracted features.
///
/// This corresponds to the FeatureConsumer interface in Java GDS.
pub trait FeatureConsumer {
    /// Accept a scalar feature value.
    fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64);

    /// Accept an array of feature values.
    fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]);
}

/// No-op feature consumer (corresponds to FeatureConsumer.NOOP in Java).
///
/// This implementation does nothing with the features it receives.
pub struct NoopConsumer;

impl FeatureConsumer for NoopConsumer {
    fn accept_scalar(&mut self, _node_offset: u64, _offset: usize, _value: f64) {
        // No-op
    }

    fn accept_array(&mut self, _node_offset: u64, _offset: usize, _values: &[f64]) {
        // No-op
    }
}

/// Constant for NOOP consumer (matches Java's FeatureConsumer.NOOP pattern).
pub const NOOP: NoopConsumer = NoopConsumer;
