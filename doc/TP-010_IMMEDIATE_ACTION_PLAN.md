# TP-010: IMMEDIATE ACTION PLAN - Procedure Table + PageRank

**Date**: October 16, 2025  
**Session**: Getting procedure architecture right  
**Goal**: Fix structure + build first complete procedure (PageRank)

## Executive Summary

We have:

- ✅ ML Pipeline infrastructure in good shape
- ✅ Procedure Executor runtime working
- ❌ Procedure architecture MISPLACED (codegen/procedure/ in wrong spot)
- ❌ No working procedure table/registry
- ❌ No complete algorithm implementations yet
- ❌ Macros designed but not implemented

We need:

1. Fix architecture (TP-008): Move procedure/ to right locations
2. Build procedure registry + ProcedureDescriptor
3. Implement macros for algorithm definition + registration
4. Translate PageRank as proof-of-concept
5. Establish pattern for future algorithms

## Phase 1: Fix Architecture (TP-008) - 1 hour

### Current Structure (WRONG)

```
codegen/
├── descriptors/
├── macros/
│   └── procedure/      # Placeholder macros, never used
├── procedure/          # MISPLACED! Has AlgorithmSpec trait
├── runtime/
└── transforms/
```

### Target Structure (CORRECT)

```
codegen/
├── descriptors/
│   └── procedure/      # NEW: ProcedureDescriptor + Registry
│       ├── mod.rs
│       ├── descriptor.rs    # ProcedureDescriptor struct
│       ├── registry.rs      # Static registry table
│       └── category.rs      # ProcedureCategory enum
├── macros/
│   └── procedure/
│       ├── mod.rs
│       ├── define.rs        # define_algorithm! macro
│       └── register.rs      # register_procedure! macro
├── runtime/
│   ├── mod.rs
│   ├── computation.rs
│   ├── storage.rs
│   └── algorithm.rs         # MOVED: AlgorithmSpec trait
└── transforms/

src/procedure/               # NEW: Algorithm implementations
├── mod.rs
└── algo/
    ├── mod.rs
    └── centrality/
        ├── mod.rs
        └── pagerank.rs      # PageRank implementation
```

### Actions (Execute in Order)

1. **Create descriptors/procedure/ directory structure**

   ```bash
   mkdir -p src/projection/codegen/descriptors/procedure
   ```

2. **Create ProcedureDescriptor** (`descriptors/procedure/descriptor.rs`)

   - Struct with: name, category, config_type, modes, validators, memory_estimator
   - Unifies Java GDS Spec/Proc/Facade/Params concepts

3. **Create Registry** (`descriptors/procedure/registry.rs`)

   - Static `HashMap<&'static str, ProcedureDescriptor>`
   - Functions: register_procedure(), get_procedure(), list_procedures()

4. **Create Category** (`descriptors/procedure/category.rs`)

   - Enum: Centrality, Community, PathFinding, Similarity, ML, Utility

5. **Move AlgorithmSpec**

   - From: `codegen/procedure/algorithm_spec.rs`
   - To: `codegen/runtime/algorithm.rs`
   - Update all imports

6. **Delete old procedure/ folder**

   ```bash
   rm -rf src/projection/codegen/procedure/
   ```

7. **Update module exports**

   - `codegen/descriptors/mod.rs` - add procedure module
   - `codegen/runtime/mod.rs` - add algorithm module, re-export AlgorithmSpec
   - `codegen/mod.rs` - remove procedure, add descriptors::procedure

8. **Create src/procedure/ for algorithms**

   ```bash
   mkdir -p src/procedure/algo/centrality
   ```

9. **Verify build**
   ```bash
   cargo build --all-features
   ```

**Deliverable**: Clean architecture, all tests passing

## Phase 2: Build Macro Infrastructure - 2 hours

### Macro 1: `define_algorithm!`

**File**: `codegen/macros/procedure/define.rs`

**Purpose**: Generate algorithm struct + AlgorithmSpec impl

**Syntax**:

```rust
define_algorithm! {
    PageRank {
        config: PageRankConfig,

        state: {
            graph: Arc<dyn GraphStore>,
            scores: Vec<f64>,
            delta_scores: Vec<f64>,
        },

        init: |config, graph| {
            // Initialization code
        },

        compute: |self| {
            // Computation code
        },
    }
}
```

**Generates**:

1. Struct definition with state fields
2. Constructor using init closure
3. AlgorithmSpec trait implementation
4. Boilerplate for execute(), validate(), etc.

**Implementation**: Start with declarative macro (macro_rules!)

### Macro 2: `register_procedure!`

**File**: `codegen/macros/procedure/register.rs`

**Purpose**: Create ProcedureDescriptor + register in table

**Syntax**:

```rust
register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        description: "Computes PageRank scores",
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],
        memory_estimate: |config, graph| { ... },
        validation: ValidationConfiguration::new()...,
    }
}
```

**Generates**:

