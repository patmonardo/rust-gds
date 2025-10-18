## Graph API Analysis: Current State & Opportunities for Algorithm Support

**Date**: October 18, 2025
**Scope**: Evaluating the current Graph system to understand what we have available for algorithm construction.

---

## 1. Current Graph API Layer

### 1.1 Core Graph Trait (`src/types/graph/graph.rs`)

The primary trait combines multiple concerns:

```rust
pub trait Graph:
    IdMap                      // Node ID mapping (external ↔ internal)
    + NodePropertyContainer    // Node properties access
    + Degrees                  // Node degree queries
    + RelationshipIterator     // Edge iteration
    + RelationshipProperties   // Edge property values
    + Send
    + Sync
```

**Key Responsibility Split:**

- **TopLevel** (Graph trait): Defines what a graph IS
- **Layer 1 - IdMap**: Manages node identity (external ID ↔ internal ID)
- **Layer 2 - Degrees**: Query how many neighbors a node has
- **Layer 3 - RelationshipIterator**: Iterate over edges (cursors)
- **Layer 4 - RelationshipProperties**: Access edge property values (weights)
- **Layer 5 - NodePropertyContainer**: Access node properties

### 1.2 Degrees Trait (`src/types/graph/degrees.rs`)

**Already Exists:**

```rust
pub trait Degrees {
    fn degree(&self, node_id: u64) -> usize;           // Out-degree
    fn degree_inverse(&self, node_id: u64) -> Option<usize>;  // In-degree
    fn degree_without_parallel_relationships(&self, node_id: u64) -> usize;
}
```

**Status**: ✅ We already have degree queries.

### 1.3 RelationshipIterator Trait (`src/types/properties/relationship/traits/relationship_iterator.rs`)

**Current API:**

```rust
pub trait RelationshipIterator: RelationshipPredicate + Send + Sync {
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,  // Default property value
    ) -> RelationshipStream<'a>;  // Box<dyn Iterator<Item = RelationshipCursorBox> + 'a>

    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;

    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator>;
}
```

**RelationshipCursor** (`src/types/properties/relationship/traits/relationship_cursor.rs`):

```rust
pub trait RelationshipCursor: Debug {
    fn source_id(&self) -> MappedNodeId;
    fn target_id(&self) -> MappedNodeId;
    fn property(&self) -> PropertyValue;  // PropertyValue = f64
}
```

**Status**: ✅ We can iterate edges with source, target, and weight (f64).

### 1.4 Adjacency List Layer (`src/types/graph/adj_list/`)

**Two-Level Cursor Model:**

1. **AdjacencyList** (course-grain):

   ```rust
   pub trait AdjacencyList: Send + Sync + Debug {
       fn degree(&self, node: MappedNodeId) -> usize;
       fn raw_adjacency_cursor(&self) -> Box<dyn AdjacencyCursor>;
       fn init_cursor(&self, cursor: &mut dyn AdjacencyCursor, node: MappedNodeId);
   }
   ```

2. **AdjacencyCursor** (fine-grain iteration):
   ```rust
   pub trait AdjacencyCursor: Send + Sync + Debug {
       fn init(&mut self, index: usize, degree: usize);
       fn size(&self) -> usize;
       fn remaining(&self) -> usize;
       fn has_next(&self) -> bool;
       fn next_vlong(&mut self) -> Option<MappedNodeId>;  // Compressed variable-length encoding
       fn peek_vlong(&self) -> Option<MappedNodeId>;
       fn skip_until(&mut self, node_id: MappedNodeId) -> Option<MappedNodeId>;
       fn advance(&mut self, node_id: MappedNodeId) -> Option<MappedNodeId>;
   }
   ```

**Status**: ✅ Low-level compression-aware edge access available.

---

## 2. What We Have for PageRank Right Now

**Available without any new code:**

### For Storage Pole (extract_messages):

```rust
// Given: node_id (current node), current_scores: Vec<f64>

// Step 1: Get out-degree (already have)
let out_degree = graph.degree(node_id);

// Step 2: Calculate contribution per edge
let contribution = current_scores[node_id] / out_degree as f64;

// Step 3: Iterate outgoing edges (already have)
let fallback_value = 0.0;  // or graph.default_property_value()
let cursor_stream = graph.stream_relationships(node_id, fallback_value);

for rel_cursor in cursor_stream {
    let target_id = rel_cursor.target_id();
    let edge_weight = rel_cursor.property();  // f64

    let message_value = contribution * edge_weight;
    incoming_messages[target_id].push((node_id, message_value));
}
```

**What was stubbed:**

- ✅ Degree queries → Already works via `graph.degree()`
- ✅ Edge iteration → Already works via `graph.stream_relationships()`
- ✅ Edge weights → Already accessible via `rel_cursor.property()`

**What to implement in storage.rs:**

**Current stub:**

