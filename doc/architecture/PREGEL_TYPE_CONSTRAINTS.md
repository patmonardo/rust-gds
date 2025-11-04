# Pregel Framework Type Constraints

**Date**: Current  
**Status**: Documentation - Type System Constraints  
**Purpose**: Document Pregel's hardcoded type constraints (u64 node IDs, f64 messages)

---

## Type Constraints

**Yes, you're correct!** Pregel framework is **hardcoded** to use:

1. **`u64` for node IDs** - All Pregel contexts and messaging infrastructure
2. **`f64` for messages** - All message values in Pregel

This is **built into the framework design** and matches Java GDS Pregel.

---

## Evidence from Code

### 1. Messages are f64

**Location**: `gds/src/pregel/messages.rs`

```rust
/// Iterator for messages in the Pregel computation.
/// Provides iteration over f64 values with optional sender tracking.
pub trait MessageIterator: Iterator<Item = f64> {
    fn sender(&self) -> Option<u64>;
}

pub struct Messages<I: MessageIterator> {
    iterator: I,
}

impl<I: MessageIterator> Iterator for Messages<I> {
    type Item = f64;  // ← Hardcoded f64
}
```

**Message sending**:
```rust
pub trait Messenger<ITERATOR: MessageIterator>: Send + Sync {
    fn send_to(&self, source_node_id: u64, target_node_id: u64, message: f64);  // ← f64
}
```

### 2. Node IDs are u64 in Pregel

**Location**: `gds/src/pregel/context/node_centric_context.rs`

```rust
pub struct NodeCentricContext<C: PregelRuntimeConfig> {
    node_id: u64,  // ← Hardcoded u64
    // ...
}

pub fn node_id(&self) -> u64 {  // ← Returns u64
    self.node_id
}

pub fn set_node_id(&mut self, node_id: u64) {  // ← Takes u64
    self.node_id = node_id;
}
```

**ComputeContext**:
```rust
pub fn send_to(&mut self, target: u64, message: f64) {  // ← u64 + f64
    // ...
}
```

### 3. Message Reducers are f64

**Location**: `gds/src/pregel/reducers.rs`

```rust
pub trait MessageReducer<T> {
    fn reduce(&self, current: T, message: T) -> T;
    fn identity(&self) -> T;
}

impl MessageReducer<f64> for SumReducer {  // ← Only f64 reducers
    fn reduce(&self, current: f64, message: f64) -> f64 { current + message }
    fn identity(&self) -> f64 { 0.0 }
}
```

---

## Type Mismatch: Graph vs Pregel

**Known Issue**: There's a type mismatch between Graph and Pregel:

- **Graph trait**: Uses `NodeId = i64` (from `id_map/mod.rs`)
- **Pregel framework**: Uses `u64` for node IDs internally

**How It's Handled**: Pregel converts internally:

```rust
// From compute_step.rs:
let node_id = node_id_usize as u64;  // Convert from Partition's usize → u64

// From node_centric_context.rs:
pub fn to_internal_id(&self, original_node_id: i64) -> u64 {
    // Converts Graph's NodeId (i64) → Pregel's u64
}

pub fn to_original_id_of(&self, internal_node_id: u64) -> i64 {
    // Converts Pregel's u64 → Graph's NodeId (i64)
}
```

**The conversion happens automatically** when calling Graph methods from Pregel contexts.

---

## Java GDS Alignment

**Java GDS Pregel** uses:
- `long` for node IDs (64-bit signed integer)
- `double` for messages (64-bit floating point)

**Rust Pregel** uses:
- `u64` for node IDs (64-bit unsigned integer)
- `f64` for messages (64-bit floating point) ✅ **Matches Java**

**Note**: Java's `long` is signed (`i64`), but Rust Pregel uses `u64`. This is handled via the conversion functions.

---

## Why These Constraints?

**f64 Messages**:
- Simplicity: Single message type simplifies framework design
- Performance: No type erasure overhead
- Common case: Most graph algorithms use numeric messages (PageRank, centrality, etc.)

**u64 Node IDs**:
- Framework internals: Message queues use u64 for indexing
- Consistency: All Pregel APIs use u64 for node IDs
- Performance: Direct indexing into arrays/queues

---

## Implications for Algorithm Implementers

**When implementing `PregelComputation`**:
1. **Messages**: Always `f64` values
2. **Node IDs**: Use `u64` when calling Pregel APIs
3. **Graph access**: Use `context.to_internal_id()` / `to_original_id_of()` for conversion

**Example**:
```rust
impl PregelComputation for MyAlgorithm {
    fn compute(&mut self, context: &mut ComputeContext<Self::Config>, messages: &mut Messages<I>) {
        let node_id = context.node_id();  // u64
        
        // To call Graph methods that use NodeId (i64):
        let graph_node_id = context.to_internal_id(node_id) as i64;  // u64 → i64
        let degree = context.degree();  // Pregel API handles conversion
        
        // Messages are always f64:
        for message_value in messages {  // message_value: f64
            // Process f64 message
        }
        
        // Send f64 message to u64 target:
        context.send_to(target_node_id, 42.0);  // u64, f64
    }
}
```

---

## Summary

- ✅ **Messages**: `f64` (hardcoded in framework)
- ✅ **Node IDs in Pregel**: `u64` (hardcoded in framework)
- ⚠️ **Graph Node IDs**: `i64` (NodeId = i64)
- ✅ **Conversion**: Handled automatically via `to_internal_id()` / `to_original_id_of()`

**This matches Java GDS Pregel architecture** where the framework enforces these types at the infrastructure level.

