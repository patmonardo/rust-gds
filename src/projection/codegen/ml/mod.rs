//! ML code generation and descriptors.
//!
//! This module contains descriptor types for ML pipelines, models, and training.
//! Follows the projection pattern: descriptors here, runtime in native/ml/.

pub mod model_descriptor;
pub mod pipeline_descriptor;
pub mod step_descriptor;
pub mod training_descriptor;

// Re-exports
pub use pipeline_descriptor::{
    AutoTuningConfig, ModelCandidate, ModelType, PipelineDescriptor, PipelineDescriptorBuilder,
    PipelineMetadata, PipelineType, SplitConfig, TrainingConfig, ValidationMetric,
};
pub use step_descriptor::{FeatureStepDescriptor, NodePropertyStepDescriptor};
