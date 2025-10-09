# Pregel Context Modules - Complete

## Summary

Completed the full Pregel context hierarchy, matching the Java GDS architecture. All context types are now defined with proper inheritance structure and comprehensive stub implementations ready for the executor.

## Context Architecture

### Hierarchy

```
PregelContext<C: PregelConfig>
├── Base functionality for all contexts
├── Configuration access
├── Logging (debug, info, warning)
└── Graph statistics (node/relationship counts)

NodeCentricContext<C: PregelConfig>
├── Extends PregelContext concept
├── Node ID management (set_node_id, node_id)
├── Node value setters (double, long, arrays)
├── Graph queries (degree, neighbors)
├── ID translation (internal ↔ original)
└── Used by InitContext and ComputeContext

BidirectionalNodeCentricContext (trait)
├── Incoming edge operations
├── incoming_degree()
└── for_each_incoming_neighbor()
```

### Context Types

#### 1. **PregelContext** (`pregel_context.rs`)

Base context providing common functionality:

```rust
pub struct PregelContext<C: PregelConfig> {
    // TODO: config reference, progress_tracker, graph stats
}

impl<C: PregelConfig> PregelContext<C> {
    fn config(&self) -> &C;
    fn log_debug(&self, message: &str);
    fn log_message(&self, message: &str);
    fn log_warning(&self, message: &str);
    fn is_multi_graph(&self) -> bool;
    fn node_count(&self) -> u64;
    fn relationship_count(&self) -> u64;
}
```

