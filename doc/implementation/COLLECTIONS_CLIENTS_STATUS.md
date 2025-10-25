# Collections Clients Status Report

**Date**: October 2024  
**Context**: Post-Codegen Event - The Dust is Settling  
**Focus**: PropertyStore as Collections Client

---

## The Big Picture

We now have **TWO Collections Clients** to the ML Collections package:

1. **Monadic PropertyStore** - Ordinary Client (Universal, Single-Level)
2. **Triadic PropertyStore** - Hyper Client (Context-Aware, Three-Level)

Both are **awash in Type-Value combinatoric madness** that macros help tame!

---

## Current State: What Exists

### ✅ NEW: Collections First Clients (Experimental)

#### Monadic PropertyStore

**Status**: ✅ Working, Macro-Generated, 15 tests passing

```
gds/src/types/properties/monadic/
├── mod.rs                  # Barrel exports
├── macros.rs              # monadic_property_values! macro
├── property.rs            # MonadicProperty (Collections-backed)
├── property_store.rs      # MonadicPropertyStore (HashMap of properties)
└── property_values.rs     # 18 generated PropertyValues types
```

**Generated Types** (via `monadic_property_values!` macro):
```rust
// 9 Primitives
MonadicLongPropertyValues<C: Collections<i64>>
MonadicDoublePropertyValues<C: Collections<f64>>
MonadicIntPropertyValues<C: Collections<i32>>
MonadicFloatPropertyValues<C: Collections<f32>>
MonadicShortPropertyValues<C: Collections<i16>>
MonadicBytePropertyValues<C: Collections<i8>>
MonadicBooleanPropertyValues<C: Collections<bool>>
MonadicCharPropertyValues<C: Collections<char>>
MonadicStringPropertyValues<C: Collections<String>>

// 9 Arrays (Vec<T>)
MonadicLongArrayPropertyValues<C: Collections<Vec<i64>>>
MonadicDoubleArrayPropertyValues<C: Collections<Vec<f64>>>
MonadicIntArrayPropertyValues<C: Collections<Vec<i32>>>
MonadicFloatArrayPropertyValues<C: Collections<Vec<f32>>>
MonadicShortArrayPropertyValues<C: Collections<Vec<i16>>>
MonadicByteArrayPropertyValues<C: Collections<Vec<i8>>>
MonadicBooleanArrayPropertyValues<C: Collections<Vec<bool>>>
MonadicCharArrayPropertyValues<C: Collections<Vec<char>>>
MonadicStringArrayPropertyValues<C: Collections<Vec<String>>>
```

**Backends Working**:
- ✅ VecLong, VecDouble, VecInt, VecFloat (all primitives)
- ✅ HugeLongArray, HugeDoubleArray
- ⏳ HugeIntArray, HugeFloatArray (need Collections impl)
- ⏳ HugeObjectArray (for arrays/maps)

#### Triadic PropertyStore

**Status**: ✅ Working, Composition-Based, 5 tests passing

```
gds/src/types/properties/triadic/
├── mod.rs                  # Architecture docs + exports
└── property_store.rs       # TriadicPropertyStore (3 MonadicPropertyStores)
```

**Structure**:
```rust
TriadicPropertyStore {
    meta_properties: MonadicPropertyStore,   // Level 0: Graph metadata
    node_properties: MonadicPropertyStore,   // Level 1: Node properties
    link_properties: MonadicPropertyStore,   // Level 2: Relationship properties
}
```

**Key Innovation**: Separate key spaces per level!

---

### ⚠️ LEGACY: Pre-Collections Implementations

#### Node PropertyStore (Legacy)

**Status**: 🔴 Still using old backend.rs/factory.rs pattern

```
gds/src/types/properties/node/
├── node_property_store.rs              # Trait definition
├── node_property_values.rs             # Trait definition
├── node_property.rs                    # Trait definition
└── impls/
    ├── default_node_property_store.rs  # Vec-based store
    ├── default_node_property_values.rs # Vec-based values
    ├── default_node_property.rs        # Default property
    └── huge_node_property_values.rs    # HugeArray-based values
```

**Legacy PropertyValues Types**:
```rust
// Default (Vec-backed)
DefaultLongNodePropertyValues
DefaultDoubleNodePropertyValues
DefaultLongArrayNodePropertyValues
DefaultDoubleArrayNodePropertyValues

// Huge (HugeArray-backed)
HugeLongNodePropertyValues
HugeDoubleNodePropertyValues
HugeLongArrayNodePropertyValues
HugeDoubleArrayNodePropertyValues
```

#### Relationship PropertyStore (Legacy)

**Status**: 🔴 Complex cursor-based system

```
gds/src/types/properties/relationship/
├── relationship_property_store.rs              # Trait
├── relationship_property_values.rs             # Trait
├── relationship_property.rs                    # Trait
├── property_cursor.rs                          # Cursor trait
└── impls/
    ├── default_relationship_property_store.rs  # Vec-based
    ├── default_relationship_property_values.rs # Vec-based
    ├── default_relationship_property.rs        # Default
    └── default_relationship_cursor.rs          # Iterator
```

