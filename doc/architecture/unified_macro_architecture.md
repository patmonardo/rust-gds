# Unified Macro Architecture for Rust-GDS

**Date**: October 10, 2025  
**Status**: Architecture design - resolving three core issues  
**Goal**: Complete macro-based property system with precise backend abstraction

---

## The Three Problems We Must Solve Together

### Problem 1: usize vs u64 Index Type Chaos

**Current state** (inconsistent):

```rust
// NodePropertyValues uses u64:
trait NodePropertyValues {
    fn long_value(&self, node_id: u64) -> i64;  // ← u64
}

// But arrays use usize:
impl DefaultLongNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values[node_id as usize]  // ← cast to usize!
    }
}

// HugeArrays use usize:
impl HugeLongArray {
    pub fn get(&self, index: usize) -> i64 {  // ← usize
        // ...
    }
}

// But element_count returns usize:
trait PropertyValues {
    fn element_count(&self) -> usize;  // ← usize
}
```

**The hack**: Everywhere we do `node_id as usize` casting.

**Why this exists**: Java GDS uses `long` (i64) for node IDs, but we use Rust arrays which need `usize` indexing.

---

### Problem 2: Java GDS Relationship Property Limitation

**Java GDS reality** (from codebase):

```java
// RelationshipPropertyValues only supports Long and Double!
interface RelationshipPropertyValues {
    double doubleValue(long relationshipId);  // Only these two!
    long longValue(long relationshipId);
}

// NodePropertyValues supports MORE types:
interface NodePropertyValues {
    double doubleValue(long nodeId);
    long longValue(long nodeId);
    double[] doubleArrayValue(long nodeId);    // Arrays supported
    float[] floatArrayValue(long nodeId);      // for nodes!
    long[] longArrayValue(long nodeId);
    // ... more types
}
```

**Why**: Neo4j database **relationships can only have Long/Double properties** in their storage layer.

**Our code** (mimics this):

```rust
// RelationshipPropertyValues trait - only Long/Double!
pub trait RelationshipPropertyValues: PropertyValues {
    fn double_value(&self, rel_index: u64) -> PropertyValuesResult<f64>;
    fn long_value(&self, rel_index: u64) -> PropertyValuesResult<i64>;
    // That's it! No arrays, no other types.
}
```

**Question**: Do we keep this limitation or go beyond Java GDS?

---

### Problem 3: Two Macro Layers Without Unification

**Current state** (separate macro systems):

**Layer 1: HugeArray macros** (`src/collections/huge_array/`):

```rust
huge_primitive_array! {
    HugeLongArray, SingleHugeLongArray, PagedHugeLongArray,
    i64, "Long", "..."
}
// Generates: get(), set(), fill(), binary_search(), cursor support
```

**Layer 2: PropertyValues macros** (`src/types/properties/`):

```rust
property_values_impl!(DefaultLongNodePropertyValues, Long);
node_long_property_values_impl!(DefaultLongNodePropertyValues);
// Generates: value_type(), element_count(), long_value(), conversions
```

**Problem**: These macros don't talk to each other!

- PropertyValues macros assume Vec storage
- HugeArray macros don't know about PropertyValues trait
- No unified backend abstraction

---

## The Unified Solution: Three-Layer Macro Architecture

### Layer 1: Backend Abstraction (NEW)

**Define the backend trait that ALL storage implements**:

```rust
/// Core backend trait - this is what unifies everything
pub trait ArrayBackend<T>: Send + Sync + std::fmt::Debug {
    /// Index type - allows flexibility (usize for local, u64 for distributed)
    type Index: Copy + Into<usize> + From<usize>;

    /// Get value at index
    fn get(&self, index: Self::Index) -> T;

    /// Set value at index
    fn set(&mut self, index: Self::Index, value: T);

    /// Number of elements
    fn len(&self) -> usize;

    /// Check if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserve additional capacity
    fn reserve(&mut self, additional: usize);

    /// Current capacity
    fn capacity(&self) -> usize;
}
```

**Key insight**: `type Index` allows us to use **either usize or u64** depending on backend!

---

### Layer 2: Backend Implementations (Macro-Generated)

**Generate backends for different storage strategies**:

