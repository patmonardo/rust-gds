//! BatchNeighbors trait for subgraph neighborhood access in GDS.
//!
//! Translated from Java GDS ml-core BatchNeighbors.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Interface for accessing neighborhood information for a batch of nodes.
///
/// Provides:
/// - Batch node IDs (nodes being processed)
/// - Neighbor access (adjacency information)
/// - Degree information (number of neighbors per node)
/// - Relationship weights (if applicable)
///
/// This is the core abstraction for graph neural network batch processing.
pub trait BatchNeighbors {
    /// Get the IDs of nodes in this batch.
    ///
    /// These are local (mapped) IDs, not original graph IDs.
    /// They form a consecutive range starting from 0.
    fn batch_ids(&self) -> &[usize];

    /// Get the size of the batch (number of nodes being processed).
    fn batch_size(&self) -> usize {
        self.batch_ids().len()
    }

    /// Get the total number of nodes in the subgraph.
    ///
    /// This includes:
    /// - All distinct nodes in batch_ids()
    /// - All distinct neighbors of those nodes
    ///
    /// This is typically larger than batch_size() because neighbors
    /// may not be in the original batch.
    fn node_count(&self) -> usize;

    /// Get the degree (number of neighbors) for a batch node.
    ///
    /// # Arguments
    /// * `batch_id` - Local ID of a node in the batch
    fn degree(&self, batch_id: usize) -> usize;

    /// Get the neighbors of a batch node.
    ///
    /// Returns a slice of local IDs representing the neighbors.
    ///
    /// # Arguments
    /// * `batch_id` - Local ID of a node in the batch
    fn neighbors(&self, batch_id: usize) -> &[usize];

    /// Get the relationship weight between two nodes.
    ///
    /// # Arguments
    /// * `src` - Local ID of the source node
    /// * `trg` - Local ID of the target node
    ///
    /// Returns the weight if the relationship exists, or a default value.
    fn relationship_weight(&self, src: usize, trg: usize) -> f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock implementation for testing the trait interface.
    struct MockBatchNeighbors {
        batch_ids: Vec<usize>,
        neighbors: Vec<Vec<usize>>,
        node_count: usize,
    }

    impl BatchNeighbors for MockBatchNeighbors {
        fn batch_ids(&self) -> &[usize] {
            &self.batch_ids
        }

        fn node_count(&self) -> usize {
            self.node_count
        }

        fn degree(&self, batch_id: usize) -> usize {
            self.neighbors[batch_id].len()
        }

        fn neighbors(&self, batch_id: usize) -> &[usize] {
            &self.neighbors[batch_id]
        }

        fn relationship_weight(&self, _src: usize, _trg: usize) -> f64 {
            1.0 // Default weight
        }
    }

    #[test]
    fn test_batch_size_default_impl() {
        let mock = MockBatchNeighbors {
            batch_ids: vec![0, 1, 2],
            neighbors: vec![vec![3, 4], vec![5], vec![6, 7, 8]],
            node_count: 9,
        };

        assert_eq!(mock.batch_size(), 3);
        assert_eq!(mock.batch_ids(), &[0, 1, 2]);
    }

    #[test]
    fn test_degree() {
        let mock = MockBatchNeighbors {
            batch_ids: vec![0, 1, 2],
            neighbors: vec![vec![3, 4], vec![5], vec![6, 7, 8]],
            node_count: 9,
        };

        assert_eq!(mock.degree(0), 2);
        assert_eq!(mock.degree(1), 1);
        assert_eq!(mock.degree(2), 3);
    }

    #[test]
    fn test_neighbors() {
        let mock = MockBatchNeighbors {
            batch_ids: vec![0, 1],
            neighbors: vec![vec![2, 3, 4], vec![5, 6]],
            node_count: 7,
        };

        assert_eq!(mock.neighbors(0), &[2, 3, 4]);
        assert_eq!(mock.neighbors(1), &[5, 6]);
    }

    #[test]
    fn test_node_count() {
        let mock = MockBatchNeighbors {
            batch_ids: vec![0, 1],
            neighbors: vec![vec![2, 3], vec![4]],
            node_count: 5, // 2 batch nodes + 3 unique neighbors
        };

        assert_eq!(mock.node_count(), 5);
    }
}
