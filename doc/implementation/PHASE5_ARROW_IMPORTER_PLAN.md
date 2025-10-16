# Phase 5: Direct Import Tasks (GAMMA Strategy)

**Status**: 🔥 In Progress  
**Strategy**: Direct accumulation → build, deferring incremental builder infrastructure  
**Target**: Phase 5/8 of TP-004

## Context

**Java GDS Pattern**:

- `ScanningNodesImporter` → `NodeImporterBuilder` → `IdMapBuilder` → build `IdMap`
- `ScanningRelationshipsImporter` → `RelationshipsBuilder` → build `RelationshipTopology`
- Incremental builders accumulate data during parallel scanning

**Rust-GDS Current State**:

- ✅ `IdMap` trait + `SimpleIdMap` (pre-built from arrays)
- ✅ `RelationshipTopology` (pre-built from adjacency lists)
- ✅ `GraphStore::new()` requires complete `IdMap` + `RelationshipTopology`
- ❌ **No incremental builder infrastructure** (IdMapBuilder, RelationshipsBuilder)
- ❌ **No batch accumulation APIs**

**Decision**: **Option A - GAMMA Approach**

- Create simple in-memory accumulators during import
- Build final structures after scanning completes
- Defer full Core/Loading builder infrastructure to future work
- **Trade-off**: Less optimal (memory overhead, no streaming), but works with current APIs

## Phase 5 Architecture

### High-Level Flow

```
Arrow Tables (Phase 2)
    ↓
BatchScanner (Phase 3)
    ↓
TaskRunner + ImportTask (Phase 4)
    ↓
NodeImportTask / EdgeImportTask (Phase 5) ← WE ARE HERE
    ↓
Accumulate in shared state (Arc<Mutex<Accumulator>>)
    ↓
Build IdMap + Topology
    ↓
Create GraphStore
```

### Key Components

1. **NodeAccumulator**: Thread-safe accumulator for node data

   - Stores: original_ids, labels_by_node, properties
   - Wrapped in `Arc<Mutex<_>>` for parallel writes
   - Method: `add_node(original_id, labels, properties)`

2. **EdgeAccumulator**: Thread-safe accumulator for edge data

   - Stores: source/target pairs, rel_types, properties
   - Wrapped in `Arc<Mutex<_>>` for parallel writes
   - Method: `add_edge(source, target, rel_type, properties)`

3. **NodeImportTask**: Concrete `ImportTask` for nodes

   - Reads batches via `ScanCursor`
   - Extracts node data from `ArrowBatchReference`
   - Writes to shared `NodeAccumulator`

4. **EdgeImportTask**: Concrete `ImportTask` for edges

   - Reads batches via `ScanCursor`
   - Extracts edge data from `ArrowBatchReference`
   - Writes to shared `EdgeAccumulator`

5. **NodeImportTaskFactory**: Creates `NodeImportTask` instances

   - Clones `Arc<NodeAccumulator>` for each task
   - Implements `TaskFactory` trait

6. **EdgeImportTaskFactory**: Creates `EdgeImportTask` instances
   - Clones `Arc<EdgeAccumulator>` for each task
   - Implements `TaskFactory` trait

## Implementation Plan

### File Structure

```
src/projection/factory/arrow/
├── mod.rs (update exports)
└── importer.rs (~600 lines) ← NEW
    ├── NodeAccumulator
    ├── EdgeAccumulator
    ├── NodeImportTask
    ├── EdgeImportTask
    ├── NodeImportTaskFactory
    ├── EdgeImportTaskFactory
    ├── ImporterError
    └── tests (8-10 module tests)
```

### Type Signatures

