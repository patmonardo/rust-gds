## 06 — Tensors (ML Core Structures)

**Scalar → Vector → Matrix → Tensor: The classic data structure hierarchy**

Tensors are the bridge between graph algorithms and ML models.

### Tensor Data Structure

**Scalar** (rank-0 tensor):
- Single value
- Example: Node PageRank score

**Vector** (rank-1 tensor):
- One-dimensional array
- Example: All node PageRank scores

**Matrix** (rank-2 tensor):
- Two-dimensional array
- Example: Node embeddings (each row is a node's embedding)

**Tensor** (rank-N tensor):
- N-dimensional array
- Example: Batch of graphs with node features

### ML Core Tensor Representations

**Key Structures**:
- Scalar implementations for primitive types
- Vector implementations (Huge/Vec backed)
- Matrix and higher-rank tensors
- Type-erased tensor abstractions

### Feature Extraction

**From graph algorithms to tensors**:
```rust
// Algorithm produces node features
let pagerank_scores = run_pagerank(&graph)?;

// Convert to ML Core tensor
let features = NodeFeatures::from_scores(pagerank_scores);
let tensor = MLTensor::from_node_features(features)?;

// Use in ML pipeline
let prediction = model.predict(tensor)?;
```

### Tensor Operations

- Element-wise operations
- Matrix multiplication
- Aggregation (sum, mean, max)
- Broadcasting
- Gradient computation (for training)

### Integration Points

**GraphStore → Algorithm → Tensor → ML Model**:
1. Load graph from GraphStore
2. Run algorithm (PageRank, centrality, etc.)
3. Extract features as tensor
4. Feed to ML model
5. Output predictions

### Next: Complete ML Functions

End-to-end ML algorithms that tie it all together.
