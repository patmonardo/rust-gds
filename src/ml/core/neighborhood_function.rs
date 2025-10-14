//! Neighborhood function trait for ML-Core in GDS.
//!
//! Translated from Java GDS ml-core NeighborhoodFunction.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Functional interface for sampling neighborhoods.
///
/// This trait defines the contract for neighborhood sampling functions used in
/// graph neural networks. Given a node ID, it returns a stream of sampled neighbor IDs.
///
/// Java equivalent:
/// ```java
/// @FunctionalInterface
/// public interface NeighborhoodFunction {
///     LongStream sample(long nodeId);
/// }
/// ```
///
/// Rust uses a trait instead of a functional interface, and returns an iterator
/// instead of a stream.
pub trait NeighborhoodFunction {
    /// Sample neighbors for a given node.
    ///
    /// # Arguments
    /// * `node_id` - The node ID to sample neighbors for
    ///
    /// # Returns
    /// An iterator over sampled neighbor node IDs.
    fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_>;
}

/// Implementation of NeighborhoodFunction that returns a vector of neighbors.
///
/// This is a simple concrete implementation for testing and basic usage.
pub struct VectorNeighborhoodFunction {
    neighbors: Vec<Vec<u64>>,
}

impl VectorNeighborhoodFunction {
    pub fn new(neighbors: Vec<Vec<u64>>) -> Self {
        Self { neighbors }
    }
}

impl NeighborhoodFunction for VectorNeighborhoodFunction {
    fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
        let idx = node_id as usize;
        if idx < self.neighbors.len() {
            Box::new(self.neighbors[idx].iter().copied())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockNeighborhoodFunction {
        neighbors: Vec<Vec<u64>>,
    }

    impl NeighborhoodFunction for MockNeighborhoodFunction {
        fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
            let idx = node_id as usize;
            if idx < self.neighbors.len() {
                Box::new(self.neighbors[idx].iter().copied())
            } else {
                Box::new(std::iter::empty())
            }
        }
    }

    #[test]
    fn test_neighborhood_function_trait() {
        let mock = MockNeighborhoodFunction {
            neighbors: vec![vec![1, 2, 3], vec![4, 5], vec![]],
        };

        let neighbors_0: Vec<_> = mock.sample(0).collect();
        assert_eq!(neighbors_0, vec![1, 2, 3]);

        let neighbors_1: Vec<_> = mock.sample(1).collect();
        assert_eq!(neighbors_1, vec![4, 5]);

        let neighbors_2: Vec<_> = mock.sample(2).collect();
        assert_eq!(neighbors_2, Vec::<u64>::new());
    }

    #[test]
    fn test_vector_neighborhood_function() {
        let sampler = VectorNeighborhoodFunction::new(vec![vec![10, 20, 30], vec![40, 50], vec![]]);

        let neighbors: Vec<_> = sampler.sample(0).collect();
        assert_eq!(neighbors, vec![10, 20, 30]);

        let neighbors: Vec<_> = sampler.sample(1).collect();
        assert_eq!(neighbors, vec![40, 50]);

        let neighbors: Vec<_> = sampler.sample(2).collect();
        assert_eq!(neighbors, Vec::<u64>::new());

        // Out of bounds returns empty
        let neighbors: Vec<_> = sampler.sample(999).collect();
        assert_eq!(neighbors, Vec::<u64>::new());
    }
}
