//! ML Descriptor Types
//!
//! Descriptor types for ML pipelines, models, and training configurations.
//! These map to Java GDS ml-algo package structures.
//!
//! ## Organization
//!
//! - `model.rs` - ModelDescriptor (architecture + hyperparameters)
//! - `pipeline.rs` - ML PipelineDescriptor (complete ML workflows)
//! - `step.rs` - StepDescriptor (feature + node property steps)
//! - `training.rs` - TrainingDescriptor (training configuration)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::descriptors::ml::*;
//!
//! let pipeline = PipelineDescriptor::new("NodeClassification")
//!     .with_type(PipelineType::NodeClassification);
//! ```

pub mod model;
pub mod pipeline;
pub mod step;
pub mod training;

// Re-exports
pub use model::ModelDescriptor;
pub use pipeline::{
    AutoTuningConfig, ModelCandidate, ModelType, PipelineDescriptor, PipelineType, SplitConfig,
    TrainingConfig, ValidationMetric,
};
pub use step::{FeatureStepDescriptor, FeatureType, NodePropertyStepDescriptor, StepDescriptor};
pub use training::TrainingDescriptor;
