# Arrow Factory Architecture (rust-gds)

**Created**: 2025-01-XX  
**Status**: Phase 5 Complete (5/8 phases, 68 tests passing)

## Big Picture: NO CSR, YES RAMDisk

The Arrow Native Factory builds **DefaultGraphStore** directly from Arrow tables.

- **NOT like Java GDS**: No `CSRGraphStoreFactory`, no compressed sparse row format
- **Simple & Direct**: Arrow → Accumulator → SimpleIdMap + RelationshipTopology → DefaultGraphStore
- **"RAMDisk" = DefaultGraphStore**: In-memory HashMap (ID mapping) + Vec (adjacency lists)

## Architecture Comparison

### Java GDS (Heavy)

```
NativeFactory
  └─> CSRGraphStoreFactory  (compressed sparse row)
       └─> IdMapBuilder + RelationshipsBuilder  (streaming incremental)
            └─> CSRGraphStore  (compressed format)
```

### rust-gds (Light)

```
ArrowNativeFactory
  └─> NodeAccumulator + EdgeAccumulator  (Vec + HashMap)
       └─> build_id_map() + build_topology()
            └─> DefaultGraphStore::new(SimpleIdMap, RelationshipTopology)
```

## Core Components

### 1. DefaultGraphStore (the "RAMDisk")

**File**: `src/types/graph/store/default_store.rs`

```rust
pub struct DefaultGraphStore {
    graph_name: String,
    database_info: DatabaseInfo,
    schema: GraphSchema,
    capabilities: GraphCapabilities,
    id_map: SimpleIdMap,  // HashMap: original_id ↔ mapped_id
    topologies: HashMap<RelationshipType, RelationshipTopology>,  // Vec-based adjacency lists
    node_properties: HashMap<PropertyKey, Box<dyn PropertyValues>>,
    relationship_properties: HashMap<PropertyKey, Box<dyn PropertyValues>>,
}
```

**Construction**:

```rust
let store = DefaultGraphStore::new(
    graph_name,
    database_info,
    schema,
    capabilities,
    id_map,       // SimpleIdMap - from NodeAccumulator
    topologies,   // HashMap<RelType, Topology> - from EdgeAccumulator
);
```

### 2. SimpleIdMap (ID Mapping)

**File**: `src/types/graph/id_map/simple_id_map.rs`

```rust
pub struct SimpleIdMap {
    original_to_mapped: HashMap<i64, MappedNodeId>,  // User ID → Internal ID
    mapped_to_original: Vec<i64>,                     // Internal ID → User ID
}
```

**Used for**:

- Mapping external IDs (from Arrow) to internal dense IDs (0..node_count)
- Bidirectional lookup: `safe_to_mapped_node_id()`, `to_original_node_id()`

### 3. RelationshipTopology (Adjacency Lists)

**File**: `src/types/graph/topology/relationship_topology.rs`

```rust
pub struct RelationshipTopology {
    outgoing: Vec<Vec<MappedNodeId>>,  // outgoing[node] = [target1, target2, ...]
    incoming: Option<Vec<Vec<MappedNodeId>>>,  // Optional reverse direction
}
```

**Used for**:

- Fast neighbor iteration
- Graph traversal
- Algorithm execution (PageRank, BFS, etc.)

## Arrow Factory Flow (Phase 1-5 Complete)

### Phase 2: ArrowReference

```
Arrow2 RecordBatch
  ↓
NodeTableReference / EdgeTableReference
  ↓
ArrowBatchReference (with start_offset, end_offset)
```

### Phase 3: Scanner

```
BatchScanner trait
  ↓
NodeBatchScanner / EdgeBatchScanner
  ↓
Atomic batch reservation (thread-safe iteration)
```

### Phase 4: Task System

```
ImportTask trait
  ↓
TaskRunner (Rayon parallel)
  ↓
Progress tracking, error aggregation
```

### Phase 5: Importer (CURRENT - COMPLETE)

```
NodeImportTask                         EdgeImportTask
  ↓                                      ↓
NodeAccumulator                        EdgeAccumulator
  Vec<i64> original_ids                  Vec<(i64, i64, RelType)> edges
  HashMap<i64, Vec<NodeLabel>> labels
  ↓                                      ↓
build_id_map()                         build_topology(&id_map)
  ↓                                      ↓
SimpleIdMap                            HashMap<RelType, RelationshipTopology>
  ────────────────────────────────────────────────────────
                    ↓
        DefaultGraphStore::new(id_map, topologies)
                    ↓
          DefaultGraphStore (in-memory "RAMDisk")
```

