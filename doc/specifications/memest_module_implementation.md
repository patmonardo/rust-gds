# Memory Estimation Service Layer (memest) Implementation

## Overview

The `memest` module provides higher-level service utilities for memory estimation that sit on top of the existing memory system (`MemoryTree`, `MemoryRange`, `Estimate`). These services handle:

- Pairing graph dimensions with memory trees
- Formatting results for human-readable display
- Validating estimations against memory budgets
- Building structured estimation results

## Implementation Status

**Completed**: Phase 1 - Foundation (3/8 services)

### ✅ Phase 1: Foundation Services (COMPLETE)

1. **GraphMemoryEstimation** (~100 lines, 2 tests) ✅

   - Container pairing `ConcreteGraphDimensions` with `MemoryTree`
   - Provides convenient access to memory ranges and dimensions
   - Methods: `dimensions()`, `memory_tree()`, `memory_range()`, `min_memory()`, `max_memory()`

2. **MemoryEstimationResult** (~170 lines, 6 tests) ✅

   - Structured result builder with human-readable formatting
   - Converts memory trees to displayable output
   - Methods: `memory_usage()`, `format_memory_usage()`, `to_map()`
   - Includes fluent builder: `MemoryEstimationResultBuilder`

3. **MemoryBudgetValidator** (~216 lines, 11 tests) ✅
   - Validates estimations against system memory constraints
   - Budget checking and deficit calculation
   - Methods: `validate()`, `validate_range()`, `remaining()`, `percentage_used()`, `deficit()`, `exceeds_budget()`

**Test Results**: All 19 tests passing (0 failed)

## Architecture Decisions

### 1. Concrete Types vs Trait Objects

**Decision**: Use `ConcreteGraphDimensions` instead of `Box<dyn GraphDimensions>`

**Rationale**:

- `GraphDimensions` trait doesn't implement `Debug` or `Clone`
- Trait objects would require extensive trait bound additions
- `ConcreteGraphDimensions` provides complete functionality
- Simpler API with factory method `ConcreteGraphDimensions::of(node_count, rel_count)`

**Impact**: All memest services use concrete type for graph dimensions

### 2. GraphDimensions API Usage

**Key Methods**:

- `node_count()` - Total number of nodes
- `rel_count_upper_bound()` - Total relationship count (not `relationship_count()`)
- `relationship_counts()` - HashMap of per-type counts (plural)

**Factory Method**: `ConcreteGraphDimensions::of(node_count, relationship_count)`

- **NOT** `::new()` - that doesn't exist
- Creates simple dimensions with node + relationship counts
- Internally stores relationship count in HashMap with `RelationshipType::all_relationships()` key

### 3. Integration with Memory System

The memest services integrate cleanly with existing memory system:

- `MemoryTree` - hierarchical memory breakdown
- `MemoryRange` - min/max memory estimates
- `Estimate::human_readable()` - formatting utility

No changes to core memory system required.

## File Structure

```
src/mem/memest/
├── mod.rs                           # Module exports
├── graph_memory_estimation.rs       # Dimensions + tree container
├── memory_estimation_result.rs      # Result builder + formatting
└── memory_budget_validator.rs       # Budget validation logic
```

## Translation Notes from Java/TypeScript

### Java → Rust Adaptations

1. **GraphDimensions Interface → ConcreteGraphDimensions**

   - Java uses interface with multiple implementations
   - Rust uses single concrete type (simpler, sufficient)

2. **Builder Pattern**

   - Java: Separate builder classes
   - Rust: Fluent builders with `Option<T>` fields + `expect()` for required fields

3. **Method Naming**

   - Java: `getNodeCount()`, `getRelationshipCount()`
   - Rust: `node_count()`, `rel_count_upper_bound()`

4. **Panic vs Exceptions**
   - Java: Throws exceptions for missing required fields
   - Rust: `expect()` with clear panic messages

## Testing Strategy

### Test Coverage

**GraphMemoryEstimation** (2 tests):

- `test_graph_memory_estimation_creation` - Basic construction + access
- `test_memory_range_access` - Memory range queries

**MemoryEstimationResult** (6 tests):

- `test_builder_pattern` - Fluent builder construction
- `test_format_memory_usage` - Human-readable formatting
- `test_to_map` - HashMap serialization
- `test_builder_missing_dimensions` - Panic on missing dimensions
- `test_builder_missing_tree` - Panic on missing tree

**MemoryBudgetValidator** (11 tests):

- `test_validate_within_budget` - Pass when under budget
- `test_validate_exact_budget` - Pass when exactly at budget
- `test_validate_exceeds_budget` - Fail when over budget
- `test_remaining_memory` - Calculate remaining bytes
- `test_remaining_memory_over_budget` - Zero remaining when over
- `test_percentage_used` - Percentage calculation
- `test_percentage_used_over_budget` - Cap at 100%
- `test_percentage_used_zero_budget` - Handle zero budget
- `test_deficit` - Calculate deficit when over budget
- `test_deficit_within_budget` - Zero deficit when under
- `test_validate_range` - Validate MemoryRange objects
- `test_validate_range_exceeds` - Fail when range exceeds

### Test Patterns

All tests follow consistent patterns:

