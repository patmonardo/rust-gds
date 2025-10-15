# Context Implementation Plan for ComputeStep

**Goal**: Wire up contexts with minimum viable API for ComputeStep to work

## What ComputeStep Actually Needs

From analyzing `ComputeStep.computeBatch()` in Java/TypeScript:

```rust
// In computeBatch():
1. computeContext.isInitialSuperstep() -> bool
2. initContext.setNodeId(node_id)
3. initFunction.init(initContext)
4. messenger.initMessageIterator(...)
5. messages.isEmpty() -> bool
6. voteBits.get(node_id) -> bool
7. voteBits.clear(node_id)
8. computeContext.setNodeId(node_id)
9. computeFunction.compute(computeContext, messages)
10. Optional: computeContext.hasSentMessage() -> bool
```

## Minimum Viable Context API

### NodeCentricContext (Base)

```rust
pub struct NodeCentricContext<C: PregelConfig> {
    node_id: u64,
    config: C,
    // Later: graph: Arc<Graph>, node_value: Arc<NodeValue>
}

impl<C: PregelConfig> NodeCentricContext<C> {
    pub fn set_node_id(&mut self, node_id: u64);  // ✅ EXISTS
    pub fn node_id(&self) -> u64;  // ✅ EXISTS
    pub fn node_count(&self) -> u64;  // Stub for now
}
```

### InitContext

```rust
pub struct InitContext<C: PregelConfig> {
    base: NodeCentricContext<C>,
}

impl<C: PregelConfig> InitContext<C> {
    pub fn new(config: C) -> Self;
    pub fn set_node_id(&mut self, node_id: u64);  // NEEDED
    pub fn node_id(&self) -> u64;

    // For later when wiring NodeValue:
    // pub fn set_node_value(&mut self, key: &str, value: f64);
}
```

### ComputeContext

```rust
pub struct ComputeContext<C: PregelConfig> {
    base: NodeCentricContext<C>,
    iteration: usize,  // NEEDED for isInitialSuperstep()
}

impl<C: PregelConfig> ComputeContext<C> {
    pub fn new(config: C, iteration: usize) -> Self;  // NEEDED
    pub fn set_node_id(&mut self, node_id: u64);  // NEEDED
    pub fn node_id(&self) -> u64;
    pub fn is_initial_superstep(&self) -> bool;  // NEEDED
    pub fn superstep(&self) -> usize;  // EXISTS

    // For later:
    // pub fn vote_to_halt(&mut self);
    // pub fn send_to_neighbors(&mut self, message: f64);
    // pub fn has_sent_message(&self) -> bool;
}
```

## Implementation Strategy

### Phase 1: Make ComputeStep Compile ✅

1. Add `new()` constructors to contexts
2. Add `set_node_id()` delegation in InitContext/ComputeContext
3. Add `iteration` field to ComputeContext
4. Add `is_initial_superstep()` method

### Phase 2: Wire NodeValue (Later)

- Add Arc<NodeValue> to contexts
- Implement set_node_value() methods
- Implement get_node_value() methods

### Phase 3: Wire Graph (Later)

- Add Arc<Graph> to contexts
- Implement forEachNeighbor()
- Implement degree() queries
- Implement ID translation

### Phase 4: Wire Messenger (Later)

- Track sent messages in ComputeContext
- Implement sendToNeighbors()
- Implement sendTo(targetId, message)
- Implement voteToHalt()

## Current Changes Needed

**NodeCentricContext**: Add proper constructor
**InitContext**: Add delegation to set_node_id
**ComputeContext**: Add iteration field + is_initial_superstep()

This unblocks ComputeStep compilation!
