# Catalog of Kantian Walks

**Date**: 2025-10-29  
**Question**: How many Kantian walks are there?

**Definition**: A **Kantian Walk** is a deterministic algorithm guided by pure reason/a priori maxims, preserving invariants. You can "set your clocks by it."

## Categorization

### ✅ Clear Kantian Walks (Pure Reason, Deterministic)

#### Centrality Algorithms (9)
1. **PageRank** - Deterministic Power Iteration, preserves probability
2. **ArticleRank** - Deterministic PageRank variant
3. **EigenVector** - Deterministic Power Iteration
4. **BetweennessCentrality** - Deterministic shortest-path based
5. **ClosenessCentrality** - Deterministic shortest-path based
6. **DegreeCentrality** - Deterministic degree counting
7. **HarmonicCentrality** - Deterministic shortest-path based
8. **HITS** - Deterministic iterative algorithm
9. **CELF** - Deterministic centrality-based

#### Pathfinding Algorithms (11)
10. **BFS** (Breadth-First Search) - Deterministic level-by-level
11. **DFS** (Depth-First Search) - Deterministic stack-based
12. **Dijkstra** - Deterministic shortest path (greedy)
13. **AStar** - Deterministic heuristic-based search
14. **BellmanFord** - Deterministic shortest path
15. **DeltaStepping** - Deterministic parallel shortest path
16. **ShortestPath** - Deterministic (general)
17. **AllPairsShortestPath** - Deterministic (all pairs)
18. **SingleSourceShortestPath** - Deterministic (single source)
19. **YenKShortestPath** - Deterministic K-shortest paths
20. **LongestPath** - Deterministic (DAG longest path)

#### Graph Structure Algorithms (8)
21. **TopologicalSort** - Deterministic dependency ordering
22. **SpanningTree** - Deterministic tree construction
23. **KSpanningTree** - Deterministic K-spanning trees
24. **SteinerTree** - Deterministic Steiner tree
25. **PCST** (Prize-Collecting Steiner Tree) - Deterministic
26. **WCC** (Weakly Connected Components) - Deterministic union-find
27. **SCC** (Strongly Connected Components) - Deterministic (Kosaraju/Tarjan)
28. **TriangleCount** - Deterministic triangle enumeration

#### Structural Analysis (5)
29. **ArticulationPoints** - Deterministic graph analysis
30. **Bridges** - Deterministic bridge detection
31. **KCore** - Deterministic core decomposition
32. **LocalClusteringCoefficient** - Deterministic clustering metric
33. **LCC** (Local Clustering Coefficient) - Deterministic

#### Similarity Algorithms (Pure Math) (3)
34. **CosineSimilarity** - Pure mathematical computation (deterministic)
35. **JaccardSimilarity** - Pure mathematical computation (deterministic)
36. **OverlapSimilarity** - Pure mathematical computation (deterministic)

#### Graph Transformations (4)
37. **ToUndirected** - Deterministic transformation
38. **CollapsePath** - Deterministic path collapsing
39. **IndexInverse** - Deterministic index inversion
40. **ScaleProperties** - Deterministic property scaling

**Total Clear Kantian Walks: ~40**

---

### ⚠️ Hybrid Algorithms (Mostly Kantian with Stochastic Elements)

#### Community Detection (Deterministic Core, Stochastic Optimization)
- **Louvain** - Deterministic optimization with stochastic initialization
- **Leiden** - Deterministic optimization with stochastic refinement
- **LabelPropagation** - Mostly deterministic, with tie-breaking randomness
- **Modularity** - Pure mathematical metric (Kantian)
- **ModularityOptimization** - Deterministic with stochastic starting points

#### Node Similarity (Deterministic Computation, Stochastic Sampling)
- **NodeSimilarity** - Deterministic similarity computation
- **FilteredNodeSimilarity** - Deterministic with filtering
- **KNN** - Deterministic K-nearest neighbors
- **FilteredKNN** - Deterministic with filtering

**These are "Kantian walks with empirical guidance"** — deterministic core with stochastic initialization/sampling.

---

### ❌ Random Walks (Stochastic, Empirical)

#### Embedding Algorithms (Stochastic)
- **Node2Vec** - Random walks + SGD (empirical)
- **FastRP** - Random projection (stochastic)
- **GraphSAGE** - Random neighbor sampling (stochastic)
- **HashGNN** - Random hashing (stochastic)
- **KGE** (Knowledge Graph Embedding) - SGD-based (empirical)

#### Pure Stochastic
- **RandomWalk** - Pure stochastic exploration

#### ML Algorithms (Stochastic)
- **RandomForest** - Random sampling
- **GradientBoosting** - SGD-based (empirical)
- **KMeans** - Stochastic initialization
- **DBSCAN** - Deterministic but uses density-based clustering

#### Miscellaneous
- **Conductance** - Pure mathematical metric (Kantian, but measures stochastic processes)
- **BetaClosenessCentrality** - Deterministic variant
- **SLLPA** - Stochastic label propagation
- **ApproximateMaximumKCut** - Approximation algorithm (hybrid)

---

## Summary

### Pure Kantian Walks: **~40 algorithms**

**By Category**:
- **Centrality**: 9
- **Pathfinding**: 11
- **Graph Structure**: 8
- **Structural Analysis**: 5
- **Similarity (Math)**: 3
- **Transformations**: 4

### Hybrid (Mostly Kantian): **~9 algorithms**
- Community detection (4)
- Node similarity (4)
- Modularity (1)

### Random Walks (Empirical): **~9 algorithms**
- Embeddings (5)
- Pure stochastic (1)
- ML algorithms (3+)

---

## Implementation Status

**Kantian Walks Implemented**:
- ✅ **PageRank** (partial - needs master_compute)
- ✅ **WCC** (Weakly Connected Components - likely implemented)
- ✅ **SpanningTree** (likely exists based on file structure)
- ❌ Most others: Not yet translated

**Total Platform Goal**: ~40 pure Kantian walks + ~9 hybrid = **~49 algorithms that use pure reason** (with or without stochastic initialization)

---

## Key Insight

**Kantian walks dominate the platform** — most graph algorithms are deterministic, preserving invariants. The stochastic/empirical algorithms are primarily:
1. Embedding algorithms (Node2Vec, GraphSAGE)
2. ML training (SGD-based)
3. Some community detection (stochastic optimization)

**The platform is primarily built on pure reason** (Kantian walks), with empirical methods (random walks, SGD) used for learning representations and some optimization tasks.