```rust
pub fn extract_messages(&self, _scores: &[f64])
    -> Result<Vec<Vec<EdgeMessage>>, AlgorithmError>
{
    // Projector: reveals duality (STUB — awaits GraphStore edge iteration API)
    // TODO: iterate edges, produce messages per node
    Ok(vec![vec![]; self.graph.node_count()])  // Empty messages
}
```

**New implementation (using existing API):**

```rust
pub fn extract_messages(&self, scores: &[f64])
    -> Result<Vec<Vec<EdgeMessage>>, AlgorithmError>
{
    let node_count = self.graph.node_count();
    let mut incoming_messages: Vec<Vec<EdgeMessage>> = vec![vec![]; node_count];

    // For each node, iterate its outgoing edges
    for source_id in 0..node_count {
        let source_mapped = source_id as u64;
        let out_degree = self.graph.degree(source_mapped);

        if out_degree == 0 {
            continue;  // Dangling node, no messages to send
        }

        let contribution = scores[source_id] / out_degree as f64;
        let fallback = 0.0;  // Default weight for unweighted edges

        // Stream outgoing edges from this node
        let rel_stream = self.graph.stream_relationships(source_mapped, fallback);

        for rel_cursor in rel_stream {
            let target_id = rel_cursor.target_id() as usize;
            let edge_weight = rel_cursor.property();  // f64 weight

            let message_value = contribution * edge_weight;
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

## 3. Graph System Architecture (Mental Model)

### 3.1 Layering

```
                   ┌─────────────────────────────────┐
                   │   Algorithm Computation         │
                   │ (PageRank, Louvain, etc.)       │
                   └────────────┬────────────────────┘
                                │
                   ┌────────────▼────────────────────┐
                   │    Graph Trait                  │
                   │ (The public contract)           │
                   └────────────┬────────────────────┘
                                │
         ┌──────────────────────┼──────────────────────┐
         │                      │                      │
    ┌────▼────────┐    ┌────────▼──────┐    ┌────────▼──────┐
    │ IdMap        │    │ Degrees       │    │ RelationshipI │
    │ (ID mapping) │    │ (Topology)    │    │ terator       │
    │              │    │               │    │ (Edge streams)│
    └──────────────┘    └───────────────┘    └───────────────┘
         │
    ┌────▼──────────────────────────────┐
    │  Adjacency List (Low-level)       │
    │  - AdjacencyList trait            │
    │  - AdjacencyCursor trait          │
    │  - VLong compression support      │
    └───────────────────────────────────┘
         │
    ┌────▼──────────────────────────────┐
    │  Storage Backends                 │
    │  - HugeArray (dense, cursor-opt)  │
    │  - Arrow (columnar, zero-copy)    │
    │  - Sparse (HashMap-based)         │
    └───────────────────────────────────┘
```

### 3.2 Data Flow for Algorithm (e.g., PageRank)

```
Algorithm (execute loop)
    │
    ├─→ graph.degree(node)                    [Degrees trait]
    ├─→ graph.stream_relationships(node)      [RelationshipIterator]
    │   └─→ RelationshipCursor { source, target, property(weight) }
    │
    └─→ Accumulate messages into new scores
```

---

## 4. Opportunities for Graph API Enhancement

### 4.1 Minor Convenience Layer (Low Effort, High Value)

**Current**: Algorithms must iterate and filter
**Proposed**: Add helper methods to make patterns obvious

```rust
/// Extension trait for algorithm convenience
pub trait GraphAlgorithmExt: Graph {
    /// Iterate all outgoing edges for a node with automatic unweighting
    fn outgoing_edges(&self, node_id: MappedNodeId)
        -> Result<Vec<(MappedNodeId, f64)>, Error>
    {
        let fallback = 0.0;
        Ok(self.stream_relationships(node_id, fallback)
            .map(|cursor| (cursor.target_id(), cursor.property()))
            .collect())
    }

    /// Iterate with convenience closure (avoids boxing)
    fn for_each_outgoing_edge<F>(&self, node_id: MappedNodeId, mut handler: F)
        -> Result<(), Error>
    where
        F: FnMut(MappedNodeId, f64) -> Result<(), Error>
    {
        let fallback = 0.0;
        for cursor in self.stream_relationships(node_id, fallback) {
            handler(cursor.target_id(), cursor.property())?;
        }
        Ok(())
    }

    /// Message passing helper (abstracts the accumulation pattern)
    fn distribute_score_via_edges(
        &self,
        node_id: MappedNodeId,
        score: f64,
    ) -> Result<Vec<(MappedNodeId, f64)>, Error>
    {
        let degree = self.degree(node_id);
        if degree == 0 {
            return Ok(vec![]);
        }

        let contribution = score / degree as f64;
        self.outgoing_edges(node_id)?
            .into_iter()
            .map(|(target, weight)| {
                Ok((target, contribution * weight))
            })
            .collect()
    }
}

