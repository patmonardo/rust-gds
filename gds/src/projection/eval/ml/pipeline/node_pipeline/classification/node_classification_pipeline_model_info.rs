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

use crate::projection::eval::ml::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::ml::pipeline::Pipeline;
use serde_json::Value;
use std::collections::HashMap;

// Placeholder types until ml-models and ml-training packages are translated
pub type TrainerConfig = ();
pub type Metric = ();
pub type ModelCandidateStats = ();
pub type TrainingMethod = ();

/// Model information for node classification pipelines.
///
/// Contains the best training parameters, metrics, pipeline definition, and predicted classes.
///
/// Note: Cannot derive Clone or Debug because NodePropertyPredictPipeline contains Box<dyn Trait>.
pub struct NodeClassificationPipelineModelInfo {
    best_parameters: TrainerConfig,
    metrics: HashMap<String, Value>,
    pipeline: NodePropertyPredictPipeline,
    classes: Vec<i64>,
}

impl NodeClassificationPipelineModelInfo {
    pub fn new(
        best_parameters: TrainerConfig,
        metrics: HashMap<String, Value>,
        pipeline: NodePropertyPredictPipeline,
        classes: Vec<i64>,
    ) -> Self {
        Self {
            best_parameters,
            metrics,
            pipeline,
            classes,
        }
    }

    /// Create model info from training results.
    pub fn of(
        _test_metrics: &HashMap<Metric, f64>,
        _outer_train_metrics: &HashMap<Metric, f64>,
        _best_candidate: &ModelCandidateStats,
        pipeline: NodePropertyPredictPipeline,
        classes: Vec<i64>,
    ) -> Self {
        // TODO: When metrics system is implemented:
        // let metrics = best_candidate.render_metrics(test_metrics, outer_train_metrics);
        let best_parameters = ();
        let metrics = HashMap::new();

        Self::new(best_parameters, metrics, pipeline, classes)
    }

    pub fn best_parameters(&self) -> &TrainerConfig {
        &self.best_parameters
    }

    pub fn metrics(&self) -> &HashMap<String, Value> {
        &self.metrics
    }

    pub fn pipeline(&self) -> &NodePropertyPredictPipeline {
        &self.pipeline
    }

    pub fn classes(&self) -> &[i64] {
        &self.classes
    }

    /// Convert to map representation for serialization.
    pub fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();

        // TODO: When TrainerConfig is implemented, add: best_parameters.to_map_with_trainer_method()
        map.insert(
            "bestParameters".to_string(),
            Value::Object(Default::default()),
        );
        map.insert(
            "classes".to_string(),
            Value::Array(
                self.classes
                    .iter()
                    .map(|c| Value::Number((*c).into()))
                    .collect(),
            ),
        );
        map.insert(
            "metrics".to_string(),
            Value::Object(
                self.metrics
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );
        map.insert(
            "pipeline".to_string(),
            Value::Object(
                self.pipeline
                    .to_map()
                    .into_iter()
                    .map(|(k, v)| (k, v))
                    .collect(),
            ),
        );

        // TODO: When ExecutableNodePropertyStep has to_map(), add:
        // map.insert("nodePropertySteps", to_map_convertible(pipeline.node_property_steps()));

        map.insert(
            "featureProperties".to_string(),
            Value::Array(
                self.pipeline
                    .feature_properties()
                    .iter()
                    .map(|s| Value::String(s.clone()))
                    .collect(),
            ),
        );

        map
    }

    /// Get optional training method.
    pub fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        // TODO: When TrainerConfig is implemented: Some(self.best_parameters.method())
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model_info() {
        let best_parameters = ();
        let metrics = HashMap::new();
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![0, 1, 2];

        let info =
            NodeClassificationPipelineModelInfo::new(best_parameters, metrics, pipeline, classes);

        assert_eq!(info.classes(), &[0, 1, 2]);
    }

    #[test]
    fn test_of_constructor() {
        let test_metrics = HashMap::new();
        let outer_train_metrics = HashMap::new();
        let best_candidate = ();
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![10, 20, 30];

        let info = NodeClassificationPipelineModelInfo::of(
            &test_metrics,
            &outer_train_metrics,
            &best_candidate,
            pipeline,
            classes,
        );

        assert_eq!(info.classes(), &[10, 20, 30]);
    }

    #[test]
    fn test_to_map() {
        let best_parameters = ();
        let metrics = HashMap::new();
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![0, 1];

        let info =
            NodeClassificationPipelineModelInfo::new(best_parameters, metrics, pipeline, classes);

        let map = info.to_map();

        // Verify map contains expected keys
        assert!(map.contains_key("bestParameters"));
        assert!(map.contains_key("classes"));
        assert!(map.contains_key("metrics"));
        assert!(map.contains_key("pipeline"));
        assert!(map.contains_key("featureProperties"));
    }

    #[test]
    fn test_optional_trainer_method() {
        let info = NodeClassificationPipelineModelInfo::new(
            (),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
            vec![0, 1],
        );

        // Should return None until TrainerConfig is implemented
        assert!(info.optional_trainer_method().is_none());
    }
}
