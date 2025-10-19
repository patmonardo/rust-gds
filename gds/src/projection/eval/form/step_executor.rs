//! Step executor - individual pipeline step execution.
//!
//! This module defines the StepExecutor trait and implementations for:
//! - NodeProperty steps (execute algorithms and write properties)
//! - Feature steps (compute graph embeddings/features)
//!
//! Design follows Java GDS NodePropertyStepExecutor.java pattern.

use std::sync::Arc;

use crate::projection::codegen::ml::step_descriptor::{
    FeatureStepDescriptor, NodePropertyStepDescriptor,
};
use crate::projection::codegen::runtimes::computation::ComputeError;
use crate::projection::eval::form::pipeline_state::PipelineState;
use crate::types::graph::Graph;
use crate::types::properties::PropertyValues;

/// Result of executing a pipeline step
#[derive(Debug)]
pub struct StepResult {
    /// Step executed successfully
    pub success: bool,

    /// Name of feature/property produced (if any)
    pub output_name: Option<String>,

    /// Descriptive message
    pub message: String,
}

impl StepResult {
    pub fn success(output_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: true,
            output_name: Some(output_name.into()),
            message: message.into(),
        }
    }

    pub fn success_no_output(message: impl Into<String>) -> Self {
        Self {
            success: true,
            output_name: None,
            message: message.into(),
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            output_name: None,
            message: message.into(),
        }
    }
}

/// Trait for executing individual pipeline steps.
///
/// Each step type (NodeProperty, Feature) implements this trait.
/// The executor handles:
/// - Validation of step configuration
/// - Execution of step logic
/// - Storage of results in PipelineState
pub trait StepExecutor: Send + Sync {
    /// Execute this step
    fn execute(
        &self,
        graph: &Arc<dyn Graph>,
        state: &mut PipelineState,
    ) -> Result<StepResult, ComputeError>;

    /// Validate step configuration (called during init phase)
    fn validate(&self) -> Result<(), ComputeError>;

    /// Get descriptive name for this step
    fn name(&self) -> &str;
}

/// Executor for NodeProperty steps.
///
/// Executes graph algorithms and writes results as node properties.
/// This allows subsequent steps to use algorithm results as features.
///
/// Maps to Java GDS ExecutableNodePropertyStep.
pub struct NodePropertyStepExecutor {
    descriptor: NodePropertyStepDescriptor,
}

impl NodePropertyStepExecutor {
    pub fn new(descriptor: NodePropertyStepDescriptor) -> Self {
        Self { descriptor }
    }

    /// Execute algorithm and extract property from graph
    fn execute_algorithm(
        &self,
        graph: &Arc<dyn Graph>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Phase 2.2: Mock property values
        // Phase 2.3+: Replace with actual algorithm execution
        // Full implementation will:
        // 1. Look up algorithm by name from AlgorithmRegistry
        // 2. Execute algorithm with config
        // 3. Write property to graph
        // 4. Return PropertyValues

        use crate::projection::eval::form::MockLongPropertyValues;

        let node_count = graph.node_count();

        // Generate mock property values
        // In Phase 2.3+, this will actually run PageRank, Louvain, etc.
        let mock_values = MockLongPropertyValues::new(node_count);

        Ok(Arc::new(mock_values) as Arc<dyn PropertyValues>)
    }
}

impl StepExecutor for NodePropertyStepExecutor {
    fn execute(
        &self,
        graph: &Arc<dyn Graph>,
        state: &mut PipelineState,
    ) -> Result<StepResult, ComputeError> {
        // Execute algorithm
        let property_values = self.execute_algorithm(graph)?;

        // Store in pipeline state
        let property_name = self.descriptor.property_name.clone();
        state.add_feature(property_name.clone(), property_values);

        Ok(StepResult::success(
            property_name,
            format!(
                "algorithm '{}' executed successfully",
                self.descriptor.algorithm
            ),
        ))
    }

    fn validate(&self) -> Result<(), ComputeError> {
        // Validate property name is not empty
        if self.descriptor.property_name.is_empty() {
            return Err(ComputeError::InitFailed(
                "property name cannot be empty".into(),
            ));
        }

        // Validate algorithm name is not empty
        if self.descriptor.algorithm.is_empty() {
            return Err(ComputeError::InitFailed(
                "algorithm name cannot be empty".into(),
            ));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.descriptor.name
    }
}

/// Executor for Feature steps.
///
/// Extracts features from source properties and optionally performs
/// dimensionality reduction/projection.
///
/// Phase 2.1: Stub implementation (placeholder)
/// Phase 2.2: Full feature computation
pub struct FeatureStepExecutor {
    descriptor: FeatureStepDescriptor,
}

impl FeatureStepExecutor {
    pub fn new(descriptor: FeatureStepDescriptor) -> Self {
        Self { descriptor }
    }

