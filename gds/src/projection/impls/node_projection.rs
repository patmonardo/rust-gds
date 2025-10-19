/// Node projection implementation.
///
/// Defines how nodes are projected into the graph, including which
/// properties to load and how to configure them.
use crate::projection::NodeLabel;
use crate::projection::PROJECT_ALL;
use crate::projection::{ElementProjection, PropertyMappings};
use std::collections::HashMap;
use std::sync::Arc;

/// Projection configuration for nodes.
///
/// Specifies a node label and its associated property mappings.
#[derive(Debug, Clone)]
pub struct NodeProjection {
    label: NodeLabel,
    properties: PropertyMappings,
}

impl NodeProjection {
    /// Key for the label field in configuration objects.
    pub const LABEL_KEY: &'static str = "label";

    /// Creates a new NodeProjection.
    ///
    /// # Arguments
    /// * `label` - The node label to project
    /// * `properties` - Property mappings for this node type
    pub fn new(label: NodeLabel, properties: PropertyMappings) -> Self {
        NodeProjection { label, properties }
    }

    /// Creates a NodeProjection with empty properties.
    pub fn of(label: NodeLabel) -> Self {
        NodeProjection {
            label,
            properties: PropertyMappings::empty(),
        }
    }

    /// Creates a NodeProjection that projects all nodes with all properties.
    pub fn all() -> Self {
        NodeProjection {
            label: NodeLabel::of(PROJECT_ALL),
            properties: PropertyMappings::empty(),
        }
    }

    /// Returns a builder for NodeProjection.
    pub fn builder() -> NodeProjectionBuilder {
        NodeProjectionBuilder::new()
    }

    /// Returns the node label for this projection.
    pub fn label(&self) -> &NodeLabel {
        &self.label
    }

    /// Returns the property mappings for this projection.
    pub fn properties(&self) -> &PropertyMappings {
        &self.properties
    }

    /// Checks if this projection targets all nodes.
    pub fn project_all(&self) -> bool {
        self.label.name() == PROJECT_ALL
    }

    /// Creates a new projection with additional property mappings.
    pub fn with_additional_property_mappings(&self, mappings: PropertyMappings) -> Self {
        NodeProjection {
            label: self.label.clone(),
            properties: self.properties.merge(&mappings),
        }
    }

    /// Converts to a configuration map.
    pub fn to_config(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            Self::LABEL_KEY.to_string(),
            serde_json::Value::String(self.label.name().to_string()),
        );

        if !self.properties.is_empty() {
            // Properties would be serialized here
            // For now, we'll leave it as a marker
            config.insert(
                "properties".to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        config
    }
}

impl ElementProjection for NodeProjection {
    fn properties(&self) -> &PropertyMappings {
        &self.properties
    }

    fn with_additional_property_mappings(
        &self,
        mappings: PropertyMappings,
    ) -> Box<dyn ElementProjection> {
        Box::new(NodeProjection {
            label: self.label.clone(),
            properties: self.properties.merge(&mappings),
        })
    }

    fn project_all(&self) -> bool {
        self.project_all()
    }

    fn include_aggregation(&self) -> bool {
        false // Nodes don't use aggregation
    }

    fn to_config(&self) -> HashMap<String, serde_json::Value> {
        self.to_config()
    }
}

/// Builder for NodeProjection.
#[derive(Debug, Default)]
pub struct NodeProjectionBuilder {
    label: Option<NodeLabel>,
    properties: Option<PropertyMappings>,
}

impl NodeProjectionBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        NodeProjectionBuilder {
            label: None,
            properties: None,
        }
    }

    /// Sets the node label.
    pub fn label(mut self, label: NodeLabel) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the property mappings.
    pub fn properties(mut self, properties: PropertyMappings) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Builds the NodeProjection.
    ///
    /// # Panics
    /// Panics if label is not set.
    pub fn build(self) -> NodeProjection {
        NodeProjection {
            label: self.label.expect("label is required"),
            properties: self.properties.unwrap_or_else(PropertyMappings::empty),
        }
    }
}

/// Type alias for a collection of node projections.
pub type NodeProjections = crate::projection::traits::Projections<NodeLabel, Arc<NodeProjection>>;

/// Type alias for a builder of node projections.
pub type NodeProjectionsBuilder =
    crate::projection::traits::ProjectionsBuilder<NodeLabel, Arc<NodeProjection>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_projection_creation() {
        let label = NodeLabel::of("Person");
        let projection = NodeProjection::of(label.clone());

        assert_eq!(projection.label().name(), "Person");
        assert!(projection.properties().is_empty());
        assert!(!projection.project_all());
    }

    #[test]
    fn test_project_all() {
        let projection = NodeProjection::all();
        assert!(projection.project_all());
        assert_eq!(projection.label().name(), PROJECT_ALL);
    }

    #[test]
    fn test_builder() {
        let label = NodeLabel::of("User");
        let projection = NodeProjection::builder().label(label.clone()).build();

        assert_eq!(projection.label().name(), "User");
        assert!(projection.properties().is_empty());
    }

    #[test]
    fn test_with_additional_properties() {
        let label = NodeLabel::of("Person");
        let projection = NodeProjection::of(label);
        let new_mappings = PropertyMappings::empty();

        let updated = projection.with_additional_property_mappings(new_mappings);
        assert_eq!(updated.label().name(), "Person");
    }

    #[test]
    fn test_to_config() {
        let label = NodeLabel::of("Person");
        let projection = NodeProjection::of(label);
        let config = projection.to_config();

        assert!(config.contains_key("label"));
        assert_eq!(
            config.get("label"),
            Some(&serde_json::Value::String("Person".to_string()))
        );
    }
}
