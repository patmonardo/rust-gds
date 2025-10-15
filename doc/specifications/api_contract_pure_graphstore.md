# Pure GraphStore API Contract

**Version**: 0.1.0  
**Status**: Stabilizing  
**Date**: October 4, 2025

## Overview

This document defines the **stable public API** for the Pure GraphStore system - the foundational graph data management layer in rust-gds. The Pure system provides clean, trait-driven abstractions for graph storage without performance optimizations or professional features (those belong in CoreGraphStore).

## Design Principles

1. **Trait-Driven**: All major abstractions are traits, allowing multiple implementations
2. **Immutable Views**: Graphs are read-only views over GraphStore data
3. **Type Safety**: Strong typing prevents misuse; compile-time guarantees where possible
4. **Zero-Copy Sharing**: Use `Arc<T>` for efficient sharing of immutable data
5. **Builder Pattern**: Immutable stores use builders for modifications
6. **Fallible Operations**: Operations that can fail return `Result<T, E>`

---

## 📦 Module Structure

```
types/
├── graph/                  # Graph views and topology
│   ├── Graph trait        # Core graph interface
│   ├── DefaultGraph       # Default implementation
│   ├── IdMap traits       # Node ID mapping
│   └── RelationshipTopology
│
├── graph_store/           # Storage orchestration
│   ├── GraphStore trait   # Central storage interface
│   ├── DefaultGraphStore  # Default implementation
│   └── Metadata types     # DatabaseInfo, GraphName, etc.
│
└── properties/            # Property system
    ├── graph/            # Graph-level properties
    ├── node/             # Node properties
    └── relationship/     # Relationship properties
```

---

## 🎯 Core Traits (Stable API)

### 1. `GraphStore` Trait

**Purpose**: Central orchestrator for all graph data management.

**Stability**: 🟢 **STABLE** - This is the main entry point

**Key Responsibilities**:

- Schema and metadata management
- Node/relationship/property CRUD operations
- Creation of filtered Graph views
- Lifecycle tracking (creation/modification times)

**Essential Methods**:

```rust
// Metadata
fn database_info(&self) -> &DatabaseInfo;
fn schema(&self) -> &GraphSchema;
fn creation_time(&self) -> DateTime<Utc>;
fn modification_time(&self) -> DateTime<Utc>;
fn capabilities(&self) -> &Capabilities;

// Node Operations
fn node_count(&self) -> usize;
fn node_labels(&self) -> HashSet<NodeLabel>;
fn has_node_label(&self, label: &NodeLabel) -> bool;
fn add_node_label(&mut self, node_label: NodeLabel) -> GraphStoreResult<()>;

// Relationship Operations
fn relationship_count(&self) -> usize;
fn relationship_types(&self) -> HashSet<RelationshipType>;
fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool;

// Property Operations (Graph Level)
fn graph_property_keys(&self) -> HashSet<String>;
fn has_graph_property(&self, property_key: &str) -> bool;
fn graph_property_values(&self, property_key: &str)
    -> GraphStoreResult<Arc<dyn GraphPropertyValues>>;
fn add_graph_property(&mut self, key: impl Into<String>,
    values: Arc<dyn GraphPropertyValues>) -> GraphStoreResult<()>;
fn remove_graph_property(&mut self, property_key: &str) -> GraphStoreResult<()>;

// Property Operations (Node Level)
fn node_property_keys(&self) -> HashSet<String>;
fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String>;
fn has_node_property(&self, property_key: &str) -> bool;
fn node_property_values(&self, property_key: &str)
    -> GraphStoreResult<Arc<dyn NodePropertyValues>>;
fn add_node_property(&mut self, labels: HashSet<NodeLabel>,
    key: impl Into<String>, values: Arc<dyn NodePropertyValues>)
    -> GraphStoreResult<()>;
fn remove_node_property(&mut self, property_key: &str) -> GraphStoreResult<()>;

// Property Operations (Relationship Level)
fn relationship_property_keys(&self) -> HashSet<String>;
fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType)
    -> HashSet<String>;
fn has_relationship_property(&self, rel_type: &RelationshipType,
    property_key: &str) -> bool;
fn relationship_property_values(&self, relationship_type: &RelationshipType,
    property_key: &str) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>>;
fn add_relationship_property(&mut self, relationship_type: RelationshipType,
    key: impl Into<String>, values: Arc<dyn RelationshipPropertyValues>)
    -> GraphStoreResult<()>;
fn remove_relationship_property(&mut self, relationship_type: &RelationshipType,
    property_key: &str) -> GraphStoreResult<()>;

// Deletion Operations
fn delete_relationships(&mut self, relationship_type: &RelationshipType)
    -> GraphStoreResult<DeletionResult>;
```

---

### 2. `Graph` Trait

