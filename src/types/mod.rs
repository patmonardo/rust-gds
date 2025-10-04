pub mod concurrency;
pub mod graph;
pub mod graph_store;
pub mod prelude;
pub mod properties;
pub mod property;
pub mod property_store;
pub mod random;
pub mod schema;

// Re-export the core types from the property module that are part of the
// public API surface.
pub use property::{
    Property, PropertySchema as PropertySchemaTrait, PropertyState, SimpleProperty, ValueType,
};
pub use property_store::{graph::GraphPropertyStore, node::NodePropertyStore, PropertyStore};

// Re-export schema types
pub use schema::{
    Aggregation, DefaultValue, Direction, GraphSchema, MutableGraphSchema, MutableNodeSchema,
    MutableNodeSchemaEntry, MutableRelationshipSchema, MutableRelationshipSchemaEntry, NodeLabel,
    NodeSchema, NodeSchemaEntry, PropertySchema, RelationshipPropertySchema, RelationshipSchema,
    RelationshipSchemaEntry, RelationshipType, SchemaError, SchemaResult,
};

// Re-export properties types
pub use properties::{
    EmptyNodePropertyContainer,
    // Graph properties
    GraphProperty,
    GraphPropertyStore as GraphPropertyStoreNew,
    GraphPropertyStoreBuilder,
    GraphPropertyValues,
    // Node properties
    NodeProperty,
    NodePropertyContainer,
    NodePropertyContainerExt,
    NodePropertyStore as NodePropertyStoreNew,
    NodePropertyStoreBuilder,
    NodePropertyValues,
    PropertyStore as PropertyStoreTrait,
    PropertyTrait,
    // Core traits
    PropertyValues,
    // Relationship properties
    RelationshipProperty,
    RelationshipPropertyStore as RelationshipPropertyStoreNew,
    RelationshipPropertyStoreBuilder,
    RelationshipPropertyValues,
};

pub type PropertyId = u32;
pub type PropertyData = String;

// Re-export IdMap traits and helpers so downstream crates can build on the high-level API.
pub use concurrency::Concurrency;
pub use graph::adj_list::{
    AdjacencyCursor, AdjacencyCursorExt, AdjacencyList, AdjacencyListExt, EdgeWeight,
    WeightedAdjacencyCursor, WeightedAdjacencyCursorExt, NOT_FOUND_TARGET,
};
pub use graph::id_map::{
    BatchNodeIterable, EmptyPartialIdMap, FilteredIdMap, IdMap, MappedNodeId, NodeConsumer,
    NodeIdBatch, NodeIdBatchIter, NodeIterator, NodeIteratorExt, NodeLabelConsumer, OriginalNodeId,
    PartialIdMap, SimpleIdMap, NOT_FOUND, NO_TYPE, START_NODE_ID,
};

// Re-export graph traits and helpers
pub use graph::{
    DefaultGraph, Degrees, Graph, GraphCharacteristics, GraphCharacteristicsBuilder, GraphExt,
    GraphResult,
};
pub use graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DefaultGraphStore, DeletionResult, GraphName,
    GraphStore, GraphStoreAdapter, GraphStoreError, GraphStoreResult,
};
pub use random::{
    RandomGraphConfig, RandomGraphError, RandomGraphResult, RandomRelationshipConfig, Randomizable,
};

/// Convenience helper that constructs a randomized [`DefaultGraphStore`] using the
/// crate's existing generator. Returns the same [`RandomGraphResult`] type exposed by the
/// `random` module to preserve detailed error information from random generation.
pub fn random_graph_store(
    cfg: &random::RandomGraphConfig,
) -> random::RandomGraphResult<graph_store::DefaultGraphStore> {
    graph_store::DefaultGraphStore::random(cfg)
}
