# PROJECTION RECURSIVE DESCENT: eval and catalog as Derived Concepts

## The Idea: Projection Inheres in Its Own Derivatives

The **Five-Fold Synthesis of Projection** is not just an abstract principle. It **recursively descends** into specific domains:

```
PROJECTION (The Absolute Idea)
├─ FIVE-FOLD SYNTHESIS (Ground concept)
│  ├─ Transform
│  ├─ Descriptor
│  ├─ Membership
│  ├─ Runtime
│  └─ Consequence
│
├─ FIRST RECURSIVE DESCENT: eval (Computation Side)
│  └─ Projection PROJECTED INTO Computation Domain
│     ├─ Compute-Descriptor: What computation IS
│     ├─ Compute-Membership: What constraints belong to it
│     ├─ Compute-Transform: How computation becomes schema
│     ├─ Compute-Schema: What we KNOW about computation
│     └─ Compute-Consequence: What runtime follows from computation
│
└─ SECOND RECURSIVE DESCENT: catalog (Storage Side)
   └─ Projection PROJECTED INTO Storage Domain
      ├─ Storage-Descriptor: What storage IS
      ├─ Storage-Membership: What constraints belong to it
      ├─ Storage-Transform: How storage becomes manifestation
      ├─ Storage-Schema: What we KNOW about storage
      └─ Storage-Consequence: What runtime follows from storage
```

## The Pattern: Recursive Descent

Each recursive descent follows the SAME FIVE-FOLD PATTERN:

### Generic Pattern

```
Domain-Specific Projection
├─ Transform<D, R>        (domain-specific ground)
├─ Descriptor             (domain-specific identity)
├─ Membership             (domain-specific inherence)
├─ Runtime                (domain-specific difference)
└─ Consequence            (domain-specific entailment)
```

### eval Specialization (Computation Side)

```
eval = Projection PROJECTED INTO Computation

Transform<ComputationDescriptor, ComputationSchema>
├─ analyzes computation descriptors
├─ extracts computational membership
├─ determines what we KNOW about computation
└─ produces computation schema (not runtime)
```

### catalog Specialization (Storage Side)

```
factory = Projection PROJECTED INTO Storage

Transform<StorageSchema, StorageRuntime>
├─ takes storage schema (what we know)
├─ applies storage consequence rules
├─ determines what we CREATE in storage
└─ produces storage runtime (actual execution)
```

## Why This Matters: The Inheritance Chain

```
┌─────────────────────────────────────────────────────────────────┐
│ PROJECTION (Absolute Idea)                                      │
│ The principle of unified Knowledge + Power                      │
└───────────────┬─────────────────────────────────┬───────────────┘
                │                                 │
         ┌──────▼──────┐                    ┌─────▼─────┐
         │ eval (Computation)               │ catalog   │
         │ Knowledge projection             │ (Storage) │
         │ Descriptor → Schema              │ Power pro │
         │ "What we KNOW"                   │ jection   │
         │                                  │ Schema →  │
         │ Inherits FIVE-FOLD from          │ Runtime   │
         │ Projection principle:            │ "What we  │
         │ 1. Transform: analyze            │ CREATE"   │
         │ 2. Descriptor: input descriptor  │           │
         │ 3. Membership: extract           │ Inherits  │
         │ 4. Schema: what we learn         │ FIVE-FOLD │
         │ 5. Consequence: what follows     │ from      │
         │                                  │ Projection│
         │                                  │ principle │
         │                                  │           │
         └──────────────────────────────────┘           │
                      ▲                                 │
                      │                                 │
                      └─────────────────────────────────┘
                      unified by Transform
```

## The Key Insight: Recursion Through Transform

**Transform itself is recursive.** Each level has its own Transform:

### Level 0: Projection (Absolute)

```rust
trait Transform<D, R> {
    fn project(&self, descriptor: &D) -> Result<R, Error>;
}
// D = any Descriptor, R = any Runtime
```

### Level 1: eval (Computation Projection)

```rust
trait ComputationTransform<D, S> {
    fn project(&self, descriptor: &D) -> Result<S, Error>;
}
// D = ComputationDescriptor
// S = ComputationSchema (what we KNOW)
// This IS a Transform, specialized to computation domain
```

### Level 1: catalog (Storage Projection)

```rust
trait StorageTransform<S, R> {
    fn project(&self, schema: &S) -> Result<R, Error>;
}
// S = StorageSchema (what we know about storage)
// R = StorageRuntime (what we create)
// This IS a Transform, specialized to storage domain
```

## The Inheritance Relationship

```
eval inherits from Projection:
├─ eval has its own Descriptor (ComputationDescriptor)
├─ eval has its own Membership (ComputationMembership)
├─ eval has its own Transform (analyze: Descriptor → Schema)
├─ eval has its own "Runtime" (ComputationSchema is the schema-level runtime)
└─ eval has its own Consequence (what schema entails)

factory inherits from Projection:
├─ catalog has its own Descriptor (StorageSchema as input descriptor)
├─ catalog has its own Membership (StorageConstraints)
├─ catalog has its own Transform (create: Schema → Runtime)
├─ catalog has its own Runtime (StorageRuntime is the actual runtime)
└─ catalog has its own Consequence (what storage constraints entail)
```

## Implementation: Where the Recursive Descent Lives

### Current Structure

