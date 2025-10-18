# The Puzzle: All Pieces in Place

## What We're Simulating

```
Java GDS Architecture:
  AlgorithmSpec interface (contract)
    ↓ creates via factory
  Algorithm instance (runtime)
    ↓ uses
  StorageLayer (HugeArray, PropertyValues)
    ↓ computes via
  ComputationLayer (tasks, threads)
    ↓ returns
  Result

Ceremony involved:
  - Reflection to discover AlgorithmSpec implementations
  - Factory pattern to instantiate algorithms
  - Runtime registration of specializations
  - Visitor pattern for factory selection
```

## What We're Building

```
Codegen-generated Architecture:
  Algorithm (genus - abstract principle)
    ↓ generates via Functor machinery
  AlgorithmSpec (species - concrete manifestation)
    ↓ knows about
  StorageRuntime (PropertyValues + HugeDoubleArray)
    ↔ projects to via Functor
  ComputationRuntime (GdsValue stream + accumulator)
    ↓ orchestrated by
  ProcedureExecutor (generic runtime loop)
    ↓ returns
  Result

No ceremony:
  - Compile-time code generation
  - Direct instantiation
  - Static specialization
  - Type-driven selection
```

## The Current Pieces (What's Already There)

### 1. GDSL Runtime (Fixed Infrastructure)

**Location:** `src/projection/eval/procedure/`

```
algorithm_spec.rs       - AlgorithmSpec trait (contract for ALL algorithms)
executor.rs             - ProcedureExecutor (orchestrates ANY algorithm)
execution_context.rs    - Runtime context passed to algorithms
execution_mode.rs       - Execution modes (COMPUTE, WRITE, etc.)
computation_result.rs   - Result container
result_consumer.rs      - Result processing
validation_config.rs    - Validation rules
```

**What it does:**

- Defines the interface algorithms must implement
- Orchestrates execution (generic loop that works for ANY algorithm)
- Handles result consumption and validation
- Passes context to algorithm

**NOT algorithm-specific:**

- Doesn't know about Sum, PageRank, etc.
- Doesn't know about specific storage/computation mappings
- Pure infrastructure

### 2. Procedure Runtime (Extensible Content)

**Location:** `src/procedure/`

```
core/          - Common utilities (prelude, result types, scaling)
algo/          - Algorithm implementations (EMPTY - where we add things)
  (currently empty, will contain sum/, pagerank/, etc.)
```

**What it is:**

- Home for algorithm implementations
- Each algorithm is a specialization of AlgorithmSpec
- Each lives in its own module

### 3. Projection Codegen System

**Location:** `src/projection/codegen/`

```
algorithm/         - Principles (the Functor machinery)
  functor.rs       - SubtleToGross, GrossToSubtle traits
  type_projector.rs - Maps descriptors to storage/computation extremes
  type_validator.rs - Validates mappings
descriptors/       - Schema definitions
membership.rs      - Membership extraction
consequence.rs     - Consequence derivation
inherence.rs       - Inherence recognition
registry.rs        - Analysis pole (Omniscience)
catalog.rs         - Generation pole (Omnipotence)
```

**What it does:**

- Defines Functor machinery (genus → species mapping)
- Can generate AlgorithmSpecs from Algorithms
- Currently manual (we simulate), will be macro-based

## The Simulation We're Building

### Three Levels Clearly Separated

**Level 0: Principles (Genus)**

```
src/projection/codegen/algorithm/functor.rs

pub trait Functor {
    type Gross;      // Storage pole (PropertyValues)
    type Subtle;     // Computation pole (GdsValue)

    fn project_to_storage(&self, subtle: Self::Subtle) -> Self::Gross;
    fn project_to_computation(&self, gross: Self::Gross) -> Self::Subtle;
}
```

What it represents: "Here's HOW to map between storage and computation"

**Level 1: Infrastructure (Fixed Runtime)**

```
src/projection/eval/procedure/

AlgorithmSpec trait:
  - name()
  - graph_name()
  - parse_config()
  - execute()
  - consume_result()

ProcedureExecutor:
  - Generic orchestrator
  - Works for ANY AlgorithmSpec
  - Doesn't need to know about specific algorithms
```

