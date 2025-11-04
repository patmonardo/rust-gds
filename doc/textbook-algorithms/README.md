# Algorithms in Rust: Graph ML Focus

**A practical course integrating algorithm design with Rust patterns and ML applications**

## Philosophy

This isn't a standard algorithms course. Instead:
- **ML-oriented**: Algorithms that matter for graph ML (PageRank, centrality, embeddings)
- **Rust-focused**: Explore Arc, Ref, Trait Objects where they actually matter
- **Practical**: Algorithms that work with your Collections, GraphStore, and ML Core

## Where This Fits

**Prerequisites**: Basic Rust (ownership, references)  
**Follows**: Standard algorithms course material (trees, BFS, DFS) but ML-focused  
**Leads to**: "ML Using Graphs" (uses these algorithms)

## Learning Path

### 01 — PageRank
**Lecture 1**: Power Iteration algorithm  
- Mathematical foundation: PR(v) = (1-d) + d × Σ(PR(u) / out_degree(u))
- Rust implementation: Pregel-based message passing
- Convergence detection and damping factor

**Key Code**: `gds/src/procedures/pagerank/pregel_computation.rs`

### 02 — Pregel Data Structures
**Lectures 2-5**: Core Pregel patterns (The Professor's next focus)
- Message aggregation (SumReducer, MaxReducer)
- State management across supersteps
- Cursor-based graph access
- Arc/Ref patterns for shared state

**Key Concepts**:
- MessageIterator and message reduction
- ComputeContext, InitContext, MasterComputeContext
- The Rust learnings (Arc patterns that cost you debugging cycles!)

### 03 — Message Passing Patterns
Advanced Pregel patterns for specific algorithms
- Connected Components (label propagation)
- Single-Source Shortest Path (Dijkstra-like)
- Triangle Counting
- Personalized PageRank

### 04 — Graph Algorithms with ML Core
Integration with ML Core and Tensor operations
- How algorithms expose results as tensors
- Graph embeddings and representations
- Algorithm results → ML features

### 05 — Rust Patterns in Practice
Hard-won lessons from the codebase
- Arc<Trait> vs Arc<dyn Trait> (reference vs trait objects)
- RefCell patterns for interior mutability
- Type erasure vs monomorphization tradeoffs
- Where Rust's ownership model really bites

## Maturity

- ✅ PageRank: Complete (production-ready)
- 🚧 Pregel Structures: In progress (you're learning the patterns now)
- 📋 Graph Algorithms: Coming as you work through Stanford/MIT material
- 📋 Rust Patterns: Document lessons from your debugging cycles

## Code Examples

**Reference**: `gds/src/procedures/`  
**Implementation**: Pure Rust algorithms, no Python bridges

---

**Next**: Start with 01-pagerank/README.md

