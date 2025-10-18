# How to Execute an Algorithm Through the Executor

## The Big Picture

We've created `SumAlgorithmSpec` that implements the `AlgorithmSpec` trait. But how do we actually _run_ it?

The answer: **`ProcedureExecutor`** orchestrates the entire execution.

```
User Code
  ↓
ProcedureExecutor::compute()
  ├─ preprocess_config()
  ├─ parse_config()
  ├─ validate_before_load()
  ├─ load_graph()
  ├─ validate_after_load()
  ├─ execute()                ← SumAlgorithmSpec::execute() called here
  ├─ consume_result()
  └─ return Output
  ↓
Result in user's hand
```

## The Three Layers

### Layer 1: Fixed GDSL Runtime (eval/procedure)

```
src/projection/eval/procedure/
├── algorithm_spec.rs       ← Define AlgorithmSpec trait
├── executor.rs             ← Define ProcedureExecutor
├── execution_context.rs    ← Pass context to algorithms
├── validation_config.rs    ← Validation rules
└── ... other infrastructure
```

**What it knows**: Generic orchestration. HOW to run algorithms, but not WHAT they do.

### Layer 2: Algorithm Implementation (procedure/algo)

```
src/procedure/algo/
└── sum/
    ├── spec.rs             ← SumAlgorithmSpec (implements AlgorithmSpec)
    ├── storage.rs          ← SumStorageRuntime (Gross pole)
    └── computation.rs      ← SumComputationRuntime (Subtle pole)
```

**What it knows**: Specific algorithm. WHAT the algorithm does.

### Layer 3: User Code

```rust
// Your code
let mut executor = ProcedureExecutor::new(context, mode);
let result = executor.compute(&mut sum_spec, &config)?;
```

**What it knows**: I have an algorithm I want to run.

## Step-by-Step Execution Flow

### Step 1: Create ExecutionContext

```rust
use rust_gds::projection::eval::procedure::{ExecutionContext, ExecutionMode};
use rust_gds::types::prelude::*;

// The context holds:
// - Graph catalog (where to load graphs from)
// - Logging infrastructure
// - Metrics collection
// - Progress tracking

let context = ExecutionContext::new(
    catalog,  // Your graph catalog
    LogLevel::Info,
);
```

**What it is**: The environment where algorithms run.

### Step 2: Create the Algorithm Specification

```rust
use rust_gds::procedure::algo::sum::{SumAlgorithmSpec, SumConfig};

let config = SumConfig {
    property_key: "node_value".to_string(),
    weight_property: None,
};

let mut sum_spec = SumAlgorithmSpec::new(
    "my_graph".to_string(),  // Graph to load
    config,
);
```

**What it is**: The algorithm instance with its configuration.

### Step 3: Create the Executor

```rust
use rust_gds::projection::eval::procedure::{ProcedureExecutor, ExecutionMode};

let mut executor = ProcedureExecutor::new(
    context,
    ExecutionMode::Stream,  // How to return results
);
```

**What it is**: The orchestrator that will run the algorithm.

### Step 4: Execute

```rust
use serde_json::json;

let config_json = json!({
    "property_key": "node_value",
    "weight_property": null,
});

// This ONE call orchestrates the entire flow!
let result = executor.compute(&mut sum_spec, &config_json)?;

// result is f64 (the sum)
println!("Sum: {}", result);
```

**What happens inside**:

1. **Preprocess** - `sum_spec.preprocess_config()`

   - Enhance config if needed (ML pipelines use this)

2. **Parse** - `sum_spec.parse_config()`

   - Validate JSON
   - Extract property_key, weight_property
   - Return validated config

3. **Validate Before Load** - `sum_spec.validation_config().validate_before_load()`

   - Check config-only constraints
   - E.g., property_key is non-empty

4. **Load Graph** - `context.load_graph("my_graph")`

   - Get GraphStore from catalog
   - Check it's not empty

5. **Validate After Load** - `sum_spec.validation_config().validate_after_load()`

   - Check config + graph constraints
   - E.g., property_key exists on graph

6. **Execute** - `sum_spec.execute()`

   - Create SumStorageRuntime (Gross pole)
   - Create SumComputationRuntime (Subtle pole)
   - Iterate nodes and accumulate
   - Return ComputationResult

7. **Consume Result** - `sum_spec.consume_result()`

   - Transform result based on ExecutionMode
   - STREAM: return raw sum
   - STATS: return sum with metadata
   - Validate output structure

8. **Return** - Final output to user

## Complete Working Example

```rust
fn execute_sum_algorithm() -> Result<(), Box<dyn std::error::Error>> {
    use rust_gds::projection::eval::procedure::{
        ExecutionContext, ExecutionMode, ProcedureExecutor, LogLevel,
    };
    use rust_gds::procedure::algo::sum::{SumAlgorithmSpec, SumConfig};
    use rust_gds::types::random_graph_store;
    use serde_json::json;

    // Step 1: Create a test graph
    let graph_store = random_graph_store(100, 0.5)?;
    let catalog = build_catalog(graph_store)?;

    // Step 2: Create execution context
    let context = ExecutionContext::new(catalog, LogLevel::Info);

    // Step 3: Create algorithm specification
    let config = SumConfig {
        property_key: "value".to_string(),
        weight_property: None,
    };

    let mut sum_spec = SumAlgorithmSpec::new(
        "test_graph".to_string(),
        config,
    );

    // Step 4: Create executor
    let mut executor = ProcedureExecutor::new(
        context,
        ExecutionMode::Stream,
    );

    // Step 5: Execute!
    let config_json = json!({
        "property_key": "value",
        "weight_property": null,
    });

    let result = executor.compute(&mut sum_spec, &config_json)?;

    println!("Sum result: {}", result);
    Ok(())
}
```

