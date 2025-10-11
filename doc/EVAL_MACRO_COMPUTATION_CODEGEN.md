# Eval Macro Computation Codegen (Phase 0)

**Date**: October 10, 2025  
**Status**: Foundation complete, ready for macro integration  
**Context**: How eval! macro generates Computer/ComputeStep from ComputationDescriptor

---

## Overview

The `eval!` macro can now generate complete computation pipelines (Computer + ComputeStep implementations) from ComputationDescriptor definitions. This document describes the codegen strategy and what the macro will emit.

---

## Architecture

### Three-Layer Stack

```
ComputationDescriptor (Svarūpa)
    ↓ [eval! macro]
Computer + ComputeStep (Generated)
    ↓ [Runtime]
Execution (init → step* → finalize)
```

**Key insight**: ComputationDescriptor is the canonical source of truth (Svarūpa). The macro projects it into concrete Computer/ComputeStep implementations.

---

## What the Macro Generates

### Input (DSL)

```rust
eval! {
    computation: BSP {
        id: 1,
        name: "page_rank",
        pattern: VertexCentric,
        init: "init_ranks",
        compute: "compute_ranks",
        max_iterations: 10,
    }
}
```

### Output (Generated Code)

```rust
// 1. Register descriptor at module load time
const _: () = {
    use rust_gds::projection::computation_descriptor::*;

    #[ctor::ctor]
    fn register_page_rank_descriptor() {
        let desc = ComputationDescriptor::new(
            1,
            "page_rank",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );
        register_computation_descriptor(desc);
    }
};

// 2. Generate ComputeStep implementation
struct PageRankStep {
    iteration: std::sync::atomic::AtomicUsize,
    max_iterations: usize,
}

impl PageRankStep {
    fn new(max_iterations: usize) -> Self {
        Self {
            iteration: std::sync::atomic::AtomicUsize::new(0),
            max_iterations,
        }
    }
}

impl rust_gds::projection::computation_runtime::ComputeStep for PageRankStep {
    fn compute(
        &self,
        ctx: &mut rust_gds::projection::computation_runtime::ComputeContext<'_>,
        messages: &rust_gds::projection::computation_runtime::Messages
    ) -> Result<bool, rust_gds::projection::computation_runtime::ComputeError> {
        let iter = self.iteration.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Call user-supplied compute function
        compute_ranks(ctx, messages)?;

        // Check convergence
        Ok(iter < self.max_iterations)
    }
}

// 3. Generate Computer implementation
struct PageRankComputer {
    step: Box<dyn rust_gds::projection::computation_runtime::ComputeStep>,
    node_values: Vec<f64>,
}

impl PageRankComputer {
    fn new(max_iterations: usize) -> Self {
        Self {
            step: Box::new(PageRankStep::new(max_iterations)),
            node_values: Vec::new(),
        }
    }
}

impl rust_gds::projection::computation_runtime::Computer for PageRankComputer {
    fn init(&mut self, ctx: &mut rust_gds::projection::computation_runtime::ComputeContext<'_>)
        -> Result<(), rust_gds::projection::computation_runtime::ComputeError>
    {
        // Allocate storage
        self.node_values = vec![1.0; ctx.node_count as usize];

        // Call user-supplied init function
        init_ranks(ctx, &mut self.node_values)?;

        Ok(())
    }

    fn step(&mut self, ctx: &mut rust_gds::projection::computation_runtime::ComputeContext<'_>)
        -> Result<bool, rust_gds::projection::computation_runtime::ComputeError>
    {
        self.step.compute(ctx, &rust_gds::projection::computation_runtime::Messages::empty())
    }

    fn finalize(&mut self, ctx: &mut rust_gds::projection::computation_runtime::ComputeContext<'_>)
        -> Result<(), rust_gds::projection::computation_runtime::ComputeError>
    {
        // Write back to property store
        // (Phase 1 will use materialize_to_property_store)
        Ok(())
    }
}

// 4. Register factory function
const _: () = {
    use rust_gds::projection::computation_runtime::*;

    fn page_rank_factory(_id: u32) -> Result<Box<dyn Computer>, ComputeError> {
        Ok(Box::new(PageRankComputer::new(10)))
    }

    #[ctor::ctor]
    fn register_page_rank_factory() {
        register_computer_factory(1, page_rank_factory);
    }
};
```

---

## User-Supplied Functions

The macro expects user code to provide these functions:

```rust
// Init function signature
fn init_ranks(
    ctx: &mut ComputeContext<'_>,
    node_values: &mut Vec<f64>
) -> Result<(), ComputeError> {
    // Initialize node values from property store
    // (Phase 1 will use materialize_from_property_store)
    Ok(())
}

// Compute function signature
fn compute_ranks(
    ctx: &mut ComputeContext<'_>,
    messages: &Messages
) -> Result<(), ComputeError> {
    // Perform one iteration of computation
    Ok(())
}
```

---

## Safety Guarantees

### What the Macro Enforces

