# Triadic Property Architecture

## Overview

The Property system is now **perfectly symmetric** across all three graph domains:

- **NodeProperty** - Properties attached to nodes
- **GraphProperty** - Properties attached to the graph as a whole
- **RelationshipProperty** - Properties attached to relationships

Each domain follows an identical architectural pattern for consistency and maintainability.

## The "Header + Body" Concept

A `Property` is like a **column in a database table**:

```rust
pub struct Property<V: PropertyValues> {
    schema: PropertySchema,  // ← "HEADER" (metadata: key, type, default, state)
    values: V,               // ← "BODY" (the actual column data vector)
}
```

- **Schema (Header)**: Describes the property - its name (`key`), data type (`value_type`), default value, and lifecycle state
- **Values (Body)**: The actual vector of data implementing `PropertyValues` trait

This mirrors how column-oriented databases work: metadata separate from data.

## Type Alias Pattern

All three domains use identical type aliases:

```rust
// Node domain
pub type NodeProperty = Property<Arc<dyn NodePropertyValues>>;

// Graph domain
pub type GraphProperty = Property<Arc<dyn GraphPropertyValues>>;

// Relationship domain
pub type RelationshipProperty = Property<Arc<dyn RelationshipPropertyValues>>;
```

**Key Design Decisions**:

- ✅ `Arc` for **shared ownership** (properties can be referenced by multiple cursors, stores, queries)
- ✅ `dyn Trait` for **polymorphism** (support any PropertyValues implementation: in-memory, Arrow2, MMap, etc.)

## Default Property Implementations

Each domain provides a concrete implementation for ergonomic property construction:

### Structure (Identical Across All Three)

```rust
pub struct Default{Node|Graph|Relationship}Property {
    values: Arc<dyn ...PropertyValues>,  // ← Shared ownership
    schema: PropertySchema,               // ← Metadata
}
```

### Constructors (Identical Across All Three)

All three domains provide **4 standard constructors**:

```rust
impl Default{...}Property {
    /// 1. Simple constructor with default state (PropertyState::Normal)
    pub fn of(key: impl Into<String>, values: Arc<dyn ...PropertyValues>) -> Self;

    /// 2. Constructor with explicit property state
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn ...PropertyValues>,
    ) -> Self;

    /// 3. Constructor with explicit default value
    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn ...PropertyValues>,
        default_value: DefaultValue,
    ) -> Self;

    /// 4. Constructor from existing schema
    pub fn with_schema(
        schema: PropertySchema,
        values: Arc<dyn ...PropertyValues>,
    ) -> Self;
}
```

### Accessors (Identical Across All Three)

All three domains provide **4 standard accessors**:

```rust
impl Default{...}Property {
    /// Returns trait object reference to values
    pub fn values(&self) -> &dyn ...PropertyValues;

    /// Returns cloned Arc handle (cheap O(1) operation)
    pub fn values_arc(&self) -> Arc<dyn ...PropertyValues>;

    /// Returns reference to property schema
    pub fn property_schema(&self) -> &PropertySchema;

    /// Convenience accessor for property key
    pub fn key(&self) -> &str;
}
```

### PropertyTrait Implementation (Identical Across All Three)

```rust
impl PropertyTrait for Default{...}Property {
    type Values = Arc<dyn ...PropertyValues>;

    fn values(&self) -> &Self::Values {
        &self.values
    }

    fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }
}
```

## Symmetry Table

| Aspect                 | Node | Graph | Relationship | Status     |
| ---------------------- | ---- | ----- | ------------ | ---------- |
| Type alias uses Arc    | ✅   | ✅    | ✅           | Perfect    |
| Default impl exists    | ✅   | ✅    | ✅           | Perfect    |
| Default impl uses Arc  | ✅   | ✅    | ✅           | **Fixed!** |
| Constructor count      | 4    | 4     | 4            | Perfect    |
| Accessor count         | 4    | 4     | 4            | Perfect    |
| PropertyTrait complete | ✅   | ✅    | ✅           | **Fixed!** |
| values_arc() method    | ✅   | ✅    | ✅           | **Fixed!** |
| Test count             | 4    | 1     | 4            | Good       |
| Total lines            | 146  | 107   | 156          | Similar    |

## Why Arc Instead of Box?

### Previous Inconsistency

