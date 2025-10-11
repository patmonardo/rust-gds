# PropertyDescriptor → ProgramDescriptor: The Dharmic Recognition 🕉️

**Date**: 2025-10-10  
**Context**: Recognition that PropertyDescriptor is misnamed - it's actually the Program  
**Key Insight**: "Algorithm + Data Structure = Programs" (Niklaus Wirth, 1976)

---

## The Recognition

> "property_descriptor is such a poor term. it is out of context. it is not a Property that projects extremes, it is a Dharma and that is really a Relation of Properties ... ie a Program!"

> "Algorithm Plus DataStructure == Programs. a ProgramDescriptor is what Projects Extremes and it is really a Collection of Properties."

**This is the correct naming!** 🎯

---

## The Misunderstanding

We called it **PropertyDescriptor** thinking it described a single property, but:

- It's the **CENTER** of the Five-Fold Brahmachakra
- It's the **Unity** that projects into Computation and Storage
- It's not ONE property, it's the **PROGRAM** (Algorithm + Data Structure)
- It's a **Collection of Properties** working together
- It's the **Dharma** (धर्म) - the law/duty/relation that governs the system

---

## The Correct Naming: ProgramDescriptor

### What a Program IS (Wirth, 1976)

```
Programs = Algorithms + Data Structures
```

### What ProgramDescriptor IS (Five-Fold Brahmachakra)

```
ProgramDescriptor = Collection<PropertyDescriptor>
                  = { properties[], algorithm, structure }
                  = THE DHARMA (the governing relation)
```

### The Projection

```
        ProgramDescriptor (Dharma/Unity)
                  ॐ
                  |
         +--------+--------+
         |                 |
    Computation        Storage
    (Algorithm)     (Data Structure)
         |                 |
    +----+----+       +----+----+
    |         |       |         |
 Descriptor Runtime  Descriptor Runtime
  (WHAT)   (HOW)     (WHAT)    (HOW)
```

**ProgramDescriptor projects into BOTH extremes simultaneously.**

---

## The Renaming Strategy

### Phase 1: Core Rename (PropertyDescriptor → ProgramDescriptor)

**Files to rename**:

1. **src/projection/property_descriptor.rs** → **src/projection/program_descriptor.rs**

   - Rename `PropertyDescriptor` struct → `ProgramDescriptor`
   - Keep `PropertyId`, `FieldDescriptor`, `StructDescriptor` (these ARE about individual properties)
   - Add `properties: Vec<PropertyDescriptor>` field (collection)
   - Add `algorithm_hint: Option<String>` field
   - Add `structure_hint: Option<String>` field

2. **Update all imports and usages**:
   - `PropertyDescriptor` → `ProgramDescriptor` (the unity/center)
   - Keep lower-level property types unchanged

### Phase 2: Semantic Clarity

**The hierarchy should be**:

```rust
// Individual property metadata (leaf level)
pub struct PropertyDescriptor {
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
}

// Program metadata (unity level) - THE DHARMA
pub struct ProgramDescriptor {
    pub name: String,
    pub properties: Vec<PropertyDescriptor>,  // Collection of properties
    pub algorithm_hint: Option<String>,        // The algorithm part
    pub structure_hint: Option<String>,        // The data structure part
}
```

**The projection contexts should hold ProgramDescriptor**:

```rust
pub struct ComputationContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub program: &'a ProgramDescriptor,        // ← The unity
    pub computation: &'a ComputationDescriptor, // ← The computation extreme
    pub node_count: usize,
}

pub struct StorageContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub program: &'a ProgramDescriptor,        // ← The unity
    pub storage: &'a StorageDescriptor,        // ← The storage extreme
    pub node_count: usize,
}
```

---

## The Philosophical Grounding

### Dharma (धर्म)

In Sanskrit philosophy:

- **Dharma** = law, duty, righteousness, **that which upholds**
- It's the **governing principle** of a system
- It's the **relation** between parts that makes them whole
- It's not ONE thing, it's the **pattern** that organizes things

### Program as Dharma

A **program** is:

- Not just algorithm OR data structure
- Not just one property OR another
- But the **RELATION** between algorithm and data structure
- The **COLLECTION** of properties working together
- The **GOVERNING LAW** of the computation

**ProgramDescriptor = The Dharma of the computation** 🕉️

### The Wirth Insight (1976)

> "Algorithms + Data Structures = Programs"

**This is the disjunctive substrate!**

- **Algorithm** = Computation (process/subtle)
- **Data Structure** = Storage (matter/gross)
- **Program** = The RELATION (unity/dharma)

**Programs are not sums, they're RELATIONS.** The `+` is not arithmetic addition but **synthesis**.

---

## The Current Misalignment

### What We Have Now

```rust
// src/projection/property_descriptor.rs
pub struct PropertyDescriptor {  // ← WRONG NAME
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    // ...
}

// Used in contexts
pub struct ComputationContext<'a> {
    pub property_descriptor: &'a PropertyDescriptor,  // ← Misleading
    // ...
}
```

**Problem**: This looks like it describes ONE property, but it's actually the CENTER of everything.

