# Type Projector as Maya — The Dialectical Absolute

**Date**: October 16, 2025  
**Author**: Pat Monardo  
**Status**: Philosophical Foundation & Design Specification

---

## The Absolute Type: Maya as Projection

Maya is the **Unity of Revealing (Vidyā) and Concealing (Avidyā)** — the dialectical synthesis that manifests the Object System through the projection of **Storage ↔ Computation**.

This is distinct from, yet presupposes, the Value Type Projection (Primitive ↔ Property) which we have already implemented via the `eval!` macro system.

## Philosophical Architecture

### The Five-Fold Synthesis of Powers (Śaktis)

**Two-Fold Division:**

1. **Revealing (Vidyā)** — Knowledge, manifestation, projection
2. **Concealing (Avidyā)** — Ignorance, occlusion, limitation

**Three-Fold Powers (Īśvara):**

1. **Creation (Sṛṣṭi)** — Bringing into being
2. **Preservation (Sthiti)** — Maintaining in being
3. **Destruction (Saṃhāra)** — Dissolution back to potential

**The Unity:**

- **Revealing + Concealing = Projection = Maya**
- **Creation + Preservation + Destruction = Īśvara**
- **Knowing both Maya and Īśvara = Brahman Realization**

### The Absolute Type System

```
                           ĀTMAN-BRAHMAN
                                 |
                                 |
                    ┌────────────┴────────────┐
                    |                         |
                  MAYA                     ĪŚVARA
          (Vidyā ↔ Avidyā)        (Sṛṣṭi-Sthiti-Saṃhāra)
       (Revealing ↔ Concealing)  (Create-Preserve-Destroy)
                    |                         |
                    |                         |
              TYPE PROJECTOR            OBJECT LIFECYCLE
           (Storage ↔ Computation)      (Build-Use-Drop)
```

## The Dialectical Object System

### Kant's Two Species of Object in General

1. **Noumenon** (Thing-in-Itself) → **Subtle** → **Primitive Values** → **Computation**
2. **Phenomenon** (Appearance) → **Gross** → **Property Values** → **Storage**

### The Critical Insight

**Value Projection** (what we have):

```rust
eval! {
    Primitive ↔ Property  // Subtle ↔ Gross
    // This is value transformation
}
```

**Type Projection** (what we need):

```rust
type_projector! {
    Storage ↔ Computation  // The Object System itself
    // This is the dialectical mapping of object modes
}
```

### The Distinction

- **Value Projection**: Maps between representations of _content_

  - Primitive ↔ Property (runtime values ↔ storage values)
  - Implemented via `eval!` macro and Functors
  - The **matter** of objects (Yoga Sutra: sthūla-svarūpa-sūkṣma)

- **Type Projection**: Maps between _modes of manifestation_
  - Storage ↔ Computation (data-at-rest ↔ data-in-motion)
  - Needs Type Projector system
  - The **form** of objects (Kant: object in general)

## Type Projector: The Zod-Like System

### What "Zod-Like" Means

Not TypeScript's Zod validation library, but:

**A runtime type descriptor/validator that dialectically maps:**

```
Storage Extreme ←→ [Type Projector as Maya] ←→ Computation Extreme
```

### The Triadic Structure (Already Present!)

```rust
pub struct PropertyDescriptor {      // Form/Svarūpa (essential nature)
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
    // ...
}

pub struct StorageDescriptor {       // Gross/Sthūla (physical manifestation)
    pub backend: BackendTechnology,
    pub layout: StorageLayout,
    pub density: Density,
    pub access_pattern: AccessPattern,
    // ...
}

pub struct ComputationDescriptor {   // Subtle/Sūkṣma (computational manifestation)
    pub id: u32,
    pub species: ComputationSpecies,
    pub pattern: ComputationPattern,
    // ...
}
```

### The Missing Link: Type Projector

**Type Projector is the dialectical unity that:**

1. Takes a `PropertyDescriptor` (Form)
2. Projects it to `StorageDescriptor` (Gross extreme)
3. Projects it to `ComputationDescriptor` (Subtle extreme)
4. Maintains the bidirectional mapping (Maya as Vidyā ↔ Avidyā)

## The Type Projector as Fichte's Science of Knowing

### Fichte's Triadic Structure

1. **Thesis**: Storage (Being, data-at-rest)
2. **Antithesis**: Computation (Becoming, data-in-motion)
3. **Synthesis**: Type Projector (Knowing, the dialectical unity)

### Self-Realization as Brahman

**Fichte**: "I am I" through knowing the dialectical unity of positing and opposing.

**Our System**: "I am Brahman" through knowing the dialectical unity of Storage and Computation (Maya), which presupposes knowing the lifecycle (Īśvara).

**The Absolute Knowing**:

```rust
impl TypeProjector {
    /// The dialectical synthesis - Maya as Unity
    fn project_to_extremes(&self, form: PropertyDescriptor)
        -> (StorageDescriptor, ComputationDescriptor)
    {
        // This IS the revelation of Maya
        // Vidyā: Revealing both extremes from Form
        // Avidyā: Concealing the unity in the duality
    }

    /// The inverse - recognizing unity in duality
    fn recognize_unity(&self,
        storage: StorageDescriptor,
        computation: ComputationDescriptor
    ) -> PropertyDescriptor
    {
        // This IS Brahman realization
        // Seeing the Form presupposed by both extremes
    }
}
```

## Design Specification

### Core Type Projector Trait

