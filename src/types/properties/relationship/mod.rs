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

// Re-export public API
pub use impls::{
    DefaultModifiableRelationshipCursor, DefaultRelationshipCursor, DefaultRelationshipProperty,
    DefaultRelationshipPropertyStore, DefaultRelationshipPropertyValues,
};
pub use relationship_properties::{
    ConstantRelationshipProperties, EmptyRelationshipProperties, RelationshipProperties,
    RelationshipPropertiesExt,
};
pub use relationship_property::RelationshipProperty;
pub use relationship_property_store::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
pub use relationship_property_values::RelationshipPropertyValues;
pub use traits::{
    all_relationships, no_relationships, not_relationships, EmptyPropertyCursor,
    ModifiableRelationshipCursor, PropertyCursor, PropertyValue, RelationshipCursor,
    RelationshipCursorBox, RelationshipIterator, RelationshipPredicate, RelationshipStream,
};
