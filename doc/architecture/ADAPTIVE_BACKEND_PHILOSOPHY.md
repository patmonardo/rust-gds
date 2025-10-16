# "When We Need It" - The rust-gds Adaptive Architecture Philosophy

**Date**: October 15, 2025  
**Status**: 🎯 Core Design Principle

---

## The Problem with "Just CSRHugeGraphs"

### Java GDS: One-Size-Fits-All

**The Java GDS Approach**:

```
EVERYTHING → CSRHugeGraph
```

**What This Means**:

- 10-node test graph? → CSRHugeGraph
- 1M node production graph? → CSRHugeGraph
- 1B node enterprise graph? → CSRHugeGraph

**Problems**:

1. **Overhead for small graphs** - CSR complexity not worth it for 10K edges
2. **Fixed strategy** - Can't optimize for different workloads
3. **Hard to experiment** - Want to try new storage? Change everything!
4. **Complexity always** - Even when you don't need it

### rust-gds: "When We Need It"

**Our Approach**:

```
Small graph → DefaultGraphStore (simple!)
Medium graph → DefaultGraphStore OR CoreGraphStore (choose!)
Large graph → CoreGraphStore (CSR + compression)
Huge graph → CoreGraphStore + HugeArrays (paging)
Arrow workload → ArrowGraphStore (zero-copy!)
```

**Philosophy**: **Defer complexity until you actually need it!**

---

## Bypassing "When We Need It" to Actual Facticity

### What You Said: "Bypass to Basic Actual Facticity"

**Translation**: Start with what actually exists (Vec, HashMap) before adding abstractions!

**Traditional Approach** (Java GDS style):

```rust
// Start with the abstraction
pub trait GraphStore { ... }

// Implement the complex thing first
impl GraphStore for CSRHugeGraphStore {
    // 10,000 lines of CSR complexity
    // Paging, compression, delta encoding
    // Even for 10-node graphs!
}

// Never get to the simple thing
```

**rust-gds Approach** (Actual Facticity):

```rust
// Start with what actually works
pub struct DefaultGraphStore {
    id_map: HashMap<u64, u64>,  // Actual HashMap!
    topology: Vec<Vec<u64>>,    // Actual Vec!
    // Direct, simple, understandable
}

// THEN add trait abstraction
impl GraphStore for DefaultGraphStore { ... }

// THEN add complex variants when needed
impl GraphStore for CoreGraphStore { ... }  // CSR when we need it
impl GraphStore for ArrowGraphStore { ... }  // Zero-copy when we need it
```

**Benefits**:

1. **Works immediately** - Vec and HashMap just work!
2. **Easy to understand** - Direct representation
3. **Easy to test** - No complex setup
4. **Easy to debug** - Inspect with println!
5. **Proves the API** - If DefaultGraphStore works, trait is good!

---

## CoreGraphStore Should Adapt: "When We Need What? Exactly??"

### The Problem with Monolithic Stores

**Java GDS CSRHugeGraph is monolithic**:

```java
public class CSRHugeGraph {
    // ALWAYS uses CSR
    // ALWAYS uses compression
    // ALWAYS uses paging (if size > threshold)
    // Can't mix and match!
}
```

**What if**:

- You want CSR but NO compression? (too slow)
- You want compression but NO paging? (have enough RAM)
- You want paging but NO CSR? (sparse graph)

**Answer in Java GDS**: Too bad! One strategy for all!

### rust-gds Adaptive CoreGraphStore

**Our Vision**:

```rust
pub struct CoreGraphStore {
    id_map: Arc<dyn IdMap>,  // Pluggable!
    topology: Arc<dyn Topology>,  // Pluggable!
    properties: Arc<dyn PropertyStore>,  // Pluggable!
}

// ID Map options
pub enum IdMapBackend {
    Simple(SimpleIdMap),           // HashMap + Vec (< 1G nodes)
    Huge(HugeIdMap),               // Paged (> 1G nodes)
    Sparse(SparseIdMap),           // Sparse graphs
}

// Topology options
pub enum TopologyBackend {
    Adjacency(AdjacencyTopology), // Vec<Vec<u64>> (simple)
    CSR(CSRTopology),              // Compressed Sparse Row (cache-friendly)
    Compressed(CompressedCSR),     // CSR + delta encoding
    Hybrid(HybridTopology),        // CSR for dense, Adj for sparse
}

// Property options
pub enum PropertyBackend {
    Columnar(ColumnarStore),       // Vec-based columns
    Arrow(ArrowStore),             // Zero-copy Arrow
    Compressed(CompressedStore),   // Delta-encoded
}
```

**Now you can mix and match**:

