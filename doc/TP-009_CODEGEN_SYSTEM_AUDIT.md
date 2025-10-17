# TP-009: Codegen System Audit & Strategy

**Date**: October 16, 2025  
**Status**: Planning - Understanding what we have vs what we need

## The Big Picture

You're right - I misunderstood. The Codegen system is about **raising infrastructure into Value Consciousness** - projecting Java GDS infrastructure (ML Pipeline, Procedure, Config) into Rust's type system with macro assistance.

## Current State: What We Actually Have

### 1. Config System (`src/config/`) - **WORKING & USED**

**The Working Macro**: `generate_config!` in `codegen/macros/config.rs`

**Where It's Used**:

- `config/model_config.rs` - ML model configurations
- `config/pregel_config.rs` - BSP computation config
- `config/morph_config.rs` - Container/morphism configs (3 types)

**What It Generates**:

- Config struct with fields
- Builder struct with fluent API
- Default implementation
- Validation method calling custom validators
- `build()` that validates and returns `Result<Config, ConfigError>`

**Example Pattern**:

```rust
crate::generate_config!(
    PregelConfig, PregelConfigBuilder,
    validate = pregel_validate,
    {
        max_iterations: usize = 10;
        concurrency: usize = 4;
        relationship_weights: Option<String> = None;
    }
);
```

### 2. Config System (`src/config/`) - **HAND-WRITTEN CONFIGS**

**Production Configs** (NOT macro-generated):

- `algo_config.rs` - PageRankConfig, LouvainConfig, NodeSimilarityConfig, BetweennessCentralityConfig
- `graph_config.rs` - GraphCreateConfig, PropertyConfig, RandomGraphGeneratorConfig, RelationshipsBuilderConfig
- `io_config.rs` - FileExporterConfig, FileImporterConfig, DatabaseExporterConfig, DatabaseImporterConfig
- `graphstore_config.rs` - GraphStoreBackendConfig, BackendChoice

**Pattern**: Hand-written builder pattern with:

- Struct definition
- Default impl
- Builder struct
- Builder methods
- `build()` with validation
- `validate()` method

**Why Hand-Written?**:

- More complex validation logic
- Composed configs (nested builders)
- Enum variants (BackendChoice)
- Rich documentation
- Full control over API

### 3. ML Descriptors (`codegen/descriptors/ml/`) - **HAND-WRITTEN STRUCTS**

**Files**:

- `pipeline.rs` - PipelineDescriptor, PipelineType, TrainingConfig, ModelCandidate
- `model.rs` - ModelDescriptor
- `step.rs` - StepDescriptor, FeatureStepDescriptor, NodePropertyStepDescriptor
- `training.rs` - TrainingDescriptor

**Purpose**: Describe ML workflow structure (WHAT it IS)