**Purpose**: Read-only view for querying graph structure and properties.

**Stability**: 🟢 **STABLE** - Core query interface

**Key Characteristics**:

- Immutable - created from GraphStore, never modified directly
- Combines multiple trait bounds: `IdMap + NodePropertyContainer + Degrees + RelationshipIterator + RelationshipProperties`
- Thread-safe - can be shared across threads via `Arc<dyn Graph>`

**Essential Methods**:

```rust
// Schema & Characteristics
fn schema(&self) -> &GraphSchema;
fn characteristics(&self) -> GraphCharacteristics;

// Basic Queries
fn is_empty(&self) -> bool;
fn relationship_count(&self) -> usize;
fn is_multi_graph(&self) -> bool;
fn has_relationship_property(&self) -> bool;

// Filtering & Views
fn relationship_type_filtered_graph(&self,
    relationship_types: &HashSet<RelationshipType>)
    -> GraphResult<Arc<dyn Graph>>;

// Traversal
fn nth_target(&self, source_id: MappedNodeId, offset: usize)
    -> Option<MappedNodeId>;

// Concurrency
fn concurrent_copy(&self) -> Arc<dyn Graph>;
fn as_node_filtered_graph(&self) -> Option<Arc<dyn FilteredIdMap>>;
```

**Inherited from `IdMap`**:

```rust
fn node_count(&self) -> usize;
fn to_mapped_node_id(&self, original: OriginalNodeId) -> Option<MappedNodeId>;
fn to_original_node_id(&self, mapped: MappedNodeId) -> Option<OriginalNodeId>;
fn node_labels(&self, mapped_node_id: MappedNodeId) -> HashSet<NodeLabel>;
fn has_label(&self, mapped_node_id: MappedNodeId, label: &NodeLabel) -> bool;
// ... and more
```

**Inherited from `Degrees`**:

```rust
fn degree(&self, node_id: MappedNodeId) -> usize;
fn degree_inverse(&self, node_id: MappedNodeId) -> Option<usize>;
fn degree_without_parallel_relationships(&self, node_id: MappedNodeId) -> usize;
```

**Inherited from `RelationshipIterator`**:

```rust
fn stream_relationships<'a>(&'a self, node_id: MappedNodeId,
    fallback_value: PropertyValue) -> RelationshipStream<'a>;
fn stream_inverse_relationships<'a>(&'a self, node_id: MappedNodeId,
    fallback_value: PropertyValue) -> RelationshipStream<'a>;
```

**Inherited from `RelationshipProperties`**:

```rust
fn default_property_value(&self) -> PropertyValue;
fn relationship_property(&self, source_id: MappedNodeId,
    target_id: MappedNodeId, fallback_value: PropertyValue) -> PropertyValue;
```

**Inherited from `NodePropertyContainer`**:

```rust
fn node_properties(&self, property_key: &str)
    -> Option<Arc<dyn NodePropertyValues>>;
fn available_node_properties(&self) -> HashSet<String>;
```

---

### 3. Property System Traits

#### 3.1 `PropertyValues` (Base Trait)

**Purpose**: Access to raw property value storage.

**Stability**: 🟢 **STABLE**

```rust
pub trait PropertyValues: Send + Sync {
    fn value_type(&self) -> ValueType;
    fn node_count(&self) -> usize;
    fn default_value(&self) -> f64;  // Note: will evolve to support all types
}
```

#### 3.2 `GraphPropertyValues`

**Purpose**: Properties attached to the entire graph (metadata).

**Stability**: 🟢 **STABLE**

```rust
pub trait GraphPropertyValues: PropertyValues {
    fn value(&self) -> PropertyValue;
}
```

#### 3.3 `NodePropertyValues`

**Purpose**: Properties indexed by node ID.

**Stability**: 🟢 **STABLE**

```rust
pub trait NodePropertyValues: PropertyValues {
    fn long_value(&self, node_id: MappedNodeId) -> Option<i64>;
    fn double_value(&self, node_id: MappedNodeId) -> Option<f64>;
    fn double_array(&self, node_id: MappedNodeId) -> Option<Vec<f64>>;
    // ... type-specific accessors
}
```

#### 3.4 `RelationshipPropertyValues`

**Purpose**: Properties indexed by relationship position.

**Stability**: 🟢 **STABLE**

```rust
pub trait RelationshipPropertyValues: PropertyValues {
    fn long_value(&self, index: u64) -> Option<i64>;
    fn double_value(&self, index: u64) -> Option<f64>;
    fn double_array(&self, index: u64) -> Option<Vec<f64>>;
    // ... type-specific accessors
}
```

---

### 4. Property Store Traits

**Purpose**: Collections of properties with builder pattern for modifications.

