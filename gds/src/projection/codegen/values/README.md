# Values Codegen: The Master Controller System

This module houses the **ValueType Master Controller** - the single source of truth for all property value generation in rust-gds.

## Architecture Overview

```
value_type_table!              ← Master Controller (defines all types)
    ↓
Universal Adapter Macros       ← Generate generic structs
    ↓
Trait Implementation Helpers   ← Generate trait impls per category
    ↓
Batch Generators              ← Generate all adapters from table
    ↓
Property Values               ← Node/Relationship/Graph implementations
```

## Core Components

### 1. `value_type_table.rs` - The Master Controller

Defines the canonical mapping from `ValueType` to Rust types:

```rust
value_type_table!(callback_macro);

// Expands to:
callback_macro!(Long, i64, IntegralScalar, 0i64);
callback_macro!(Double, f64, FloatingPointScalar, 0.0f64);
callback_macro!(LongArray, Option<Vec<i64>>, IntegralArray, None);
// ... etc for all 46 ValueTypes
```

**Categories:**
- `IntegralScalar`: i8, i16, i32, i64, i128
- `FloatingPointScalar`: f32, f64
- `OtherScalar`: bool, char, String
- `IntegralArray`: Option<Vec<i8>>, Option<Vec<i16>>, etc.
- `FloatingPointArray`: Option<Vec<f32>>, Option<Vec<f64>>
- `OtherArray`: Option<Vec<bool>>, Option<Vec<char>>, Option<Vec<String>>

### 2. Property Macros (in `../property/`)

Located in `projection/codegen/property/`:

- **`triadic_macros.rs`** - Universal adapter generators:
  - `node_universal_adapter!` - Generate `DefaultLongNodePropertyValues<C>`
  - `relationship_universal_adapter!` - Generate relationship adapters
  - `graph_universal_adapter!` - Generate graph adapters
  - `generate_all_node_adapters!()` - Batch generation from table
  - `generate_all_relationship_adapters!()` - Batch generation
  - `generate_all_graph_adapters!()` - Batch generation

- **`property_values.rs`** - Trait implementation helpers:
  - `impl_property_values_universal!` - Base `PropertyValues` trait
  - `impl_node_property_values_universal!` - `NodePropertyValues` trait
  - `impl_typed_node_property_values_universal!` - Type-specific traits
    - IntegralScalar → `LongNodePropertyValues`
    - FloatingPointScalar → `DoubleNodePropertyValues`
    - Arrays → Array-specific traits

### 3. Other Value Macros (legacy)

- **`scalar_macros.rs`** - GdsValue scalar implementations (String, Long, etc.)
- **`array_macros.rs`** - GdsValue array implementations
- **`binary_macros.rs`** - Binary values with MIME types
- **`factory_macros.rs`** - PrimitiveValues factory
- **`primitive_generator.rs`** - Unified generation

## Usage Patterns

### For Property Values (Current System)

```rust
// In default_node_property_values.rs:
use crate::{generate_all_node_adapters, generate_all_node_array_adapters};

generate_all_node_adapters!();        // Generates all scalar adapters
generate_all_node_array_adapters!();  // Generates all array adapters

// Result: 20+ structs, all generic over Collections backend C:
// - DefaultLongNodePropertyValues<C>
// - DefaultDoubleNodePropertyValues<C>
// - DefaultLongArrayNodePropertyValues<C>
// - etc.
```

### Backend Selection

```rust
// Vec backend:
use crate::collections::backends::vec::VecLong;
let backend = VecLong::from(vec![1, 2, 3]);
let props = DefaultLongNodePropertyValues::from_collection(backend, 3);

// Huge backend (future):
use crate::collections::backends::huge::HugeLong;
let backend = HugeLong::with_capacity(1_000_000_000);
let props = DefaultLongNodePropertyValues::from_collection(backend, node_count);

// Arrow backend (future):
use crate::collections::backends::arrow::ArrowLong;
let backend = ArrowLong::from_arrow_array(arrow_array);
let props = DefaultLongNodePropertyValues::from_collection(backend, count);
```

## File Organization

```
gds/src/projection/codegen/
├── values/                      ← Value codegen (YOU ARE HERE)
│   ├── value_type_table.rs      ← MASTER CONTROLLER
│   ├── scalar_macros.rs         ← GdsValue scalars
│   ├── array_macros.rs          ← GdsValue arrays
│   ├── binary_macros.rs         ← Binary values
│   ├── factory_macros.rs        ← PrimitiveValues factory
│   ├── primitive_generator.rs   ← Unified generation
│   ├── mod.rs                   ← Re-exports
│   └── README.md                ← This file
│
├── property/                    ← Property-specific macros
│   ├── triadic_macros.rs        ← Universal adapters (uses value_type_table)
│   ├── property_values.rs       ← Trait implementation helpers
│   └── ...
│
├── eval/                        ← Evaluation macros
│   └── eval_macro.rs            ← Has its own eval_value_type_table!
│
└── mod.rs                       ← Re-exports all to crate::projection::codegen
```

## Adding New ValueTypes

To add a new ValueType:

1. **Add to ValueType enum** (`gds/src/types/value_type.rs`):
```rust
pub enum ValueType {
    // ... existing
    Decimal,  // New!
}
```

2. **Add to value_type_table** (`value_type_table.rs`):
```rust
$callback!(Decimal, rust_decimal::Decimal, OtherScalar, Decimal::ZERO);
```

3. **Implement Collections backend**:
```rust
// gds/src/collections/backends/vec/vec_decimal.rs
pub struct VecDecimal { data: Vec<Decimal> }
impl Collections<Decimal> for VecDecimal { /* ... */ }
impl PropertyValuesAdapter<Decimal> for VecDecimal {}
```

4. **Done!** All property values automatically support Decimal via the universal adapters.

## Philosophy

The value_type_table is the **Master Controller** because:

1. **Single Source of Truth**: All type definitions flow from here
2. **Consistency**: Every ValueType maps to exactly one Rust type
3. **Extensibility**: Adding types is trivial - update table, done
4. **Backend Agnostic**: Types work with Vec, Huge, Arrow automatically
5. **Schema Foundation**: Enables Arrow/Polars schema awareness

This is the bridge between **Schema** (Types) and **Storage** (Collections).

## Current Status

**✅ Complete:**
- ValueType table with 23 types (scalars + arrays)
- Universal adapter macros for Node/Relationship/Graph
- Node scalar adapters fully migrated and tested
- Collections trait system in place

**🚧 In Progress:**
- Node array adapters (generated but trait impls incomplete)
- Relationship adapters (macros ready, not yet applied)
- Graph adapters (macros ready, not yet applied)

**🔮 Future:**
- Arrow Collections backend
- Huge Collections backend
- Runtime backend selection via CollectionsConfig
- All 46 ValueTypes from the spec

