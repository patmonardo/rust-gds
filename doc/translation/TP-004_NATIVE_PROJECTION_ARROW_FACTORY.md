# Translation Plan TP-004: Native Projection → Arrow Factory

**Document Type**: Translation Plan (Prakasa)  
**Translation ID**: TP-004  
**Date**: October 15, 2025  
**Status**: 🌟 Prakasa (Illumination) - Ready for Kriya (Action)  
**Estimated Effort**: 24-30 hours (8 phases, ~70 source files → 35 Rust files)  
**Priority**: 🎯 CRITICAL - "The Absolute Form's Kernel"

---

## 🕉️ Membership Protocol (Fichte's Method)

**This Translation Plan places itself within the rust-gds Encyclopedia as**:

- **Location**: `doc/translation/TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md`
- **Category**: Translation Plans (Prakasa → Kriya bridge)
- **Related ADR**: ADR0007 (Translation Plan Protocol)
- **Related Design**: `NATIVE_PROJECTION_ARROW_DESIGN.md`
- **Parent Plan**: TP-002 (Graph Projection API)
- **Precedence**: Follows LinkPipeline completion (25/25 files)

**Purpose**: Translate Java GDS Native Projection (~70 files) into Arrow-native Factory system for rust-gds. This is the **entry point for all external data** into GraphStore.

---

## Context & Motivation

### The Core Insight

> "This works with Neo4j Projection because Neo4j is Native for GDS. It won't be for us. But if Arrow is Native for us then we now know what to translate this into."

**What we're translating**:

- **Source**: Neo4j GDS Native Projection (~70 Java files)
- **Pattern**: Neo4j Cursors → GDS Storage (Neo4j IS native for Java GDS)
- **Target**: Arrow-native Factory system (Arrow IS native for rust-gds!)

**Why NOT 1:1 translation**:

- Neo4j cursors ≠ Arrow batches (different data model)
- Database API ≠ In-memory columnar (different access pattern)
- Opportunity for zero-copy optimization (Arrow is ALREADY columnar!)

### Current State Analysis

**Existing rust-gds structure**:

```
src/projection/
├── traits/           ✅ Projection API (ElementProjection, PropertyMapping)
├── impls/            ✅ Implementations (NodeProjection, PropertyMappings)
├── codegen/          ✅ Eval macro system (value_type_table!, functors)
├── native/           🚧 UNCLEAR PURPOSE - ML stuff? Local execution?
│   ├── ml/           (ML pipelines)
│   ├── form/         (Form processing)
│   └── native_factory.rs  (EMPTY!)
└── node_label.rs, orientation.rs, etc.
```

**Key insight**: `projection/native/` is NOT about "native data sources" - it's about "native/local execution" (ML, codegen)!

### The Naming Problem

**Current confusion**:

- `projection/native/` implies "native data sources" but actually contains ML/execution code
- `native_factory.rs` is empty (placeholder?)
- "Native" means different things in different contexts

**Solution**: Create `projection/factory/` for native data source factories!

---

## Translation Strategy

### NOT a 1:1 Translation

**Why Design-Driven Translation**:

1. **Conceptual shift**: Database cursors → Arrow batches
2. **Different native source**: Neo4j Transaction API → Arrow RecordBatch API
3. **Zero-copy opportunity**: Arrow is already columnar!
4. **rust-gds specific**: Defines our native interface

**Translation approach**:

- **Structure**: Keep the architecture patterns (Scanner, Task, Importer, Consumer)
- **Types**: Replace Neo4j cursors with Arrow batches
- **Optimization**: Add zero-copy paths where Arrow arrays map directly to PropertyValues
- **Extension**: Design for multiple native sources (Arrow, Polars, DuckDB, Neo4j)

### Module Organization

**NEW top-level location**:

```
src/projection/factory/         (NEW - "The Absolute Form's Kernel")
├── mod.rs                      (Factory trait + prelude)
├── arrow/                      (Arrow-native factory - PRIORITY)
│   ├── mod.rs                  (Arrow factory public API)
│   ├── factory.rs              (ArrowNativeFactory entry point)
│   ├── reference.rs            (TableReference, BatchReference)
│   ├── scanner.rs              (BatchScanner trait + impls)
│   ├── task.rs                 (ParallelImportTask)
│   ├── importer.rs             (NodeBatchImporter, EdgeBatchImporter)
│   ├── consumer.rs             (BufferedConsumers)
│   ├── properties.rs           (Arrow column → Property mapping)
│   └── config.rs               (ArrowProjectionConfig)
│
├── polars/                     (Future: Polars-native)
│   └── ...
│
└── neo4j/                      (Future: Neo4j-native - compatibility)
    └── ...
```