### What We Should Have

```rust
// src/projection/program_descriptor.rs
pub struct PropertyDescriptor {  // ← Keep this for individual properties
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    // ...
}

pub struct ProgramDescriptor {   // ← NEW: The unity/dharma
    pub name: String,
    pub properties: Vec<PropertyDescriptor>,  // Collection
    pub algorithm_hint: Option<String>,
    pub structure_hint: Option<String>,
}

// Used in contexts
pub struct ComputationContext<'a> {
    pub program: &'a ProgramDescriptor,  // ← Clear: the whole program
    pub computation: &'a ComputationDescriptor,  // ← The computation extreme
    // ...
}
```

**Solution**: Clear hierarchy and naming.

---

## Implementation Plan

### Step 1: Create program_descriptor.rs with new structure

````rust
// src/projection/program_descriptor.rs

use crate::types::ValueType;
use serde::{Deserialize, Serialize};

pub type PropertyId = u32;
pub type StructId = u32;

/// Field descriptor inside a Struct/UDT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub name: String,
    pub value_type: ValueType,
    pub offset: u16,
}

/// Descriptor for a user-defined struct (UDT)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructDescriptor {
    pub id: StructId,
    pub name: String,
    pub fields: Vec<FieldDescriptor>,
}

/// Storage hint for property backends
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StorageHint {
    FixedWidth,
    VariableLength,
    ListAsOffsets,
    ColumnarStruct,
    SerializedRow,
}

/// Individual property descriptor (leaf level)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
}

impl PropertyDescriptor {
    pub fn new(id: PropertyId, name: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            id,
            name: name.into(),
            value_type,
            nullable: true,
            storage_hint: StorageHint::VariableLength,
        }
    }

    pub fn with_storage_hint(mut self, hint: StorageHint) -> Self {
        self.storage_hint = hint;
        self
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
}

/// Program descriptor - The Dharma (Unity) that projects into extremes.
///
/// A program is the RELATION between Algorithm and Data Structure (Wirth, 1976).
/// This is the CENTER of the Five-Fold Brahmachakra - the unity that projects
/// into Computation (algorithm/process) and Storage (data structure/matter).
///
/// # The Dharma (धर्म)
///
/// In Sanskrit philosophy, Dharma is the governing law, the principle that upholds.
/// A ProgramDescriptor is the Dharma of a computation - the collection of properties
/// and their relations that define what the program IS.
///
/// # Five-Fold Structure
///
/// ```text
///         ProgramDescriptor (Dharma/Unity)
///                   ॐ
///                   |
///          +--------+--------+
///          |                 |
///     Computation        Storage
///     (Algorithm)     (Data Structure)
/// ```
///
/// The program projects into both extremes simultaneously:
/// - **Computation**: HOW to compute (algorithm)
/// - **Storage**: WHERE to store (data structure)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramDescriptor {
    /// Name of the program/algorithm
    pub name: String,

    /// Collection of properties involved in this program
    pub properties: Vec<PropertyDescriptor>,

    /// Hint about the algorithm (e.g., "pagerank", "louvain")
    pub algorithm_hint: Option<String>,

    /// Hint about data structure (e.g., "columnar", "sparse")
    pub structure_hint: Option<String>,
}

impl ProgramDescriptor {
    /// Create a new program descriptor.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            properties: Vec::new(),
            algorithm_hint: None,
            structure_hint: None,
        }
    }

    /// Add a property to the program.
    pub fn with_property(mut self, property: PropertyDescriptor) -> Self {
        self.properties.push(property);
        self
    }

    /// Set algorithm hint.
    pub fn with_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.algorithm_hint = Some(algorithm.into());
        self
    }

    /// Set data structure hint.
    pub fn with_structure(mut self, structure: impl Into<String>) -> Self {
        self.structure_hint = Some(structure.into());
        self
    }

    /// Get a property by name.
    pub fn get_property(&self, name: &str) -> Option<&PropertyDescriptor> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get a property by ID.
    pub fn get_property_by_id(&self, id: PropertyId) -> Option<&PropertyDescriptor> {
        self.properties.iter().find(|p| p.id == id)
    }
}
````

### Step 2: Update contexts to use ProgramDescriptor

```rust
// src/projection/computation_runtime.rs
pub struct ComputationContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub program: &'a ProgramDescriptor,          // ← Changed from property_descriptor
    pub computation: &'a ComputationDescriptor,  // ← Keep computation extreme
    pub node_count: usize,
}

// src/projection/storage_runtime.rs
pub struct StorageContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub program: &'a ProgramDescriptor,          // ← Changed from property_descriptor
    pub storage: &'a StorageDescriptor,          // ← Keep storage extreme
    pub node_count: usize,
}
```

### Step 3: Update mod.rs structure

````rust
// src/projection/mod.rs

//! Projection layer: The Five-Fold Brahmachakra
//!
//! ```text
//!         ProgramDescriptor (Dharma/Unity)
//!                   ॐ
//!                   |
//!          +--------+--------+
//!          |                 |
//!     Computation        Storage
//!     (Algorithm)     (Data Structure)
//!          |                 |
//!     +----+----+       +----+----+
//!     |         |       |         |
//! Descriptor Runtime  Descriptor Runtime
//!  (WHAT)    (HOW)     (WHAT)    (HOW)
//! ```

