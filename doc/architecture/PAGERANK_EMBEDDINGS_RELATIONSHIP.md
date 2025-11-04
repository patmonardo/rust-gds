# PageRank and Embeddings/Similarity: Relationship Analysis

**Date**: 2025-10-29  
**Question**: Does PageRank presuppose Embeddings and Similarity?

## Answer: No - PageRank is Independent

**PageRank does NOT depend on Embeddings or Similarity algorithms.**

However, they can be **used together** in different ways.

## Dependency Direction

**PageRank → Embeddings/Similarity** (PageRank can be input)
- **NOT** Embeddings/Similarity → PageRank (PageRank doesn't need them)

### PageRank as Input to Embeddings

PageRank scores can be used **as features** for embedding algorithms:

1. **Feature Engineering**:
   - Compute PageRank scores first
   - Use PageRank as a **node feature** in embedding algorithms
   - Embeddings learn from PageRank + graph structure

2. **Multi-Feature Embeddings**:
   - Combine PageRank with other centrality measures
   - Use as input features to GraphSAGE, FastRP, etc.
   - Enriches embeddings with centrality information

3. **Hybrid Approaches**:
   - Some embedding methods use PageRank-weighted sampling
   - PageRank guides which nodes to emphasize in embeddings

### PageRank and Similarity

PageRank scores can be used **directly in similarity computations**:

1. **Score-Based Similarity**:
   - Compare nodes by PageRank scores
   - Nodes with similar PageRank = similar importance
   - Can compute similarity without embeddings

2. **Embedding-Based Similarity**:
   - Compute embeddings (which may use PageRank as feature)
   - Then compute similarity between embeddings
   - PageRank contributes to embedding quality

## Algorithm Independence

**PageRank is self-contained**:
- Only needs: Graph structure (nodes, edges)
- Computes: Centrality scores (scalars per node)
- No dependency on: Embeddings, Similarity, or any other algorithm

**PageRank Implementation** (`gds/src/procedures/pagerank/`):
- ✅ Independent Pregel computation
- ✅ No embedding dependencies
- ✅ No similarity dependencies
- ✅ Pure graph algorithm

## Why This Matters

**You can implement PageRank first** without needing:
- ❌ Embeddings (Node2Vec, FastRP, GraphSAGE)
- ❌ Similarity algorithms (NodeSimilarity, CosineSimilarity)

**But you'll want embeddings/similarity soon** because:
- ✅ PageRank scores are useful features for ML
- ✅ Similarity is a common downstream task
- ✅ Embeddings enable many ML workflows

## Translation Priority

**Current Status**:
- ✅ **PageRank**: Partially implemented (Pregel framework, needs master_compute)
- ❌ **Embeddings**: Infrastructure exists (RandomWalkSampler), but algorithms not translated
  - Node2Vec: Structure exists, compute() is TODO
  - FastRP: Structure exists, compute() is TODO
  - GraphSAGE: Structure exists, compute() is TODO
- ❌ **Similarity**: Algorithms listed in AlgorithmLabel, but not translated

**Recommendation**:
1. **Complete PageRank first** (master_compute, normalization)
2. **Then translate Embeddings** (Node2Vec, FastRP) - PageRank can be a feature
3. **Then translate Similarity** (can use PageRank scores or embeddings)

**PageRank doesn't presuppose them, but they often use PageRank as input.**

## Architecture Notes

**PageRank Output**:
- Scalar scores per node (`f64`)
- Can be stored as node properties
- Can be used as features for embeddings

**Embeddings Output**:
- Vector embeddings per node (`DoubleArray`)
- Can incorporate PageRank as one dimension
- Can be compared using similarity

**Similarity Output**:
- Similarity scores between nodes
- Can compare PageRank scores directly
- Can compare embedding vectors

**Conclusion**: PageRank is independent and can be completed without embeddings/similarity. However, embeddings and similarity often benefit from using PageRank as input features.

