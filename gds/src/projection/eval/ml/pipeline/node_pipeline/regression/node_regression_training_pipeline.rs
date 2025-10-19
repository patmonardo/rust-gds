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

use crate::ml::training_method::TrainingMethod;
use crate::projection::eval::ml::pipeline::TrainingType;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

// TODO: NodePropertyTrainingPipeline base trait will be implemented when node_pipeline infrastructure is complete

/// Node regression training pipeline.
///
/// Extends the base `NodePropertyTrainingPipeline` with regression-specific logic.
#[derive(Debug, Clone)]
pub struct NodeRegressionTrainingPipeline {
    // TODO: Will inherit fields from NodePropertyTrainingPipeline when trait-based composition is implemented
}

impl NodeRegressionTrainingPipeline {
    pub const PIPELINE_TYPE: &'static str = "Node regression training pipeline";
    pub const MODEL_TYPE: &'static str = "NodeRegression";

    /// Create a new node regression training pipeline.
    pub fn new() -> Self {
        // TODO: Initialize with TrainingType::Regression
        Self {}
    }

    /// Returns the pipeline type string.
    pub fn pipeline_type(&self) -> &str {
        Self::PIPELINE_TYPE
    }

    /// Validates pipeline-specific constraints before execution.
    ///
    /// For regression, this is currently a no-op.
    pub fn specific_validate_before_execution(&self, _graph_store: &Arc<DefaultGraphStore>) {
        // No specific validation for regression pipelines yet
    }

    /// Returns whether this pipeline requires eager feature computation.
    ///
    /// Returns `true` if the training parameter space contains RandomForestRegression
    /// configurations, which require all features to be computed upfront.
    pub fn require_eager_features(&self) -> bool {
        // TODO: Implement when TrainingMethod enum is translated
        // Check if trainingParameterSpace.get(TrainingMethod::RandomForestRegression).isEmpty()
        // For now, return false as a safe default
        false
    }
}

impl Default for NodeRegressionTrainingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pipeline() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        assert_eq!(
            pipeline.pipeline_type(),
            "Node regression training pipeline"
        );
    }

    #[test]
    fn test_pipeline_constants() {
        assert_eq!(
            NodeRegressionTrainingPipeline::PIPELINE_TYPE,
            "Node regression training pipeline"
        );
        assert_eq!(NodeRegressionTrainingPipeline::MODEL_TYPE, "NodeRegression");
    }

    #[test]
    fn test_default() {
        let pipeline = NodeRegressionTrainingPipeline::default();
        assert_eq!(
            pipeline.pipeline_type(),
            "Node regression training pipeline"
        );
    }

    #[test]
    fn test_require_eager_features_default() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        // Without RandomForestRegression in parameter space, should return false
        assert!(!pipeline.require_eager_features());
    }

    #[test]
    fn test_specific_validation() {
        use crate::types::random::RandomGraphConfig;

        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));

        // Should not panic - validation is currently a no-op
        pipeline.specific_validate_before_execution(&graph_store);
    }
}