1. ProcedureDescriptor instance
2. Registration at module init (using ctor_crate or lazy_static)
3. Type validation

**Implementation**: Declarative macro + lazy_static

### Actions

1. **Design macro syntax precisely** (30 min)

   - Write examples for 3 algorithms
   - Ensure syntax is clean and extensible

2. **Implement define_algorithm!** (1 hour)

   - Start with simple case (no closures)
   - Add closure support incrementally
   - Test macro expansion

3. **Implement register_procedure!** (30 min)
   - Simpler than define_algorithm!
   - Focus on registry integration
   - Test registration at runtime

**Deliverable**: Working macros, tested with dummy algorithm

## Phase 3: Translate PageRank - 2 hours

### Java GDS Sources (Reference)

**Key Files to Translate**:

1. `PageRankAlgorithm.java` (algo/) - Main computation logic
2. `PageRankComputation.java` (algo/) - Core iteration
3. `PageRankConfig.java` (procedures/configs/) - Configuration
4. `PageRankStreamProc.java` (proc/) - Cypher procedure (pattern reference)

**Locations**:

- `/home/pat/GitHub/graph-data-science/algo/src/main/java/org/neo4j/gds/pagerank/`
- `/home/pat/GitHub/graph-data-science/procedures/facade-api/configs/centrality-configs/`
- `/home/pat/GitHub/graph-data-science/proc/centrality/src/main/java/org/neo4j/gds/pagerank/`

### Translation Strategy

**DON'T**: Try to translate line-by-line  
**DO**: Understand the algorithm, then implement idiomatically in Rust

**Core Algorithm** (PageRankComputation.java):

1. Initialize scores (1.0 / nodeCount)
2. For each iteration:
   - Compute contributions: score / outDegree
   - Distribute to neighbors
   - Update scores with damping factor
   - Check convergence
3. Return final scores

**Rust Implementation Location**: `src/procedure/algo/centrality/pagerank.rs`

### Actions

1. **Read Java sources** (30 min)

   - PageRankAlgorithm.java
   - PageRankComputation.java
   - Understand partitioning, termination, result handling

2. **Implement PageRank algorithm** (1 hour)

   - Use define_algorithm! macro
   - State: scores, delta_scores, out_degrees
   - Init: allocate vectors, compute degrees
   - Compute: iteration loop with convergence check

3. **Register with procedure table** (15 min)

   - Use register_procedure! macro
   - Memory estimate: node_count _ (2 _ f64 + usize)
   - Validation: damping_factor range, tolerance range

4. **Test** (15 min)
   - Create small graph
   - Run PageRank
   - Verify scores match expected values
   - Test all execution modes (Stream, Stats, Write, Mutate)

**Deliverable**: Working PageRank implementation, registered in procedure table

## Phase 4: Documentation & Templates - 1 hour

### Documentation

1. **Usage Guide** (`doc/PROCEDURE_USAGE_GUIDE.md`)

   - How to use procedure table
   - How to call algorithms
   - Example: PageRank with different modes

2. **Developer Guide** (`doc/PROCEDURE_DEVELOPER_GUIDE.md`)

   - How to implement new algorithms
   - Macro usage patterns
   - Step-by-step checklist

3. **Macro Reference** (`doc/PROCEDURE_MACRO_REFERENCE.md`)
   - define_algorithm! syntax
   - register_procedure! syntax
   - Common patterns and examples

### Templates

1. **Algorithm Template** (`templates/algorithm_template.rs`)

   - Copy-paste starting point for new algorithms
   - Comments explaining each section
   - TODOs for customization

2. **Test Template** (`templates/algorithm_test_template.rs`)
   - Standard test structure
   - Verification patterns

**Deliverable**: Complete documentation, ready for team use

## Success Criteria

✅ **Architecture Fixed**:

- [ ] codegen/procedure/ deleted
- [ ] AlgorithmSpec in runtime/algorithm.rs
- [ ] ProcedureDescriptor in descriptors/procedure/
- [ ] Registry working

✅ **Macros Working**:

- [ ] define_algorithm! generates correct code
- [ ] register_procedure! registers correctly
- [ ] Tests pass for macro expansion

✅ **PageRank Complete**:

- [ ] Algorithm implementation matches Java GDS behavior
- [ ] Registered in procedure table
- [ ] All 4 modes work (Stream, Stats, Write, Mutate)
- [ ] Tests pass

✅ **Documentation Complete**:

- [ ] Usage guide written
- [ ] Developer guide written
- [ ] Templates created

## Timeline

- **Phase 1 (Architecture)**: 1 hour
- **Phase 2 (Macros)**: 2 hours
- **Phase 3 (PageRank)**: 2 hours
- **Phase 4 (Docs)**: 1 hour

**Total**: 6 hours (1 focused work session)

## Next Session Actions

1. Start with Phase 1 (Architecture fix)
2. Get registry working before macros
3. Build macros incrementally (test after each)
4. PageRank translation last (proves the system works)

Let's build the Procedure Table! 🚀