**Stability**: 🟡 **STABILIZING** - API is functional but may add convenience methods

#### 4.1 `GraphPropertyStore`

```rust
pub trait GraphPropertyStore {
    type Property;
    type Builder: GraphPropertyStoreBuilder;

    fn empty() -> Self;
    fn new(properties: HashMap<String, Self::Property>) -> Self;
    fn builder() -> Self::Builder;

    fn has_property(&self, property_key: &str) -> bool;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_builder(&self) -> Self::Builder;
}

pub trait GraphPropertyStoreBuilder {
    type Store: GraphPropertyStore;
    type Property;

    fn new() -> Self;
    fn put_property(self, key: impl Into<String>,
        values: Arc<dyn GraphPropertyValues>) -> Self;
    fn build(self) -> Self::Store;
}
```

#### 4.2 `NodePropertyStore`

```rust
pub trait NodePropertyStore {
    type Property;
    type Builder: NodePropertyStoreBuilder;

    fn empty() -> Self;
    fn new(properties: HashMap<String, Self::Property>) -> Self;
    fn builder() -> Self::Builder;

    fn has_property(&self, property_key: &str) -> bool;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_builder(&self) -> Self::Builder;
}

pub trait NodePropertyStoreBuilder {
    type Store: NodePropertyStore;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;
    fn put_if_absent(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn remove_property(self, key: &str) -> Self;
    fn build(self) -> Self::Store;
}
```

#### 4.3 `RelationshipPropertyStore`

```rust
pub trait RelationshipPropertyStore {
    type Property;
    type Builder: RelationshipPropertyStoreBuilder;

    fn empty() -> Self;
    fn new(properties: HashMap<String, Self::Property>) -> Self;
    fn builder() -> Self::Builder;

    fn has_property(&self, property_key: &str) -> bool;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_builder(&self) -> Self::Builder;
}

pub trait RelationshipPropertyStoreBuilder {
    type Store: RelationshipPropertyStore;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;
    fn put_if_absent(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn remove_property(self, key: &str) -> Self;
    fn build(self) -> Self::Store;
}
```

---

## 🏗️ Concrete Implementations

### `DefaultGraphStore`

**Stability**: 🟢 **STABLE** - Primary implementation

**Constructor**:

```rust
pub fn new(
    graph_name: GraphName,
    database_info: DatabaseInfo,
    schema: GraphSchema,
    capabilities: Capabilities,
    id_map: SimpleIdMap,
    relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
) -> Self
```

**View Creation**:

```rust
pub fn graph(&self) -> Arc<DefaultGraph>
```

### `DefaultGraph`

**Stability**: 🟢 **STABLE** - Primary graph view

**Constructor**:

```rust
pub fn new(
    schema: Arc<GraphSchema>,
    id_map: Arc<SimpleIdMap>,
    characteristics: GraphCharacteristics,
    topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_types: Vec<RelationshipType>,
    inverse_indexed_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_edges: bool,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    relationship_properties: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    relationship_property_selectors: HashMap<RelationshipType, String>,
) -> Self
```

---

## 📝 Supporting Types

### Metadata Types

**Stability**: 🟢 **STABLE**

```rust
pub struct GraphName { /* ... */ }
pub struct DatabaseId { /* ... */ }
pub struct DatabaseInfo { /* ... */ }
pub enum DatabaseLocation { /* ... */ }
pub struct Capabilities { /* ... */ }
pub struct DeletionResult { /* ... */ }
```

### Schema Types

**Stability**: 🟢 **STABLE**

```rust
pub struct GraphSchema { /* ... */ }
pub struct NodeLabel { /* ... */ }
pub struct RelationshipType { /* ... */ }
pub struct PropertySchema { /* ... */ }
pub enum ValueType { Long, Double, String, Boolean, /* arrays */ }
pub enum PropertyState { Normal, Deleted }
pub struct DefaultValue { /* ... */ }
```

### Graph Structure Types

**Stability**: 🟢 **STABLE**

```rust
pub type MappedNodeId = u64;
pub type OriginalNodeId = i64;
pub const NOT_FOUND: i64 = -1;

pub struct RelationshipTopology { /* ... */ }
pub struct GraphCharacteristics { /* ... */ }
pub struct GraphCharacteristicsBuilder { /* ... */ }
```

### Property Cursor Types

**Stability**: 🟢 **STABLE**

```rust
pub trait RelationshipCursor {
    fn source_id(&self) -> MappedNodeId;
    fn target_id(&self) -> MappedNodeId;
    fn property(&self) -> PropertyValue;
}

pub type RelationshipStream<'a> = Box<dyn Iterator<Item = Box<dyn RelationshipCursor>> + 'a>;
pub type PropertyValue = f64;  // Will expand to enum in future
```

---

## 🔒 Internal APIs (Unstable)

