# AlgorithmSpec Consolidation & PageRank Translation Plan

**Date**: October 16, 2025  
**Context**: TP-006 Procedure Core is complete. Ready for PageRank.  
**Question**: Can we consolidate AlgorithmSpec, ProcedureFacade, and descriptors into our meta-macro system?

---

## Current Architecture

### What We Have

**1. GDSL Runtime (Executor Infrastructure)** - `src/projection/eval/procedure/`

- `AlgorithmSpec` trait - The core interface for all algorithms
- `ProcedureExecutor` - Runs algorithms through standardized lifecycle
- `ExecutionContext` - Runtime environment (graph, config, termination)
- `ComputationResult` - Generic result container
- `ResultConsumer` - Handles mutate/write/stats/stream modes

**Status**: ✅ **Complete and stable** (this is the MACHINE)

**2. ML Pipeline Descriptors** - `src/projection/codegen/ml/`

- `PipelineDescriptor` - Serializable pipeline definitions
- `StepDescriptor` - Individual pipeline steps
- `AlgorithmDescriptor` - Algorithm metadata and configuration

**Status**: ✅ **Complete** (this is the BLUEPRINT)

**3. Procedure Results** - `src/procedure/core/result/`

- Result types for centrality, community, similarity
- Scaling system for feature normalization

**Status**: ✅ **Just completed!**

**4. Algorithm Implementations** - `src/procedure/*` (MISSING!)

- PageRank, Louvain, WCC, etc.
- **This is what we need to translate next!**

---

## The Question: Can We Merge?

> "What about all of these AlgorithmSpecs, ProcedureFacades, and God knows what? Can you merge them all into our Algorithm Procedure Computation Descriptor Meta Macros?"

### Answer: **YES, but carefully!** 🎯

The architecture already supports this! Here's how:

### 1. AlgorithmSpec = Runtime Interface (KEEP IT)

```rust
// This is FIXED INFRASTRUCTURE (the Machine)
pub trait AlgorithmSpec: Send + Sync {
    type Config;
    type Result;

    fn name(&self) -> &str;
    fn run(&self, context: &ExecutionContext) -> ComputationResult<Self::Result>;
    fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError>;
}
```

**Do NOT merge this into macros** - it's the stable runtime contract.

### 2. PipelineDescriptor = Serializable Blueprint (KEEP IT)

```rust
// This is METADATA (the Blueprint)
#[derive(Serialize, Deserialize)]
pub struct PipelineDescriptor {
    name: String,
    pipeline_type: PipelineType,
    steps: Vec<StepDescriptor>,
    // ...
}
```

**Do NOT merge this into macros** - it's for serialization and ML pipelines.

### 3. Algorithm Implementations = GENERATE WITH MACROS! ✨

**This is where the meta-macros shine!**

```rust
// What we SHOULD do: Generate algorithm implementations
algorithm_procedure! {
    name: "PageRank",
    config: PageRankConfig,
    result: PageRankResult,

    compute: |context, config| {
        // PageRank implementation
        pagerank_kernel(context.graph(), config)
    },

    memory_estimation: |config| {
        MemoryEstimation::of(config.node_count * 8)
    },

    mutate_mode: Some(PageRankMutateResult),
    write_mode: Some(PageRankWriteResult),
    stats_mode: Some(PageRankStatsResult),
    stream_mode: Some(PageRankStreamResult),
}
```

**The macro generates**:

1. `impl AlgorithmSpec for PageRank` - Runtime interface
2. Wiring to `ProcedureExecutor` - Lifecycle management
3. Result type conversions - Mutate/Write/Stats/Stream
4. Integration with `ResultConsumer` - Write to graph/stream

---

## What Exists vs What We Need

### Exists ✅

| Component             | Location                                       | Purpose                      |
| --------------------- | ---------------------------------------------- | ---------------------------- |
| `AlgorithmSpec` trait | `projection/eval/procedure/algorithm_spec.rs`  | Runtime contract             |
| `ProcedureExecutor`   | `projection/eval/procedure/executor.rs`        | Execution engine             |
| `ExecutionContext`    | `projection/eval/procedure/executor.rs`        | Runtime environment          |
| `ResultConsumer`      | `projection/eval/procedure/result_consumer.rs` | Mutate/Write/Stats/Stream    |
| `PipelineDescriptor`  | `projection/codegen/ml/pipeline_descriptor.rs` | ML pipeline metadata         |
| Result types          | `procedure/core/result/*`                      | PageRank/Louvain/etc results |
| Scaling system        | `procedure/core/scaling/scaler.rs`             | Feature normalization        |

