use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::graph::DefaultGraphProperty;
use crate::types::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: GraphProperty resolves to the concrete default implementation.
pub type GraphProperty = DefaultGraphProperty;

/// Factory helper mirroring Java's GraphProperty.of(...)
pub fn graph_property_of(
    key: impl Into<String>,
    state: PropertyState,
    values: Arc<dyn GraphPropertyValues>,
) -> GraphProperty {
    GraphProperty::with_state(key, state, values)
}
