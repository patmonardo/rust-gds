//! WCC Storage Runtime
use crate::types::graph::Graph;
use crate::types::properties::relationship::traits::RelationshipIterator as _;
use super::computation::{WccComputationRuntime, WccComputationResult};

pub struct WccStorageRuntime {
    concurrency: usize,
}

impl WccStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    /// Compute WCC given an oriented/filtered Graph view (use undirected semantics)
    pub fn compute_wcc(
        &self,
        computation: &mut WccComputationRuntime,
        graph: &dyn Graph,
    ) -> WccComputationResult {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        // Undirected: neighbors are union of out and in targets; dedupe via Vec + sort/unique is overkill per node, so we push both directions; computation uses union-find so duplicates are harmless.
        let get_neighbors = |node: usize| -> Vec<usize> {
            let id = node as u64;
            let mut out: Vec<usize> = graph
                .stream_relationships(id as i64, fallback)
                .map(|c| c.target_id() as usize)
                .collect();
            let mut inc: Vec<usize> = graph
                .stream_inverse_relationships(id as i64, fallback)
                .map(|c| c.source_id() as usize)
                .collect();
            out.append(&mut inc);
            out
        };

        computation.compute(node_count, get_neighbors)
    }
}
