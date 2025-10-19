//! SubGraph implementation for GDS.
//!
//! Translated from Java GDS ml-core SubGraph.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::BatchNeighbors;

/// SubGraph represents a sampled neighborhood subgraph for batch processing.
///
/// This is the main implementation of BatchNeighbors used in GNN training.
pub struct SubGraph {
    /// Local IDs of nodes in the input batch
    mapped_batch_node_ids: Vec<usize>,

    /// All original node IDs in the subgraph (batch + neighbors)
    original_node_ids: Vec<u64>,

    /// Adjacency list: neighbors[node_id] = [neighbor_ids...]
    neighbors: Vec<Vec<usize>>,

    /// Whether the graph has relationship weights
    weighted: bool,
}

impl SubGraph {
    /// Create a new SubGraph.
    pub fn new(
        mapped_batch_node_ids: Vec<usize>,
        original_node_ids: Vec<u64>,
        neighbors: Vec<Vec<usize>>,
        weighted: bool,
    ) -> Self {
        Self {
            mapped_batch_node_ids,
            original_node_ids,
            neighbors,
            weighted,
        }
    }

    /// Get the original node IDs for all nodes in the subgraph.
    pub fn original_node_ids(&self) -> &[u64] {
        &self.original_node_ids
    }

    /// Check if this subgraph has relationship weights.
    pub fn is_weighted(&self) -> bool {
        self.weighted
    }
}

impl BatchNeighbors for SubGraph {
    fn batch_ids(&self) -> &[usize] {
        &self.mapped_batch_node_ids
    }

    fn node_count(&self) -> usize {
        self.original_node_ids.len()
    }

    fn degree(&self, batch_id: usize) -> usize {
        self.neighbors[batch_id].len()
    }

    fn neighbors(&self, batch_id: usize) -> &[usize] {
        &self.neighbors[batch_id]
    }

    fn relationship_weight(&self, _src: usize, _trg: usize) -> f64 {
        // TODO: Implement actual weight lookup
        // For now, return default weight
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subgraph_creation() {
        let subgraph = SubGraph::new(
            vec![0, 1, 2],
            vec![10, 20, 30, 40, 50],
            vec![vec![3, 4], vec![4], vec![3]],
            false,
        );

        assert_eq!(subgraph.batch_size(), 3);
        assert_eq!(subgraph.node_count(), 5);
        assert!(!subgraph.is_weighted());
    }

    #[test]
    fn test_subgraph_batch_neighbors() {
        let subgraph = SubGraph::new(
            vec![0, 1],
            vec![100, 200, 300, 400],
            vec![vec![2, 3], vec![3]],
            false,
        );

        assert_eq!(subgraph.degree(0), 2);
        assert_eq!(subgraph.degree(1), 1);
        assert_eq!(subgraph.neighbors(0), &[2, 3]);
        assert_eq!(subgraph.neighbors(1), &[3]);
    }

    #[test]
    fn test_subgraph_original_ids() {
        let original_ids = vec![10, 20, 30, 40];
        let subgraph = SubGraph::new(
            vec![0, 1],
            original_ids.clone(),
            vec![vec![2], vec![3]],
            false,
        );

        assert_eq!(subgraph.original_node_ids(), &original_ids[..]);
    }
}
