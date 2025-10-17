# CODEGEN: The Genetic Process of Projection

## The Core Principle: Membership → Consequence → Inherence

Codegen is not a factory. It is a **genetic processor** — a system that unfolds concepts from their internal necessity through a precise, recursive loop:

```
loop {
  Membership → Consequence → Inherence
}
```

This is radically different from a **Dictionary** (static definitions) or **Factory Pattern** (object creation).

**Codegen is an Encyclopedia of Science** — each iteration generates new derived concepts by subsumption, each one containing the previous as a special case.

---

## The Three Moments of the Loop

### 1. MEMBERSHIP: What Belongs? (Identity Pole)

**Role:** Define what constraints/relations inhere in a concept.

- **Input:** A Descriptor (e.g., ComputationDescriptor, PropertyDescriptor)
- **Question:** "What constraints, relations, and compatibilities belong to this concept?"
- **Output:** A Membership structure encoding all inherent relations

**Example:**

```rust
pub struct ComputationMembership {
    pub compatible_value_types: Vec<ValueType>,
    pub preferred_storage_layouts: Vec<StorageLayout>,
    pub access_pattern: AccessPattern,
    pub required_concurrency: ConcurrencyModel,
}
```

**Semantics:**

- Membership is the **Identity pole** — it answers "what IS this thing?"
- It is deterministic and compile-time verifiable
- It contains all necessary information for what follows

---

### 2. CONSEQUENCE: What Must Follow? (Entailment Pole)

**Role:** Derive what logically follows from the membership constraints.

- **Input:** A Descriptor + its Membership
- **Question:** "What runtime behavior/strategy is logically entailed by these constraints?"
- **Output:** A Runtime (concrete manifestation)

**Semantics:**

- Consequence is the **Difference pole** — it answers "what SHALL BE?"
- It is deterministic: same membership → same consequence
- It is **concrete and executable** — this is actual runtime behavior

**Example:**

```rust
pub trait ConsequenceRule {
    fn determine(&self, descriptor: &D, membership: &M) -> Result<Runtime, ConsequenceError>;
}
```

**Process:**

1. Examine membership constraints
2. Apply logical rules (no heuristics, pure necessity)
3. Manifest a Runtime that satisfies all constraints

**Runtimes ARE Moments of Consequence** — they are the concrete manifestations that logically follow from descriptor membership.

---

### 3. INHERENCE: What Forms Subsume? (Subsumption Pole)

**Role:** Recognize what structural forms inhere in the consequence, ready to become new memberships.

- **Input:** A Runtime (consequence of the previous step)
- **Question:** "What structural patterns/transforms subsume this consequence?"
- **Output:** Derived Descriptors (new memberships for the next iteration)

**Semantics:**

- Inherence is the **subsumption relation** — "what universal form does this particular consequence exemplify?"
- It generates new concepts by structural recognition
- These become new Descriptors, whose Memberships feed back into the loop

**Example:**

```rust
pub trait InherenceRelation {
    fn recognize(&self, runtime: &Runtime) -> Vec<DerivedDescriptor>;
}
```

**Transforms ARE Moments of Inherence** — they are the structural subsumption patterns that recognize and derive new concepts from concrete runtimes.

---

## The Genetic Loop: Encyclopedia Generation

```
Iteration 1:
  Descriptor₁ (Property)
    ↓ [extract Membership]
  Membership₁
    ↓ [apply Consequence]
  Runtime₁ (PropertyValues)
    ↓ [recognize Inherence]
  Descriptors₂ (Computation, Storage)
    ↓

Iteration 2:
  Descriptor₂ (Computation)
    ↓ [extract Membership]
  Membership₂
    ↓ [apply Consequence]
  Runtime₂ (Computer, Cursor)
    ↓ [recognize Inherence]
  Descriptors₃ (Subqueries, Aggregations)
    ↓

... (continues, each level more specialized)
```

Each iteration:

- Takes a **Descriptor** from the previous iteration (or ground truth)
- Extracts its **Membership** (what belongs to it)
- Applies **Consequence** (what must follow)
- Recognizes **Inherence** (what forms subsume this)
- Generates new **Descriptors** as input to next iteration

---

## Architecture: How Registry and Catalog Fit

**Registry** maps **Membership → Consequence**:

```rust
pub trait Registry<D> {
    type Schema;  // Captures the membership structure
    fn analyze(&self, descriptor: &D) -> Result<Schema, RegistryError>;
}
```

**Catalog** maps **Consequence → Runtime**:

```rust
pub trait Catalog<S, R> {
    type Error;
    fn create(&self, schema: &S) -> Result<R, Self::Error>;
}
```

