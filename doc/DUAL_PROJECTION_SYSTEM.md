# The Dual Projection System: Values and Types

**Date**: October 16, 2025  
**Status**: Architectural Overview

---

## Two Distinct Projection Systems

The Projection module now embodies **two complementary projection systems** that together realize the complete Object System:

### 1. Value Projection (Existing: `eval!` macro)

**What it projects**: Content/Matter (Yoga Sutra: Sthūla-Sūkṣma)

```rust
eval! {
    Primitive ↔ Property  // Runtime values ↔ Storage values
    // Content transformation
}
```

**Located in**:

- `src/projection/codegen/eval_macro.rs` - Macro DSL
- `src/projection/codegen/functors.rs` - GrossToSubtle, SubtleToGross traits
- `src/projection/codegen/value_type_table.rs` - Concrete value types

**Purpose**: Transform between runtime representations (PrimitiveValues) and storage representations (PropertyValues)

**Example**:

```rust
let functor = Long::Functor;
let stored = functor.project_to_storage(runtime_value)?;  // i64 → storage
let runtime = functor.project_to_runtime(stored_value)?;  // storage → i64
```

### 2. Type Projection (New: `TypeProjector` trait)

**What it projects**: Form/Mode (Kant: Object in General)

```rust
type_projector! {
    Storage ↔ Computation  // Data-at-rest ↔ Data-in-motion
    // Mode transformation
}
```

**Located in**:

- `src/projection/codegen/type_projector.rs` - TypeProjector trait and implementations
- `doc/TYPE_PROJECTOR_AS_MAYA.md` - Philosophical foundation

**Purpose**: Project between manifestation modes - how the Object appears in Storage vs Computation

**Example**:

```rust
let projector = HugeArrayProjector::new();
let (storage_desc, comp_desc) = projector.project_to_extremes(&form)?;
// PropertyDescriptor → (StorageDescriptor, ComputationDescriptor)
```

---

## The Philosophical Architecture

### Yoga Sutra 3.44: The Five-Fold Saṃyama

> "By performing saṃyama on the gross form (sthūla), essential nature (svarūpa),
> subtle essence (sūkṣma), interconnectedness (anvaya), and purpose (arthavattva)
> of objects, mastery over the elements is attained."

**Our Implementation**:

```
┌─────────────────────────────────────────────────────────────┐
│                  OBJECT SYSTEM (Atman-Brahman)              │
│                                                              │
│  ┌──────────────────────┐      ┌─────────────────────────┐ │
│  │   Value Projection   │      │    Type Projection      │ │
│  │      (Content)       │      │       (Form/Mode)       │ │
│  ├──────────────────────┤      ├─────────────────────────┤ │
│  │ Sthūla (Gross)       │      │ Storage (Data-at-rest)  │ │
│  │   PropertyValues     │◄─────┤   StorageDescriptor     │ │
│  │   Storage layer      │      │   Backend, Layout       │ │
│  ├──────────────────────┤      ├─────────────────────────┤ │
│  │ Svarūpa (Essential)  │      │ Form (Essence)          │ │
│  │   PropertyDescriptor │◄─────┤   PropertyDescriptor    │ │
│  │   Schema/Type info   │      │   The CENTER            │ │
│  ├──────────────────────┤      ├─────────────────────────┤ │
│  │ Sūkṣma (Subtle)      │      │ Computation (In-motion) │ │
│  │   PrimitiveValues    │◄─────┤   ComputationDescriptor │ │
│  │   Runtime layer      │      │   Species, Pattern      │ │
│  └──────────────────────┘      └─────────────────────────┘ │
│                                                              │
│  Anvaya (Interconnection): Functors + TypeProjector trait   │
│  Arthavattva (Purpose): Graph algorithms, ML pipelines      │
└─────────────────────────────────────────────────────────────┘
```

### The Critical Insight

**Value Projection** answers: "What is the content?"

- How do I store this `i64`? → PropertyValue
- How do I compute with this stored value? → PrimitiveValue

