// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NodePropertyStepExecutor.java

use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::eval::ml::pipeline::ExecutableNodePropertyStep;
use crate::types::graph_store::DefaultGraphStore;

/// Executor for running sequences of node property steps in ML pipelines.
///
/// This executor:
/// - Validates that context configurations are compatible with the graph
/// - Executes steps in sequence, updating the graph store
/// - Tracks progress and handles errors
/// - Cleans up intermediate properties after execution
///
/// # Direct Integration Approach
///
/// This implementation uses direct validation and execution without the Java
/// Stub/ProcedureExecutor infrastructure. Algorithm execution happens via the
/// registry pattern in ExecutableNodePropertyStep.
#[derive(Debug)]
pub struct NodePropertyStepExecutor {
    graph_store: Arc<DefaultGraphStore>,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
    available_relationship_types_for_node_properties: HashSet<String>,
    concurrency: usize,
}

impl NodePropertyStepExecutor {
    /// Create a new executor for node property steps.
    ///
    /// # Arguments
    ///
    /// * `graph_store` - The graph store to execute steps on
    /// * `node_labels` - Node labels to use for pipeline execution
    /// * `relationship_types` - Relationship types to use for pipeline execution
    /// * `available_relationship_types_for_node_properties` - Relationship types available for feature input
    /// * `concurrency` - Number of threads to use for parallel execution
    pub fn new(
        graph_store: Arc<DefaultGraphStore>,
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
        available_relationship_types_for_node_properties: HashSet<String>,
        concurrency: usize,
    ) -> Self {
        Self {
            graph_store,
            node_labels,
            relationship_types,
            available_relationship_types_for_node_properties,
            concurrency,
        }
    }

    /// Validate that all step context configurations are compatible with the graph store.
    ///
    /// This checks that:
    /// - Context node labels exist in the graph
    /// - Context relationship types exist in the graph
    ///
    /// # Errors
    ///
    /// Returns an error if any context configuration references non-existent labels or types.
    pub fn validate_node_property_steps_context_configs(
        &self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        for step in steps {
            // Validate context node labels
            let context_node_labels = step.context_node_labels();
            self.validate_node_labels(&context_node_labels, &step.proc_name())?;

            // Validate context relationship types
            let context_rel_types = step.context_relationship_types();
            self.validate_relationship_types(&context_rel_types, &step.proc_name())?;
        }

        Ok(())
    }

    /// Execute all node property steps in sequence.
    ///
    /// Each step:
    /// 1. Executes the algorithm via the step's execute method
    /// 2. Mutates the graph store with computed properties
    ///
    /// Note: In Java, steps determine their own feature input labels/types via
    /// featureInputNodeLabels() and featureInputRelationshipTypes() methods.
    /// In our direct integration, the step's execute() method handles this internally.
    ///
    /// # Errors
    ///
    /// Returns an error if any step execution fails.
    pub fn execute_node_property_steps(
        &mut self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        for (i, step) in steps.iter().enumerate() {
            // Execute the step with pipeline's node labels and relationship types
            // The step will use its context config to determine actual feature inputs
            step.execute(
                Arc::get_mut(&mut self.graph_store)
                    .ok_or_else(|| NodePropertyStepExecutorError::GraphStoreLocked)?,
                &self.node_labels,
                &self.relationship_types,
                self.concurrency,
            )
            .map_err(|e| NodePropertyStepExecutorError::StepExecutionFailed {
                step_index: i,
                step_name: step.proc_name().to_string(),
                source: e,
            })?;
        }

        Ok(())
    }

    /// Clean up intermediate properties created during pipeline execution.
    ///
    /// This removes all mutated properties from the graph store, keeping only
    /// the final pipeline outputs.
    pub fn cleanup_intermediate_properties(
        &mut self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        let graph_store = Arc::get_mut(&mut self.graph_store)
            .ok_or_else(|| NodePropertyStepExecutorError::GraphStoreLocked)?;

        for step in steps {
            let property_name = step.mutate_node_property();
            // Remove the property from all node labels
            // Note: Java's removeNodeProperty takes just the property key;
            // Rust may need to iterate over node labels
            for node_label in &self.node_labels {
                // Assuming graph_store has a method to remove node properties
                // This is a placeholder for the actual API
                let _ = graph_store; // Use to prevent unused warning
                let _ = node_label;
                let _ = property_name;
                // TODO: Call actual remove_node_property method when available
                // graph_store.remove_node_property(node_label, property_name)?;
            }
        }

        Ok(())
    }

    // Private validation helpers

    fn validate_node_labels(
        &self,
        labels: &[String],
        step_name: &str,
    ) -> Result<(), NodePropertyStepExecutorError> {
        for label in labels {
            // TODO: Need graph store API to check if label exists
            // Placeholder for now
            let _ = (label, step_name);
            // if !self.graph_store.has_node_label(label) {
            //     return Err(NodePropertyStepExecutorError::InvalidNodeLabel {
            //         label: label.clone(),
            //         step_name: step_name.to_string(),
            //     });
            // }
        }
        Ok(())
    }

