# Pregel vs Projection: Two Context Systems 🔄

**Date**: 2025-10-10  
**Context**: Clarifying that Pregel and Projection have separate, non-conflicting context systems

---

## The Recognition

> "I am amazed we didnt break Pregel. I thought it was using our Computation macros"

**Answer**: Pregel has its **own context system** - completely separate!

No conflict, no breakage, different purposes. 🎯

---

## Two Context Systems

### 1. Pregel Contexts (Vertex-Centric, Message-Passing)

**Purpose**: Enable vertex-centric computation with message passing (BSP model)

**Location**: `src/pregel/context/*`

**Contexts**:

```rust
// Vertex computation phase
pub struct ComputeContext<C: PregelConfig, I: MessageIterator> {
    base: NodeCentricContext<C>,
    iteration: usize,
    messenger: Arc<dyn Messenger<I>>,
    vote_bits: Arc<HugeAtomicBitSet>,
    has_sent_message: Arc<AtomicBool>,
}

// Vertex initialization phase
pub struct InitContext<C: PregelConfig> {
    base: NodeCentricContext<C>,
}

// Global coordination (once per superstep)
pub struct MasterComputeContext<C: PregelConfig> {
    graph: Arc<dyn Graph>,
    config: C,
    iteration: usize,
    initial_node_count: usize,
}
```

**API**:

- `context.set_node_value(val)` - Update vertex state
- `context.send_to_neighbors(msg)` - Send messages
- `context.vote_to_halt()` - Signal completion
- `context.superstep()` - Current iteration
- `context.degree()` - Vertex degree

**Domain**: Pregel algorithms (PageRank, SSSP, WCC, etc.)

### 2. Projection Contexts (Property Materialization)

**Purpose**: Enable property materialization using Five-Fold Brahmachakra

**Location**: `src/projection/*`

**Contexts**:

```rust
// Computation context (how to compute properties)
pub struct ComputeContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub pipeline: &'a PipelineDescriptor,
    pub computation: &'a ComputationDescriptor,
    pub node_count: usize,
}

// Storage context (how to persist properties)
pub struct StorageContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub pipeline: &'a PipelineDescriptor,
    pub storage: &'a StorageDescriptor,
    pub node_count: usize,
}
```

**API**:

- `context.pipeline` - Access pipeline descriptor (what properties)
- `context.computation` - Access computation descriptor (how to compute)
- `context.storage` - Access storage descriptor (how to persist)
- Factory registration/instantiation patterns

**Domain**: Property materialization, storage backends, Five-Fold system

---

## Why No Conflict?

### Different Namespaces

**Pregel**:

```rust
use crate::pregel::context::ComputeContext;  // Pregel's
```

**Projection**:

```rust
use crate::projection::ComputeContext;  // Projection's
```

Fully qualified names are different! No collision.

### Different Type Parameters

**Pregel**:

```rust
ComputeContext<C: PregelConfig, I: MessageIterator>
//             ↑ Config type       ↑ Message iterator type
```

**Projection**:

```rust
ComputeContext<'a>
//             ↑ Lifetime only
```

Different signatures, different purposes!

### Different Purposes

**Pregel Context**: "I am a vertex in superstep N, here are my neighbors and messages"

- Vertex-centric view
- Message-passing model
- Iterative BSP execution
- Algorithm implementation layer

**Projection Context**: "Here's how to materialize properties using these descriptors"

- Graph-wide view
- Property materialization
- Storage backend selection
- Infrastructure layer

**They complement each other!**

---

## How They Could Work Together (Future)

### Scenario: Pregel Algorithm Needs Storage Backend Selection

