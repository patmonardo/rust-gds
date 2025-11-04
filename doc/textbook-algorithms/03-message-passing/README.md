## 03 — Message Passing Patterns

Advanced Pregel patterns for specific algorithms

### Connected Components
Label propagation algorithm
- Each node maintains component ID
- Broadcast ID to neighbors, adopt minimum
- Converge when all nodes in component have same ID

**Implementation**: `archive/examples/04-computation/pregel_connected_components.rs`

### Single-Source Shortest Path
Dijkstra-like algorithm via message passing
- Initialize source to 0, others to INF
- Propagate distances, minimize at each node
- Used in routing and centrality metrics

### Triangle Counting
Count closed triplets for graph analysis
- Message passing to aggregate triangle counts
- Parallelizable via Pregel

### Personalized PageRank
PageRank from specific source nodes
- Propagate rank only from source set
- Useful for recommendation systems
- Already supported in our PageRank implementation

### Pattern: Graph-Level Features

All these algorithms produce **graph-level features**:
- Node-level: scores per node (PageRank, centrality)
- Edge-level: weights per edge (similarity, flow)
- Graph-level: aggregate metrics (density, clustering)

These features feed into ML models.

### Next: Tensor Integration

Once you compute features, expose them as tensors for ML Core.

