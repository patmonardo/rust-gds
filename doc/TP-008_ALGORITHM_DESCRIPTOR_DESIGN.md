# TP-008: Algorithm Descriptor Architecture Design

**Date**: October 16, 2025  
**Purpose**: Design proper descriptor architecture for algorithm procedures  
**Context**: User identified that algorithm_spec should be AlgorithmDescriptor

---

## The Problem User Identified

> "procedure folder is out of place and I say it belongs under descriptor  
> but that means we should begin with a design for algorithm_descriptor  
> that combines Specs/Facades/Procs from the Java GDS Proc system"

**Status**: ✅ **ABSOLUTELY CORRECT OBSERVATION**

---

## Java GDS Architecture (Source of Truth)

### The Three Layers

```
Java GDS Procedure System:

1. *Proc.java (Neo4j Procedure Layer)
   └── @Context GraphDataScienceProcedures facade
   └── @Procedure annotations
   └── Stream<ResultType> methods

2. *BusinessFacade.java (Application Layer)
   └── Orchestrates algorithm execution
   └── Handles modes (Stream, Write, Mutate, Stats, Estimate)
   └── Validation, estimation, result consumption

3. AlgorithmSpec<ALGO, ALGO_RESULT, CONFIG, RESULT, ALGO_FACTORY>
   └── THE DESCRIPTOR (what algorithm IS)
   └── algorithmFactory()
   └── newConfigFunction()
   └── computationResultConsumer()
   └── validationConfig()
```

### Example: PageRank

```java
// 1. Proc (Neo4j layer)
public class PageRankStreamProc {
    @Context
    public GraphDataScienceProcedures facade;

    @Procedure("gds.pageRank.stream")
    public Stream<PageRankResult> stream(String graphName, Map<String, Object> config) {
        return facade.algorithms().centrality().pageRankStream(graphName, config);
    }
}

// 2. Facade (Application layer)
public class CentralityAlgorithmsStreamModeBusinessFacade {
    public Stream<PageRankResult> pageRankStream(String graphName, Map<String, Object> config) {
        // Orchestrates execution using AlgorithmSpec
    }
}

// 3. AlgorithmSpec (Descriptor layer)
public class PageRankAlgorithmSpec implements AlgorithmSpec<...> {
    public String name() { return "pageRank"; }
    public AlgorithmFactory algorithmFactory() { ... }
    public NewConfigFunction newConfigFunction() { ... }
    public ComputationResultConsumer computationResultConsumer() { ... }
}
```

---

## What "AlgorithmDescriptor" Should Be

### Current State (WRONG!)

```
codegen/procedure/algorithm_spec.rs  ← Trait only, not a descriptor!
```

**Problem**:

- It's a TRAIT (contract to implement)
- Not a DESCRIPTOR (struct describing an algorithm)
- Missing: Factory, Config builder, Result consumer, Validation

### Correct State (AlgorithmDescriptor)

```rust
// codegen/descriptors/procedure/algorithm.rs

pub struct AlgorithmDescriptor {
    /// Algorithm metadata
    pub id: u32,
    pub name: &'static str,
    pub category: AlgorithmCategory,

    /// Configuration descriptor
    pub config_descriptor: ConfigDescriptor,

    /// Computation descriptor (how algorithm computes)
    pub computation: ComputationDescriptor,

    /// Storage requirements
    pub storage: StorageDescriptor,

    /// Supported execution modes
    pub modes: ExecutionModes,

    /// Validation rules
    pub validation: ValidationDescriptor,

    /// Factory function pointer
    pub factory: AlgorithmFactory,
}

pub enum AlgorithmCategory {
    Centrality,
    Community,
    PathFinding,
    Similarity,
    ML,
    // ...
}

pub struct ExecutionModes {
    pub stream: bool,
    pub write: bool,
    pub mutate: bool,
    pub stats: bool,
}

// The trait becomes HOW to execute what the descriptor describes
pub trait Algorithm {
    type Output;
    type Config;

    fn compute(&self, graph: &Graph, config: &Self::Config) -> Self::Output;
}
```

---

## The Correct Architecture

### Five-Fold Structure (Corrected)

