# Memory Estimation Service Layer - Phase 2 Progress Report

**Date**: 2025-10-09  
**Status**: Phase 2A Complete (Fictitious Graph Estimation)  
**Tests**: 26 passing (0 failed)

---

## Overview

Phase 2 extends the memest module with estimation services that can predict memory requirements for graphs of various sizes and configurations. This is critical for capacity planning and pre-flight validation before loading large graphs.

## Completed: Phase 2A - Fictitious Graph Estimation

### FictitiousGraphEstimationService

**File**: `src/mem/memest/fictitious_graph_estimation.rs` (~330 lines, 7 tests)

**Purpose**: Estimates memory usage for hypothetical graphs without requiring a live data source. Enables "what-if" analysis and capacity planning.

**Key Features**:

1. **Simple Estimation**: `estimate(node_count, relationship_count)`

   - Uses empirical formulas for typical graph storage overhead
   - Accounts for node storage (~32 bytes/node), relationship storage (~24 bytes/rel), adjacency lists (~16 bytes/rel)
   - Adds ~10% metadata overhead

2. **Detailed Estimation**: `estimate_detailed(nodes, rels, label_count, property_count)`

   - Includes label storage overhead (~8 bytes per node per label)
   - Includes property storage overhead (~16 bytes per property per element)
   - Separate tracking for node properties vs relationship properties
   - Hierarchical memory tree showing breakdown

3. **Memory Tree Generation**:
   - Builds structured `MemoryTree` showing component breakdown
   - Separates storage into: Node Storage, Relationship Storage, Adjacency Lists, Label Storage, Property Storage, Metadata Overhead
   - Property storage further broken down into node vs relationship properties

**Empirical Formulas**:

```rust
// Base storage
node_memory = node_count * 32      // ID mapping, degree arrays, metadata
rel_memory = rel_count * 24        // source, target, properties pointer
adjacency_memory = rel_count * 16  // forward + backward indices

// Label and property overhead
label_memory = node_count * label_count * 8
node_property_memory = node_count * property_count * 16
rel_property_memory = rel_count * property_count * 16

// Overhead
base_overhead = (node + rel + adjacency) / 10  // ~10%
property_overhead = (node_props + rel_props) / 20  // ~5%
```

**Test Coverage** (7 tests):

- `test_simple_estimation` - Basic construction and access
- `test_detailed_estimation` - Detailed vs simple comparison
- `test_memory_scaling` - Linear scaling validation
- `test_zero_relationships` - Edge case: nodes only
- `test_memory_tree_structure` - Tree structure validation
- `test_detailed_with_properties` - Property overhead
- `test_detailed_with_labels` - Label overhead

**Example Usage**:

```rust
use rust_gds::mem::memest::FictitiousGraphEstimationService;

let service = FictitiousGraphEstimationService::new();

// Simple estimation
let estimation = service.estimate(1_000_000, 5_000_000);
println!("Memory: {} bytes", estimation.min_memory());

// Detailed with 3 labels and 5 properties per element
let detailed = service.estimate_detailed(1_000_000, 5_000_000, 3, 5);
println!("Detailed memory: {} bytes", detailed.min_memory());
```

---

## Examples

### memest_showcase.rs

**Purpose**: Demonstrates complete workflow for memory estimation and capacity planning.

**Scenarios**:

1. **Simple Graph Estimation** - Compare small/medium/large graphs
2. **Detailed Estimation** - Show impact of labels and properties
3. **Budget Validation** - Test against various memory budgets
4. **Capacity Planning** - Analyze multiple graph scenarios

**Sample Output**:

```
Small graph (10K nodes, 50K rels):
  Min memory: 2.43 MiB

Medium graph (100K nodes, 1M rels):
  Min memory: 45.32 MiB

Large graph (1M nodes, 10M rels):
  Min memory: 453.19 MiB

Graph with 3 labels and 5 properties per element:
  Memory: 135.73 MiB
  Overhead: 90.41 MiB (+199.5%)

Large budget (1.00 GiB): ✓ FITS
    Used: 22.1% | Remaining: 797.41 MiB
```

---

## Integration with Phase 1

The Phase 2A service builds on Phase 1 foundations:

```
FictitiousGraphEstimationService
  └─> GraphMemoryEstimation (container)
       ├─> ConcreteGraphDimensions (from core)
       └─> MemoryTree (hierarchical breakdown)
            └─> MemoryRange (min/max bytes)

MemoryEstimationResultBuilder
  └─> MemoryEstimationResult (formatted output)
       ├─> format_memory_usage() [uses Estimate::human_readable()]
       └─> to_map() [serialization]

MemoryBudgetValidator
  └─> validate(MemoryEstimationResult)
       ├─> remaining()
       ├─> percentage_used()
       └─> deficit()
```

**Complete Flow**:

