## PageRank Storage Implementation: Using the Real Graph API

**Status**: Ready to implement (no new Graph API needed)

---

## What We Discovered

The Graph system **already provides** what PageRank needs:

1. **Degree queries** (constant time): `graph.degree(node_id: u64) -> usize`
2. **Edge iteration** (cursor streaming): `graph.stream_relationships(node_id, fallback) -> RelationshipStream`
3. **Edge weights** (f64 per edge): `rel_cursor.property() -> f64`
4. **Node count**: `graph.node_count() -> u64`

These are exposed through the standard Graph traits:

- `Degrees` trait → `degree()` method
- `RelationshipIterator` trait → `stream_relationships()` method
- Each `RelationshipCursor` has `.property()` returning f64

---

## Current Stub (storage.rs extract_messages)

```rust
pub fn extract_messages(
    &self,
    _scores: &[f64],
) -> Result<Vec<Vec<EdgeMessage>>, AlgorithmError> {
    // Projector: reveals duality (STUB — awaits GraphStore edge iteration API)
    // TODO: iterate edges, produce messages per node
    Ok(vec![vec![]; self.graph.node_count() as usize])
}
```

---

## Implementation Using Existing API

### The Algorithm (in plain terms)

```
For each node in the graph:
    out_degree = graph.degree(node)
    if out_degree == 0, skip (dangling node sends no messages)

    contribution_per_edge = current_score[node] / out_degree

    For each outgoing edge (node → target) with weight:
        message_value = contribution_per_edge * weight
        incoming_messages[target].push((node, message_value))

Return incoming_messages (ready for accumulation)
```

### The Code

```rust
pub fn extract_messages(
    &self,
    scores: &[f64],
) -> Result<Vec<Vec<EdgeMessage>>, AlgorithmError> {
    let node_count = self.graph.node_count() as usize;
    let mut incoming_messages: Vec<Vec<EdgeMessage>> = vec![vec![]; node_count];

    // For each node, iterate its outgoing edges
    for source_id in 0..node_count {
        let source_mapped = source_id as u64;  // Map to GraphStore ID type

        // Step 1: Get out-degree (Validator verifies cardinality)
        let out_degree = self.graph.degree(source_mapped);

        // Step 2: Skip dangling nodes
        if out_degree == 0 {
            continue;
        }

        // Step 3: Calculate contribution per edge (damping-aware)
        let contribution = scores[source_id] / out_degree as f64;

        // Step 4: Iterate outgoing edges (Projector reveals duality)
        let fallback_weight = 1.0;  // Default for unweighted edges
        let rel_stream = self.graph.stream_relationships(source_mapped, fallback_weight);

        for rel_cursor in rel_stream {
            // Step 5: Extract target and weight from cursor
            let target_id = rel_cursor.target_id() as usize;
            let edge_weight = rel_cursor.property();  // f64

            // Step 6: Calculate message value
            let message_value = contribution * edge_weight;

            // Step 7: Queue message (will be accumulated in compute pole)
            incoming_messages[target_id].push(EdgeMessage {
                target_node: source_id,
                score_contribution: message_value,
            });
        }
    }

    Ok(incoming_messages)
}
```

---

## Why This Works

### Type Alignment

| What We Need           | Graph API Provides                              | Type                           | Notes                        |
| ---------------------- | ----------------------------------------------- | ------------------------------ | ---------------------------- |
| Number of nodes        | `graph.node_count()`                            | `u64`                          | Cast to `usize` for indexing |
| Out-degree of node     | `graph.degree(node_id)`                         | `usize`                        | Already correct type         |
| Iterate outgoing edges | `graph.stream_relationships(node_id, fallback)` | `Iterator<RelationshipCursor>` | Cursor-based, lazy           |
| Edge weight            | `rel_cursor.property()`                         | `f64`                          | Directly usable              |
| Target node ID         | `rel_cursor.target_id()`                        | `u64`                          | Cast to `usize` for indexing |

### Cursor Efficiency

The `RelationshipStream` uses a cursor-based model:

- **No allocation** for edges (they're streamed)
- **Supports compression** (VLong encoding for target IDs)
- **Supports backends**: HugeArray, Arrow, Sparse
- **Efficient for large graphs**: Doesn't materialize edge list

### Error Handling

What could go wrong?

1. **Graph access errors**: Wrapped by `graph.stream_relationships()` if needed
   - Already returns `RelationshipStream` (handles failures)
2. **Invalid node ID**: We iterate valid range (0..node_count)
   - No risk of out-of-bounds
3. **NaN/Inf in weights**: Could occur in edge properties
   - Can add validation if needed (optional)

---

## Get Out-Degree Caching (storage.rs)

Currently stubbed:

```rust
pub fn get_out_degree(&mut self, node_id: usize) -> Result<usize, AlgorithmError> {
    if let Some(&cached) = self.out_degree_cache.get(&node_id) {
        return Ok(cached);
    }

    // TODO: Query graph for out-degree
    Ok(0)
}
```

**Optimization note**: We compute out_degree in the loop above. Could cache if we want to avoid recomputation, but:

- **Currently**: Each iteration computes degree fresh (acceptable if degree is O(1) lookup)
- **Optimized**: Cache after first iteration (reduces degree() calls)

If caching matters:

```rust
pub fn get_out_degree(&mut self, node_id: u64) -> Result<usize, AlgorithmError> {
    let node_key = node_id as usize;

    if let Some(&cached) = self.out_degree_cache.get(&node_key) {
        return Ok(cached);
    }

    let degree = self.graph.degree(node_id);
    self.out_degree_cache.insert(node_key, degree);
    Ok(degree)
}
```

Then in `extract_messages()`, use `self.get_out_degree(source_mapped)?` instead of `self.graph.degree(source_mapped)`.

**Trade-off**: HashMap lookup vs. direct degree() call. For most graphs, direct call is faster. Cache only if profiling shows degree() is a bottleneck.

---

## Testing Strategy

### Unit Test (extract_messages)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_messages_simple_graph() {
        // Create a simple 3-node graph:
        // 0 → 1 (weight 1.0)
        // 0 → 2 (weight 1.0)  [dangling: 0 has out-degree 2]
        // 1 → 2 (weight 1.0)  [dangling: 1 has out-degree 1]
        // 2 has no outgoing edges [dangling: 2 has out-degree 0]

        // Scores: [1.0, 1.0, 1.0]
        // Expected messages:
        //   - To node 1: (0, 0.5)     [1.0 / 2 = 0.5]
        //   - To node 2: (0, 0.5), (1, 1.0)  [1.0/2 + 1.0/1]

        // Arrange
        let graph = create_test_graph_3_nodes();
        let storage = PageRankStorageRuntime::new(&graph);
        let scores = vec![1.0, 1.0, 1.0];

        // Act
        let messages = storage.extract_messages(&scores).unwrap();

        // Assert
        assert_eq!(messages[0].len(), 0);  // Node 0 receives nothing
        assert_eq!(messages[1].len(), 1);  // Node 1 receives from 0
        assert_eq!(messages[1][0].score_contribution, 0.5);

        assert_eq!(messages[2].len(), 2);  // Node 2 receives from 0 and 1
        let msg_from_0 = messages[2].iter().find(|m| m.target_node == 0).unwrap();
        let msg_from_1 = messages[2].iter().find(|m| m.target_node == 1).unwrap();
        assert_eq!(msg_from_0.score_contribution, 0.5);
        assert_eq!(msg_from_1.score_contribution, 1.0);
    }
}
```

---

## Summary: Ready to Implement

**Files to edit**:

1. `src/procedure/algo/pagerank/storage.rs` — Replace `extract_messages()` stub

**Graph API calls needed**:

- `graph.node_count()` ✅
- `graph.degree(node_id)` ✅
- `graph.stream_relationships(node_id, fallback)` ✅
- `rel_cursor.target_id()` ✅
- `rel_cursor.property()` ✅

**Lines of code**: ~30-40 (replace stub)

**Complexity**: Low (straightforward nested loop over cursors)

**Validation**: Test with deterministic 3-node graph

---

## Question for Reflection

**Is there any reason to NOT implement this now?**

The API is available, the pattern is clear, the error handling is straightforward. The only reason to delay would be if:

1. We want to review Java GDS first for validation ← **This is good practice**
2. We need to understand edge weight semantics better ← **Clarify: are unweighted edges weight=1.0?**
3. We're concerned about dangling node behavior ← **Standard: dangling nodes send no messages**

---

## Next: Java GDS Review

We should examine the original PageRank source to confirm:

1. How dangling nodes are handled (skip or special treatment?)
2. Whether edge weights are supported or assumed uniform
3. Whether the algorithm normalizes scores (optional in our version)
4. Whether relationship types are filtered (for multi-type graphs)

File to review: `/home/pat/GitHub/graph-data-science/algo/algo-core/src/main/java/org/neo4j/gds/pagerank/PageRank.java`

Once confirmed, we can implement with confidence.
