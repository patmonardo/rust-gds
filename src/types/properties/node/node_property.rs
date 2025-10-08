use super::node_property_values::NodePropertyValues;
use crate::types::properties::node::DefaultNodeProperty;
use crate::types::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: NodeProperty resolves to the concrete default implementation.
pub type NodeProperty = DefaultNodeProperty;

/// Factory helper mirroring Java's NodeProperty.of(...)
pub fn node_property_of(
    key: impl Into<String>,
    origin: PropertyState,
    values: Arc<dyn NodePropertyValues>,
) -> NodeProperty {
    // DefaultValue and PropertySchema creation is handled inside DefaultNodeProperty::with_state
    NodeProperty::with_state(key, origin, values)
}