```
src/projection/codegen/
├── macros/
│   └── procedure/
│       ├── algorithm.rs        → generate_algorithm_descriptor! macro
│       └── config.rs           → algorithm_config! macro
│
├── descriptors/
│   ├── property.rs             → THE CENTER
│   ├── computation.rs
│   ├── storage.rs
│   ├── pipeline.rs
│   ├── ml/ (ML descriptors)
│   └── procedure/              → NEW! Algorithm descriptors
│       ├── algorithm.rs        → AlgorithmDescriptor struct
│       ├── config.rs           → ConfigDescriptor
│       ├── validation.rs       → ValidationDescriptor
│       └── registry.rs         → Algorithm registry
│
├── runtime/
│   ├── computation.rs
│   ├── storage.rs
│   └── algorithm.rs            → NEW! Algorithm trait (execution contract)
│
├── transforms/
│   └── ...
│
└── eval/
    └── procedure/
        ├── executor.rs         → Executes Algorithm trait impls
        ├── facade.rs           → Business facade (mode routing)
        └── proc.rs             → Proc layer (future: Neo4j integration)
```

---

## Migration Path

### Phase 1: Create AlgorithmDescriptor

1. Create `descriptors/procedure/` directory
2. Move `procedure/algorithm_spec.rs` → `descriptors/procedure/algorithm.rs`
3. Rename `AlgorithmSpec` trait → `Algorithm` trait
4. Create `AlgorithmDescriptor` struct (the REAL descriptor)
5. Move trait to `runtime/algorithm.rs`

### Phase 2: Update Macro

```rust
// macros/procedure/algorithm.rs

generate_algorithm_descriptor! {
    name: PageRank,
    category: Centrality,

    config: PageRankConfig {
        damping_factor: f64 = 0.85,
        max_iterations: usize = 20,
    },

    computation: {
        species: Bsp,
        pattern: VertexCentric,
    },

    storage: {
        hint: Dense,
        properties: ["pagerank"],
    },

    modes: [stream, write, mutate, stats],

    validation: {
        before_load: [
            node_property_exists("seedProperty"),
        ],
        after_load: [
            graph_has_nodes,
        ],
    },
}

// This generates:
// 1. AlgorithmDescriptor static instance
// 2. PageRankConfig struct + builder
// 3. Validation configuration
// 4. Mode wrappers (StreamPageRank, WritePageRank, etc.)
```

### Phase 3: Registry Pattern

```rust
// descriptors/procedure/registry.rs

lazy_static! {
    static ref ALGORITHM_REGISTRY: RwLock<HashMap<u32, AlgorithmDescriptor>> =
        RwLock::new(HashMap::new());
}

pub fn register_algorithm(descriptor: AlgorithmDescriptor) {
    ALGORITHM_REGISTRY.write().unwrap().insert(descriptor.id, descriptor);
}

pub fn get_algorithm_descriptor(id: u32) -> Option<AlgorithmDescriptor> {
    ALGORITHM_REGISTRY.read().unwrap().get(&id).cloned()
}
```

---

## Resolving the Naming Issues

### Issue 1: "procedure" module location

**Current (WRONG)**:

```
codegen/procedure/              ← Top-level (out of place!)
└── algorithm_spec.rs
```

**Correct**:

```
codegen/descriptors/procedure/  ← Under descriptors (descriptor pattern!)
├── algorithm.rs                → AlgorithmDescriptor struct
├── config.rs                   → ConfigDescriptor
├── validation.rs               → ValidationDescriptor
└── registry.rs                 → Registry functions
```

### Issue 2: algorithm_spec vs algorithm_descriptor

**Old Name**: `AlgorithmSpec` (trait)  
**Problem**: "Spec" is vague - is it a trait? a descriptor? both?

**New Names**:

- `AlgorithmDescriptor` (struct) - WHAT the algorithm IS
- `Algorithm` (trait) - HOW to execute it

### Issue 3: Where does Algorithm trait live?

**NOT** in `descriptors/` (descriptors are structs, not traits)  
**YES** in `runtime/algorithm.rs` (runtime contracts)

