use super::relationship_property_values::RelationshipPropertyValues;
use crate::types::properties::property::DefaultProperty;
use crate::types::property::PropertyState;
use std::sync::Arc;

/// Alias for ergonomics: RelationshipProperty is the canonical DefaultProperty instance.
pub type RelationshipProperty = DefaultProperty;

/// Factory helper mirroring Java's RelationshipProperty.of(...)
pub fn relationship_property_of(
    key: impl Into<String>,
    state: PropertyState,
    values: Arc<dyn RelationshipPropertyValues>,
) -> RelationshipProperty {
    DefaultProperty::of(key, state, values)
}