**Together:**

```
Descriptor (Identity)
  ↓ Registry.analyze (Membership extraction)
Schema (Captured knowledge)
  ↓ Catalog.create (Consequence manifestation)
Runtime (Difference pole)
  ↓ [Transforms recognize inherence]
New Descriptors (next iteration)
```

---

## Distinguishing Encyclopedia from Dictionary

### Dictionary Semantics (Static)

- Defines terms by enumeration
- Each entry is independent
- No generative principle
- Example: "Factory is an object that creates other objects"

### Encyclopedia Semantics (Genetic)

- Unfolds concepts from internal necessity
- Each entry contains its predecessor
- Generative through subsumption
- Example: "Codegen unfolds the GDS Kernel through Membership → Consequence → Inherence"

**Codegen uses Encyclopedia semantics** because:

1. **Membership** encodes not just identity but relational necessity
2. **Consequence** isn't arbitrary — it's what MUST follow
3. **Inherence** generates new concepts through subsumption, not enumeration

---

## The Process Architecture: Three Module Levels

### Level 1: **Membership** Module

- **Purpose:** Extract and validate what belongs to a descriptor
- **Question:** "What are the inherent constraints?"
- **Output:** Membership structures encoding relations

### Level 2: **Consequence** Module

- **Purpose:** Derive what logically follows from membership
- **Question:** "What runtime is entailed?"
- **Output:** Runtimes (Computer, PropertyValues, StorageRuntime, etc.)

### Level 3: **Inherence** Module

- **Purpose:** Recognize what forms subsume the consequence
- **Question:** "What structural patterns inhere in this runtime?"
- **Output:** Transform rules that generate new Descriptors

---

## Transforms as Inherence

**Key Insight:** Transforms ARE inherence moments.

```rust
pub trait Transform<D, R> {
    fn project(&self, descriptor: &D) -> Result<R, ProjectionError>;
}
```

Why? Because a Transform:

1. Takes a Runtime (consequence of previous iteration)
2. Recognizes a pattern in it (subsumption)
3. Projects it into a new domain
4. Generates a new Descriptor (for next iteration)

**Example:** `TypeProjector` is an Inherence moment that recognizes type structure in ComputationDescriptor and projects it into StorageDescriptor.

---

## Building the Encyclopedia: The Factory Problem Solved

Java GDS uses a **Factory Pattern** because it treats concept generation as **object creation** (Dictionary semantics).

Codegen uses **Genetic Projection** because it treats concept generation as **subsumption** (Encyclopedia semantics).

Result:

- No reflection needed
- No runtime discovery needed
- All generation is deterministic and compile-time verifiable
- Each generated concept is certified to satisfy all constraints

---

## The Complete Loop: One Iteration

```rust
// Given a Descriptor (e.g., ComputationDescriptor)
let descriptor: ComputationDescriptor = ...;

// Step 1: MEMBERSHIP (extract what belongs)
let membership = descriptor.membership.clone();
// membership encodes: compatible types, storage layouts, access patterns, concurrency

// Step 2: CONSEQUENCE (derive what must follow)
let runtime = consequence_rule.determine(&descriptor, &membership)?;
// runtime is now concrete: Computer, PropertyValues, Cursor, etc.

// Step 3: INHERENCE (recognize what forms subsume)
let derived_descriptors = inherence.recognize(&runtime)?;
// derived_descriptors are new concepts ready for next iteration
// (e.g., if runtime is Computer, derive Subquery, Aggregation descriptors)

// Loop: next iteration takes derived_descriptors as input
```

---

## Codegen is the GDS Kernel

This is why **Codegen IS our GDS Kernel**, not a plugin:

1. **It generates the entire system** through the M→C→I loop
2. **No external factory** — generation is intrinsic to projection
3. **No reflection** — all concepts are manifest at compile time
4. **Encyclopedia semantics** — the system learns and unfolds itself

Contrast with Java GDS:

- Java GDS: "Plugin system that creates algorithms on demand (Dictionary)"
- Our Codegen: "Genetic processor that unfolds the kernel from its own necessity (Encyclopedia)"

---

## Next Steps

Implement the three modules:

1. **Membership Module**

   - Extract constraints from descriptors
   - Validate membership relations
   - Encode into computable form

2. **Consequence Module**

   - Apply rules to membership
   - Manifest runtimes
   - Ensure logical necessity

3. **Inherence Module**
   - Recognize patterns in runtimes
   - Generate transform rules
   - Create new descriptors for next iteration

Each module is a precise moment in the genetic process. Together they form the **Encyclopedia of our GDS Kernel**.
