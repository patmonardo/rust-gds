// Property system module for Rust GDS
//
// This module provides the property storage and value access layer for the graph.
// It mirrors the TypeScript API from api/properties/.
//
// Key concepts:
// - PropertyValues: Base trait for accessing typed property values
// - Property: Combines PropertyValues with a PropertySchema
// - PropertyStore: Collection of properties indexed by key
//
// Specialized implementations:
// - Node properties: Values indexed by node ID
// - Graph properties: Scalar values for graph-level properties
// - Relationship properties: Values associated with relationships

pub mod property;
pub mod property_store;
pub mod property_values;

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export core traits and types
pub use property::{Property, PropertyTrait};
pub use property_store::{PropertyStore, PropertyStoreError};
pub use property_values::{PropertyValues, PropertyValuesError};

// Re-export specialized types
pub use graph::{
    GraphProperty, GraphPropertyStore, GraphPropertyStoreBuilder, GraphPropertyValues,
};
pub use node::{
    EmptyNodePropertyContainer, NodeProperty, NodePropertyContainer, NodePropertyContainerExt,
    NodePropertyStore, NodePropertyStoreBuilder, NodePropertyValues,
};
pub use relationship::{
    all_relationships, for_each_relationship, for_each_relationship_with_property,
    no_relationships, not_relationships, AndThenRelationshipConsumer,
    AndThenRelationshipWithPropertyConsumer, ConstantRelationshipProperties, EmptyPropertyCursor,
    EmptyRelationshipProperties, ModifiableRelationshipCursor, PropertyCursor, PropertyValue,
    RelationshipConsumer, RelationshipConsumerExt, RelationshipCursor, RelationshipCursorBox,
    RelationshipIterator, RelationshipIteratorExt, RelationshipPredicate, RelationshipProperties,
    RelationshipPropertiesExt, RelationshipProperty, RelationshipPropertyStore,
    RelationshipPropertyStoreBuilder, RelationshipPropertyValues, RelationshipStream,
    RelationshipWithPropertyConsumer, RelationshipWithPropertyConsumerExt,
};
