//! Golden tests for ML pipeline descriptor translation.
//!
//! Validates that pipeline descriptors can be created, serialized, and round-tripped.

use rust_gds::projection::codegen::ml::{
    pipeline_descriptor::{
        AutoTuningConfig, ModelCandidate, ModelType, PipelineConfig, PipelineDescriptor,
        PipelineType, SearchStrategy, SplitConfig, TrainingConfig, ValidationConfig,
        ValidationMetric,
    },
    step_descriptor::{
        FeatureStepDescriptor, FeatureType, NodePropertyStepDescriptor, StepDescriptor,
    },
};
use std::collections::HashMap;

#[test]
fn test_pipeline_descriptor_creation() {
    let pipeline = PipelineDescriptor::builder(
        "test-pipeline".to_string(),
        PipelineType::NodeClassification {
            target_property: "label".to_string(),
        },
    )
    .add_step(StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
        "pagerank-step".to_string(),
        "pageRank".to_string(),
        "pageRank".to_string(),
    )))
    .add_step(StepDescriptor::Feature(FeatureStepDescriptor::new(
        "feature-extraction".to_string(),
        FeatureType::Scalar,
        vec!["pageRank".to_string()],
    )))
    .training_config(TrainingConfig {
        model_candidates: vec![ModelCandidate {
            model_type: ModelType::LogisticRegression,
            params: HashMap::new(),
        }],
        split_config: SplitConfig {
            train_fraction: 0.6,
            validation_fraction: 0.2,
            test_fraction: 0.2,
            seed: 42,
            stratify: true,
        },
        validation_metric: ValidationMetric::Accuracy,
    })
    .pipeline_config(PipelineConfig {
        auto_tuning: Some(AutoTuningConfig {
            max_trials: 10,
            search_strategy: SearchStrategy::GridSearch,
        }),
        validation: Some(ValidationConfig {
            validation_fraction: 0.2,
        }),
    })
    .build()
    .expect("Pipeline build failed");

    assert_eq!(pipeline.name, "test-pipeline");
    assert_eq!(pipeline.steps.len(), 2);
}

#[test]
fn test_pipeline_descriptor_serialization() {
    let pipeline = PipelineDescriptor::builder(
        "serialization-test".to_string(),
        PipelineType::NodeRegression {
            target_property: "value".to_string(),
        },
    )
    .training_config(TrainingConfig {
        model_candidates: vec![ModelCandidate {
            model_type: ModelType::LinearRegression,
            params: HashMap::new(),
        }],
        split_config: SplitConfig {
            train_fraction: 0.5,
            validation_fraction: 0.3,
            test_fraction: 0.2,
            seed: 123,
            stratify: false,
        },
        validation_metric: ValidationMetric::R2,
    })
    .pipeline_config(PipelineConfig {
        auto_tuning: None,
        validation: Some(ValidationConfig {
            validation_fraction: 0.3,
        }),
    })
    .build()
    .expect("Pipeline build failed");

    let json = serde_json::to_string(&pipeline).expect("serialization failed");
    let deserialized: PipelineDescriptor =
        serde_json::from_str(&json).expect("deserialization failed");

    assert_eq!(deserialized.name, "serialization-test");
    assert_eq!(
        deserialized
            .config
            .validation
            .as_ref()
            .unwrap()
            .validation_fraction,
        0.3
    );
}

#[test]
fn test_node_property_step_creation() {
    let step = NodePropertyStepDescriptor::new(
        "test-step".to_string(),
        "fastRP".to_string(),
        "embedding".to_string(),
    );

    assert_eq!(step.name, "test-step");
    assert_eq!(step.algorithm, "fastRP");
    assert_eq!(step.property_name, "embedding");
}

#[test]
fn test_feature_step_with_dimension() {
    let step = FeatureStepDescriptor::new(
        "feature-step".to_string(),
        FeatureType::Array,
        vec!["prop1".to_string(), "prop2".to_string()],
    )
    .with_target_dimension(128);

    assert_eq!(step.target_dimension, Some(128));
}