1. Service creates estimation with `estimate()` or `estimate_detailed()`
2. Returns `GraphMemoryEstimation` (dimensions + memory tree)
3. Wrap in `MemoryEstimationResult` for formatting
4. Validate with `MemoryBudgetValidator`
5. Display with human-readable formatting

---

## Test Results

**Module**: `src/mem/memest/`  
**Command**: `cargo test --lib memest --features core`

```
running 26 tests

Phase 1 Tests (19):
test mem::memest::graph_memory_estimation::tests::test_graph_memory_estimation_creation ... ok
test mem::memest::graph_memory_estimation::tests::test_memory_range_access ... ok
test mem::memest::memory_estimation_result::tests::test_builder_pattern ... ok
test mem::memest::memory_estimation_result::tests::test_format_memory_usage ... ok
test mem::memest::memory_estimation_result::tests::test_to_map ... ok
test mem::memest::memory_estimation_result::tests::test_builder_missing_dimensions ... ok
test mem::memest::memory_estimation_result::tests::test_builder_missing_tree ... ok
test mem::memest::memory_budget_validator::tests::test_validate_within_budget ... ok
test mem::memest::memory_budget_validator::tests::test_validate_exact_budget ... ok
test mem::memest::memory_budget_validator::tests::test_validate_exceeds_budget ... ok
test mem::memest::memory_budget_validator::tests::test_remaining_memory ... ok
test mem::memest::memory_budget_validator::tests::test_remaining_memory_over_budget ... ok
test mem::memest::memory_budget_validator::tests::test_percentage_used ... ok
test mem::memest::memory_budget_validator::tests::test_percentage_used_over_budget ... ok
test mem::memest::memory_budget_validator::tests::test_percentage_used_zero_budget ... ok
test mem::memest::memory_budget_validator::tests::test_deficit ... ok
test mem::memest::memory_budget_validator::tests::test_deficit_within_budget ... ok
test mem::memest::memory_budget_validator::tests::test_validate_range ... ok
test mem::memest::memory_budget_validator::tests::test_validate_range_exceeds ... ok

Phase 2A Tests (7):
test mem::memest::fictitious_graph_estimation::tests::test_simple_estimation ... ok
test mem::memest::fictitious_graph_estimation::tests::test_detailed_estimation ... ok
test mem::memest::fictitious_graph_estimation::tests::test_memory_scaling ... ok
test mem::memest::fictitious_graph_estimation::tests::test_zero_relationships ... ok
test mem::memest::fictitious_graph_estimation::tests::test_memory_tree_structure ... ok
test mem::memest::fictitious_graph_estimation::tests::test_detailed_with_properties ... ok
test mem::memest::fictitious_graph_estimation::tests::test_detailed_with_labels ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured
```

---

## Key Insights

### 1. Empirical Formulas Work Well

The empirical formulas for memory estimation produce reasonable results:

- Small graph (10K nodes, 50K rels): ~2.4 MiB
- Medium graph (100K nodes, 1M rels): ~45 MiB
- Large graph (1M nodes, 10M rels): ~453 MiB

Memory scales roughly linearly with graph size, with expected overhead for metadata.

### 2. Property Impact is Significant

Properties add substantial overhead:

- Graph without properties: 45 MiB
- Same graph with 5 properties/element: 136 MiB (+200%)

This validates the need for detailed estimation when properties are involved.

### 3. Labels Have Modest Impact

Label overhead is much smaller than property overhead:

- Graph without labels: 45 MiB
- Same graph with 3 labels: 48 MiB (+5%)

Labels are relatively cheap compared to properties.

### 4. Capacity Planning is Powerful

The fictitious service enables answering questions like:

- "Can I load a 10M node graph on an 8GB machine?" → Yes (4.4 GB, 55% of budget)
- "How much memory for 100K nodes with 5 properties each?" → 136 MiB
- "What's the largest graph I can fit in 2GB?" → ~4.4M nodes, 44M relationships

---

## Pending Work: Phase 2B - Advanced Services

The following services are **blocked** and require additional infrastructure:

### 1. DatabaseGraphStoreEstimationService

**Blocked by**: GraphStoreFactory, GraphLoaderContext
**Reason**: Needs ability to query actual database for real dimensions

**Design**:

```rust
pub struct DatabaseGraphStoreEstimationService {
    graph_loader_context: GraphLoaderContext,
    user: User,
}

impl DatabaseGraphStoreEstimationService {
    pub fn estimate(&self, config: GraphProjectConfig) -> GraphMemoryEstimation {
        // Get factory from supplier
        let factory = GraphStoreFactorySupplier::supplier(config)
            .get(self.graph_loader_context);

        // Return actual dimensions + memory estimate
        GraphMemoryEstimation::new(
            factory.dimensions(),
            factory.estimate_memory_usage_after_loading()
        )
    }
}
```

**Dependencies Needed**:

- `GraphStoreFactory` trait with `dimensions()` and `estimate_memory_usage_after_loading()`
- `GraphStoreFactorySupplier` for obtaining factories
- `GraphLoaderContext` for database access
- `GraphProjectConfig` for projection configuration

### 2. GraphMemoryEstimationService

**Blocked by**: GraphStoreCatalog, algorithm infrastructure
**Reason**: Needs to access existing graphs and combine with algorithm estimations

**Design**:

```rust
pub struct GraphMemoryEstimationService;

impl GraphMemoryEstimationService {
    pub fn estimate_algorithm_for_graph(
        &self,
        graph_name: GraphName,
        algorithm_estimation: MemoryEstimation,
        user: User
    ) -> GraphMemoryEstimation {
        // Get graph from catalog
        let graph_store = GraphStoreCatalog::get(user.username(), graph_name).graph_store();

        // Estimate algorithm on graph dimensions
        let tree = algorithm_estimation.estimate(
            graph_store.dimensions(),
            graph_store.concurrency()
        );

        GraphMemoryEstimation::new(graph_store.dimensions(), tree)
    }
}
```

**Dependencies Needed**:

- `GraphStoreCatalog` for accessing existing graphs
- `MemoryEstimation` trait for algorithms
- `GraphName` type
- `User` authentication type

### 3. AlgorithmMemoryEstimationService

**Blocked by**: Algorithm definition infrastructure
**Reason**: Needs `MemoryEstimateDefinition` trait for algorithms

**Design**:

```rust
pub struct AlgorithmMemoryEstimationService {
    graph_service: GraphMemoryEstimationService,
}

impl AlgorithmMemoryEstimationService {
    pub fn estimate(
        &self,
        graph_name: GraphName,
        algorithm_def: MemoryEstimateDefinition,
        config: AlgorithmConfiguration,
        user: User
    ) -> GraphMemoryEstimation {
        let estimation = algorithm_def.memory_estimation();
        self.graph_service.estimate_algorithm_for_graph(graph_name, estimation, user)
    }
}
```

**Dependencies Needed**:

- `MemoryEstimateDefinition` trait for algorithm definitions
- `AlgorithmConfiguration` type
- Graph service from #2

### 4. MultiAlgorithmMemoryEstimation

**Blocked by**: Algorithm infrastructure
**Reason**: Needs algorithm definitions and combined estimations

**Design**:

```rust
pub struct MultiAlgorithmMemoryEstimation {
    definitions: Vec<(MemoryEstimateDefinition, AlgorithmConfiguration)>,
}

impl MultiAlgorithmMemoryEstimation {
    pub fn add(&mut self, def: MemoryEstimateDefinition, config: AlgorithmConfiguration);

    pub fn estimate(&self) -> MemoryEstimation {
        let estimations: Vec<_> = self.definitions.iter()
            .map(|(def, _)| def.memory_estimation())
            .collect();

        MemoryEstimations::max_estimation("Maximum algorithm memory", estimations)
    }
}
```

**Dependencies Needed**:

- `MemoryEstimations::max_estimation()` combinator
- Algorithm infrastructure from #3

---

## Recommendations

### Short Term: Complete What's Possible Now

✅ **FictitiousGraphEstimationService** - Complete and working
✅ **Integration with existing memest Phase 1** - Complete
✅ **Example demonstrating full workflow** - Complete
✅ **Documentation** - In progress

### Medium Term: Build GraphStore Factory Infrastructure

Before implementing Phase 2B services, we need:

1. **GraphStoreFactory trait** with memory estimation capabilities
2. **GraphLoaderContext** for database access
3. **GraphStoreCatalog** for managing loaded graphs
4. **GraphProjectConfig** enhancements

### Long Term: Algorithm Memory Estimation

For algorithm-specific estimation:

1. **MemoryEstimation trait** for algorithms
2. **MemoryEstimateDefinition** for algorithm metadata
3. **Algorithm registry/catalog**
4. **Combined estimation strategies**

---

## Summary

**Phase 2A Achievement**: FictitiousGraphEstimationService provides powerful capacity planning capabilities without requiring any database infrastructure. Users can:

- Estimate memory for hypothetical graphs
- Compare different graph sizes
- Validate against memory budgets
- Plan capacity for future workloads

**Lines of Code**:

- Phase 1: ~486 lines (3 modules)
- Phase 2A: ~330 lines (1 module + example)
- **Total**: ~816 lines, 26 tests passing

**Next Steps**:

- ✅ Document Phase 2A completion
- ⏸️ Wait for GraphStore factory infrastructure
- ⏸️ Implement Phase 2B services when dependencies available

The fictitious estimation service provides immediate value for capacity planning and can be used standalone. Advanced services requiring database integration and algorithm definitions can be added incrementally as the infrastructure becomes available.

---

**Implementation Date**: 2025-10-09  
**Status**: Phase 2A Complete ✅  
**Tests**: 26/26 passing ✅  
**Ready for**: Capacity planning, "what-if" analysis, memory budget validation
