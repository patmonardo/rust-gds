//! Batch transformer interface for ML in GDS.
//!
//! Translated from Java GDS ml-core BatchTransformer.java.
//! This is a literal 1:1 translation following repository translation policy.

/// A functional interface for transforming batch indices.
pub trait BatchTransformer {
    /// Apply a transformation to the given index.
    fn apply(&self, index: u64) -> u64;
}

/// Identity batch transformer that returns the index unchanged.
#[derive(Copy, Clone, Debug, Default)]
pub struct IdentityBatchTransformer;

impl BatchTransformer for IdentityBatchTransformer {
    fn apply(&self, index: u64) -> u64 {
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_transformer() {
        let transformer = IdentityBatchTransformer;
        assert_eq!(transformer.apply(0), 0);
        assert_eq!(transformer.apply(42), 42);
        assert_eq!(transformer.apply(u64::MAX), u64::MAX);
    }
}
