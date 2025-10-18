## Quick Reference: Graph API for Algorithms

**Use this when implementing algorithm storage layers.**

---

## Core API Summary

### Querying Node Cardinality

```rust
// Get total nodes
let node_count: u64 = graph.node_count();

// Get out-degree (number of outgoing edges)
let out_deg: usize = graph.degree(node_id);

// Get in-degree (number of incoming edges)
let in_deg: Option<usize> = graph.degree_inverse(node_id);

// Skip dangling nodes
if graph.degree(node_id) == 0 {
    continue;
}
```

**Type Notes**:

- `node_id` is `u64`
- `degree` returns `usize`
- Node IDs are MappedNodeId (u64 alias)

---

## Iterating Edges

### Basic Pattern (Outgoing Edges)

```rust
let fallback_weight = 1.0;  // Default for unweighted edges
let rel_stream = graph.stream_relationships(node_id, fallback_weight);

for rel_cursor in rel_stream {
    let target: u64 = rel_cursor.target_id();
    let weight: f64 = rel_cursor.property();

    // Process edge (node_id → target, weight)
}
```

### With Index Casting (For Array Access)

```rust
for rel_cursor in graph.stream_relationships(node_id, 1.0) {
    let target_idx = rel_cursor.target_id() as usize;
    let weight = rel_cursor.property();

    scores[target_idx] += weight * message_value;
}
```

### Incoming Edges (If Supported)

```rust
let rel_stream = graph.stream_inverse_relationships(node_id, 1.0);

for rel_cursor in rel_stream {
    let source: u64 = rel_cursor.source_id();  // Note: source_id
    let weight: f64 = rel_cursor.property();
}
```

---

## Common Patterns

### Message Passing (PageRank Style)

```rust
// For each node, send messages to neighbors
for source_id in 0..graph.node_count() {
    let source_idx = source_id as usize;
    let out_degree = graph.degree(source_id);

    if out_degree == 0 { continue; }  // Dangling node

    let contribution = scores[source_idx] / out_degree as f64;

    for rel_cursor in graph.stream_relationships(source_id, 1.0) {
        let target_idx = rel_cursor.target_id() as usize;
        let edge_weight = rel_cursor.property();

        let message = contribution * edge_weight;
        incoming_messages[target_idx].push((source_idx, message));
    }
}
```

### Aggregation (Community Detection Style)

```rust
// For each node, aggregate incoming data
for target_id in 0..graph.node_count() {
    let target_idx = target_id as usize;
    let in_degree = graph.degree_inverse(target_id).unwrap_or(0);

    if in_degree == 0 { continue; }  // No incoming edges

    let mut aggregated = 0.0;
    for rel_cursor in graph.stream_inverse_relationships(target_id, 1.0) {
        let source_idx = rel_cursor.source_id() as usize;
        let edge_weight = rel_cursor.property();

        aggregated += node_values[source_idx] * edge_weight;
    }

    new_values[target_idx] = aggregated / in_degree as f64;
}
```

### Checking Specific Edge

```rust
// Check if edge exists (relationship predicate)
let exists = graph.exists(source_id, target_id);

// Get degree without parallel edges (for deduplication)
let unique_degree = graph.degree_without_parallel_relationships(node_id);
```

---

## Type Conversions

| From           | To                | Pattern            |
| -------------- | ----------------- | ------------------ |
| `u64` node ID  | `usize` for array | `as usize`         |
| `usize` index  | `u64` node ID     | `as u64`           |
| `MappedNodeId` | Use directly      | Type alias for u64 |
| `f64` weight   | Integer degree    | `as usize`         |

---

## Error Handling

### Current Error Types

```rust
pub enum AlgorithmError {
    Execution(String),           // Computation failed
    Graph(String),               // Graph access failed
    Convergence(String),         // Failed to converge
    InvalidGraph(String),        // Graph doesn't match algorithm requirements
    ResourceExhausted(String),   // Memory/time limit exceeded
}
```

### Validation Pattern

```rust
pub fn validate_scores(&self, scores: &[f64]) -> Result<bool, AlgorithmError> {
    if scores.len() != self.graph.node_count() as usize {
        return Err(AlgorithmError::Execution(
            format!("Score array size {} != node count {}",
                    scores.len(),
                    self.graph.node_count())
        ));
    }

    for (i, score) in scores.iter().enumerate() {
        if !score.is_finite() {
            return Err(AlgorithmError::Execution(
                format!("Score {} is not finite: {}", i, score)
            ));
        }
    }

    Ok(true)
}
```

---

## Default Behavior

### Unweighted Graphs

```rust
// If graph has no relationship properties, use fallback
let fallback_weight = 1.0;
for rel_cursor in graph.stream_relationships(node_id, fallback_weight) {
    let weight = rel_cursor.property();  // Returns fallback_weight
}
```

### Directed vs Undirected

```rust
// Outgoing edges (always available)
graph.stream_relationships(node_id, 1.0)

// Incoming edges (may not be available, check docs)
graph.stream_inverse_relationships(node_id, 1.0)
```

### Dangling Nodes