impl<T: Graph + ?Sized> GraphAlgorithmExt for T {}
```

### 4.2 Relationship Filtering (Medium Effort, Critical for Production)

**Current**: All relationships in one namespace
**Proposed**: Filter by relationship type

```rust
pub trait RelationshipTypeFiltered: Graph {
    /// Iterate edges of only specific types
    fn stream_relationships_typed<'a>(
        &'a self,
        node_id: MappedNodeId,
        rel_types: &[RelationshipType],  // Filter by type
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;
}
```

### 4.3 Bidirectional Operations (For Algorithms Like LPA)

**Current**: Must iterate forward and backward separately
**Proposed**: Single pass for both directions

```rust
pub trait BidirectionalGraph: Graph {
    /// Iterate incoming and outgoing in single pass
    fn stream_all_neighbors<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a>;  // Both directions
}
```

---

## 5. Immediate Path Forward

### 5.1 For PageRank (This Week)

**No new traits needed.** Use existing API:

```
storage.extract_messages():
    for each node:
        for each outgoing edge via stream_relationships():
            accumulate message
            push to incoming_messages[target]
```

### 5.2 For Future Algorithms (Louvain, LPA, etc.)

**Phase 1 - Convenience Extension** (1-2 hours):

- Add `GraphAlgorithmExt` trait with helpers
- Tests showing pattern usage

**Phase 2 - Filtering** (2-4 hours):

- Add `RelationshipTypeFiltered` support
- Necessary for multi-type graphs (e.g., IMDB with Movie→Actor, Actor→Writer)

**Phase 3 - Bidirectional** (1-2 hours):

- Add `BidirectionalGraph` for symmetric operations
- Useful for community detection, clustering

---

## 6. Key Insights About Current Architecture

### ✅ Strengths

1. **Already cursor-optimized**: RelationshipIterator uses streaming cursor model
2. **Already weight-aware**: PropertyValue (f64) attached to each edge
3. **Already degree-queryable**: No need to iterate to count neighbors
4. **Already ID-mapped**: Abstracts external vs. internal node IDs
5. **Already composition-ready**: Traits stack (Graph = IdMap + Degrees + RelationshipIterator + ...)

### ⚠️ Current Rough Edges

1. **No obvious algorithm pattern**: Developers must know to use:

   - `graph.degree(node)` for cardinality
   - `graph.stream_relationships(node, fallback)` for iteration
   - `rel_cursor.property()` for weights
   - These are well-designed but unintuitive without docs

2. **Relationship type filtering**: Must post-filter in algorithm code

   - Should be first-class in GraphStore

3. **Bidirectional iteration**: Requires two separate passes
   - Could be optimized with unified direction-agnostic iterator

### 🎯 Design Pattern Already in Place

The architecture already follows **separation of concerns**:

- **IdMap** = Identity concern
- **Degrees** = Cardinality concern
- **RelationshipIterator** = Traversal concern
- **RelationshipProperties** = Weight concern
- **NodePropertyContainer** = Node attributes concern

This is exactly what Validator-Projector-Functor needs!

---

## 7. Recommended Next Steps

### Step 1: Use What We Have (This Session)

```rust
// PageRank storage.rs extract_messages() implementation
// Using: graph.degree(), graph.stream_relationships(), rel_cursor.property()
// Result: ✅ Algorithm works with existing API
```

### Step 2: Document the Pattern (Next Session)

```rust
// Create doc/ALGORITHM_GRAPH_API_PATTERNS.md
// Show:
//   - How to query degree
//   - How to iterate edges with weights
//   - How to accumulate messages
//   - How to handle dangling nodes
```

### Step 3: Add Convenience Layer (Post-PageRank)

```rust
// src/types/graph/algorithm_ext.rs
// pub trait GraphAlgorithmExt
// Methods: outgoing_edges(), for_each_outgoing_edge(), distribute_score_via_edges()
// Goal: Make patterns obvious to future algorithm developers
```

### Step 4: Type-Filter Support (For Production Multi-Type Graphs)

```rust
// Later: support relationship_type_filtered_graph() usage in algorithms
// Enable algorithms on specific relationship namespaces
```

---

## 8. Conclusion: The Graph System is Ready

The current Graph API provides **everything we need** for PageRank and most standard GDS algorithms:

- ✅ Degree queries (constant time)
- ✅ Edge iteration (cursor-based, efficient)
- ✅ Weight access (f64 per edge)
- ✅ Node ID mapping (abstract external IDs)
- ✅ Property containers (node and edge attributes)

**The missing piece is not capability—it's clarity.**

We should:

1. **Implement PageRank using existing API** (confirms fitness)
2. **Document the pattern** (helps future developers)
3. **Add convenience extension** (reduces boilerplate)
4. **Plan filtering & bidirectional** (for production scale)

The Functor is real. The storage (Prajna) talks to computation (Jnana) via a clean interface. We just needed to see it.
