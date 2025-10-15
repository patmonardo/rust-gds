# Triadic Property Store Architecture

## Overview

The property store system follows a **triadic design pattern** with three parallel implementations:

1. **NodePropertyStore** - Properties indexed by node ID
2. **GraphPropertyStore** - Properties for the entire graph
3. **RelationshipPropertyStore** - Properties indexed by relationship ID

Each domain follows an identical architectural pattern for consistency and maintainability.

## Unified Design Pattern

### Four-Component Structure

Each property store domain consists of:

1. **Store Trait** - Contract defining what a property store can do
2. **Builder Trait** - Contract defining how to construct a store
3. **Store Implementation** - Concrete struct with HashMap storage
4. **Builder Implementation** - Concrete builder struct

### Complete Implementation Structure

Each domain has **exactly 4 impl blocks** in its default implementation:

```rust
// 1. Store Trait Implementation
impl {Node|Graph|Relationship}PropertyStore for Default{...}PropertyStore { ... }

// 2. Builder Trait Implementation
impl {Node|Graph|Relationship}PropertyStoreBuilder for Default{...}PropertyStoreBuilder { ... }

// 3. Store Inherent Methods (ergonomics)
impl Default{...}PropertyStore {
    pub fn len(&self) -> usize { ... }
    pub fn is_empty(&self) -> bool { ... }
    pub fn get(&self, key: &str) -> Option<&Property> { ... }
    pub fn contains_key(&self, key: &str) -> bool { ... }
    pub fn {node|graph|relationship}_properties(&self) -> &HashMap<String, Property> { ... }
}

// 4. Builder Inherent Methods (ergonomics)
impl Default{...}PropertyStoreBuilder {
    pub fn put_property(&self, key: impl Into<String>, values: Arc<dyn PropertyValues>) -> Self { ... }
}
```

## Trait Alignment

### Store Trait Methods (Standardized)

All three store traits now have identical method signatures:

```rust
pub trait {Node|Graph|Relationship}PropertyStore {
    type Property;
    type Builder: ...StoreBuilder<Store = Self, Property = Self::Property>;

    fn empty() -> Self where Self: Sized;
    fn new(properties: HashMap<String, Self::Property>) -> Self where Self: Sized;
    fn builder() -> Self::Builder where Self: Sized;

    fn has_property(&self, property_key: &str) -> bool;
    fn property_key_set(&self) -> Vec<&str>;
    fn get_property(&self, property_key: &str) -> Option<&Self::Property>;
    fn get_all_properties(&self) -> Vec<&Self::Property>;
    fn get_property_values(&self, property_key: &str) -> Option<&dyn PropertyValues>;

    fn size(&self) -> usize;           // ← Standardized (was count() for Node)
    fn is_empty(&self) -> bool;
    fn to_builder(&self) -> Self::Builder;
}
```

**Key Change**: NodePropertyStore now uses `size()` instead of `count()` for consistency.

### Builder Trait Methods (Standardized)

All three builder traits now have identical method signatures:

```rust
pub trait {Node|Graph|Relationship}PropertyStoreBuilder {
    type Store: ...PropertyStore<Builder = Self, Property = Self::Property>;
    type Property;

    fn new() -> Self;
    fn from_store(store: &Self::Store) -> Self;

    fn properties(self, props: HashMap<String, Self::Property>) -> Self;
    fn put_if_absent(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn put(self, key: impl Into<String>, property: Self::Property) -> Self;
    fn remove_property(self, key: &str) -> Self;

    fn build(self) -> Self::Store;
}
```

**Key Changes**:

- GraphPropertyStoreBuilder added: `properties()`, `put_if_absent()`, `remove_property()`
- All three now have the exact same builder API

## Why Inherent Impls?

### The Ergonomics Problem

**Without inherent methods** (requires trait import):

```rust
use crate::types::properties::node::NodePropertyStore;  // ← Must import trait!

let store = DefaultNodePropertyStore::builder().build();
if store.is_empty() { ... }  // ← Only works with trait in scope
```

**With inherent methods** (no imports needed):

```rust
// No trait import needed!
let store = DefaultNodePropertyStore::builder().build();
if store.is_empty() { ... }  // ← Works directly, more ergonomic
let len = store.len();         // ← Common method works without imports
```

### Consistency Across Domains

