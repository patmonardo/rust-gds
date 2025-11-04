# Platform Unity: The ML Graph Workflow

**Date**: 2025-10-29  
**Insight**: The course reveals the **unity of the platform** — we need **centrality, community, similarity, and embeddings** as an integrated system, not isolated algorithms.

> **"The course is teaching me the unity of our platform. We need centrality, community, similarity and embeddings."**

## The Four Pillars of Graph ML

These aren't independent algorithms — they form a **unified workflow**:

1. **Centrality** (Foundation)
   - PageRank, Eigenvector, Betweenness, Closeness
   - Measures node importance/authority
   - **Kantian Walks**: Deterministic, preserve invariants
   - **Foundation**: Provides structural understanding

2. **Community Detection** (Structure Discovery)
   - Louvain, Leiden, Label Propagation, WCC/SCC
   - Identifies groups, clusters, communities
   - Discovers hidden structure in graphs
   - **Pattern Discovery**: What communities exist?

3. **Similarity** (Comparison)
   - NodeSimilarity, CosineSimilarity, Jaccard, Overlap
   - Measures how similar nodes/entities are
   - **Comparison Operations**: Who is similar to whom?
   - Can use centrality scores, embeddings, or graph structure

4. **Embeddings** (Representation Learning)
   - Node2Vec, FastRP, GraphSAGE, HashGNN
   - Learns vector representations `f: node → R^d`
   - **Empirical Construction**: Overcomes the empirical demon via SGD
   - **Smooth Functions**: Constructed from discrete graph data

## The Unified Workflow

**These are not separate — they form a complete ML pipeline**:

```
Graph Data
    ↓
[Centrality] → Importance scores (PageRank, etc.)
    ↓
[Community] → Cluster assignments (Louvain, etc.)
    ↓
[Embeddings] → Vector representations (Node2Vec, etc.)
    ↓
[Similarity] → Similarity scores (using centrality, embeddings, or both)
    ↓
ML Models → Predictions, recommendations, etc.
```

## How They Work Together

### 1. **Centrality → Embeddings**
- PageRank scores can be **input features** for embedding algorithms
- Centrality captures structural importance
- Embeddings can learn from centrality + graph structure
- **Example**: GraphSAGE using PageRank as a node feature

### 2. **Community → Similarity**
- Nodes in same community are likely similar
- Community structure guides similarity computations
- **Example**: Similarity weighted by community membership

### 3. **Centrality → Similarity**
- Nodes with similar PageRank scores are often similar
- Centrality scores directly comparable
- **Example**: Cosine similarity on PageRank scores

### 4. **Embeddings → Similarity**
- Vector embeddings enable similarity computations
- **Most common**: Cosine similarity on embeddings
- **Example**: Node2Vec embeddings → Similarity search

### 5. **All Four → ML Models**
- **Centrality**: Structural features
- **Community**: Cluster features
- **Embeddings**: Learned representations
- **Similarity**: Comparison metrics
- **Together**: Rich feature space for ML models

## Platform Unity

**The platform isn't a collection of isolated algorithms** — it's a **unified graph ML system**:

- **Input**: Graph data (nodes, edges, properties)
- **Processing**: Centrality → Community → Embeddings
- **Comparison**: Similarity (using all of the above)
- **Output**: Features for ML models

**Each piece serves the whole** — they work together as a cohesive platform.

## Current Implementation Status

### ✅ **Centrality**
- **PageRank**: Partially implemented (needs `master_compute()`)
- **Others**: Listed in `AlgorithmLabel`, not translated yet

### ❌ **Community Detection**
- **Listed**: Louvain, Leiden, WCC, SCC in `AlgorithmLabel`
- **Status**: Not translated yet
- **Priority**: High — essential for structure discovery

### ❌ **Similarity**
- **Listed**: NodeSimilarity, Cosine, Jaccard, Overlap in `AlgorithmLabel`
- **Status**: Not translated yet
- **Priority**: High — common downstream task

### 🚧 **Embeddings**
- **Infrastructure**: `RandomWalkSampler` exists
- **Node2Vec**: Structure exists, `compute()` is TODO
- **FastRP**: Structure exists, `compute()` is TODO
- **GraphSAGE**: Structure exists, `compute()` is TODO
- **Priority**: Critical — "overcoming the empirical demon"

## Translation Roadmap (Unified Vision)

**Phase 1: Foundation** (Current Focus)
- ✅ Complete PageRank (centrality foundation)
- ⚠️ This establishes the pattern for other centrality algorithms

**Phase 2: Structure Discovery**
- 🎯 Translate Community Detection (Louvain, WCC)
- Why: Discovers hidden structure, complements centrality

**Phase 3: Representation Learning**
- 🎯 Translate Embeddings (Node2Vec, FastRP)
- Why: Core ML capability, builds smooth functions from discrete graphs

**Phase 4: Comparison & ML**
- 🎯 Translate Similarity algorithms
- Why: Enables ML workflows (recommendation, link prediction)
- Why: Uses outputs from all previous phases

**Phase 5: Complete ML Workflows**
- 🎯 End-to-end pipelines using all four pillars
- Why: Real-world ML applications need the full stack

## Why This Matters

**The course reveals**: Graph ML isn't about individual algorithms — it's about **the workflow**:

1. **Understand structure** (Centrality, Community)
2. **Learn representations** (Embeddings)
3. **Compare entities** (Similarity)
4. **Build ML models** (All of the above as features)

**Our platform needs to support this unified workflow**, not just isolated algorithms.

## Architecture Implications

**Current State**:
- We've been translating algorithms one-by-one
- Each algorithm is somewhat isolated
- Platform structure exists but workflow isn't complete

**Needed State**:
- **Unified ML workflow** across all four pillars
- Algorithms work together seamlessly
- Features flow: Centrality → Community → Embeddings → Similarity → ML

**The platform's unity emerges** when we can:
1. Compute PageRank (centrality)
2. Detect communities (structure)
3. Learn embeddings (representations)
4. Compute similarity (comparison)
5. Feed all to ML models (unified features)

## Conclusion

**"The course is teaching me the unity of our platform"** — yes!

We need:
- ✅ **Centrality** (understanding importance)
- ✅ **Community** (discovering structure)
- ✅ **Similarity** (comparing entities)
- ✅ **Embeddings** (learning representations)

**Together**, these form a complete graph ML platform. The algorithms aren't separate — they're **unified components of the ML workflow**.

**Next Steps**: Complete PageRank foundation, then prioritize the remaining three pillars to enable the full workflow.