**Special Complexity**: Relationships are sparse, indexed²!

#### Graph PropertyStore (Legacy)

**Status**: 🔴 Simple but legacy pattern

```
gds/src/types/properties/graph/
├── graph_property_store.rs              # Trait
├── graph_property_values.rs             # Trait
├── graph_property.rs                    # Trait
└── impls/
    ├── default_graph_property_store.rs  # Vec-based
    ├── default_graph_property_values.rs # Vec-based
    └── default_graph_property.rs        # Default
```

**Legacy PropertyValues Types**:
```rust
DefaultLongGraphPropertyValues
DefaultDoubleGraphPropertyValues
DefaultLongArrayGraphPropertyValues
DefaultDoubleArrayGraphPropertyValues
DefaultFloatArrayGraphPropertyValues
```

---

## The Type-Value Combinatoric Explosion

### What We're Facing

**Value Types**: 46 types in `ValueType` enum
- 9 primitives (Long, Double, Int, Float, Short, Byte, Boolean, Char, String)
- 9 arrays (LongArray, DoubleArray, etc.)
- 9 maps (LongMap, DoubleMap, etc.)
- Plus: Object, ListValue, MapValue, GeometryValue, etc.

**Property Levels**: 3 contexts
- Node properties (indexed by node_id)
- Relationship properties (indexed by rel_id, often sparse)
- Graph properties (scalar or collection)

**Backends**: Multiple storage strategies
- Vec (small, dense)
- HugeArray (billions of elements, paged)
- Arrow (columnar, SIMD)
- Compressed, Encrypted, Distributed, etc.

**Total Combinations**: 46 types × 3 levels × N backends = 🤯

### How Macros Help

#### Current: Monadic Macro

```rust
monadic_property_values!(MonadicLongPropertyValues => i64, ValueType::Long);
```

Generates:
- Struct definition with generic Collections backend
- `new()` and `values()` methods
- `PropertyValues` trait impl

**Coverage**: 18/46 types (primitives + arrays)

#### Needed: More Macros!

We need macros for:
1. **Map types** - `HashMap<K, V>` as element type
2. **Object types** - Generic object storage
3. **Geometry types** - Spatial data
4. **Triadic generation** - Auto-generate level-specific impls

---

## The Path Forward

### Phase 1: Complete Monadic Coverage ⏳

**Goal**: All 46 ValueTypes in Monadic

1. ✅ 9 Primitives (DONE)
2. ✅ 9 Arrays (DONE)
3. ⏳ 9 Maps (HashMap<K, V>)
4. ⏳ Object types
5. ⏳ List/Map value types
6. ⏳ Geometry types
7. ⏳ Temporal types

**Challenge**: Collections<T> needs to support:
- `Collections<HashMap<String, i64>>` for maps
- `Collections<Box<dyn Any>>` for objects?
- Custom serialization for complex types

### Phase 2: Implement Missing Collections Backends ⏳

**Goal**: All backends for all primitive/array types

Currently missing Collections impl:
- ⏳ HugeIntArray, HugeFloatArray, HugeShortArray, HugeByteArray
- ⏳ HugeBooleanArray, HugeCharArray, HugeStringArray
- ⏳ HugeObjectArray (for arrays/maps)
- ⏳ Arrow* (all types)
- ⏳ Compressed* (all types)

### Phase 3: Migrate Legacy to Collections First 🔮

**Option A**: Create adapter layer
```rust
// Bridge legacy to Collections
impl NodePropertyValues for MonadicLongPropertyValues<C> {
    fn node_property(&self, node_id: NodeId) -> Option<i64> {
        self.values.get(node_id as usize)
    }
}
```

**Option B**: Replace legacy entirely
- Delete old default/huge implementations
- Use Monadic + Collections everywhere
- Update GraphStore to use Triadic

**Option C**: Coexistence
- Keep legacy for compatibility
- New code uses Collections First
- Gradual migration over time

### Phase 4: The Triadic Evolution 🔮

**Vision**: GraphStore becomes a Triadic client

```rust
struct GraphStore {
    properties: TriadicPropertyStore,  // Meta/Node/Link composition
    topology: CSRGraph,
    // ...
}

impl GraphStore {
    fn node_property(&self, key: &str, node_id: NodeId) -> Option<PropertyValue> {
        self.properties.get_node_property_values(key)?
            .get(node_id as usize)
    }
}
```

**Benefits**:
- Separate key spaces for node vs rel vs graph
- Independent backend selection per level
- Cleaner separation of concerns
- Easier testing and evolution

---

## The Macro Challenge

### Current Macro

`monadic_property_values!` is **simple and powerful**:
- Generates struct + impl in one go
- Generic over Collections backend
- Works for primitives and arrays

### What We Need

#### 1. Map Property Values Macro

```rust
monadic_map_property_values!(
    MonadicLongMapPropertyValues => HashMap<String, i64>, 
    ValueType::LongMap
);
```

Challenge: Collections<HashMap<K, V>> semantics?