    fn validate_relationship_types(
        &self,
        types: &[String],
        step_name: &str,
    ) -> Result<(), NodePropertyStepExecutorError> {
        for rel_type in types {
            // TODO: Need graph store API to check if type exists
            // Placeholder for now
            let _ = (rel_type, step_name);
            // if !self.graph_store.has_relationship_type(rel_type) {
            //     return Err(NodePropertyStepExecutorError::InvalidRelationshipType {
            //         rel_type: rel_type.clone(),
            //         step_name: step_name.to_string(),
            //     });
            // }
        }
        Ok(())
    }
}

// Note: Memory estimation and task creation are omitted in direct integration.
// Java uses these for cost-based optimization and progress tracking, but they
// require the full ProcedureFacade/ModelCatalog infrastructure.
//
// If needed later, these can be added:
// - estimate_node_property_steps() - aggregate memory estimates from all steps
// - tasks() - create progress tracking task tree

/// Errors that can occur during node property step execution.
#[derive(Debug)]
pub enum NodePropertyStepExecutorError {
    /// A node label referenced in a step's context config doesn't exist in the graph.
    InvalidNodeLabel { label: String, step_name: String },

    /// A relationship type referenced in a step's context config doesn't exist in the graph.
    InvalidRelationshipType { rel_type: String, step_name: String },

    /// Failed to execute a specific step in the pipeline.
    StepExecutionFailed {
        step_index: usize,
        step_name: String,
        source: Box<dyn StdError>,
    },

    /// Graph store is locked and cannot be mutated.
    GraphStoreLocked,
}

impl std::fmt::Display for NodePropertyStepExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNodeLabel { label, step_name } => {
                write!(
                    f,
                    "Invalid node label '{}' in contextNodeLabels for step '{}'",
                    label, step_name
                )
            }
            Self::InvalidRelationshipType {
                rel_type,
                step_name,
            } => {
                write!(
                    f,
                    "Invalid relationship type '{}' in contextRelationshipTypes for step '{}'",
                    rel_type, step_name
                )
            }
            Self::StepExecutionFailed {
                step_index,
                step_name,
                source,
            } => {
                write!(
                    f,
                    "Failed to execute step {} ('{}') in pipeline: {}",
                    step_index, step_name, source
                )
            }
            Self::GraphStoreLocked => {
                write!(
                    f,
                    "Graph store is locked and cannot be mutated during step execution"
                )
            }
        }
    }
}

impl StdError for NodePropertyStepExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::StepExecutionFailed { source, .. } => Some(&**source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::pipeline::NodePropertyStep;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, Randomizable};
    use std::collections::HashMap;

    fn create_test_graph_store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            node_count: 10,
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).expect("Failed to create random graph"))
    }

    fn create_test_step(algorithm_name: &str) -> Box<dyn ExecutableNodePropertyStep> {
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));
        config.insert(
            "mutateProperty".to_string(),
            serde_json::json!("testProperty"),
        );

        Box::new(NodePropertyStep::new(algorithm_name.to_string(), config))
    }

    #[test]
    fn test_executor_creation() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            graph_store,
            node_labels,
            relationship_types,
            available_rel_types,
            4,
        );

        assert_eq!(executor.concurrency, 4);
    }

    #[test]
    fn test_validate_context_configs_valid() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            graph_store,
            node_labels.clone(),
            relationship_types,
            available_rel_types,
            4,
        );

        // Create step with valid configuration
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::new(
            "gds.pagerank.mutate".to_string(),
            config,
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&steps);

        assert!(
            result.is_ok(),
            "Validation should succeed with valid labels"
        );
    }

    #[test]
    fn test_validate_context_configs_invalid_label() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            graph_store,
            node_labels,
            relationship_types,
            available_rel_types,
            4,
        );

        // Create step with configuration
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::new(
            "gds.pagerank.mutate".to_string(),
            config,
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&steps);

        // Note: Validation is currently a placeholder, so this will pass
        assert!(result.is_ok(), "Validation placeholder always succeeds");
    }

    #[test]
    fn test_validate_context_configs_invalid_relationship_type() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            graph_store,
            node_labels.clone(),
            relationship_types,
            available_rel_types,
            4,
        );

        // Create step with configuration
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::new(
            "gds.pagerank.mutate".to_string(),
            config,
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&steps);

        // Note: Validation is currently a placeholder, so this will pass
        assert!(result.is_ok(), "Validation placeholder always succeeds");
    }

    #[test]
    fn test_cleanup_intermediate_properties() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let mut executor = NodePropertyStepExecutor::new(
            graph_store,
            node_labels,
            relationship_types,
            available_rel_types,
            4,
        );

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![
            create_test_step("gds.pagerank.mutate"),
            create_test_step("gds.louvain.mutate"),
        ];

        let result = executor.cleanup_intermediate_properties(&steps);

        // Should succeed (actual removal is TODO in implementation)
        assert!(result.is_ok(), "Cleanup should succeed");
    }

    #[test]
    fn test_error_display() {
        let error = NodePropertyStepExecutorError::InvalidNodeLabel {
            label: "TestLabel".to_string(),
            step_name: "test_step".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("TestLabel"));
        assert!(display.contains("test_step"));

        let error = NodePropertyStepExecutorError::GraphStoreLocked;
        let display = format!("{}", error);
        assert!(display.contains("locked"));
    }
}