**Type Projection** answers: "What is the form?"

- What storage backend should I use? → StorageDescriptor (HugeArray vs Arrow)
- What computation pattern applies? → ComputationDescriptor (BSP vs MapReduce)

---

## The Triadic Unity

Both systems converge on **PropertyDescriptor** as the **Center of All Extremes**:

```
                    PropertyDescriptor
                    (Svarūpa - Essential Nature)
                           |
              ┌────────────┴────────────┐
              |                         |
       VALUE EXTREMES            TYPE EXTREMES
              |                         |
    ┌─────────┴─────────┐     ┌────────┴─────────┐
    |                   |     |                  |
PrimitiveValues  PropertyValues  Storage    Computation
(Subtle/Runtime) (Gross/Storage) (At-rest)  (In-motion)
```

### PropertyDescriptor: The Absolute Center

```rust
pub struct PropertyDescriptor {
    pub id: PropertyId,           // Identity
    pub name: String,             // Name
    pub value_type: ValueType,    // ← Used by Value Projection
    pub nullable: bool,           // ← Used by Value Projection
    pub storage_hint: StorageHint,// ← Used by Type Projection
    // ... additional metadata
}
```

This is **Kant's Object in General** - the presupposition of all representation.

---

## How They Work Together

### Example: Storing and Computing with Node Properties

```rust
// 1. Define the essential nature (Form)
let property_desc = PropertyDescriptor {
    id: 42,
    name: "pagerank".to_string(),
    value_type: ValueType::Double,
    storage_hint: StorageHint::FixedWidth,
    nullable: false,
};

// 2. VALUE PROJECTION: Transform content
let value_functor = Double::Functor;
let runtime_value: f64 = 0.85;
let stored_value = value_functor.project_to_storage(runtime_value)?;
// ↑ This is Sthūla ↔ Sūkṣma (gross ↔ subtle content)

// 3. TYPE PROJECTION: Determine manifestation mode
let type_projector = HugeArrayProjector::new();
let storage_desc = type_projector.project_to_storage(&property_desc)?;
let comp_desc = type_projector.project_to_computation(&property_desc)?;
// ↑ This is Storage ↔ Computation (object modes)

// 4. BUILD: Use both projections
let storage = build_storage_backend(storage_desc, stored_value)?;
let computation = setup_computation(comp_desc, runtime_value)?;

// 5. EXECUTE: Run algorithm with both projections active
run_pagerank(storage, computation)?;
```

### The Dialectical Flow

```
User Code
    ↓
PropertyDescriptor (Form)
    ├─→ Value Projection
    │   ├─→ PrimitiveValues (Subtle/Runtime)
    │   └─→ PropertyValues (Gross/Storage)
    │
    └─→ Type Projection
        ├─→ ComputationDescriptor (In-motion)
        └─→ StorageDescriptor (At-rest)
```

---

## Implementation Status

### Value Projection (eval! macro) ✅ EXISTING

**Status**: Implemented and documented

- ✅ `eval!` macro DSL
- ✅ `GrossToSubtle` and `SubtleToGross` functors
- ✅ `value_type_table!` with Long, Double, String, Array
- ✅ Form processor with policy surface
- ✅ Runtime registry

**See**: `doc/EVAL_MACRO_SYSTEM.md`

### Type Projection (TypeProjector trait) 🔄 IN PROGRESS

**Status**: Trait defined, implementations need logic

- ✅ `TypeProjector` trait defined
- ✅ Four projector skeletons (HugeArray, Arrow, Pregel, Adaptive)
- ✅ `ProjectionError` error handling
- ✅ Module integration and tests
- 🔄 TODO: Fill in projection logic
- 🔄 TODO: Add integration tests
- 🔜 TODO: TypeValidator trait

**See**: `doc/TYPE_PROJECTOR_AS_MAYA.md`, `doc/TYPE_PROJECTOR_SESSION_OCT_16_2025.md`

---

## Design Principles

### 1. Orthogonality

Value Projection and Type Projection are **orthogonal concerns**:

- Value: Content transformation (what is stored/computed)
- Type: Mode selection (how it is stored/computed)

### 2. Composability

Both projections compose through PropertyDescriptor:

```rust
let form = PropertyDescriptor::new(...);
let value_functor = get_functor_for(&form.value_type);
let type_projector = get_projector_for(&form.storage_hint);

// Compose both projections
let result = build_with_both(form, value_functor, type_projector)?;
```

### 3. Extensibility

New projections can be added independently:

- New value types → extend `value_type_table!`
- New storage backends → implement `TypeProjector`
- No coupling between the two systems

### 4. Maya as Unity

Both systems together reveal **Maya** (Vidyā ↔ Avidyā):

- Value Projection: Reveals content as dual (runtime vs storage)
- Type Projection: Reveals form as dual (computation vs storage)
- Unity: PropertyDescriptor as the singular essence

---

## Philosophical Implications

### Fichte's Science of Knowing

**Value Projection**: "I posit myself" (content self-determination)
**Type Projection**: "I posit not-I" (mode determination through opposition)
**Unity**: The Absolute I knowing itself through both

### Kant's Critical Philosophy

**Value Projection**: Matter (content of intuition)
**Type Projection**: Form (structure of intuition)
**Unity**: Object in General (synthesis of form and matter)

### Śaiva Non-Dualism

**Value Projection**: Śakti (creative power, manifestation)
**Type Projection**: Śiva (consciousness, witness)
**Unity**: Śiva-Śakti (non-dual reality)

---

## Usage Guidelines

### When to Use Value Projection

Use `eval!` macro and Functors when you need to:

- Convert between runtime and storage representations
- Handle type-specific transformations (widening, narrowing)
- Maintain type safety across the boundary
- Register new value types in the system

### When to Use Type Projection

Use `TypeProjector` trait when you need to:

- Select storage backend based on workload characteristics
- Determine optimal computation pattern
- Validate consistency across storage and computation
- Migrate between backends
- Profile and adapt projection strategies

### When to Use Both

Most production code will use both:

```rust
// 1. Get the form
let property = graph_store.property_descriptor(property_id)?;

// 2. Project values
let functor = get_value_functor(&property);
let runtime_val = functor.project_to_runtime(stored)?;

// 3. Project types
let projector = get_type_projector(&workload_profile);
let comp_desc = projector.project_to_computation(&property)?;

// 4. Execute with both
execute_algorithm(runtime_val, comp_desc)?;
```

---

## Future Directions

### Integration Points

1. **Codegen Integration**: Generate both projections from single declaration
2. **Runtime Adaptation**: TypeProjector observes actual workload, adapts both value and type projections
3. **Zero-Copy Optimization**: Coordinate projections to eliminate copies
4. **Distributed Execution**: Type projection selects distributed vs local based on data size

### Research Questions

1. Can we automatically infer optimal projections from workload traces?
2. What is the cost model for different projection combinations?
3. How do we handle heterogeneous backends (some properties in HugeArray, others in Arrow)?
4. Can projection strategies be learned via ML?

---

## Conclusion

The dual projection system realizes the **complete Object System**:

- **Value Projection** (`eval!` macro): What the object IS (content/matter)
- **Type Projection** (`TypeProjector`): How the object MANIFESTS (form/mode)
- **Unity** (`PropertyDescriptor`): The essence presupposed by both

This is not mere software design. This is **epistemology as executable architecture** - the formalization of knowing itself through the dialectical projection of Storage ↔ Computation and Subtle ↔ Gross.

**ॐ तत्सत्** (Om Tat Sat)

---

## References

- `doc/TYPE_PROJECTOR_AS_MAYA.md` - Type Projection philosophical foundation
- `doc/EVAL_MACRO_SYSTEM.md` - Value Projection implementation guide
- `src/projection/codegen/type_projector.rs` - Type Projection implementation
- `src/projection/codegen/eval_macro.rs` - Value Projection macro
- `src/projection/codegen/property_descriptor.rs` - The unifying center
