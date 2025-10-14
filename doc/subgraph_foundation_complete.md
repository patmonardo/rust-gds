# Subgraph Components Translation - Complete

**Date**: October 14, 2025  
**Status**: Foundation Complete ✅  
**Tests**: 223 ML tests passing (19 new subgraph tests)  
**Components**: LocalIdMap, BatchNeighbors, SubGraph, NeighborhoodSampler (stub)

## Translation Order & Rationale

Based on dependency analysis of Java GDS ml-core subgraph package:

1. **LocalIdMap** (foundation - no dependencies)
2. **BatchNeighbors** (trait/interface - no dependencies)
3. **SubGraph** (implements BatchNeighbors, uses LocalIdMap internally)
4. **NeighborhoodSampler** (stub - requires Graph API integration)

This order ensures each component builds on stable foundations.

## Component Translations

### 1. LocalIdMap - Bidirectional ID Mapping

**Purpose**: Maps between original graph node IDs (u64) and local consecutive IDs (usize).

**Java Implementation**:

- Uses HPPC's `LongArrayList` + `LongIntHashMap` for memory efficiency
- Provides `toMapped(long)` and `toOriginal(int)` methods
- Used internally by SubGraph for efficient array indexing

**Rust Translation**:

```rust
pub struct LocalIdMap {
    original_ids: Vec<u64>,              // Local → Original
    original_to_internal: HashMap<u64, usize>,  // Original → Local
}
```

**Key Methods**:

- `to_mapped(&mut self, original_id: u64) -> usize` - Get/assign local ID
- `to_original(&self, local_id: usize) -> u64` - Lookup original ID
- `of(original_ids: &[u64]) -> Self` - Factory from slice
- `of_sorted(original_ids: &[u64]) -> Self` - Factory with deterministic ordering

**Design Decision**: Use `HashMap<u64, usize>` instead of a Rust HPPC equivalent for:

1. **Clarity** - Standard library type, well-understood
2. **Correctness** - Proven correct behavior
3. **Maintainability** - No external dependency

Performance optimization can come later if profiling shows it's needed.

**Tests**: 12 tests covering creation, bidirectional mapping, equality, edge cases

### 2. BatchNeighbors - Trait for Subgraph Access

**Purpose**: Interface for accessing neighborhood information in batch processing.

**Java Implementation**:

```java
public interface BatchNeighbors {
    int[] batchIds();
    default int batchSize() { return batchIds().length; }
    int nodeCount();
    int degree(int batchId);
    int[] neighbors(int batchId);
    double relationshipWeight(int src, int trg);
}
```

**Rust Translation**:

```rust
pub trait BatchNeighbors {
    fn batch_ids(&self) -> &[usize];
    fn batch_size(&self) -> usize { self.batch_ids().len() }  // Default impl
    fn node_count(&self) -> usize;
    fn degree(&self, batch_id: usize) -> usize;
    fn neighbors(&self, batch_id: usize) -> &[usize];
    fn relationship_weight(&self, src: usize, trg: usize) -> f64;
}
```

**Key Differences**:

- **Lifetimes**: Return `&[usize]` instead of `int[]` (borrows, no copy)
- **Default impl**: `batch_size()` has default implementation
- **Type safety**: `usize` for array indices (never negative)

**Tests**: 4 tests using mock implementation to verify trait contract

### 3. SubGraph - Main Implementation

**Purpose**: Represents a sampled neighborhood subgraph for GNN batch processing.

**Java Implementation**:

```java
public final class SubGraph implements BatchNeighbors {
    private final int[] mappedBatchNodeIds;
    private final long[] originalNodeIds;
    final int[][] neighbors;
    private final RelationshipWeights relationshipWeightsFunction;

    // Complex buildSubGraph() static method with LocalIdMap usage
}
```

**Rust Translation (Current - Foundation)**:

```rust
pub struct SubGraph {
    mapped_batch_node_ids: Vec<usize>,
    original_node_ids: Vec<u64>,
    neighbors: Vec<Vec<usize>>,
    weighted: bool,
}

impl BatchNeighbors for SubGraph {
    fn batch_ids(&self) -> &[usize] { &self.mapped_batch_node_ids }
    fn node_count(&self) -> usize { self.original_node_ids.len() }
    fn degree(&self, batch_id: usize) -> usize { self.neighbors[batch_id].len() }
    fn neighbors(&self, batch_id: usize) -> &[usize] { &self.neighbors[batch_id] }
    fn relationship_weight(&self, _src: usize, _trg: usize) -> f64 { 1.0 }  // TODO
}
```

**Not Yet Translated**:

- `buildSubGraph()` static method - requires NeighborhoodFunction integration
- `buildSubGraphs()` - requires list of NeighborhoodFunctions
- `relationshipWeightFunction()` - requires Graph API
- Actual weight lookup in `relationship_weight()`

**Rationale**: Foundation first. The struct and trait implementation are complete and testable. The complex builder methods require Graph API integration which is a separate concern.

**Tests**: 3 tests for basic SubGraph creation and BatchNeighbors implementation

### 4. NeighborhoodSampler - Stub for Graph API Integration

**Purpose**: Samples neighborhoods for graph neural network batch processing.

**Java Implementation**:

```java
public class NeighborhoodSampler {
    private final long randomSeed;

    public LongStream sample(Graph graph, long nodeId, int numberOfSamples) {
        // Uses Graph API, UniformSampler, WeightedUniformSampler
    }
}
```

**Rust Translation (Stub)**:

```rust
pub struct NeighborhoodSampler {
    random_seed: u64,
}

impl NeighborhoodSampler {
    pub fn new(random_seed: u64) -> Self {
        Self { random_seed }
    }
}
```

