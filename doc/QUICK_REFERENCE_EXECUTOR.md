# Quick Reference: How to Execute an Algorithm via ProcedureExecutor

## One-Minute Overview

```rust
// 1. User writes this:
let config_json = json!({
    "property_key": "node_value",
    "weight_property": null,
});

// 2. You create the algorithm spec:
let mut algorithm = SumAlgorithmSpec::new(
    "my_graph".to_string(),
    SumConfig {
        property_key: "node_value".to_string(),
        weight_property: None,
    },
);

// 3. You create an executor:
let mut executor = ProcedureExecutor::new(
    ExecutionContext::new("username"),
    ExecutionMode::Stream,
);

// 4. Executor runs the whole flow:
let result = executor.compute(&mut algorithm, &config_json)?;
// Result is f64 (the sum)
```

That's it. The executor handles everything else.

## What the Executor Does (Behind the Scenes)

```
executor.compute()
  ↓
[1] algorithm.preprocess_config(&mut config)
     - Enhance config with context (optional)
  ↓
[2] config = algorithm.parse_config(&config)
     - Validate JSON
     - Extract required parameters
     - Return validated config
  ↓
[3] validation = algorithm.validation_config(&context)
     - Get validation rules
  ↓
[4] validation.validate_before_load(&config)
     - Validate config without graph
  ↓
[5] graph = context.load_graph(graph_name)
     - Get graph from catalog
  ↓
[6] validation.validate_after_load(graph, &config)
     - Validate config with graph present
  ↓
[7] result = algorithm.execute(graph, &config, &context)
     - RUN THE ALGORITHM
     - Returns ComputationResult<Output>
  ↓
[8] output = algorithm.consume_result(result, mode)
     - Transform result based on execution mode
     - Return final Output
  ↓
OUTPUT (f64 for Sum)
```

## How Storage ↔ Computation Works

### Storage Pole (Gross)

```rust
// In storage.rs
impl SumStorageRuntime {
    pub fn get_node_value(&self, node_id: u32) -> Result<f64, _> {
        // Read from PropertyValues (persistent storage)
        // Return f64 (value for computation)
        Ok(1.0)  // placeholder: read real value here
    }
}
```

### Computation Pole (Subtle)

```rust
// In computation.rs
impl SumComputationRuntime {
    pub fn add_value(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }
}
```

### The Functor (The Mapping)

```rust
// In spec.rs execute() method
for node_id in 0..graph_store.node_count() {
    // FUNCTOR: Storage → Computation
    let value = storage.get_node_value(node_id)?;  // Gross → Subtle
    computation.add_value(value);                  // Accumulate
}
```

## Configuration

### What Users Provide

```json
{
  "property_key": "node_value",
  "weight_property": null
}
```

### What Algorithm Does

```rust
fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
    // Extract and validate
    let property_key = input.get("property_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ConfigError::MissingParameter("property_key"))?;

    // Return validated config
    Ok(json!({
        "property_key": property_key,
        "weight_property": weight_property,
    }))
}
```

### What Executor Uses

```rust
// Validated config passed to execute()
algorithm.execute(&graph, &validated_config, &context)
```

## Execution Modes

### STREAM

- Returns all algorithm results
- Good for: exploratory analysis, debugging
- Use when: you want to see everything

```rust
ExecutionMode::Stream
  ↓
consume_result() returns raw result
  ↓
User sees: the sum value
```

### STATS

- Returns summary statistics
- Good for: monitoring, dashboards
- Use when: you want aggregates

```rust
ExecutionMode::Stats
  ↓
consume_result() returns stats summary
  ↓
User sees: computed metrics
```

### WRITE modes (rejected for Sum)

- WriteNodeProperty, WriteRelationship
- Sum doesn't support (read-only)

```rust
ExecutionMode::WriteNodeProperty
  ↓
consume_result() returns error
  ↓
User sees: "operation not supported"
```

## Projection Hints

```rust
fn projection_hint(&self) -> ProjectionHint {
    ProjectionHint::Dense  // Tell executor about access pattern
}
```

What each means:

- **Dense**: Cursor iteration, access all nodes (Sum, PageRank)
- **Columnar**: Zero-copy export, mmap-friendly (BFS to file)
- **Sparse**: HashMap-based, selective access (Louvain)
- **Auto**: Let executor analyze graph density

## Error Handling