    /// Compute feature from source properties
    fn compute_feature(
        &self,
        graph: &Arc<dyn Graph>,
        _state: &PipelineState,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Phase 2.2: Mock feature computation based on feature_type
        // Phase 2.3+: Replace with actual algorithm implementations

        use crate::projection::eval::form::MockEmbeddingPropertyValues;

        let node_count = graph.node_count();
        let dimension = self.descriptor.target_dimension.unwrap_or(128);

        // Generate mock embeddings
        // In Phase 2.3+, this will call actual FastRP, Node2Vec, etc.
        let mock_values = MockEmbeddingPropertyValues::new(node_count, dimension);

        Ok(Arc::new(mock_values) as Arc<dyn PropertyValues>)
    }
}

impl StepExecutor for FeatureStepExecutor {
    fn execute(
        &self,
        graph: &Arc<dyn Graph>,
        state: &mut PipelineState,
    ) -> Result<StepResult, ComputeError> {
        // Compute feature
        let feature_values = self.compute_feature(graph, state)?;

        // Use step name as feature name
        let feature_name = self.descriptor.name.clone();

        // Store in pipeline state
        state.add_feature(feature_name.clone(), feature_values);

        Ok(StepResult::success(
            feature_name,
            "feature computed successfully",
        ))
    }

    fn validate(&self) -> Result<(), ComputeError> {
        // Validate step name
        if self.descriptor.name.is_empty() {
            return Err(ComputeError::InitFailed("step name cannot be empty".into()));
        }

        // Validate source properties
        if self.descriptor.source_properties.is_empty() {
            return Err(ComputeError::InitFailed(
                "feature step must have at least one source property".into(),
            ));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.descriptor.name
    }
}

/// Factory function to create step executor from descriptor
pub fn create_step_executor(
    descriptor: &crate::projection::codegen::ml::step_descriptor::StepDescriptor,
) -> Box<dyn StepExecutor> {
    use crate::projection::codegen::ml::step_descriptor::StepDescriptor;

    match descriptor {
        StepDescriptor::NodeProperty(desc) => Box::new(NodePropertyStepExecutor::new(desc.clone())),
        StepDescriptor::Feature(desc) => Box::new(FeatureStepExecutor::new(desc.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::codegen::ml::step_descriptor::FeatureType;

    #[test]
    fn test_step_result_success() {
        let result = StepResult::success("feature_name", "computed successfully");
        assert!(result.success);
        assert_eq!(result.output_name, Some("feature_name".to_string()));
        assert_eq!(result.message, "computed successfully");
    }

    #[test]
    fn test_step_result_failure() {
        let result = StepResult::failure("computation failed");
        assert!(!result.success);
        assert_eq!(result.output_name, None);
        assert_eq!(result.message, "computation failed");
    }

    #[test]
    fn test_node_property_executor_validate() {
        let descriptor = NodePropertyStepDescriptor::new(
            "pagerank_step".into(),
            "pageRank".into(),
            "pr_score".into(),
        );

        let executor = NodePropertyStepExecutor::new(descriptor);
        assert!(executor.validate().is_ok());
        assert_eq!(executor.name(), "pagerank_step");
    }

    #[test]
    fn test_node_property_executor_validate_empty_property() {
        let descriptor = NodePropertyStepDescriptor::new(
            "step1".into(),
            "pageRank".into(),
            "".into(), // empty property name
        );

        let executor = NodePropertyStepExecutor::new(descriptor);
        let result = executor.validate();

        assert!(result.is_err());
        assert!(matches!(result, Err(ComputeError::InitFailed(_))));
    }

    #[test]
    fn test_node_property_executor_validate_empty_algorithm() {
        let descriptor = NodePropertyStepDescriptor::new(
            "step1".into(),
            "".into(), // empty algorithm
            "prop".into(),
        );

        let executor = NodePropertyStepExecutor::new(descriptor);
        let result = executor.validate();

        assert!(result.is_err());
        assert!(matches!(result, Err(ComputeError::InitFailed(_))));
    }

    #[test]
    fn test_feature_executor_creation() {
        let descriptor = FeatureStepDescriptor::new(
            "fastrp_feature".into(),
            FeatureType::Embedding,
            vec!["prop1".into(), "prop2".into()],
        );

        let executor = FeatureStepExecutor::new(descriptor);
        assert!(executor.validate().is_ok());
        assert_eq!(executor.name(), "fastrp_feature");
    }

    #[test]
    fn test_feature_executor_validate_empty_sources() {
        let descriptor = FeatureStepDescriptor::new(
            "feature1".into(),
            FeatureType::Scalar,
            vec![], // no source properties
        );

        let executor = FeatureStepExecutor::new(descriptor);
        let result = executor.validate();

        assert!(result.is_err());
        assert!(matches!(result, Err(ComputeError::InitFailed(_))));
    }
}