## Key Insight: The Executor is Generic

The `ProcedureExecutor` doesn't care about Sum specifically:

```rust
pub fn compute<A: AlgorithmSpec>(
    &mut self,
    algorithm: &mut A,
    config_input: &JsonValue,
) -> Result<A::Output, ExecutorError>
```

**Notice**: Generic over `A: AlgorithmSpec`

This means:

- Same executor for PageRank, Louvain, Sum, etc.
- Just pass a different AlgorithmSpec implementation
- Executor handles the orchestration generically

### Adding a New Algorithm (Future)

```rust
// 1. Create PageRankSpec (implements AlgorithmSpec)
let mut pagerank_spec = PageRankSpec::new("my_graph".to_string(), config);

// 2. Same executor!
let result = executor.compute(&mut pagerank_spec, &config_json)?;

// That's it!
```

## The Functor Machinery in Action

During `execute()`, here's where the Functor works:

```rust
// In SumAlgorithmSpec::execute()

// Create Storage Runtime (Gross pole)
let storage = SumStorageRuntime::new(graph_store, property_key)?;
//            ↑ knows how to access PropertyValues

// Create Computation Runtime (Subtle pole)
let mut computation = SumComputationRuntime::new();
//                    ↑ knows how to accumulate

// Project Gross → Subtle for each node
for node_id in 0..node_count {
    // THIS IS THE FUNCTOR IN ACTION:
    let value = storage.get_node_value(node_id)?;
    //           ↑ PropertyValues (Gross)
    //           ↓ projects to
    //           ↓ f64 (Subtle)
    computation.add_value(value);
    //           ↑ accumulates in computation runtime
}
```

## Error Handling Path

What if something goes wrong?

```
ProcedureExecutor::compute()
  ├─ parse_config fails?
  │  └─ return Err(ExecutorError::Config(ConfigError))
  │
  ├─ validate fails?
  │  └─ return Err(ExecutorError::Validation(ValidationError))
  │
  ├─ load_graph fails?
  │  └─ return Err(ExecutorError::Context(ContextError))
  │
  ├─ execute fails?
  │  └─ return Err(ExecutorError::Algorithm(AlgorithmError))
  │
  └─ consume_result fails?
     └─ return Err(ExecutorError::Consumer(ConsumerError))
```

Each error type is specific to where it occurred.

## ExecutionMode Effects

Different modes affect how results are consumed:

```rust
// STREAM mode
let result = executor.compute(&mut sum_spec, &config_json)?;
// Returns: f64 (the sum value directly)

// STATS mode
let result = executor.compute(&mut sum_spec, &config_json)?;
// Returns: f64 (sum with implicit metadata)
// (Future: could return {sum: f64, count: u64, time: Duration})

// WRITE mode
let result = executor.compute(&mut sum_spec, &config_json)?;
// Not supported by SumAlgorithmSpec (read-only)
// Returns: Err(ConsumerError::UnsupportedMode)
```

## What Makes This Design Work

1. **Fixed Infrastructure (eval/procedure)**

   - Provides AlgorithmSpec trait contract
   - Provides ProcedureExecutor orchestration
   - Generic over all algorithms
   - Never needs to change when adding new algorithms

2. **Extensible Content (procedure/algo)**

   - Each algorithm is independent
   - Each implements AlgorithmSpec
   - Can be added without modifying executor
   - Can use different storage/computation strategies

3. **Separation of Concerns**

   - Executor = HOW to run (generic)
   - Algorithm = WHAT to run (specific)
   - Never confused between the two

4. **Type Safety**
   - AlgorithmSpec trait enforces contract
   - Each algorithm's Output type is known at compile time
   - Executor is generic but statically typed
   - No runtime type checking needed

## Next Steps: Implementing Another Algorithm

Once you understand this flow, adding PageRank is straightforward:

1. **Create `src/procedure/algo/pagerank/spec.rs`**

   - Implement `AlgorithmSpec` for PageRank
   - Implement `execute()` with PageRank-specific logic

2. **Create `src/procedure/algo/pagerank/storage.rs`**

   - Implement PageRankStorageRuntime
   - Know how to access graph structure

3. **Create `src/procedure/algo/pagerank/computation.rs`**

   - Implement PageRankComputationRuntime
   - Implement PageRank iteration logic

4. **Register in `src/procedure/algo/mod.rs`**

   ```rust
   pub mod pagerank;
   pub use pagerank::PageRankAlgorithmSpec;
   ```

5. **Use the same executor!**
   ```rust
   let mut pagerank_spec = PageRankAlgorithmSpec::new("graph", config);
   let result = executor.compute(&mut pagerank_spec, &config_json)?;
   ```

Same orchestration loop. Different algorithm.
