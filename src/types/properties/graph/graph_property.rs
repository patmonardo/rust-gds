use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::property::DefaultProperty;
use crate::types::property::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: GraphProperty is the canonical DefaultProperty instance.
pub type GraphProperty = DefaultProperty;

/// Factory helper mirroring Java's GraphProperty.of(...)
pub fn graph_property_of(
    key: impl Into<String>,
    state: PropertyState,
    values: Arc<dyn GraphPropertyValues>,
) -> GraphProperty {
    DefaultProperty::of(key, state, values)
}
