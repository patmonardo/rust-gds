// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineExecutor.java

use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::native::ml::pipeline::{
    NodePropertyStepExecutor, Pipeline, PipelineGraphFilter,
};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::schema::GraphSchema;

/// Dataset split types for training and evaluation pipelines.
///
/// Java GDS defines this as a nested enum inside PipelineExecutor:
/// ```java
/// public enum DatasetSplits {
///     TRAIN,
///     TEST,
///     TEST_COMPLEMENT,
///     FEATURE_INPUT
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetSplits {
    /// Training dataset split.
    Train,
    /// Test dataset split (for evaluation).
    Test,
    /// Test complement (all nodes not in test set).
    TestComplement,
    /// Feature input (nodes used for feature extraction, includes train+test+context).
    FeatureInput,
}

/// Abstract pipeline executor for training and evaluation.
///
/// This trait implements the template method pattern, providing a standard
/// execution flow (compute()) while allowing subclasses to customize specific
/// steps via abstract methods.
///
/// # Direct Integration Approach
///
/// Unlike Java's ExecutionContext wrapper, this takes Arc<DefaultGraphStore>
/// directly, making GraphStore access explicit and clear.
///
/// # Type Parameters
///
/// * `PIPELINE` - The pipeline type (e.g., NodeClassificationPipeline)
/// * `RESULT` - The result type (e.g., classification predictions)
///
/// # Java Source (PipelineExecutor.java)
/// ```java
/// public abstract class PipelineExecutor<
///     PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
///     PIPELINE extends Pipeline<?>,
///     RESULT
/// > extends Algorithm<RESULT> {
///     protected final PIPELINE pipeline;
///     protected final PIPELINE_CONFIG config;
///     protected final ExecutionContext executionContext;
///     protected final GraphStore graphStore;
///     protected final GraphSchema schemaBeforeSteps;
///     
///     public abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters();
///     public abstract void splitDatasets();
///     protected abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits);
///     protected abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps();
///     
///     @Override
///     public RESULT compute() { /* template method */ }
/// }
/// ```
pub trait PipelineExecutor<PIPELINE: Pipeline, RESULT> {
    /// Access the pipeline being executed.
    fn pipeline(&self) -> &PIPELINE;

    /// Access the graph store (mutable for property steps).
    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore>;

    /// Access the graph store (immutable).
    fn graph_store(&self) -> &Arc<DefaultGraphStore>;

    /// Access the schema before node property steps were executed.
    ///
    /// This captures the original graph schema, useful for validation
    /// and determining which properties were added by the pipeline.
    fn schema_before_steps(&self) -> &GraphSchema;

    /// Access the node labels for this execution.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for this execution.
    fn relationship_types(&self) -> &[String];

    /// Access the concurrency setting.
    fn concurrency(&self) -> usize;

    /// Generate dataset split graph filters.
    ///
    /// Creates filters for:
    /// - TRAIN: Training data nodes/relationships
    /// - TEST: Test data nodes/relationships
    /// - TEST_COMPLEMENT: All nodes not in test set
    /// - FEATURE_INPUT: All nodes used for feature extraction (train+test+context)
    ///
    /// Java: `abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters()`
    fn generate_dataset_split_graph_filters(&self) -> HashMap<DatasetSplits, PipelineGraphFilter>;

    /// Split the dataset into train/test partitions.
    ///
    /// This may create properties or modify graph state to mark splits.
    /// Called after filters are generated but before step execution.
    ///
    /// Java: `abstract void splitDatasets()`
    fn split_datasets(&mut self) -> Result<(), PipelineExecutorError>;

    /// Execute the pipeline algorithm on the split datasets.
    ///
    /// Called after:
    /// - Dataset splits are created
    /// - Node property steps are executed
    /// - Feature properties are validated
    ///
    /// Java: `abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits)`
    fn execute(
        &mut self,
        data_splits: &HashMap<DatasetSplits, PipelineGraphFilter>,
    ) -> Result<RESULT, PipelineExecutorError>;

    /// Get relationship types available for node property steps.
    ///
    /// This determines which relationship types can be used by algorithms
    /// during feature extraction.
    ///
    /// Java: `abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps()`
    fn get_available_rel_types_for_node_property_steps(&self) -> HashSet<String>;

    /// Additional graph store cleanup after execution.
    ///
    /// Override to clean up temporary properties or state after pipeline runs.
    /// Default implementation does nothing.
    ///
    /// Java: `protected void additionalGraphStoreCleanup(Map<DatasetSplits, PipelineGraphFilter> datasets)`
    fn additional_graph_store_cleanup(
        &mut self,
        _datasets: &HashMap<DatasetSplits, PipelineGraphFilter>,
    ) -> Result<(), PipelineExecutorError> {
        Ok(())
    }

