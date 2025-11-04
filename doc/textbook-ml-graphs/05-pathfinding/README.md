## 05 — Pathfinding & Graph Algorithms

**Foundation algorithms for graph ML**

Algorithm building blocks that produce features for ML models.

### Pathfinding Algorithms

**Dijkstra's Algorithm**:
- Single-source shortest paths (weighted)
- Foundation for many centrality measures
- ML features: distance-based features

**A* Search**:
- Heuristic-guided pathfinding
- Weighted graphs with node priorities
- Efficient for routing and navigation

**Bellman-Ford**:
- Handles negative edge weights
- Distributed shortest paths
- Negative cycles detection

### Graph Centrality Measures

Produce node-level features for ML:
- **Betweenness**: Number of shortest paths passing through
- **Closeness**: Average distance to all other nodes
- **Harmonic**: Weighted closeness metric
- **Eigenvector**: Influence via connected neighbors

### Community Detection

Graph structure analysis:
- **Louvain**: Modularity optimization
- **Label Propagation**: Fast community identification
- **Spectral**: Eigenvalue-based partitioning
- Use communities as graph-level features

### Why These Matter for ML

**Graph algorithms → ML features**:
- Pathfinding yields distance features
- Centrality yields importance features
- Communities yield structural features
- All feed into ML models

### Integration with ML Core

Once algorithms compute features, expose as tensors:
```rust
let distances = dijkstra(&graph, source)?;
let tensor = MLTensor::from_features(distances)?;
```

### Next: Tensor Operations

Learn how ML Core structures organize these features.
