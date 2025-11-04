## 07 — ML Functions (End-to-End Algorithms)

Complete ML algorithms in Rust using ML Core.

### ML Functions in ML Core

**Core principle**: ML Functions are complete algorithms, not just theory:
- Subgraph extraction and sampling
- Graph embeddings and representations
- Feature engineering from graph structure
- Integration with external ML libraries

### Subgraph Algorithms

**Sampling and extraction**:
- Random walk sampling
- Neighborhood sampling
- Graph coarsening
- Subgraph isomorphism patterns

### Graph Embeddings

Represent graphs/vertices as vectors for ML:
- Node2Vec: Random-walk based
- Graph2Vec: Whole graph embeddings
- Structural embeddings
- Integration with ML pipelines

### Feature Engineering

Extract ML features from graph structure:
- Graph-level features (size, density, clustering)
- Node-level features (centrality, degree, triangles)
- Edge-level features (Jaccard similarity, Adamic-Adar)
- These feed into ML models

### ML Core Integration

How ML Functions connect components:
```rust
// Extract subgraph features
let features = extract_features(&subgraph)?;

// Convert to ML Core Tensor
let tensor = MLTensor::from_graph_features(features)?;

// Apply ML model
let prediction = ml_model.predict(tensor)?;
```

### Real-World Applications

- Recommendation systems
- Fraud detection
- Social network analysis
- Knowledge graph completion
- Drug discovery

### Next: Production Scale

Once algorithms work, scale to billion-node graphs with Arrow.

