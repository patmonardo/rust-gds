# Quick Reference: Adding New Algorithms

This guide shows how to add a new algorithm following the SumAlgorithmSpec pattern.

## Template Structure

Every algorithm needs this structure:

```
src/procedure/algo/{algo_name}/
├── mod.rs              - Module hub (declares submodules)
├── spec.rs             - AlgorithmSpec implementation (THE KEY FILE)
├── storage.rs          - Gross pole (persistent storage access)
└── computation.rs      - Subtle pole (ephemeral accumulation)
```

## Step 1: Create Directory

```bash
mkdir -p src/procedure/algo/{algo_name}
```

## Step 2: Create mod.rs

```rust
//! {AlgoName} Algorithm
//!
//! This module implements the {algorithm description}.
//!
//! - `spec.rs` - {AlgoName}AlgorithmSpec (implements AlgorithmSpec trait)
//! - `storage.rs` - Storage runtime ({Gross pole})
//! - `computation.rs` - Computation runtime ({Subtle pole})

pub mod computation;
pub mod spec;
pub mod storage;

pub use spec::{AlgorithmSpec, AlgorithmConfig};
pub use storage::StorageRuntime;
pub use computation::ComputationRuntime;
```

## Step 3: Create computation.rs (Subtle Pole)

```rust
//! Computation Runtime - Ephemeral accumulation

#[derive(Debug, Clone)]
pub struct ComputationRuntime {
    // State that changes during algorithm
    // Examples:
    //   PageRank: new_scores accumulator
    //   Louvain: community moves accumulator
    //   BFS: frontier queue
}

impl ComputationRuntime {
    pub fn new() -> Self {
        Self { /* ... */ }
    }

    pub fn process(&mut self, value: f64) {
        // Accumulate/process values from storage
    }

    pub fn result(&self) -> f64 {
        // Return final result
    }
}

impl Default for ComputationRuntime {
    fn default() -> Self { Self::new() }
}
```

## Step 4: Create storage.rs (Gross Pole)

```rust
//! Storage Runtime - Persistent access to PropertyValues

use crate::types::prelude::GraphStore;
use crate::projection::eval::procedure::AlgorithmError;

pub struct StorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    property_key: String,
}

impl<'a, G: GraphStore> StorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G, property_key: &str)
        -> Result<Self, AlgorithmError>
    {
        Ok(Self {
            graph_store,
            property_key: property_key.to_string(),
        })
    }

    pub fn get_value(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // TODO: Actually read from PropertyValues
        // For now, placeholder
        Ok(1.0)
    }
}
```

## Step 5: Create spec.rs (THE KEY FILE)

This is the most important file. It implements the AlgorithmSpec trait.

```rust
//! Algorithm Specification

use crate::projection::eval::procedure::{
    AlgorithmSpec, ComputationResult, ConfigError, ConsumerError,
    ExecutionContext, ExecutionMode, ProjectionHint, LogLevel,
    AlgorithmError, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;

use super::storage::StorageRuntime;
use super::computation::ComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone)]
pub struct AlgorithmConfig {
    pub property_key: String,
    // Add algorithm-specific fields
}

// ============================================================================
// Algorithm Specification
// ============================================================================

pub struct AlgorithmSpec {
    graph_name: String,
    config: AlgorithmConfig,
}

impl AlgorithmSpec {
    pub fn new(graph_name: String, config: AlgorithmConfig) -> Self {
        Self { graph_name, config }
    }
}

// ============================================================================
// AlgorithmSpec Trait Implementation
// ============================================================================

impl AlgorithmSpec for AlgorithmSpec {
    type Output = f64;  // Change based on output type

    fn name(&self) -> &str {
        "algorithm_name"
    }

    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Auto  // Or Dense/Columnar/Sparse
    }

    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Parse JSON config
        let property_key = input
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConfigError::MissingParameter("property_key".into()))?
            .to_string();

        Ok(json!({ "property_key": property_key }))
    }

    fn validation_config(&self, _context: &ExecutionContext)
        -> ValidationConfiguration
    {
        ValidationConfiguration::empty()
    }

    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        let property_key = config
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing property_key".into()))?;

        context.log(LogLevel::Info,
            &format!("Computing algorithm for property: {}", property_key));

        let timer = Instant::now();

        // **FUNCTOR IN ACTION**:
        // 1. Create storage runtime (Gross pole - PropertyValues)
        let storage = StorageRuntime::new(graph_store, property_key)?;

        // 2. Create computation runtime (Subtle pole - accumulation)
        let mut computation = ComputationRuntime::new();

        // 3. Iterate and process
        for node_id in 0..graph_store.node_count() as u32 {
            let value = storage.get_value(node_id)?;  // Gross
            computation.process(value);               // Subtle
        }

        let elapsed = timer.elapsed();

        Ok(ComputationResult::new(computation.result(), elapsed))
    }

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => Ok(result.into_result()),
            ExecutionMode::Stats => Ok(result.into_result()),
            other => Err(ConsumerError::UnsupportedMode(other.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let spec = AlgorithmSpec::new(
            "test".to_string(),
            AlgorithmConfig { property_key: "value".to_string() },
        );
        assert_eq!(spec.name(), "algorithm_name");
    }
}
```

