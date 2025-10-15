// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PredictPipelineExecutor.java

use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::native::ml::pipeline::{
    NodePropertyStepExecutor, Pipeline, PipelineGraphFilter,
};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Abstract pipeline executor for prediction (no training/test splits).
///
/// This is a simplified executor for running trained models on new data.
/// Unlike PipelineExecutor, this doesn't split datasets or train models -
/// it just runs node property steps and executes prediction.
///
/// # Direct Integration Approach
///
/// Takes Arc<DefaultGraphStore> directly instead of wrapping in ExecutionContext.
///
/// # Type Parameters
///
/// * `PIPELINE` - The pipeline type (e.g., NodeClassificationPipeline)
/// * `RESULT` - The prediction result type
///
/// # Java Source (PredictPipelineExecutor.java)
/// ```java
/// public abstract class PredictPipelineExecutor<
///     PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
///     PIPELINE extends Pipeline<?>,
///     RESULT
/// > extends Algorithm<RESULT> {
///     protected final PIPELINE pipeline;
///     protected final PIPELINE_CONFIG config;
///     protected final ExecutionContext executionContext;
///     protected final GraphStore graphStore;
///     
///     protected abstract RESULT execute();
///     protected abstract PipelineGraphFilter nodePropertyStepFilter();
///     
///     @Override
///     public RESULT compute() { /* template method */ }
/// }
/// ```
pub trait PredictPipelineExecutor<PIPELINE: Pipeline, RESULT> {
    /// Access the pipeline being executed.
    fn pipeline(&self) -> &PIPELINE;

    /// Access the graph store (mutable for property steps).
    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore>;

    /// Access the graph store (immutable).
    fn graph_store(&self) -> &Arc<DefaultGraphStore>;

    /// Access the node labels for this execution.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for this execution.
    fn relationship_types(&self) -> &[String];

    /// Access the concurrency setting.
    fn concurrency(&self) -> usize;

    /// Execute the prediction algorithm.
    ///
    /// Called after node property steps are executed and features are validated.
    ///
    /// Java: `abstract RESULT execute()`
    fn execute(&mut self) -> Result<RESULT, PredictPipelineExecutorError>;

    /// Get the graph filter for node property steps.
    ///
    /// This defines which nodes/relationships to use for feature extraction.
    /// In prediction, this typically includes all nodes to predict on plus
    /// any context nodes needed for algorithms.
    ///
    /// Java: `abstract PipelineGraphFilter nodePropertyStepFilter()`
    fn node_property_step_filter(&self) -> PipelineGraphFilter;

    /// Execute the complete prediction pipeline (template method).
    ///
    /// This implements the standard prediction flow:
    /// 1. Get node property step filter
    /// 2. Validate pipeline against graph
    /// 3. Create node property step executor
    /// 4. Execute node property steps
    /// 5. Validate feature properties
    /// 6. Execute prediction
    /// 7. Cleanup intermediate properties
    ///
    /// Java: `@Override public RESULT compute()`
    fn compute(&mut self) -> Result<RESULT, PredictPipelineExecutorError> {
        // 1. Get node property step filter
        let node_property_step_filter = self.node_property_step_filter();

        // 2. Validate pipeline before execution
        // featureInput nodeLabels contain nodes to predict on plus contextNodeLabels
        self.pipeline()
            .validate_before_execution(&*self.graph_store(), &node_property_step_filter.node_labels)
            .map_err(|e| PredictPipelineExecutorError::PipelineValidationFailed(Box::new(e)))?;

        // 3. Create node property step executor
        // For prediction, all relationship types in the graph are available
        let all_relationship_types: std::collections::HashSet<String> = self
            .graph_store()
            .relationship_types()
            .iter()
            .map(|rt| rt.name().to_string())
            .collect();

        let mut node_property_step_executor = NodePropertyStepExecutor::new(
            self.graph_store().clone(),
            node_property_step_filter.node_labels.clone(),
            node_property_step_filter.relationship_types.clone(),
            all_relationship_types,
            self.concurrency(),
        );

        // 4-7. Execute steps, prediction, and cleanup
        let result: Result<RESULT, PredictPipelineExecutorError> = (|| {
            // 4. Execute node property steps
            // Note: We don't validate the size of the feature-input graph as not every
            // nodePropertyStep needs relationships
            node_property_step_executor
                .execute_node_property_steps(self.pipeline().node_property_steps())
                .map_err(|e| PredictPipelineExecutorError::StepExecutionFailed(Box::new(e)))?;

            // 5. Validate feature properties
            self.pipeline()
                .validate_feature_properties(
                    &*self.graph_store(),
                    &node_property_step_filter.node_labels,
                )
                .map_err(|e| PredictPipelineExecutorError::FeatureValidationFailed(Box::new(e)))?;

            // 6. Execute prediction
            self.execute()
        })();

        // 7. Cleanup (always runs, even if error occurred)
        let cleanup_error = node_property_step_executor
            .cleanup_intermediate_properties(self.pipeline().node_property_steps())
            .err();

        // Return result, or cleanup error if result was Ok
        match (result, cleanup_error) {
            (Ok(value), None) => Ok(value),
            (Ok(_), Some(e)) => Err(PredictPipelineExecutorError::CleanupFailed(Box::new(e))),
            (Err(e), _) => Err(e),
        }
    }
}

/// Errors that can occur during prediction pipeline execution.
#[derive(Debug)]
pub enum PredictPipelineExecutorError {
    /// Pipeline validation failed before execution.
    PipelineValidationFailed(Box<dyn StdError>),

    /// Step execution failed.
    StepExecutionFailed(Box<dyn StdError>),

    /// Feature validation failed after steps.
    FeatureValidationFailed(Box<dyn StdError>),

    /// Prediction execution failed.
    ExecutionFailed(String),

    /// Cleanup failed.
    CleanupFailed(Box<dyn StdError>),
}

impl std::fmt::Display for PredictPipelineExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PipelineValidationFailed(e) => {
                write!(f, "Pipeline validation failed: {}", e)
            }
            Self::StepExecutionFailed(e) => {
                write!(f, "Step execution failed: {}", e)
            }
            Self::FeatureValidationFailed(e) => {
                write!(f, "Feature validation failed: {}", e)
            }
            Self::ExecutionFailed(msg) => {
                write!(f, "Prediction execution failed: {}", msg)
            }
            Self::CleanupFailed(e) => {
                write!(f, "Cleanup failed: {}", e)
            }
        }
    }
}

impl StdError for PredictPipelineExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::PipelineValidationFailed(e)
            | Self::StepExecutionFailed(e)
            | Self::FeatureValidationFailed(e)
            | Self::CleanupFailed(e) => Some(&**e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = PredictPipelineExecutorError::ExecutionFailed("prediction error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("prediction error"));
        assert!(display.contains("Prediction execution failed"));
    }
}
