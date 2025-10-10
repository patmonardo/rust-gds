# Eval Macro System: Strategic Role & Architecture

**Date**: October 10, 2025  
**Status**: Active Development (Built Today)  
**Context**: Post-PropertyStore Integration, Pre-GDSL Runtime

---

## 🎯 Executive Summary

The **Eval Macro System** (`value_type_table!`) is **NOT** a speculative experiment. It is the **central runtime bridge** between:

1. **Persistent Storage** (PropertyStore/PropertyValues - "Gross World")
2. **Runtime Computation** (PrimitiveValues/GdsValue - "Subtle World")
3. **GDSL Kernel** (TypeScript DSL → Rust execution engine)
4. **Pipeline Configuration** (HugeArray vs Arrow vs Sparse backends per ML algorithm)

**Core Insight**: The eval macro **IS** the Form Processor — the transcendental projection that maps compile-time schema into both storage adapters and runtime drivers through a single source of truth.

**See Also**: `PIPELINE_BACKEND_CONFIGURATION_STRATEGY.md` for details on backend selection and ML pipeline optimization.

---

## 🌉 The Bridge Problem

### Current Reality

We have **two parallel type systems** that must interoperate:

```
PropertyStore (Persistent)          PrimitiveValues (Runtime)
┌────────────────────────┐         ┌────────────────────────┐
│ NodePropertyValues     │         │ GdsValue trait         │
│ - long_value(u64)      │ ←────→  │ - as_long()           │
│ - double_value(u64)    │         │ - as_double()         │
│ - array_value(u64)     │         │ - as_array()          │
└────────────────────────┘         └────────────────────────┘
       Gross World                        Subtle World
       (Storage)                          (Compute)
```

**The u64/usize Problem**:

- PropertyStore uses `u64` node IDs (public API, portable)
- Internal Rust uses `usize` indices (platform-dependent: 32-bit vs 64-bit)
- **Unsafe casts everywhere**: `node_id as usize` scattered throughout codebase
- **No overflow protection**: Silent wraparound on 32-bit targets

**The Type Conversion Problem**:

- PropertyStore → Pregel: manual conversion per type
- Pregel → PropertyStore: manual conversion back
- No central policy for widening (i32→i64, f32→f64)
- No validation layer between worlds

---

## 🔧 The Form Processor Solution

### Architecture: Three-Layer Projection

```
┌──────────────────────────────────────────────────────────┐
│              Noumenal (Pure Form)                        │
│          value_type_table! macro DSL                     │
│     Single Source of Truth for All Types                │
└──────────────────────────────────────────────────────────┘
                           ↓
                    Form Processor
                (Transcendental/Pure Nama)
                  Policy Enforcement
           ┌──────────────┴───────────────┐
           ↓                              ↓
┌──────────────────────┐      ┌──────────────────────┐
│ Phenomenal (Gross)   │      │ Phenomenal (Subtle)  │
│ PropertyValues       │      │ PrimitiveValues      │
│ Storage Adapters     │      │ Runtime Drivers      │
│ (Columnar, Arrow)    │      │ (Cursor, Compute)    │
└──────────────────────┘      └──────────────────────┘
```

### Key Components

#### 1. **form_processor.rs** (The Policy Surface)

**Location**: `src/projection/form_processor.rs`

**Purpose**: Centralized, auditable boundary between worlds

**Core Functions**:

```rust
/// Safe u64 → usize conversion with overflow protection
pub fn checked_u64_to_usize(id: u64) -> Result<usize, FormProcessorError> {
    usize::try_from(id).map_err(|_| FormProcessorError::IndexOverflow(id))
}

/// Safe widening i32 → i64 (always succeeds)
pub fn widen_i32_to_i64(v: i32) -> i64 { v as i64 }

/// Safe widening f32 → f64 (always succeeds)
pub fn widen_f32_to_f64(v: f32) -> f64 { v as f64 }
```

