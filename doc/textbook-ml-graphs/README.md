# ML Using Graphs by Example: A Rust-GDS Textbook

**Progressive, difficulty-ordered learning path for modern graph data science**

## Course Philosophy

This textbook begins where a modern **ML with Graphs** course begins: **PageRank**.
The first 10 lectures of such courses establish PageRank as the foundation of graph centrality.

### Where This Course Fits

**Prerequisites** (covered elsewhere):
- Basic algorithm structures (trees, stacks, queues)  
- Graph traversal (BFS, DFS, topological sort)  
- Pathfinding (Dijkstra, A*, Bellman-Ford)

**This Course** (starts here):
- PageRank using **Power Iteration** ← **Lecture 1**
- Graph-level features and centrality measures
- Graph neural networks and embeddings
- Real-world ML applications

## Learning Path (Complete Course)

### Part I: Foundations (Chapters 01-03)
**Substrate for graph ML**

#### 01 — Foundations (Collections)
Learn the substrate: Collections across Vec, Huge, and Arrow-ready interfaces.
- Portable APIs across storage backends
- Memory management and estimation
- Concurrent data structures

#### 02 — Structures (GraphStore)
Configure and assemble the modern GraphStore with adaptive backends.
- Configuration-driven architecture
- Property system (graph/node/relationship triad)
- Adaptive backend selection (Vec → Huge → Arrow)

#### 03 — Graph API (Projection & Traversal)
Projection model and traversal patterns.
- Node and relationship projections
- Cursor-based traversal
- Filtered views and property access

### Part II: Algorithms & Computation (Chapters 04-06)
**Core ML algorithms and data structures**

#### 04 — Computation (Pregel + PageRank) ← **Course Begins Here**
Algorithm structure and property store integration.
- **PageRank via Power Iteration** ← Lecture 1
- Pregel message-passing framework
- Property-backed computations

#### 05 — Pathfinding & Graph Algorithms
Foundation algorithms for graph ML.
- Dijkstra, A*, Bellman-Ford (pathfinding)
- Graph centrality measures
- Community detection algorithms
- Building blocks for ML features

#### 06 — Tensors (ML Core Structures)
Tensor operations on graph features.
- Scalar → Vector → Matrix → Tensor (data structure hierarchy)
- ML Core tensor representations
- Feature extraction from graph algorithms
- Algorithm results → Tensor operations

### Part III: ML & Scale (Chapters 07-08)
**End-to-end ML and production**

#### 07 — ML Functions (End-to-End Algorithms)
Integrated ML algorithms in Rust.
- ML Functions in ML Core
- Complete ML pipelines: Subgraph, embeddings
- Integration with external ML libraries
- Real-world applications

#### 08 — Scale (Production Patterns)
Large-scale graph operations and persistence.
- Billion-scale graphs (Huge backend)
- Partitioning strategies
- Arrow persistence and zero-copy ML pipelines

## Maturity Badges

- ✅ **Complete** - Production-ready, fully working examples
- 🚧 **In Progress** - Works but API stabilizing
- 📋 **Planned** - Coming soon, designed but not implemented

## Code Examples

**Staging Area**: `archive/examples/` (31+ examples organized by chapter)  
**Curated Workspace**: `gds/examples/` (you manually select what to study)  
**Teaching Material**: This doc folder (structure and learning paths)

## PageRank: Course Lecture 1

Our PageRank implementation uses **Power Iteration**, the standard iterative algorithm:

```
PR(v) = (1-d) + d × Σ(PR(u) / out_degree(u))
```

Where:
- `PR(v)` = PageRank score of node v
- `d` = damping factor (typically 0.85)
- Iterates until convergence (L1 norm < tolerance)

**Implementation**: `gds/src/procedures/pagerank/pregel_computation.rs`

## Architectural Terminology

This textbook uses **Real:Ideal Type:Value Systems** terminology (Kant-Fichte-Hegel language) to describe the Property architecture:

- **Real Type:Value System** = Store-bound, Schema-bound (PropertyValues)
- **Ideal Type:Value System** = Free Values, Schema-less (PrimitiveValues/FeatureSpace)
- **Property** = Middle that unites both systems (UniversalAdapter, Middle of Middles)

See **TERMINOLOGY.md** for complete architectural terminology guide.

---

**Next**: Start with 01-foundations/README.md

