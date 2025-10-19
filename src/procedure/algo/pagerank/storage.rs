//! Storage Runtime for PageRank
//!
//! This module implements the **Prajna pole** of the Functor machinery.
//! It represents persistent data structures (PropertyValues, graph topology in storage).
//!
//! ## The Prajna Pole
//!
//! Storage is the **unmanifest potential** of the network:
//! - The actual edges and their weights
//! - The node degrees and structure
//! - The scores waiting to manifest as computation
//!
//! Storage is READ-ONLY during iteration (it IS the potential).
//! Computation manifests what storage holds.

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::graph::Degrees;
use crate::types::prelude::GraphStore;
use crate::types::properties::relationship::traits::RelationshipIterator;
use std::collections::HashMap;

/// Edge message: source → target score contribution
#[derive(Debug, Clone)]
pub struct EdgeMessage {
    pub target_node: usize,
    pub score_contribution: f64,
}

/// Storage Runtime for PageRank
///
/// Reads from graph storage and produces messages for computation.
/// This is the **Gross pole** (manifest in storage form).
pub struct PageRankStorageRuntime<'a, G: GraphStore> {
    graph: &'a G,
    // Cache node out-degrees (may expand with more caching)
    out_degree_cache: HashMap<usize, usize>,
}

impl<'a, G> PageRankStorageRuntime<'a, G>
where
    G: GraphStore + Degrees + RelationshipIterator,
{
    /// Create a new storage runtime
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            out_degree_cache: HashMap::new(),
        }
    }

    /// Validate that scores array is properly formed
    ///
    /// This is the **Validator** recognizing the form:
    /// "Are these scores in the right shape for this graph?"
    pub fn validate_scores(&self, scores: &[f64]) -> Result<bool, AlgorithmError> {
        if scores.len() != self.graph.node_count() {
            return Err(AlgorithmError::Execution(format!(
                "Score array size {} != node count {}",
                scores.len(),
                self.graph.node_count()
            )));
        }

        // Check no NaN or Inf
        for (i, score) in scores.iter().enumerate() {
            if !score.is_finite() {
                return Err(AlgorithmError::Execution(format!(
                    "Score at node {} is not finite: {}",
                    i, score
                )));
            }
        }

        Ok(true)
    }

    /// Extract messages from current deltas based on graph topology
    ///
    /// This is the **Projector** revealing duality:
    /// "How do storage deltas manifest as messages ready for computation?"
    ///
    /// Follows Java GDS PageRankComputation.compute():
    /// For each node with delta > 0:
    ///   For each outgoing edge (node → target) with weight w:
    ///     message = (delta * w) / out_degree[node]
    ///   Send message to target
    ///
    /// Input: deltas (not absolute scores)
    /// Output: incoming_messages[target] = list of (source, contribution) pairs
    pub fn extract_messages(
        &self,
        deltas: &[f64],
    ) -> Result<Vec<Vec<EdgeMessage>>, AlgorithmError> {
        let node_count = self.graph.node_count();
        let mut incoming_messages: Vec<Vec<EdgeMessage>> = vec![Vec::new(); node_count];

        // For each source node
        for (source_id, _delta) in deltas.iter().enumerate().take(node_count) {
            let source_mapped = source_id as u64;
            let delta = deltas[source_id];

            // Get out-degree
            let degree = self.graph.degree(source_mapped);

            // Skip if no outgoing edges
            if degree == 0 {
                continue;
            }

            // Compute message value per edge
            let contribution_per_edge = delta / degree as f64;

            // Iterate outgoing edges
            let rel_stream = self.graph.stream_relationships(source_mapped, 1.0);
            for rel_cursor in rel_stream {
                let target_id = rel_cursor.target_id() as usize;
                let edge_weight = rel_cursor.property();

                // Message = (delta / degree) * edge_weight
                let message_value = contribution_per_edge * edge_weight;

                incoming_messages[target_id].push(EdgeMessage {
                    target_node: source_id,
                    score_contribution: message_value,
                });
            }
        }

        Ok(incoming_messages)
    }

    /// Get out-degree of a node (with caching)
    pub fn get_out_degree(&mut self, node_id: usize) -> Result<usize, AlgorithmError> {
        if let Some(degree) = self.out_degree_cache.get(&node_id) {
            return Ok(*degree);
        }

        // TODO: Query graph for actual out-degree
        // For now, placeholder
        let degree = 1; // stub

        self.out_degree_cache.insert(node_id, degree);
        Ok(degree)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_validate_scores_correct_size() {
        // Test placeholder: actual test requires a GraphStore
        // This is scaffolding for future implementation
        let _scores = vec![0.1, 0.2, 0.3];
        // Once GraphStore is available:
        // let storage = PageRankStorageRuntime::new(&graph);
        // assert!(storage.validate_scores(&scores).is_ok());
    }
}