```rust
/// The Absolute Type - Maya as dialectical projector
pub trait TypeProjector {
    /// Project Form to Storage extreme (Vidyā: Revealing as Gross)
    fn project_to_storage(&self, form: &PropertyDescriptor)
        -> Result<StorageDescriptor, ProjectionError>;

    /// Project Form to Computation extreme (Vidyā: Revealing as Subtle)
    fn project_to_computation(&self, form: &PropertyDescriptor)
        -> Result<ComputationDescriptor, ProjectionError>;

    /// Recognize Form from Storage manifestation (Avidyā: Concealing Computation)
    fn recognize_from_storage(&self, storage: &StorageDescriptor)
        -> Result<PropertyDescriptor, ProjectionError>;

    /// Recognize Form from Computation manifestation (Avidyā: Concealing Storage)
    fn recognize_from_computation(&self, computation: &ComputationDescriptor)
        -> Result<PropertyDescriptor, ProjectionError>;

    /// The dialectical synthesis: validate consistency across extremes
    fn validate_projection(&self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor
    ) -> Result<(), ProjectionError>;
}
```

### Policy-Based Projector Implementations

```rust
/// HugeArray-optimized projector
pub struct HugeArrayProjector {
    // Policies for projecting to chunked, dense, sequential storage
}

/// Arrow-optimized projector
pub struct ArrowProjector {
    // Policies for projecting to columnar, zero-copy storage
}

/// Pregel-optimized projector
pub struct PregelProjector {
    // Policies for projecting to vertex-centric computation
}

/// Adaptive projector using runtime profiling
pub struct AdaptiveProjector {
    profiler: RuntimeProfiler,
    // Chooses projection strategy based on actual workload
}
```

### Runtime Type Validation

```rust
/// Runtime type validator - the "Zod-like" aspect
pub trait TypeValidator {
    /// Validate that a value conforms to descriptor
    fn validate(&self, value: &dyn Any, desc: &PropertyDescriptor) -> Result<(), ValidationError>;

    /// Infer descriptor from runtime value
    fn infer_descriptor(&self, value: &dyn Any) -> Option<PropertyDescriptor>;

    /// Check compatibility between descriptors
    fn is_compatible(&self, source: &PropertyDescriptor, target: &PropertyDescriptor) -> bool;
}
```

## Implementation Strategy

### Phase 1: Type Projector Foundation (Current)

- ✅ PropertyDescriptor (Form/Svarūpa) - Already exists
- ✅ StorageDescriptor (Gross/Sthūla) - Already exists
- ✅ ComputationDescriptor (Subtle/Sūkṣma) - Already exists
- 🔄 Define `TypeProjector` trait
- 🔄 Implement basic projectors (HugeArray, Arrow, Pregel)

### Phase 2: Runtime Validation

- 🔜 Define `TypeValidator` trait
- 🔜 Implement validation logic
- 🔜 Add runtime descriptor inference
- 🔜 Integration with existing descriptor registries

### Phase 3: Adaptive Projection

- 🔜 Runtime profiling infrastructure
- 🔜 Adaptive projector that learns optimal mappings
- 🔜 Migration between storage backends
- 🔜 Computation pattern detection

### Phase 4: Codegen Integration

- 🔜 Macro DSL for declaring projection policies
- 🔜 Generated projector implementations
- 🔜 Compile-time validation
- 🔜 Zero-cost abstractions via monomorphization

## Philosophical Implications

### Maya as Known by Me

The Type Projector is not just a technical system. It is the **formalization of Maya as knowable structure**.

**Brahman Realization** (in our system):

```rust
// I am Brahman because I KNOW the dialectical mapping
let projector = TypeProjector::new();
let (storage, computation) = projector.project_to_extremes(form);
// ↑ This IS Vidyā (revealing the duality)

let recognized_form = projector.recognize_unity(storage, computation)?;
// ↑ This IS Avidyā (concealing by recognizing the unity)

assert_eq!(form, recognized_form);
// ↑ This IS Brahman knowing itself through Maya
```

### The All-Seeing Eye (सर्वज्ञ)

The Type Projector is **Sarvajña** (All-Knowing) because it:

1. Sees the Form (PropertyDescriptor)
2. Projects to all manifestations (Storage, Computation)
3. Recognizes unity in duality
4. Validates consistency across extremes

This is **transcendental apperception** — the "I think" that accompanies all representations.

## Next Steps

1. **Implement `TypeProjector` trait** in `src/projection/codegen/type_projector.rs`
2. **Create basic projector implementations** for HugeArray, Arrow, Pregel
3. **Wire into existing descriptor systems** (PropertyDescriptor, StorageDescriptor, ComputationDescriptor)
4. **Add validation and inference logic**
5. **Document projection policies** for each backend
6. **Design macro DSL** for declaring custom projectors

## References

### Primary Sources

- **Yoga Sutra 3.44**: Saṃyama on gross, essential nature, subtle, interconnectedness, purpose
- **Fichte, Science of Knowledge**: Thesis-Antithesis-Synthesis as absolute knowing
- **Kant, Critique of Pure Reason**: Object in general presupposing noumenon and phenomenon
- **Śaiva Philosophy**: Five-fold powers (Pañca-kṛtya) of Śiva

### Technical References

- `src/projection/codegen/property_descriptor.rs` — Form/Svarūpa
- `src/projection/codegen/storage_descriptor.rs` — Gross/Sthūla
- `src/projection/codegen/computation_descriptor.rs` — Subtle/Sūkṣma
- `src/projection/codegen/eval_macro.rs` — Value projection (existing)
- `doc/adr0002_triadic_graphstore_architecture.md` — Triadic pattern precedent

---

**ॐ तत्सत्** (Om Tat Sat) — That Absolute Truth

The Type Projector IS Maya known by Brahman (the developer/system) through dialectical synthesis of Storage and Computation extremes.
