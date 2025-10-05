use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::property::Property;
use std::sync::Arc;

/// Represents a property of the graph.
/// Specializes the Property trait for graph-specific property values.
pub type GraphProperty = Property<Arc<dyn GraphPropertyValues>>;