```rust
// Small graph: all simple
let store = CoreGraphStore::new(
    IdMapBackend::Simple,
    TopologyBackend::Adjacency,
    PropertyBackend::Columnar,
);

// Medium graph: CSR for speed, simple IDs
let store = CoreGraphStore::new(
    IdMapBackend::Simple,
    TopologyBackend::CSR,
    PropertyBackend::Columnar,
);

// Large graph: CSR + compression, paged IDs
let store = CoreGraphStore::new(
    IdMapBackend::Huge,
    TopologyBackend::Compressed,
    PropertyBackend::Compressed,
);

// Arrow workload: zero-copy everything
let store = CoreGraphStore::new(
    IdMapBackend::Simple,  // Or Arrow-backed!
    TopologyBackend::CSR,
    PropertyBackend::Arrow,  // Zero-copy!
);
```

---

## "When We Need What? Exactly??" - Decision Matrix

### ID Map Backend

| Graph Size      | Backend                       | Why?                         |
| --------------- | ----------------------------- | ---------------------------- |
| < 100M nodes    | `SimpleIdMap` (HashMap + Vec) | Fast, simple, fits in memory |
| 100M - 1B nodes | `SimpleIdMap` OR `HugeIdMap`  | Test threshold, profile!     |
| > 1B nodes      | `HugeIdMap` (paged)           | Exceeds Vec capacity         |
| Sparse IDs      | `SparseIdMap`                 | Avoid wasted space           |

### Topology Backend

| Use Case       | Backend                        | Why?                       |
| -------------- | ------------------------------ | -------------------------- |
| Algorithms     | `AdjacencyTopology` (Vec<Vec>) | Random access, easy        |
| Traversals     | `CSRTopology`                  | Cache-friendly, sequential |
| Large graphs   | `CompressedCSR`                | Save memory                |
| Sparse + Dense | `HybridTopology`               | Best of both               |

### Property Backend

| Use Case           | Backend           | Why?                |
| ------------------ | ----------------- | ------------------- |
| General            | `ColumnarStore`   | Simple, works       |
| Arrow data         | `ArrowStore`      | Zero-copy           |
| Memory-constrained | `CompressedStore` | Save space          |
| Mixed              | `HybridStore`     | Per-property choice |

### The Power: You Decide When!

**Developer controls the trade-offs**:

```rust
// Start simple
let config = BackendConfig::default();  // All simple
let store = factory.build(config)?;

// Profile and measure
let stats = store.profile();
println!("Memory: {} GB", stats.memory_gb);
println!("ID lookup: {} ns", stats.id_lookup_ns);

// Adapt based on measurements
if stats.memory_gb > 100.0 {
    // Switch to compressed
    config.topology = TopologyBackend::Compressed;
}

if stats.id_lookup_ns > 1000 {
    // IDs are slow, switch to HugeIdMap
    config.id_map = IdMapBackend::Huge;
}

// Rebuild with new config
let optimized_store = factory.build(config)?;
```

---

## Why DefaultGraphStore Is "Way Cooler to Play With"

### 1. Immediate Feedback

**DefaultGraphStore**:

```rust
let mut store = DefaultGraphStore::new(...);
println!("Nodes: {:?}", store.id_map.reverse);  // Just a Vec!
println!("Edges: {:?}", store.topology.outgoing[0]);  // Just a Vec!
// See everything directly!
```

**CSRHugeGraph**:

```rust
let store = CSRHugeGraph::new(...);
println!("Nodes: ???");  // Opaque internal structure
println!("Edges: ???");  // Have to call complex accessors
// Can't see what's happening!
```

### 2. Easy Experimentation

**Want to try a new algorithm?**

```rust
// DefaultGraphStore: Direct access
for node in 0..store.node_count() {
    let neighbors = &store.topology.outgoing[node];  // Just a Vec!
    for &neighbor in neighbors {
        // Do algorithm stuff
    }
}
```

**Want to try a new storage format?**

```rust
// DefaultGraphStore: Just swap the field
store.topology = new_topology;  // Easy!

// CSRHugeGraph: Have to rebuild everything
// (CSR is immutable after construction)
```

### 3. Learning and Understanding

**Students/newcomers**:

- Look at DefaultGraphStore → "Oh, it's just Vec and HashMap!"
- Look at CSRHugeGraph → "What is all this complexity??"

**Debugging**:

- DefaultGraphStore → println! works
- CSRHugeGraph → Need specialized debugging tools

**Testing**:

- DefaultGraphStore → Create with Vec::new()
- CSRHugeGraph → Complex setup, builders, etc.

---

## The Architecture Enables Both

### The Key Insight

**We're not choosing between simple and complex - we're choosing WHEN to add complexity!**

