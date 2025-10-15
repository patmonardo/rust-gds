// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineTrainAlgorithm.java

use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::native::ml::pipeline::{
    PipelineTrainer, ResultToModelConverter, TrainingPipeline,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Abstract algorithm for training ML pipelines.
///
/// This trait orchestrates the complete training process:
/// 1. Validate training parameter space
/// 2. Validate pipeline against graph
/// 3. Run pipeline trainer (model selection + training)
/// 4. Convert result to catalog model
///
/// # Direct Integration Approach
///
/// Unlike Java's extends Algorithm<RESULT>, this is a trait with a default
/// `compute()` implementation. Takes Arc<DefaultGraphStore> directly.
///
/// # Type Parameters
///
/// * `RESULT` - Training result type (before conversion to model)
/// * `MODEL` - Catalog model container type
/// * `FEATURE_STEP` - Feature step type (e.g., NodeFeatureStep)
///
/// # Java Source (PipelineTrainAlgorithm.java)
/// ```java
/// public abstract class PipelineTrainAlgorithm<
///     RESULT,
///     MODEL_RESULT extends CatalogModelContainer<?, CONFIG, ?>,
///     CONFIG extends AlgoBaseConfig & ModelConfig,
///     FEATURE_STEP extends FeatureStep
/// > extends Algorithm<MODEL_RESULT> {
///     protected final TrainingPipeline<FEATURE_STEP> pipeline;
///     protected final GraphStore graphStore;
///     protected final CONFIG config;
///     private final PipelineTrainer<RESULT> pipelineTrainer;
///     private final ResultToModelConverter<MODEL_RESULT, RESULT> toCatalogModelConverter;
///     
///     @Override
///     public MODEL_RESULT compute() { /* ... */ }
/// }
/// ```
pub trait PipelineTrainAlgorithm<RESULT, MODEL, P: TrainingPipeline + ?Sized> {
    /// Access the training pipeline.
    fn pipeline(&self) -> &P;

    /// Access the graph store.
    fn graph_store(&self) -> &Arc<DefaultGraphStore>;

    /// Access the node labels for training.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for training.
    fn relationship_types(&self) -> &[String];

    /// Access the pipeline trainer.
    fn pipeline_trainer_mut(&mut self) -> &mut dyn PipelineTrainer<Result = RESULT>;

    /// Access the result-to-model converter.
    fn result_to_model_converter(&self) -> &dyn ResultToModelConverter<MODEL, RESULT>;

    /// Execute the complete training process (template method).
    ///
    /// This orchestrates:
    /// 1. Validate training parameter space (at least one model candidate)
    /// 2. Validate pipeline against graph
    /// 3. Capture original schema (before node property steps)
    /// 4. Run pipeline trainer (model selection + training)
    /// 5. Convert result to catalog model
    ///
    /// Java: `@Override public MODEL_RESULT compute()`
    fn compute(&mut self) -> Result<MODEL, PipelineTrainAlgorithmError> {
        // 1. Validate training parameter space
        self.pipeline()
            .validate_training_parameter_space()
            .map_err(|e| PipelineTrainAlgorithmError::ValidationFailed(e))?;

        // 2. Validate pipeline before execution
        self.pipeline()
            .validate_before_execution(self.graph_store(), self.node_labels())
            .map_err(|e| PipelineTrainAlgorithmError::ValidationFailed(Box::new(e)))?;

        // 3. Capture original schema (before node property steps)
        let node_labels_set: HashSet<NodeLabel> = self
            .node_labels()
            .iter()
            .map(|s| NodeLabel::of(s.as_str()))
            .collect();
        let rel_types_set: HashSet<RelationshipType> = self
            .relationship_types()
            .iter()
            .map(|s| RelationshipType::of(s.as_str()))
            .collect();

        let original_schema = self
            .graph_store()
            .schema()
            .filter_node_labels(&node_labels_set)
            .filter_relationship_types(&rel_types_set);

        // 4. Run pipeline trainer
        let pipeline_train_result = self
            .pipeline_trainer_mut()
            .run()
            .map_err(|e| PipelineTrainAlgorithmError::TrainingFailed(e))?;

        // 5. Convert result to catalog model
        let model = self
            .result_to_model_converter()
            .to_model(pipeline_train_result, &original_schema);

        Ok(model)
    }
}

/// Errors that can occur during pipeline training.
#[derive(Debug)]
pub enum PipelineTrainAlgorithmError {
    /// Pipeline validation failed.
    ValidationFailed(Box<dyn StdError>),

    /// Training execution failed.
    TrainingFailed(Box<dyn StdError>),

    /// Model conversion failed.
    ConversionFailed(String),
}

impl std::fmt::Display for PipelineTrainAlgorithmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationFailed(e) => write!(f, "Pipeline validation failed: {}", e),
            Self::TrainingFailed(e) => write!(f, "Training failed: {}", e),
            Self::ConversionFailed(msg) => write!(f, "Model conversion failed: {}", msg),
        }
    }
}

impl StdError for PipelineTrainAlgorithmError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ValidationFailed(e) | Self::TrainingFailed(e) => Some(&**e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = PipelineTrainAlgorithmError::ConversionFailed("bad format".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Model conversion failed"));
        assert!(display.contains("bad format"));
    }
}