```rust
/// Macro to generate HugeArray backend
macro_rules! huge_array_backend {
    ($name:ident, $element:ty) => {
        pub struct $name {
            data: HugeArrayStorage<$element>,  // Single or Paged
        }

        impl ArrayBackend<$element> for $name {
            type Index = usize;  // ← HugeArray uses usize locally

            fn get(&self, index: usize) -> $element {
                self.data.get(index)
            }

            fn set(&mut self, index: usize, value: $element) {
                self.data.set(index, value)
            }

            fn len(&self) -> usize {
                self.data.len()
            }

            fn capacity(&self) -> usize {
                self.data.capacity()
            }

            fn reserve(&mut self, additional: usize) {
                self.data.reserve(additional)
            }
        }

        // HugeArray-specific methods (cursors, binary_search, etc.)
        impl $name {
            pub fn new(size: usize) -> Self { /* ... */ }
            pub fn fill(&mut self, value: $element) { /* ... */ }
            pub fn cursor(&self) -> HugeCursor<$element> { /* ... */ }
            // ... all the HugeArray-specific functionality
        }
    };
}

/// Macro to generate Arrow backend
macro_rules! arrow_array_backend {
    ($name:ident, $element:ty, $arrow_type:ty) => {
        pub struct $name {
            data: Arc<$arrow_type>,
        }

        impl ArrayBackend<$element> for $name {
            type Index = usize;  // Arrow also uses usize

            fn get(&self, index: usize) -> $element {
                self.data.value(index)
            }

            // Arrow arrays are immutable, so set() panics or returns error
            fn set(&mut self, _index: usize, _value: $element) {
                panic!("Arrow arrays are immutable")
            }

            fn len(&self) -> usize {
                self.data.len()
            }

            // ... other methods
        }

        // Arrow-specific methods
        impl $name {
            pub fn from_arrow(arr: Arc<$arrow_type>) -> Self { /* ... */ }
            pub fn to_arrow(&self) -> Arc<$arrow_type> { /* ... */ }
            pub fn slice(&self, offset: usize, length: usize) -> Self { /* ... */ }
        }
    };
}

/// Macro to generate Sparse backend
macro_rules! sparse_array_backend {
    ($name:ident, $element:ty, $default:expr) => {
        pub struct $name {
            data: HashMap<usize, $element>,
            default: $element,
            len: usize,
        }

        impl ArrayBackend<$element> for $name {
            type Index = usize;

            fn get(&self, index: usize) -> $element {
                *self.data.get(&index).unwrap_or(&self.default)
            }

            fn set(&mut self, index: usize, value: $element) {
                if value != self.default {
                    self.data.insert(index, value);
                }
                if index >= self.len {
                    self.len = index + 1;
                }
            }

            fn len(&self) -> usize {
                self.len
            }

            // ... other methods
        }

        // Sparse-specific methods
        impl $name {
            pub fn with_default(default: $element) -> Self { /* ... */ }
            pub fn density(&self) -> f64 { /* ... */ }
            pub fn non_default_indices(&self) -> Vec<usize> { /* ... */ }
        }
    };
}
```

---

### Layer 3: PropertyValues Integration (Macro-Generated)

**Generate PropertyValues implementations that USE backends**:

```rust
/// Unified macro to generate PropertyValues with any backend
macro_rules! generate_property_values {
    (
        $struct_name:ident,      // e.g., DefaultLongNodePropertyValues
        $backend:ty,             // e.g., HugeLongArray or ArrowLongArray
        $element:ty,             // e.g., i64
        $value_type:ident,       // e.g., Long
        $accessor:ident,         // e.g., long_value
        $property_trait:ident    // e.g., LongNodePropertyValues
    ) => {
        pub struct $struct_name {
            values: $backend,  // ← Backend abstraction!
            node_count: usize,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                Self { values, node_count }
            }

            // Convenience constructor from Vec (converts to backend)
            pub fn from_vec(vec: Vec<$element>, node_count: usize) -> Self
            where
                $backend: From<Vec<$element>>
            {
                Self::new(vec.into(), node_count)
            }
        }

        // Implement base PropertyValues trait
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.node_count
            }
        }

        // Implement specific property trait
        impl $property_trait for $struct_name {
            fn $accessor(&self, node_id: u64) -> PropertyValuesResult<$element> {
                // ← Key: handle u64 → usize conversion here!
                let index = node_id as usize;
                if index < self.values.len() {
                    Ok(self.values.get(index))
                } else {
                    Err(PropertyValuesError::IndexOutOfBounds {
                        index: node_id,
                        size: self.values.len(),
                    })
                }
            }
        }

        // Generate all NodePropertyValues trait methods (conversions, etc.)
        node_property_values_impl!($struct_name, $element, $accessor);
    };
}
```

**Usage**:

```rust
// Generate with HugeArray backend:
generate_property_values!(
    DefaultLongNodePropertyValues,
    HugeLongArray,
    i64,
    Long,
    long_value,
    LongNodePropertyValues
);

// Generate with Arrow backend:
generate_property_values!(
    ArrowLongNodePropertyValues,
    ArrowLongArray,
    i64,
    Long,
    long_value,
    LongNodePropertyValues
);

// Generate with Sparse backend:
generate_property_values!(
    SparseLongNodePropertyValues,
    SparseLongArray,
    i64,
    Long,
    long_value,
    LongNodePropertyValues
);
```

---

## Resolving the u64/usize Issue

### Decision: Use usize Internally, Accept u64 at API Boundary

**Rationale**:

1. Rust arrays **require** usize indexing (cannot change)
2. Java GDS uses `long` (i64) for node IDs (API compatibility)
3. GraphStore may use u64 for distributed graphs in future

**Solution**: PropertyValues trait accepts u64, converts to usize internally:

```rust
// PUBLIC API: Uses u64 (Java GDS compatibility)
pub trait NodePropertyValues: PropertyValues {
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64>;
    //                            ^^^^ u64 for API
}

// INTERNAL IMPL: Converts to usize
impl LongNodePropertyValues for DefaultLongNodePropertyValues {
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        let index = node_id as usize;  // ← Explicit conversion at boundary
        if index < self.values.len() {
            Ok(self.values.get(index))  // ← usize for array access
        } else {
            Err(PropertyValuesError::IndexOutOfBounds {
                index: node_id,  // Report as u64 in error
                size: self.values.len(),
            })
        }
    }
}
```

**Benefit**: Conversion happens **once at the trait boundary**, not scattered everywhere.

---

## Resolving the Relationship Property Limitation

### Decision: Support All Types, But Mark as Extension

**Java GDS limitation**:

- Relationships: Only Long/Double
- Nodes: Long, Double, Arrays, etc.

**Rust-GDS approach**:

```rust
// Base trait (Java GDS compatible):
pub trait RelationshipPropertyValues: PropertyValues {
    fn long_value(&self, rel_index: u64) -> PropertyValuesResult<i64>;
    fn double_value(&self, rel_index: u64) -> PropertyValuesResult<f64>;
    // ← Only Long/Double (Neo4j limitation)
}

// Extended trait (Rust-GDS extension):
pub trait ExtendedRelationshipPropertyValues: RelationshipPropertyValues {
    fn long_array_value(&self, rel_index: u64) -> PropertyValuesResult<Vec<i64>>;
    fn double_array_value(&self, rel_index: u64) -> PropertyValuesResult<Vec<f64>>;
    fn float_array_value(&self, rel_index: u64) -> PropertyValuesResult<Vec<f32>>;
    // ← Additional types (not in Java GDS)
}
```

**Benefit**:

- Core API matches Java GDS (Long/Double only)
- Extensions available for advanced use cases
- Clear documentation of what's standard vs extension

---

## The Two-Layer Distinction (Graph vs GraphStore)

### GraphStore Layer (Storage)

**Concern**: Physical storage of graph data
**Uses**: HugeArrays, Arrow, Sparse backends
**Index type**: usize (local memory indexing)

```rust
// GraphStore layer - storage details
pub struct DefaultGraphStore {
    id_map: HugeLongArray,              // usize indexing
    adjacency: Vec<HugeLongArray>,      // usize indexing
    node_properties: PropertyStore,      // usize indexing internally
}
```

### Graph Layer (API)

**Concern**: Logical graph operations
**Uses**: u64 node/relationship IDs (API compatibility)
**Index type**: u64 (exposed to users, Java GDS compatible)

```rust
// Graph layer - user-facing API
pub trait Graph {
    fn degree(&self, node_id: u64) -> usize;  // ← u64 at API
    fn neighbors(&self, node_id: u64) -> NeighborIterator;
    fn node_property(&self, node_id: u64, key: &str) -> Option<Value>;
}
```

**Conversion happens at boundary**:

```rust
impl Graph for DefaultGraphStore {
    fn degree(&self, node_id: u64) -> usize {
        let index = node_id as usize;  // ← Conversion at boundary
        self.adjacency[index].len()     // ← usize internally
    }
}
```

---

## Complete Macro Hierarchy

