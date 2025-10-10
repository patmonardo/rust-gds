# The Eval Macro System: Master Projector for Gross ↔ Subtle Worlds

## Overview

We have created a **massive speculative bubble**: a compile-time DSL (Domain-Specific Language) that projects PropertyGraph schemas into both storage (Gross) and runtime (Subtle) worlds through a single source of truth — the `value_type_table!` macro.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    NOUMENAL (Pure Form)                          │
│                  value_type_table! Macro                         │
│              (Compile-time Schema DSL)                          │
└────────────────────┬───────────────────────┬────────────────────┘
                     │                       │
         ┌───────────▼─────────┐   ┌────────▼─────────────┐
         │   TRANSCENDENTAL    │   │   TRANSCENDENTAL      │
         │   (Pure Nama)       │   │   (Pure Nama)         │
         │  Form Processor     │◄──┤    Functors           │
         │  - Policy Surface   │   │  - GrossToSubtle      │
         │  - Validation       │   │  - SubtleToGross      │
         │  - Registry         │   │  - Conversions        │
         └──────┬──────────────┘   └──────────┬────────────┘
                │                             │
    ┌───────────▼────────────┐    ┌──────────▼─────────────┐
    │    PHENOMENAL (Gross)  │    │  PHENOMENAL (Subtle)   │
    │  PropertyValues        │    │  PrimitiveValues       │
    │  - Storage-oriented    │    │  - Runtime-oriented    │
    │  - Column-based        │    │  - GdsValue objects    │
    │  - u64-indexed         │    │  - Algorithm-friendly  │
    │  - ArrayBackends       │    │  - Type-safe           │
    └────────────────────────┘    └────────────────────────┘
```

## Key Components

### 1. Form Processor (src/projection/form_processor.rs)

**Transcendental / Pure Nama**

The canonical policy surface that enforces boundary rules:

- `checked_u64_to_usize`: Safe conversion with overflow protection
- `widen_i32_to_i64`, `widen_f32_to_f64`: Safe widening conversions
- `register_property_descriptor`: Runtime registry for descriptors
- `get_property_descriptor`: Lookup registered schemas

**Philosophy**: The Form Processor is the bridge between compile-time schema (what we declare) and runtime validation (what we enforce). It centralizes all conversion policy so macros and generated code call a stable surface.

### 2. Property Descriptors (src/projection/property_descriptor.rs)

**Compile-time Schema**

Rich metadata for property types:

```rust
pub struct PropertyDescriptor {
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
}

