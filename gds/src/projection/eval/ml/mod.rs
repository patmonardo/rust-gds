//! ML runtime execution.
//!
//! This module contains runtime executors for ML pipelines.
//! Descriptors are in codegen/ml/, runtime is here.

pub mod features;
pub mod graph_procedure;
pub mod mock_property_values;
pub mod models;
pub mod pipeline;
pub mod pipeline_executor;
pub mod pipeline_state;
pub mod step_executor;
pub mod training_executor;

// Re-exports for convenience
pub use features::{
    DefaultFeatureAssembler, FeatureAssembler, IdentityTransformation, NormalizeTransformation,
    Transformation,
};
pub use graph_procedure::{
    create_mock_registry, GraphProcedure, GraphProcedureRegistry, MockFastRPProcedure,
    MockLouvainProcedure, MockPageRankProcedure,
};
pub use mock_property_values::{
    MockDoublePropertyValues, MockEmbeddingPropertyValues, MockLongPropertyValues,
};
pub use models::{DecisionTreeClassifier, Model, ModelError, ModelMetadata};
pub use pipeline::{ExecutableNodePropertyStep, FeatureStep, Pipeline, PipelineValidationError};
pub use pipeline_executor::{PipelineExecutor, PipelineResult};
pub use pipeline_state::{DatasetSplits, ExecutionPhase, PipelineState};
pub use step_executor::{
    create_step_executor, FeatureStepExecutor, NodePropertyStepExecutor, StepExecutor, StepResult,
};
pub use training_executor::{TrainingError, TrainingExecutor, TrainingResult, TrainingStatistics};
