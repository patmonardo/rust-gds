# Property Trait Implementation Pattern

This document describes the standardized pattern for implementing the `Property` trait across all three property domains (Graph, Node, Relationship).

## Canonical Pattern

All `Default*Property` structs follow this exact pattern:

### Structure

```rust
#[derive(Debug, Clone)]
pub struct Default*Property {
    values: Arc<dyn *PropertyValues>,
    schema: PropertySchema,
}
```

### Imports (standardized order)

```rust
use crate::types::default_value::DefaultValue;
use crate::types::properties::Property;           // ← canonical re-export
use crate::types::properties::PropertyValues;     // ← canonical re-export
use crate::types::properties::{domain}::{Domain}PropertyValues;
use crate::types::property_state::PropertyState;
use crate::types::schema::PropertySchema;
use std::sync::Arc;
```

### Property Trait Implementation (identical across all domains)

```rust
impl Property for Default*Property {
    fn values(&self) -> Arc<dyn PropertyValues> {
        Arc::clone(&self.values) as Arc<dyn PropertyValues>
    }

    fn schema(&self) -> &PropertySchema {
        &self.schema
    }
}
```

**Key points:**

1. **Explicit cast required**: `as Arc<dyn PropertyValues>` is necessary because Rust doesn't automatically coerce `Arc<dyn SubTrait>` → `Arc<dyn SuperTrait>`
2. **Method order**: `values()` first, then `schema()` (matches trait definition order)
3. **No associated types**: The `Property` trait has no associated `Values` type

## Domain-Specific Accessors

Each implementation provides domain-specific accessors in addition to the trait impl:

```rust
impl Default*Property {
    // Constructors
    pub fn of(key: impl Into<String>, values: Arc<dyn *PropertyValues>) -> Self
    pub fn with_state(key, state, values) -> Self
    pub fn with_default(key, state, values, default_value) -> Self
    pub fn with_schema(schema, values) -> Self

    // Domain-specific accessors
    pub fn values(&self) -> &dyn *PropertyValues           // trait object reference
    pub fn values_arc(&self) -> Arc<dyn *PropertyValues>   // cloned Arc handle
    pub fn property_schema(&self) -> &PropertySchema
    pub fn key(&self) -> &str
}
```

## Why This Pattern

### Import Path Consistency

- Use `crate::types::properties::Property` (canonical re-export from `mod.rs`)
- Avoid nested paths like `crate::types::properties::property::Property`
- This ensures all files resolve to the same trait definition

### Explicit Cast Necessity

The trait hierarchy is:

```
PropertyValues (base trait)
    ├── GraphPropertyValues
    ├── NodePropertyValues
    └── RelationshipPropertyValues
```

Rust requires explicit casting when returning a trait object through a supertrait:

- ✗ `Arc::clone(&self.values)` — type error
- ✓ `Arc::clone(&self.values) as Arc<dyn PropertyValues>` — explicit upcast

### Domain Accessor Coexistence

- **Trait method** `values(&self) -> Arc<dyn PropertyValues>` — for generic Property usage
- **Inherent method** `values(&self) -> &dyn *PropertyValues` — for domain-specific usage
- **Helper method** `values_arc(&self) -> Arc<dyn *PropertyValues>` — cheap Arc clone

These don't conflict because:

1. Trait methods and inherent methods occupy different namespaces
2. The trait method returns the owned Arc (required by trait)
3. The inherent methods provide ergonomic domain-specific access

## Files Using This Pattern

- `src/types/properties/graph/impls/default_graph_property.rs`
- `src/types/properties/node/impls/default_node_property.rs`
- `src/types/properties/relationship/impls/default_relationship_property.rs`

All three files now use **identical** `impl Property` blocks.