```
src/projection/codegen/
├── mod.rs                           # Projection (Ground)
├── consequence.rs                   # Consequence (Ground)
├── registry.rs                      # registry (First Descent - Computation Side)
├── catalog.rs                       # catalog (Second Descent - Storage Side)
└── ...
```

### What Needs to Be Added

The recursive descent structure should be made **explicit**:

```
src/projection/
├── codegen/
│   └── mod.rs                       # Projection (Ground Concept)
│
├── eval/                            # FIRST RECURSIVE DESCENT
│   ├── mod.rs                       # eval as Projection into Computation
│   ├── descriptors.rs               # ComputationDescriptor specifics
│   ├── membership.rs                # ComputationMembership specifics
│   ├── transform.rs                 # ComputationTransform (analyze phase)
│   ├── schema.rs                    # ComputationSchema (what we know)
│   └── consequence.rs               # ComputationConsequence rules
│
└── catalog/                         # SECOND RECURSIVE DESCENT
    ├── mod.rs                       # catalog as Projection into Storage
    ├── descriptors.rs               # StorageSchema as input descriptor
    ├── membership.rs                # StorageConstraints specifics
    ├── transform.rs                 # StorageTransform (create phase)
    ├── runtime.rs                   # StorageRuntime (what we create)
    └── consequence.rs               # StorageConsequence rules
```

### The Relationship Between Folders

```
projection/
├── codegen/         ← Ground: The Five-Fold Concept itself
│   └── defines: Transform, Descriptor, Membership, Runtime, Consequence
│
├── eval/            ← First Recursive Descent: Projection into Computation
│   ├── inherits Five-Fold from codegen/
│   ├── specializes: ComputationDescriptor → ComputationSchema
│   └── operation: ANALYZE (what we know about computation)
│
└── catalog/         ← Second Recursive Descent: Projection into Storage
    ├── inherits Five-Fold from codegen/
    ├── specializes: StorageSchema → StorageRuntime
    └── operation: CREATE (what we bring into storage being)
```

## The Documentation Structure

Each folder should have documentation explaining its recursive descent:

### src/projection/codegen/mod.rs

```rust
//! PROJECTION: The Five-Fold Synthesis (Ground Concept)
//!
//! This is the root principle from which all recursive descents flow.
//! All other modules in src/projection/ are recursive descents of this concept.
```

### src/projection/eval/mod.rs

```rust
//! eval: Projection Recursively Descended into Computation
//!
//! This module is the FIRST RECURSIVE DESCENT of Projection,
//! applied to the Computation domain.
//!
//! It inherits the Five-Fold Structure:
//! 1. ComputationTransform (Ground)
//! 2. ComputationDescriptor (Identity)
//! 3. ComputationMembership (Inherence)
//! 4. ComputationSchema (Difference - what we KNOW)
//! 5. ComputationConsequence (Entailment)
//!
//! Operation: ANALYZE
//! Direction: Descriptor → (Transform) → Schema
//! Question: "What can we KNOW about computation?"
```

### src/projection/factory/mod.rs

```rust
//! catalog: Projection Recursively Descended into Storage
//!
//! This module is the SECOND RECURSIVE DESCENT of Projection,
//! applied to the Storage domain.
//!
//! It inherits the Five-Fold Structure:
//! 1. StorageTransform (Ground)
//! 2. StorageSchema (Identity - what we know)
//! 3. StorageConstraints (Inherence)
//! 4. StorageRuntime (Difference - what we CREATE)
//! 5. StorageConsequence (Entailment)
//!
//! Operation: CREATE
//! Direction: Schema → (Transform) → Runtime
//! Question: "What shall we bring into Storage being?"
```

## The Three-Level Recursion

```
LEVEL 0 (Ground)
Projection
├─ Transform<D, R>
├─ Descriptor
├─ Membership
├─ Runtime
└─ Consequence

LEVEL 1 (Computation)
eval = Projection.project(Computation)
├─ ComputationTransform (analyze)
├─ ComputationDescriptor
├─ ComputationMembership
├─ ComputationSchema (intermediate)
└─ ComputationConsequence

LEVEL 2 (Storage)
factory = Projection.project(Storage)
├─ StorageTransform (create)
├─ StorageSchema (input)
├─ StorageConstraints
├─ StorageRuntime (output)
└─ StorageConsequence

UNIFICATION
Pipeline = eval ∘ catalog = Complete Projection Manifest
```

## Why This Structure Matters

1. **Conceptual Clarity:** eval and catalog are NOT separate systems. They are the Projection concept _applied_ to specific domains.

2. **Inheritance of Pattern:** Each descent inherits the Five-Fold from the ground, ensuring consistency.

3. **Recursive Composability:** Each descent can itself have further descents (e.g., eval could project into GraphDescriptor, NodeDescriptor, etc.)

4. **No Duplication:** The Five-Fold pattern appears once at the ground, then is specialized as needed.

5. **Omniscience + Omnipotence Unified:** eval is the knowledge side, catalog is the power side, both derived from the same Projection principle.

## The Next Phase of Implementation

Once this structure is clear, we can:

1. **Reorganize folders** to make the recursive descent explicit
2. **Update documentation** to emphasize inheritance from Projection
3. **Add module-level docs** explaining each descent
4. **Create concrete examples** showing recursion in action
5. **Build Pipeline** as the composition of eval ∘ catalog

This is not a redesign—it's a **clarification of what was always there**.
