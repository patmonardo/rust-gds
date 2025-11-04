# Pregel Type System Isolation

**Date**: Current  
**Status**: Documentation - Understanding Pregel's Isolated Type System  
**Purpose**: Document the three-way type split and why Pregel is an isolated execution system

---

## The Type System Split

**Pregel wants to be its own isolated execution system**, and this creates a **three-way type split**:

1. **Graph API**: `NodeId = i64` (signed, matches Java GDS)
2. **Pregel Framework**: `u64` (unsigned, for internal indexing)
3. **NodeValue Storage**: `usize` (for array indexing)

---

## Evidence of Isolation

### 1. NodeValue Uses usize for Storage

**Location**: `gds/src/pregel/node_value.rs`

```rust
/// Get a double value for a specific node and property.
pub fn double_value(&self, key: &str, node_id: usize) -> f64 {
    // Uses usize for direct array indexing
    PropertyArray::Double(arr) => arr.get(node_id),
}

pub fn set(&mut self, key: &str, node_id: usize, value: f64) {
    // usize for array access
    PropertyArray::Double(arr) => arr.set(node_id, value),
}
```

**Why usize?**
- Direct array indexing: `HugeDoubleArray.get(usize)` for performance
- Array sizes use `usize` in Rust
- Fast path: no conversion needed for array access

### 2. Pregel Contexts Use u64

**Location**: `gds/src/pregel/context/node_centric_context.rs`

```rust
pub struct NodeCentricContext<C: PregelRuntimeConfig> {
    node_id: u64,  // ← Pregel's internal ID type
    // ...
}

pub fn set_node_value(&mut self, key: &str, value: f64) {
    self.node_value
        .write()
        .set(key, self.node_id as usize, value);  // ← u64 → usize conversion
}
```

**Why u64?**
- Framework convention: all Pregel APIs use `u64`
- Message system: `send_to(source: u64, target: u64, message: f64)`
- Consistency: one type for all Pregel operations

### 3. Graph Uses i64 (NodeId)

**Location**: `gds/src/types/graph/id_map/mod.rs`

```rust
pub type NodeId = i64;  // ← Graph API type
```

**Why i64?**
- Java GDS alignment: matches Java `Long` type
- Original/External IDs can be negative (e.g., -1 for "not found")
- Graph store convention

---

## The Conversion Points

### Conversion 1: Pregel u64 → NodeValue usize

**Location**: `node_centric_context.rs` line 146

```rust
pub fn set_node_value(&mut self, key: &str, value: f64) {
    self.node_value.write().set(key, self.node_id as usize, value);
    //              ↑ u64                              ↑ usize
}
```

**Pattern**: Every time Pregel context writes to NodeValue, it converts `u64 → usize`

### Conversion 2: Graph i64 → Pregel u64

**Location**: `node_centric_context.rs` lines 246-250

```rust
pub fn to_internal_id(&self, original_node_id: i64) -> u64 {
    self.graph
        .to_mapped_node_id(original_node_id)  // i64 → i64
        .expect("node should exist in graph") as u64  // i64 → u64
}

// Used when calling Graph methods:
pub fn degree(&self) -> usize {
    self.graph.degree(self.node_id as i64) as usize
    //                    ↑ u64 → i64        ↑ usize
}
```

**Pattern**: When Pregel calls Graph methods, it converts `u64 → i64`

### Conversion 3: NodeValue usize → Pregel u64

**Location**: `master_compute_context.rs`

```rust
pub fn double_node_value(&self, node_id: usize, key: &str) -> f64 {
    self.node_values.read().double_value(key, node_id)
    //                                  ↑ usize in NodeValue
}

// But Pregel caller uses u64:
let value = context.double_node_value(node_id, "rank");
//                              ↑ u64, but converted to usize inside
```

**Pattern**: MasterComputeContext converts `u64 → usize` when reading from NodeValue

---

## Why This Isolation?

**Pregel as BSP Framework**:
- Wants to be **portable** (could work with different graph backends)
- Wants **consistent API** (all node IDs are u64 in Pregel)
- Wants **performance** (direct usize array indexing in NodeValue)
- Wants **abstraction** (hides Graph's i64 convention)

**The Trade-off**:
- Conversion overhead at boundaries (minimal, mostly casts)
- Type system complexity (three types, three conversion points)
- Mental overhead (gotta remember which type where)

---

## The Type Flow

**Algorithm Execution Flow**:

```
User calls Pregel.execute()
    ↓
PregelExecutor loads Graph (i64 node IDs)
    ↓
Graph.get_graph() → Arc<dyn Graph> (i64 NodeId)
    ↓
Pregel contexts convert: i64 → u64 (via to_internal_id)
    ↓
ComputeContext.node_id() → u64
    ↓
Write to NodeValue: u64 → usize (as usize cast)
    ↓
NodeValue storage: usize (direct array index)
    ↓
Read from NodeValue: usize → u64 (when accessed via context)
```

**Message Passing Flow**:

```
ComputeContext.node_id() → u64 (current node)
    ↓
context.send_to(target: u64, message: f64)  // Pregel types
    ↓
Messenger.send_to(source: u64, target: u64, message: f64)
    ↓
Message queues indexed by u64
    ↓
Next superstep: Messages<f64> iterator
```

---

## The Insight

**Pregel is an isolated execution system** that:
1. **Owns its types**: `u64` for nodes, `f64` for messages
2. **Abstracts Graph**: Converts to/from Graph's `i64` NodeId
3. **Uses usize for storage**: Direct array indexing for performance

**This matches Java GDS Pregel**:
- Java Pregel uses `long` (i64) but internally converts to array indices
- Rust Pregel makes the conversion explicit: `u64` (framework) → `usize` (storage)

**Why separate?**
- **Graph API** = Store-bound, schema-aware (Real Type:Value system)
- **Pregel Runtime** = Ephemeral, schema-free computation (Ideal Type:Value system)
- **NodeValue** = Columnar storage for BSP state (optimized for iteration)

**The conversion points are the Functor boundaries**:
- Graph → Pregel = Real → Ideal (Storage → Computation)
- Pregel → NodeValue = Framework → Storage (Computation → State)

---

## Summary

**Three Type Systems**:
1. **Graph**: `NodeId = i64` (store-bound, Java-aligned)
2. **Pregel**: `u64` (framework convention, isolated)
3. **NodeValue**: `usize` (array indexing, performance)

**Conversion Happens At**:
- Graph → Pregel: `to_internal_id(i64) → u64`
- Pregel → NodeValue: `u64 → usize` (as usize cast)
- NodeValue → Pregel: `usize → u64` (implicit in API design)

**The Isolation Serves**:
- Portability (could swap Graph implementation)
- Performance (direct usize array access)
- Abstraction (hides Graph's type conventions)

**Status**: This is intentional isolation, not accidental complexity. Pregel wants to be its own thing.

