# Projection Module — The GDSL Runtime

This module is now the home of the **Eval Macro System** — a compile-time DSL that projects PropertyGraph schemas into both storage (Gross) and runtime (Subtle) worlds.

## Quick Start

### Define Property Types

```rust
use crate::projection::property_descriptor::StorageHint;
use crate::types::ValueType;

value_type_table! {
    MyProperty {
        id: 100,
        value_type: ValueType::Long,
        storage_hint: StorageHint::FixedWidth,
        rust_type: i64,
    },
}
```

### Initialize the System

```rust
use crate::projection::value_type_table;

fn init() {
    // Register all defined property types
    let count = value_type_table::registry::register_all();
    println!("Registered {} property types", count);
}
```

### Use Functors for Conversion

```rust
use crate::projection::functors::{GrossToSubtle, SubtleToGross};
use crate::projection::value_type_table::Long;

fn convert() {
    let functor = Long::Functor;

    // Runtime → Storage
    let stored = functor.project_to_storage(runtime_value)?;

    // Storage → Runtime
    let runtime = functor.project_to_runtime(stored_value)?;
}
```

## Module Structure

```
projection/
├── eval_macro.rs           ← value_type_table! macro DSL
├── form_processor.rs       ← Policy surface (u64→usize, widening, registry)
├── functors.rs             ← Gross ↔ Subtle conversion traits
├── property_descriptor.rs  ← Compile-time schema types
├── value_type_table.rs     ← Prototype with Long, Double, String, Array
└── ... (existing projection code)
```

## Key Concepts

### Triadic Model

1. **Noumenal (Pure Form)**: The `value_type_table!` macro — compile-time schema
2. **Transcendental (Pure Nama)**: Form Processor + Functors — policy & conversion
3. **Phenomenal**: Gross (PropertyValues storage) + Subtle (PrimitiveValues runtime)

### Form Processor

The canonical policy surface:

- `checked_u64_to_usize(id)` — safe conversion with overflow protection
- `widen_i32_to_i64(v)`, `widen_f32_to_f64(v)` — safe widening
- `register_property_descriptor(desc)` — runtime registration
- `get_property_descriptor(id)` — descriptor lookup

### Functors

Category-theoretic mappings:

- `SubtleToGross` — runtime → storage projection
- `GrossToSubtle` — storage → runtime projection
- `GrossSubtleFunctor` — bidirectional conversion

### Property Descriptors

Compile-time schema with rich metadata:

- `id: PropertyId` — unique identifier
- `name: String` — property name
- `value_type: ValueType` — type information
- `nullable: bool` — nullability
- `storage_hint: StorageHint` — backend selection guide

## Storage Hints

- `FixedWidth` → HugeArray<T> (i64, f64, etc.)
- `VariableLength` → offsets + heap (String, Vec<u8>)
- `ListAsOffsets` → offsets + child column (Vec<T>)
- `ColumnarStruct` → column-per-field (UDTs)
- `SerializedRow` → row-wise bytes (complex UDTs)

## Generated Code

For each property type entry, the macro generates:

1. A module (`mod Long`, `mod StringProp`, etc.)
2. `DESCRIPTOR` static with metadata
3. `register()` function for runtime registration
4. `Functor` struct implementing conversions
5. Complete unit tests

Plus a `registry` module with:

- `register_all()` — batch registration
- `all_property_ids()` — list all IDs

## Examples

See:

- `value_type_table.rs` — prototype with 4 types
- `tests/` in each generated module
- `doc/EVAL_MACRO_SYSTEM.md` — comprehensive guide

## Design Philosophy

Based on Yoga Sutra 3.44:

> "By performing samyama on the gross form, essential nature, subtle essence,
> interconnectedness, and purpose of objects, mastery over the elements is attained."

- **Sthūla (Gross)** = PropertyValues storage
- **Svarūpa (Essential Nature)** = PropertyDescriptor schema
- **Sūkṣma (Subtle)** = PrimitiveValues runtime
- **Anvaya (Interconnectedness)** = Functors
- **Arthavattva (Purpose)** = Form Processor
- **Saṃyamāt (Focused projection)** = Eval Macro

## Benefits

1. **Single Source of Truth**: One macro entry → complete implementation
2. **Unlimited Types**: No artificial restrictions (primitives, arrays, UDTs, audio, etc.)
3. **Safe by Default**: Checked conversions, fail-fast on errors
4. **Test-Driven**: Every type includes generated tests
5. **Zero-Copy Ready**: Functor abstraction supports efficient backends

## Next Steps

1. Implement real functor conversions (type checking, widening logic)
2. Generate typed accessors (`get_long`, `get_string`, etc.)
3. Wire descriptors to ArrayBackend selection
4. Add UDT support (struct layouts)
5. Integrate with Pregel (NodeValue conversions)

## Related Documentation

- `doc/EVAL_MACRO_SYSTEM.md` — Complete system documentation
- `doc/adr0006_projection_as_gdsl.md` — Architectural decision record
- `doc/EVAL_MACRO_IMPLEMENTATION_SUMMARY.md` — Implementation summary
- `doc/EVAL_MACRO_VISUAL_OVERVIEW.md` — Visual diagrams

---

**This is not a database — it's a General FormBase**: a macro-driven property graph processor for data science and ML pipelines where properties can be anything! 🚀
