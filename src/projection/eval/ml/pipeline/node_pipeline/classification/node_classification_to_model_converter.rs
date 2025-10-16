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
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
// Placeholder types until model catalog and schema are translated
pub type GraphSchema = ();
pub type Model = ();
pub type GdsVersion = String;
pub type NodePropertyPredictPipeline = ();
pub type NodeClassificationPipelineModelInfo = ();

/// Converter from training result to catalog model.
///
/// Transforms a NodeClassificationTrainResult into a NodeClassificationModelResult
/// suitable for storage in the model catalog.
pub struct NodeClassificationToModelConverter {
    pipeline: NodeClassificationTrainingPipeline,
    config: NodeClassificationPipelineTrainConfig,
}

impl NodeClassificationToModelConverter {
    pub fn new(
        pipeline: NodeClassificationTrainingPipeline,
        config: NodeClassificationPipelineTrainConfig,
    ) -> Self {
        Self { pipeline, config }
    }

    pub fn pipeline(&self) -> &NodeClassificationTrainingPipeline {
        &self.pipeline
    }

    pub fn config(&self) -> &NodeClassificationPipelineTrainConfig {
        &self.config
    }

    /// Convert training result to model result.
    pub fn to_model(
        &self,
        result: &NodeClassificationTrainResult,
        _original_schema: &GraphSchema,
    ) -> NodeClassificationModelResult {
        // TODO: When Model catalog is implemented, create actual Model:
        // let catalog_model = Model::of(
        //     GdsVersionInfoProvider::gds_version(),
        //     NodeClassificationTrainingPipeline::MODEL_TYPE,
        //     original_schema,
        //     result.classifier().data(),
        //     self.config.clone(),
        //     NodeClassificationPipelineModelInfo::of(
        //         result.training_statistics().winning_model_test_metrics(),
        //         result.training_statistics().winning_model_outer_train_metrics(),
        //         result.training_statistics().best_candidate(),
        //         NodePropertyPredictPipeline::from(self.pipeline.clone()),
        //         result.class_id_map().original_ids_list(),
        //     )
        // );

        let catalog_model = ();
        let training_statistics = result.training_statistics().clone();

        NodeClassificationModelResult::new(catalog_model, training_statistics)
    }
}

// TODO: Implement ResultToModelConverter trait when available
// impl ResultToModelConverter<NodeClassificationModelResult, NodeClassificationTrainResult> for NodeClassificationToModelConverter {
//     fn to_model(&self, result: &NodeClassificationTrainResult, original_schema: &GraphSchema) -> NodeClassificationModelResult {
//         self.to_model(result, original_schema)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::subgraph::LocalIdMap;

    #[test]
    fn test_new_converter() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();

        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        // Verify accessors work
        let _pipeline = converter.pipeline();
        let _config = converter.config();
    }

    #[test]
    fn test_to_model() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        let train_result = NodeClassificationTrainResult::new(
            (),
            (),
            LocalIdMap::of(&[0, 1, 2]),
            LongMultiSet::new(),
        );
        let schema = ();

        let model_result = converter.to_model(&train_result, &schema);

        // Verify result was created
        let _catalog_model = model_result.catalog_model();
        let _stats = model_result.training_statistics();
    }

    #[test]
    fn test_converter_references() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();

        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        // Verify pipeline and config are accessible
        assert_eq!(
            converter.pipeline().pipeline_type(),
            NodeClassificationTrainingPipeline::PIPELINE_TYPE
        );
    }
}
