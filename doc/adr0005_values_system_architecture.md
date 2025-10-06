# ADR 0005: Values System Architecture and Second Macro System

**Status**: Draft  
**Date**: 2025-10-06  
**Context**: Values system cleanup revealed the architecture for individual value manipulation

## Context

During the PropertyValue enum removal (ADR 0003), we discovered an orphaned `src/values/` module that represented early attempts at individual value handling. After integration and cleanup, the architecture of the **Second Macro System** has emerged clearly.

## The Two Macro Systems

### First Macro System: PropertyValues (Columnar)

**Purpose**: Bulk operations on columns of data at GraphStore/PropertyStore level

**Location**: `src/types/properties/property_values.rs`

**Macros**:

- `property_values_impl!` (5 variants)
- `node_*_property_values_impl!` (domain-specific helpers)

**Traits**:

```rust
PropertyValues {
    fn value_type(&self) -> ValueType;
    fn element_count(&self) -> usize;
}

NodePropertyValues: PropertyValues {
    fn long_values(&self) -> Option<&[i64]>;
    fn double_values(&self) -> Option<&[f64]>;
    // ... returns slices/references to columns
}
```

**Storage**: Vec<T> (current PureGraphStore), Arrow2 arrays (future CoreGraphStore)

**Use Case**: Store and retrieve entire property columns efficiently

---

### Second Macro System: GdsValue (Individual Values)

**Purpose**: Individual value extraction and manipulation at Projection/Graph level

**Location**: `src/values/`

**Macros**: To be implemented (this ADR)

**Traits**:

```rust
GdsValue {
    fn value_type(&self) -> ValueType;
    fn as_object(&self) -> JsonValue;
}

// Value Accessors - the bridge to actual data
IntegralValue: GdsValue {
    fn long_value(&self) -> i64;  // Extract the value!
}

FloatingPointValue: GdsValue {
    fn double_value(&self) -> f64;  // Extract the value!
}

// Array Accessors
IntegralArray: Array {
    fn long_value(&self, idx: usize) -> i64;
    fn long_array_value(&self) -> Vec<i64>;
}

FloatingPointArray: Array {
    fn double_value(&self, idx: usize) -> f64;
    fn double_array_value(&self) -> Vec<f64>;
}
```

**Factory**: `PrimitiveValues::of()` / `create()`

**Implementations**: `Default*` structs (DefaultLongValue, DefaultLongArray, etc.)

**Use Case**: When traversing graphs via cursors/iterators, extract individual property values

---

## Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│  Algorithm Layer                                         │
│  (operates on Graph projections)                         │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│  Projection/Graph Layer                                  │
│  • Cursors, Iterators                                    │
│  • RelationshipCursor::property() → f64                  │
│  • Extract individual values via GdsValue                │  ← Second Macro System
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│  GraphStore/PropertyStore Layer                          │
│  • PropertyValues (columns)                              │  ← First Macro System
│  • Bulk operations on slices                             │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│  Storage Backend                                         │
│  • Pure: Vec<T>                                          │
│  • Core: Arrow2 arrays                                   │
│  • Warm: mmap'd Arrow2                                   │
│  • Cold: compressed Arrow2                               │
└─────────────────────────────────────────────────────────┘
```

## Current State

### Clean Structure

```
src/values/
├── mod.rs                      (module root)
├── primitive_values.rs         (PrimitiveValues factory - entry point)
├── traits/
│   ├── mod.rs
│   └── gds_value.rs           (trait hierarchy)
└── impls/
    ├── mod.rs
    ├── array_equals.rs        (cross-type equality)
    └── gds_value_impls.rs     (Default* implementations)