## GAMMA Strategy (Phase 5)

**Decision**: Simple accumulation, defer builder infrastructure

**Rationale**:

- Java GDS has `IdMapBuilder` + `RelationshipsBuilder` (12-15 hours to translate)
- rust-gds doesn't need streaming for MVP (focus on correctness first)
- Simple `Vec` + `HashMap` accumulation is fast enough for medium graphs

**Trade-offs**:

- ✅ Fast implementation (4 hours vs 15 hours)
- ✅ Easy to understand and debug
- ✅ Sufficient for graphs up to ~10M nodes
- ⚠️ Higher memory usage (stores all data before building)
- ⚠️ Cannot stream (must fit in memory)

**Future**: Can add incremental builders later if needed (Phase 9+)

## Key Differences from Java GDS

| Feature    | Java GDS                           | rust-gds                         |
| ---------- | ---------------------------------- | -------------------------------- |
| Storage    | CSR (compressed)                   | Adjacency lists (simple)         |
| Factory    | CSRGraphStoreFactory               | DefaultGraphStore::new()         |
| Builders   | IdMapBuilder, RelationshipsBuilder | NodeAccumulator, EdgeAccumulator |
| Strategy   | Streaming incremental              | Batch accumulation               |
| Memory     | Low (compression)                  | Medium (uncompressed)            |
| Complexity | High                               | Low                              |

## Phase 5 Implementation Details

**File**: `src/projection/factory/arrow/importer.rs` (~627 lines)

### NodeAccumulator

```rust
pub struct NodeAccumulator {
    original_ids: Vec<i64>,
    labels_by_node: HashMap<i64, Vec<NodeLabel>>,
}

impl NodeAccumulator {
    pub fn add_node(&mut self, original_id: i64, labels: Vec<NodeLabel>);
    pub fn build_id_map(self) -> SimpleIdMap;
}
```

**Usage**:

```rust
let mut acc = NodeAccumulator::new();
// Parallel tasks call:
acc.add_node(123, vec![NodeLabel::of("Person")]);
acc.add_node(456, vec![NodeLabel::of("City")]);
// After all tasks complete:
let id_map = acc.build_id_map();  // Builds SimpleIdMap
```

### EdgeAccumulator

```rust
pub struct EdgeAccumulator {
    edges: Vec<(i64, i64, RelationshipType)>,
}

impl EdgeAccumulator {
    pub fn add_edge(&mut self, source: i64, target: i64, rel_type: RelationshipType);
    pub fn build_topology(&self, id_map: &SimpleIdMap) -> Result<HashMap<RelType, RelationshipTopology>>;
}
```

**Usage**:

```rust
let mut acc = EdgeAccumulator::new();
// Parallel tasks call:
acc.add_edge(123, 456, RelationshipType::of("KNOWS"));
acc.add_edge(456, 789, RelationshipType::of("LIVES_IN"));
// After all tasks complete:
let topologies = acc.build_topology(&id_map)?;  // Maps IDs, builds adjacency lists
```

### ImportTask Implementations

**NodeImportTask**:

```rust
impl ImportTask for NodeImportTask {
    fn execute(&mut self) -> Result<ImportResult> {
        while let Some(batch_ref) = self.scanner.next_batch()? {
            let (ids, labels) = process_node_batch(&batch_ref)?;
            let mut acc = self.accumulator.lock().unwrap();
            for (id, lbls) in ids.into_iter().zip(labels) {
                acc.add_node(id, lbls);
            }
        }
        Ok(ImportResult::new())
    }
}
```

**EdgeImportTask**:

```rust
impl ImportTask for EdgeImportTask {
    fn execute(&mut self) -> Result<ImportResult> {
        while let Some(batch_ref) = self.scanner.next_batch()? {
            let (sources, targets, types) = process_edge_batch(&batch_ref)?;
            let mut acc = self.accumulator.lock().unwrap();
            for ((src, tgt), typ) in sources.zip(targets).zip(types) {
                acc.add_edge(src, tgt, typ);
            }
        }
        Ok(ImportResult::new())
    }
}
```

