## 01 — PageRank: Power Iteration

**This is Lecture 1 of the ML with Graphs course.**

PageRank is the foundational algorithm for graph centrality. It uses Power Iteration - the core of many graph ML algorithms.

### The Algorithm

```
PR(v) = (1-d) + d × Σ(PR(u) / out_degree(u))
```

**Where**:
- `PR(v)` = PageRank score of node v
- `d` = damping factor (typically 0.85)
- Sum over all nodes u that link to v
- Converges when L1 norm of changes < tolerance

### Our Implementation

**Location**: `gds/src/procedures/pagerank/pregel_computation.rs`

**Key components**:
1. **Schema**: Single "pagerank" double property per node
2. **Init**: Each node starts with alpha = (1-d)
3. **Compute**: 
   - Sum incoming messages (neighbors' rank contributions)
   - Apply formula: new_rank = alpha + damping_factor × sum
   - Send rank/out_degree to neighbors
   - Vote to halt if change < tolerance

### Why This Matters for ML

PageRank is the prototypical **graph-level feature**:
- Each node gets a single scalar score
- Scores capture graph structure (popularity, authority)
- Perfect input for ML models
- Powers many downstream algorithms

### Next Lectures

After PageRank, you'll spend 4 lectures on **Pregel data structures**:
- Message aggregation patterns
- Cursor-based graph access
- Rust Arc/Ref patterns (the hard lessons!)

