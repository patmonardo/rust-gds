// Relationship properties module
//
// Provides property storage for relationship properties.
// Relationship properties are values associated with edges in the graph.

pub mod property_cursor;
pub mod relationship_properties;
pub mod relationship_property;
pub mod relationship_property_store;
pub mod relationship_property_values;
pub mod traits;

// Re-export public API
pub use property_cursor::{EmptyPropertyCursor, PropertyCursor, PropertyValue};
pub use relationship_properties::{
    ConstantRelationshipProperties, EmptyRelationshipProperties, RelationshipProperties,
    RelationshipPropertiesExt,
};
pub use relationship_property::RelationshipProperty;
pub use relationship_property_store::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
pub use relationship_property_values::{
    DefaultRelationshipPropertyValues, RelationshipPropertyValues,
};
pub use traits::{
    all_relationships, for_each_relationship, for_each_relationship_with_property,
    no_relationships, not_relationships, AndThenRelationshipConsumer,
    AndThenRelationshipWithPropertyConsumer, ModifiableRelationshipCursor, RelationshipConsumer,
    RelationshipConsumerExt, RelationshipCursor, RelationshipCursorBox, RelationshipIterator,
    RelationshipIteratorExt, RelationshipPredicate, RelationshipStream,
    RelationshipWithPropertyConsumer, RelationshipWithPropertyConsumerExt,
};