```

### Trait Hierarchy

```
GdsValue (core)
├── Array
│   ├── IntegralArray
│   │   └── LongArray (marker)
│   └── FloatingPointArray
│       ├── FloatArray (marker)
│       └── DoubleArray (marker)
├── IntegralValue (scalar accessor)
└── FloatingPointValue (scalar accessor)
```

### Current Implementations (Hand-written)

- `DefaultLongValue` (i64 scalar)
- `DefaultFloatingPointValue` (f64 scalar)
- `DefaultLongArray` (Vec<i64>)
- `DefaultIntLongArray` (Vec<i32> → i64)
- `DefaultShortLongArray` (Vec<i16> → i64)
- `DefaultByteLongArray` (Vec<u8> → i64)
- `DefaultDoubleArray` (Vec<f64>)
- `DefaultFloatArray` (Vec<f32> → f64)
- `GdsNoValue` (null singleton)

## Decision: Second Macro System Requirements

### What Needs Generation

**1. Default\* Implementations**
Current implementations are repetitive boilerplate:

```rust
pub struct DefaultLongArray {
    data: Arc<Vec<i64>>,
}
impl DefaultLongArray {
    pub fn new(data: Vec<i64>) -> Self { ... }
}
impl IntegralArray for DefaultLongArray { ... }
impl Array for DefaultLongArray { ... }
impl GdsValue for DefaultLongArray { ... }
```

**Pattern**: Same structure for all array types (Long, Double, Float)

**2. PrimitiveValues Factory Enhancement**
Currently hand-written match arms. Could be generated for all 45 ValueType variants.

**3. Type Conversions**

- Int/Short/Byte → Long conversions
- Float → Double conversions
- Handle optional/nullable values
- Default value integration

### Macro Goals

**`gds_value_impl!` macro** should generate:

1. Struct definition with `Arc<Vec<T>>` backing
2. `new()` constructor
3. Trait implementations (GdsValue, Array, Integral/FloatingPointArray)
4. Type conversions where needed

**`gds_value_factory!` macro** should generate:

1. PrimitiveValues::of() match arms for all types
2. Smart type inference (int vs float, scalar vs array)
3. Integration with DefaultValue system

## Consequences

### Positive

- **Clean separation**: PropertyValues (columns) vs GdsValue (individual)
- **Type safety**: ValueType enum drives both systems
- **Flexibility**: Can add new value types by extending macros
- **Performance**: Arc<Vec<T>> allows cheap cloning for cursors
- **Future-proof**: Ready for Arrow2 backing (individual values from IPC)

### Challenges

- **Macro complexity**: Need careful design to avoid First Macro System mistakes
- **Type conversions**: Must handle widening (i32→i64, f32→f64) correctly
- **Null handling**: GdsNoValue integration with nullable types
- **String types**: Not yet supported (TODO)
- **Map types**: 45 ValueType variants, only 8 implemented so far

### Integration Points

- **Projection Layer**: DefaultGraph needs GdsValue for property extraction
- **Cursor System**: RelationshipCursor uses f64, but could use GdsValue
- **PropertyValues Bridge**: Extract from columns → individual GdsValue instances
- **DefaultValue**: Schema-level defaults → runtime GdsValue instances

## Next Steps

1. **Design gds_value_impl! macro** for Default\* struct generation
2. **Create macro_rules module** at `src/values/macros.rs`
3. **Generate remaining implementations** (String, Map types)
4. **Enhance PrimitiveValues factory** with macro-generated match arms
5. **Review Projection layer** (DefaultGraph) for GdsValue integration
6. **Write tests** for value extraction and type conversions

## References

- ADR 0001: Property Graph Store Design
- ADR 0003: Node Property Value Contract (PropertyValue enum removal)
- ADR 0004: Property Cursors (First Macro System)
- TS/Java GDS: GdsValue hierarchy
- Arrow2 documentation: Individual value extraction from IPC

---

**Key Insight**: The Values system is the **Second Macro System** - it provides individual value access for the Projection layer, while the PropertyValues system (First Macro System) provides columnar access for the GraphStore layer. Both systems are driven by the same ValueType enum and will eventually support all 45 variants.