```rust
// Accumulator for node data
pub struct NodeAccumulator {
    original_ids: Vec<i64>,
    labels_by_node: HashMap<usize, HashSet<NodeLabel>>,
    // Property accumulation TBD (Phase 6)
}

impl NodeAccumulator {
    pub fn new() -> Self;
    pub fn add_node(&mut self, original_id: i64, labels: Vec<NodeLabel>);
    pub fn build_id_map(self) -> SimpleIdMap;
}

// Accumulator for edge data
pub struct EdgeAccumulator {
    edges: Vec<(i64, i64, RelationshipType)>, // (source_orig, target_orig, type)
    // Property accumulation TBD (Phase 6)
}

impl EdgeAccumulator {
    pub fn new() -> Self;
    pub fn add_edge(&mut self, source: i64, target: i64, rel_type: RelationshipType);
    pub fn build_topology(self, id_map: &SimpleIdMap) -> HashMap<RelationshipType, RelationshipTopology>;
}

// Import task for nodes
pub struct NodeImportTask {
    task_index: usize,
    accumulator: Arc<Mutex<NodeAccumulator>>,
}

impl ImportTask for NodeImportTask {
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError>;
    fn task_name(&self) -> String;
    fn task_index(&self) -> usize;
}

// Import task for edges
pub struct EdgeImportTask {
    task_index: usize,
    accumulator: Arc<Mutex<EdgeAccumulator>>,
    id_map: Arc<SimpleIdMap>, // For original → mapped lookup
}

impl ImportTask for EdgeImportTask;

// Task factories
pub struct NodeImportTaskFactory {
    accumulator: Arc<Mutex<NodeAccumulator>>,
}

impl TaskFactory for NodeImportTaskFactory {
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError>;
}

pub struct EdgeImportTaskFactory {
    accumulator: Arc<Mutex<EdgeAccumulator>>,
    id_map: Arc<SimpleIdMap>,
}

impl TaskFactory for EdgeImportTaskFactory;
```

### Arrow Column Extraction Pattern

```rust
// Inside NodeImportTask::execute()
while cursor.reserve_batch() {
    let mut batch_count = 0u64;

    cursor.consume_batch(&mut |batch: &ArrowBatchReference| {
        // Extract ID column
        let id_column = batch.int64_column(0).ok_or(...)?;

        // Extract label column (if present)
        let label_column = batch.utf8_column(1); // Optional

        // Iterate batch range
        for i in batch.start_offset()..batch.end_offset() {
            let original_id = id_column.value(i);
            let labels = if let Some(labels_col) = label_column {
                vec![NodeLabel::of(labels_col.value(i))]
            } else {
                vec![]
            };

            // Write to accumulator
            accumulator.lock().unwrap().add_node(original_id, labels);
            batch_count += 1;
        }

        true // Continue
    });
}
```

### Accumulator → Build Pattern

```rust
// After all tasks complete
let accumulator = Arc::try_unwrap(accumulator_arc)
    .map_err(|_| "accumulator still has references")?
    .into_inner()?;

// Build IdMap
let id_map = accumulator.build_id_map();

// Build topologies
let topologies = edge_accumulator.build_topology(&id_map);

// Create GraphStore
let store = DefaultGraphStore::new(
    graph_name,
    database_info,
    schema,
    capabilities,
    id_map,
    topologies,
);
```

## Testing Strategy

### Module Tests (importer.rs)

1. **NodeAccumulator**:

   - `test_node_accumulator_add_node`: Add nodes, verify count
   - `test_node_accumulator_build_id_map`: Build IdMap, verify mappings
   - `test_node_accumulator_with_labels`: Add labeled nodes

2. **EdgeAccumulator**:

   - `test_edge_accumulator_add_edge`: Add edges, verify count
   - `test_edge_accumulator_build_topology`: Build topology, verify adjacency
   - `test_edge_accumulator_multiple_types`: Multiple relationship types

3. **NodeImportTask**:

   - `test_node_import_task_execute`: Mock cursor, verify import

4. **EdgeImportTask**:
   - `test_edge_import_task_execute`: Mock cursor, verify import

### Integration Tests (test_phase5_arrow_importer.rs)

1. **End-to-End Node Import**:

   - Create node table (Phase 2)
   - Create scanner (Phase 3)
   - Create task factory (Phase 5)
   - Run TaskRunner (Phase 4)
   - Verify IdMap correctness

2. **End-to-End Edge Import**:

   - Create edge table (Phase 2)
   - Create scanner (Phase 3)
   - Require pre-built IdMap
   - Create task factory (Phase 5)
   - Run TaskRunner (Phase 4)
   - Verify topology correctness

3. **Parallel Import**:
   - Large dataset (10,000 nodes, 50,000 edges)
   - 4 parallel tasks
   - Verify no data loss
   - Verify correct ordering

## Known Limitations (Gamma Trade-offs)

### 1. Memory Overhead

**Issue**: All data accumulated in memory before building final structures

**Impact**: Cannot stream large imports, requires 2x memory (accumulator + final structures)

**Mitigation**: Document memory requirements, add size warnings

**Future**: Implement streaming builders in Core/Loading package

### 2. No Incremental ID Mapping

**Issue**: Must collect all nodes before creating IdMap

**Impact**: Edge import requires complete IdMap upfront (two-pass required)

**Mitigation**: Document two-pass requirement in factory orchestration

