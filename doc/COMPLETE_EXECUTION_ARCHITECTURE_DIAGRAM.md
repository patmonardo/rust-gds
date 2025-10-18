# Complete Execution Architecture Diagram

## High-Level View: Three Layers

```
┌─────────────────────────────────────────────────────────────┐
│ USER CODE                                                   │
│                                                             │
│  executor.compute(&mut sum_spec, &config_json)?            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ GDSL RUNTIME (Fixed Infrastructure)                         │
│ src/projection/eval/procedure/                              │
│                                                             │
│  ProcedureExecutor                                          │
│    1. preprocess_config()                                   │
│    2. parse_config()                                        │
│    3. validate_before_load()                                │
│    4. load_graph()                                          │
│    5. validate_after_load()                                 │
│    6. execute() ← delegates to algorithm                    │
│    7. consume_result()                                      │
│    8. return result                                         │
│                                                             │
│  Generic over all AlgorithmSpec implementations             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ ALGORITHM IMPLEMENTATION (Extensible Content)               │
│ src/procedure/algo/sum/                                     │
│                                                             │
│  SumAlgorithmSpec (implements AlgorithmSpec)                │
│    + SumStorageRuntime (Gross pole - PropertyValues)        │
│    + SumComputationRuntime (Subtle pole - accumulation)     │
│    + Functor mapping between them                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
                      RESULT
```

## Orchestration Sequence Diagram

```
User Code
  │
  ├─ Creates ExecutionContext
  │    │ (holds catalog, logging, metrics)
  │    └─→ ProcedureExecutor
  │
  ├─ Creates SumAlgorithmSpec
  │    │ (holds graph_name, config)
  │    └─→ ProcedureExecutor
  │
  ├─ Calls executor.compute(&sum_spec, config_json)
  │    │
  │    ├─ 1. sum_spec.preprocess_config()
  │    │    └─ Enhance config with defaults/context
  │    │
  │    ├─ 2. sum_spec.parse_config()
  │    │    └─ Parse & validate JSON
  │    │
  │    ├─ 3. sum_spec.validation_config().validate_before_load()
  │    │    └─ Check config-only constraints
  │    │
  │    ├─ 4. context.load_graph(graph_name)
  │    │    └─ Get GraphStore from catalog
  │    │
  │    ├─ 5. sum_spec.validation_config().validate_after_load()
  │    │    └─ Check config + graph constraints
  │    │
  │    ├─ 6. sum_spec.execute(graph_store, config, context)
  │    │    │
  │    │    ├─ Create SumStorageRuntime(graph_store)
  │    │    │  └─ Knows how to access PropertyValues (Gross)
  │    │    │
  │    │    ├─ Create SumComputationRuntime()
  │    │    │  └─ Knows how to accumulate (Subtle)
  │    │    │
  │    │    ├─ For each node_id in graph:
  │    │    │  │
  │    │    │  ├─ value = storage.get_node_value(node_id)
  │    │    │  │  └─ FUNCTOR: PropertyValues → f64
  │    │    │  │
  │    │    │  └─ computation.add_value(value)
  │    │    │     └─ Accumulate in Subtle pole
  │    │    │
  │    │    └─ Return ComputationResult { sum, elapsed_time }
  │    │
  │    ├─ 7. sum_spec.consume_result(result, mode)
  │    │    └─ Transform result based on ExecutionMode
  │    │       (STREAM: raw sum, STATS: sum with metadata)
  │    │
  │    └─ 8. Return f64
  │
  └─ Handle result
```

## Data Flow: Gross ↔ Subtle

During `execute()`, the Functor machinery projects between two poles:

```
GROSS POLE (Storage/Persistent)        SUBTLE POLE (Computation/Ephemeral)
─────────────────────────────────      ──────────────────────────────────

GraphStore
  ├─ node_count: 1000
  ├─ properties:
  │  └─ "node_value": PropertyValues
  │     ├─ [0] = 1.0
  │     ├─ [1] = 2.5
  │     └─ ...
  └─ relationships

        ↓ FUNCTOR: get_node_value() ↓

                                       SumComputationRuntime
                                         sum: 0.0
                                         count: 0

                                         add_value(1.0)
                                         sum: 1.0
                                         count: 1

                                         add_value(2.5)
                                         sum: 3.5
                                         count: 2

                                         ... (iterate all nodes)

                                         FINAL: sum: 2500.0
```

## Error Handling Architecture

```
ProcedureExecutor::compute()
  │
  └─ Orchestration Phase
     │
     ├─ parse_config()
     │  └─ Error? → Err(ExecutorError::Config(ConfigError::...))
     │
     ├─ validation_config().validate_before_load()
     │  └─ Error? → Err(ExecutorError::Validation(ValidationError::...))
     │
     ├─ load_graph()
     │  └─ Error? → Err(ExecutorError::Context(ContextError::...))
     │
     ├─ validation_config().validate_after_load()
     │  └─ Error? → Err(ExecutorError::Validation(ValidationError::...))
     │
     ├─ execute()
     │  └─ Error? → Err(ExecutorError::Algorithm(AlgorithmError::...))
     │
     └─ consume_result()
        └─ Error? → Err(ExecutorError::Consumer(ConsumerError::...))
```

Each error type tells you WHERE the problem occurred:

- **ConfigError** - JSON parsing or validation failed
- **ValidationError** - Config/graph validation failed
- **ContextError** - Graph loading or context problem
- **AlgorithmError** - Algorithm execution failed
- **ConsumerError** - Result processing failed

## Type Safety Throughout

```
AlgorithmSpec trait (defines contract)
    ↑ implemented by
    │
SumAlgorithmSpec (specific algorithm)
    ├─ type Output = f64
    ├─ fn execute() → Result<ComputationResult<f64>, AlgorithmError>
    └─ fn consume_result() → Result<f64, ConsumerError>

ProcedureExecutor::compute<A: AlgorithmSpec>()
    ├─ Returns: Result<A::Output, ExecutorError>
    └─ For SumAlgorithmSpec: Result<f64, ExecutorError>

User gets:
    ├─ compile-time guarantee: result is f64
    ├─ no runtime type checking
    └─ errors are statically known
```

## Generic Over All Algorithms

```
impl<A: AlgorithmSpec> ProcedureExecutor {
    pub fn compute<A: AlgorithmSpec>(
        &mut self,
        algorithm: &mut A,
        config_input: &JsonValue,
    ) -> Result<A::Output, ExecutorError>
}

// Works with SumAlgorithmSpec
executor.compute(&mut sum_spec, &config)?;
// → Result<f64, ExecutorError>

// Would work with PageRankAlgorithmSpec (future)
executor.compute(&mut pagerank_spec, &config)?;
// → Result<Vec<(NodeId, f64)>, ExecutorError>

// Would work with LouvainAlgorithmSpec (future)
executor.compute(&mut louvain_spec, &config)?;
// → Result<Vec<Community>, ExecutorError>

// Same executor for all!
```

## The Three-Tier Modularity

```
Tier 1: GDSL Runtime (FIXED)
└─ src/projection/eval/procedure/
   ├─ AlgorithmSpec trait (contract)
   ├─ ProcedureExecutor (orchestrator)
   ├─ ExecutionContext (environment)
   ├─ ValidationConfiguration (rules)
   └─ Error types

   ✓ Never changes
   ✓ Generic over all algorithms
   ✓ Involved in Codegen

Tier 2: Algorithm Infrastructure (SHARED)
└─ src/procedure/core/
   ├─ Result builders
   ├─ Statistics types
   ├─ Feature scaling
   └─ Common utilities

   ✓ Shared by multiple algorithms
   ✓ Independent of specific algorithms

Tier 3: Algorithm Implementations (EXTENSIBLE)
└─ src/procedure/algo/
   ├─ sum/
   │  ├─ spec.rs (SumAlgorithmSpec - implements AlgorithmSpec)
   │  ├─ storage.rs (SumStorageRuntime - Gross pole)
   │  └─ computation.rs (SumComputationRuntime - Subtle pole)
   │
   ├─ pagerank/ (future)
   ├─ louvain/ (future)
   └─ ...

   ✓ New algorithms added without modifying Tiers 1 or 2
   ✓ Each is independent
   ✓ Each plugs into generic Executor
```

## The Separation That Makes This Work

```
┌─────────────────────────────────────────────────────────┐
│ "HOW" (Generic Infrastructure)                          │
│                                                         │
│ ProcedureExecutor knows:                                │
│ - How to preprocess config                              │
│ - How to parse JSON                                     │
│ - How to validate                                       │
│ - How to load graphs                                    │
│ - How to call execute()                                 │
│ - How to consume results                                │
│                                                         │
│ Does NOT know:                                          │
│ - What Sum aggregates                                   │
│ - What PageRank computes                                │
│ - Storage/computation details                           │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ "WHAT" (Specific Algorithm)                             │
│                                                         │
│ SumAlgorithmSpec knows:                                 │
│ - What to compute (sum of properties)                   │
│ - How to access storage (PropertyValues)                │
│ - How to accumulate (Subtle pole)                       │
│ - How to map Gross ↔ Subtle                             │
│                                                         │
│ Does NOT know:                                          │
│ - How to orchestrate execution (that's executor job)    │
│ - How to handle other algorithms                        │
│ - Global validation rules                               │
└─────────────────────────────────────────────────────────┘
```

## Why This Architecture Works

1. **Decoupling**: Executor doesn't know about Sum. Sum doesn't know about Executor.
2. **Reusability**: Same Executor for PageRank, Louvain, Sum, etc.
3. **Extensibility**: Add new algorithms without modifying Executor.
4. **Type Safety**: Each algorithm's Output type is known at compile time.
5. **Error Clarity**: Each error type indicates exactly where the problem is.
6. **Generic Orchestration**: Executor works with ANY AlgorithmSpec implementation.

## The Functor Pattern in Context

```
GROSS POLE (Storage)         FUNCTOR          SUBTLE POLE (Computation)
───────────────────────      ───────          ──────────────────────────

PropertyValues               algorithm         GdsValue stream
(indexed array)          →   logic      →     (ephemeral values)

Position-based              node_id          Transformative
Fixed structure             ↓                Logic-based
Persistent                  get_node_value() Ephemeral
                            ↓
                            f64

Multiple nodes              Many iterations   Single accumulator
accessed in sequence        of Functor        building up result
```

This is the universal pattern: **Storage ↔ Computation** mapping at runtime.
