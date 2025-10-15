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

// Placeholder types until corresponding packages are translated
pub type Regressor = ();
pub type TrainingStatistics = ();
pub type RegressorData = ();
pub type NodeRegressionPipelineModelInfo = ();
pub type CatalogModelContainer = ();

use super::NodeRegressionPipelineTrainConfig;

/// Result of training a node regression pipeline.
///
/// Contains the trained regressor model and associated training statistics
/// (cross-validation scores, best parameters, etc.).
///
/// Java source: `NodeRegressionTrainResult.java` (Immutables @ValueClass)
#[derive(Debug, Clone)]
pub struct NodeRegressionTrainResult {
    regressor: Regressor,
    training_statistics: TrainingStatistics,
}

impl NodeRegressionTrainResult {
    pub fn new(regressor: Regressor, training_statistics: TrainingStatistics) -> Self {
        Self {
            regressor,
            training_statistics,
        }
    }

    /// Returns the trained regressor model.
    pub fn regressor(&self) -> &Regressor {
        &self.regressor
    }

    /// Returns training statistics (CV scores, best params, etc.).
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

/// Result of training a node regression pipeline with catalog integration.
///
/// Extends the basic train result with model catalog metadata.
/// This is what gets stored in ModelCatalog after training.
///
/// Java source: `NodeRegressionTrainPipelineResult` (nested @ValueClass)
///
/// # Generic Parameters
/// This implements the `CatalogModelContainer<DATA, CONFIG, INFO>` pattern:
/// - `DATA`: RegressorData - serialized model weights/parameters
/// - `CONFIG`: NodeRegressionPipelineTrainConfig - training configuration
/// - `INFO`: NodeRegressionPipelineModelInfo - custom metadata (feature importance, splits)
#[derive(Debug, Clone)]
pub struct NodeRegressionTrainPipelineResult {
    // Model catalog fields (from CatalogModelContainer)
    regressor_data: RegressorData,
    train_config: NodeRegressionPipelineTrainConfig,
    model_info: NodeRegressionPipelineModelInfo,

    // Training-specific field
    training_statistics: TrainingStatistics,
}

impl NodeRegressionTrainPipelineResult {
    pub fn new(
        regressor_data: RegressorData,
        train_config: NodeRegressionPipelineTrainConfig,
        model_info: NodeRegressionPipelineModelInfo,
        training_statistics: TrainingStatistics,
    ) -> Self {
        Self {
            regressor_data,
            train_config,
            model_info,
            training_statistics,
        }
    }

    /// Returns the serialized regressor model data.
    pub fn regressor_data(&self) -> &RegressorData {
        &self.regressor_data
    }

    /// Returns the training configuration used.
    pub fn train_config(&self) -> &NodeRegressionPipelineTrainConfig {
        &self.train_config
    }

    /// Returns custom model metadata (feature importance, splits, etc.).
    pub fn model_info(&self) -> &NodeRegressionPipelineModelInfo {
        &self.model_info
    }

    /// Returns training statistics.
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_result_new() {
        let result = NodeRegressionTrainResult::new((), ());
        assert!(result.regressor() == &());
        assert!(result.training_statistics() == &());
    }

    #[test]
    fn test_pipeline_result_new() {
        let config = NodeRegressionPipelineTrainConfig::default();
        let result = NodeRegressionTrainPipelineResult::new(
            (), // regressor_data
            config,
            (), // model_info
            (), // training_statistics
        );

        assert!(result.regressor_data() == &());
        assert!(result.model_info() == &());
        assert!(result.training_statistics() == &());
    }

    #[test]
    fn test_pipeline_result_config_access() {
        let config = NodeRegressionPipelineTrainConfig::default();
        let result = NodeRegressionTrainPipelineResult::new((), config.clone(), (), ());

        assert_eq!(result.train_config().pipeline(), config.pipeline());
        assert_eq!(
            result.train_config().target_property(),
            config.target_property()
        );
    }
}