**Future**: Implement incremental IdMapBuilder with dynamic assignment

### 3. Lock Contention

**Issue**: Parallel tasks share `Mutex<Accumulator>`

**Impact**: Lock contention may reduce parallel efficiency for small batches

**Mitigation**: Use batch-level locking (lock once per batch, not per record)

**Future**: Implement lock-free concurrent accumulators (crossbeam, dashmap)

### 4. No Property Support Yet

**Issue**: Phase 5 focuses on topology, properties deferred to Phase 6

**Impact**: Cannot import node/edge properties yet

**Mitigation**: Clearly document as Phase 6 work

**Future**: Extend accumulators with property columns (Phase 6)

### 5. No Validation

**Issue**: No validation of source/target IDs, dangling relationships possible

**Impact**: Invalid edges may cause runtime panics

**Mitigation**: Add basic ID validation, skip invalid edges with warning

**Future**: Add comprehensive validation in Core/Loading builders

## Integration with Phases 1-4

### Using NodeImportTask

```rust
// Create accumulator
let node_accumulator = Arc::new(Mutex::new(NodeAccumulator::new()));

// Create scanner (Phase 3)
let node_table = NodeTableReference::new(...)?; // Phase 2
let scanner = Arc::new(NodeBatchScanner::new(node_table, 10_000)?);

// Create factory (Phase 5)
let factory = Arc::new(NodeImportTaskFactory::new(node_accumulator.clone()));

// Run import (Phase 4)
let runner = TaskRunner::new(4)?;
let result = runner.run_import(scanner, factory)?;

println!("Imported {} nodes", result.total_records_imported);

// Build IdMap
let accumulator = Arc::try_unwrap(node_accumulator)?.into_inner()?;
let id_map = accumulator.build_id_map();
```

### Using EdgeImportTask

```rust
// Requires pre-built IdMap from node import
let id_map = Arc::new(id_map);

// Create accumulator
let edge_accumulator = Arc::new(Mutex::new(EdgeAccumulator::new()));

// Create scanner (Phase 3)
let edge_table = EdgeTableReference::new(...)?; // Phase 2
let scanner = Arc::new(EdgeBatchScanner::new(edge_table, 10_000)?);

// Create factory (Phase 5)
let factory = Arc::new(EdgeImportTaskFactory::new(
    edge_accumulator.clone(),
    id_map.clone(),
));

// Run import (Phase 4)
let runner = TaskRunner::new(4)?;
let result = runner.run_import(scanner, factory)?;

println!("Imported {} edges", result.total_records_imported);

// Build topologies
let accumulator = Arc::try_unwrap(edge_accumulator)?.into_inner()?;
let topologies = accumulator.build_topology(&id_map);

// Create GraphStore
let store = DefaultGraphStore::new(
    graph_name,
    database_info,
    schema,
    capabilities,
    (*id_map).clone(),
    topologies,
);
```

## Success Criteria

- ✅ `NodeAccumulator` implemented and tested
- ✅ `EdgeAccumulator` implemented and tested
- ✅ `NodeImportTask` implements `ImportTask` trait
- ✅ `EdgeImportTask` implements `ImportTask` trait
- ✅ Task factories implement `TaskFactory` trait
- ✅ 8-10 module tests passing
- ✅ 3-5 integration tests passing
- ✅ End-to-end node import working
- ✅ End-to-end edge import working (with pre-built IdMap)
- ✅ Build succeeds with zero errors
- ✅ Documentation complete

## Next Steps (Phase 6)

After Phase 5 completion:

1. **Property Mapping**: Extend accumulators to handle node/edge properties
2. **Arrow → PropertyValues**: Zero-copy property extraction from Arrow columns
3. **Type Conversion**: Handle Arrow types → GraphStore value types
4. **Default Values**: Support missing/null values with defaults

## Future Work (Beyond TP-004)

**Core/Loading Builder Infrastructure** (separate task):

- Implement incremental `IdMapBuilder` (streaming node ID assignment)
- Implement incremental `RelationshipsBuilder` (streaming edge accumulation)
- Implement `NodeImporterBuilder` (coordinates ID mapping + properties)
- Implement `LabelInformation.Builder` (tracks label assignments)
- Integrate with Arrow Factory for single-pass, streaming imports
- Eliminate memory overhead of two-pass approach

**Estimated Effort**: 15-20 hours for complete Core/Loading translation

---

**Phase 5 Estimated Time**: 4-5 hours  
**Current Status**: Planning complete, ready to implement 🚀
