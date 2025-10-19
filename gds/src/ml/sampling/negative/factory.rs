// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Factory method for creating NegativeSampler instances.
//!
//! 1:1 translation of NegativeSampler.of() static factory method from Java GDS.

use super::{NegativeSampler, RandomNegativeSampler, UserInputNegativeSampler};
use crate::projection::NodeLabel;
use crate::types::graph::{Graph, IdMap};
use crate::types::graph_store::GraphStore;
use std::sync::Arc;

/// Creates a NegativeSampler based on configuration.
/// 1:1 translation of NegativeSampler.of() from Java GDS.
///
/// If `negative_relationship_type` is provided, uses UserInputNegativeSampler.
/// Otherwise, uses RandomNegativeSampler.
pub fn create_sampler<GS: GraphStore>(
    _graph_store: Arc<GS>,
    graph: Arc<dyn Graph>,
    _source_and_target_node_labels: Vec<NodeLabel>,
    negative_relationship_type: Option<String>,
    negative_sampling_ratio: f64,
    test_positive_count: i64,
    train_positive_count: i64,
    valid_source_nodes: Arc<dyn IdMap>,
    valid_target_nodes: Arc<dyn IdMap>,
    source_labels: Vec<NodeLabel>,
    target_labels: Vec<NodeLabel>,
    random_seed: Option<u64>,
) -> Box<dyn NegativeSampler> {
    if let Some(_rel_type) = negative_relationship_type {
        // TODO: create filtered graph once GraphStore filtering API is available.
        let negative_example_graph = Arc::clone(&graph);

        let test_train_fraction =
            test_positive_count as f64 / (test_positive_count + train_positive_count) as f64;

        Box::new(UserInputNegativeSampler::new(
            negative_example_graph,
            test_train_fraction,
            random_seed,
            source_labels,
            target_labels,
        ))
    } else {
        let test_sample_count =
            scaled_negative_samples(test_positive_count, negative_sampling_ratio);
        let train_sample_count =
            scaled_negative_samples(train_positive_count, negative_sampling_ratio);

        Box::new(RandomNegativeSampler::new(
            graph,
            test_sample_count,
            train_sample_count,
            valid_source_nodes,
            valid_target_nodes,
            random_seed,
        ))
    }
}

fn scaled_negative_samples(count: i64, ratio: f64) -> usize {
    if count <= 0 {
        return 0;
    }
    let scaled = (count as f64 * ratio).max(0.0);
    scaled.floor() as usize
}