These are implementation details that **may change**:

### Implementation Structs (Unstable)

- `DefaultLongNodePropertyValues`
- `DefaultDoubleNodePropertyValues`
- `DefaultRelationshipPropertyValues`
- `DefaultGraphPropertyValues`
- `DefaultRelationshipCursor`
- `DefaultModifiableRelationshipCursor`
- `SelectedRelationshipProperty` (private)
- `PropertyTraversalMode` (private)

### Helper Functions (Unstable)

- `compute_topology_offsets()` - internal optimization
- `build_selected_relationship_properties()` - internal caching
- `auto_select_property_key()` - internal heuristic

---

## 🎯 API Usage Patterns

### Pattern 1: Create a GraphStore

```rust
let graph_name = GraphName::new("my_graph");
let database_info = DatabaseInfo::new(
    DatabaseId::new("db"),
    DatabaseLocation::local(),
);
let schema = GraphSchema::empty();
let capabilities = Capabilities::default();
let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);

let topology = RelationshipTopology::new(
    vec![vec![1, 2], vec![2, 3], vec![3], vec![]],
    None
);

let mut topologies = HashMap::new();
topologies.insert(RelationshipType::of("KNOWS"), topology);

let store = DefaultGraphStore::new(
    graph_name,
    database_info,
    schema,
    capabilities,
    id_map,
    topologies,
);
```

### Pattern 2: Add Properties

```rust
// Add node property
let values = Arc::new(DefaultLongNodePropertyValues::new(
    vec![10, 20, 30, 40],
    4
));
store.add_node_property(
    HashSet::from([NodeLabel::of("Person")]),
    "age",
    values
)?;

// Add relationship property
let rel_values = Arc::new(DefaultRelationshipPropertyValues::with_default(
    vec![1.0, 2.0, 3.0],
    3
));
store.add_relationship_property(
    RelationshipType::of("KNOWS"),
    "weight",
    rel_values
)?;
```

### Pattern 3: Query via Graph View

```rust
let graph = store.graph();

// Traverse relationships
for cursor in graph.stream_relationships(0, 0.0) {
    println!("Edge: {} -> {} (weight: {})",
        cursor.source_id(),
        cursor.target_id(),
        cursor.property()
    );
}

// Check properties
if let Some(age_values) = graph.node_properties("age") {
    if let Some(age) = age_values.long_value(0) {
        println!("Node 0 age: {}", age);
    }
}

// Filter by relationship type
let filtered = graph.relationship_type_filtered_graph(
    &HashSet::from([RelationshipType::of("KNOWS")])
)?;
```

### Pattern 4: Modify Properties (via Builder)

```rust
// Relationship stores use builder pattern since they're immutable
// GraphStore handles this internally:
store.add_relationship_property(rel_type, "weight", values)?;
store.remove_relationship_property(&rel_type, "weight")?;

// Direct builder usage (for custom implementations):
let store = RelationshipPropertyStore::empty();
let updated = store.to_builder()
    .put("weight", property)
    .build();
```

---

## ✅ Stability Commitments

### What Won't Change (Guaranteed Stable)

1. **Core trait signatures**: `Graph`, `GraphStore`, `*PropertyValues`
2. **Type aliases**: `MappedNodeId`, `OriginalNodeId`, `GraphResult`, etc.
3. **Constructor signatures** for `DefaultGraph` and `DefaultGraphStore`
4. **Error types**: `GraphStoreError` variants
5. **Metadata types**: `GraphName`, `DatabaseInfo`, etc.

### What May Evolve (Semi-Stable)

1. **Property store builder APIs** - may add convenience methods
2. **Internal optimization helpers** - implementation details
3. **Default implementations** - may add more efficient versions
4. **PropertyValue type** - currently `f64`, will become enum

### What Will Change (Unstable)

1. **Concrete property value implementations** - internal details
2. **Private helper functions and structs**
3. **Performance optimization strategies**
4. **Memory layout of internal structures**

---

## 🚀 Next Steps for CoreGraphStore

When moving to CoreGraphStore, these APIs will be **enhanced but not broken**:

1. **Polars/Arrow Integration**: PropertyValues will internally use Arrow arrays
2. **Memory Mapping**: Large graphs will use mmap for storage
3. **Compression**: Optional compression for property values
4. **Serialization**: Save/load graph stores to disk
5. **Batch Operations**: Bulk insert/update APIs

The Pure → Core transition should be **additive**, not breaking.

---

## 📚 References

- TypeScript GDS API: `/ts-gds/api/`
- ADR 0001: Property Graph Store Design
- ADR 0002: Triadic GraphStore Architecture
- ADR 0003: Node Property Value Contract

---

**Document Status**: Ready for review  
**Next Review**: After integration tests