**Pattern**: Plain structs with serde derives

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDescriptor {
    pub name: String,
    pub pipeline_type: PipelineType,
    pub steps: Vec<StepDescriptor>,
    pub training_config: TrainingConfig,
    pub config: PipelineConfig,
    pub metadata: PipelineMetadata,
}
```

**Why No Macros?**: These are data structures, not builders. Clean and simple.

### 4. Runtime Contracts (`codegen/runtime/`) - **HAND-WRITTEN TRAITS**

**Files**:

- `computation.rs` - Computer, ComputeStep, ComputeContext (HOW computation executes)
- `storage.rs` - StorageRuntime, StorageAccessor, StorageContext (HOW storage executes)

**Purpose**: Runtime execution contracts (HOW things RUN)

**Why No Macros?**: These are trait definitions. Maybe helper macros could reduce boilerplate in impls.

### 5. The Unused Macro Folder (`codegen/macros/procedure/`) - **ASPIRATIONAL JUNK**

**Files**:

- `config.rs` - `algorithm_config!` macro (NEVER USED - 191 lines of fantasy)
- `algorithm.rs` - `define_algorithm!` macro (NEVER USED - probably similar)
- `mod.rs` - Documentation of what these SHOULD do

**The Problem**: These are placeholder demonstrations that were never integrated.

**Why They Exist**: Good intentions - "we should have macros for algorithms!"

**Why They're Not Used**: The hand-written patterns work fine and give us control.

### 6. The Mysterious `eval_macro.rs` - **PHILOSOPHICAL PLACEHOLDER**

**File**: `codegen/macros/eval_macro.rs` - `value_type_table!` macro

**Purpose** (from docs): "Master Projector" for property types - project compile-time schema into both Gross (storage) and Subtle (runtime) worlds.

**Status**: NEVER USED - only in doc examples

**The Vision**: Single source of truth for property type projection:

```rust
value_type_table! {
    Long {
        id: 1,
        value_type: ValueType::Long,
        storage_hint: StorageHint::Dense,
        rust_type: i64,
        gross_adapter: LongPropertyValues,
        subtle_impl: LongGdsValue,
    },
    // ...
}
```

**Why Not Used**: We have working property system already. This is a "someday" unification.

## The Gap: What's Missing

### 1. Algorithm Registration

**Need**: Macro to register algorithms with:

- Name, category, description
- Config type binding
- Validation requirements
- Execution modes (stream, stats, write, mutate)
- Memory estimation

**Current State**: Hand-written in `procedure/` infrastructure

**Macro Opportunity**:

```rust
define_algorithm! {
    PageRank {
        name: "pageRank",
        category: "Centrality",
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],
        validation: |config, graph| { ... },
        memory_estimate: |config, graph| { ... },
    }
}
```

### 2. ML Pipeline Component Registration

**Need**: Macro to register ML components:

- Models (LogisticRegression, RandomForest, etc.)
- Feature steps (NodeProperty, Degree, etc.)
- Training algorithms

**Current State**: Partially in descriptors, no macro support

### 3. Computation Registration

**Need**: Macro for BSP-style computation registration

**Current State**: Some support in runtime traits, could use macro sugar

## The Strategy: What Macros SHOULD Do

### Core Principle: **Reduce Repetition, Not Replace Structure**

Macros should:

1. ✅ **Eliminate boilerplate** (builder patterns, registration code)
2. ✅ **Enforce consistency** (all algorithms register the same way)
3. ✅ **Enable codegen** (read Java, emit Rust via macros)
4. ❌ **NOT replace** hand-written structs when they're clear
5. ❌ **NOT complicate** simple data structures

### Macro Use Cases (Ranked by Value)

1. **HIGH VALUE**: Algorithm registration

   - Reduces 100+ lines of boilerplate per algorithm
   - Ensures consistent registry API
   - Makes adding new algorithms trivial

2. **HIGH VALUE**: Config builders (already working!)

   - `generate_config!` is proven useful
   - Could extend for more complex validation

3. **MEDIUM VALUE**: Property type projection

   - `value_type_table!` could unify property system
   - But current system works, so not urgent

4. **MEDIUM VALUE**: Computation registration

   - BSP computers could use macro sugar
   - But trait impls are already clean

5. **LOW VALUE**: Descriptor generation
   - ML descriptors are fine as plain structs
   - Adding macros would obscure, not clarify

## Tomorrow's Work: Understand WHAT Codegen Does

Before we can fix the codegen structure, we need to answer:

1. **What is Projection?**

   - You said: "raising infrastructure into Value Consciousness"
   - Need to map: Java GDS → Rust types → Runtime execution
   - The Codegen system is the bridge

2. **What are the Five-Fold layers?**

   - descriptors/ = WHAT (Identity/Science)
   - runtime/ = HOW (Difference/Manifestation)
   - macros/ = AUTOMATION (Code generation)
   - transforms/ = TRANSLATION (Between representations)
   - procedure/ = ??? (Misplaced - should be part of descriptors/runtime)

3. **How does Config fit?**

   - Configs are runtime parameters
   - Used by runtime execution contracts
   - Generated by macros (partially)
   - Need clear flow: Descriptor → Config → Runtime

4. **What's the relationship to Java GDS?**
   - We have "A LOT of GDS Java ML Pipeline and Procedure Infrastructure"
   - Need to understand: What gets translated? What gets projected?
   - The Codegen system should make this translation mechanical

## Questions for Tomorrow

1. What does "raising into Value Consciousness" mean practically?
2. How should Descriptors → Runtime flow work?
3. Where does the Config system fit in Five-Fold?
4. What would a perfect algorithm registration macro look like?
5. Should procedure/ move to descriptors/procedure/ or runtime/algorithm.rs? (TP-008)
6. What is transforms/ actually for? (Not investigated yet)

## Current Understanding: The Architecture

```
Java GDS (Source)
    ↓
Codegen System (Projection/Translation)
    ├── descriptors/    → WHAT things ARE (data structures)
    ├── runtime/        → HOW things EXECUTE (traits)
    ├── macros/         → AUTOMATION (reduce boilerplate)
    ├── transforms/     → ??? (not investigated)
    └── procedure/      → MISPLACED (should be split)
    ↓
Runtime Execution (Target)
    ├── Config          → Parameters for execution
    ├── Validation      → Ensure correctness
    └── Execution       → Actual graph computation
```

## Immediate Actions (Tomorrow)

1. ✅ Read transforms/ to understand its role
2. ✅ Map the complete flow: Java → Codegen → Runtime → Execution
3. ✅ Document what "Projection" means in this context
4. ✅ Understand Five-Fold Brahmachakra in practical terms
5. ✅ Design proper algorithm registration macro
6. ✅ Fix procedure/ misplacement (TP-008)
7. ✅ Write clear strategy for macro expansion

## The Key Insight

You're right - I was dismissing the macros because I didn't understand **the vision**:

**Codegen isn't about generating code for its own sake.**

**Codegen is about PROJECTION - taking Java GDS infrastructure and raising it into Rust's value-conscious type system.**

The macros are the **tools** that make this projection mechanical, consistent, and complete.

Tomorrow we clarify the vision, then implement it.

Good night. 🙏