**Benefits**:

- ✅ Single point of policy enforcement
- ✅ Target-portable (32-bit vs 64-bit)
- ✅ Fail-fast on overflow (no silent corruption)
- ✅ Auditable (grep for `checked_u64_to_usize`)
- ✅ Instrumentable (add logging/metrics)

**Current Usage**:

- ❌ **NOT YET USED** - still manual `as usize` casts everywhere
- 🎯 **TODO**: Migrate PropertyValues implementations to use form_processor helpers

#### 2. **property_descriptor.rs** (Compile-Time Schema)

**Location**: `src/projection/property_descriptor.rs`

**Purpose**: Rich metadata for each property type

**Key Types**:

```rust
pub struct PropertyDescriptor {
    pub id: u32,                    // Unique type ID
    pub name: String,               // "Long", "Double", "LongArray"
    pub value_type: ValueType,      // Enum variant
    pub nullable: bool,             // Null allowed?
    pub storage_hint: StorageHint,  // Backend selection
}

pub enum StorageHint {
    Columnar,       // Arrow2, Vec<T>
    Sparse,         // HashMap, RoaringBitmap
    Compressed,     // LZ4, Zstd
    ZeroCopy,       // mmap, Arrow buffers
}
```

**Use Cases**:

- Backend selection (columnar vs sparse)
- Query planning (zero-copy views)
- Type validation (runtime checks)
- Schema evolution (version migration)

#### 3. **functors.rs** (Categorical Mappings)

**Location**: `src/projection/functors.rs`

**Purpose**: Bidirectional conversions between worlds

**Traits**:

```rust
/// Storage → Runtime projection (Gross → Subtle)
pub trait GrossToSubtle {
    fn project_from_storage(
        &self,
        gross: &dyn NodePropertyValues,
        node_id: u64,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

/// Runtime → Storage projection (Subtle → Gross)
pub trait SubtleToGross {
    fn project_to_storage(
        &self,
        subtle: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

/// Bidirectional functor (composition)
pub trait GrossSubtleFunctor: GrossToSubtle + SubtleToGross {}
```

**Example (Generated by Macro)**:

```rust
impl GrossToSubtle for LongFunctor {
    fn project_from_storage(
        &self,
        gross: &dyn NodePropertyValues,
        node_id: u64,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError> {
        let idx = form_processor::checked_u64_to_usize(node_id)?;
        let value = gross.long_value_unchecked(node_id);
        Ok(Some(Arc::new(DefaultLongValue::new(value))))
    }
}
```

#### 4. **eval_macro.rs** (The Master Projector)

**Location**: `src/projection/eval_macro.rs`

**Purpose**: Single source of truth for all property types

**Macro Syntax**:

```rust
value_type_table! {
    Long {
        id: 1,
        value_type: ValueType::Long,
        storage_hint: StorageHint::Columnar,
        rust_type: i64,
        gross_adapter: DefaultLongNodePropertyValues,
        subtle_impl: DefaultLongValue,
    },
    Double { ... },
    LongArray { ... },
    // ... unlimited types
}
```

**Generated Code Per Entry**:

```rust
pub mod Long {
    // 1. Compile-time descriptor
    pub static ref DESCRIPTOR: PropertyDescriptor = PropertyDescriptor { ... };

    // 2. Runtime registration
    pub fn register() -> bool { ... }

    // 3. Bidirectional functor
    pub struct Functor;
    impl GrossToSubtle for Functor { ... }
    impl SubtleToGross for Functor { ... }

    // 4. Typed accessors (future)
    pub fn get_long(store: &PropertyStore, node_id: u64) -> Result<i64, ...> { ... }

    // 5. Integration tests
    #[cfg(test)]
    mod tests { ... }
}
```

---

## 🔍 The u64/usize Problem in Detail

### Current State (Unsafe)

Throughout the codebase, we have **unchecked casts**:

```rust
// src/types/properties/node/impls/default_node_property_values.rs
fn long_value_unchecked(&self, node_id: u64) -> i64 {
    self.values[node_id as usize]  // ← UNSAFE! Can overflow on 32-bit
}

fn double_value_unchecked(&self, node_id: u64) -> f64 {
    self.values[node_id as usize]  // ← UNSAFE!
}

// Similar in 30+ other locations...
```

**Problem Scenarios**:

1. **32-bit Target**:

   ```rust
   let node_id: u64 = 5_000_000_000;  // 5 billion
   let idx = node_id as usize;         // Wraps to 705_032_704 on 32-bit!
   self.values[idx]                    // WRONG DATA returned!
   ```

2. **Silent Corruption**:

   - No panic
   - No error
   - Just returns value from wrong node
   - Debugging nightmare

3. **Portability Issue**:
   - Code works on 64-bit dev machines
   - Fails silently on 32-bit targets (ARM, WASM32)
   - No test coverage for this scenario

### Target State (Safe)

**All conversions through form_processor**:

```rust
// src/types/properties/node/impls/default_node_property_values.rs
fn long_value(&self, node_id: u64) -> Result<i64, FormProcessorError> {
    let idx = form_processor::checked_u64_to_usize(node_id)?;
    Ok(self.values[idx])
}

fn long_value_unchecked(&self, node_id: u64) -> i64 {
    // Only called in hot paths after validation
    // Document assumption: node_id < usize::MAX
    debug_assert!(node_id <= usize::MAX as u64);
    self.values[node_id as usize]
}
```

**Migration Strategy**:

1. **Phase 1**: Add checked methods alongside unchecked
2. **Phase 2**: Migrate callers to checked methods
3. **Phase 3**: Make unchecked methods internal-only (pub(crate))
4. **Phase 4**: Add #[must_use] and validation

---

## 🚀 GDSL Runtime Integration

### The TypeScript → Rust Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│  GDSL (TypeScript)                                          │
│  graph.nodes().filter(n => n.degree > 10)                  │
│         .property("pagerank", Double)                       │
└─────────────────────────────────────────────────────────────┘
                           ↓ (N-API Binding)
┌─────────────────────────────────────────────────────────────┐
│  Projection Layer (Rust)                                    │
│  - Parse DSL → PropertyDescriptor lookup                    │
│  - Validate types (Double exists? Nullable?)                │
│  - Create Functor (GrossSubtleFunctor)                      │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│  Form Processor                                             │
│  - checked_u64_to_usize (safe indexing)                     │
│  - Functor.project_from_storage (PropertyStore → GdsValue)  │
│  - Type validation, widening, coercion                      │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│  Execution (Pregel, Algorithms)                             │
│  - PrimitiveValues as runtime currency                      │
│  - Typed accessors (no casts, no panics)                    │
└─────────────────────────────────────────────────────────────┘
```

### Example: GDSL Query Execution

**TypeScript (GDSL)**:

```typescript
const result = await graph
  .pregel()
  .schema({
    value: { type: "Double", source: "pagerank" },
  })
  .compute((ctx) => {
    const rank = ctx.doubleNodeValue("value");
    ctx.setNodeValue("value", rank * 0.85);
  })
  .run();