    /// Execute the complete pipeline (template method).
    ///
    /// This implements the standard pipeline execution flow:
    /// 1. Generate dataset split filters
    /// 2. Validate pipeline against graph
    /// 3. Create node property step executor
    /// 4. Validate step context configs
    /// 5. Split datasets
    /// 6. Execute node property steps
    /// 7. Validate feature properties
    /// 8. Execute algorithm (train/test/predict)
    /// 9. Cleanup intermediate properties
    ///
    /// Java: `@Override public RESULT compute()`
    fn compute(&mut self) -> Result<RESULT, PipelineExecutorError> {
        // 1. Generate dataset split filters
        let data_split_graph_filters = self.generate_dataset_split_graph_filters();
        let feature_input_graph_filter = data_split_graph_filters
            .get(&DatasetSplits::FeatureInput)
            .ok_or_else(|| {
                PipelineExecutorError::MissingDatasetSplit("FEATURE_INPUT".to_string())
            })?;

        // 2. Validate pipeline before execution
        // featureInput nodeLabels contain source&target nodeLabel used in training/testing plus contextNodeLabels
        self.pipeline()
            .validate_before_execution(
                &*self.graph_store(),
                &feature_input_graph_filter.node_labels,
            )
            .map_err(|e| PipelineExecutorError::PipelineValidationFailed(Box::new(e)))?;

        // 3. Create node property step executor
        let mut node_property_step_executor = NodePropertyStepExecutor::new(
            self.graph_store().clone(),
            feature_input_graph_filter.node_labels.clone(),
            feature_input_graph_filter.relationship_types.clone(),
            self.get_available_rel_types_for_node_property_steps(),
            self.concurrency(),
        );

        // 4. Validate node property steps context configs
        node_property_step_executor
            .validate_node_property_steps_context_configs(self.pipeline().node_property_steps())
            .map_err(|e| PipelineExecutorError::StepValidationFailed(Box::new(e)))?;

        // 5. Split datasets
        self.split_datasets()?;

        // 6-9. Execute steps, algorithm, and cleanup
        let result: Result<RESULT, PipelineExecutorError> = (|| {
            // 6. Execute node property steps
            // Note: We don't validate the size of the feature-input graph as not every
            // nodePropertyStep needs relationships
            node_property_step_executor
                .execute_node_property_steps(self.pipeline().node_property_steps())
                .map_err(|e| PipelineExecutorError::StepExecutionFailed(Box::new(e)))?;

            // 7. Validate feature properties
            self.pipeline()
                .validate_feature_properties(&*self.graph_store(), self.node_labels())
                .map_err(|e| PipelineExecutorError::FeatureValidationFailed(Box::new(e)))?;

            // 8. Execute algorithm
            self.execute(&data_split_graph_filters)
        })();

        // 9. Cleanup (always runs, even if error occurred)
        let cleanup_result = (|| -> Result<(), PipelineExecutorError> {
            node_property_step_executor
                .cleanup_intermediate_properties(self.pipeline().node_property_steps())
                .map_err(|e| PipelineExecutorError::CleanupFailed(Box::new(e)))?;

            self.additional_graph_store_cleanup(&data_split_graph_filters)?;

            Ok(())
        })();

        // Return result, or cleanup error if result was Ok
        match (result, cleanup_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        }
    }
}

/// Errors that can occur during pipeline execution.
#[derive(Debug)]
pub enum PipelineExecutorError {
    /// Dataset split is missing from the filter map.
    MissingDatasetSplit(String),

    /// Pipeline validation failed before execution.
    PipelineValidationFailed(Box<dyn StdError>),

    /// Step validation failed (context configs invalid).
    StepValidationFailed(Box<dyn StdError>),

    /// Dataset splitting failed.
    DatasetSplitFailed(String),

    /// Step execution failed.
    StepExecutionFailed(Box<dyn StdError>),

    /// Feature validation failed after steps.
    FeatureValidationFailed(Box<dyn StdError>),

    /// Algorithm execution failed.
    ExecutionFailed(String),

    /// Cleanup failed.
    CleanupFailed(Box<dyn StdError>),
}

impl std::fmt::Display for PipelineExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDatasetSplit(split) => {
                write!(f, "Missing dataset split: {}", split)
            }
            Self::PipelineValidationFailed(e) => {
                write!(f, "Pipeline validation failed: {}", e)
            }
            Self::StepValidationFailed(e) => {
                write!(f, "Step validation failed: {}", e)
            }
            Self::DatasetSplitFailed(msg) => {
                write!(f, "Dataset splitting failed: {}", msg)
            }
            Self::StepExecutionFailed(e) => {
                write!(f, "Step execution failed: {}", e)
            }
            Self::FeatureValidationFailed(e) => {
                write!(f, "Feature validation failed: {}", e)
            }
            Self::ExecutionFailed(msg) => {
                write!(f, "Execution failed: {}", msg)
            }
            Self::CleanupFailed(e) => {
                write!(f, "Cleanup failed: {}", e)
            }
        }
    }
}

impl StdError for PipelineExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::PipelineValidationFailed(e)
            | Self::StepValidationFailed(e)
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
    fn test_dataset_splits_enum() {
        // Test enum variants exist
        let train = DatasetSplits::Train;
        let test = DatasetSplits::Test;
        let test_complement = DatasetSplits::TestComplement;
        let feature_input = DatasetSplits::FeatureInput;

        // Test equality
        assert_eq!(train, DatasetSplits::Train);
        assert_ne!(train, test);

        // Test hash (can use in HashMap)
        let mut map = HashMap::new();
        map.insert(train, "train data");
        map.insert(test, "test data");
        assert_eq!(map.get(&DatasetSplits::Train), Some(&"train data"));
    }

    #[test]
    fn test_error_display() {
        let error = PipelineExecutorError::MissingDatasetSplit("TRAIN".to_string());
        let display = format!("{}", error);
        assert!(display.contains("TRAIN"));

        let error = PipelineExecutorError::ExecutionFailed("algorithm error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("algorithm error"));
    }
}
