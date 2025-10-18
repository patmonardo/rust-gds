# Weekend Comprehension Guide: Procedures Architecture

**Date**: October 17, 2025  
**Purpose**: Deep understanding of Procedures subsystem before Pipeline work  
**Scope**: AlgorithmSpec trait → Executor → Computation → Storage  
**Time**: Entire weekend  
**Outcome**: Ready to implement Pipelines with confidence

---

## The Codegen Sophistication You've Built

This is the most complex codegen in rust-gds:

```
Procedures (Most Sophisticated)
├── AlgorithmSpec trait (generic container for algorithms)
├── ProcedureExecutor (orchestration)
├── Multiple computation backends
│   ├── Direct (Sum: single-pass, in-memory accumulation)
│   ├── Pregel (PageRank: iterative message-passing)
│   └── [Future: MapReduce-like, Stream processing, etc.]
├── Storage subsystem (Property columns)
└── Result consumption (Stream/Stats/Write modes)

Versus earlier sophistication:
Concurrency (excellent: Arc, Mutex, RwLock patterns, proven)
Collections (solid: Iterator machinery, cursor patterns)
Graphs (foundational: GraphStore trait, projections)
```

The Procedures system is **meta-algorithmic**: it doesn't implement one algorithm, it provides a framework for **any algorithm to be implemented correctly**.

---

## Three Layers to Comprehend This Weekend

### Layer 1: The Contract (AlgorithmSpec Trait)

**File**: `src/projection/eval/procedure/algorithm_spec.rs` (~518 lines)

**What it does**: Defines what every algorithm must provide to be executable.

**Key method signatures**:

```rust
trait AlgorithmSpec {
    type Output;                    // What the algorithm produces

    fn name(&self) -> &str;         // "pagerank", "betweenness", etc.
    fn graph_name(&self) -> &str;   // Which graph to run on
    fn projection_hint(&self) -> ProjectionHint;  // Optimization guidance

    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError>;
    fn validation_config(&self, context: &ExecutionContext) -> ValidationConfiguration;

    fn execute<G: GraphStore>(
        &self,
        graph: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError>;

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError>;
}
```

**What to understand**:

- `parse_config()` = "I know how to read my configuration from JSON"
- `execute()` = "I know how to run myself"
- `consume_result()` = "I know how to produce output based on the mode (Stream/Stats/Write)"
- This is **completely generic** - any algorithm can implement it

**Why it matters**: The Executor doesn't care WHAT algorithm you're running, only that it implements AlgorithmSpec.

---

### Layer 2: The Orchestrator (ProcedureExecutor)

**File**: `src/projection/eval/procedure/executor.rs` (~507 lines)

**What it does**: Runs any AlgorithmSpec-implementing algorithm through a fixed pipeline.

**The pipeline** (critical to understand):

```
1. PARSE
   Input: String (JSON or command line)
   ↓ parse_command()
   → AlgorithmRef { name, graph_name, config }

2. VALIDATE
   Input: AlgorithmRef
   ↓ validate_algorithm()
   → Confirmation algorithm exists and config is valid

3. LOAD
   Input: AlgorithmRef
   ↓ load_algorithm()
   → Concrete algorithm instance (e.g., SumAlgorithmSpec)

4. EXECUTE
   Input: Algorithm instance + GraphStore
   ↓ algorithm.execute()
   → ComputationResult { output, elapsed_time }

5. CONSUME
   Input: ComputationResult
   ↓ algorithm.consume_result(ExecutionMode)
   → Final output (Stream/Stats/Write format)

6. RETURN
   → Result to user
```

**Key insight**: Steps 1-3-5 are generic (same for all algorithms). Steps 2-4 are algorithm-specific.

**Code pattern**:

```rust
pub fn execute_procedure(
    &self,
    procedure: &str,  // "pagerank {config}"
    graph_store: &G,
) -> Result<String> {
    // 1. Parse
    let (name, config) = parse_command(procedure)?;

    // 2. Validate
    validate_algorithm(&name)?;

    // 3. Load
    let algorithm = self.registry.load(&name)?;

    // 4. Execute
    let result = algorithm.execute(&graph_store, &config)?;

    // 5. Consume
    algorithm.consume_result(result, &ExecutionMode::Stream)?
}
```

**What to understand**:

- Executor is a **state machine**: each step depends on previous
- Each algorithm only implements 3 methods; Executor does the rest
- This is why Sum took 400 lines—but PageRank will also take ~400 lines (same structure)

---

### Layer 3: The Implementation (Sum as Proof)

**Files**:

- `src/procedure/algo/sum/spec.rs` (~400 lines) - SumAlgorithmSpec impl
- `src/procedure/algo/sum/storage.rs` (~80 lines) - Storage runtime
- `src/procedure/algo/sum/computation.rs` (~110 lines) - Computation runtime

**What it does**: Proves the pattern works end-to-end.

**The pattern**:

```
SumAlgorithmSpec (implements AlgorithmSpec)
├── parse_config()
│   ↓ reads: { property: "weight", target_nodes: [1,2,3] }
│   ↓ produces: validated config
│
├── execute()
│   ↓ creates: SumStorageRuntime (holds results)
│   ↓ creates: SumComputationRuntime (does iteration)
│   ↓ runs: storage.execute(computation)?
│   ↓ produces: ComputationResult with sum
│
└── consume_result()
    ↓ Stream mode: "sum: 15"
    ↓ Stats mode: "sum: 15, property: weight"
```

**What to understand**:

- **Storage** = where results accumulate (property columns, accumulators)
- **Computation** = how to update storage (iteration logic)
- They're kept separate (Functor pattern)
- Storage is "Gross pole" (material/concrete)
- Computation is "Subtle pole" (logical/abstract)

---

## The Sophisticated Parts

These are the parts that make Procedures truly sophisticated:

### 1. Configuration as Type-Safe Builders

```rust
// Not this (fragile):
config: { "iterations": 20 }

// But this (validated at compile-time):
PageRankConfig {
    max_iterations: 20,
    damping_factor: 0.85,
    tolerance: 1e-4,
}
```

**File to study**: `src/config/` - all configs follow builder pattern

### 2. Execution Context for Logging + Metadata

```rust
// Algorithms don't hardcode logging
// Instead, they use context:
context.log(LogLevel::Info, "Computing pagerank...");

// Context can be:
// - Testing (no-op logs)
// - Production (file logger)
// - Debug (verbose logging)
// - Analytics (track metrics)
```

**File to study**: `src/projection/eval/procedure/context.rs`

### 3. GraphStore as Generic Type Parameter

```rust
fn execute<G: GraphStore>(
    &self,
    graph: &G,
    ...
) -> Result<...> {
    // Works with ANY GraphStore implementation
    // DefaultGraphStore, ArrowGraphStore, SparseGraphStore, etc.
}
```

This means algorithms are **backend-agnostic**. Same code, different storage.

### 4. Projection Hints for Query Optimization

```rust
fn projection_hint(&self) -> ProjectionHint {
    // PageRank says "I send messages to neighbors"
    // Executor can then optimize projection layout
    VertexCentric
}
```

Not just "run the algorithm", but "run it optimally for this graph topology".

### 5. Multiple Execution Modes

```rust
enum ExecutionMode {
    Stream,     // Return scores as JSON stream
    Stats,      // Return aggregate statistics
    Write,      // Write results to property
    Explain,    // Show execution plan
}
```

Same algorithm can output different formats.

---

## What's Production-Ready vs Speculative

### ✅ Production-Ready

- **AlgorithmSpec trait**: Clear contract, proven by Sum
- **ProcedureExecutor**: Orchestration works end-to-end
- **Configuration system**: Validates at build(), not runtime
- **Context API**: Logging, metadata, timing all solid
- **Result consumption**: Multiple modes working
- **Test infrastructure**: 10 integration tests passing

### ⏳ Proven-But-Partial

