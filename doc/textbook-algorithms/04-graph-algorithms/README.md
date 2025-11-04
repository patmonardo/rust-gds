## 04 — Graph Algorithms with ML Core Integration

Algorithms that produce features for ML models

### Graph Centrality Measures
- **PageRank**: Authority/influence (already implemented)
- **Betweenness**: Mediation, bottleneck identification
- **Closeness**: Distance-based importance
- **Eigenvector**: Influence via connected nodes

### Graph Embeddings
Represent nodes as vectors for ML
- Node2Vec: Random-walk based embeddings
- Graph Convolutional Networks: Feature aggregation
- Attention mechanisms for graph data

### Community Detection
- **Louvain**: Modularity optimization
- **Label Propagation**: Fast community identification
- **Spectral Clustering**: Eigenvalue-based partitioning

### Tensor Integration

Once algorithms compute features, expose as tensors:

```rust
// Graph algorithm produces node-level features
let features = pagerank.run(&graph)?;

// Convert to ML Core Tensor
let tensor = MLTensor::from_features(features);

// Use in ML pipeline
let prediction = model.predict(tensor)?;
```

### ML Core Structures

Integration with your ML Core:
- Feature extraction from graph algorithms
- Batch processing of multiple graphs
- Tensor operations on graph features

**Key Principle**: Graph algorithms produce features; ML Core consumes them.

### Coming Up: Rust Patterns

Where the rubber meets the road: Arc, Ref, Trait Objects in graph algorithm contexts.

