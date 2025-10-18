use super::edge_splitter::{BaseEdgeSplitter, EdgeSplitter};
use crate::{
    core::{
        concurrency::{Concurrency, RunWithConcurrency},
        graph::Graph,
        id_map::IdMap,
        partition::PartitionUtils,
        relationship::RelationshipType,
    },
    graph::builder::RelationshipsBuilder,
};
use std::sync::{atomic::AtomicUsize, Arc};

/// Splitter for directed graphs that maintains edge direction
pub struct DirectedEdgeSplitter {
    base: BaseEdgeSplitter,
}

impl DirectedEdgeSplitter {
    /// Creates a new DirectedEdgeSplitter
    pub fn new(
        maybe_seed: Option<u64>,
        root_nodes: Arc<IdMap>,
        source_nodes: Arc<IdMap>,
        target_nodes: Arc<IdMap>,
        selected_relationship_type: RelationshipType,
        remaining_relationship_type: RelationshipType,
        concurrency: usize,
    ) -> Self {
        Self {
            base: BaseEdgeSplitter::new(
                maybe_seed,
                root_nodes,
                source_nodes,
                target_nodes,
                selected_relationship_type,
                remaining_relationship_type,
                concurrency,
            ),
        }
    }
}

impl EdgeSplitter for DirectedEdgeSplitter {
    fn split_positive_examples(
        &mut self,
        graph: Arc<Graph>,
        holdout_fraction: f64,
        remaining_rel_property_key: Option<String>,
    ) -> super::edge_splitter::SplitResult {
        // Delegate to base implementation
        self.base
            .split_positive_examples(graph, holdout_fraction, remaining_rel_property_key)
    }

    fn sample(&mut self, probability: f64) -> bool {
        self.base.sample(probability)
    }

    fn valid_positive_relationship_candidate_count(
        &self,
        graph: &Graph,
        is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
    ) -> usize {
        let valid_relationship_count = Arc::new(AtomicUsize::new(0));

        // Create tasks for each partition
        let count_valid_relationship_tasks =
            PartitionUtils::degree_partition(graph, self.base.concurrency(), {
                let graph = graph.clone();
                let valid_relationship_count = valid_relationship_count.clone();
                let is_valid_node_pair = is_valid_node_pair.clone();

                move |partition| {
                    let concurrent_graph = graph.concurrent_copy();

                    Box::new(move || {
                        let mut local_count = 0;

                        for node_id in partition.iter() {
                            concurrent_graph.for_each_relationship(node_id, |_, target, _| {
                                if is_valid_node_pair(node_id, target) {
                                    local_count += 1;
                                }
                                true
                            });
                        }

                        valid_relationship_count
                            .fetch_add(local_count, std::sync::atomic::Ordering::Relaxed);
                    }) as Box<dyn FnOnce() + Send>
                }
            });

        // Run tasks concurrently
        RunWithConcurrency::new()
            .concurrency(self.base.concurrency())
            .tasks(count_valid_relationship_tasks)
            .run();

        valid_relationship_count.load(std::sync::atomic::Ordering::Relaxed)
    }

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
    ) {
        graph.for_each_relationship(node_id, |source, target, weight| {
            if is_valid_node_pair(source, target) {
                *candidate_edges_remaining -= 1;

                if *positive_samples_remaining > 0
                    && self.sample(
                        (*positive_samples_remaining as f64)
                            / (*candidate_edges_remaining as f64 + 1.0),
                    )
                {
                    // Add to selected relationships
                    selected_rels_builder.add(source, target, weight);
                    *selected_rel_count += 1;
                    *positive_samples_remaining -= 1;
                } else {
                    // Add to remaining relationships
                    (remaining_rels_consumer)(source, target, weight);
                    *remaining_rel_count += 1;
                }
            }
            true
        });
    }
}
