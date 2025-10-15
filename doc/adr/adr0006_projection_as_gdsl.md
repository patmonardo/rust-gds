# ADR 0006: Projection as GDSL — Eval Macro System

## Status

Accepted

## Context

We need a way to define property types for our PropertyGraph system that:

1. Supports unlimited types (primitives, arrays, lists, UDTs, even audio files)
2. Generates both storage adapters (Gross/PropertyValues) and runtime drivers (Subtle/PrimitiveValues)
3. Centralizes conversion policy between storage and runtime representations
4. Avoids boilerplate and maintains a single source of truth
5. Is not a database but a general-purpose data science pipeline processor

The original implementation scattered property logic across multiple modules:

- `src/types/` had ValueType enums and PropertySchema
- `src/values/` had PrimitiveValues and conversion helpers
- `src/types/properties/` had PropertyValues storage adapters
- Conversion logic was duplicated in multiple places

## Decision

We have moved the Form Processor to `src/projection/` and created the **Eval Macro System**:

### Architecture: Triadic Projection Model

```
Noumenal (Pure Form)
    ↓
    value_type_table! macro
    ↓
Transcendental (Pure Nama) ←→ Form Processor + Functors
    ↓                              ↓
Phenomenal (Gross)         Phenomenal (Subtle)
PropertyValues             PrimitiveValues
```

### Key Components

1. **form_processor.rs** (Transcendental/Pure Nama)

   - Canonical policy surface for conversions
   - Runtime PropertyDescriptor registry
   - Safe boundary helpers (u64→usize, widening casts)

2. **property_descriptor.rs** (Compile-time Schema)

   - PropertyDescriptor: rich metadata for property types
   - StorageHint: guides backend selection
   - StructDescriptor/FieldDescriptor: for UDTs

3. **functors.rs** (Categorical Mappings)

   - SubtleToGross: runtime → storage projection
   - GrossToSubtle: storage → runtime projection
   - GrossSubtleFunctor: bidirectional conversion

4. **eval_macro.rs** (The Master Projector)

   - `value_type_table!` macro DSL
   - Generates descriptors, registration, functors, tests per entry
   - Single source of truth for all property types

5. **value_type_table.rs** (Prototype)
   - Demonstrates the macro with Long, Double, StringProp, LongArray
   - Includes integration tests

### Benefits

1. **Single Source of Truth**: One macro invocation defines compile-time schema, runtime validation, and both storage/runtime implementations.

2. **Unlimited Type Support**: No artificial restrictions. Properties can be:

   - Primitives (i64, f64, String, etc.)
   - Arrays/Lists (Vec<T>)
   - Structs/UDTs (user-defined types with fields)
   - Nested types (List<Struct<...>>)
   - Arbitrary payloads (audio, embeddings, binary blobs)

3. **Centralized Policy**: Form Processor enforces all conversion rules. Generated code and macros call stable helpers rather than inlining casts.

4. **Safe by Default**:

   - Checked u64→usize conversions (overflow protection)
   - Only safe widening (i32→i64, f32→f64)
   - Explicit coercion for everything else
   - Fail-fast on type mismatches

5. **Test-Driven Generation**: Every generated module includes unit tests for descriptors, registration, and functors.

6. **Zero-Copy Ready**: Functor abstraction allows backends (Arrow, mmap) to implement zero-copy views without changing call sites.

7. **Separation of Concerns**:
   - **projection/**: compile-time schema projection (macros, descriptors)
   - **values/**: runtime Subtle world (GdsValue, PrimitiveValues)
   - **types/properties/**: runtime Gross world (PropertyValues, storage)

### Why Projection?

Projection is the right home because:

- It already handles property mappings and element projections
- The native factory lives here
- It's the conceptual "kernel bridge" between schema and runtime
- GDSL (Graph DSL) will project into kernel space through the native factory

### Philosophical Alignment

This design directly implements Yoga Sutra 3.44:

- **Sthūla (Gross)**: PropertyValues storage
- **Svarūpa (Essential Nature)**: PropertyDescriptor schema
- **Sūkṣma (Subtle)**: PrimitiveValues runtime
- **Anvaya (Interconnectedness)**: Functors
- **Arthavattva (Purpose)**: Form Processor policy
- **Saṃyamāt (Through focused meditation)**: Eval Macro projection

## Consequences

### Positive

1. Adding new property types requires only a macro entry (3-5 lines).
2. All conversion logic centralized in Form Processor.
3. Generated code is testable and documented.
4. No artificial restrictions on property types.
5. Clear path to advanced features (UDTs, zero-copy views, typed accessors).
6. PropertyGraph is now a **General FormBase** not a database.

### Negative

1. More indirection: property type → descriptor → functor → backend.
2. Generated code may be harder to debug (macro expansion).
3. Compile-time cost of macro expansion (mitigated by incremental compilation).

### Neutral

1. Form Processor moved from `values/` to `projection/` (values now focuses purely on Subtle runtime).
2. PropertyDescriptor is compile-time-only (runtime uses PropertySchema).
3. Functor implementations currently stub (identity); need real conversions next.

## Migration Path

1. ✅ Move form_processor to projection
2. ✅ Create property_descriptor types
3. ✅ Create functor traits
4. ✅ Implement eval_macro DSL
5. ✅ Add prototype value_type_table with 4 types
6. ✅ Wire everything via projection/mod.rs
7. ✅ Add comprehensive tests
8. 🔄 Implement real functor conversions (type checking, widening)
9. 🔄 Generate typed accessors (get_long, get_string, etc.)
10. 🔄 Wire descriptors to ArrayBackend selection
11. 🔄 Add UDT support (struct layouts)
12. 🔄 Integrate with Pregel (NodeValue conversions)

## Related ADRs

- ADR 0002: Triadic GraphStore Architecture
- ADR 0005: Values System Architecture
- (Future) ADR 0007: GDSL Language Specification

## References

- `doc/EVAL_MACRO_SYSTEM.md`: Complete system documentation
- Yoga Sutra 3.44: Philosophical foundation
- Category Theory: Functors as structure-preserving mappings

## Notes

This is a **massive speculative bubble** that sets the foundation for unlimited expressiveness in our PropertyGraph system. We are not building a database — we are building a General FormBase: a macro-driven property graph processor for data science and ML pipelines where properties can be anything.

The key insight: **Projection IS the GDSL runtime** — the compile-time DSL that projects schema into both storage (Gross) and runtime (Subtle) worlds through a single, testable, documented macro invocation.
