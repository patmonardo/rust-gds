# Projection Module

## Overview

The projection module provides foundational types for identifying and classifying nodes and relationships in graph projections. These types are central to the GDS architecture and mirror the TypeScript implementation while leveraging Rust's strengths.

## Components

### NodeLabel (`src/projection/node_label.rs`)

A type-safe, interned identifier for node labels in a graph.

**Key Features:**

- **Interning**: Labels with the same name share underlying storage for memory efficiency
- **Thread-safe**: Uses `RwLock` for concurrent access to the instance cache
- **Special Labels**: `ALL_NODES` (`"__ALL__"`) for wildcard matching
- **Factory Method**: `NodeLabel::of(name)` for creating/retrieving interned instances
- **Bulk Creation**: `NodeLabel::list_of(["Person", "Company"])` for multiple labels

**Traits Implemented:**

- `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`
- `Display`, `Debug`

**Example:**

```rust
use rust_gds::projection::NodeLabel;

let person = NodeLabel::of("Person");
let company = NodeLabel::of("Company");
let all = NodeLabel::all_nodes();

assert_eq!(person.name(), "Person");
assert!(all.is_all_nodes());
```

### RelationshipType (`src/projection/relationship_type.rs`)

A type-safe, interned identifier for relationship types in a graph.

**Key Features:**

- **Interning**: Types with the same name share underlying storage
- **Thread-safe**: Uses `RwLock` for concurrent access
- **Special Types**: `ALL_RELATIONSHIPS` (`"__ALL__"`) for wildcard matching
- **Factory Method**: `RelationshipType::of(name)` for creating/retrieving instances
- **Bulk Creation**: `RelationshipType::list_of(["KNOWS", "FOLLOWS"])` for multiple types

**Traits Implemented:**

- `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`
- `Display`, `Debug`

**Example:**

```rust
use rust_gds::projection::RelationshipType;

let knows = RelationshipType::of("KNOWS");
let follows = RelationshipType::of("FOLLOWS");
let all = RelationshipType::all_relationships();

assert_eq!(knows.name(), "KNOWS");
assert!(all.is_all_relationships());
```

## Design Principles

### 1. Interning Pattern

Both `NodeLabel` and `RelationshipType` use interning to ensure:

- Memory efficiency: Same-named instances share storage
- Fast equality: Pointer comparison for interned instances
- Thread-safety: Concurrent access via `RwLock`

### 2. Idiomatic Rust

- Uses `Arc<String>` for shared ownership of names
- Implements standard traits (`Hash`, `Eq`, `Ord`, etc.)
- Provides ergonomic factory methods (`of`, `list_of`)
- Comprehensive test coverage

### 3. Compatibility with TypeScript GDS

The Rust implementation mirrors the TypeScript API:

- Factory method pattern (`of`, `listOf`)
- Special wildcard constants (`ALL_NODES`, `ALL_RELATIONSHIPS`)
- Instance caching for efficiency
- Similar naming conventions

## Testing

All projection types have comprehensive test coverage:

- Creation and interning
- Special constants (ALL_NODES, ALL_RELATIONSHIPS)
- Bulk creation (list_of)
- Display formatting
- Ordering and comparison

Run tests:

```bash
cargo test projection::
```

## Usage in Graph Store

These types are foundational for:

- **IdMap**: Node labels identify node collections
- **Graph Schema**: Defines available labels and relationship types
- **Property Stores**: Properties are associated with labels/types
- **Projection Filters**: Select subsets of nodes/relationships by label/type

## Next Steps

1. **IdMap**: Implement node ID mapping with label support
2. **Graph Traits**: Define Graph and GraphStore traits using these types
3. **Schema Integration**: Wire projection types into schema module
4. **Property Store**: Connect property stores to labels and types

## Dependencies

- `lazy_static`: For static instance caches
- `std::sync::RwLock`: For thread-safe access to caches
- `std::sync::Arc`: For shared ownership of names

## Performance Characteristics

- **Creation**: O(log n) for first instance, O(1) for subsequent (with read lock)
- **Comparison**: O(1) for interned instances (pointer equality)
- **Memory**: Shared storage for duplicate names
- **Thread-safety**: Read-optimized locking (many readers, few writers)
