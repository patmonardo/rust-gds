## 02 — Pregel Data Structures

**Lectures 2-5: Core Pregel patterns**

Pregel is a vertex-centric message-passing framework. These lectures explore the data structures that make it work in Rust.

### Key Types

**MessageIterator<I>**: Stream messages to a vertex  
**Messages<I>**: Iterator over incoming messages  
**ComputeContext**: Current node, neighbors, superstep state  
**InitContext**: Initialization phase  
**MasterComputeContext**: Global coordination

### Message Reduction

Pregel aggregates multiple messages to the same target:
- **SumReducer**: Add messages (PageRank uses this)
- **MaxReducer**: Take maximum
- **MinReducer**: Take minimum
- Custom reducers for domain-specific aggregation

### Cursor-Based Graph Access

```rust
// Access neighbors via cursor
for cursor in graph.stream_relationships(node_id, fallback) {
    let target = cursor.target_id();
    let weight = cursor.property();
    context.send_to(target, message_value);
}
```

### Rust Patterns in Pregel

**Arc<Trait> vs Arc<dyn Trait>**:
- Reference stability vs type erasure tradeoffs
- Performance implications of trait objects
- When interior mutability (RefCell) is needed

**Shared State**:
- How to share GraphStore across supersteps
- Message buffers and synchronization
- Convergence detection across workers

### Lesson from the Codebase

You learned Arc/Ref patterns the hard way during codegen debugging. This course documents those patterns where they actually matter in graph algorithms.

**Example**: `gds/src/pregel/` shows how message passing works without Python bridges.

### Coming Up

After understanding Pregel structures, you'll implement more algorithms using these patterns.