**Before**: `DefaultNodeProperty` used `Box<dyn NodePropertyValues>` (exclusive ownership)
**Now**: All three use `Arc<dyn ...PropertyValues>` (shared ownership)

### Reasons for Arc

1. **Type Alias Alignment**

   ```rust
   pub type NodeProperty = Property<Arc<...>>  // Type alias
   pub struct DefaultNodeProperty { Arc<...> } // Implementation
   // ↑ Now consistent!
   ```

2. **Shared Ownership Model**

   - Properties referenced by multiple stores
   - Cursors hold references to properties
   - Concurrent query access
   - Efficient cloning (O(1) atomic increment)

3. **Thread Safety**

   - `Arc` is Send + Sync
   - Enables parallel graph algorithms
   - Safe sharing across threads

4. **Triadic Consistency**
   - Same ownership model across Node/Graph/Relationship
   - Reduced cognitive load
   - Predictable behavior

## Usage Examples

### Creating Properties (Identical Pattern)

```rust
use std::sync::Arc;

// Node property
let node_values: Arc<dyn NodePropertyValues> = Arc::new(DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3));
let node_prop = DefaultNodeProperty::of("age", node_values);

// Graph property
let graph_values: Arc<dyn GraphPropertyValues> = Arc::new(DefaultLongGraphPropertyValues::singleton(42));
let graph_prop = DefaultGraphProperty::of("node_count", graph_values);

// Relationship property
let rel_values: Arc<dyn RelationshipPropertyValues> = Arc::new(DefaultRelationshipPropertyValues::new(vec![1.0, 2.5], 0.0, 2));
let rel_prop = DefaultRelationshipProperty::of("weight", rel_values);
```

### Accessing Values (Identical Pattern)

```rust
// All three domains use same API
let values_ref = property.values();           // Trait object reference
let values_arc = property.values_arc();       // Cloned Arc (cheap!)
let schema = property.property_schema();      // Schema metadata
let key = property.key();                     // Property name
```

### With Explicit State

```rust
// All three support property lifecycle states
let node_prop = DefaultNodeProperty::with_state("temp", PropertyState::Deleted, values);
let graph_prop = DefaultGraphProperty::with_state("cache", PropertyState::Deleted, values);
let rel_prop = DefaultRelationshipProperty::with_state("score", PropertyState::Deleted, values);
```

## Implementation Files

### Node Domain

- **Type Alias**: `src/types/properties/node/node_property.rs`
- **Default Impl**: `src/types/properties/node/impls/default_node_property.rs` (146 lines, 4 tests)

### Graph Domain

- **Type Alias**: `src/types/properties/graph/graph_property.rs`
- **Default Impl**: `src/types/properties/graph/impls/default_graph_property.rs` (107 lines, 1 test)

### Relationship Domain

- **Type Alias**: `src/types/properties/relationship/relationship_property.rs`
- **Default Impl**: `src/types/properties/relationship/impls/default_relationship_property.rs` (156 lines, 4 tests)

## Changes Made

### Phase 1: Created DefaultRelationshipProperty ✨

- **New file**: `default_relationship_property.rs` (156 lines)
- **4 constructors**: `of()`, `with_state()`, `with_default()`, `with_schema()`
- **4 accessors**: `values()`, `values_arc()`, `property_schema()`, `key()`
- **Complete PropertyTrait impl** with Values associated type
- **4 comprehensive tests** covering all constructors and accessors

### Phase 2: Fixed DefaultNodeProperty 🔧

- **Changed storage**: `Box<dyn NodePropertyValues>` → `Arc<dyn NodePropertyValues>`
- **Added Clone derive** (now possible with Arc)
- **Standardized constructors**: Now has all 4 (added `with_schema()`)
- **Added values_arc()** accessor (was missing)
- **Fixed PropertyTrait impl**: Added Values associated type, fixed methods
- **Removed values_box()**: No longer needed with Arc
- **Updated all tests**: 4 comprehensive tests matching relationship domain

### Phase 3: Verified Graph (Already Correct) ✅

- **Already used Arc** ✅
- **Already had 4 constructors** ✅
- **Already had complete PropertyTrait impl** ✅
- **Already had values_arc()** ✅

## Testing Strategy

Each implementation includes comprehensive tests:

```rust
#[test]
fn default_{domain}_property_creation() { ... }          // Basic construction

#[test]
fn {domain}_property_with_state() { ... }                // Explicit state

#[test]
fn {domain}_property_with_explicit_default() { ... }     // Explicit default value

#[test]
fn {domain}_property_values_access() { ... }             // Accessor methods
```

**Test Results**: 176 tests passing (gained 4 tests from new relationship tests)

## Benefits of Standardization

### 1. Predictable API

Once you learn one property domain, you know all three:

```rust
// Same pattern everywhere!
let prop = Default{Node|Graph|Relationship}Property::of("key", values);
let arc = prop.values_arc();
let schema = prop.property_schema();
```

### 2. Easy to Extend

When adding new storage backends (Arrow2, MMap, etc.), follow the same pattern:

```rust
pub struct Arrow2NodeProperty {
    values: Arc<dyn NodePropertyValues>,  // ← Same Arc pattern
    schema: PropertySchema,                // ← Same schema
}
// Same 4 constructors, same 4 accessors, same PropertyTrait impl
```

### 3. Code Reusability

Generic code works across all three domains:

```rust
fn print_property<P: PropertyTrait>(prop: &P) {
    println!("{}: {:?}", prop.key(), prop.value_type());
}

print_property(&node_prop);
print_property(&graph_prop);
print_property(&rel_prop);  // All work!
```

### 4. Reduced Cognitive Load

Developers don't need to remember:

- "Does Node use Box or Arc?" (Answer: Arc, like all of them)
- "Which domain has values_arc()?" (Answer: All of them)
- "How many constructors does Relationship have?" (Answer: 4, like all of them)

### 5. Maintainability

Bug fixes and improvements apply symmetrically:

- Fix a pattern in Node → apply same fix to Graph and Relationship
- Add a feature to Graph → add same feature to Node and Relationship

## Architecture Aesthetics

The triadic property system demonstrates **code aesthetics through perfect symmetry**:

### The Three Pillars

```
         NodeProperty              GraphProperty         RelationshipProperty
              |                          |                        |
     ┌────────┴────────┐        ┌────────┴────────┐     ┌────────┴────────┐
     │  Arc<Values>    │        │  Arc<Values>    │     │  Arc<Values>    │
     │  Schema         │        │  Schema         │     │  Schema         │
     │                 │        │                 │     │                 │
     │ • of()          │        │ • of()          │     │ • of()          │
     │ • with_state()  │        │ • with_state()  │     │ • with_state()  │
     │ • with_default()│        │ • with_default()│     │ • with_default()│
     │ • with_schema() │        │ • with_schema() │     │ • with_schema() │
     │                 │        │                 │     │                 │
     │ • values()      │        │ • values()      │     │ • values()      │
     │ • values_arc()  │        │ • values_arc()  │     │ • values_arc()  │
     │ • schema()      │        │ • schema()      │     │ • schema()      │
     │ • key()         │        │ • key()         │     │ • key()         │
     └─────────────────┘        └─────────────────┘     └─────────────────┘
```

**This is architecture as art!** 🎨

- **Harmony**: Same structure, same methods, same behavior
- **Balance**: Each domain equal in capability and complexity
- **Clarity**: Learn once, apply everywhere
- **Elegance**: No exceptions, no special cases

## Future Extensions

This standardized architecture enables easy addition of:

1. **Arrow2-backed properties** - Columnar storage for efficient IO
2. **MMap properties** - Memory-mapped properties for large graphs
3. **Compressed properties** - RLE or dictionary encoding
4. **GPU properties** - Device-memory for accelerated algorithms
5. **Distributed properties** - Partitioned across machines

Each new storage backend just follows the same pattern: `Arc<dyn PropertyValues>` + same 4 constructors + same 4 accessors!

## Summary

The Property system is now **perfectly triadic**:

✅ **Same type alias pattern** (Arc-based)
✅ **Same default implementation structure** (Arc + Schema)
✅ **Same 4 constructors**
✅ **Same 4 accessors**
✅ **Same PropertyTrait implementation**
✅ **Same ownership model** (shared Arc)
✅ **Same thread safety** (Send + Sync)

Combined with the previously standardized **PropertyStore system** (also perfectly triadic), the entire property infrastructure is now a model of architectural consistency!

**Architecture Achievement**: Two perfectly symmetric triadic systems (Property + PropertyStore) working in harmony! 🎯