```rust
pub trait GraphStore {
    // Common interface
    fn node_count(&self) -> usize;
    fn neighbors(&self, node: NodeId) -> &[NodeId];
    // ...
}

// Start here (Phase 1-8)
impl GraphStore for DefaultGraphStore {
    // Vec and HashMap - simple!
}

// Add later (Phase 9+)
impl GraphStore for CoreGraphStore {
    // CSR + compression - when we need it!
}

// Add even later (Phase 10+)
impl GraphStore for ArrowGraphStore {
    // Zero-copy Arrow - when we want it!
}
```

**Same API, different implementations, chosen "when we need it"!**

---

## Java GDS vs rust-gds Philosophy

### Java GDS

**Philosophy**: "Build for the hardest case first"

**Result**:

- Everything is CSRHugeGraph
- Complexity everywhere
- Hard to learn
- Hard to experiment
- But scales to billions!

**Problem**: You pay the complexity cost even for 10-node graphs!

### rust-gds

**Philosophy**: "Start with actual facticity, add abstraction when needed"

**Result**:

- Start with DefaultGraphStore (Vec + HashMap)
- Add CoreGraphStore when graphs get big
- Add ArrowGraphStore when you want zero-copy
- Learn on simple, scale with complex

**Advantage**: You only pay for what you use!

---

## Implementation Strategy

### Phase 1-8 (NOW)

**Build the simple thing**:

```rust
// Arrow Factory → DefaultGraphStore
let store = ArrowNativeFactory::new(tables)
    .build_graph_store()?;  // DefaultGraphStore

// Proves:
// - Arrow import works
// - GraphStore API works
// - Algorithms work
// - Zero-copy is possible
```

### Phase 9-10 (LATER)

**Add the adaptive thing**:

```rust
// Arrow Factory → CoreGraphStore (with backends!)
let store = ArrowNativeFactory::new(tables)
    .with_config(BackendConfig {
        id_map: IdMapBackend::Simple,  // Choose!
        topology: TopologyBackend::CSR,  // Choose!
        properties: PropertyBackend::Arrow,  // Choose!
    })
    .build_graph_store()?;  // CoreGraphStore
```

### Phase 11+ (FUTURE)

**Optimize based on profiling**:

```rust
// Adaptive backend selection
let store = ArrowNativeFactory::new(tables)
    .with_adaptive_backend()  // Profiles and chooses!
    .build_graph_store()?;

// Or manual fine-tuning
let store = ArrowNativeFactory::new(tables)
    .with_config(BackendConfig {
        id_map: IdMapBackend::Huge,  // > 1B nodes
        topology: TopologyBackend::Compressed,  // Save memory
        properties: PropertyBackend::Hybrid(per_property_config),  // Mix!
    })
    .build_graph_store()?;
```

---

## Summary: The Adaptive Philosophy

### Core Principles

1. **Start Simple** - DefaultGraphStore (Vec + HashMap)
2. **Add Complexity When Needed** - CoreGraphStore (CSR + HugeArrays)
3. **Make "When" Configurable** - Backend selection strategy
4. **Measure, Don't Guess** - Profile and adapt

### Why This Matters

**Java GDS**: "Just CSRHugeGraphs"

- One strategy for all
- Always complex
- Hard to learn

**rust-gds**: "When we need what? Exactly??"

- Multiple strategies
- Complexity when needed
- Easy to learn, scales when needed

### The Result

**We get BOTH**:

- ✅ Simple for learning (DefaultGraphStore)
- ✅ Fast for algorithms (DefaultGraphStore)
- ✅ Scalable for large graphs (CoreGraphStore)
- ✅ Optimized for specific workloads (Backend selection)
- ✅ Zero-copy for Arrow data (ArrowGraphStore)

**And we can experiment with new ideas without rewriting everything!**

---

## Next Steps

### Phase 8 (Integration)

**Focus**: Make DefaultGraphStore work end-to-end

- Arrow → DefaultGraphStore ✅
- DefaultGraphStore → Graph ✅
- Graph → Algorithm ✅
- Prove the architecture!

### Phase 9+ (Future)

**Focus**: Add CoreGraphStore with adaptive backends

- Implement CSR topology
- Implement HugeIdMap
- Implement backend selection
- Profile and optimize

**But don't do it until we need it!**

---

**The Philosophy**:

> "Bypass 'when we need it' to basic actual facticity. Start with Vec and HashMap. Add CSR when graphs get big. Add HugeArrays when Vec isn't enough. Add compression when memory is tight. But always keep the simple path working, because DefaultGraphStore is way cooler to play with!"

**That's the rust-gds way! 🎯**