#### 2. Triadic Property Values Macro

```rust
triadic_property_values!(
    LongPropertyValues => i64, 
    ValueType::Long,
    node_indexed: true,    // Indexed by node_id
    rel_indexed: true,     // Indexed by rel_id
    graph_scalar: true     // Single value or list
);
```

Generates three impls: Node, Rel, Graph variants!

#### 3. Universal Property Store Macro

```rust
property_store!(
    MyPropertyStore,
    backend: Collections,
    levels: [Meta, Node, Link],
    types: [Long, Double, String, ...]
);
```

Auto-generates entire store with all types!

---

## Open Questions

### Architecture

1. **Should Triadic replace legacy Node/Rel/Graph PropertyStores?**
   - Pro: Cleaner, Collections First, separate key spaces
   - Con: Breaking change, migration complexity

2. **Should PropertyStore be monadic with smart indexing?**
   - You suggested: "just need a property_store module that hooks into monadic"
   - This could work! Monadic as backend, smart accessor layer on top

3. **How to handle relationship sparsity?**
   - Relationships are often sparse (not all edges have all properties)
   - Cursors help with iteration, but Collections needs sparse support

### Collections Trait

1. **Should Collections<T> support HashMap<K, V> as T?**
   - Needed for map types
   - How to index into a collection of maps?

2. **Should Collections support null/missing values?**
   - Many properties are sparse
   - Option<T> vs explicit null bitmask?

3. **What about object/dynamic types?**
   - Collections<Box<dyn Any>>?
   - Collections<serde_json::Value>?

### Macros

1. **How granular should macros be?**
   - One macro per type? (18+ macros)
   - One mega-macro for all types? (complex)
   - Layered macros? (core + extensions)

2. **Should macros generate trait impls or structs?**
   - Current: generates both
   - Alternative: generate structs, manual trait impls

3. **How to handle backend-specific optimizations?**
   - HugeArray has special paging logic
   - Arrow has SIMD kernels
   - Generic macro can't capture all nuance

---

## What's Working Right Now

### ✅ Proven Concepts

1. **Collections as Universal Backend** - Works for 18 types!
2. **Macro Generation** - Reduces boilerplate dramatically
3. **Monadic Pattern** - Simple, testable, composable
4. **Triadic Composition** - Three monads = powerful abstraction
5. **Separate Key Spaces** - Natural and flexible

### ⏳ In Progress

1. **More Collections Backends** - Need to implement for all types
2. **Map/Object Support** - Collections<HashMap> semantics unclear
3. **Legacy Migration** - Path from old to new unclear

### 🔮 Future Work

1. **Full ValueType Coverage** - 28 more types to go!
2. **GraphStore Integration** - Should it use Triadic?
3. **Performance Validation** - Is Collections overhead acceptable?
4. **Macro Ecosystem** - Need more code generation tools

---

## Immediate Next Steps

### Yesterday: Codegen Massive Event ✅

- Implemented monadic_property_values! macro
- Generated 18 PropertyValues types
- Proved Collections First works!

### Today: Dust Settling, Exploration Phase

**We should explore**:

1. **Collections Backends Inventory**
   - What's implemented?
   - What's missing?
   - What needs Collections trait?

2. **Legacy PropertyStore Analysis**
   - How do Node/Rel/Graph stores differ?
   - Can they all use Monadic?
   - What special logic is needed?

3. **Type-Value Matrix**
   - Which types work with which backends?
   - Which combinations are tested?
   - Which are high-priority?

4. **Macro Strategy**
   - Do we need more macros?
   - Should we generalize existing ones?
   - How to handle backend-specific logic?

---

## Your Question

> "so you are saying we just need a property_store module that hooks into monadic store and property impls?"

**YES!** That's the vision:

```rust
// Monadic is the universal backend
MonadicPropertyStore<C: Collections<T>> {
    properties: HashMap<String, MonadicProperty<C>>
}

// Smart accessors layer on top
trait NodePropertyStore {
    fn node_property(&self, key: &str, node_id: NodeId) -> Option<PropertyValue>;
}

impl<C> NodePropertyStore for MonadicPropertyStore<C> {
    fn node_property(&self, key: &str, node_id: NodeId) -> Option<PropertyValue> {
        self.properties.get(key)?
            .values()  // Gets Collections<T>
            .get(node_id as usize)  // Index into collection
    }
}
```

**The pattern**:
1. Monadic = universal storage (Collections First)
2. Smart layer = domain-specific access (Node/Rel/Graph semantics)
3. Triadic = composition for three-level systems

---

## Bottom Line

We're at an **exciting inflection point**:

- ✅ **Monadic works** - 18 types, macro-generated, tested
- ✅ **Triadic works** - composition pattern validated
- ⏳ **Collections partially complete** - more backends needed
- ⏳ **Legacy still exists** - migration path unclear
- 🔮 **Future is bright** - but lots to explore!

The experiment is **just beginning**, and the dust is **just settling**. 

Let's see what's going on with our Collections clients! 🚀

