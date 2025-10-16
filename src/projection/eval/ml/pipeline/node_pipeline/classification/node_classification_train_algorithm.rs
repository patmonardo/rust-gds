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

use super::node_classification_model_result::NodeClassificationModelResult;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_to_model_converter::NodeClassificationToModelConverter;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

// Placeholder types until pipeline infrastructure is translated
pub type PipelineTrainer = ();
pub type ProgressTracker = ();

/// Train algorithm for node classification pipelines.
///
/// This is an adapter that wires together the pipeline trainer, training pipeline,
/// model converter, graph store, and configuration.
pub struct NodeClassificationTrainAlgorithm {
    pipeline_trainer: PipelineTrainer,
    pipeline: NodeClassificationTrainingPipeline,
    converter: NodeClassificationToModelConverter,
    graph_store: Arc<DefaultGraphStore>,
    config: NodeClassificationPipelineTrainConfig,
    progress_tracker: ProgressTracker,
}

impl NodeClassificationTrainAlgorithm {
    pub fn new(
        pipeline_trainer: PipelineTrainer,
        pipeline: NodeClassificationTrainingPipeline,
        graph_store: Arc<DefaultGraphStore>,
        config: NodeClassificationPipelineTrainConfig,
        progress_tracker: ProgressTracker,
    ) -> Self {
        let converter = NodeClassificationToModelConverter::new(pipeline.clone(), config.clone());

        Self {
            pipeline_trainer,
            pipeline,
            converter,
            graph_store,
            config,
            progress_tracker,
        }
    }

    pub fn pipeline_trainer(&self) -> &PipelineTrainer {
        &self.pipeline_trainer
    }

    pub fn pipeline(&self) -> &NodeClassificationTrainingPipeline {
        &self.pipeline
    }

    pub fn converter(&self) -> &NodeClassificationToModelConverter {
        &self.converter
    }

    pub fn graph_store(&self) -> Arc<DefaultGraphStore> {
        Arc::clone(&self.graph_store)
    }

    pub fn config(&self) -> &NodeClassificationPipelineTrainConfig {
        &self.config
    }

    pub fn progress_tracker(&self) -> &ProgressTracker {
        &self.progress_tracker
    }

    // TODO: Add compute() method when PipelineTrainAlgorithm trait is translated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

    #[test]
    fn test_new_train_algorithm() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline_trainer = ();
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let progress_tracker = ();

        let algorithm = NodeClassificationTrainAlgorithm::new(
            pipeline_trainer,
            pipeline,
            graph_store.clone(),
            train_config,
            progress_tracker,
        );

        // Verify accessors work
        let _trainer = algorithm.pipeline_trainer();
        let _pipeline = algorithm.pipeline();
        let _converter = algorithm.converter();
        let _store = algorithm.graph_store();
        let _config = algorithm.config();
        let _tracker = algorithm.progress_tracker();
    }
}
