//! Pipeline trait
//!
//! Direct 1:1 translation of Java `org.neo4j.gds.ml.pipeline.Pipeline<FEATURE_STEP>`.

use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::fmt;

use super::executable_node_property_step::ExecutableNodePropertyStep;
use super::feature_step::FeatureStep;

/// Base Pipeline trait.
///
/// **Java Source**: `org.neo4j.gds.ml.pipeline.Pipeline<FEATURE_STEP>`
///
/// Direct translation of Java `Pipeline<FEATURE_STEP extends FeatureStep>` interface.
///
/// **Design Philosophy**:
/// Pipeline is a 2-stage process:
/// 1. **Node Property Steps** - Execute graph algorithms to compute node properties
/// 2. **Feature Steps** - Extract features from node properties for ML
///
/// **Type Parameter**:
/// `FeatureStep` - Associated type determining whether this is a Node or Link pipeline
///
/// ```java
/// public interface Pipeline<FEATURE_STEP extends FeatureStep> extends ToMapConvertible {
///     List<ExecutableNodePropertyStep> nodePropertySteps();
///     List<FEATURE_STEP> featureSteps();
///     default List<String> featureProperties() { ... }
///     default void validateBeforeExecution(GraphStore, Collection<NodeLabel>) { ... }
///     void specificValidateBeforeExecution(GraphStore);
///     default void validateFeatureProperties(GraphStore, Collection<NodeLabel>) { ... }
///     default Set<String> featurePropertiesMissingFromGraph(...) { ... }
/// }
/// ```
pub trait Pipeline {
    /// Associated feature step type.
    ///
    /// **Java**: Generic parameter `FEATURE_STEP extends FeatureStep`
    type FeatureStep: FeatureStep;

    /// Node property computation steps (graph algorithms).
    ///
    /// **Java**: `List<ExecutableNodePropertyStep> nodePropertySteps()`
    fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>];

    /// Feature extraction steps.
    ///
    /// **Java**: `List<FEATURE_STEP> featureSteps()`
    fn feature_steps(&self) -> &[Self::FeatureStep];

    /// Feature properties required for ML (derived from feature steps).
    ///
    /// **Java**:
    /// ```java
    /// default List<String> featureProperties() {
    ///     return featureSteps()
    ///         .stream()
    ///         .flatMap(step -> step.inputNodeProperties().stream())
    ///         .collect(Collectors.toList());
    /// }
    /// ```
    fn feature_properties(&self) -> Vec<String> {
        self.feature_steps()
            .iter()
            .flat_map(|step| step.input_node_properties())
            .map(String::from)
            .collect()
    }

    /// Validate pipeline before execution.
    ///
    /// **Java**:
    /// ```java
    /// default void validateBeforeExecution(GraphStore graphStore, Collection<NodeLabel> nodeLabels) {
    ///     Set<String> invalidProperties = featurePropertiesMissingFromGraph(graphStore, nodeLabels);
    ///     
    ///     // Remove properties that will be created by node property steps
    ///     nodePropertySteps().stream()
    ///         .flatMap(step -> Stream.ofNullable((String) step.config().get(MUTATE_PROPERTY_KEY)))
    ///         .forEach(invalidProperties::remove);
    ///         
    ///     if (!invalidProperties.isEmpty()) {
    ///         throw Pipeline.missingNodePropertiesFromFeatureSteps(invalidProperties);
    ///     }
    ///     
    ///     specificValidateBeforeExecution(graphStore);
    /// }
    /// ```
    fn validate_before_execution(
        &self,
        graph_store: &crate::types::graph_store::DefaultGraphStore,
        node_labels: &[String],
    ) -> Result<(), PipelineValidationError> {
        let mut invalid_properties =
            self.feature_properties_missing_from_graph(graph_store, node_labels);

        // Remove properties that will be created by node property steps
        for step in self.node_property_steps() {
            if let Some(mutate_key) = step.config().get("mutateProperty") {
                if let Some(prop_name) = mutate_key.as_str() {
                    invalid_properties.remove(prop_name);
                }
            }
        }

        if !invalid_properties.is_empty() {
            return Err(PipelineValidationError::missing_node_properties(
                invalid_properties,
            ));
        }

        self.specific_validate_before_execution(graph_store)?;

        Ok(())
    }

    /// Pipeline-specific validation (overridable).
    ///
    /// **Java**: `void specificValidateBeforeExecution(GraphStore graphStore)`
    fn specific_validate_before_execution(
        &self,
        graph_store: &crate::types::graph_store::DefaultGraphStore,
    ) -> Result<(), PipelineValidationError>;

    /// Validate that feature properties exist after node property steps.
    ///
    /// **Called after** node property steps have executed.
    ///
    /// **Java**:
    /// ```java
    /// default void validateFeatureProperties(GraphStore graphStore, Collection<NodeLabel> nodeLabels) {
    ///     Set<String> invalidProperties = featurePropertiesMissingFromGraph(graphStore, nodeLabels);
    ///     
    ///     if (!invalidProperties.isEmpty()) {
    ///         throw missingNodePropertiesFromFeatureSteps(invalidProperties);
    ///     }
    /// }
    /// ```
    fn validate_feature_properties(
        &self,
        graph_store: &crate::types::graph_store::DefaultGraphStore,
        node_labels: &[String],
    ) -> Result<(), PipelineValidationError> {
        let invalid_properties =
            self.feature_properties_missing_from_graph(graph_store, node_labels);

        if !invalid_properties.is_empty() {
            return Err(PipelineValidationError::missing_node_properties(
                invalid_properties,
            ));
        }

        Ok(())
    }

    /// Find feature properties that are missing from the graph.
    ///
    /// **Java**:
    /// ```java
    /// default Set<String> featurePropertiesMissingFromGraph(GraphStore graphStore, Collection<NodeLabel> nodeLabels) {
    ///     var graphProperties = graphStore.nodePropertyKeys(nodeLabels);
    ///     
    ///     return featureSteps()
    ///         .stream()
    ///         .flatMap(step -> step.inputNodeProperties().stream())
    ///         .filter(property -> !graphProperties.contains(property))
    ///         .collect(Collectors.toSet());
    /// }
    /// ```
    fn feature_properties_missing_from_graph(
        &self,
        _graph_store: &crate::types::graph_store::DefaultGraphStore,
        _node_labels: &[String],
    ) -> HashSet<String> {
        // TODO: Implement node_property_keys on DefaultGraphStore
        // For now, return empty set (all properties valid)
        HashSet::new()
    }

    /// Convert pipeline to map for serialization (ToMapConvertible).
    ///
    /// **Java**: Inherited from `ToMapConvertible` interface
    fn to_map(&self) -> HashMap<String, serde_json::Value>;
}