- **Pregel integration**: Infrastructure exists (14 files) but not yet integrated with AlgorithmSpec
- **Computation trait**: Generic, but only direct-execution proven (Sum)
- **Storage subsystem**: Property columns work, but need to verify with Pregel

### 🚨 Speculative (Needs Review)

This is where early-day reasoning might have gone astray. Need to check:

**File 1**: `src/projection/eval/procedure/computation.rs`

- Question: Is this the right abstraction for all computation types?
- Check: Does it work for Pregel? MapReduce? Stream processing?
- Risk: Might be too specific to single-pass algorithms

**File 2**: `src/projection/eval/procedure/storage.rs`

- Question: Does PropertyValues correctly model "results accumulating"?
- Check: Can it handle message queues (Pregel)? Streaming state?
- Risk: Might need refactoring for stateful iterative algorithms

**File 3**: `src/projection/eval/procedure/validation.rs`

- Question: Is validation too strict? Not strict enough?
- Check: What do actual algorithms need to validate?
- Risk: Over-engineering validation before we have 5+ algorithms

**File 4**: `src/projection/eval/procedure/context.rs`

- Question: What should execution context contain?
- Check: Are we missing crucial metadata?
- Risk: Might need to add project_name, user_id, session_id, etc. for ML pipelines

---

## Your Weekend Study Plan

### Friday Night (45 min)

1. Read: `doc/QUICK_REFERENCE_EXECUTOR.md` (overview)
2. Read: `doc/PROCEDURE_EXECUTOR_TRANSLATION.md` (Java→Rust mapping)
3. Read: `doc/PROCEDURE_INFRASTRUCTURE_OVERVIEW.md` (architecture)

**Output**: Understand the high-level flow.

### Saturday Morning (2 hours)

1. Read: `src/projection/eval/procedure/algorithm_spec.rs` line by line
2. Read: `src/projection/eval/procedure/executor.rs` - trace execution path
3. Create: Your own summary of "execute<G>() contract"

**Output**: Understand the contract and orchestration.

### Saturday Afternoon (3 hours)

1. Read: `src/procedure/algo/sum/spec.rs`
2. Read: `src/procedure/algo/sum/storage.rs`
3. Read: `src/procedure/algo/sum/computation.rs`
4. Read: `tests/integration_sum_executor.rs`
5. **Trace**: How does sum=15 flow from input→output?

**Output**: Understand one complete algorithm implementation.

### Sunday Morning (2 hours)

1. Check: `src/projection/eval/procedure/computation.rs` - is this generic enough?
2. Check: `src/projection/eval/procedure/storage.rs` - will this work for Pregel?
3. Check: `src/projection/eval/procedure/validation.rs` - is this necessary?
4. Document: Your observations

**Output**: Identify what might be speculative.

### Sunday Afternoon (2 hours)

1. Review: `doc/PAGERANK_SESSION_10_READY.md` - the PageRank plan you just got
2. Cross-reference: Where do Pregel patterns differ from Sum patterns?
3. Identify: What code needs cleanup before PageRank
4. List: Questions for next week

**Output**: Ready to implement PageRank with confidence.

---

## Key Questions to Answer by Sunday

1. **Computation trait**: Is it doing too much? Too little?

   - Does it assume single-pass execution?
   - Can it handle iterative computation (Pregel)?
   - Will it support streaming computation later?

2. **Storage subsystem**: Does PropertyValues work as computation output?

   - It's designed for "properties on nodes"
   - But Sum uses it for "accumulated results"
   - Will this work for Pregel message queues?

3. **ValidationConfiguration**: Is it over-engineered?

   - What does it actually need to validate?
   - Should algorithms define their own validations?

4. **ExecutionContext**: Is it missing anything for ML pipelines?

   - Does it need user/session/project context?
   - Should it track model versions, feature versions?

5. **Projection hints**: Are they being used?
   - Who reads them?
   - Do they actually optimize anything?
   - Should Executor enforce them?

---