```rust
// runtime/algorithm.rs

pub trait Algorithm: Send + Sync {
    type Output: Send + Sync;
    type Config;

    fn compute(&self, graph: &Graph, config: &Self::Config) -> Self::Output;
}

// Executor uses this trait
impl ProcedureExecutor {
    pub fn execute<A: Algorithm>(&self, algo: A, config: A::Config) -> Result<A::Output> {
        // ...
    }
}
```

---

## Java GDS Mapping

| Java GDS                     | Rust GDS                         | Location                              |
| ---------------------------- | -------------------------------- | ------------------------------------- |
| AlgorithmSpec<...> interface | AlgorithmDescriptor struct       | `descriptors/procedure/algorithm.rs`  |
| Algorithm<RESULT> interface  | Algorithm trait                  | `runtime/algorithm.rs`                |
| AlgorithmFactory             | Factory fn pointer in descriptor | `descriptors/procedure/algorithm.rs`  |
| NewConfigFunction            | Config builder methods           | Generated by macro                    |
| ComputationResultConsumer    | Result consumer methods          | `eval/procedure/facade.rs`            |
| ValidationConfiguration      | ValidationDescriptor             | `descriptors/procedure/validation.rs` |
| \*Proc.java classes          | Future: proc module              | `eval/procedure/proc.rs`              |
| \*BusinessFacade.java        | ProcedureFacade                  | `eval/procedure/facade.rs`            |

---

## Benefits of This Design

### 1. Clear Separation

- **Descriptors** (descriptors/procedure/) = WHAT algorithms ARE
- **Runtime** (runtime/algorithm.rs) = HOW to execute
- **Eval** (eval/procedure/) = WHERE execution happens

### 2. Registry Pattern

- All algorithms registered at compile-time
- Easy discovery: "what algorithms are available?"
- Catalog integration: list algorithms, show metadata

### 3. Macro Generation

- Single declarative spec generates everything
- No boilerplate
- Type-safe

### 4. Java GDS Compatibility

- Clear mapping from Java concepts
- Easy to translate procedures
- Same architecture, idiomatic Rust

---

## Implementation Plan

### Step 1: Fix Naming Collisions First (Blocking)

Before we can do this refactor, we need to fix:

- ✅ Pipeline naming collision
- ✅ eval/ml/ broken imports

### Step 2: Create New Structure

1. Create `descriptors/procedure/` directory
2. Design `AlgorithmDescriptor` struct
3. Move algorithm trait to `runtime/algorithm.rs`
4. Update macros to generate descriptors

### Step 3: Migrate Existing Code

1. Update imports
2. Update documentation
3. Update examples

### Step 4: Registry Integration

1. Implement registration system
2. Add catalog support
3. Add discovery APIs

---

## Questions to Resolve

1. **Should we do this refactor NOW or after fixing current issues?**

   - Option A: Fix pipeline/ml issues first, then refactor
   - Option B: Do it all at once (risky!)

2. **How should Algorithm trait look?**

   - Simple trait like current AlgorithmSpec?
   - More complex with lifecycle methods?

3. **What goes in AlgorithmDescriptor?**

   - Minimal (just metadata)?
   - Rich (includes validation, factory, etc.)?

4. **Should we support dynamic algorithm registration?**
   - Compile-time only (macro-generated)?
   - Runtime registration (plugin system)?

---

## User's Vision (My Understanding)

You want:

1. ✅ `descriptors/procedure/` not top-level `procedure/`
2. ✅ `AlgorithmDescriptor` struct (not just trait)
3. ✅ Combines Spec + Facade + Proc concerns from Java GDS
4. ✅ Clear mapping to Java GDS architecture
5. ✅ Registry pattern for algorithm discovery

**Is this correct?** Please confirm or clarify!

---

## Next Steps

**Immediate** (fixes current mess):

1. Fix pipeline naming collision
2. Fix eval/ml/ broken imports
3. Verify build with all features

**Then** (clean refactor): 4. Design AlgorithmDescriptor struct 5. Create descriptors/procedure/ module 6. Migrate algorithm_spec → algorithm descriptor 7. Update macros and registry

---

**Status**: ⏸️ WAITING FOR USER CONFIRMATION  
**Question**: Should I fix current issues first, or design AlgorithmDescriptor now?
