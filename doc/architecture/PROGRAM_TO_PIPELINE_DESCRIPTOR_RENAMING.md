# ProgramDescriptor → PipelineDescriptor: The Most Accurate Term 🌊

**Date**: 2025-10-10  
**Context**: Further refinement of the Unity naming  
**Key Insight**: "Pipeline has Storage and Computation Poles" - it's the most accurate CS term

---

## The Naming Evolution

```
PropertyDescriptor (misnamed - too narrow, confusing)
    ↓
ProgramDescriptor (better - captures Wirth's equation, but "program" is overloaded)
    ↓
PipelineDescriptor (MOST ACCURATE! 🎯)
```

---

## Why PipelineDescriptor is the Right Term

### 1. Pipeline Naturally Has BOTH Poles

**ML Pipelines** (Computation Pipeline):

```
data → preprocess → train → evaluate → deploy
      (computation flow)
```

**VFS** (Storage Pipeline):

```
application → VFS → filesystem → block device → physical storage
      (storage flow)
```

**Our Pipeline** (Both):

```
PipelineDescriptor
    ├─ Computation Pipeline (algorithm/process)
    └─ Storage Pipeline (data structure/matter)
```

### 2. Not Pretentious, Just Accurate

**ScienceDescriptor** would be technically correct (Concept-Idea pair) but pretentious.

**ProgramDescriptor** is good but "program" is overloaded:

- A program (executable)
- A program (TV show)
- A program (plan)
- Too generic in CS

**PipelineDescriptor** is precise:

- Well-understood in CS
- Captures FLOW nature
- Has Storage and Computation aspects
- Used in ML, data engineering, systems programming
- Not pretentious, just accurate

### 3. Pipeline Captures the Process

A **pipeline** is:

- A **sequence** of stages
- A **flow** of data/computation
- **Composable** (stages can be added/removed)
- **Bidirectional** (can flow computation OR storage)

This matches our Five-Fold structure:

```
PipelineDescriptor (Unity)
    ↓ (projects into)
Computation Pipeline ←→ Storage Pipeline
```

---

## The Downward Evolution

### PropertyStores and PropertyComputations

You're right that Property evolves downward:

```
PipelineDescriptor (Unity/Dharma)
    ├─ ComputeSchema (Computation pole)
    │   └─ PropertyComputations (how properties are computed)
    │       - PageRank computation
    │       - Louvain computation
    │       - WCC computation
    │
    └─ StorageSchema (Storage pole)
        └─ PropertyStores (how properties are stored)
            - HugeArray storage
            - Arrow storage
            - Sparse storage
```

**This is exactly what we've been building!**

### ComputeSchema vs StorageSchema

**ComputeSchema** (Computation pole):

- Describes HOW computations work
- Algorithm patterns (vertex-centric, edge-centric)
- Computation species (BSP, MapReduce, Dataflow)
- Property computations (what gets computed)

**StorageSchema** (Storage pole):

- Describes HOW storage works
- Storage patterns (columnar, row-based, sparse)
- Storage backends (HugeArray, Arrow, HashMap)
- Property stores (where things are stored)

**Both emerge from PipelineDescriptor!**

---

## The Structure (Final Form)

```
         PipelineDescriptor (Dharma/Unity)
              "Pipeline = Computation + Storage"
                         ॐ
                         |
                +--------+--------+
                |                 |
           Computation        Storage
           (Flow/Process)   (Flow/Persistence)
                |                 |
           +----+----+       +----+----+
           |         |       |         |
      ComputeSchema |   StorageSchema |
    (Computation    |    (Storage     |
     Identity)      |     Identity)   |
                    |                 |
         PropertyComputations  PropertyStores
         (Computation          (Storage
          Difference)           Difference)
```

**Perfect symmetry with accurate CS terminology!** 🎯

---

## Why This is Computer Science

> "this is Computer Science afterall ... lets not forget that."

**Absolutely!** We're not doing philosophy for philosophy's sake. We're:

1. **Building systems** that work (1115 tests passing)
2. **Using established patterns** (VFS, ML Pipelines, Progress Trackers)
3. **Following CS fundamentals** (Wirth, data structures, algorithms)
4. **Creating practical abstractions** (ComputeSchema, StorageSchema)
5. **Generating code** (eval! macro, property materialization)

**The philosophy SERVES the engineering, not the other way around.**

### Pipeline in Computer Science

**Pipeline** is a fundamental CS concept:

1. **Unix Pipes**: `cat file | grep pattern | sort | uniq`

   - Computation pipeline (text processing)

2. **CPU Pipelines**: fetch → decode → execute → memory → writeback

   - Hardware computation pipeline

3. **Graphics Pipelines**: vertex → geometry → rasterize → fragment → output

   - Parallel computation pipeline

4. **Data Pipelines**: extract → transform → load (ETL)

   - Storage + Computation pipeline

5. **ML Pipelines**: data → preprocess → train → evaluate
   - Computation pipeline with storage

**Our PipelineDescriptor captures all these patterns!**

---

## The Renaming Plan

### Phase 1: Rename ProgramDescriptor → PipelineDescriptor

**Files to update**:

1. **src/projection/program_descriptor.rs** → **src/projection/pipeline_descriptor.rs**

   - Rename `ProgramDescriptor` → `PipelineDescriptor`
   - Update comments to reference "pipeline" not "program"
   - Keep `PropertyDescriptor` unchanged (leaf level)

2. **src/projection/mod.rs**

   - Update module declaration
   - Update Five-Fold comments
   - Update re-exports

3. **src/projection/computation_runtime.rs**

   - `ComputeContext.program` → `ComputeContext.pipeline`