```

**Rust Execution Flow**:

1. **N-API receives**: `{ value: { type: 'Double', source: 'pagerank' } }`

2. **PropertyDescriptor lookup**:

   ```rust
   let desc = get_property_descriptor_by_name("Double")?;
   // Found: DESCRIPTOR { id: 2, value_type: Double, ... }
   ```

3. **Functor creation**:

   ```rust
   let functor = Double::Functor;
   ```

4. **PropertyStore → Pregel initialization**:

   ```rust
   for node_id in 0..graph.node_count() {
       let idx = form_processor::checked_u64_to_usize(node_id)?;
       let property_values = graph.node_properties("pagerank")?;

       // Use functor to convert Gross → Subtle
       let gds_value = functor.project_from_storage(
           property_values.as_ref(),
           node_id
       )?;

       // Store in Pregel NodeValue
       node_values.set("value", idx, gds_value);
   }
   ```

5. **Compute execution**:

   ```rust
   let rank = ctx.double_node_value("value");  // Type-safe accessor
   ctx.set_node_value("value", rank * 0.85);
   ```

6. **Result materialization**:

   ```rust
   for node_id in 0..graph.node_count() {
       let idx = form_processor::checked_u64_to_usize(node_id)?;
       let gds_value = node_values.get("value", idx)?;

       // Use functor to convert Subtle → Gross
       let property_value = functor.project_to_storage(Some(gds_value))?;

       result_properties.set(node_id, property_value);
   }
   ```

---

## 📊 Current Status & Migration Plan

### ✅ Completed (Today - Oct 10)

- [x] **form_processor.rs**: Core helpers implemented
- [x] **property_descriptor.rs**: Schema types defined
- [x] **functors.rs**: Trait hierarchy complete
- [x] **eval_macro.rs**: Macro DSL implemented
- [x] **value_type_table.rs**: Prototype with 4 types (Long, Double, StringProp, LongArray)
- [x] **Integration tests**: 11 new tests passing
- [x] **Documentation**: ADR 0006, MEGA_MACRO_FACTORY.md

### 🔄 In Progress

- [ ] **Functor implementations**: Currently identity stubs, need real conversions
- [ ] **PropertyDescriptor registry**: Runtime lookup by name/id
- [ ] **Type validation**: Enforce type matching at boundaries
- [ ] **Typed accessors**: Generate get_long, get_double, etc.

### 🎯 Next Phase (Post-Quality Control)

#### 1. **Migrate PropertyValues to Form Processor** (1-2 days)

**Files to update** (30+ occurrences):

```
src/types/properties/node/impls/default_node_property_values.rs
src/types/properties/node/impls/values/*.rs
src/pregel/node_value.rs
src/pregel/context/*.rs
```

**Pattern**:

```rust
// Before:
fn long_value(&self, node_id: u64) -> i64 {
    self.values[node_id as usize]
}

// After:
fn long_value(&self, node_id: u64) -> Result<i64, FormProcessorError> {
    let idx = form_processor::checked_u64_to_usize(node_id)?;
    Ok(self.values[idx])
}
```

**Testing**:

```rust
#[test]
#[should_panic]
fn test_overflow_protection_32bit() {
    // Simulate 32-bit target
    let huge_id: u64 = u64::MAX;
    let result = form_processor::checked_u64_to_usize(huge_id);
    assert!(result.is_err());
}
```

#### 2. **Implement Real Functors** (2-3 days)

**Current** (identity stub):

```rust
impl GrossToSubtle for Functor {
    fn project_from_storage(...) -> Result<...> {
        Ok(value)  // Just pass through
    }
}
```

**Target** (real conversion):

```rust
impl GrossToSubtle for LongFunctor {
    fn project_from_storage(
        &self,
        gross: &dyn NodePropertyValues,
        node_id: u64,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError> {
        // 1. Safe indexing
        let idx = form_processor::checked_u64_to_usize(node_id)?;

        // 2. Type validation
        if gross.value_type() != ValueType::Long {
            return Err(FormProcessorError::TypeMismatch {
                expected: ValueType::Long,
                got: gross.value_type(),
            });
        }

        // 3. Extract value
        let value = gross.long_value_unchecked(node_id);

        // 4. Wrap in GdsValue
        Ok(Some(Arc::new(DefaultLongValue::new(value))))
    }
}
```

**Testing**:

```rust
#[test]
fn test_long_functor_converts_correctly() {
    let property_values = create_long_property_values(vec![100, 200, 300]);
    let functor = Long::Functor;

    let result = functor.project_from_storage(
        property_values.as_ref(),
        1
    ).unwrap();

    assert_eq!(result.unwrap().as_long().unwrap(), 200);
}

#[test]
fn test_long_functor_rejects_wrong_type() {
    let property_values = create_double_property_values(vec![1.5, 2.5]);
    let functor = Long::Functor;

    let result = functor.project_from_storage(
        property_values.as_ref(),
        0
    );

    assert!(matches!(result, Err(FormProcessorError::TypeMismatch { .. })));
}
```

#### 3. **PropertyDescriptor Registry** (1 day)

**Current**: Global HashMap in form_processor.rs

**Add**:

```rust
// Register by name
pub fn register_property_descriptor_by_name(name: &str, desc: PropertyDescriptor);

// Lookup by name (GDSL uses this)
pub fn get_property_descriptor_by_name(name: &str) -> Option<PropertyDescriptor>;

// List all registered
pub fn list_property_descriptors() -> Vec<PropertyDescriptor>;
```

**Usage**:

```rust
// During startup
Long::register();
Double::register();
LongArray::register();
// ... etc

// At runtime (GDSL query)
let desc = get_property_descriptor_by_name("Double")?;
let functor = create_functor_for_descriptor(&desc)?;
```

#### 4. **Expand value_type_table** (Ongoing)

**Current**: 4 types (Long, Double, StringProp, LongArray)

**Add** (prioritized):

```rust
value_type_table! {
    // Existing:
    Long { ... },
    Double { ... },
    LongArray { ... },

    // New (priority order):
    DoubleArray { ... },     // Pregel needs this
    FloatArray { ... },      // Common in ML
    String { ... },          // Text properties
    Boolean { ... },         // Flags

    // Future:
    Struct { ... },          // UDTs
    Embedding { ... },       // Vector embeddings (512-dim f32)
    Audio { ... },           // WAV/MP3 blobs
}
```

---

## 🎓 Philosophical Foundation

### Yoga Sutra 3.44 - The Five-Fold Projection

**Sanskrit**:

> sthūla-svarūpa-sūkṣma-anvaya-arthavattva-saṃyamāt bhūta-jaya

**Translation**:

> By focused meditation (saṃyamāt) on the gross (sthūla), essential nature (svarūpa), subtle (sūkṣma), interconnectedness (anvaya), and purpose (arthavattva), mastery over the elements (bhūta-jaya) is attained.

**Mapping to Eval Macro**:

| Sanskrit        | Eval Macro Component      | Description                          |
| --------------- | ------------------------- | ------------------------------------ |
| **Sthūla**      | PropertyValues (Gross)    | Physical storage (columnar, arrays)  |
| **Svarūpa**     | PropertyDescriptor (Form) | Essential schema (type, constraints) |
| **Sūkṣma**      | PrimitiveValues (Subtle)  | Runtime values (compute, cursors)    |
| **Anvaya**      | Functors (Mappings)       | Bidirectional conversions            |
| **Arthavattva** | Form Processor (Policy)   | Purpose/validation at boundary       |
| **Saṃyamāt**    | value_type_table! (Macro) | Focused projection (single source)   |
| **Bhūta-jaya**  | Type Safety (Correctness) | Mastery over runtime representation  |

**Key Insight**:
The eval macro **IS** the saṃyamāt (focused meditation) that projects the svarūpa (essential schema) into both sthūla (storage) and sūkṣma (runtime) worlds through anvaya (functors), governed by arthavattva (form processor policy), achieving bhūta-jaya (type safety).

---

## 🔒 Safety Guarantees

### What the Form Processor Ensures

1. **No Overflow**:

   ```rust
   // Panics on 32-bit if node_id > usize::MAX
   checked_u64_to_usize(node_id)?
   ```

2. **No Type Confusion**:

   ```rust
   // Errors if PropertyStore has Double but GdsValue expects Long
   if property_type != expected_type {
       return Err(FormProcessorError::TypeMismatch { ... });
   }
   ```

3. **No Null Derefs**:

   ```rust
   // Functor returns Option<Arc<dyn GdsValue>>
   match functor.project_from_storage(...) {
       Ok(Some(value)) => { /* use value */ },
       Ok(None) => { /* handle null */ },
       Err(e) => { /* handle error */ },
   }
   ```

4. **No Silent Widening**:

   ```rust
   // Only explicit, safe widening via form_processor
   widen_i32_to_i64(v)  // OK: i32 always fits in i64
   widen_f32_to_f64(v)  // OK: f32 always fits in f64
   // Anything else must go through explicit coercion + validation
   ```

5. **Auditable**:

   ```bash
   # All boundary crossings in one place
   git grep "form_processor::"

   # All unsafe casts should be eliminated
   git grep "as usize" -- src/types/properties/
   ```

---

## 📚 Key Documents

### Essential Reading

1. **ADR 0006**: `doc/adr0006_projection_as_gdsl.md`

   - Architectural decision rationale
   - Philosophical foundation (Yoga Sutra 3.44)
   - Migration path

2. **MEGA_MACRO_FACTORY.md**: `doc/MEGA_MACRO_FACTORY.md`

   - Macro system design
   - Comparison to Zod (TypeScript runtime validation)
   - Code generation patterns

3. **EVAL_MACRO_IMPLEMENTATION_SUMMARY.md**: `doc/EVAL_MACRO_IMPLEMENTATION_SUMMARY.md`
   - Complete system documentation
   - File-by-file breakdown
   - Test results

### Related ADRs

- **ADR 0002**: Triadic GraphStore Architecture (Store ↔ Graph ↔ Projection)
- **ADR 0005**: Values System Architecture (GdsValue, PrimitiveValues)
- **(Future) ADR 0007**: GDSL Language Specification

### Related Strategic Documents

- **PIPELINE_BACKEND_CONFIGURATION_STRATEGY.md**: Backend selection (HugeArray/Arrow/Sparse) for ML pipelines
- **EVAL_MACRO_MIGRATION_PLAN.md**: u64/usize safety migration plan
- **unified_macro_architecture.md**: Three-layer macro system design

---

## 💡 Key Takeaways

1. **Eval Macro is CENTRAL, not speculative**: It's the bridge between PropertyStore (storage) and PrimitiveValues (compute).

2. **Form Processor enforces safety**: All u64→usize conversions, type validations, and widening rules go through one auditable surface.

3. **Functors enable GDSL**: TypeScript DSL → PropertyDescriptor lookup → Functor → Safe execution.

4. **Migration is urgent**: 30+ unsafe `as usize` casts must be replaced with `checked_u64_to_usize()`.

5. **Backend flexibility**: Eval macro enables HugeArray vs Arrow vs Sparse selection per ML pipeline (10-100x performance gains).

6. **This is systems architecture**: Not "vibe programming" — careful, deliberate boundary management.

---

## 🎯 Action Items (Post-Quality Control)

### Immediate (This Week)

- [ ] Audit all `node_id as usize` occurrences
- [ ] Create tracking issue for form_processor migration
- [ ] Add PropertyDescriptor lookup by name
- [ ] Implement real functors for Long, Double, LongArray

### Short-Term (Next Sprint)

- [ ] Migrate PropertyValues implementations to checked conversions
- [ ] Add DoubleArray, FloatArray to value_type_table
- [ ] Create integration tests for GDSL query execution
- [ ] Performance benchmark: functor overhead vs direct access

### Long-Term (Future)

- [ ] UDT support (struct layouts)
- [ ] Zero-copy views (Arrow buffers)
- [ ] GDSL parser integration
- [ ] N-API bindings for TypeScript

---

**Bottom Line**: The eval macro system is **the kernel bridge** — the transcendental surface where GDSL (TypeScript expressiveness) meets PropertyGraph (Rust performance) with **type safety, portability, and auditability**. It's not optional; it's foundational.