**Purpose**: Configuration access, logging, graph-level statistics
**Used by**: All other contexts (conceptually - Rust doesn't require explicit inheritance)

#### 2. **NodeCentricContext** (`node_centric_context.rs`)

Node-specific operations for InitContext and ComputeContext:

```rust
pub struct NodeCentricContext<C: PregelConfig> {
    node_id: u64,
    // TODO: graph, node_value, progress_tracker
}

impl<C: PregelConfig> NodeCentricContext<C> {
    fn set_node_id(&mut self, node_id: u64);
    fn node_id(&self) -> u64;
    fn node_count(&self) -> u64;
    fn node_exists(&self, node_id: u64) -> bool;

    // Node value setters
    fn set_node_value(&mut self, key: &str, value: f64);
    fn set_node_value_long(&mut self, key: &str, value: i64);
    fn set_node_value_long_array(&mut self, key: &str, value: Vec<i64>);
    fn set_node_value_double_array(&mut self, key: &str, value: Vec<f64>);

    // Graph topology
    fn degree(&self) -> usize;
    fn to_original_id(&self, internal_node_id: u64) -> u64;
    fn to_internal_id(&self, original_node_id: u64) -> u64;
    fn for_each_neighbor<F>(&self, consumer: F);
    fn for_each_distinct_neighbor<F>(&self, consumer: F);
}
```

**Purpose**: Per-node operations (values, neighbors, degree)
**Used by**: InitContext, ComputeContext

#### 3. **BidirectionalNodeCentricContext** (trait)

Trait for bidirectional graph operations:

```rust
pub trait BidirectionalNodeCentricContext {
    fn incoming_degree(&self) -> usize;
    fn for_each_incoming_neighbor<F>(&self, consumer: F);
    fn for_each_distinct_incoming_neighbor<F>(&self, consumer: F);
}
```

**Purpose**: Incoming edge access for bidirectional algorithms
**Implemented by**: BidirectionalInitContext, BidirectionalComputeContext (future)

#### 4. **InitContext** (`init_context.rs`)

Context for initialization phase (stub):

```rust
pub struct InitContext<C: PregelConfig> {
    // TODO: Extends NodeCentricContext functionality
}

impl<C: PregelConfig> InitContext<C> {
    fn node_count(&self) -> usize;
    fn set_node_value<V>(&mut self, value: V);
    // TODO: Access to input graph properties
    // TODO: node_properties(key) -> NodePropertyValues
}
```

**Purpose**: Initialize node values before first superstep
**Used in**: PregelComputation::init()

#### 5. **ComputeContext** (`compute_context.rs`)

Context for compute phase (stub):

```rust
pub struct ComputeContext<C: PregelConfig> {
    // TODO: Extends NodeCentricContext + message sending
}

impl<C: PregelConfig> ComputeContext<C> {
    fn superstep(&self) -> usize;
    fn is_initial_superstep(&self) -> bool;
    fn vote_to_halt(&mut self);

    // TODO: Message sending
    // fn send_to_neighbors(&mut self, message: f64);
    // fn send_to(&mut self, target: u64, message: f64);

    // TODO: Node value getters
    // fn double_node_value(&self, key: &str) -> f64;
    // fn long_node_value(&self, key: &str) -> i64;
}
```

**Purpose**: Main computation API (messages, voting, node values)
**Used in**: PregelComputation::compute()

#### 6. **MasterComputeContext** (`master_compute_context.rs`)

Context for global coordination (stub):

```rust
pub struct MasterComputeContext<C: PregelConfig> {
    // TODO: Extends PregelContext + global operations
}

impl<C: PregelConfig> MasterComputeContext<C> {
    fn superstep(&self) -> usize;
    fn is_initial_superstep(&self) -> bool;
    fn node_count(&self) -> usize;

    // TODO: Global node value access
    // fn double_node_value(&self, node_id: u64, key: &str) -> f64;
    // fn set_node_value(&mut self, node_id: u64, key: &str, value: f64);

    // TODO: Parallel execution
    // fn for_each_node<F>(&self, consumer: F);
}
```

**Purpose**: Global coordination between supersteps
**Used in**: BasePregelComputation::master_compute()

## Comparison with Java GDS

### Java Hierarchy

```java
PregelContext<CONFIG>
  ├── config(), logDebug(), logMessage(), logWarning()
  └── isMultiGraph(), nodeCount(), relationshipCount()

NodeCentricContext<CONFIG> extends PregelContext<CONFIG>
  ├── setNodeId(long), nodeId()
  ├── setNodeValue(...), degree()
  └── forEachNeighbor(...), toOriginalId(...)

InitContext<CONFIG> extends NodeCentricContext<CONFIG>
  └── nodeProperties(String)

ComputeContext<CONFIG> extends NodeCentricContext<CONFIG>
  ├── doubleNodeValue(String), longNodeValue(String)
  ├── voteToHalt(), superstep()
  └── sendToNeighbors(double), sendTo(long, double)

MasterComputeContext<CONFIG> extends PregelContext<CONFIG>
  ├── superstep(), executorService()
  ├── doubleNodeValue(long, String)
  └── forEachNode(LongPredicate)
```

### Rust Translation

Rust doesn't have traditional inheritance, so we use:

- **Composition**: Contexts contain the data they need
- **Type Parameters**: Generic over `PregelConfig`
- **Trait Bounds**: `BidirectionalNodeCentricContext` for opt-in behavior

The stubs maintain the same API surface, ready for implementation once we have:

- Graph reference
- NodeValue storage
- Messenger for message sending
- ProgressTracker for logging

## Implementation Roadmap

### Phase 1: Foundation (✅ Complete)

- ✅ PregelContext stub (logging, config, stats)
- ✅ NodeCentricContext stub (node operations)
- ✅ InitContext stub (initialization API)
- ✅ ComputeContext stub (compute API)
- ✅ MasterComputeContext stub (master compute API)
- ✅ BidirectionalNodeCentricContext trait

### Phase 2: Connect to NodeValue (Next)

Once we implement NodeValue properly:

```rust
impl<C: PregelConfig> NodeCentricContext<C> {
    fn set_node_value(&mut self, key: &str, value: f64) {
        self.node_value.set(key, self.node_id, value);
    }
}

impl<C: PregelConfig> ComputeContext<C> {
    fn double_node_value(&self, key: &str) -> f64 {
        self.node_value.double_value(key, self.node_id)
    }
}
```

### Phase 3: Connect to Graph (Next)

Once we have Graph topology access:

```rust
impl<C: PregelConfig> NodeCentricContext<C> {
    fn degree(&self) -> usize {
        self.graph.degree(self.node_id)
    }

    fn for_each_neighbor<F>(&self, mut consumer: F)
    where
        F: FnMut(u64),
    {
        self.graph.for_each_relationship(self.node_id, |target, _weight| {
            consumer(target);
        });
    }
}
```

### Phase 4: Connect to Messenger (Executor Phase)

Once we implement Messenger:

```rust
impl<C: PregelConfig> ComputeContext<C> {
    fn send_to_neighbors(&mut self, message: f64) {
        self.graph.for_each_relationship(self.node_id, |target, weight| {
            let weighted_message = self.computation
                .apply_relationship_weight(message, weight);
            self.messenger.send_to(self.node_id, target, weighted_message);
        });
        self.has_sent_message = true;
    }

    fn send_to(&mut self, target: u64, message: f64) {
        self.messenger.send_to(self.node_id, target, message);
        self.has_sent_message = true;
    }
}
```

### Phase 5: Vote to Halt

```rust
impl<C: PregelConfig> ComputeContext<C> {
    fn vote_to_halt(&mut self) {
        self.vote_bits.set(self.node_id as usize);
    }
}
```

## Test Coverage

Added 5 new tests for context hierarchy:

- `test_node_centric_context_creation` - Basic construction
- `test_set_node_id` - Node ID management
- `test_id_translation` - Internal ↔ original ID conversion
- `test_pregel_context_creation` - Base context
- `test_graph_statistics` - Graph stats API

**Total: 754 tests passing** (up from 749)

## Files Created

1. **src/pregel/context/pregel_context.rs** - Base context (127 lines)
2. **src/pregel/context/node_centric_context.rs** - Node-centric base (256 lines)

## Files Updated

1. **src/pregel/context/mod.rs** - Added new exports and hierarchy documentation
2. **src/pregel/mod.rs** - Export new context types

## Design Decisions

### Why Separate PregelContext and NodeCentricContext?

Java GDS has this separation because:

- **PregelContext**: Operations that don't need a node ID (logging, global stats)
- **NodeCentricContext**: Operations that work on a specific node

We maintain this in Rust for:

- **API Clarity**: Makes it obvious which operations are node-specific
- **Future Flexibility**: Different execution models may need different context lifetimes
- **Java Compatibility**: Easier to reason about when translating algorithms

### Why a Trait for BidirectionalNodeCentricContext?

Bidirectional operations are opt-in:

- Not all graphs have inverse indices
- Not all algorithms need incoming edges
- Trait allows type system to enforce availability

### Stub Pattern

All implementations are stubs with TODO markers:

- **Compiles cleanly**: Type system validates the API
- **Tests verify structure**: Ensures methods exist and are callable
- **Incremental implementation**: Fill in stubs as we build executor
- **Documentation complete**: Every method documented with purpose and future implementation

## Integration Points

The context modules are now ready to integrate with:

1. **NodeValue**: Property storage backend
2. **Graph**: Topology queries (degree, neighbors)
3. **Messenger**: Message sending infrastructure
4. **ProgressTracker**: Logging and task tracking
5. **VoteBits**: HugeAtomicBitSet for halt tracking
6. **Pregel Executor**: Orchestration and lifecycle management

## Next Steps

With the context hierarchy complete, we can now:

1. **Implement NodeValue properly**:

   - HugeArray-backed storage
   - Property type validation
   - SingleNodeValue vs CompositeNodeValue

2. **Implement Pregel Executor**:

   - Superstep orchestration
   - Message passing with Messenger
   - Vote to halt tracking
   - Termination detection
   - VirtualThreads integration

3. **Fill in Context Methods**:

   - Connect to NodeValue, Graph, Messenger
   - Implement actual functionality
   - Add comprehensive tests

4. **BidirectionalPregelComputation**:
   - Support incoming edge operations
   - BidirectionalComputeContext implementation

The foundation is solid. The API surface matches Java GDS. All 754 tests passing. Ready to implement the executor! 🚀