/// Pipeline validation error.
///
/// **Java**: Translates Java exceptions thrown from Pipeline validation methods.
///
/// **Java Source**: Various `IllegalArgumentException` from `Pipeline.java`:
/// ```java
/// static IllegalArgumentException missingNodePropertiesFromFeatureSteps(Set<String> invalidProperties) {
///     return new IllegalArgumentException(formatWithLocale(
///         "Node properties %s defined in the feature steps do not exist in the graph or part of the pipeline",
///         invalidProperties.stream().sorted().collect(Collectors.toList())
///     ));
/// }
/// ```
#[derive(Debug, Clone)]
pub enum PipelineValidationError {
    /// Missing node properties from feature steps.
    ///
    /// **Java**: `Pipeline.missingNodePropertiesFromFeatureSteps(Set<String>)`
    MissingNodeProperties { properties: Vec<String> },

    /// Graph structure invalid.
    GraphStructureInvalid { message: String },

    /// Other validation error.
    Other { message: String },
}

impl PipelineValidationError {
    pub fn missing_node_properties(properties: HashSet<String>) -> Self {
        let mut sorted: Vec<_> = properties.into_iter().collect();
        sorted.sort();
        Self::MissingNodeProperties { properties: sorted }
    }

    pub fn graph_structure(message: impl Into<String>) -> Self {
        Self::GraphStructureInvalid {
            message: message.into(),
        }
    }
}

impl fmt::Display for PipelineValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNodeProperties { properties } => {
                write!(
                    f,
                    "Node properties {:?} defined in the feature steps do not exist in the graph or part of the pipeline",
                    properties
                )
            }
            Self::GraphStructureInvalid { message } => {
                write!(f, "Graph structure invalid: {}", message)
            }
            Self::Other { message } => {
                write!(f, "Pipeline validation error: {}", message)
            }
        }
    }
}

impl StdError for PipelineValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let mut props = HashSet::new();
        props.insert("embedding".to_string());
        props.insert("pagerank".to_string());

        let error = PipelineValidationError::missing_node_properties(props);
        let msg = error.to_string();

        assert!(msg.contains("embedding"));
        assert!(msg.contains("pagerank"));
    }
}