## The Three Speculative Ends You Mentioned

### End 1: Computation Runtime Trait

**Location**: `src/projection/eval/procedure/computation.rs`  
**Speculation**: "Does this work for all computation types?"

**What to check**:

- Read the trait definition
- Does it assume local state only?
- Does it handle distributed state (needed for Pregel)?
- Does it handle streaming state (needed later)?

**Resolution path**:

- PageRank will prove whether it works
- If not, create ComputationRuntime → ComputationRuntimeIterative
- If yes, it's production-ready

### End 2: Storage Runtime Trait

**Location**: `src/projection/eval/procedure/storage.rs`  
**Speculation**: "Does PropertyValues fit 'computed results'?"

**What to check**:

- How does it store accumulating results?
- Can it handle (node_id → value) mapping?
- Can it handle (node_id → vector of values)?
- Can it handle (edge_id → value) for edge properties?

**Resolution path**:

- PageRank will show if it's sufficient
- If not, create StorageRuntime → StorageRuntimeMessageQueue
- If yes, it's production-ready

### End 3: Validation System

**Location**: `src/projection/eval/procedure/validation.rs` + `validation_config()` method  
**Speculation**: "Is validation necessary? Over-engineered?"

**What to check**:

- What does ValidationConfiguration actually validate?
- Is it used by ProcedureExecutor?
- Do algorithms need custom validation?

**Resolution path**:

- See if PageRank needs validation beyond parse_config()
- If no, maybe validation is premature optimization
- If yes, flesh out the system

---

## Tie It All Together: The Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  ProcedureExecutor (Orchestrator)                           │
│  ├─ parse_command()      → AlgorithmRef                     │
│  ├─ validate_algorithm() → OK/Error                         │
│  ├─ load_algorithm()     → AlgorithmSpec instance           │
│  ├─ execute()            → ComputationResult                │
│  └─ consume_result()     → Stream/Stats/Write output        │
└────────────────┬────────────────────────────────────────────┘
                 │
    ┌────────────┴────────────┬──────────────┐
    │                         │              │
    ▼                         ▼              ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│    Sum      │  │  PageRank   │  │ Betweenness │
│  AlgoSpec   │  │  AlgoSpec   │  │  AlgoSpec   │
└─────────────┘  └─────────────┘  └─────────────┘
    │                 │                  │
    ├─ parse_config   ├─ parse_config    ├─ parse_config
    ├─ execute        ├─ execute         ├─ execute
    └─ consume        └─ consume         └─ consume

    Each calls:
    ┌──────────────────────┐  ┌──────────────────────┐
    │  StorageRuntime      │  │  ComputationRuntime  │
    │  ├─ set()           │  │  ├─ next_superstep() │
    │  ├─ get()           │  │  └─ converged?       │
    │  └─ finalize()      │  └──────────────────────┘
    └──────────────────────┘
```

Each algorithm is a **plugin** to the executor. The executor is **invariant**. The algorithms **vary**.

---

## Next Week: Pipelines

Once you understand Procedures:

```rust
trait PipelineSpec {
    type Output;

    fn stages(&self) -> Vec<&dyn AlgorithmSpec>;  // Composed algorithms
    fn features(&self) -> Vec<&dyn FeatureSpec>;  // Feature engineering
    fn models(&self) -> Vec<&dyn ModelSpec>;      // ML models

    fn execute<G: GraphStore>(
        &self,
        graph: &G,
    ) -> Result<PipelineResult>;
}
```

Pipelines are **compositions of procedures + features + models**.

That will make sense once you see how Procedures work.

---

## Breathing Room

You're at a natural pause point. The architecture is:

- ✅ Sound (Executor orchestration works)
- ✅ Proven (Sum working end-to-end)
- ✅ Extensible (PageRank ready)
- ⏳ Speculative (Computation/Storage need review)

The weekend is for understanding, not building. Take your time. The foundation is solid.

By Monday, you'll see exactly what needs cleanup and what's production-ready.

Trust the process. 🙏