1. **Checked index conversions**: All `node_id as usize` uses `checked_u64_to_usize`
2. **Result-based errors**: No `unwrap()` or `expect()` in generated code
3. **Trait imports**: Uses `rust_gds::types::prelude::*`
4. **Registration safety**: Uses `#[ctor::ctor]` for deterministic registration order

### Example: Safe Index Conversion

```rust
// Generated code uses FormProcessor
use rust_gds::projection::form_processor::checked_u64_to_usize;

fn access_node_value(node_id: u64, values: &[f64]) -> Result<f64, ComputeError> {
    let idx = checked_u64_to_usize(node_id)
        .map_err(|e| ComputeError::InitFailed(format!("index overflow: {}", e)))?;

    values.get(idx)
        .copied()
        .ok_or_else(|| ComputeError::StepFailed("node value missing".into()))
}
```

---

## Integration with Property System

### Phase 0 (Current)

Manual property handling in user-supplied functions:

```rust
fn init_ranks(ctx: &mut ComputeContext<'_>, values: &mut Vec<f64>) -> Result<(), ComputeError> {
    // User manually loads from property store
    if let Some(props) = ctx.graph.node_properties() {
        for node_id in 0..ctx.node_count {
            let idx = checked_u64_to_usize(node_id)?;
            let rank = props.get_double_property(node_id, "seed_rank")
                .unwrap_or(1.0);
            values[idx] = rank;
        }
    }
    Ok(())
}
```

### Phase 1 (Next)

Macro generates materializer calls:

```rust
fn init_ranks(ctx: &mut ComputeContext<'_>, values: &mut Vec<f64>) -> Result<(), ComputeError> {
    // Generated by macro
    materialize_from_property_store(
        ctx.graph,
        &SCHEMA_PAGE_RANK,
        values
    ).map_err(|e| ComputeError::InitFailed(format!("materialize: {}", e)))?;

    Ok(())
}
```

---

## Testing Strategy

### Unit Tests

Test generated components in isolation:

```rust
#[test]
fn test_generated_step() {
    let step = PageRankStep::new(5);
    let graph = random_graph_store(42);
    let mut ctx = ComputeContext::new(&graph.graph);

    // Should run for 5 iterations
    for i in 0..5 {
        assert!(step.compute(&mut ctx, &Messages::empty()).unwrap());
    }

    // 6th should return false (converged)
    assert!(!step.compute(&mut ctx, &Messages::empty()).unwrap());
}
```

### Integration Tests

Test full lifecycle:

```rust
#[test]
fn test_computation_lifecycle() {
    let mut computer = instantiate_computer_from_descriptor(1).unwrap();
    let graph = random_graph_store(42);
    let mut ctx = ComputeContext::new(&graph.graph);

    // Full cycle
    computer.init(&mut ctx).unwrap();
    while computer.step(&mut ctx).unwrap() {}
    computer.finalize(&mut ctx).unwrap();
}
```

### Example

See `examples/computation_lifecycle_demo.rs` for complete runnable demo.

---

## Next Steps

### Phase 0 (Complete) ✅

- [x] ComputationDescriptor registry
- [x] Computer/ComputeStep traits
- [x] Factory registration system
- [x] Safe index conversion helpers
- [x] Example demonstrating full lifecycle

### Phase 1 (Next)

- [ ] Macro proc implementation (parse DSL → emit code)
- [ ] Property materializer integration
- [ ] Backend selection hooks
- [ ] Error handling improvements

### Phase 2 (Future)

- [ ] Multiple computation species (MapReduce, Dataflow)
- [ ] Composition operators
- [ ] Cross-package extraction to @reality

---

## Design Notes

### Why Factory Registration?

Allows runtime instantiation without macro-time type knowledge:

```rust
// Macro registers factories by descriptor id
register_computer_factory(1, pagerank_factory);
register_computer_factory(2, louvain_factory);

// Runtime selects by id
let computer = instantiate_computer_from_descriptor(user_selected_id)?;
```

### Why Trait-Based?

- **Object safety**: `Box<dyn Computer>` allows heterogeneous collections
- **Extension**: User code can implement Computer without macro
- **Testing**: Easy to create mock implementations

### Why Atomic Iteration Counter?

- **Thread-safe**: ComputeStep may be called from multiple threads
- **Simple**: No need for Mutex overhead
- **Efficient**: Single atomic increment per step

---

## Philosophical Mapping

### Five-Fold Structure

1. **Rūpa** (Form): ComputationDescriptor (schema)
2. **Vedanā** (Contact): Messages (communication)
3. **Saññā** (Recognition): ComputeStep (processing)
4. **Saṅkhāra** (Formation): Computer (lifecycle)
5. **Viññāṇa** (Result): Finalize (output)

### Nondual Flow

```
@reality IN (ComputationDescriptor)
    ↓ [eval! macro projection]
Computer/ComputeStep (Generated)
    ↓ [Runtime execution]
@reality OUT (Results written back)

NONDUAL: Single source of truth flows through
```

---

**Bottom line**: The foundation is complete. Macro implementation can now focus on parsing DSL and emitting the patterns shown above. All safety guarantees, registration mechanisms, and runtime contracts are in place.

**Next**: Implement the macro proc that parses `eval! { computation: ... }` and emits this code.
