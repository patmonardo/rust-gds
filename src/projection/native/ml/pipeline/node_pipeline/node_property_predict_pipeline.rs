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

use super::NodeFeatureStep;
use crate::projection::native::ml::pipeline::{ExecutableNodePropertyStep, Pipeline};
use crate::types::graph_store::DefaultGraphStore;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// A prediction pipeline for node property prediction.
///
/// This is an immutable snapshot of a pipeline used for making predictions
/// on new data after training is complete.
pub struct NodePropertyPredictPipeline {
    node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
    feature_steps: Vec<NodeFeatureStep>,
}

impl NodePropertyPredictPipeline {
    /// Empty pipeline with no steps.
    pub fn empty() -> Self {
        Self {
            node_property_steps: vec![],
            feature_steps: vec![],
        }
    }

    /// Creates a new predict pipeline from lists of steps.
    pub fn new(
        node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> Self {
        Self {
            node_property_steps,
            feature_steps,
        }
    }

    /// Creates a predict pipeline from a training pipeline.
    pub fn from_pipeline<P: Pipeline<FeatureStep = NodeFeatureStep>>(pipeline: &P) -> Self {
        // Note: This requires cloning the Box contents, which may not be possible
        // for all ExecutableNodePropertyStep implementations
        unimplemented!("from_pipeline needs Box cloning strategy")
    }

    /// Returns the list of feature properties used by this pipeline.
    pub fn feature_properties(&self) -> Vec<String> {
        use crate::projection::native::ml::pipeline::FeatureStep;
        self.feature_steps
            .iter()
            .flat_map(|step| step.input_node_properties().iter().cloned())
            .collect()
    }
}

impl Pipeline for NodePropertyPredictPipeline {
    type FeatureStep = NodeFeatureStep;

    fn to_map(&self) -> HashMap<String, Value> {
        use crate::projection::native::ml::pipeline::FeatureStep;
        let mut map = HashMap::new();

        let node_property_steps: Vec<Value> = self
            .node_property_steps
            .iter()
            .map(|step| {
                let step_map: serde_json::Map<String, Value> = step.to_map().into_iter().collect();
                Value::Object(step_map)
            })
            .collect();
        map.insert(
            "nodePropertySteps".to_string(),
            Value::Array(node_property_steps),
        );

        let feature_steps: Vec<Value> = self
            .feature_steps
            .iter()
            .map(|step| {
                let step_map: serde_json::Map<String, Value> = step.to_map().into_iter().collect();
                Value::Object(step_map)
            })
            .collect();
        map.insert("featureProperties".to_string(), Value::Array(feature_steps));

        map
    }
    fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
        &self.node_property_steps
    }

    fn feature_steps(&self) -> &[Self::FeatureStep] {
        &self.feature_steps
    }

    fn specific_validate_before_execution(
        &self,
        _graph_store: &DefaultGraphStore,
    ) -> Result<(), crate::projection::native::ml::pipeline::PipelineValidationError> {
        // No specific validation for predict pipeline
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::native::ml::pipeline::ExecutableNodePropertyStep;

    #[test]
    fn test_empty_pipeline() {
        let pipeline = NodePropertyPredictPipeline::empty();
        assert_eq!(pipeline.node_property_steps().len(), 0);
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_new_pipeline() {
        let feature_step = NodeFeatureStep::of("age");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step]);

        assert_eq!(pipeline.node_property_steps().len(), 0);
        assert_eq!(pipeline.feature_steps().len(), 1);
    }

    #[test]
    fn test_feature_properties() {
        let feature_step1 = NodeFeatureStep::of("age");
        let feature_step2 = NodeFeatureStep::of("income");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step1, feature_step2]);

        let props = pipeline.feature_properties();
        assert_eq!(props.len(), 2);
        assert!(props.contains(&"age".to_string()));
        assert!(props.contains(&"income".to_string()));
    }

    #[test]
    fn test_to_map() {
        let feature_step = NodeFeatureStep::of("age");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step]);

        let map = pipeline.to_map();
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureProperties"));
        assert_eq!(map.get("featureProperties").unwrap().len(), 1);
    }

    #[test]
    fn test_add_feature_step() {
        let mut pipeline = NodePropertyPredictPipeline::empty();
        pipeline.add_feature_step(NodeFeatureStep::of("age"));

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].node_property(), "age");
    }

    #[test]
    fn test_specific_validate_before_execution() {
        use crate::types::random::RandomGraphConfig;

        let pipeline = NodePropertyPredictPipeline::empty();
        let graph_store = RandomGraphConfig::seeded(42)
            .node_count(100)
            .average_degree(5.0)
            .generate();

        assert!(pipeline
            .specific_validate_before_execution(&graph_store)
            .is_ok());
    }
}
