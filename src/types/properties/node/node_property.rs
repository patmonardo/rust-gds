use super::node_property_values::NodePropertyValues;
use crate::types::properties::property::Property;
use std::sync::Arc;

/// Represents a property of a node in the graph.
/// Specializes the Property trait for node-specific property values.
pub type NodeProperty = Property<Arc<dyn NodePropertyValues>>;
