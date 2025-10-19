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

use super::node_property_prediction_split_config::NodePropertyPredictionSplitConfig;
use crate::projection::eval::ml::pipeline::training_pipeline::TrainingPipeline;
use std::collections::HashMap;

/// Abstract base for node property training pipelines.
///
/// This trait extends TrainingPipeline with node-specific configuration
/// for splitting datasets and controlling feature computation.
pub trait NodePropertyTrainingPipeline: TrainingPipeline {
    /// Returns the split configuration for train/test/validation sets.
    fn split_config(&self) -> &NodePropertyPredictionSplitConfig;

    /// Sets the split configuration.
    fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig);

    /// Returns whether eager feature computation is required.
    ///
    /// When true, all features are computed upfront before training.
    /// When false, features can be computed lazily during training.
    fn require_eager_features(&self) -> bool;

    /// Returns the feature pipeline description for serialization.
    fn feature_pipeline_description(
        &self,
    ) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        use crate::projection::eval::ml::pipeline::FeatureStep;
        let mut desc = HashMap::new();

        // Node property steps
        let node_property_steps: Vec<HashMap<String, serde_json::Value>> = self
            .node_property_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();
        desc.insert("nodePropertySteps".to_string(), node_property_steps);

        // Feature properties
        let feature_steps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps()
            .iter()
            .map(|step| step.to_map())
            .collect();
        desc.insert("featureProperties".to_string(), feature_steps);

        desc
    }

    /// Returns additional entries for pipeline serialization.
    fn additional_entries(&self) -> HashMap<String, HashMap<String, String>> {
        let mut entries = HashMap::new();
        entries.insert("splitConfig".to_string(), self.split_config().to_map());
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::training_method::TrainingMethod;
    use crate::projection::eval::ml::pipeline::auto_tuning_config::AutoTuningConfig;
    use crate::projection::eval::ml::pipeline::training_pipeline::TunableTrainerConfig;
    use crate::projection::eval::ml::pipeline::{ExecutableNodePropertyStep, FeatureStep};
    use std::sync::Arc;

    // Mock feature step for testing
    #[derive(Clone, Debug)]
    struct MockFeatureStep;

    impl FeatureStep for MockFeatureStep {
        fn input_node_properties(&self) -> Vec<String> {
            vec!["age".to_string()]
        }
        fn name(&self) -> &str {
            "mock"
        }
        fn configuration(&self) -> HashMap<String, String> {
            HashMap::new()
        }
        fn to_map(&self) -> HashMap<String, String> {
            let mut map = HashMap::new();
            map.insert("type".to_string(), "mock".to_string());
            map
        }
    }

    // Mock training pipeline for testing
    struct MockNodeTrainingPipeline {
        split_config: NodePropertyPredictionSplitConfig,
        node_property_steps: Vec<Arc<ExecutableNodePropertyStep>>,
        feature_steps: Vec<MockFeatureStep>,
    }

    impl TrainingPipeline for MockNodeTrainingPipeline {
        fn training_parameter_space(
            &self,
        ) -> &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
            unimplemented!()
        }
        fn auto_tuning_config(&self) -> &AutoTuningConfig {
            unimplemented!()
        }
        fn number_of_model_selection_trials(&self) -> usize {
            5
        }
        fn node_property_steps(&self) -> &[Arc<ExecutableNodePropertyStep>] {
            &self.node_property_steps
        }
        fn add_node_property_step(&mut self, _step: ExecutableNodePropertyStep) {
            unimplemented!()
        }
        fn feature_steps(&self) -> Vec<MockFeatureStep> {
            self.feature_steps.clone()
        }
        fn add_feature_step(&mut self, _step: MockFeatureStep) {
            unimplemented!()
        }
    }

    impl NodePropertyTrainingPipeline for MockNodeTrainingPipeline {
        fn split_config(&self) -> &NodePropertyPredictionSplitConfig {
            &self.split_config
        }

        fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig) {
            self.split_config = split_config;
        }

        fn require_eager_features(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_split_config_access() {
        let mut pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![],
        };

        assert_eq!(pipeline.split_config().test_fraction(), 0.3);
        assert_eq!(pipeline.split_config().validation_folds(), 3);

        let new_config = NodePropertyPredictionSplitConfig::new(0.2, 5).unwrap();
        pipeline.set_split_config(new_config);

        assert_eq!(pipeline.split_config().test_fraction(), 0.2);
        assert_eq!(pipeline.split_config().validation_folds(), 5);
    }

    #[test]
    fn test_require_eager_features() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![],
        };

        assert_eq!(pipeline.require_eager_features(), false);
    }

    #[test]
    fn test_feature_pipeline_description() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::default(),
            node_property_steps: vec![],
            feature_steps: vec![MockFeatureStep],
        };

        let desc = pipeline.feature_pipeline_description();
        assert!(desc.contains_key("nodePropertySteps"));
        assert!(desc.contains_key("featureProperties"));
        assert_eq!(desc.get("featureProperties").unwrap().len(), 1);
    }

    #[test]
    fn test_additional_entries() {
        let pipeline = MockNodeTrainingPipeline {
            split_config: NodePropertyPredictionSplitConfig::new(0.25, 4).unwrap(),
            node_property_steps: vec![],
            feature_steps: vec![],
        };

        let entries = pipeline.additional_entries();
        assert!(entries.contains_key("splitConfig"));
        let split_map = entries.get("splitConfig").unwrap();
        assert_eq!(split_map.get("testFraction"), Some(&"0.25".to_string()));
    }
}
