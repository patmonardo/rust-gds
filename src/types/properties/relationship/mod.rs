// Relationship properties module
//
// Provides property storage for relationship properties.
// Relationship properties are values associated with edges in the graph.

pub mod impls;
pub mod relationship_properties;
pub mod relationship_property;
pub mod relationship_property_store;
pub mod relationship_property_values;
pub mod traits;

// Re-export commonly used items from traits
pub use traits::{
    EmptyPropertyCursor, ModifiableRelationshipCursor, PropertyCursor, PropertyValue,
    RelationshipCursor, RelationshipCursorBox, RelationshipIterator, RelationshipPredicate,
    RelationshipStream,
};

// Re-export commonly used items from impls
pub use impls::{
    DefaultModifiableRelationshipCursor, DefaultRelationshipCursor,
    DefaultRelationshipPropertyStore, DefaultRelationshipPropertyStoreBuilder,
    DefaultRelationshipPropertyValues,
};