## Step 6: Update src/procedure/algo/mod.rs

Add to the module declaration:

```rust
pub mod {algo_name};

pub use {algo_name}::{AlgorithmSpec, AlgorithmConfig};
```

## Step 7: Update src/procedure/mod.rs

The re-export will happen automatically.

## Step 8: Compile and Test

```bash
cargo check     # Should compile
cargo test      # All tests should pass
```

## Key Points to Remember

1. **Storage pole** (`storage.rs`): Knows about persistent data (PropertyValues, arrays)
2. **Computation pole** (`computation.rs`): Knows about ephemeral state (accumulators, queues)
3. **Specification** (`spec.rs`): Bridges them via `execute()` method
4. **Executor** doesn't change - it's generic infrastructure

## Checklist for New Algorithms

- [ ] Create directory: `src/procedure/algo/{name}/`
- [ ] Create `mod.rs` with module declarations
- [ ] Create `computation.rs` with runtime state and operations
- [ ] Create `storage.rs` with PropertyValues access
- [ ] Create `spec.rs` implementing `AlgorithmSpec` trait
- [ ] Add module to `src/procedure/algo/mod.rs`
- [ ] Add tests to each file
- [ ] Verify: `cargo check` (no errors)
- [ ] Verify: `cargo test` (all pass)
- [ ] Update `get_value()` in storage.rs to read real PropertyValues (if needed)

## Pattern Matches Across Algorithms

**Same execute() pattern for all algorithms:**

```rust
fn execute() {
    // 1. Create storage runtime (Gross pole)
    let storage = StorageRuntime::new(graph, property)?;

    // 2. Create computation runtime (Subtle pole)
    let mut computation = ComputationRuntime::new();

    // 3. Iterate (pattern depends on algorithm)
    for node_id in 0..graph.node_count() {
        let value = storage.get_value(node_id)?;
        computation.process(value);
    }

    // 4. Return result
    Ok(ComputationResult::new(computation.result(), elapsed))
}
```

Only the `process()` method changes between algorithms.
Only the storage access pattern changes between algorithms.
The executor never changes.

## Real Examples in Codebase

Reference implementations:

- Sum: `src/procedure/algo/sum/` - simple, one-pass aggregation
- Future: PageRank would be multi-pass iteration
- Future: Louvain would be phase-based iteration

## When to Use This Pattern

✅ Always for new algorithms
✅ Always follow the three-pole architecture
✅ Always implement AlgorithmSpec trait
✅ Always keep executor out of algorithm logic

## When You're Done

Your new algorithm:

- Works with existing executor (no changes needed)
- Can use different storage backends (Auto/Dense/Columnar/Sparse)
- Follows proven pattern
- Ready for Codegen integration

That's it! Add another algorithm the same way.
