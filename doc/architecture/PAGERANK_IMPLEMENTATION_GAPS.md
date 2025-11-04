# PageRank Implementation Gaps

**Date**: Current  
**Status**: Discovery - Gaps identified from Java GDS source analysis  
**Purpose**: Document missing pieces for complete PageRank implementation

---

## Overview

Studying the Java GDS PageRank source (`EigenvectorComputation.java`) reveals several architectural components missing from our Rust implementation. These are foundational for ML graph algorithms.

---

## Missing Components

### 1. Master Compute Method (`master_compute()`)

**Java Source**: `org.neo4j.gds.pagerank.EigenvectorComputation.masterCompute()`

**What It Does**:
- Runs **once per superstep** (not per node)
- Performs **global convergence checking** across all nodes
- Handles **L2-Norm normalization** (Power Iteration)
- Updates node values atomically after normalization

**Java Pattern**:
```java
@Override
public boolean masterCompute(MasterComputeContext<C> context) {
    // 1. Create DoubleNodePropertyValues for NEXT_RANK
    // 2. Normalize using L2-Norm scaler
    // 3. Check convergence: |normalizedNextRank - normalizedCurrRank| > tolerance
    // 4. Update all RANK values atomically
    // 5. Return true if converged (early termination)
    return didConverge;
}
```

**Rust Status**:
- ✅ `MasterComputeContext` exists but is **stubbed** (`gds/src/pregel/context/master_compute_context.rs`)
- ✅ `PregelComputation` trait has `master_compute()` method (default impl returns false)
- ❌ `PageRankPregelComputation` **doesn't implement `master_compute()`**
- ❌ Master compute context's `for_each_node()`, `node_value()` are stubs

**Why It Matters**:
- **Power Iteration** requires normalization after each step
- Global convergence checking is more efficient than per-node voting
- Atomic updates prevent race conditions in parallel execution

---

### 2. Weight Denominator Function

**Java Source**: `EigenvectorComputation` constructor accepts `LongToDoubleFunction weightDenominator`

**What It Does**:
```java
// For unweighted graphs: returns 1.0
// For weighted graphs: returns sum of relationship weights
context.sendToNeighbors(nextRank / weightDenominator.applyAsDouble(context.nodeId()));
```

**Java Pattern**:
- Unweighted: `weightDenominator = nodeId -> 1.0`
- Weighted: `weightDenominator = nodeId -> sum(relationship_weights(nodeId))`
- Used to normalize message values before sending

**Rust Status**:
- ✅ `context.degree()` exists but returns integer degree
- ❌ No `weightDenominator` function abstraction
- ❌ Weighted graph normalization not handled

**Why It Matters**:
- PageRank on weighted graphs requires proper weight normalization
- Degree vs. weighted degree confusion can cause incorrect scores

---

### 3. Two-Value Schema (RANK + NEXT_RANK)

**Java Source**: `EigenvectorComputation.schema()`

**What It Does**:
```java
return new PregelSchema.Builder()
    .add(RANK, ValueType.DOUBLE)        // Current rank
    .add(NEXT_RANK, ValueType.DOUBLE)    // Computed rank (before normalization)
    .build();
```

**Purpose**:
- **RANK**: Current normalized PageRank value (read by users)
- **NEXT_RANK**: Computed value before normalization (internal state)
- Master compute normalizes NEXT_RANK → RANK atomically

**Rust Status**:
- ✅ `PregelSchema` supports multiple properties
- ❌ `PageRankPregelComputation` only uses single "pagerank" property
- ❌ No separation between current and next rank

**Why It Matters**:
- Enables atomic updates (no partial state visible)
- Supports Power Iteration normalization pattern

---

### 4. L2-Norm Normalization (Scaler)

**Java Source**: `EigenvectorComputation.masterCompute()` uses `ScalerFactory.parse(L2Norm.TYPE)`

**What It Does**:
```java
// Normalize using L2-Norm (Power iteration)
var scaler = ScalerFactory.parse(L2Norm.TYPE).create(
    properties,
    context.nodeCount(),
    concurrency,
    ProgressTracker.NULL_TRACKER,
    context.executorService()
);

var normalizedNextRank = scaler.scaleProperty(nodeId);
```

**Purpose**:
- Normalizes vector to unit length: `||v||₂ = 1`
- Required for Power Iteration to converge to dominant eigenvector
- Different from standard PageRank (which uses damping factor)

**Rust Status**:
- ❌ No `Scaler` or `L2Norm` implementation
- ❌ No normalization in master compute

**Why It Matters**:
- **Eigenvector Centrality** requires L2-Norm normalization
- **Power Iteration** is fundamental to spectral graph theory
- Foundation for many ML graph algorithms

---

### 5. Parallel Partition Processing

**Java Source**: `MasterComputeContext` uses `PartitionUtils.rangePartition()`

**What It Does**:
```java
var tasks = PartitionUtils.rangePartition(concurrency, context.nodeCount(),
    partition -> (Runnable) () -> partition.consume(nodeId -> {
        // Process nodeId in parallel
    })
);

RunWithConcurrency.builder()
    .concurrency(concurrency)
    .tasks(tasks)
    .executor(context.executorService())
    .run();
```

**Purpose**:
- Parallel processing of all nodes in master compute
- Efficient convergence checking across partitions

**Rust Status**:
- ✅ `Partitioning` exists in `core::utils::partition`
- ❌ `MasterComputeContext.for_each_node()` is stubbed
- ❌ No parallel processing pattern in master compute

---

## Why This Is Foundational ML

**PageRank and Power Iteration** are core to:

1. **Spectral Graph Theory**: Eigenvectors of adjacency matrix
2. **Network Analysis**: Node centrality, influence propagation
3. **Graph Neural Networks**: Message passing foundations
4. **Community Detection**: Modularity, clustering
5. **Recommendation Systems**: Personalized PageRank variants

**Learning Path**:
- Standard PageRank (damping factor) → Matrix form
- Power Iteration → Eigenvector centrality
- Master compute pattern → Distributed algorithms
- Normalization → Numerical stability

---

## Implementation Roadmap

### Phase 1: Master Compute Infrastructure

1. **Implement `MasterComputeContext` methods**:
   - `node_value()` - Read node values
   - `set_node_value()` - Update node values
   - `for_each_node()` - Parallel iteration

2. **Add `master_compute()` to `PageRankPregelComputation`**:
   - Implement convergence checking
   - Add two-value schema (RANK + NEXT_RANK)

### Phase 2: Normalization

3. **Implement L2-Norm Scaler**:
   - Create `scaler` module in `gds/src/ml/scaling/`
   - Implement `L2Norm::scale_property()`
   - Integrate with master compute

### Phase 3: Weight Normalization

4. **Add `weightDenominator` abstraction**:
   - Trait: `WeightDenominator: Fn(u64) -> f64`
   - Unweighted: constant function returning 1.0
   - Weighted: sum of relationship weights

### Phase 4: Integration

5. **Wire master compute into `spec.rs`**:
   - Replace TODO at line 318
   - Use `PageRankPregelComputation` with master compute
   - Test convergence and normalization

---

## Codegen Insight

**The Pattern**:
1. Java GDS defines complete architecture (master compute, normalization, weights)
2. Rust implementation has **stubs** that know their role but need implementation
3. **Codegen will fill in** the TODOs following the Java GDS pattern

**The Architecture**:
- Master compute = **Global coordination** (Middle of Middles)
- Normalization = **Type:Value transformation** (Ideal → Ideal)
- Weight denominator = **Store → Computation mapping** (Real → Ideal)

**Status**: Gaps documented. Ready for implementation work.