pub mod program_descriptor;         // 1. Unity (Dharma)
pub mod computation_descriptor;     // 2. Computation Identity
pub mod computation_runtime;        // 3. Computation Difference
pub mod storage_descriptor;         // 4. Storage Identity
pub mod storage_runtime;            // 5. Storage Difference

// Re-exports
pub use program_descriptor::{
    FieldDescriptor, PropertyDescriptor, PropertyId, ProgramDescriptor,
    StorageHint, StructDescriptor, StructId,
};
// ... rest
````

### Step 4: Update all usage sites

**Files to update**:

- All test files in `tests/`
- All example files in `examples/`
- Documentation in `doc/`

**Pattern**:

```rust
// OLD
use rust_gds::projection::PropertyDescriptor;
let prop = PropertyDescriptor::new(0, "pagerank", ValueType::Double);

// NEW
use rust_gds::projection::{ProgramDescriptor, PropertyDescriptor};
let property = PropertyDescriptor::new(0, "pagerank", ValueType::Double);
let program = ProgramDescriptor::new("PageRank")
    .with_property(property)
    .with_algorithm("pagerank")
    .with_structure("columnar");
```

---

## The Semantic Clarity

### Before (Confusing)

```rust
PropertyDescriptor  // Is this one property? The collection? The whole program?
```

### After (Clear)

```rust
PropertyDescriptor  // ← Individual property metadata (leaf)
ProgramDescriptor   // ← The whole program (unity/dharma)
                    //   = Collection of properties
                    //   = Algorithm + Data Structure
                    //   = The RELATION
```

---

## The eval! Macro Impact

### Current (Implicit)

```rust
eval! {
    property: { name: "pagerank", type: double },
    // ... generates PropertyDescriptor (confusing)
}
```

### Future (Explicit)

```rust
eval! {
    program: {
        name: "PageRank",
        properties: [
            { name: "pagerank", type: double },
            { name: "iterations", type: long },
        ],
        algorithm: "pagerank",
        structure: "columnar",
    },
    computation: { ... },
    storage: { ... },
}
```

**The macro generates the COMPLETE Five-Fold structure:**

1. **ProgramDescriptor** (unity) - The dharma
2. **ComputationDescriptor** (identity) - What computation IS
3. **Computer** impl (difference) - How computation EXECUTES
4. **StorageDescriptor** (identity) - What storage IS
5. **StorageRuntime** impl (difference) - How storage EXECUTES

---

## The Philosophical Payoff

### Wirth's Equation (1976)

```
Programs = Algorithms + Data Structures
```

### Our Five-Fold Structure

```
ProgramDescriptor (Dharma)
    = Relation(Algorithm, DataStructure)
    = Relation(Computation, Storage)
    = Unity that projects into extremes
```

### The Recognition

**A program is not a sum, it's a RELATION.**

The `+` in Wirth's equation is not arithmetic addition but **synthesis** (Hegelian Aufhebung):

- **Thesis**: Algorithm (Computation)
- **Antithesis**: Data Structure (Storage)
- **Synthesis**: Program (The Relation/Dharma)

**ProgramDescriptor captures this synthesis.** 🕉️

---

## Migration Checklist

- [ ] Create `src/projection/program_descriptor.rs` with new structure
- [ ] Update `src/projection/mod.rs` imports and comments
- [ ] Update `src/projection/computation_runtime.rs` (ComputationContext)
- [ ] Update `src/projection/storage_runtime.rs` (StorageContext)
- [ ] Update `src/projection/computation_descriptor.rs` (if needed)
- [ ] Update `src/projection/storage_descriptor.rs` (if needed)
- [ ] Update all test files (`tests/*.rs`)
- [ ] Update all example files (`examples/*.rs`)
- [ ] Update documentation (`doc/*.md`)
- [ ] Run `cargo test --lib projection`
- [ ] Run `cargo test`
- [ ] Run examples
- [ ] Update eval! macro (future)

---

## Summary

**The Recognition**: PropertyDescriptor is misnamed. It's not about one property, it's about the **PROGRAM** - the collection of properties and their governing relation (Algorithm + Data Structure).

**The Solution**: Rename to **ProgramDescriptor** (the Dharma/Unity) and clarify the hierarchy:

- **PropertyDescriptor** = Individual property metadata (leaf level)
- **ProgramDescriptor** = The whole program (unity level)
  - Collection of properties
  - Algorithm hint
  - Data structure hint
  - The RELATION that projects into Computation and Storage

**The Payoff**: Clear semantics, correct philosophy, and alignment with Wirth's foundational insight.

**Back to the 70s**: "Algorithm + Data Structure = Programs" 🌿💨📚

---

> "it is not a Property that projects extremes, it is a Dharma and that is really a Relation of Properties ... ie a Program!"

**Tat Tvam Asi** (तत् त्वम् असि) - Thou Art That 🕉️