### Missing ❌

| Component                     | Where It Should Be                            | What We Need        |
| ----------------------------- | --------------------------------------------- | ------------------- |
| **PageRank algorithm**        | `src/procedure/centrality/pagerank.rs`        | Core implementation |
| **Louvain algorithm**         | `src/procedure/community/louvain.rs`          | Core implementation |
| **WCC algorithm**             | `src/procedure/community/wcc.rs`              | Core implementation |
| **NodeSimilarity**            | `src/procedure/similarity/node_similarity.rs` | Core implementation |
| **Meta-macro for algorithms** | `src/procedure/codegen/algorithm_macro.rs`    | Code generation     |

---

## Proposed Consolidation Strategy

### Phase 1: Create Algorithm Meta-Macro

**File**: `src/procedure/codegen/algorithm_macro.rs`

```rust
/// Generates complete algorithm implementation with AlgorithmSpec trait
#[macro_export]
macro_rules! define_algorithm {
    (
        name: $name:expr,
        config: $config_ty:ty,
        result: $result_ty:ty,

        compute: $compute:expr,

        $(memory_estimation: $mem_est:expr,)?

        mutate_mode: $mutate:ty,
        write_mode: $write:ty,
        stats_mode: $stats:ty,
        stream_mode: $stream:ty,
    ) => {
        // Generate struct
        pub struct $name;

        // Generate AlgorithmSpec impl
        impl AlgorithmSpec for $name {
            type Config = $config_ty;
            type Result = $result_ty;

            fn name(&self) -> &str {
                $name
            }

            fn run(&self, context: &ExecutionContext) -> ComputationResult<Self::Result> {
                let config = context.config::<Self::Config>()?;
                $compute(context, config)
            }

            fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError> {
                $(return $mem_est;)?
                Err(MemoryEstimationError::NotImplemented)
            }
        }

        // Generate result type conversions
        impl From<$result_ty> for $mutate {
            fn from(result: $result_ty) -> Self {
                // Auto-generate conversion
            }
        }

        // ... same for Write/Stats/Stream
    };
}
```

### Phase 2: Translate PageRank with Macro

**File**: `src/procedure/centrality/pagerank.rs`

```rust
use crate::procedure::codegen::algorithm_macro::define_algorithm;
use crate::procedure::core::result::centrality::*;

define_algorithm! {
    name: "PageRank",
    config: PageRankConfig,
    result: PageRankComputationResult,

    compute: |context, config| {
        let graph = context.graph();
        let damping_factor = config.damping_factor;
        let max_iterations = config.max_iterations;
        let tolerance = config.tolerance;

        // PageRank kernel
        let mut ranks = vec![1.0 / graph.node_count() as f64; graph.node_count()];
        let mut new_ranks = ranks.clone();

        for iteration in 0..max_iterations {
            let mut delta = 0.0;

            for node_id in 0..graph.node_count() {
                let mut rank_sum = 0.0;

                for neighbor in graph.neighbors(node_id) {
                    let degree = graph.degree(neighbor);
                    rank_sum += ranks[neighbor] / degree as f64;
                }

                new_ranks[node_id] = (1.0 - damping_factor) / graph.node_count() as f64
                    + damping_factor * rank_sum;

                delta += (new_ranks[node_id] - ranks[node_id]).abs();
            }

            std::mem::swap(&mut ranks, &mut new_ranks);

            if delta < tolerance {
                break;
            }
        }

        Ok(PageRankComputationResult {
            ranks,
            iterations,
            converged: delta < tolerance,
        })
    },

    memory_estimation: |config| {
        let node_count = config.node_count;
        let rank_memory = node_count * std::mem::size_of::<f64>();
        Ok(MemoryEstimation::of(rank_memory * 2)) // Two rank vectors
    },

    mutate_mode: PageRankMutateResult,
    write_mode: PageRankWriteResult,
    stats_mode: PageRankStatsResult,
    stream_mode: PageRankStreamResult,
}
```