**Rationale**:

- **Top-level `factory/`**: Factories are "entry points for AI" (user quote)
- **Extensible**: Arrow first, but designed for multiple native sources
- **Clear separation**: `projection/native/` stays ML/execution, `projection/factory/` is data ingestion
- **Semantic clarity**: "Factory" = creating GraphStores from native data sources

---

## Source Material Analysis

### Java GDS Native Projection Structure

**Location**: `/home/pat/GitHub/graph-data-science/native-projection/src/main/java/org/neo4j/gds/projection/`

**~70 Java files organized as**:

#### 1. Factory Pattern (2 files → 1 Rust file)

```
NativeFactory.java (191 lines)                    → factory.rs (ArrowNativeFactory)
NativeFactorySupplierProvider.java (68 lines)     → (integrated into factory.rs)
```

#### 2. Reference System (7 files → 3 Rust files)

```
RecordReference.java                              → reference.rs (ArrowReference trait)
├── NodeReference.java                            → reference.rs (NodeTableReference)
│   ├── NodeCursorReference.java                  → (integrated - single table type)
│   └── NodeLabelIndexReference.java              → (integrated - label filtering)
│       └── MultipleNodeLabelIndexReference.java  → (integrated - multi-label)
└── RelationshipReference.java                    → reference.rs (EdgeTableReference)
    └── RelationshipScanCursorReference.java      → (integrated)
```

#### 3. Scanner System (8 files → 4 Rust files)

```
StoreScanner.java (interface)                     → scanner.rs (BatchScanner trait)
├── NodeCursorBasedScanner.java                   → scanner.rs (NodeBatchScanner)
├── NodeLabelIndexBasedScanner.java               → (integrated - label filtering)
├── MultipleNodeLabelIndexBasedScanner.java       → (integrated - multi-label)
├── RelationshipScanCursorBasedScanner.java       → scanner.rs (EdgeBatchScanner)
├── AbstractCursorBasedScanner.java (174 lines)   → scanner.rs (base logic)
└── AbstractNodeCursorBasedScanner.java           → (integrated)
```

#### 4. Task System (3 files → 2 Rust files)

```
RecordScannerTaskRunner.java (84 lines)           → task.rs (ParallelTaskRunner)
├── NodesScannerTask.java (211 lines)             → task.rs (NodeImportTask)
└── RelationshipsScannerTask.java (254 lines)     → task.rs (EdgeImportTask)
```

#### 5. Importer System (3 files → 3 Rust files)

```
ScanningRecordsImporter.java (126 lines)          → importer.rs (base importer logic)
├── ScanningNodesImporter.java (221 lines)        → importer.rs (NodeBatchImporter)
└── ScanningRelationshipsImporter.java (186 lines)→ importer.rs (EdgeBatchImporter)
```

#### 6. Consumer System (3 files → 2 Rust files)

```
BufferedNodeConsumer.java (123 lines)             → consumer.rs (BufferedNodeConsumer)
BufferedRelationshipConsumer.java (103 lines)     → consumer.rs (BufferedEdgeConsumer)
BufferedCompositeRelationshipConsumer.java (59)   → (integrated into EdgeConsumer)
```

#### 7. Property Helpers (3 files → 1 Rust file)

```
NativeNodePropertyImporter.java (364 lines)       → properties.rs (ArrowPropertyMapper)
NativeRelationshipPropertyReadHelper.java (95)    → (integrated into properties.rs)
LoadablePropertyMappings.java (57 lines)          → (use existing PropertyMappings)
```

#### 8. Configuration & Utilities (8 files → 2 Rust files)

