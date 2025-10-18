# Concrete Implementation Plan: SumAlgorithmSpec

## What We've Understood

**AlgorithmSpec Trait Contract** (518 lines, `src/projection/eval/procedure/algorithm_spec.rs`):

- Required method: `execute()` - Run the algorithm
- Required method: `consume_result()` - Format output
- Required method: `parse_config()` - Parse JSON config
- Required method: `name()` - Algorithm name
- Required method: `graph_name()` - Which graph to load
- Optional method: `preprocess_config()` - Enhance config
- Optional method: `validation_config()` - Get validators
- Optional method: `projection_hint()` - Guide storage backend
- Associated type: `Output` - Result type

**ProcedureExecutor Orchestration** (507 lines, `src/projection/eval/procedure/executor.rs`):

1. Preprocess config (if algorithm needs to)
2. Parse config via `parse_config()`
3. Get validation config
4. Validate before load (config only)
5. Load graph from catalog
6. Check graph not empty
7. Validate after load (config + graph)
8. Execute algorithm via `execute()`
9. Consume result via `consume_result()`
10. Return final Output

## What SumAlgorithmSpec Must Do

```
INPUT: User says "sum all node values"
  ├─ graph_name: "my_graph"
  ├─ property_key: "value"
  ├─ weight_property: optional
  └─ output_format: "decimal" or "integer"

ALGORITHM:
  1. Parse config: Extract graph_name, property_key, etc.
  2. Validate: Ensure property exists on graph
  3. Execute: Sum all node property values
  4. Return: Single f64 or i64 value

OUTPUT: f64 (the sum)
```

## Directory Structure to Create

```
src/procedure/algo/
├── mod.rs                    (NEW - declare sum module)
└── sum/                      (NEW - sum aggregation)
    ├── mod.rs                (NEW - module hub)
    ├── spec.rs               (NEW - SumAlgorithmSpec, AlgorithmSpec impl)
    ├── storage.rs            (NEW - PropertyValues access)
    └── computation.rs        (NEW - Sum accumulation logic)
```

## File 1: `src/procedure/algo/mod.rs`

```rust
//! Algorithm implementations
//!
//! Each algorithm lives in a submodule:
//! - `sum` - Sum aggregation
//! - `count` - Count elements
//! - `average` - Average value
//! - `pagerank` - PageRank centrality
//! - etc.

pub mod sum;

// Re-export public types
pub use sum::{SumAlgorithmSpec, SumConfig};
```

## File 2: `src/procedure/algo/sum/mod.rs`

```rust
//! Sum Aggregation Algorithm
//!
//! This is the hub that declares submodules and re-exports.

pub mod computation;
pub mod spec;
pub mod storage;

// Re-export what users of this algorithm need
pub use spec::{SumAlgorithmSpec, SumConfig};
pub use storage::SumStorageRuntime;
pub use computation::SumComputationRuntime;
```

## File 3: `src/procedure/algo/sum/spec.rs` - THE KEY FILE

This is where `SumAlgorithmSpec` implements `AlgorithmSpec` trait.

````rust
use crate::projection::eval::procedure::{
    AlgorithmSpec, AlgorithmError, ComputationResult, ConfigError, ConsumerError,
    ExecutionContext, ExecutionMode, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::*;
use serde_json::{json, Value as JsonValue};

use super::storage::SumStorageRuntime;
use super::computation::SumComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Sum Aggregation Configuration
#[derive(Debug, Clone)]
pub struct SumConfig {
    pub property_key: String,
    pub weight_property: Option<String>,
}

// ============================================================================
// Algorithm Specification
// ============================================================================

/// Sum Aggregation Algorithm Specification
///
/// Implements the AlgorithmSpec trait for Sum aggregation.
/// Maps Storage (PropertyValues) ↔ Computation (GdsValue stream).
pub struct SumAlgorithmSpec {
    pub graph_name: String,
    pub config: SumConfig,
}

impl SumAlgorithmSpec {
    pub fn new(graph_name: String, config: SumConfig) -> Self {
        Self { graph_name, config }
    }
}

// ============================================================================
// AlgorithmSpec Implementation
// ============================================================================

impl AlgorithmSpec for SumAlgorithmSpec {
    /// Output is f64 (the sum value)
    type Output = f64;

    /// Algorithm name
    fn name(&self) -> &str {
        "sum"
    }

    /// Graph to load
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Prefer dense arrays (Sum iterates all nodes)
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON config
    ///
    /// **Input JSON**:
    /// ```json
    /// {
    ///   "property_key": "value",
    ///   "weight_property": null,
    ///   "return_stats": true
    /// }
    /// ```
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Extract property_key (required)
        let property_key = input
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConfigError::MissingParameter("property_key".to_string()))?
            .to_string();

        // Extract weight_property (optional)
        let weight_property = input
            .get("weight_property")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Return validated config
        Ok(json!({
            "property_key": property_key,
            "weight_property": weight_property,
        }))
    }

    /// Validate configuration
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        // For now, no special validation
        // Could add: property exists, is numeric, etc.
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    ///
    /// **Flow**:
    /// 1. Create SumStorageRuntime (accesses PropertyValues)
    /// 2. Create SumComputationRuntime (accumulates sum)
    /// 3. Project values from storage → computation
    /// 4. Return sum as Output
    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        // Extract config
        let property_key = config
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing property_key".to_string()))?;

        context.log(
            LogLevel::Info,
            &format!("Computing sum for property: {}", property_key),
        );

        let timer = std::time::Instant::now();

        // Create storage runtime (knows how to access PropertyValues)
        let storage = SumStorageRuntime::new(graph_store, property_key)?;

        // Create computation runtime (knows how to accumulate)
        let mut computation = SumComputationRuntime::new();

        // Iterate all nodes and accumulate sum
        for node_id in 0..graph_store.node_count() as u32 {
            // Project from storage (Gross) → computation (Subtle)
            let value = storage.get_node_value(node_id)?;
            computation.add_value(value);
        }

        let elapsed = timer.elapsed();

        // Return result wrapped in ComputationResult
        Ok(ComputationResult::new(computation.sum(), elapsed))
    }

    /// Consume result and produce final output
    ///
    /// **Processing**:
    /// - STREAM mode: Return the sum value
    /// - STATS mode: Return {sum: value, count: node_count}
    /// - Other modes: Error (sum is read-only)
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => {
                // Stream mode: return raw sum
                Ok(result.into_result())
            }
            ExecutionMode::Stats => {
                // Stats mode: return sum with metadata
                // For now, just return the sum (could add more metadata)
                Ok(result.into_result())
            }
            other => {
                // Sum is read-only
                Err(ConsumerError::UnsupportedMode(other.clone()))
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_algorithm_name() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.name(), "sum");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_sum_parse_config() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "property_key": "node_value",
            "weight_property": null,
        });

        let config = spec.parse_config(&input);
        assert!(config.is_ok());
    }

    #[test]
    fn test_sum_parse_config_missing_property_key() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );

        let input = json!({
            "weight_property": null,
        });

        let config = spec.parse_config(&input);
        assert!(config.is_err());
    }
}
````