```rust
// Nodes with out-degree 0
if graph.degree(node_id) == 0 {
    // This node sends no messages (standard behavior)
    // But it can receive messages
    continue;
}
```

---

## Performance Considerations

### Degree Lookup

```rust
// O(1) operation - safe to call frequently
let deg = graph.degree(node_id);
```

### Streaming Relationships

```rust
// O(degree) operation - lazy evaluation
// No allocations (cursor-based)
// Compression-aware (backend optimized)
for rel_cursor in graph.stream_relationships(node_id, 1.0) {
    // Process each edge
}
```

### Caching Pattern

```rust
// If degree is queried multiple times, might cache:
let degree = self.degree_cache
    .entry(node_id)
    .or_insert_with(|| graph.degree(node_id));

// But usually direct call is faster than HashMap lookup
```

---

## Concurrent Access

### Thread-Safe Usage

```rust
// Graph implements Send + Sync
let graph = Arc::new(graph);
let graph_clone = graph.clone();

// Safe to pass to threads
std::thread::spawn(move || {
    for rel_cursor in graph_clone.stream_relationships(0, 1.0) {
        // Process concurrently
    }
});
```

### Concurrent Iterator Copy

```rust
// If you need the graph in parallel iteration:
let graph_copy = graph.concurrent_copy();

// Now each thread can use graph_copy safely
```

---

## What's NOT Available (Yet)

### Relationship Type Filtering

```rust
// Not (yet) in base API:
// graph.stream_relationships_typed(node_id, &[rel_type1, rel_type2], fallback)

// Workaround: filter in algorithm code
for rel_cursor in graph.stream_relationships(node_id, 1.0) {
    if !interesting_types.contains(&rel_cursor.rel_type()) {
        continue;
    }
}
```

### Edge Properties Beyond Weight

```rust
// Only f64 property available per edge
let weight: f64 = rel_cursor.property();

// Other properties accessed via node/relationship property containers
```

### Bulk Edge Operations

```rust
// No: get_all_edges() -> Vec<(source, target, weight)>
// Instead: iterate per source node

// for source in 0..graph.node_count() {
//     for rel_cursor in graph.stream_relationships(source, 1.0) { ... }
// }
```

---

## Example: Implementing Betweenness Centrality

```rust
pub fn extract_dependencies(&self, node_id: u64)
    -> Result<Vec<(u64, f64)>, AlgorithmError>
{
    let mut dependencies = vec![];

    // For each incoming edge
    for rel_cursor in self.graph.stream_inverse_relationships(node_id, 1.0) {
        let source = rel_cursor.source_id();
        let weight = rel_cursor.property();

        dependencies.push((source, weight));
    }

    Ok(dependencies)
}
```

---

## Example: Implementing Label Propagation

```rust
pub fn gather_labels(&self, node_id: u64)
    -> Result<HashMap<Label, f64>, AlgorithmError>
{
    let mut label_scores = HashMap::new();
    let in_degree = self.graph.degree_inverse(node_id).unwrap_or(0);

    if in_degree == 0 {
        return Ok(label_scores);  // No incoming, no labels to gather
    }

    for rel_cursor in self.graph.stream_inverse_relationships(node_id, 1.0) {
        let source_idx = rel_cursor.source_id() as usize;
        let edge_weight = rel_cursor.property();

        let source_label = current_labels[source_idx];
        let contribution = edge_weight / in_degree as f64;

        *label_scores.entry(source_label).or_insert(0.0) += contribution;
    }

    Ok(label_scores)
}
```

---

## Debugging Tips

### Print Edge List

```rust
println!("Edges from node {}:", node_id);
for rel_cursor in graph.stream_relationships(node_id, 1.0) {
    println!("  {} -> {}: {}",
             node_id,
             rel_cursor.target_id(),
             rel_cursor.property());
}
```

### Validate Degree Consistency

```rust
for node_id in 0..graph.node_count() {
    let stated_degree = graph.degree(node_id);
    let actual_degree = graph.stream_relationships(node_id, 1.0).count();

    if stated_degree != actual_degree {
        eprintln!("Degree mismatch at node {}: stated={}, actual={}",
                  node_id, stated_degree, actual_degree);
    }
}
```

### Check for Dangling Nodes

```rust
let dangling_count = (0..graph.node_count())
    .filter(|node_id| graph.degree(*node_id) == 0)
    .count();

println!("Dangling nodes: {}/{}", dangling_count, graph.node_count());
```

---

## Links to Full Reference

- **Graph trait**: `src/types/graph/graph.rs`
- **Degrees trait**: `src/types/graph/degrees.rs`
- **RelationshipIterator**: `src/types/properties/relationship/traits/relationship_iterator.rs`
- **RelationshipCursor**: `src/types/properties/relationship/traits/relationship_cursor.rs`
- **AdjacencyList**: `src/types/graph/adj_list/adjacency_list.rs`

---

## Summary

**All you need**:

1. `graph.node_count()` — how many nodes
2. `graph.degree(node)` — node cardinality
3. `graph.stream_relationships(node, fallback)` — iterate outgoing edges
4. `rel_cursor.target_id()` — where edge goes
5. `rel_cursor.property()` — edge weight

**That's the entire API for most algorithms.** ✅