```
GraphProjectFromStoreConfig.java (199 lines)      → config.rs (ArrowProjectionConfig)
GraphDimensionsReader.java (269 lines)            → (integrated - size estimation)
GraphDimensionsValidation.java (91 lines)         → (integrated - validation)
ScanState.java (72 lines)                         → (integrated into task.rs)
CompositeNodeScan.java (55 lines)                 → (integrated into scanner.rs)
NodeScannerFactory.java (94 lines)                → (integrated into factory.rs)
NodeLabelIndexLookupImpl.java (68 lines)          → (integrated into scanner.rs)
GraphProjectNativeResult.java (66 lines)          → (use existing result types)
```

**Total**: ~70 Java files (4,500+ lines) → ~35 Rust files (estimated 3,000-4,000 lines)

---

## Phase Breakdown

### Phase 1: Core Infrastructure (4-5 hours)

**Goal**: Factory entry point + basic types

**Files to create**:

1. `src/projection/factory/mod.rs` (50 lines)

   - Factory trait (abstract interface)
   - GraphStoreFactory trait
   - Re-exports and prelude

2. `src/projection/factory/arrow/mod.rs` (30 lines)

   - Arrow factory module
   - Public API surface

3. `src/projection/factory/arrow/factory.rs` (150 lines)

   - `ArrowNativeFactory` struct
   - `from_tables()` constructor
   - `build_graph_store()` method skeleton
   - Basic error types

4. `src/projection/factory/arrow/config.rs` (100 lines)
   - `ArrowProjectionConfig` struct
   - Builder pattern
   - Validation

**Source files**: NativeFactory.java, GraphProjectFromStoreConfig.java

**Tests**: Factory creation, config validation

**Completion criteria**: Factory instantiates, config validates, compiles with zero errors

---

### Phase 2: Reference System (3-4 hours)

**Goal**: Arrow table references and schema mapping

**Files to create**:

1. `src/projection/factory/arrow/reference.rs` (250 lines)
   - `ArrowReference` trait
   - `NodeTableReference` struct
   - `EdgeTableReference` struct
   - `BatchIterator` for chunking
   - Schema inference helpers

**Source files**:

- RecordReference.java
- NodeReference.java, NodeCursorReference.java
- NodeLabelIndexReference.java, MultipleNodeLabelIndexReference.java
- RelationshipReference.java, RelationshipScanCursorReference.java

**Key changes from Java**:

- Replace Neo4j cursor references with Arrow table references
- Add schema metadata support (Arrow schema → GraphStore schema)
- Support chunked iteration over large tables

**Tests**: Table references, schema inference, batch iteration

**Completion criteria**: Can create table references, infer schemas, iterate batches

---

### Phase 3: Scanner System (5-6 hours)

**Goal**: Parallel batch scanning of Arrow tables

**Files to create**:

1. `src/projection/factory/arrow/scanner.rs` (400 lines)
   - `BatchScanner` trait
   - `NodeBatchScanner` impl
   - `EdgeBatchScanner` impl
   - `AbstractBatchScanner` base logic
   - Parallel scan coordination

**Source files**:

- StoreScanner.java (91 lines)
- AbstractCursorBasedScanner.java (174 lines)
- NodeCursorBasedScanner.java (59 lines)
- NodeLabelIndexBasedScanner.java (66 lines)
- MultipleNodeLabelIndexBasedScanner.java (77 lines)
- RelationshipScanCursorBasedScanner.java (86 lines)
- AbstractNodeCursorBasedScanner.java (46 lines)
- NodeScannerFactory.java (94 lines)

**Key changes from Java**:

- Replace Neo4j cursor scanning with Arrow batch scanning
- Parallel iteration over RecordBatches
- Label filtering using Arrow compute kernels
- Memory-efficient streaming (don't load entire table)

**Tests**:

- Sequential scanning
- Parallel scanning
- Label filtering
- Large table handling

**Completion criteria**: Can scan node/edge tables in parallel, filter by label, memory-bounded

---

### Phase 4: Task System (4-5 hours)

**Goal**: Parallel import task orchestration

**Files to create**:

1. `src/projection/factory/arrow/task.rs` (350 lines)
   - `ImportTask` trait
   - `NodeImportTask` struct
   - `EdgeImportTask` struct
   - `ParallelTaskRunner` orchestration
   - `ScanState` for progress tracking

**Source files**:

- RecordScannerTaskRunner.java (84 lines)
- NodesScannerTask.java (211 lines)
- RelationshipsScannerTask.java (254 lines)
- ScanState.java (72 lines)

**Key changes from Java**:

- Use Rayon for parallel task execution
- Async/await for concurrent imports (optional)
- Progress tracking with atomic counters
- Error aggregation across tasks

**Tests**:

- Single task execution
- Parallel task execution
- Error handling
- Progress reporting

**Completion criteria**: Can execute import tasks in parallel, aggregate results, handle errors

---

### Phase 5: Importer System (6-7 hours)

**Goal**: Import Arrow batches into GraphStore

**Files to create**:

1. `src/projection/factory/arrow/importer.rs` (500 lines)
   - `BatchImporter` trait
   - `NodeBatchImporter` impl
   - `EdgeBatchImporter` impl
   - Integration with GraphStore
   - ID mapping (Arrow IDs → GDS node IDs)

**Source files**:

- ScanningRecordsImporter.java (126 lines)
- ScanningNodesImporter.java (221 lines)
- ScanningRelationshipsImporter.java (186 lines)

**Key changes from Java**:

- Replace Neo4j node references with Arrow row indices
- Use GraphStore builders directly
- Zero-copy where Arrow arrays map to PropertyValues
- Batch processing for efficiency

**Integration points**:

- GraphStore node/edge builders
- NodeIdMapper (Arrow ID → GDS ID)
- PropertyStore writers
- Topology builders

**Tests**:

- Node import (single batch)
- Edge import (single batch)
- ID mapping correctness
- Property import

**Completion criteria**: Can import nodes and edges from Arrow batches into GraphStore

---

### Phase 6: Property Mapping (4-5 hours)

**Goal**: Map Arrow columns to GraphStore properties

**Files to create**:

1. `src/projection/factory/arrow/properties.rs` (400 lines)
   - `ArrowPropertyMapper` struct
   - `PropertyMappingRegistry` for configuration
   - Column type inference
   - Zero-copy optimization paths
   - Type conversion helpers

**Source files**:

- NativeNodePropertyImporter.java (364 lines)
- NativeRelationshipPropertyReadHelper.java (95 lines)
- LoadablePropertyMappings.java (57 lines)

**Key changes from Java**:

- Arrow column types → ValueType mapping
- Zero-copy path: Arrow array → PropertyValues (when types align)
- Copy path: Arrow array → converted PropertyValues (when types differ)
- Null handling (Arrow nullability → GDS defaults)

**Integration with existing**:

- Use `PropertyMappings` from `projection/impls/`
- Use `PropertyMapping` trait from `projection/traits/`
- Leverage eval macro system for type conversions

**Tests**:

- Column type inference
- Zero-copy mapping (i64, f64, String)
- Type conversion (i32→i64, f32→f64)
- Null handling
- Default values

**Completion criteria**: Can map Arrow columns to properties, zero-copy where possible

---

### Phase 7: Consumer System (3-4 hours)

**Goal**: Buffered writers for GraphStore

**Files to create**:

1. `src/projection/factory/arrow/consumer.rs` (250 lines)
   - `BufferedNodeConsumer` struct
   - `BufferedEdgeConsumer` struct
   - Flush strategies (size-based, time-based)
   - Error handling

**Source files**:

- BufferedNodeConsumer.java (123 lines)
- BufferedRelationshipConsumer.java (103 lines)
- BufferedCompositeRelationshipConsumer.java (59 lines)

**Key changes from Java**:

- Use Rust iterators for batch processing
- RAII for automatic flushing (Drop trait)
- Configurable buffer sizes
- Support multi-type edges (composite)

**Tests**:

- Buffering behavior
- Automatic flushing
- Manual flush
- Error propagation

**Completion criteria**: Buffered consumers reduce GraphStore write contention

---

### Phase 8: Integration & Optimization (4-5 hours)

**Goal**: End-to-end integration and zero-copy optimization

**Tasks**:

1. Wire all components together in `ArrowNativeFactory::build_graph_store()`
2. Add zero-copy fast paths (Arrow array → PropertyValues)
3. Performance benchmarks
4. Documentation and examples
5. Integration tests

**Zero-copy optimization**:

```rust
// When Arrow Int64Array maps directly to Long PropertyValues
impl ArrowPropertyMapper {
    fn zero_copy_path(&self, arrow_array: &Int64Array) -> PropertyValues {
        // Wrap Arrow buffer directly in PropertyValues (zero-copy!)
        PropertyValues::from_arrow_buffer(arrow_array.values())
    }
}
```

**Integration tests**:

- Small graph import (correctness)
- Large graph import (performance)
- Multi-label nodes
- Multi-type edges
- Property import
- Error cases

**Documentation**:

- Module-level docs
- Usage examples
- Performance characteristics
- Zero-copy optimization guide

**Completion criteria**:

- End-to-end Arrow → GraphStore working
- Zero-copy paths verified
- Benchmarks show performance gains
- Full documentation

---

## File Mapping (Java → Rust)

### Core Factory (2 Java → 1 Rust)

| Java File                          | Lines | Rust File    | Est. Lines | Phase |
| ---------------------------------- | ----- | ------------ | ---------- | ----- |
| NativeFactory.java                 | 191   | factory.rs   | 150        | 1     |
| NativeFactorySupplierProvider.java | 68    | (integrated) | -          | 1     |

### Reference System (7 Java → 1 Rust)

| Java File                            | Lines | Rust File    | Est. Lines | Phase |
| ------------------------------------ | ----- | ------------ | ---------- | ----- |
| RecordReference.java                 | 23    | reference.rs | 250        | 2     |
| NodeReference.java                   | 34    | (integrated) | -          | 2     |
| NodeCursorReference.java             | 54    | (integrated) | -          | 2     |
| NodeLabelIndexReference.java         | 76    | (integrated) | -          | 2     |
| MultipleNodeLabelIndexReference.java | 78    | (integrated) | -          | 2     |
| RelationshipReference.java           | 35    | (integrated) | -          | 2     |
| RelationshipScanCursorReference.java | 57    | (integrated) | -          | 2     |

### Scanner System (8 Java → 1 Rust)

| Java File                               | Lines | Rust File    | Est. Lines | Phase |
| --------------------------------------- | ----- | ------------ | ---------- | ----- |
| StoreScanner.java                       | 91    | scanner.rs   | 400        | 3     |
| AbstractCursorBasedScanner.java         | 174   | (base logic) | -          | 3     |
| NodeCursorBasedScanner.java             | 59    | (integrated) | -          | 3     |
| NodeLabelIndexBasedScanner.java         | 66    | (integrated) | -          | 3     |
| MultipleNodeLabelIndexBasedScanner.java | 77    | (integrated) | -          | 3     |
| RelationshipScanCursorBasedScanner.java | 86    | (integrated) | -          | 3     |
| AbstractNodeCursorBasedScanner.java     | 46    | (integrated) | -          | 3     |
| NodeScannerFactory.java                 | 94    | (integrated) | -          | 3     |

### Task System (4 Java → 1 Rust)

| Java File                     | Lines | Rust File    | Est. Lines | Phase |
| ----------------------------- | ----- | ------------ | ---------- | ----- |
| RecordScannerTaskRunner.java  | 84    | task.rs      | 350        | 4     |
| NodesScannerTask.java         | 211   | (integrated) | -          | 4     |
| RelationshipsScannerTask.java | 254   | (integrated) | -          | 4     |
| ScanState.java                | 72    | (integrated) | -          | 4     |

### Importer System (3 Java → 1 Rust)

| Java File                          | Lines | Rust File    | Est. Lines | Phase |
| ---------------------------------- | ----- | ------------ | ---------- | ----- |
| ScanningRecordsImporter.java       | 126   | importer.rs  | 500        | 5     |
| ScanningNodesImporter.java         | 221   | (integrated) | -          | 5     |
| ScanningRelationshipsImporter.java | 186   | (integrated) | -          | 5     |

### Property Mapping (3 Java → 1 Rust)

| Java File                                 | Lines | Rust File      | Est. Lines | Phase |
| ----------------------------------------- | ----- | -------------- | ---------- | ----- |
| NativeNodePropertyImporter.java           | 364   | properties.rs  | 400        | 6     |
| NativeRelationshipPropertyReadHelper.java | 95    | (integrated)   | -          | 6     |
| LoadablePropertyMappings.java             | 57    | (use existing) | -          | 6     |

### Consumer System (3 Java → 1 Rust)

| Java File                                  | Lines | Rust File    | Est. Lines | Phase |
| ------------------------------------------ | ----- | ------------ | ---------- | ----- |
| BufferedNodeConsumer.java                  | 123   | consumer.rs  | 250        | 7     |
| BufferedRelationshipConsumer.java          | 103   | (integrated) | -          | 7     |
| BufferedCompositeRelationshipConsumer.java | 59    | (integrated) | -          | 7     |

### Configuration & Utilities (8 Java → 2 Rust)

| Java File                        | Lines | Rust File      | Est. Lines | Phase |
| -------------------------------- | ----- | -------------- | ---------- | ----- |
| GraphProjectFromStoreConfig.java | 199   | config.rs      | 100        | 1     |
| GraphDimensionsReader.java       | 269   | (integrated)   | -          | 8     |
| GraphDimensionsValidation.java   | 91    | (integrated)   | -          | 1     |
| CompositeNodeScan.java           | 55    | (integrated)   | -          | 3     |
| NodeLabelIndexLookupImpl.java    | 68    | (integrated)   | -          | 3     |
| GraphProjectNativeResult.java    | 66    | (use existing) | -          | 8     |

**Summary**: ~35 Java files (2,900+ lines) → 9 Rust files (2,400+ lines) + integrated logic

---

## Integration with Existing rust-gds

### Dependencies

**Must use**:

- `projection/traits/` - ElementProjection, PropertyMapping
- `projection/impls/` - PropertyMappings, NodeProjection
- `projection/codegen/` - value_type_table!, functors (for type conversion)
- `types/` - ValueType, DefaultValue, PropertyValues
- GraphStore builders

**Must NOT conflict with**:

- `projection/native/` (rename consideration: `projection/execution/`?)

### Property System Integration

**Existing**:

```rust
// projection/impls/property_mappings.rs
pub struct PropertyMappings {
    mappings: Vec<PropertyMapping>,
}
```

**New integration**:

```rust
// projection/factory/arrow/properties.rs
impl ArrowPropertyMapper {
    pub fn from_property_mapping(
        mapping: &PropertyMapping,
        arrow_schema: &Schema,
    ) -> Result<Self> {
        // Map PropertyMapping → ArrowPropertyMapper
    }
}

// Convert PropertyMappings → ArrowPropertyMapper registry
pub fn create_property_registry(
    mappings: &PropertyMappings,
    schema: &Schema,
) -> PropertyMappingRegistry;
```

**Zero-copy leverage**:

```rust
// Use existing value_type_table! functors for type conversion
use crate::projection::functors::{GrossToSubtle, SubtleToGross};
use crate::projection::value_type_table::Long;

impl ArrowPropertyMapper {
    fn convert_with_functor(&self, arrow_value: i64) -> PropertyValue {
        let functor = Long::Functor;
        functor.project_to_storage(arrow_value)
    }
}
```

---

## Testing Strategy

### Unit Tests (per phase)

- Phase 1: Factory creation, config validation
- Phase 2: Table references, schema inference, batch iteration
- Phase 3: Sequential/parallel scanning, label filtering
- Phase 4: Task execution, error handling, progress tracking
- Phase 5: Node/edge import, ID mapping
- Phase 6: Column type inference, zero-copy, conversions
- Phase 7: Buffering, flushing, error propagation

### Integration Tests (Phase 8)

1. **Small graph import** (correctness)

   - 10 nodes, 20 edges
   - Multiple labels/types
   - Properties of various types
   - Verify GraphStore contents

2. **Large graph import** (performance)

   - 1M nodes, 5M edges
   - Measure throughput (nodes/sec, edges/sec)
   - Memory usage
   - Parallel scaling

3. **Zero-copy verification**

   - Benchmark with zero-copy vs copy paths
   - Measure memory allocations
   - Verify correctness

4. **Error cases**
   - Invalid schemas
   - Type mismatches
   - Missing required columns
   - Null handling

### Benchmarks (Phase 8)

```rust
#[bench]
fn bench_arrow_import_nodes(b: &mut Bencher) {
    let table = create_test_arrow_table(1_000_000);
    b.iter(|| {
        let factory = ArrowNativeFactory::from_tables(vec![table.clone()], vec![]);
        factory.build_graph_store().unwrap();
    });
}
```

---

## Success Criteria

### Per Phase

- ✅ All files compile with zero errors
- ✅ All unit tests pass
- ✅ Clippy clean (zero warnings)
- ✅ Rustfmt compliant

### Overall (Phase 8)

- ✅ End-to-end Arrow → GraphStore working
- ✅ All integration tests pass
- ✅ Zero-copy paths verified (benchmarks)
- ✅ Performance acceptable (>100K nodes/sec single-threaded)
- ✅ Memory usage bounded (streaming, not loading entire table)
- ✅ Full documentation (module docs, examples)
- ✅ No regressions in existing tests

---

## Risk Assessment

### High Risk

1. **Zero-copy complexity**: Arrow array ownership vs GDS PropertyValues lifetime
   - **Mitigation**: Start with copy paths, optimize later
2. **GraphStore builder API**: May need modifications for batch import
   - **Mitigation**: Review GraphStore API in Phase 5, propose changes if needed

### Medium Risk

1. **Arrow crate API churn**: Arrow-rs is evolving
   - **Mitigation**: Pin to stable version, document upgrade path
2. **Parallel task coordination**: Complex error handling across threads
   - **Mitigation**: Use proven patterns (Rayon, channels), extensive testing

### Low Risk

1. **Type conversion**: Arrow types → GDS types
   - **Mitigation**: Leverage existing value_type_table! system
2. **Property mapping**: Complex but well-defined
   - **Mitigation**: Reuse existing PropertyMappings, add Arrow-specific layer

---

## Dependencies & Prerequisites

### External Crates

```toml
[dependencies]
arrow = "53"              # Arrow columnar format
parquet = "53"            # Parquet file format (optional)
rayon = "1.7"             # Parallel iteration
crossbeam = "0.8"         # Channels for task coordination
```

### Internal Dependencies

- ✅ `projection/traits/` - ElementProjection, PropertyMapping
- ✅ `projection/impls/` - PropertyMappings
- ✅ `projection/codegen/` - value_type_table!, functors
- ✅ `types/` - ValueType, DefaultValue, PropertyValues
- ✅ GraphStore - node/edge builders

### Prerequisites

- LinkPipeline complete (25/25 files) ✅
- PropertyMappings API stabilized (current task)
- GraphStore builder API reviewed

---

## Post-Translation Work

### Polars Integration (Future)

```rust
// projection/factory/polars/
// Polars DataFrames → Arrow → GraphStore
pub struct PolarsNativeFactory {
    node_dataframes: Vec<DataFrame>,
    edge_dataframes: Vec<DataFrame>,
}

impl PolarsNativeFactory {
    pub fn build_graph_store(&self) -> Result<GraphStore> {
        // Convert Polars DataFrames to Arrow tables
        let arrow_tables = self.to_arrow_tables();
        // Delegate to ArrowNativeFactory
        ArrowNativeFactory::from_tables(arrow_tables, vec![])
            .build_graph_store()
    }
}
```

### Neo4j Integration (Future)

```rust
// projection/factory/neo4j/
// Neo4j native projection for compatibility
pub struct Neo4jNativeFactory {
    transaction: Transaction,
    config: Neo4jProjectionConfig,
}

impl Neo4jNativeFactory {
    pub fn build_graph_store(&self) -> Result<GraphStore> {
        // Use Neo4j cursors (like Java GDS)
        // OR: Neo4j → Arrow → GraphStore
    }
}
```

### Zero-Copy Optimization (Future)

```rust
// Investigate Arrow arrays as backing storage for PropertyValues
pub struct ArrowBackedPropertyValues {
    arrow_array: Arc<dyn Array>,
    // Zero-copy wrapper!
}

impl PropertyValues for ArrowBackedPropertyValues {
    fn get(&self, index: usize) -> PropertyValue {
        // Direct array access, no copy
    }
}
```

---

## Documentation Plan

### Module-level Docs

````rust
//! Arrow-native graph projection factory.
//!
//! This module provides the "Lock and Load" interface to Arrow tables,
//! enabling zero-copy (where possible) import of graph data into GraphStore.
//!
//! # Quick Start
//!
//! ```rust
//! use rust_gds::projection::factory::arrow::ArrowNativeFactory;
//!
//! let node_table = /* Arrow table with nodes */;
//! let edge_table = /* Arrow table with edges */;
//!
//! let factory = ArrowNativeFactory::from_tables(
//!     vec![node_table],
//!     vec![edge_table],
//! );
//!
//! let graph_store = factory.build_graph_store()?;
//! ```
//!
//! # Architecture
//!
//! Arrow RecordBatches → BatchScanner → ImportTasks → GraphStore
//!
//! # Zero-Copy Optimization
//!
//! When Arrow column types match GDS PropertyValues types, data is
//! wrapped directly (zero-copy). Otherwise, type conversion is applied.
````

### Examples

```rust
// examples/arrow_import.rs
// Demonstrate Arrow → GraphStore import

// examples/polars_import.rs
// Demonstrate Polars → Arrow → GraphStore

// examples/zero_copy_benchmark.rs
// Compare zero-copy vs copy performance
```

---

## Timeline & Effort Estimate

| Phase | Description                | Effort | Cumulative |
| ----- | -------------------------- | ------ | ---------- |
| 1     | Core Infrastructure        | 4-5h   | 4-5h       |
| 2     | Reference System           | 3-4h   | 7-9h       |
| 3     | Scanner System             | 5-6h   | 12-15h     |
| 4     | Task System                | 4-5h   | 16-20h     |
| 5     | Importer System            | 6-7h   | 22-27h     |
| 6     | Property Mapping           | 4-5h   | 26-32h     |
| 7     | Consumer System            | 3-4h   | 29-36h     |
| 8     | Integration & Optimization | 4-5h   | 33-41h     |

**Total Estimated Effort**: 24-30 hours (optimistic), 33-41 hours (with buffer)

**Suggested Schedule** (assuming 4-hour work sessions):

- Week 1: Phases 1-2 (infrastructure + references)
- Week 2: Phases 3-4 (scanners + tasks)
- Week 3: Phases 5-6 (importers + properties)
- Week 4: Phases 7-8 (consumers + integration)

---

## Completion Report Template

Upon completion, create: `doc/translation/TC-004_NATIVE_PROJECTION_COMPLETION.md`

**Template**:

```markdown
# Translation Completion TC-004: Native Projection → Arrow Factory

**Completion Date**: [Date]
**Total Effort**: [Hours]
**Files Created**: [Count]
**Tests Added**: [Count]
**Lines of Code**: [Rust LOC]

## Summary

[What was accomplished]

## Deviations from Plan

[Any changes from TP-004]

## Performance Results

[Benchmark results, zero-copy gains]

## Known Issues / Future Work

[TODOs, Bija seeds]

## Lessons Learned

[What we learned]
```

---

## Related Documents

- **Design**: `doc/architecture/NATIVE_PROJECTION_ARROW_DESIGN.md`
- **ADR**: `doc/adr/adr0007_translation_plan_protocol.md`
- **Parent Plan**: `doc/translation/TP-002_GRAPH_PROJECTION_API.md`
- **Template**: `doc/translation/TRANSLATION_WORKFLOW_TEMPLATE.md`
- **LinkPipeline**: `doc/translation/TC-003_LINK_PIPELINE_COMPLETION.md`

---

## Status

**Translation Plan State**: ✅ Prakasa Complete (Ready for Kriya)  
**Approval Status**: 🔄 Awaiting approval  
**Priority**: 🎯 CRITICAL - "The Absolute Form's Kernel"  
**Blocking**: Polars integration, high-performance graph loading

---

## Final Notes

**Why this is "The Absolute Form's Kernel"**:

> "The Projector is our Absolute Form and is what we do for a living. It is getting Very Complex, it is that!"

**NativeFactory = The Entry Point**:

- ALL external data enters GraphStore through factories
- Defines what "native" means for rust-gds
- Performance bottleneck (must be fast!)
- Extensible to multiple data sources

**Arrow IS Native**:

- Arrow is already columnar (like PropertyValues!)
- Zero-copy opportunity (unique to rust-gds)
- Industry standard (Polars, DuckDB, DataFusion)
- Future-proof (Arrow ecosystem growing)

**This translation establishes**:

1. The data ingestion architecture for rust-gds
2. The pattern for adding new native sources
3. The zero-copy optimization pathway
4. The foundation for Polars/Arrow integration

**"Factory is really a Top-Level idea for AI entry points"** ✅

---

_Tat Tvam Asi_ - This IS That (Arrow IS Native!)  
🚀📊✨ **Ready for Kriya (Action)!**