## Thread Safety (Arc<Mutex<Accumulator>>)

**Problem**: Multiple parallel tasks need to write to shared accumulator

**Solution**: Wrap in `Arc<Mutex<_>>`

```rust
let node_accumulator = Arc::new(Mutex::new(NodeAccumulator::new()));
let edge_accumulator = Arc::new(Mutex::new(EdgeAccumulator::new()));

// Create tasks with cloned Arc:
let tasks: Vec<_> = (0..num_threads)
    .map(|i| {
        let acc = Arc::clone(&node_accumulator);
        let scanner = node_scanner.clone();
        NodeImportTask::new(scanner, acc)
    })
    .collect();

// Run in parallel:
task_runner.run(tasks)?;

// Extract final result:
let accumulator = Arc::try_unwrap(node_accumulator)?.into_inner()?;
let id_map = accumulator.build_id_map();
```

## Two-Pass Requirement

**Why two passes?**

1. **Pass 1 - Nodes**: Build IdMap

   - Scan all node batches
   - Accumulate original_ids + labels
   - Build `SimpleIdMap` (original → mapped)

2. **Pass 2 - Edges**: Build Topology (requires IdMap)
   - Scan all edge batches
   - Accumulate (source, target, type) tuples
   - Map original IDs → mapped IDs using IdMap
   - Build adjacency lists grouped by RelationshipType

**Cannot be done in one pass**: Edges reference node IDs that may not be seen yet

## Test Coverage (68 tests passing)

### Phase 1: Core Infrastructure (9 tests)

- Config validation
- Factory creation
- Error handling

### Phase 2: Reference System (25 tests)

- Schema inference
- Batch slicing (with start_offset, end_offset)
- Type conversion

### Phase 3: Scanner System (17 tests)

- Atomic batch reservation
- Thread-safe iteration
- Progress tracking

### Phase 4: Task System (8 tests)

- Parallel execution
- Error aggregation
- Progress reporting

### Phase 5: Importer (9 tests)

- `test_node_accumulator_empty`
- `test_node_accumulator_add_node`
- `test_node_accumulator_build_id_map`
- `test_edge_accumulator_empty`
- `test_edge_accumulator_add_edge`
- `test_edge_accumulator_build_topology`
- `test_edge_accumulator_invalid_node_id`
- `test_node_import_task_factory`
- `test_edge_import_task_factory`

## Next Steps (Phases 6-8)

### Phase 6: Property Mapping

- Extend NodeAccumulator to handle property columns
- Extend EdgeAccumulator to handle property columns
- Arrow column type → PropertyValues conversion
- Default value handling
- **Estimated**: 4-5 hours

### Phase 7: Consumer System

- Optional buffering layer
- Batch aggregation optimization
- May defer to future work
- **Estimated**: 3-4 hours

### Phase 8: Integration & Testing

- End-to-end tests: Arrow → GraphStore → Graph → Algorithm
- Performance benchmarks
- Documentation
- Example usage
- **Estimated**: 4-5 hours

## Example Usage (Future - Phase 8)

```rust
use rust_gds::projection::factory::arrow::{ArrowFactoryConfig, ArrowNativeFactory};

// Read Arrow tables (Parquet, IPC, CSV, etc.)
let node_table = read_parquet("nodes.parquet")?;
let edge_table = read_parquet("edges.parquet")?;

// Configure factory
let config = ArrowFactoryConfig::builder()
    .graph_name("my_graph")
    .node_table(node_table)
    .edge_table(edge_table)
    .build()?;

// Build GraphStore
let factory = ArrowNativeFactory::new(config);
let graph_store = factory.build()?;  // Returns DefaultGraphStore

// Use with algorithms
let graph = Graph::from_store(graph_store);
let pagerank = PageRank::new().run(&graph)?;
```

## Summary

- ✅ **Phase 5 Complete**: NodeAccumulator + EdgeAccumulator → DefaultGraphStore
- ✅ **68 tests passing**: All phases validated
- ✅ **Architecture clarified**: No CSR, direct RAMDisk construction
- ✅ **GAMMA strategy**: Simple, fast, correct
- 🎯 **Next**: Phase 6 Property Mapping (extend accumulators for properties)

**The Arrow Factory builds DefaultGraphStore directly - no CSR, no factory chaining, just simple in-memory RAMDisk construction.**
