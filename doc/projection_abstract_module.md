# Projection Abstract Module

## Overview

The abstract projection module provides foundational traits and types for graph element projections in the Rust GDS architecture. This module closely mirrors the TypeScript GDS implementation while leveraging Rust's strengths for type safety, performance, and concurrency.

## Module Structure

```
src/projection/abstract/
├── mod.rs                      # Module exports
├── property_mapping.rs         # Property mapping and aggregation
├── element_projection.rs       # Element projection traits and property mappings
└── abstract_projections.rs     # Collection management for projections
```

## Core Components

### 1. PropertyMapping (`property_mapping.rs`)

Represents a mapping from source graph properties to projected properties, with support for default values and aggregation strategies.

**Key Features:**

- Maps source property names to result property names
- Supports default values for missing properties
- Configurable aggregation strategies (Sum, Min, Max, Count, etc.)
- Validation for property keys and wildcard usage
- Builder pattern for fluent API

**Aggregation Strategies:**

```rust
pub enum Aggregation {
    Default,  // Context-dependent
    None,     // Fail on multiple values
    Sum,      // Sum all values
    Min,      // Take minimum
    Max,      // Take maximum
    Single,   // Take first value
    Count,    // Count occurrences
}
```

**Example:**

```rust
use rust_gds::projection::{PropertyMapping, Aggregation};

// Simple mapping
let mapping = PropertyMapping::of("age")?;

// Mapping with source property
let mapping = PropertyMapping::with_source("score", "user_score")?;

// Full builder
let mapping = PropertyMappingBuilder::new("weight")
    .neo_property_key("edge_weight")
    .aggregation(Aggregation::Max)
    .build()?;
```

### 2. ElementProjection (`element_projection.rs`)

Base trait for projections of graph elements (nodes or relationships) with property mappings.

**PropertyMappings Collection:**

- Manages multiple PropertyMapping instances
- Efficient lookup by property key
- Builder pattern for fluent construction
- Merge support for combining mappings

**InlineProperties Trait:**

- Enables fluent API for adding properties during projection building
- Supports incremental property addition
- Validates property conflicts

**Example:**

```rust
use rust_gds::projection::{PropertyMappings, PropertyMapping};

let mappings = PropertyMappings::builder()
    .add_property("age")?
    .add_property_with_source("score", "user_score")?
    .build();

assert_eq!(mappings.len(), 2);
```

### 3. AbstractProjections (`abstract_projections.rs`)

Base trait for collections of element projections, indexed by element identifiers (NodeLabel or RelationshipType).

**Key Features:**

- Generic over identifier and projection types
- Efficient HashMap-backed storage
- Query interface for projections
- Property aggregation across projections
- Builder pattern for construction

**Projections Implementation:**

```rust
use rust_gds::projection::{Projections, NodeLabel};

let projections = Projections::builder()
    .add(NodeLabel::of("Person"), person_projection)
    .add(NodeLabel::of("Company"), company_projection)
    .build();

assert_eq!(projections.size(), 2);
```

## Design Principles

### 1. Type Safety

- Strong typing for identifiers, properties, and aggregations
- Result types for fallible operations
- Generic trait constraints ensure correctness

### 2. Idiomatic Rust

- Builder patterns for fluent APIs
- Trait-based abstractions
- Ownership and borrowing for zero-copy when possible
- Comprehensive error handling

### 3. Compatibility with TypeScript GDS

The Rust implementation mirrors TypeScript GDS while improving:

- **Type Safety**: Compile-time guarantees vs runtime checks
- **Performance**: Zero-copy operations, efficient collections
- **Concurrency**: Thread-safe designs, no GIL limitations
- **Memory**: Predictable allocation, no garbage collection pauses

### 4. Flexibility

- Generic over identifier and projection types
- Extensible via traits
- Composable builders
- Merge and filter operations

## API Patterns

### Builder Pattern

Most types provide fluent builders:

```rust
let mapping = PropertyMappingBuilder::new("property")
    .neo_property_key("source")
    .aggregation(Aggregation::Sum)
    .build()?;
```

### Factory Methods

Simple construction via `of` methods:

```rust
let mapping = PropertyMapping::of("property")?;
let mappings = PropertyMappings::empty();
```

### Trait-Based Extension

Implement `InlineProperties` for custom projection types:

```rust
impl InlineProperties for MyProjection {
    fn inline_builder(&mut self) -> &mut InlinePropertiesBuilder {
        &mut self.inline_builder
    }
}
```

## Testing

All abstract projection types have comprehensive test coverage:

- Property mapping creation and validation
- Aggregation parsing and resolution
- Property mappings builder and merge
- Inline properties builder
- Projections collection management

Run tests:

```bash
cargo test projection::abstract::
```

## Integration Points

### With Schema Module

- PropertyMapping uses `DefaultValue` from schema
- Element identifiers (NodeLabel, RelationshipType) from projection root

### With Property Stores

- PropertyMappings define how properties are projected
- Aggregation strategies control value combination

### With Graph Projections

- NodeProjection and RelationshipProjection will use these abstractions
- GraphStore filters and transforms via projections

## Performance Characteristics

- **PropertyMapping**: O(1) creation, O(1) validation
- **PropertyMappings**: O(1) lookup, O(n) merge, O(n) iteration
- **Projections**: O(1) lookup, O(n) iteration, HashMap-backed

## Next Steps

1. **NodeProjection**: Concrete projection for nodes using these abstractions
2. **RelationshipProjection**: Concrete projection for relationships
3. **GraphProjection**: Combine node and relationship projections
4. **Integration**: Wire projections into GraphStore and IdMap

## Dependencies

- `crate::types::schema::default_value::DefaultValue` - For default values
- `std::collections::{HashMap, HashSet}` - For collections
- `serde_json` - For serialization (ElementProjection)

## Thread Safety

All types are thread-safe when appropriate:

- `PropertyMapping`: `Send + Sync` (immutable after creation)
- `PropertyMappings`: `Send + Sync` (uses interior mutability via builders)
- Builders: Not `Send + Sync` (designed for single-threaded construction)
