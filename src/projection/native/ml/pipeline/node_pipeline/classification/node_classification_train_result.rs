/*
 * Copyright (c) "Neo4j"
 * Neo4j Sweden AB [http://neo4j.com]
 *
 * This file is part of Neo4j.
 *
 * Neo4j is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

// Placeholder types until ml-models and ml-training packages are translated
pub type Classifier = ();
pub type TrainingStatistics = ();
pub type LongMultiSet = std::collections::HashMap<i64, usize>;

use crate::ml::core::subgraph::LocalIdMap;

/// Result of training a node classification model.
///
/// Contains the trained classifier, training statistics, class ID mappings, and class counts.
#[derive(Debug, Clone)]
pub struct NodeClassificationTrainResult {
    classifier: Classifier,
    training_statistics: TrainingStatistics,
    class_id_map: LocalIdMap,
    class_counts: LongMultiSet,
}

impl NodeClassificationTrainResult {
    pub fn new(
        classifier: Classifier,
        training_statistics: TrainingStatistics,
        class_id_map: LocalIdMap,
        class_counts: LongMultiSet,
    ) -> Self {
        Self {
            classifier,
            training_statistics,
            class_id_map,
            class_counts,
        }
    }

    pub fn classifier(&self) -> &Classifier {
        &self.classifier
    }

    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }

    pub fn class_id_map(&self) -> &LocalIdMap {
        &self.class_id_map
    }

    pub fn class_counts(&self) -> &LongMultiSet {
        &self.class_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_train_result() {
        let classifier = ();
        let training_statistics = ();
        let class_id_map = LocalIdMap::of(&[0, 1, 2]);
        let class_counts = LongMultiSet::new();

        let result = NodeClassificationTrainResult::new(
            classifier,
            training_statistics,
            class_id_map,
            class_counts,
        );

        assert_eq!(result.class_id_map().size(), 3);
    }

    #[test]
    fn test_accessors() {
        let classifier = ();
        let training_statistics = ();
        let class_id_map = LocalIdMap::of(&[10, 20, 30]);
        let class_counts = LongMultiSet::new();

        let result = NodeClassificationTrainResult::new(
            classifier,
            training_statistics,
            class_id_map.clone(),
            class_counts,
        );

        // Verify accessors return correct references
        assert_eq!(result.class_id_map().original_ids_list(), &vec![10, 20, 30]);
    }
}
