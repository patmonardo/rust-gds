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

//! Dataset bootstrapping for random forests.
//!
//! 1:1 translation of DatasetBootstrapper.java from Java GDS.

use crate::collections::BitSet;
use rand::prelude::*;
use std::sync::Arc;

/// ReadOnlyHugeLongArray type alias for shared read-only arrays
pub type ReadOnlyHugeLongArray = Arc<Vec<u64>>;

/// Utility for bootstrapping training datasets in random forests.
/// 1:1 translation of DatasetBootstrapper.java from Java GDS.
pub struct DatasetBootstrapper;

impl DatasetBootstrapper {
    /// Bootstrap a training set by sampling with replacement.
    /// 1:1 translation of DatasetBootstrapper.bootstrap() from Java GDS.
    ///
    /// # Arguments
    /// * `rng` - Random number generator
    /// * `num_feature_vectors_ratio` - Ratio of feature vectors to sample (0.0-1.0)
    /// * `train_set` - Original training set (node IDs)
    /// * `bootstrapped_train_set_indices` - BitSet to track which indices were sampled
    ///
    /// # Returns
    /// A new ReadOnlyHugeLongArray containing the bootstrapped sample indices
    pub fn bootstrap(
        rng: &mut impl Rng,
        num_feature_vectors_ratio: f64,
        train_set: &ReadOnlyHugeLongArray,
        bootstrapped_train_set_indices: &mut BitSet,
    ) -> ReadOnlyHugeLongArray {
        assert!(
            (0.0..=1.0).contains(&num_feature_vectors_ratio),
            "num_feature_vectors_ratio must be between 0.0 and 1.0"
        );

        let num_vectors = (num_feature_vectors_ratio * train_set.len() as f64).ceil() as usize;
        let mut bootstrapped_vectors = Vec::with_capacity(num_vectors);

        for _ in 0..num_vectors {
            // Sample with replacement
            let sampled_idx = rng.gen_range(0..train_set.len());

            // Store the sampled node id (translate from train set idx)
            bootstrapped_vectors.push(train_set[sampled_idx]);

            // Track which indices were sampled
            bootstrapped_train_set_indices.set(sampled_idx);
        }

        Arc::new(bootstrapped_vectors)
    }
}