## File 4: `src/procedure/algo/sum/storage.rs`

This is the **Storage Pole** - knows how to access PropertyValues.

```rust
use crate::types::prelude::*;
use crate::projection::eval::procedure::AlgorithmError;

/// Storage Runtime for Sum Aggregation
///
/// This is the **Gross pole** - accesses persistent data structures.
/// It knows how to extract values from PropertyValues (storage layer).
pub struct SumStorageRuntime<'a> {
    graph_store: &'a dyn GraphStore,
    property_key: String,
}

impl<'a> SumStorageRuntime<'a> {
    /// Create storage runtime
    pub fn new(graph_store: &'a dyn GraphStore, property_key: &str) -> Result<Self, AlgorithmError> {
        Ok(Self {
            graph_store,
            property_key: property_key.to_string(),
        })
    }

    /// Get node value from storage
    ///
    /// This projects from PropertyValues (Gross - persistent storage)
    /// to f64 (Subtle - computation value)
    pub fn get_node_value(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For now, simplified: always return 1.0
        // In reality, would:
        // 1. Look up PropertyValues by property_key
        // 2. Extract value for node_id
        // 3. Convert to f64
        // 4. Handle missing values

        // This is where the Functor machinery actually works:
        // PropertyValues (Gross) → f64 (Subtle)
        Ok(1.0)
    }
}
```

## File 5: `src/procedure/algo/sum/computation.rs`

This is the **Computation Pole** - accumulates the sum.

```rust
/// Computation Runtime for Sum Aggregation
///
/// This is the **Subtle pole** - ephemeral accumulation.
/// It knows how to accumulate values in-memory.
pub struct SumComputationRuntime {
    /// Accumulator for sum
    sum: f64,
    /// Count of values processed
    count: usize,
}

impl SumComputationRuntime {
    /// Create computation runtime
    pub fn new() -> Self {
        Self { sum: 0.0, count: 0 }
    }

    /// Add a value to the sum
    ///
    /// This is the **Subtle pole** - accumulation happens here.
    /// Values coming from PropertyValues (Gross) are accumulated here.
    pub fn add_value(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    /// Get final sum
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Get count
    pub fn count(&self) -> usize {
        self.count
    }
}

impl Default for SumComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
```

## Implementation Strategy

**Step 1**: Create the four files above in `src/procedure/algo/sum/`
**Step 2**: Update `src/procedure/mod.rs` to include `pub mod algo;`
**Step 3**: Test that it compiles
**Step 4**: Enhance `get_node_value()` to actually read from PropertyValues
**Step 5**: Run against actual graphs and verify results
**Step 6**: Create integration tests in `tests/`

## Why This Works

```
Genus (Principle):
  "Sum all node values"

Species (Manifestation):
  SumAlgorithmSpec
    + Implements AlgorithmSpec trait
    + Contains SumStorageRuntime (Gross pole - PropertyValues)
    + Contains SumComputationRuntime (Subtle pole - accumulation)
    + Knows how to map between them

Executor (Fixed Infrastructure):
  ProcedureExecutor
    1. Calls parse_config() → Get SumConfig
    2. Calls execute() → Returns sum value
    3. Calls consume_result() → Returns final output
    4. Never needs to know what SumAlgorithmSpec is!

Result:
  "The sum of all node values"
```

This is the complete Functor machinery instantiated:

- Storage Runtime ↔ Computation Runtime mapping via `get_node_value()`
- PropertyValues (Gross) → f64 (Subtle) projection
- Executor doesn't need to know about this; it just calls methods
- New algorithms can be added without modifying Executor