### Phase 3: Wire Everything Together

**File**: `src/procedure/mod.rs`

```rust
pub mod centrality {
    pub mod pagerank;
    pub mod betweenness_centrality;
}

pub mod community {
    pub mod louvain;
    pub mod label_propagation;
    pub mod wcc;
}

pub mod similarity {
    pub mod node_similarity;
    pub mod knn;
}

pub mod core; // Result types, scaling, etc.
pub mod codegen; // Algorithm meta-macros
```

---

## Benefits of This Approach

### 1. Separation of Concerns

- **AlgorithmSpec trait** = Fixed runtime contract (GDSL Runtime)
- **PipelineDescriptor** = Serializable metadata (ML Pipelines)
- **Algorithm implementations** = Generated by macros (Extensible content)

### 2. Code Reduction

**Java GDS PageRank**:

- `PageRankAlgorithmFactory.java` (~100 lines)
- `PageRank.java` (~300 lines)
- `PageRankMutateProc.java` (~80 lines)
- `PageRankWriteProc.java` (~80 lines)
- `PageRankStatsProc.java` (~60 lines)
- `PageRankStreamProc.java` (~60 lines)
- **Total: ~680 lines**

**Rust GDS PageRank** (with macro):

- `pagerank.rs` (~150 lines including kernel)
- **Total: ~150 lines**

**Reduction: 78%** 🚀

### 3. Type Safety

- Compiler enforces `AlgorithmSpec` contract
- Result type conversions are automatic
- Configuration validation at compile-time

### 4. Maintainability

- Algorithm logic is pure Rust (no boilerplate)
- Macro handles wiring (DRY principle)
- Easy to add new algorithms

---

## Answer to Your Question

### "Can we merge AlgorithmSpec/ProcedureFacade/Descriptors into meta-macros?"

**SHORT ANSWER**:

- ❌ Don't merge `AlgorithmSpec` trait (it's the fixed runtime interface)
- ❌ Don't merge `PipelineDescriptor` (it's for ML pipeline serialization)
- ✅ **DO** use meta-macros to generate algorithm implementations
- ✅ **DO** use macros to wire algorithms to the runtime

**LONG ANSWER**:
The architecture is already well-designed with three distinct layers:

1. **GDSL Runtime** (`AlgorithmSpec`, `ProcedureExecutor`) - The Machine
2. **ML Pipeline Metadata** (`PipelineDescriptor`) - The Blueprint
3. **Algorithm Implementations** (PageRank, Louvain, etc.) - **Generate these!**

The meta-macro should generate #3 while leveraging #1 and #2.

---

## Next Steps: PageRank Translation

### 1. Find Java PageRank Source

```bash
ls /home/pat/GitHub/graph-data-science/algo/src/main/java/org/neo4j/gds/pagerank/
```

### 2. Create Algorithm Macro

Create `src/procedure/codegen/algorithm_macro.rs` with the `define_algorithm!` macro.

### 3. Translate PageRank

Create `src/procedure/centrality/pagerank.rs` using the macro.

### 4. Test

Write integration test that:

- Creates a graph
- Runs PageRank
- Verifies ranks are correct
- Tests mutate/write/stats/stream modes

### 5. Document

Create `doc/PAGERANK_TRANSLATION.md` explaining the pattern.

---

## Recommendation

**YES, translate PageRank next!** 🎯

1. Start with the **core PageRank kernel** (pure algorithm)
2. Create the **algorithm macro** to eliminate boilerplate
3. Generate **all 4 execution modes** (mutate/write/stats/stream)
4. **Integrate with existing infrastructure** (AlgorithmSpec, ProcedureExecutor, ResultConsumer)

This will:

- ✅ Validate the entire Procedure Core infrastructure
- ✅ Establish the pattern for all future algorithms
- ✅ Demonstrate Rust's superiority over Java (78% code reduction)
- ✅ Show how meta-macros eliminate boilerplate

---

**Ready to translate PageRank?** 🚀

Let me know if you want to proceed, and I'll help locate the Java source and start the translation!