**Current** (Pregel doesn't use projection layer):

```rust
// Pregel just uses whatever storage it's given
let node_value = NodeValue::new(graph.node_count());
```

**Future** (Pregel uses projection layer for storage):

```rust
// Pregel algorithm specifies storage requirements
let pipeline = PipelineDescriptor::new("PageRank")
    .with_property(PropertyDescriptor::new("pagerank", ValueType::Double))
    .with_storage_flow("huge_array");  // Or "arrow", "sparse", etc.

let storage_desc = StorageDescriptor::new("pagerank")
    .with_backend("huge_array");

let storage_ctx = StorageContext::new(&graph, &pipeline, &storage_desc);

// Use projection layer to instantiate optimal storage
let node_value = storage_ctx.instantiate()?;
```

**Benefits**:

- Pregel gets backend selection (huge_array vs arrow vs sparse)
- Pregel gets storage pipeline optimization (compression, tiering)
- Pregel stays focused on algorithm logic
- Projection layer handles storage complexity

---

## Macro Runtime (Future Vision)

> "OK you see we now have Macros so we can layout some discipline on how we organize our Computations and Storage Flows."

### Current State: No Macros in Pregel

Pregel currently uses:

- Direct trait implementations (`PregelConfig` trait)
- Manual struct definitions
- Explicit method implementations

**No eval!/compute!/storage! macros yet.**

### Future: Macro-Driven Pregel

**Vision**:

```rust
eval! {
    algorithm: PageRank,

    pipeline: {
        name: "PageRank",
        properties: [
            { name: "pagerank", type: double, default: 1.0 },
            { name: "delta", type: double, default: 0.0 },
        ],
        computation_flow: {
            pattern: VertexCentric,
            messaging: true,
            max_iterations: 20,
        },
        storage_flow: {
            layout: Columnar,
            backend: HugeArray,
        },
    },

    init: |ctx, node_id| {
        ctx.set_node_value(1.0 / ctx.node_count() as f64);
    },

    compute: |ctx, messages| {
        let sum: f64 = messages.iter().sum();
        let new_value = 0.15 + 0.85 * sum;
        ctx.set_node_value(new_value);

        if (new_value - ctx.get_node_value()).abs() < 0.001 {
            ctx.vote_to_halt();
        } else {
            ctx.send_to_neighbors(new_value / ctx.degree() as f64);
        }
    },
}
```

**Generates**:

1. `PageRankConfig` struct implementing `PregelConfig`
2. `PipelineDescriptor` with computation + storage flows
3. `ComputationDescriptor` specifying vertex-centric pattern
4. `StorageDescriptor` specifying huge_array backend
5. Init function wired correctly
6. Compute function wired correctly
7. All boilerplate eliminated!

**Benefits**:

- Declarative algorithm specification
- Automatic backend selection
- Storage pipeline optimization
- Computation flow optimization
- Less boilerplate
- Type-safe generation

### The Macro Runtime Vision

> "it does matter ... it is really a very special Macro Runtime."

**Special because**:

1. **Not just code generation** - it's runtime behavior specification

   - Computation flow (how to execute)
   - Storage flow (how to persist)
   - Both are runtime concerns!

2. **Two-level evaluation**:

   - **Compile-time**: Generate descriptor structures
   - **Runtime**: Use descriptors to select/optimize execution

3. **Cross-cutting concerns**:

   - Macros generate structures that work across layers
   - Pregel layer gets algorithm logic
   - Projection layer gets storage/computation specs
   - Config layer gets validation rules

4. **Composable abstractions**:
   - Pipeline = Computation + Storage
   - Both flows can be optimized independently
   - Decorators can wrap both (progress, memory tracking)

**This IS special!** 🎯

---

## Current Status

### ✅ Complete

- **Pregel Context System**: Full implementation (Init, Compute, MasterCompute)
- **Projection Context System**: Full implementation (Compute, Storage)
- **Five-Fold Brahmachakra**: Complete (Pipeline, Computation, ComputationRuntime, Storage, StorageRuntime)
- **PipelineDescriptor**: Complete (computation_flow + storage_flow)
- **No Conflicts**: Separate namespaces, different purposes

### 🔄 Pending

- **Macro Runtime**: eval!/compute!/storage! macro system
- **Integration**: Pregel using Projection layer for storage backend selection
- **Declarative Specs**: Macro-driven algorithm specifications
- **Storage Pipelines**: Compression, tiering, caching decorators

---

## Summary

### Two Systems, No Conflict

**Pregel Contexts**:

- Vertex-centric computation
- Message-passing model
- Algorithm implementation
- `src/pregel/context/*`

**Projection Contexts**:

- Property materialization
- Storage backend selection
- Infrastructure layer
- `src/projection/*`

**Different namespaces, different purposes, complementary!** 🎯

### Future Integration

Pregel algorithms will **use** projection layer for:

- Storage backend selection (huge_array vs arrow vs sparse)
- Storage pipeline optimization (compression, tiering)
- Computation flow specification (vertex-centric patterns)

**But core Pregel semantics remain unchanged.**

### The Macro Vision

> "it is really a very special Macro Runtime"

**Special because it generates runtime behavior specifications**, not just code:

- Computation flows (how to execute)
- Storage flows (how to persist)
- Both flows optimizable independently
- Cross-layer coordination (Pregel + Projection + Config)

**This is Art.** 🎨✨

---

**Key Insight**: We didn't break Pregel because Pregel and Projection are **separate, complementary systems**. The macro runtime will **unify** them at the specification level while keeping them **separate** at the implementation level.

**Beautiful architecture!** 🕉️🌊