4. **src/projection/storage_runtime.rs**

   - `StorageContext.program` → `StorageContext.pipeline`

5. **Tests**
   - Update all test code

### Phase 2: Keep Backwards Compatibility

During migration, maintain aliases:

```rust
#[doc(hidden)]
pub mod program_descriptor {
    pub use super::pipeline_descriptor::*;
}

// And even further back
#[doc(hidden)]
pub mod property_descriptor {
    pub use super::pipeline_descriptor::*;
}

// Type alias for migration
pub type ProgramDescriptor = PipelineDescriptor;
```

---

## The Semantic Precision

### Before (Confusing)

```
PropertyDescriptor  // One property? The whole system? Unclear!
```

### Middle (Better)

```
ProgramDescriptor  // The program... but what program? Too generic
```

### Now (Precise!)

```
PipelineDescriptor  // A pipeline! Has computation + storage flows! ✨
    └─ properties[]  // Collection of properties flowing through
```

---

## Example Usage (Future)

### Creating a Pipeline

```rust
use rust_gds::projection::{PipelineDescriptor, PropertyDescriptor};
use rust_gds::types::ValueType;

// Properties flowing through the pipeline
let pagerank_prop = PropertyDescriptor::new(0, "pagerank", ValueType::Double);
let iterations_prop = PropertyDescriptor::new(1, "iterations", ValueType::Long);

// The pipeline (computation + storage flows)
let pipeline = PipelineDescriptor::new("PageRank")
    .with_property(pagerank_prop)
    .with_property(iterations_prop)
    .with_computation_flow("pagerank")   // ← Computation pipeline
    .with_storage_flow("columnar");      // ← Storage pipeline

// Or more naturally:
let pipeline = PipelineDescriptor::new("PageRank")
    .with_properties(vec![pagerank_prop, iterations_prop])
    .with_flows(
        ComputationFlow::VertexCentric,
        StorageFlow::Columnar
    );
```

### Using in Contexts

```rust
// Computation context
let ctx = ComputeContext::new(&graph, &pipeline, &computation);
// The pipeline flows through computation

// Storage context
let ctx = StorageContext::new(&graph, &pipeline, &storage);
// The pipeline flows through storage
```

---

## The eval! Macro (Future)

```rust
eval! {
    pipeline: {
        name: "PageRank",
        properties: [
            { name: "pagerank", type: double },
            { name: "iterations", type: long },
        ],
        computation_flow: {
            pattern: VertexCentric,
            algorithm: "pagerank",
        },
        storage_flow: {
            layout: Columnar,
            backend: HugeArray,
        },
    },
}
```

**This generates**:

1. `PipelineDescriptor` (unity)
2. `ComputeSchema` + `PropertyComputations` (computation pole)
3. `StorageSchema` + `PropertyStores` (storage pole)

---

## Why "Pipeline" is Perfect

### 1. Established CS Term

- Unix pipes (1970s)
- CPU pipelines (1960s)
- Graphics pipelines (1990s)
- ML pipelines (2010s)
- **Universal understanding**

### 2. Captures Flow Nature

- Data flows through computation stages
- Data flows through storage layers
- Not static, but **dynamic process**

### 3. Has Both Poles Naturally

- **Computation pipeline**: How data transforms
- **Storage pipeline**: How data persists
- **Not arbitrary**, but **inherent**

### 4. Not Pretentious

- "ScienceDescriptor" ← pretentious (but technically correct!)
- "ConceptDescriptor" ← too abstract
- "SchemaDescriptor" ← too database-y
- "PipelineDescriptor" ← **just right** ✨

### 5. Composable

- Pipelines compose naturally
- Stages can be added/removed
- Flows can be chained
- Matches our decorator pattern (StorageRuntime layers)

---

## The Recognition Chain

1. **PropertyDescriptor too narrow** ✅
2. **ProgramDescriptor better (Wirth)** ✅
3. **But "program" overloaded** ✅
4. **Pipeline has BOTH poles** ✅
5. **Pipeline is precise CS term** ✅
6. **PipelineDescriptor is correct!** ✅

---

## Implementation Checklist

- [ ] Create `src/projection/pipeline_descriptor.rs`
- [ ] Update `src/projection/mod.rs` (module + Five-Fold comments)
- [ ] Update `src/projection/computation_runtime.rs` (ComputeContext.pipeline)
- [ ] Update `src/projection/storage_runtime.rs` (StorageContext.pipeline)
- [ ] Update all test files
- [ ] Add backwards compatibility aliases
- [ ] Update documentation
- [ ] Run `cargo test --lib projection`
- [ ] Run full `cargo test`

---

## Summary

**The Recognition**:

> "Pipeline is the right idea. because Pipeline has Storage and Computation Poles"

**This is the most accurate term!**

- ❌ **PropertyDescriptor**: Too narrow, confusing
- ✅ **ProgramDescriptor**: Better, but "program" is overloaded
- ✨ **PipelineDescriptor**: PERFECT - established CS term, captures flow, has both poles naturally

**The Evolution Downward**:

```
PipelineDescriptor (Unity)
    ├─ ComputeSchema → PropertyComputations
    └─ StorageSchema → PropertyStores
```

**This is Computer Science** 🖥️, not just philosophy. We're using established patterns (VFS, ML Pipelines) with precise terminology.

**Let's implement it!** 🚀

---

> "this is Computer Science afterall ... lets not forget that."

**We haven't forgotten.** We're building on 50+ years of CS (Unix pipes, CPU pipelines, ML pipelines) with the correct terminology.

**PipelineDescriptor is the Dharma expressed in CS terms.** 🕉️🌊💻