**Why Stub?**

- Requires Graph API from `rust-gds` core (not in ml-core)
- Requires integration with our existing samplers (UniformSampler, WeightedUniformSampler)
- Requires `RelationshipCursor` and graph streaming APIs
- Should be implemented when Graph API integration happens

**Tests**: None yet (stub only)

## Integration with Existing Code

### MultiMean Function - Updated

**Before**:

```rust
pub struct BatchNeighbors { /* placeholder */ }

pub struct MultiMean {
    base: VariableBase,
    sub_graph: BatchNeighbors,  // Concrete type
}
```

**After**:

```rust
use crate::ml::core::subgraph::BatchNeighbors;

pub struct MultiMean {
    base: VariableBase,
    sub_graph: Box<dyn BatchNeighbors>,  // Trait object
}
```

**Benefit**: Can now accept any BatchNeighbors implementation (SubGraph, mock, etc.)

### ElementWiseMax Function - Updated

Same pattern as MultiMean - replaced placeholder with real trait object.

## Patterns Demonstrated

### 1. Foundation-First Translation

**Approach**: Translate data structures before algorithms.

**Rationale**:

- LocalIdMap is pure data structure (no Graph API dependency)
- BatchNeighbors defines the interface (contract-first)
- SubGraph implements the interface with testable foundation
- Complex builder methods can be added incrementally

**Benefit**: Build confidence with passing tests before tackling complex integrations.

### 2. Trait Objects for Polymorphism

**Java Pattern**:

```java
public interface BatchNeighbors { ... }
public class SubGraph implements BatchNeighbors { ... }

void process(BatchNeighbors neighbors) { ... }  // Accepts any implementation
```

**Rust Pattern**:

```rust
pub trait BatchNeighbors { ... }
pub struct SubGraph { ... }
impl BatchNeighbors for SubGraph { ... }

fn process(neighbors: &dyn BatchNeighbors) { ... }  // Trait object
// Or:
fn process(neighbors: Box<dyn BatchNeighbors>) { ... }  // Owned trait object
```

**When to Use**:

- **Trait object (`&dyn T`, `Box<dyn T>`)**: Runtime polymorphism, like Java interfaces
- **Generic (`<T: Trait>`)**: Compile-time polymorphism, monomorphization

For ML-Core, trait objects match Java's design - functions accept any BatchNeighbors implementation.

### 3. HashMap vs Specialized Collections

**Java's HPPC** (High Performance Primitive Collections):

- `LongIntHashMap` - specialized for long→int mapping
- `LongArrayList` - specialized for long arrays
- **Why**: Avoids boxing overhead in Java

**Rust's Standard Library**:

- `HashMap<u64, usize>` - already primitive-optimized
- `Vec<u64>` - already primitive-optimized
- **Why**: No boxing in Rust, standard library is already fast

**Decision**: Use standard library first, profile later. Premature optimization is the root of all evil.

## Test Coverage Summary

**Local ID Mapping (12 tests)**:

- ✅ Basic creation and size
- ✅ Bidirectional mapping (original ↔ local)
- ✅ Consecutive ID assignment
- ✅ Duplicate handling
- ✅ Factory methods (of, of_sorted)
- ✅ Equality and consistency

**BatchNeighbors Trait (4 tests)**:

- ✅ Default batch_size() implementation
- ✅ Degree calculation
- ✅ Neighbor access
- ✅ Node count

**SubGraph Implementation (3 tests)**:

- ✅ Creation and basic fields
- ✅ BatchNeighbors trait implementation
- ✅ Original ID preservation

**Total**: 19 new subgraph tests, all passing

## Remaining Work

### High Priority

1. **SubGraph Builder Methods**

   - `buildSubGraph()` - single layer neighborhood sampling
   - `buildSubGraphs()` - multi-layer (GNN message passing)
   - Requires: NeighborhoodFunction trait/interface

2. **Relationship Weights**

   - Implement actual weight lookup in `relationship_weight()`
   - Requires: RelationshipWeights abstraction or Graph API

3. **NeighborhoodSampler Implementation**
   - Integrate with Graph API
   - Use existing UniformSampler and WeightedUniformSampler
   - Handle weighted vs unweighted graphs

### Medium Priority

4. **NeighborhoodFunction Abstraction**

   - Java: functional interface for `LongStream sample(long nodeId)`
   - Rust: trait or closure type
   - Used by SubGraph builder methods

5. **RelationshipWeights Abstraction**
   - Java: functional interface for `double weight(long src, long trg)`
   - Rust: trait or closure type
   - Needed for weighted subgraphs

### Low Priority

6. **Performance Optimization**
   - Profile LocalIdMap (HashMap vs specialized)
   - Profile neighbor storage (Vec<Vec<usize>> vs flattened)
   - Consider memory estimation utilities (Java has MemoryEstimation)

## Next Steps

With the foundation complete, the next major components are:

1. **Graph API Integration** - Connect ml-core to rust-gds graph types
2. **NeighborhoodFunction** - Define sampling interface
3. **Full SubGraph Builder** - Implement multi-layer sampling
4. **Training Loops** - Use SubGraph in actual GNN training

**Milestone Achieved**: 🎉 **Subgraph Foundation Complete**

All core data structures translated, 19 tests passing, ready for Graph API integration.

---

**Learning Note**: This translation demonstrates the "foundation-first" approach:

1. Data structures (LocalIdMap)
2. Interfaces (BatchNeighbors trait)
3. Basic implementations (SubGraph struct)
4. Complex algorithms (builder methods - next phase)

Each layer builds on the previous, allowing continuous validation through tests.
