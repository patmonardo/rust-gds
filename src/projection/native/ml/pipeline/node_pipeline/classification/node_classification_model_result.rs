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

use super::node_classification_train_result::TrainingStatistics;

// Placeholder types until ml-models and model catalog are translated
pub type Classifier = ();
pub type ClassifierData = ();
pub type CatalogModel = ();
pub type NodeClassificationPipelineTrainConfig = ();
pub type NodeClassificationPipelineModelInfo = ();

/// Result of node classification model creation containing the catalog model and training statistics.
///
/// This is a value class that wraps the model catalog entry with training statistics.
#[derive(Debug, Clone)]
pub struct NodeClassificationModelResult {
    // TODO: When model catalog is implemented, this should be:
    // model: Model<ClassifierData, NodeClassificationPipelineTrainConfig, NodeClassificationPipelineModelInfo>
    catalog_model: CatalogModel,
    training_statistics: TrainingStatistics,
}

impl NodeClassificationModelResult {
    pub fn new(catalog_model: CatalogModel, training_statistics: TrainingStatistics) -> Self {
        Self {
            catalog_model,
            training_statistics,
        }
    }

    pub fn catalog_model(&self) -> &CatalogModel {
        &self.catalog_model
    }

    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model_result() {
        let catalog_model = ();
        let training_statistics = ();

        let result = NodeClassificationModelResult::new(catalog_model, training_statistics);

        // Verify accessors work
        let _model = result.catalog_model();
        let _stats = result.training_statistics();
    }

    #[test]
    fn test_accessors() {
        let catalog_model = ();
        let training_statistics = ();

        let result = NodeClassificationModelResult::new(catalog_model, training_statistics);

        // Verify both accessors return references
        assert!(std::ptr::eq(result.catalog_model(), &result.catalog_model));
        assert!(std::ptr::eq(
            result.training_statistics(),
            &result.training_statistics
        ));
    }
}