```
┌─────────────────────────────────────────────────┐
│ Layer 3: PropertyValues (User-Facing)          │
│ - Implements PropertyValues trait              │
│ - Accepts u64 at API boundary                  │
│ - Converts to backend Index type               │
│ - Generated by: generate_property_values!      │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│ Layer 2: Array Backends (Storage Strategy)     │
│ - HugeArray (paged Vec)                        │
│ - Arrow (columnar)                             │
│ - Sparse (HashMap)                             │
│ - All implement ArrayBackend<T>                │
│ - Generated by: huge_array_backend!,           │
│   arrow_array_backend!, sparse_array_backend!  │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│ Layer 1: ArrayBackend Trait (Unification)      │
│ - trait ArrayBackend<T>                        │
│ - type Index (flexible: usize or u64)         │
│ - Core operations: get, set, len, reserve      │
│ - Backend-agnostic                             │
└─────────────────────────────────────────────────┘
```

---

## Implementation Roadmap

### Phase 1: Define ArrayBackend Trait (Today)

**Files to create**:

1. `src/collections/array_backend.rs` - Core trait definition
2. `src/collections/backends/mod.rs` - Backend module structure

**Tasks**:

- [ ] Define `ArrayBackend<T>` trait with `type Index`
- [ ] Document trait semantics and requirements
- [ ] Add basic tests

---

### Phase 2: Refactor HugeArray to Implement ArrayBackend (Today)

**Files to modify**:

1. `src/collections/huge_array/huge_long_array.rs`
2. `src/collections/huge_array/huge_double_array.rs`

**Tasks**:

- [ ] Implement `ArrayBackend<i64>` for `HugeLongArray`
- [ ] Implement `ArrayBackend<f64>` for `HugeDoubleArray`
- [ ] Keep existing HugeArray-specific methods
- [ ] Update tests

---

### Phase 3: Create Unified PropertyValues Macro (Tomorrow)

**Files to create**:

1. `src/types/properties/macros/unified.rs`

**Tasks**:

- [ ] Create `generate_property_values!` macro
- [ ] Handle u64 → usize conversion at boundary
- [ ] Support multiple backends via generic
- [ ] Generate all NodePropertyValues variants
- [ ] Generate RelationshipPropertyValues variants

---

### Phase 4: Generate All PropertyValues with New Macro (Tomorrow)

**Files to modify**:

1. `src/types/properties/node/impls/values/long.rs`
2. `src/types/properties/node/impls/values/double.rs`
3. (All other property value implementations)

**Tasks**:

- [ ] Replace hand-written impls with macro invocations
- [ ] Test all property types
- [ ] Verify API compatibility
- [ ] Update documentation

---

### Phase 5: Add Arrow Backend (Next Week)

**Files to create**:

1. `src/collections/backends/arrow.rs`

**Tasks**:

- [ ] Add `arrow2` dependency
- [ ] Create `arrow_array_backend!` macro
- [ ] Generate ArrowLongArray, ArrowDoubleArray
- [ ] Implement zero-copy conversions
- [ ] Benchmark vs HugeArray

---

## Benefits of Unified Architecture

1. ✅ **Single source of truth** for array operations (ArrayBackend trait)
2. ✅ **Backend flexibility** (HugeArray, Arrow, Sparse, etc.)
3. ✅ **Type consistency** (all backends use same interface)
4. ✅ **Clear u64/usize boundary** (conversion at API layer only)
5. ✅ **Macro-generated consistency** (impossible to have bugs from hand-cranking)
6. ✅ **Java GDS compatibility** (core API matches, extensions available)
7. ✅ **Future-proof** (can add new backends without changing PropertyValues)

---

## Open Questions

1. **Should ArrayBackend::Index be generic?**

   - Current: `type Index: Copy + Into<usize> + From<usize>`
   - Alternative: Just use `usize` everywhere, convert at PropertyValues boundary
   - **Recommendation**: Keep generic for future distributed backends

2. **Should we support ExtendedRelationshipPropertyValues immediately?**

   - Yes: More flexible than Java GDS
   - No: Stick to Java GDS compatibility first
   - **Recommendation**: Define trait but mark as experimental

3. **Arrow backend: mutable or immutable?**

   - Arrow arrays are typically immutable
   - Should `set()` return `Result` instead of panicking?
   - **Recommendation**: Return error, document immutability

4. **Should Sparse backend be included in Phase 1?**
   - Pro: Useful for properties with many defaults
   - Con: Adds complexity
   - **Recommendation**: Phase 2, after HugeArray + Arrow working

---

## Next Action

**Which path**?

**Option A**: Define ArrayBackend trait first (foundation)
**Option B**: Refactor one PropertyValues type as proof-of-concept
**Option C**: Create complete macro system first (top-down)

**My recommendation**: **Option A** - Define ArrayBackend trait precisely, then build up from there.

Shall we start with the ArrayBackend trait definition?