All three domains now provide the same ergonomic convenience methods:

```rust
// Node properties
let node_store = DefaultNodePropertyStore::builder().build();
assert!(node_store.is_empty());
assert_eq!(node_store.len(), 0);

// Graph properties
let graph_store = DefaultGraphPropertyStore::builder().build();
assert!(graph_store.is_empty());
assert_eq!(graph_store.len(), 0);

// Relationship properties
let rel_store = DefaultRelationshipPropertyStore::builder().build();
assert!(rel_store.is_empty());
assert_eq!(rel_store.len(), 0);
```

## Implementation Files

### Node Domain

- **Trait**: `src/types/properties/node/node_property_store.rs`
- **Implementation**: `src/types/properties/node/impls/default_node_property_store.rs`

### Graph Domain

- **Trait**: `src/types/properties/graph/graph_property_store.rs`
- **Implementation**: `src/types/properties/graph/impls/default_graph_property_store.rs`

### Relationship Domain

- **Trait**: `src/types/properties/relationship/relationship_property_store.rs`
- **Implementation**: `src/types/properties/relationship/impls/default_relationship_property_store.rs`

## Benefits of Standardization

### 1. **Predictable API**

Once you learn one property store domain, you know all three. Methods have the same names and signatures.

### 2. **Easy to Extend**

When adding new storage backends (Arrow2, MMap, etc.), follow the same 4-impl pattern:

```rust
// New Arrow2-backed node property store
impl NodePropertyStore for Arrow2NodePropertyStore { ... }      // 1. Trait
impl NodePropertyStoreBuilder for Arrow2NodePropertyStoreBuilder { ... }  // 2. Builder
impl Arrow2NodePropertyStore { /* convenience */ }              // 3. Inherent store
impl Arrow2NodePropertyStoreBuilder { /* convenience */ }       // 4. Inherent builder
```

### 3. **Clear Code Organization**

Each implementation file has a consistent structure:

```
1. Imports
2. Struct definitions (Store + Builder)
3. Store trait implementation
4. Builder trait implementation
5. Store inherent methods
6. Builder inherent methods
7. Tests
```

### 4. **Reduced Cognitive Load**

Developers don't need to remember which domain has which methods. All three are identical.

### 5. **Maintainability**

Bug fixes and improvements can be applied symmetrically across all three domains.

## Usage Examples

### Building Stores (Identical Pattern)

```rust
// Node properties
let node_store = DefaultNodePropertyStore::builder()
    .put_property("age", Arc::new(age_values))
    .put_property("score", Arc::new(score_values))
    .build();

// Graph properties
let graph_store = DefaultGraphPropertyStore::builder()
    .put_property("density", Arc::new(density_values))
    .put_property("diameter", Arc::new(diameter_values))
    .build();

// Relationship properties
let rel_store = DefaultRelationshipPropertyStore::builder()
    .put_property("weight", Arc::new(weight_values))
    .put_property("distance", Arc::new(distance_values))
    .build();
```

### Querying Stores (Identical Pattern)

```rust
// All three domains use the same API
if store.contains_key("age") {
    let prop = store.get("age").unwrap();
    println!("Found {} properties", store.len());
}
```

## Future Extensions

This standardized architecture makes it easy to add:

1. **Arrow2-backed stores** - Columnar storage for efficient IO
2. **MMap stores** - Memory-mapped properties for large graphs
3. **Compressed stores** - RLE or dictionary encoding for repeated values
4. **Distributed stores** - Partitioned properties across machines
5. **GPU stores** - Device-memory properties for accelerated algorithms

Each new storage backend just implements the same 4-impl pattern!

## Testing Strategy

Each implementation includes comprehensive tests covering:

- Empty builder behavior
- Put and get operations
- Put-if-absent semantics
- Property removal
- Builder round-trips (to_builder → build)
- Inherent method access (len, is_empty, etc.)

See test modules in each implementation file for examples.

## Aesthetic Achievement

The triadic property store system demonstrates **code aesthetics through consistency**:

- **Symmetry**: Three parallel domains with identical structure
- **Predictability**: Same methods, same patterns, same behavior
- **Extensibility**: Clear template for new storage backends
- **Ergonomics**: Inherent methods reduce boilerplate imports
- **Maintainability**: Fix once, apply everywhere

This is architecture as art! 🎨
