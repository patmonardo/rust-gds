# PageRank Master Compute: Infrastructure vs. Algorithm

**Date**: Current  
**Status**: Clarification - Distinguishing infrastructure from algorithm logic  
**Purpose**: Document the separation between `MasterComputeContext` (infrastructure) and `master_compute()` (algorithm)

---

## The Key Distinction

When implementing PageRank's master compute, there are **two separate concerns**:

1. **`MasterComputeContext`**: Infrastructure for accessing node values, iteration count, graph metadata
2. **`master_compute()` method**: The actual algorithm logic (convergence, normalization)

---

## 1. MasterComputeContext (Infrastructure Layer)

**Location**: `gds/src/pregel/context/master_compute_context.rs`

**Role**: Provides **access** to computation state, not the algorithm itself.

**What It Provides**:
- `double_node_value(node_id, key)` - Read node property values
- `set_double_node_value(node_id, key, value)` - Update node property values
- `for_each_node(consumer)` - Iterate over all nodes
- `superstep()`, `node_count()`, `config()` - Metadata access

**Status**: ✅ **Now Implemented** (was stubbed, now provides actual access to `NodeValue` storage)

**Key Insight**: This is just a **view/accessor** - it doesn't implement any algorithm. It's infrastructure.

---

## 2. master_compute() Method (Algorithm Layer)

**Location**: `gds/src/procedures/pagerank/pregel_computation.rs`

**Role**: Contains the **actual algorithm logic** for convergence checking and normalization.

**What It Does** (from Java `EigenvectorComputation.masterCompute()`):

```rust
fn master_compute(&mut self, context: &mut MasterComputeContext<Self::Config>) -> bool {
    // 1. Read NEXT_RANK values computed in compute() phase
    // 2. Normalize using L2-Norm (Power Iteration):
    //    - Compute ||next_rank||₂ (L2 norm)
    //    - Scale each value: normalized = next_rank / ||next_rank||₂
    // 3. Check convergence: |normalized_next - normalized_curr| > tolerance
    // 4. Atomically copy NEXT_RANK → RANK for all nodes
    // 5. Return true if converged (early termination)
}
```

**Status**: ⚠️ **Stubbed** - Needs full implementation

**Algorithm Steps** (from Java):

1. **Create DoubleNodePropertyValues** for NEXT_RANK:
   ```java
   var properties = new DoubleNodePropertyValues() {
       public double doubleValue(long nodeId) {
           return context.doubleNodeValue(nodeId, NEXT_RANK);
       }
   };
   ```

2. **Normalize using L2-Norm Scaler**:
   ```java
   var scaler = ScalerFactory.parse(L2Norm.TYPE).create(...);
   var normalizedNextRank = scaler.scaleProperty(nodeId);
   ```

3. **Check Convergence** (parallel):
   ```java
   var didConverge = new MutableBoolean(true);
   PartitionUtils.rangePartition(concurrency, nodeCount,
       partition -> () -> partition.consume(nodeId -> {
           var normalizedNext = scaler.scaleProperty(nodeId);
           var normalizedCurr = context.doubleNodeValue(nodeId, RANK);
           if (Math.abs(normalizedNext - normalizedCurr) > tolerance) {
               didConverge.setFalse();
           }
           context.setNodeValue(nodeId, RANK, normalizedNext);
       })
   ).run();
   ```

4. **Return Convergence Status**:
   ```java
   return !context.isInitialSuperstep() && didConverge.booleanValue();
   ```

---

## Implementation Status

### ✅ Infrastructure Complete

**MasterComputeContext methods**:
- `double_node_value()` - ✅ Implemented (reads from NodeValue storage)
- `set_double_node_value()` - ✅ Implemented (writes to NodeValue storage)
- `for_each_node()` - ✅ Implemented (iterates 0..node_count)
- Metadata access - ✅ Complete

### ⚠️ Algorithm Logic Stubbed

**master_compute() method**:
- Convergence checking - ❌ Not implemented
- L2-Norm normalization - ❌ Not implemented (needs Scaler module)
- Two-value schema (RANK + NEXT_RANK) - ❌ Not implemented
- Parallel processing - ❌ Not implemented (needs PartitionUtils integration)

---

## What This Means

**The distinction is crucial**:

- **Context = Infrastructure**: "Give me node 5's rank value" → `context.double_node_value(5, "rank")`
- **master_compute() = Algorithm**: "Check if all nodes converged" → Implement convergence logic

**Current State**:
- We have the **tools** (MasterComputeContext) ✅
- We need the **algorithm** (master_compute implementation) ⚠️

**Next Steps**:
1. ✅ Complete `MasterComputeContext` infrastructure (DONE)
2. 🔄 Implement L2-Norm Scaler module
3. 🔄 Update schema to RANK + NEXT_RANK
4. 🔄 Implement convergence checking in `master_compute()`
5. 🔄 Add parallel processing support

---

## Java Source Pattern

**The Java code shows this separation clearly**:

```java
// Infrastructure (MasterComputeContext)
context.doubleNodeValue(nodeId, RANK);              // Read
context.setNodeValue(nodeId, RANK, value);          // Write
context.nodeCount();                                // Metadata

// Algorithm (masterCompute method)
public boolean masterCompute(MasterComputeContext<C> context) {
    // Algorithm logic here: normalization, convergence, updates
    // Uses context to read/write, but logic is in this method
}
```

**This is the pattern we're following in Rust**:
- `MasterComputeContext` = Infrastructure (now complete)
- `master_compute()` = Algorithm (needs implementation)

---

## Summary

**What Was Confusing**:
- MasterComputeContext had stubbed methods
- Thought master_compute() was missing

**The Reality**:
- MasterComputeContext is just **access infrastructure** (now complete)
- `master_compute()` method is where the **algorithm lives** (needs implementation)

**The Algorithm** (`master_compute()`) will use the infrastructure (`MasterComputeContext`) to:
1. Read NEXT_RANK values
2. Normalize them (L2-Norm)
3. Check convergence
4. Update RANK values atomically

**Status**: Infrastructure complete. Algorithm implementation is next step.