```rust
// Arrange
let dimensions = ConcreteGraphDimensions::of(node_count, rel_count);
let tree = MemoryTree::leaf("Test".to_string(), MemoryRange::of(bytes));

// Act
let result = ServiceType::new(dimensions, tree);

// Assert
assert_eq!(result.some_method(), expected_value);
```

## Pending Work: Phase 2 - Estimation Services

The following services remain to be implemented (from Java/TypeScript sources):

1. **FictitiousGraphStoreEstimationService**

   - Estimate memory for hypothetical graphs with specified dimensions
   - Used for "what-if" analysis before loading data

2. **DatabaseGraphStoreEstimationService**

   - Estimate memory for actual database graphs
   - Queries database for real dimensions

3. **GraphMemoryEstimationService**

   - Core coordination service
   - Delegates to database or fictitious services
   - High-level API for graph memory estimation

4. **AlgorithmMemoryEstimationService**

   - Algorithm-specific memory estimation
   - Combines graph estimation with algorithm overhead
   - Used for pre-flight validation

5. **MultiAlgorithmMemoryEstimation**
   - Combine multiple algorithm estimations
   - Handle pipeline memory requirements
   - Aggregate memory budgets

## Usage Examples

### Basic Memory Estimation

```rust
use rust_gds::mem::memest::{GraphMemoryEstimation, MemoryBudgetValidator};
use rust_gds::core::graph_dimensions::ConcreteGraphDimensions;
use rust_gds::mem::{MemoryTree, MemoryRange};

// Create dimensions
let dimensions = ConcreteGraphDimensions::of(1_000_000, 5_000_000);

// Build memory tree (from algorithm-specific estimation)
let tree = MemoryTree::leaf(
    "Graph Storage".to_string(),
    MemoryRange::of(1024 * 1024 * 1024)  // 1 GiB
);

// Create estimation
let estimation = GraphMemoryEstimation::new(dimensions, tree);

// Check memory requirements
println!("Min memory: {} bytes", estimation.min_memory());
println!("Max memory: {} bytes", estimation.max_memory());
```

### Budget Validation

```rust
use rust_gds::mem::memest::{MemoryEstimationResult, MemoryBudgetValidator};

// Build result
let result = MemoryEstimationResultBuilder::new()
    .with_dimensions(dimensions)
    .with_memory_tree(tree)
    .build();

// Validate against budget
let validator = MemoryBudgetValidator::new(8 * 1024 * 1024 * 1024); // 8 GiB

if validator.validate(&result) {
    println!("✓ Estimation fits within budget");
    println!("  Remaining: {} bytes", validator.remaining(&result));
    println!("  Used: {:.1}%", validator.percentage_used(&result));
} else {
    println!("✗ Insufficient memory!");
    println!("  Deficit: {} bytes", validator.deficit(&result));
}
```

### Human-Readable Output

```rust
// Format for display
let formatted = result.format_memory_usage();
println!("Required memory: {}", formatted);  // e.g., "1.5 GiB"

// Convert to map for serialization
let map = result.to_map();
println!("Node count: {}", map["nodeCount"]);
println!("Relationship count: {}", map["relationshipCount"]);
println!("Required memory: {}", map["requiredMemory"]);
```

## Key Learnings

### 1. ConcreteGraphDimensions API

- Factory method: `ConcreteGraphDimensions::of(node_count, rel_count)`
- **NOT** `::new()` - that doesn't exist
- Total relationship count via `rel_count_upper_bound()` not `relationship_count()`
- Need `use crate::core::graph_dimensions::GraphDimensions;` for trait methods

### 2. File Corruption Risk

- Overlapping string replacements can create duplicate code blocks
- Better to recreate file cleanly than attempt multiple overlapping edits
- Always verify file contents after string replacement

### 3. Trait Objects vs Concrete Types

- Rust trait objects require careful trait bound management
- When possible, prefer concrete types for simpler API
- `Box<dyn Trait>` adds complexity without benefit in this case

### 4. Test-Driven Implementation

- Write tests alongside implementation
- Catch API mismatches early (e.g., `::new()` vs `::of()`)
- Deterministic test data with fixed dimensions

## Module Dependencies

```
memest module depends on:
├── core::graph_dimensions
│   ├── ConcreteGraphDimensions (concrete type)
│   └── GraphDimensions (trait for methods)
└── mem
    ├── MemoryTree (hierarchical estimation)
    ├── MemoryRange (min/max bounds)
    └── Estimate (formatting utilities)
```

## Next Steps

1. Implement Phase 2 estimation services (5 remaining services)
2. Add integration tests with real GraphStore instances
3. Create examples demonstrating service usage
4. Document service coordination patterns
5. Add benchmarks for estimation performance

## Compilation Status

**Result**: ✅ All tests passing

- Compiled without errors
- 19 tests passing (0 failed)
- Only warnings: deprecated TaskStoreHolder (intentional)
- Ready for Phase 2 implementation

---

**Implementation Date**: 2025-01-XX  
**Status**: Phase 1 Complete, Phase 2 Pending  
**Test Coverage**: 19 tests across 3 modules  
**Lines of Code**: ~486 lines (100 + 170 + 216)