What it represents: "Here's HOW to run ANY algorithm generically"

**Level 2: Specializations (Species)**

```
src/procedure/algo/sum/

SumAlgorithmSpec implements AlgorithmSpec + Functor:
  - Concrete Sum algorithm
  - Knows: PropertyValues storage format
  - Knows: GdsValue computation stream
  - Knows: How to map between them (Functor instance)
  - Knows: How to produce result
```

What it represents: "Here's WHAT a Sum algorithm looks like in practice"

## The Flow (Complete)

```
User:
  "Execute Sum aggregation on graph with values"
    ↓
ProcedureExecutor (generic, from eval/procedure/):
  1. Load execution context
  2. Parse config via AlgorithmSpec::parse_config
  3. Load graph from catalog
  4. Create SumAlgorithmSpec instance
  5. Call algorithm.execute(&graph, &config)
    ↓
SumAlgorithmSpec (from procedure/algo/sum/):
  1. Extract PropertyValues from graph (Storage Runtime - Gross)
  2. Apply Functor: project_to_computation()
  3. Compute sum over GdsValue stream (Computation Runtime - Subtle)
  4. Return result
    ↓
ProcedureExecutor (back to generic):
  1. Validate result
  2. Consume result via AlgorithmSpec::consume_result
  3. Return final output to user
```

## The Pieces We Need to Create

### For Sum Aggregation:

**`src/procedure/algo/sum/mod.rs`**

- Module hub
- Re-exports

**`src/procedure/algo/sum/spec.rs`**

- `SumAlgorithmSpec` struct
- Implements `AlgorithmSpec` trait (from eval/procedure)
- Implements `Functor` trait (from codegen/algorithm)
- Methods:
  - `execute()` - orchestrate storage + computation
  - `parse_config()` - parse user config
  - `consume_result()` - format output
  - `project_to_storage()` - Gross direction
  - `project_to_computation()` - Subtle direction

**`src/procedure/algo/sum/storage.rs`**

- `SumStorageRuntime<T>` struct
- Holds: `Arc<HugeDoubleArray>`, graph reference
- Methods to extract values by node ID
- Knows: "I am the persistent layer"

**`src/procedure/algo/sum/computation.rs`**

- `SumComputationRuntime<T>` struct
- Holds: accumulator state, type conversions
- Implements: accumulation logic
- Knows: "I am the ephemeral transformation"

### For Module System:

**`src/procedure/algo/mod.rs`**

- Declares sub-modules: `sum`, `count`, `average`, etc.
- Re-exports all AlgorithmSpecs

## Summary: The Puzzle Complete

```
┌─────────────────────────────────────────────────────────────┐
│ GENUS (Algorithm Principle)                                 │
│ - Abstract idea of "sum"                                    │
│ - Lives in: codegen/algorithm (Functor machinery)           │
└─────────────────────────────────────────────────────────────┘
           ↓ MAP via Functor
┌─────────────────────────────────────────────────────────────┐
│ SPECIES (AlgorithmSpec Implementation)                      │
│ - Concrete SumAlgorithmSpec                                 │
│ - Lives in: procedure/algo/sum                              │
│ - Implements: AlgorithmSpec + Functor                       │
│ - Knows: Storage (PropertyValues) ↔ Computation (GdsValue)  │
└─────────────────────────────────────────────────────────────┘
           ↓ ORCHESTRATED BY
┌─────────────────────────────────────────────────────────────┐
│ INFRASTRUCTURE (ProcedureExecutor)                          │
│ - Generic runtime loop                                      │
│ - Lives in: projection/eval/procedure                       │
│ - Works for: ANY AlgorithmSpec                              │
│ - Doesn't know: About Sum specifically                      │
└─────────────────────────────────────────────────────────────┘
           ↓ PRODUCES
┌─────────────────────────────────────────────────────────────┐
│ RESULT (Output)                                             │
│ - Sum value aggregated from all nodes                       │
│ - Type-safe (i64 or f64)                                    │
│ - Validated by AlgorithmSpec::consume_result               │
└─────────────────────────────────────────────────────────────┘
```

This is what we simulate, implement, and eventually generate via Codegen.
