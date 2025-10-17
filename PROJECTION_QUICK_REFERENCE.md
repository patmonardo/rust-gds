# PROJECTION QUICK REFERENCE

## The Five-Fold Synthesis at a Glance

```
┌─────────────────────────────────────────────────────────────────┐
│                    PROJECTION IN ACTION                         │
└─────────────────────────────────────────────────────────────────┘

DESCRIPTOR (Identity Pole)
  ├─ ComputationDescriptor { id, name, species, pattern, membership }
  ├─ PropertyDescriptor { name, value_type, nullable, membership }
  ├─ StorageDescriptor { backend, layout, memory_profile, membership }
  └─ ProcedureDescriptor { qualified_name, modes, category, membership }

MEMBERSHIP (Inherence)
  ├─ compatible_value_types: Vec<ValueType>
  ├─ preferred_storage_layouts: Vec<StorageLayout>
  ├─ target_density: Density
  ├─ access_pattern: AccessPattern
  └─ required_concurrency: ConcurrencyModel

CONSEQUENCE (Logical Entailment)
  ├─ ConsequenceRule::determine_concurrency_strategy()
  └─ ConsequenceRule::validate_membership()

RUNTIME (Difference Pole)
  ├─ Computer { init(), step(), finalize() }
  ├─ PropertyValues { read(), iterator() }
  ├─ StorageRuntime { read(), write(), flush() }
  └─ ProcedureFacade { N-API, TypeScript, CLI }

TRANSFORM (Ground)
  └─ trait Transform<D, R> { fn project(d: &D) → R }

eval ∘ factory = Complete Projection
```

## Essential Imports

```rust
// Omniscience (Knowledge)
use rust_gds::projection::codegen::eval::{Eval, FunctionEval, EvalError};

// Omnipotence (Power)
use rust_gds::projection::codegen::factory::{Factory, FunctionFactory, FactoryError};

// Core concepts
use rust_gds::projection::codegen::{
    ComputationDescriptor, ComputationMembership,
    PropertyDescriptor, StorageDescriptor,
    consequence::ConsequenceRule,
};
```

## Basic Pattern

```rust
// 1. Define a descriptor (Identity)
let membership = ComputationMembership::new(
    vec![ValueType::Long],
    vec![StorageLayout::Chunked],
);
let descriptor = ComputationDescriptor::new(
    1,
    "my_algorithm",
    ComputationSpecies::Bsp,
    ComputationPattern::VertexCentric,
    membership,
);

// 2. Analyze it (Omniscience)
let schema = eval_analyzer.analyze(&descriptor)?;
// schema now contains what we KNOW about this descriptor

// 3. Create runtime from schema (Omnipotence)
let runtime = runtime_factory.create(&schema)?;
// runtime is now the concrete manifestation

// 4. Validate constraints (Consequence)
ConsequenceRule::validate_membership(&descriptor)?;
let strategy = ConsequenceRule::determine_concurrency_strategy(&descriptor);
```

## Key Queries

```rust
// Can this descriptor accept this value type?
descriptor.accepts(&ValueType::Long)

// Does this descriptor optimize for this layout?
descriptor.optimizes_for(&StorageLayout::Chunked)

// What concurrency model is required?
descriptor.required_concurrency()
```

## File Organization

```
src/projection/codegen/
├── mod.rs                          # Five-Fold organization
├── consequence.rs                  # ConsequenceRule
├── eval.rs                         # Eval trait & FunctionEval
├── factory.rs                      # Factory trait & FunctionFactory
├── transforms/                     # Transform implementations (existing)
├── descriptors/
│   ├── computation.rs              # ComputationDescriptor with membership
│   ├── property.rs                 # PropertyDescriptor
│   └── storage.rs                  # StorageDescriptor
└── runtimes/
    ├── computation.rs              # Computer trait
    ├── storage.rs                  # StorageRuntime trait
    └── procedure.rs                # ProcedureFacade
```

## The Two-Fold Application Flow

```
Input: Descriptor (what IS)
   ↓
eval.analyze(descriptor)
   ↓ (Omniscience: Knowledge extraction)
   ↓
Output: Schema (what we KNOW)
   ↓
factory.create(schema)
   ↓ (Omnipotence: Power of manifestation)
   ↓
Output: Runtime (what SHALL BE)
```

## Design Principles

1. **Descriptors are data** — Immutable, Serializable, no runtime discovery
2. **Membership is inherence** — All constraints encoded in descriptor
3. **Consequence is deterministic** — Same membership → same runtime
4. **eval ≠ factory** — Clean separation: analyze vs. create
5. **Transform is root** — Everything flows from unified principle

## What NOT to Do

❌ Create runtimes without eval first
❌ Modify descriptors after registration
❌ Use runtime reflection for strategy selection
❌ Hide membership constraints in factory logic
❌ Create ambiguous consequence rules

## What TO Do

✅ Use descriptors as compile-time constants
✅ Encode all constraints in membership fields
✅ Query descriptors to discover capabilities
✅ Use eval to extract schema
✅ Use factory to materialize runtime
✅ Validate membership with ConsequenceRule

## The Absolute Idea

```
Projection = Omniscience ∘ Omnipotence
           = Knowledge unified with Power
           = The Architecture of Being Itself

eval(descriptor) + factory(schema) = Complete projection
knowledge + power = maya overcome
```

## Testing

```bash
# Check compilation
cargo check --lib

# Run projection tests
cargo test --lib projection::codegen

# All tests should pass ✓
```

## Status

| Phase | Status         | What                          |
| ----- | -------------- | ----------------------------- |
| I     | ✓ COMPLETE     | Define Five-Fold Concept      |
| II    | ⏳ IN PROGRESS | Apply with eval + factory     |
| III   | 📋 PLANNED     | Realize with full integration |

## Next Steps

1. Integrate with eval! macro
2. Extend Membership with more constraints
3. Complete Pipeline orchestration
4. Verify runtime generation
5. Document final phase

---

**Remember:** Projection is not just code. It is the principle by which reality itself organizes. When this is complete, you will have encoded the Architecture of Being Itself into Rust.

The stakes are high. 🙏
