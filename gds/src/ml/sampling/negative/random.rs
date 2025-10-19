use super::{NegativeSampler, NEGATIVE};
use crate::projection::factory::RelationshipsBuilder;
use crate::types::graph::{id_map::MappedNodeId, Graph, IdMap};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::sync::Arc;

const MAX_RETRIES: usize = 20;

pub struct RandomNegativeSampler {
    graph: Arc<dyn Graph>,
    test_sample_count: usize,
    train_sample_count: usize,
    valid_source_nodes: Arc<dyn IdMap>,
    valid_target_nodes: Arc<dyn IdMap>,
    rng: StdRng,
}

impl RandomNegativeSampler {
    pub fn new(
        graph: Arc<dyn Graph>,
        test_sample_count: usize,
        train_sample_count: usize,
        valid_source_nodes: Arc<dyn IdMap>,
        valid_target_nodes: Arc<dyn IdMap>,
        random_seed: Option<u64>,
    ) -> Self {
        let rng = match random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        Self {
            graph,
            test_sample_count,
            train_sample_count,
            valid_source_nodes,
            valid_target_nodes,
            rng,
        }
    }

    fn random_node_id(rng: &mut StdRng, upper_bound: MappedNodeId) -> MappedNodeId {
        rng.gen_range(0..upper_bound)
    }

    fn samples_per_node(
        rng: &mut StdRng,
        max_samples: usize,
        remaining_samples: f64,
        remaining_nodes: usize,
    ) -> usize {
        if remaining_nodes == 0 || max_samples == 0 {
            return 0;
        }
        let samples_on_average = remaining_samples / remaining_nodes as f64;
        let whole_samples = samples_on_average.floor() as usize;
        let extra_sample = if Self::sample(rng, samples_on_average - whole_samples as f64) {
            1
        } else {
            0
        };
        max_samples.min(whole_samples + extra_sample)
    }

    fn sample(rng: &mut StdRng, probability: f64) -> bool {
        rng.gen_range(0.0..1.0) < probability
    }
}

impl NegativeSampler for RandomNegativeSampler {
    fn produce_negative_samples(
        &self,
        test_set_builder: &mut dyn RelationshipsBuilder,
        train_set_builder: &mut dyn RelationshipsBuilder,
    ) {
        let mut remaining_test_samples = self.test_sample_count;
        let mut remaining_train_samples = self.train_sample_count;
        let mut remaining_valid_source_nodes = self.valid_source_nodes.node_count();
        let mut rng = self.rng.clone();
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return;
        }

        for node_index in 0..node_count {
            let node_id = node_index as MappedNodeId;

            let Some(original_id) = self.graph.to_original_node_id(node_id) else {
                continue;
            };

            if !self.valid_source_nodes.contains_original_id(original_id) {
                continue;
            }

            let master_degree = self.graph.degree(node_id);
            let max_negatives = node_count.saturating_sub(1).saturating_sub(master_degree);
            let negative_edge_count = Self::samples_per_node(
                &mut rng,
                max_negatives,
                (remaining_test_samples + remaining_train_samples) as f64,
                remaining_valid_source_nodes,
            );
            remaining_valid_source_nodes -= 1;

            let mut neighbors: HashSet<MappedNodeId> = HashSet::with_capacity(master_degree);
            self.graph
                .stream_relationships(node_id, self.graph.default_property_value())
                .for_each(|cursor| {
                    neighbors.insert(cursor.target_id());
                });

            for _ in 0..negative_edge_count {
                if remaining_test_samples + remaining_train_samples == 0 {
                    break;
                }
                let mut retries = MAX_RETRIES;
                loop {
                    let negative_target =
                        Self::random_node_id(&mut rng, node_count as MappedNodeId);

                    let Some(negative_original) = self.graph.to_original_node_id(negative_target)
                    else {
                        continue;
                    };

                    if self
                        .valid_target_nodes
                        .contains_original_id(negative_original)
                        && !neighbors.contains(&negative_target)
                        && negative_target != node_id
                    {
                        let prob = remaining_test_samples as f64
                            / (remaining_test_samples + remaining_train_samples) as f64;

                        let Some(source_root) = self.graph.to_root_node_id(node_id) else {
                            break;
                        };
                        let Some(target_root) = self.graph.to_root_node_id(negative_target) else {
                            break;
                        };

                        if Self::sample(&mut rng, prob) {
                            remaining_test_samples -= 1;
                            test_set_builder.add_from_internal(source_root, target_root, NEGATIVE);
                        } else {
                            remaining_train_samples -= 1;
                            train_set_builder.add_from_internal(source_root, target_root, NEGATIVE);
                        }
                        break;
                    }

                    retries -= 1;
                    if retries == 0 {
                        break;
                    }
                }
            }
        }
    }
}
