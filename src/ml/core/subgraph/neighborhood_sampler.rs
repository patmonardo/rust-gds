//! Neighborhood sampler for subgraphs in GDS.
//!
//! Translated from Java GDS ml-core NeighborhoodSampler.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Samples neighborhoods for graph neural network batch processing.
///
/// TODO: Full implementation requires Graph API integration.
/// This will use UniformSampler and WeightedUniformSampler to sample neighbors.
pub struct NeighborhoodSampler {
    random_seed: u64,
}

impl NeighborhoodSampler {
    /// Create a new neighborhood sampler with the given random seed.
    pub fn new(random_seed: u64) -> Self {
        Self { random_seed }
    }

    /// Get the random seed (for future implementation).
    #[allow(dead_code)]
    pub fn random_seed(&self) -> u64 {
        self.random_seed
    }
}
