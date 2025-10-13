//! Golden tests for ML pipeline descriptor translation.
//!
//! Validates that pipeline descriptors can be created, serialized, and round-tripped.

use rust_gds::projection::codegen::ml::{
    pipeline_descriptor::{AutoTuningConfig, PipelineConfig, PipelineDescriptor, ValidationConfig},
    step_descriptor::{
        FeatureStepDescriptor, FeatureType, NodePropertyStepDescriptor, StepDescriptor,
    },
};

#[test]
fn test_pipeline_descriptor_creation() {
    let pipeline = PipelineDescriptor::new(
        "test-pipeline".to_string(),
        vec![
            StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
                "pagerank-step".to_string(),
                "pageRank".to_string(),
                "pageRank".to_string(),
            )),
            StepDescriptor::Feature(FeatureStepDescriptor::new(
                "feature-extraction".to_string(),
                FeatureType::Scalar,
                vec!["pageRank".to_string()],
            )),
        ],
        PipelineConfig {
            auto_tuning: Some(AutoTuningConfig { max_trials: 10 }),
            validation: ValidationConfig {
                validation_fraction: 0.2,
            },
        },
    );

    assert_eq!(pipeline.name, "test-pipeline");
    assert_eq!(pipeline.steps.len(), 2);
}

#[test]
fn test_pipeline_descriptor_serialization() {
    let pipeline = PipelineDescriptor::new(
        "serialization-test".to_string(),
        vec![],
        PipelineConfig {
            auto_tuning: None,
            validation: ValidationConfig {
                validation_fraction: 0.3,
            },
        },
    );

    let json = serde_json::to_string(&pipeline).expect("serialization failed");
    let deserialized: PipelineDescriptor =
        serde_json::from_str(&json).expect("deserialization failed");

    assert_eq!(deserialized.name, "serialization-test");
    assert_eq!(deserialized.config.validation.validation_fraction, 0.3);
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
