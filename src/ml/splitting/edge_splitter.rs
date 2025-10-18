use crate::{
    core::{graph::Graph, id_map::IdMap, relationship::RelationshipType},
    graph::builder::RelationshipsBuilder,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::Arc;

pub const POSITIVE: f64 = 1.0;
pub const RELATIONSHIP_PROPERTY: &str = "label";

/// Result of splitting edges in a graph
#[derive(Debug)]
pub struct SplitResult {
    pub selected_relationships: RelationshipsBuilder,
    pub remaining_relationships: RelationshipsBuilder,
    pub selected_rel_count: usize,
    pub remaining_rel_count: usize,
}

/// Base trait for edge splitting strategies
pub trait EdgeSplitter {
    /// Splits positive examples in the graph
    fn split_positive_examples(
        &mut self,
        graph: Arc<Graph>,
        holdout_fraction: f64,
        remaining_rel_property_key: Option<String>,
    ) -> SplitResult;

    /// Samples based on probability
    fn sample(&mut self, probability: f64) -> bool;

    /// Counts valid positive relationship candidates
    fn valid_positive_relationship_candidate_count(
        &self,
        graph: &Graph,
        is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
    ) -> usize;

    /// Performs positive sampling
    fn positive_sampling(
        &mut self,
        graph: &Graph,
        selected_rels_builder: &mut RelationshipsBuilder,
        remaining_rels_consumer: Arc<dyn Fn(i64, i64, f64) + Send + Sync>,
        selected_rel_count: &mut usize,
        remaining_rel_count: &mut usize,
        node_id: i64,
        is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
        positive_samples_remaining: &mut usize,
        candidate_edges_remaining: &mut usize,
    );
}

/// Base implementation for edge splitters
pub struct BaseEdgeSplitter {
    rng: StdRng,
    selected_relationship_type: RelationshipType,
    remaining_relationship_type: RelationshipType,
    source_nodes: Arc<IdMap>,
    target_nodes: Arc<IdMap>,
    root_nodes: Arc<IdMap>,
    concurrency: usize,
}

impl BaseEdgeSplitter {
    /// Creates a new BaseEdgeSplitter
    pub fn new(
        seed: Option<u64>,
        root_nodes: Arc<IdMap>,
        source_nodes: Arc<IdMap>,
        target_nodes: Arc<IdMap>,
        selected_relationship_type: RelationshipType,
        remaining_relationship_type: RelationshipType,
        concurrency: usize,
    ) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        Self {
            rng,
            selected_relationship_type,
            remaining_relationship_type,
            source_nodes,
            target_nodes,
            root_nodes,
            concurrency,
        }
    }

    /// Checks if a node pair is valid
    pub fn is_valid_node_pair(&self, source: i64, target: i64) -> bool {
        source != target && self.source_nodes.contains(source) && self.target_nodes.contains(target)
    }

    /// Gets the source nodes
    pub fn source_nodes(&self) -> &Arc<IdMap> {
        &self.source_nodes
    }

    /// Gets the target nodes
    pub fn target_nodes(&self) -> &Arc<IdMap> {
        &self.target_nodes
    }

    /// Gets the root nodes
    pub fn root_nodes(&self) -> &Arc<IdMap> {
        &self.root_nodes
    }

    /// Gets the selected relationship type
    pub fn selected_relationship_type(&self) -> &RelationshipType {
        &self.selected_relationship_type
    }

    /// Gets the remaining relationship type
    pub fn remaining_relationship_type(&self) -> &RelationshipType {
        &self.remaining_relationship_type
    }

    /// Gets the concurrency level
    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl EdgeSplitter for BaseEdgeSplitter {
    fn sample(&mut self, probability: f64) -> bool {
        self.rng.gen::<f64>() < probability
    }

    // These are implemented by specific splitters
    fn split_positive_examples(
        &mut self,
        _graph: Arc<Graph>,
        _holdout_fraction: f64,
        _remaining_rel_property_key: Option<String>,
    ) -> SplitResult {
        unimplemented!("Implemented by specific splitters")
    }

    fn valid_positive_relationship_candidate_count(
        &self,
        _graph: &Graph,
        _is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
    ) -> usize {
        unimplemented!("Implemented by specific splitters")
    }

    fn positive_sampling(
        &mut self,
        _graph: &Graph,
        _selected_rels_builder: &mut RelationshipsBuilder,
        _remaining_rels_consumer: Arc<dyn Fn(i64, i64, f64) + Send + Sync>,
        _selected_rel_count: &mut usize,
        _remaining_rel_count: &mut usize,
        _node_id: i64,
        _is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
        _positive_samples_remaining: &mut usize,
        _candidate_edges_remaining: &mut usize,
    ) {
        unimplemented!("Implemented by specific splitters")
    }
}
