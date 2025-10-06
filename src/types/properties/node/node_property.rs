use super::node_property_values::NodePropertyValues;
use crate::types::properties::property::DefaultProperty;
use crate::types::property::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: NodeProperty is the canonical DefaultProperty instance.
pub type NodeProperty = DefaultProperty;

/// Factory helper mirroring Java's NodeProperty.of(...)
pub fn node_property_of(
    key: impl Into<String>,
    origin: PropertyState,
    values: Arc<dyn NodePropertyValues>,
) -> NodeProperty {
    // DefaultValue and PropertySchema creation is handled inside DefaultProperty::of
    DefaultProperty::of(key.into(), origin, values)
}
