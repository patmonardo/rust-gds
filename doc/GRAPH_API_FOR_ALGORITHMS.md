## Graph API for Algorithm Construction

**Problem**: PageRank stubs show we need clear, specialized APIs for the Storage and Computation poles to interact with GraphStore.

Currently:

```rust
// What we want to call:
storage.validate_scores(&scores)?;
storage.extract_messages(&current_scores)?;  // ← Needs edge iteration!
storage.get_out_degree(node_id)?;            // ← Needs degree queries!

compute.accumulate_scores(&messages, &mut new_scores)?;
compute.apply_damping(&mut scores, initial, factor)?;
compute.compute_residual(&old, &new)?;
```

**What GraphStore needs to provide:**

- Edge iteration (for message extraction)
- Node degree queries (out-degree, in-degree, total)
- Property value access (already have this via PropertyValues)
- Relationship type filtering (optional, for future)

---

## Proposed: Specialized Graph APIs for Algo Computation

### 1. Edge Access Traits

```rust
/// Edge iteration capability
pub trait EdgeIterator {
    /// Iterate outgoing edges from a node
    /// Returns: (target_node_id, relationship_type, weight?)
    fn outgoing_edges(&self, node_id: usize) -> Result<Vec<(usize, u32, Option<f64>)>, Error>;

    /// Iterate incoming edges to a node
    fn incoming_edges(&self, node_id: usize) -> Result<Vec<(usize, u32, Option<f64>)>, Error>;

    /// Iterate all edges in the graph
    fn all_edges(&self) -> Result<EdgeCursor, Error>;
}

/// Degree queries
pub trait DegreeProvider {
    /// Out-degree (number of outgoing edges)
    fn out_degree(&self, node_id: usize) -> Result<usize, Error>;

    /// In-degree (number of incoming edges)
    fn in_degree(&self, node_id: usize) -> Result<usize, Error>;

    /// Total degree (in + out)
    fn total_degree(&self, node_id: usize) -> Result<usize, Error>;
}
```

### 2. Specialized Storage Runtime API

```rust
/// For algorithms that need edge-based computation
pub trait AlgorithmStorageRuntime<G: GraphStore> {
    /// Initialize from graph
    fn new(graph: &G) -> Self;

    /// Validate current state (Validator's role)
    fn validate(&self, state: &AlgorithmState) -> Result<bool, Error>;

    /// Extract computation input from storage (Projector's role)
    fn extract_for_computation(&self, state: &AlgorithmState) -> Result<ComputationInput, Error>;

    /// Reify computation result back to storage form (Functor's role)
    fn reify_from_computation(&self, result: ComputationOutput) -> Result<AlgorithmState, Error>;
}
```

### 3. Cursor-Based Edge Iteration (Efficient)

For large graphs, avoid allocating Vec<Edge>:

```rust
/// Cursor over edges (lazy iteration)
pub trait EdgeCursor {
    /// Current edge (node_id, target_id, weight)
    fn current(&self) -> (usize, usize, Option<f64>);

    /// Move to next edge
    fn next(&mut self) -> bool;

    /// Reset to beginning
    fn reset(&mut self);
}

/// Outgoing edges from specific node (cursor-based)
pub trait OutgoingEdgeCursor {
    fn for_node(&self, node_id: usize) -> Result<impl EdgeCursor, Error>;
}
```

### 4. Message-Passing API (For Pregel-style Algorithms)

```rust
/// Message aggregation across edges
pub trait MessagePassing {
    /// Extract outgoing messages from scores
    fn extract_messages(
        &self,
        scores: &PropertyValues,
        damping: f64,
    ) -> Result<Vec<Vec<(usize, f64)>>, Error>;  // per-node incoming messages

    /// Efficient: process edges directly without intermediate Vec
    fn for_each_outgoing_edge<F>(
        &self,
        node_id: usize,
        scores: &PropertyValues,
        handler: F,
    ) -> Result<(), Error>
    where
        F: FnMut(usize, f64) -> Result<(), Error>;  // target_id, message_value
}
```

---

## Evolution Path

### Phase 1: Minimal (Current)

```
GraphStore trait
  └─ Basic: node_count(), edge_count(), node_labels()
```

### Phase 2: Storage Pole (Needed for PageRank)

```
GraphStore extends EdgeIterator
  └─ outgoing_edges(node_id) → Vec<(target, rel_type, weight)>
  └─ incoming_edges(node_id) → Vec<(source, rel_type, weight)>
```

### Phase 3: Computation Pole (For Efficiency)

```
GraphStore extends DegreeProvider
  └─ out_degree(node_id) → usize (cached)
  └─ in_degree(node_id) → usize (cached)
```

### Phase 4: Message-Passing (For Pregel)

```
GraphStore extends MessagePassing
  └─ for_each_outgoing_edge(...) → avoid allocations
```

---

## Questions for Graph System Review

1. **Edge Storage**: How are edges currently stored?

   - Adjacency list per node?
   - Adjacency matrix?
   - CSR (Compressed Sparse Row) format?

2. **Weight Semantics**: Are edge weights always present? Optional?

   - If optional, what's the default?
   - How are unweighted graphs represented?

3. **Relationship Types**: Are they used in filtering?

   - Should `outgoing_edges()` filter by type?
   - Or iterate all outgoing regardless?

4. **Degree Caching**: Should degree queries cache results?

   - Recompute each call?
   - Pre-computed during graph construction?

5. **Cursor Efficiency**: For billion-node graphs, should we avoid Vec allocation?
   - Iterator pattern (EdgeCursor)?
   - Or Vec<Edge> is fine?

---

## Implementation Priority

**For PageRank seed to work:**

1. Edge iteration API (priority: HIGH)
2. Degree queries (priority: HIGH)
3. Weight handling (priority: MEDIUM — can assume uniform 1.0 for now)
4. Message-passing optimizations (priority: LOW — future)

**Sketch for PageRank:**

```rust
// Storage pole: extract messages
let messages: Vec<Vec<(usize, f64)>> = (0..node_count)
    .map(|node_id| {
        let degree = graph.out_degree(node_id)?;
        if degree == 0 { return Ok(vec![]); }

        graph.outgoing_edges(node_id)?
            .iter()
            .map(|(target, _rel_type, weight)| {
                let w = weight.unwrap_or(1.0);
                let msg = scores[node_id] * w / degree as f64;
                (*target, msg)
            })
            .collect()
    })
    .collect::<Result<Vec<_>, _>>()?;

// Computation pole: accumulate messages
let mut new_scores = vec![0.0; node_count];
for (target, msg) in messages.into_iter().flatten() {
    new_scores[target] += msg;
}

// Damping
for score in &mut new_scores {
    *score = (1.0 - damping) / node_count as f64 + damping * *score;
}
```

---

## Next Steps

1. **Review GraphStore implementation** in `src/types/graph/`
2. **Identify edge storage structure** (adjacency list? CSR?)
3. **Design EdgeIterator trait** based on current storage
4. **Implement for existing backends** (HugeArray, Arrow, Sparse)
5. **Update PageRank storage.rs** to use new API

This is where the **Functor becomes real** — a clear interface between what Storage IS and what Computation NEEDS.