```rust
// Config parsing error
parse_config(&config)?
  → Err(ConfigError::MissingParameter("property_key"))

// Validation error
validation.validate_after_load(graph, &config)?
  → Err(ValidationError::...)

// Execution error
algorithm.execute(graph, &config, context)?
  → Err(AlgorithmError::Execution("..."))

// Consumption error
consume_result(result, mode)?
  → Err(ConsumerError::UnsupportedMode(...))

// Executor wraps all in ExecutorError
executor.compute()
  → Err(ExecutorError::...)
```

## Implementation Checklist: Creating a New Algorithm

To add a new algorithm (e.g., PageRank):

```
□ Create src/procedure/algo/pagerank/mod.rs
□ Create src/procedure/algo/pagerank/spec.rs
  ├─ struct PageRankAlgorithmSpec
  ├─ impl AlgorithmSpec for PageRankAlgorithmSpec
  ├─ Implement: name()
  ├─ Implement: graph_name()
  ├─ Implement: projection_hint()
  ├─ Implement: parse_config()
  ├─ Implement: execute()
  ├─ Implement: consume_result()
  └─ Tests

□ Create src/procedure/algo/pagerank/storage.rs
  ├─ struct PageRankStorageRuntime
  ├─ fn get_node_value()
  └─ Tests

□ Create src/procedure/algo/pagerank/computation.rs
  ├─ struct PageRankComputationRuntime
  ├─ fn execute_iteration()
  └─ Tests

□ Update src/procedure/algo/mod.rs
  └─ pub mod pagerank;

□ Update src/procedure/mod.rs
  └─ pub use algo::PageRankAlgorithmSpec;

□ Run: cargo test
□ Run: cargo build
□ Add integration tests in tests/
```

**That's it.** Executor will automatically support PageRank without modification.

## Common Patterns

### Getting a value from config

```rust
let property_key = config
    .get("property_key")
    .and_then(|v| v.as_str())
    .ok_or_else(|| ConfigError::MissingParameter("property_key"))?
    .to_string();
```

### Creating an optional value

```rust
let weight_property = config
    .get("weight_property")
    .and_then(|v| v.as_str())
    .map(|s| s.to_string());
```

### Returning validated config

```rust
Ok(json!({
    "property_key": property_key,
    "weight_property": weight_property,
}))
```

### Handling computation result

```rust
let result = ComputationResult::new(computed_value, elapsed);
Ok(result)
```

### Handling executor-orchestrated execution

```rust
fn execute<G: GraphStore>(
    &self,
    graph_store: &G,
    config: &JsonValue,
    context: &ExecutionContext,
) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
    // Your algorithm here
    Ok(ComputationResult::new(output, elapsed))
}
```

## Debugging

### Print what executor is doing:

```rust
let context = ExecutionContext::new("user")
    .with_log_level(LogLevel::Debug);

executor = ProcedureExecutor::new(context, mode);
```

### Executor logs:

```
[INFO] Starting procedure: sum on graph: my_graph
[DEBUG] Config preprocessing took: 0ms
[DEBUG] Config parsed: {...}
[DEBUG] Before-load validation passed
[INFO] Graph loaded: 1000 nodes, 5000 rels (12ms)
[DEBUG] After-load validation passed
[INFO] Executing algorithm: sum
[INFO] Algorithm completed: sum (compute: 1ms, total: 13ms)
```

### Your algorithm logs:

```rust
context.log(LogLevel::Info, &format!(
    "Computing sum for property: {}",
    property_key
));
```

## Testing

### Unit test (algorithm methods)

```rust
#[test]
fn test_parse_config() {
    let spec = SumAlgorithmSpec::new(...);
    let result = spec.parse_config(&json!({...}));
    assert!(result.is_ok());
}
```

### Integration test (full flow)

```rust
#[test]
fn test_executor_flow() {
    let mut algorithm = SumAlgorithmSpec::new(...);
    let mut executor = ProcedureExecutor::new(...);
    let result = executor.compute(&mut algorithm, &config_json);
    assert!(result.is_ok());
}
```

## Key Insight

**The executor is a generic loop.** It doesn't know what algorithm runs. It just:

1. Calls methods on AlgorithmSpec (the contract)
2. Passes data between methods
3. Handles errors

**Your algorithm** is the specific implementation. You:

1. Implement AlgorithmSpec trait methods
2. Provide computation logic
3. Handle the storage ↔ computation mapping

**Result**: New algorithms are just new trait implementations. The executor works for all of them.
