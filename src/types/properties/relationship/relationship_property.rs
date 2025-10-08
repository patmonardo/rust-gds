use super::relationship_property_values::RelationshipPropertyValues;
use crate::types::properties::relationship::DefaultRelationshipProperty;
use crate::types::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: RelationshipProperty resolves to the concrete default implementation.
pub type RelationshipProperty = DefaultRelationshipProperty;

/// Factory helper mirroring Java's RelationshipProperty.of(...)
pub fn relationship_property_of(
    key: impl Into<String>,
    state: PropertyState,
    values: Arc<dyn RelationshipPropertyValues>,
) -> RelationshipProperty {
    DefaultRelationshipProperty::with_state(key, state, values)
}
