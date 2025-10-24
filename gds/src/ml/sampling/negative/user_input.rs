use super::{NegativeSampler, NEGATIVE};
use crate::projection::{factory::RelationshipsBuilder, NodeLabel};
use crate::types::graph::{id_map::MappedNodeId, Graph};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::sync::Arc;

pub struct UserInputNegativeSampler {
    negative_example_graph: Arc<dyn Graph>,
    test_train_fraction: f64,
    rng: StdRng,
}

impl UserInputNegativeSampler {
    pub fn new(
        negative_example_graph: Arc<dyn Graph>,
        test_train_fraction: f64,
        random_seed: Option<u64>,
        source_labels: Vec<NodeLabel>,
        target_labels: Vec<NodeLabel>,
    ) -> Self {
        if !negative_example_graph.schema().is_undirected() {
            panic!("UserInputNegativeSampler requires graph to be UNDIRECTED");
        }

        let rng = match random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        let sampler = Self {
            negative_example_graph,
            test_train_fraction,
            rng,
        };

        sampler.validate_negative_relationships(&source_labels, &target_labels);
        sampler
    }

    fn sample(rng: &mut StdRng, probability: f64) -> bool {
        rng.gen_range(0.0..1.0) < probability
    }

    fn validate_negative_relationships(
        &self,
        source_labels: &[NodeLabel],
        target_labels: &[NodeLabel],
    ) {
        let mut node_consumer = |node_id: MappedNodeId| {
            self.negative_example_graph
                .stream_relationships(node_id, self.negative_example_graph.default_property_value())
                .for_each(|cursor| {
                    let s = cursor.source_id();
                    let t = cursor.target_id();

                    let source_node_labels = self.negative_example_graph.node_labels(s);
                    let target_node_labels = self.negative_example_graph.node_labels(t);

                    if !self.node_pairs_have_valid_labels(
                        &source_node_labels,
                        &target_node_labels,
                        source_labels,
                        target_labels,
                    ) {
                        let source_original = self
                            .negative_example_graph
                            .to_original_node_id(s)
                            .unwrap_or(-1);
                        let target_original = self
                            .negative_example_graph
                            .to_original_node_id(t)
                            .unwrap_or(-1);

                        panic!(
                            "Invalid relationship between nodes {} and {}. Found labels {:?} and {:?}, expected between {:?} and {:?}",
                            source_original,
                            target_original,
                            source_node_labels,
                            target_node_labels,
                            source_labels,
                            target_labels
                        );
                    }
                });
            true
        };

        self.negative_example_graph
            .for_each_node(&mut node_consumer);
    }

    fn node_pairs_have_valid_labels(
        &self,
        candidate_source: &HashSet<NodeLabel>,
        candidate_target: &HashSet<NodeLabel>,
        valid_source_labels: &[NodeLabel],
        valid_target_labels: &[NodeLabel],
    ) -> bool {
        (candidate_source
            .iter()
            .any(|l| valid_source_labels.contains(l))
            && candidate_target
                .iter()
                .any(|l| valid_target_labels.contains(l)))
            || (candidate_source
                .iter()
                .any(|l| valid_target_labels.contains(l))
                && candidate_target
                    .iter()
                    .any(|l| valid_source_labels.contains(l)))
    }
}

impl NegativeSampler for UserInputNegativeSampler {
    fn produce_negative_samples(
        &self,
        test_set_builder: &mut dyn RelationshipsBuilder,
        train_set_builder: &mut dyn RelationshipsBuilder,
    ) {
        let total_relationship_count = self.negative_example_graph.relationship_count() / 2;
        let test_relationship_count =
            (total_relationship_count as f64 * self.test_train_fraction) as usize;
        let mut test_relationships_to_add = test_relationship_count;
        let mut train_relationships_to_add = total_relationship_count - test_relationship_count;

        let mut rng = self.rng.clone();

        let mut node_consumer = |node_id: MappedNodeId| {
            self.negative_example_graph
                .stream_relationships(
                    node_id,
                    self.negative_example_graph.default_property_value(),
                )
                .for_each(|cursor| {
                    let s = cursor.source_id();
                    let t = cursor.target_id();

                    if s < t {
                        let total_remaining =
                            test_relationships_to_add + train_relationships_to_add;
                        if total_remaining == 0 {
                            return;
                        }

                        let prob = test_relationships_to_add as f64 / total_remaining as f64;

                        let Some(root_s) = self.negative_example_graph.to_root_node_id(s) else {
                            return;
                        };
                        let Some(root_t) = self.negative_example_graph.to_root_node_id(t) else {
                            return;
                        };

                        if Self::sample(&mut rng, prob) {
                            if test_relationships_to_add > 0 {
                                test_relationships_to_add -= 1;
                                test_set_builder.add_from_internal(root_s as u64, root_t as u64, NEGATIVE);
                            }
                        } else if train_relationships_to_add > 0 {
                            train_relationships_to_add -= 1;
                            train_set_builder.add_from_internal(root_s as u64, root_t as u64, NEGATIVE);
                        }
                    }
                });
            true
        };

        self.negative_example_graph
            .for_each_node(&mut node_consumer);
    }
}
