//! Pipeline module - Java GDS ML Pipeline translation
//!
//! **Module Organization**: 1:1 mapping to Java `org.neo4j.gds.ml.pipeline` package.
//!
//! ```
//! Java:                                    Rust:
//! org.neo4j.gds.ml.pipeline/              src/projection/native/ml/pipeline/
//! ├── Pipeline.java                  →    ├── pipeline_trait.rs
//! ├── FeatureStep.java               →    ├── feature_step.rs
//! ├── ExecutableNodePropertyStep.java →   ├── executable_node_property_step.rs
//! ├── PipelineExecutor.java          →    ├── pipeline_executor.rs (TODO)
//! ├── NodePropertyStep.java          →    ├── node_property_step.rs (TODO)
//! └── ...                                  └── ...
//! ```

pub mod auto_tuning_config;
pub mod executable_node_property_step;
pub mod feature_step;
pub mod feature_step_util;
pub mod node_property_step;
pub mod node_property_step_context_config;
pub mod node_property_step_executor;
pub mod node_property_step_factory;
pub mod non_empty_set_validation;
pub mod pipeline_catalog;
pub mod pipeline_companion;
pub mod pipeline_executor;
pub mod pipeline_graph_filter;
pub mod pipeline_train_algorithm;
pub mod pipeline_trainer;
pub mod pipeline_trait;
pub mod predict_pipeline_executor;
pub mod result_to_model_converter;
pub mod training_pipeline;

// Sub-packages
pub mod link_pipeline;
pub mod node_pipeline;

// Re-exports for convenience
pub use auto_tuning_config::{AutoTuningConfig, AutoTuningConfigError};
pub use executable_node_property_step::ExecutableNodePropertyStep;
pub use feature_step::FeatureStep;
pub use feature_step_util::{
    property_dimension, throw_nan_error, validate_computed_features, FeatureStepError,
};
pub use node_property_step::{NodePropertyStep, NodePropertyStepError, MUTATE_PROPERTY_KEY};
pub use node_property_step_context_config::NodePropertyStepContextConfig;
pub use node_property_step_executor::{NodePropertyStepExecutor, NodePropertyStepExecutorError};
pub use node_property_step_factory::{
    create_node_property_step, create_node_property_step_with_context, NodePropertyStepFactoryError,
};
pub use non_empty_set_validation::{
    validate_node_set_size, validate_rel_set_size, ValidationError, MIN_SET_SIZE,
    MIN_TEST_COMPLEMENT_SET_SIZE, MIN_TRAIN_SET_SIZE,
};
pub use pipeline_catalog::{PipelineCatalog, PipelineCatalogEntry};
pub use pipeline_companion::{
    prepare_pipeline_config, validate_main_metric, PipelineCompanionError, ANONYMOUS_GRAPH,
    OUT_OF_BAG_ERROR,
};
pub use pipeline_executor::{DatasetSplits, PipelineExecutor, PipelineExecutorError};
pub use pipeline_graph_filter::PipelineGraphFilter;
pub use pipeline_train_algorithm::{PipelineTrainAlgorithm, PipelineTrainAlgorithmError};
pub use pipeline_trainer::PipelineTrainer;
pub use pipeline_trait::{Pipeline, PipelineValidationError};
pub use predict_pipeline_executor::{PredictPipelineExecutor, PredictPipelineExecutorError};
pub use result_to_model_converter::ResultToModelConverter;
pub use training_pipeline::{TrainingMethod, TrainingPipeline, TrainingType, TunableTrainerConfig};
