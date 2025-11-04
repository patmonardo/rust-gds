# PageRank Codegen Architecture Analysis

**Date**: Current  
**Status**: Documentation of Translation Pattern  
**Purpose**: Understand the codegen architecture for PageRank translation from Java GDS

---

## Java GDS Source Location

**Authoritative Reference**: `/home/pat/GitHub/graph-data-science`

This is the original Neo4j Graph Data Science Java source code. All Rust implementations are translated from this source for 1:1 accuracy.

**Reference**: `.github/copilot-instructions.md` line 10

---

## PageRank Dual Computation Architecture

PageRank has **two computation paths**, both following the **Real:Ideal Type:Value** Functor pattern:

### 1. Storage Runtime (`storage.rs`) - **Real Pole**

**Role**: GraphStore → f64 mapping (Real Type:Value system)

```rust
// storage.rs explicitly knows this mapping:
/// This is where the Functor machinery works:
/// GraphStore (Real) → f64 (Ideal)
pub fn get_node_degree(&self, node_id: u64) -> Result<f64, AlgorithmError>
```

**Status**: ⚠️ **Stubbed** - Has TODOs but architecture is clear:
- Knows how to map from GraphStore (Real/persistent) to computation values (Ideal/ephemeral)
- Implements the **Real → Ideal** quadrant of the Four-fold Functor system
- Represents "Genesis" (storage-bound beginning) → **Result<>** (computation value)

### 2. Computation Runtime (`computation.rs`) - **Ideal Pole**

**Role**: Ephemeral PageRank scores and message accumulation (Ideal Type:Value system)

```rust
// computation.rs manages ephemeral computation state:
pub struct PageRankComputationRuntime {
    scores: HashMap<u64, f64>,          // Ideal: free computation values
    previous_scores: HashMap<u64, f64>,
    damping_factor: f64,
    // ...
}
```

**Status**: ✅ **Complete** - Full implementation

**Translation Pattern**:
- **Java GDS**: `org.neo4j.gds.algorithms.centrality.PageRankComputation`
- **Rust**: `PageRankComputationRuntime` in `computation.rs`
- Implements the **Ideal** quadrant (ephemeral computation)

### 3. Pregel Computation (`pregel_computation.rs`) - **Pregel Integration**

**Role**: PageRank via Pregel message-passing framework

```rust
// pregel_computation.rs implements PregelComputation trait:
impl PregelComputation for PageRankPregelComputation {
    fn compute<I: MessageIterator>(...)
    // Uses Power Iteration:
    // PR(v) = (1-d) + d × Σ(PR(u) / out_degree(u))
}
```

**Status**: ✅ **Complete** - Full Pregel-based implementation

**Translation Pattern**:
- **Java GDS**: `org.neo4j.gds.pregel.PageRankComputation` (Pregel-based)
- **Rust**: `PageRankPregelComputation` in `pregel_computation.rs`
- Implements **complete Power Iteration** algorithm

---

## The Codegen Pattern (Four-fold Functor System)

### Storage → Computation (Real → Ideal)

**Genesis**: `storage.rs::get_node_degree()` - GraphStore (Real)  
**Result<>**: `computation.rs::get_score()` - f64 (Ideal)

**The Functor**: Mapping from persistent PropertyValues to ephemeral computation values

### Current TODO Integration Point

**File**: `gds/src/procedures/pagerank/spec.rs` line 318

```rust
// TODO: Implement actual PageRank message passing
// For now, this is a placeholder that simulates the algorithm
// In a real implementation, this would:
// 1. For each node, compute outgoing messages
// 2. Distribute messages to neighbors
// 3. Accumulate incoming messages
// 4. Update scores
```

**Solution**: Integrate `PageRankPregelComputation` from `pregel_computation.rs` into the main `spec.rs` execution loop.

**The Integration**:
- `spec.rs` currently uses `PageRankComputationRuntime` (computation.rs)
- `PageRankPregelComputation` (pregel_computation.rs) already implements full message passing
- **Next Step**: Wire `PageRankPregelComputation` into `spec.rs::execute()` method

---

## The Four-fold Turning in PageRank

Each **EvalStep** represents one iteration of PageRank:

1. **Real → Ideal**: `storage.rs` reads GraphStore → produces f64 values
2. **Ideal → Real**: `computation.rs` accumulates → updates PropertyValues (if persisted)
3. **Type Validation**: Schema validation of PropertyValues
4. **Value Transformation**: Message value transformations (weighting, normalization)

**Genesis → Result<>**: Each quadrant completes its telos (from beginning to completion)

---

## Translation References

### Java GDS PageRank Sources

**Base Package**: `org.neo4j.gds.algorithms.centrality`

1. **PageRankComputation.java**
   - → `computation.rs` (Power Iteration standalone)

2. **PageRankConfig.java**
   - → `spec.rs::PageRankConfig` (configuration)

3. **PageRankResult.java**
   - → `spec.rs::PageRankComputationResult` (result type)

**Pregel Package**: `org.neo4j.gds.pregel`

4. **PageRankComputation.java** (Pregel-based)
   - → `pregel_computation.rs` (Pregel integration)

**Storage Package**: `org.neo4j.gds.api.GraphStore`

5. **GraphStore.java**
   - → `storage.rs::PageRankStorageRuntime` (GraphStore access)

---

## Codegen Architecture Summary

**The Pattern**:
1. **Read Java GDS source** from `/home/pat/GitHub/graph-data-science`
2. **Translate 1:1** to Rust modules
3. **Maintain Functor architecture**: Real:Ideal Type:Value mapping
4. **Codegen generates** the Four-fold PropertyFunctor system automatically
5. **EvalStep orchestrates** all four quadrants (Four-fold Turning of the Wheel)

**The Insight**: 
- `storage.rs` is **stubbed** but **architecturally complete**
- It knows its role: **Store → f64** (Real → Ideal)
- Codegen will fill in the TODOs following the established Functor pattern
- The `pregel_computation.rs` shows the **complete pattern** already working

---

## Next Steps

1. ✅ **Document architecture** (this file)
2. 🔄 **Integrate Pregel computation** into `spec.rs` (replace TODO at line 318)
3. 🔄 **Complete storage.rs TODOs** following the Java GDS source pattern
4. ✅ **Verify codegen** generates correct PropertyFunctors for Real↔Ideal mapping

**Status**: Architecture documented. Ready for implementation work.

