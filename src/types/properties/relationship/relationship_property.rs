use super::relationship_property_values::RelationshipPropertyValues;
use crate::types::properties::property::Property;
use std::sync::Arc;

/// Represents a property associated with relationships in the graph.
/// Specializes the Property trait for relationship-specific property values.
pub type RelationshipProperty = Property<Arc<dyn RelationshipPropertyValues>>;