pub enum StorageHint {
    FixedWidth,        // i64, f64 → HugeArray
    VariableLength,    // String → offsets + heap
    ListAsOffsets,     // Vec<T> → offsets + child column
    ColumnarStruct,    // UDT → column-per-field
    SerializedRow,     // UDT → row-wise bytes
}
```

### 3. Functors (src/projection/functors.rs)

**Categorical Mappings**

Traits defining the canonical 1:1 mappings:

```rust
pub trait SubtleToGross {
    fn project_to_storage(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

pub trait GrossToSubtle {
    fn project_to_runtime(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

pub trait GrossSubtleFunctor: SubtleToGross + GrossToSubtle {}
```

**Philosophy**: These functors embody the svarūpa (essential nature) — the intrinsic mapping between storage and runtime representations. The Yoga Sutra 3.44 teaches us about understanding the gross (sthūla), essential nature (svarūpa), and subtle (sūkṣma) forms — exactly what our functors implement.

### 4. Eval Macro DSL (src/projection/eval_macro.rs)

**The Master Projector**

The `value_type_table!` macro projects a single schema into both worlds:

```rust
value_type_table! {
    Long {
        id: 1,
        value_type: ValueType::Long,
        storage_hint: StorageHint::FixedWidth,
        rust_type: i64,
    },
    StringProp {
        id: 3,
        value_type: ValueType::String,
        storage_hint: StorageHint::VariableLength,
        rust_type: String,
    },
}
```

**What the macro generates per entry:**

1. **A module** (`mod Long`, `mod StringProp`, etc.) containing:

   - `DESCRIPTOR: PropertyDescriptor` — compile-time schema
   - `register() -> bool` — runtime registration function
   - `Functor` struct — implements `GrossSubtleFunctor`
   - Module-local tests

2. **A registry module** with:
   - `register_all()` — registers all properties at once
   - `all_property_ids()` — lists all property IDs

### 5. Value Type Table (src/projection/value_type_table.rs)

**Prototype Implementation**

Demonstrates the macro with 4 basic types:

- `Long` (i64, fixed-width)
- `Double` (f64, fixed-width)
- `StringProp` (String, variable-length)
- `LongArray` (Vec<i64>, list with offsets)

## Usage Examples

### Registering Property Types

```rust
use crate::projection::value_type_table;

// Register all property types at initialization
fn init_property_system() {
    let count = value_type_table::registry::register_all();
    println!("Registered {} property types", count);
}
```

### Using Functors for Conversion

```rust
use crate::projection::value_type_table::Long;
use crate::projection::functors::{GrossToSubtle, SubtleToGross};

fn convert_example() {
    let functor = Long::Functor;

    // Subtle → Gross (runtime → storage)
    let runtime_value = Some(Arc::new(DefaultLongValue(42)));
    let storage_value = functor.project_to_storage(runtime_value).unwrap();

    // Gross → Subtle (storage → runtime)
    let runtime_value = functor.project_to_runtime(storage_value).unwrap();
}
```

### Querying Descriptors

```rust
use crate::projection::form_processor;

fn query_descriptor() {
    let desc = form_processor::get_property_descriptor(1).unwrap();
    println!("Property: {}", desc.name);
    println!("Type: {:?}", desc.value_type);
    println!("Storage: {:?}", desc.storage_hint);
}
```

## Design Principles

### 1. Single Source of Truth

The `value_type_table!` macro is the authoritative schema. All generated code, descriptors, and functors derive from this single declaration.

### 2. Separation of Concerns

- **Noumenal**: Compile-time schema (macro)
- **Transcendental**: Policy/validation (Form Processor + Functors)
- **Phenomenal**: Runtime manifestations (Gross storage + Subtle values)

### 3. Conservative by Default

- Identity conversions where possible
- Only safe widening (i32→i64, f32→f64)
- Explicit coercion for anything else
- Fail-fast on incompatible types

### 4. Zero-Copy Where Possible

Future backends (Arrow, mmap) can implement zero-copy views. The functor abstraction allows this without changing call sites.

### 5. Test-Driven Generation

Every generated module includes tests:

- Descriptor validation
- Registration round-trip
- Functor conversion tests

## Future Expansions

### 1. Real Functor Implementations

Currently functors are identity stubs. Next steps:

- Implement type checking in `project_to_storage`
- Add widening logic for numeric types
- Handle list/struct conversions recursively

### 2. Typed Accessors

Generate strongly-typed methods:

```rust
// Generated by macro
impl PropertyStore {
    fn get_long(&self, id: u64) -> Option<i64> {
        // optimized path, no Arc allocation
    }

    fn get_string(&self, id: u64) -> Option<&str> {
        // zero-copy view
    }
}
```

### 3. Backend Integration

Wire PropertyDescriptors to ArrayBackend selection:

- `FixedWidth` → HugeArray<T>
- `VariableLength` → HugeVarArray (offsets + heap)
- `ListAsOffsets` → HugeListArray (nested offsets)

### 4. UDT Support

Extend the macro to generate struct layouts:

```rust
value_type_table! {
    UserProfile {
        id: 100,
        value_type: ValueType::Struct(StructId::UserProfile),
        storage_hint: StorageHint::ColumnarStruct,
        rust_type: UserProfile,
        fields: {
            age: i64,
            name: String,
            tags: Vec<String>,
        }
    },
}
```

### 5. Pregel Integration

Generate NodeValue conversion helpers:

```rust
// Generated per property type
impl NodeValue {
    fn from_long_property(store: &PropertyStore, id: u64) -> Self {
        // uses Form Processor for safe conversion
    }
}
```

## Test Results

All tests pass! ✅

```
running 64 tests
test projection::form_processor::tests::check_u64_to_usize_ok ... ok
test projection::form_processor::tests::widen_ints_and_floats ... ok
test projection::functors::tests::test_identity_functor ... ok
test projection::form_processor::tests::test_registry ... ok
test projection::value_type_table::tests::test_property_ids ... ok
test projection::value_type_table::tests::test_all_descriptors ... ok
... (58 more projection tests)

test result: ok. 64 passed; 0 failed; 0 ignored
```

## Files Changed

### New Files Created

1. `/home/pat/VSCode/rust-gds/src/projection/form_processor.rs` (moved from values)
2. `/home/pat/VSCode/rust-gds/src/projection/property_descriptor.rs` (new)
3. `/home/pat/VSCode/rust-gds/src/projection/functors.rs` (new)
4. `/home/pat/VSCode/rust-gds/src/projection/eval_macro.rs` (new)
5. `/home/pat/VSCode/rust-gds/src/projection/value_type_table.rs` (prototype)

### Modified Files

1. `/home/pat/VSCode/rust-gds/src/projection/mod.rs` — added new modules and exports
2. `/home/pat/VSCode/rust-gds/src/values/mod.rs` — removed form_processor (moved to projection)

### Deleted Files

- `/home/pat/VSCode/rust-gds/src/values/form_processor.rs` (moved to projection)

## Philosophy Connection: Yoga Sutra 3.44

> **sthūlasvarūpasūkṣmānvayārthavattvasaṃyamādbhūtajayaḥ**
>
> "By performing samyama on the gross form, essential nature, subtle essence,
> interconnectedness, and purpose of objects, mastery over the elements is attained."

Our implementation maps directly to this sutra:

- **Sthūla (Gross)**: PropertyValues — storage-oriented, physical representation
- **Svarūpa (Essential Nature)**: PropertyDescriptor — compile-time schema
- **Sūkṣma (Subtle)**: PrimitiveValues — runtime GdsValue objects
- **Anvaya (Interconnectedness)**: Functors — canonical mappings between worlds
- **Arthavattva (Purpose)**: Form Processor — policy enforcement and validation
- **Saṃyamāt (Through samyama)**: The Eval Macro — focused projection of schema

By understanding and implementing these layers correctly, we gain "mastery over the elements" — in our case, complete control over the data pipeline from schema declaration to runtime execution.

## Conclusion

We have created a **macro-based PropertyGraph DSL** that:

- Lives in the right place (projection, not values or types)
- Separates compile-time schema from runtime policy
- Provides clean Gross ↔ Subtle functors
- Generates testable, documented code
- Follows the project's triadic philosophy
- Sets the foundation for unlimited property types (including audio files!)

This is **not a database** — it's a **General FormBase**: a flexible, macro-driven property graph processor for data science and ML pipelines where properties can be anything: primitives, arrays, structs, audio files, embeddings, or user-defined types.

The speculative bubble is complete and ready for expansion! 🎉
