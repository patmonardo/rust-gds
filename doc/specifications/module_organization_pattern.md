# Module Organization Pattern

## Philosophy

This codebase follows a **top-level module export pattern** that eliminates redundant path segments and keeps imports clean and predictable.

## Core Principles

### 1. Export everything at the top level

Each `mod.rs` re-exports all public items from its submodules so consumers never need to reference implementation file names.

```rust
// ✅ Good: import from top-level module
use crate::types::properties::Property;
use crate::types::properties::PropertyValues;
use crate::types::properties::node::NodePropertyValues;

// ❌ Bad: referencing implementation files
use crate::types::properties::property::Property;
use crate::types::properties::property_values::PropertyValues;
```

### 2. No single-module file names in paths

If a type lives in `foo/bar.rs`, it should be re-exported by `foo/mod.rs`. Consumers import from `foo`, never `foo::bar`.

```rust
// In src/types/properties/mod.rs
pub mod property;           // implementation file
pub use property::Property; // ← re-export at module level

// In consumer code
use crate::types::properties::Property;  // ✅ Clean
```

### 3. Important submodules appear explicitly

When a submodule contains a cohesive subsystem (like `node`, `graph`, `relationship`), it appears in the path:

```rust
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
```

This reflects the **domain architecture** (graph/node/relationship are first-class concepts).

### 4. Eliminate verbose paths in expressions

Prefer importing types at the top and using short names in the code:

```rust
// ✅ Good: import once, use short name
use crate::types::PropertyState;

fn example() {
    let state = PropertyState::Persistent;
}

// ❌ Bad: full path in expression
fn example() {
    let state = crate::types::property_state::PropertyState::Persistent;
}
```

## Module Architecture

```
src/
├── lib.rs                          // Crate root, re-exports major modules
├── types/
│   ├── mod.rs                      // Re-exports all types/* items
│   ├── default_value.rs
│   ├── property_state.rs
│   ├── value_type.rs
│   ├── schema/
│   │   ├── mod.rs                  // Re-exports schema types
│   │   └── property_schema.rs
│   └── properties/
│       ├── mod.rs                  // Re-exports Property, PropertyValues, etc.
│       ├── property.rs             // Property trait definition
│       ├── property_values.rs      // PropertyValues trait definition
│       ├── graph/
│       │   ├── mod.rs              // Re-exports graph property types
│       │   ├── graph_property_values.rs
│       │   └── impls/
│       │       ├── mod.rs
│       │       ├── default_graph_property.rs
│       │       └── ...
│       ├── node/
│       │   └── ... (same pattern)
│       └── relationship/
│           └── ... (same pattern)
├── projection/
│   ├── mod.rs                      // Re-exports projection types
│   └── ...
└── values/
    ├── mod.rs                      // Re-exports value types
    └── ...
```

## Pattern for `mod.rs`

Every `mod.rs` follows this template:

```rust
// 1. Declare submodules (private by default)
mod property;
mod property_values;

// 2. Re-export public items at this level
pub use property::Property;
pub use property_values::PropertyValues;

// 3. Declare and re-export nested submodules if they're cohesive subsystems
pub mod graph;
pub mod node;
pub mod relationship;
```

**Key insight:** Implementation file names (`property.rs`, `property_values.rs`) are internal details. The public API is what `mod.rs` re-exports.

## Standard Import Order (Consistent Across Files)

```rust
// 1. Standard library
use std::sync::Arc;

// 2. External crates
use serde::{Deserialize, Serialize};

// 3. Crate-level types (alphabetical)
use crate::types::default_value::DefaultValue;
use crate::types::properties::Property;
use crate::types::properties::PropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::property_state::PropertyState;
use crate::types::schema::PropertySchema;
```

## Migration Checklist

- [x] Properties module: standardized to top-level exports
- [x] Property trait implementations: consistent imports across graph/node/relationship
- [x] DefaultValue usage: modernized to lowercase API
- [ ] Audit remaining verbose paths in expressions (grep `crate::.*::.*::.*` in non-import lines)
- [ ] Verify all `mod.rs` follow the re-export pattern

## Why This Matters

This pattern makes the codebase:

1. **Predictable** — consumers know where to import from
2. **Refactorable** — moving implementation files doesn't break imports if `mod.rs` stays stable
3. **Readable** — no stuttering paths like `foo::foo::Foo`
4. **Maintainable** — AI agents and humans can follow the same mental model

## Reference

- Java GDS (baroque but sensible): complex abstractions with clear domain boundaries
- Rust-GDS philosophy: embrace the abstraction machine, but keep the module graph simple

---

**Status:** ✅ Clean build, clippy, and tests all green. Module architecture stabilized.
