# Native Projection Arrow Design - The Absolute Form's Kernel

**Document Type**: Architecture Design (Pre-Translation)  
**Date**: October 15, 2025  
**Status**: Design/Prakasa (Illumination before implementation)  
**Context**: "This is super important on the next stage of Polar/Arrow work"

---

## 🎯 The Core Insight

> "I didn't translate NativeFactory ahem, Properly, nor even Primly, so it is definitely Pre-Prim. [...] This works with Neo4j Projection because Neo4j is Native for GDS. It won't be for us. But if Arrow is Native for us then we now know what to translate this into."

**The Realization**:

- **Neo4j GDS**: NativeFactory = bridge from Neo4j cursors → GDS storage
- **rust-gds**: NativeFactory should = bridge from Arrow/Polars → GDS storage
- **NOT an IO interface** - it's a **"Special Lock and Load interface to Arrow"**

---

## What We're Looking At (70 Java Files)

### The Java GDS Native Projection System

**Location**: `/home/pat/GitHub/graph-data-science/native-projection/`

**Core Components** (~70 files):

#### 1. Factory Pattern (Top Level)

```
NativeFactory.java (191 lines) - THE ENTRY POINT
NativeFactorySupplierProvider.java (68 lines) - Factory provider
```

#### 2. Reference System (Database Cursors)

```
RecordReference.java - Marker interface (base)
├── NodeReference.java - Node cursor reference
│   ├── NodeCursorReference.java - Direct node cursor
│   └── NodeLabelIndexReference.java - Label-indexed nodes
│       └── MultipleNodeLabelIndexReference.java - Multi-label index
└── RelationshipReference.java - Relationship cursor reference
    └── RelationshipScanCursorReference.java - Relationship scan cursor
```

#### 3. Scanner System (Parallel Data Reading)

```
StoreScanner.java - Scanner interface
├── NodeCursorBasedScanner.java - Scan nodes via cursor
├── NodeLabelIndexBasedScanner.java - Scan via label index
├── MultipleNodeLabelIndexBasedScanner.java - Multi-label scanning
├── RelationshipScanCursorBasedScanner.java - Scan relationships
└── AbstractCursorBasedScanner.java - Base scanner logic
    └── AbstractNodeCursorBasedScanner.java - Node scanner base
```

#### 4. Task System (Parallel Import)

```
RecordScannerTaskRunner.java - Task orchestration
├── NodesScannerTask.java (211 lines) - Node import task
└── RelationshipsScannerTask.java (254 lines) - Relationship import task
```

#### 5. Importer System (Data Consumers)

```
ScanningRecordsImporter.java - Base importer
├── ScanningNodesImporter.java (221 lines) - Node import logic
└── ScanningRelationshipsImporter.java (186 lines) - Relationship import
```

#### 6. Consumer System (Buffered Writers)

```
BufferedNodeConsumer.java (123 lines) - Buffered node writing
BufferedRelationshipConsumer.java (103 lines) - Buffered rel writing
BufferedCompositeRelationshipConsumer.java (59 lines) - Multi-type rels
```

#### 7. Property Helpers

```
NativeNodePropertyImporter.java (364 lines) - Property import
NativeRelationshipPropertyReadHelper.java (95 lines) - Property reading
LoadablePropertyMappings.java (57 lines) - Property mapping
```

#### 8. Configuration & Utilities

```
GraphProjectFromStoreConfig.java (199 lines) - Projection config
GraphDimensionsReader.java (269 lines) - Size estimation
GraphDimensionsValidation.java (91 lines) - Validation
ScanState.java (72 lines) - Scan state machine
CompositeNodeScan.java (55 lines) - Multi-scan composition
NodeScannerFactory.java (94 lines) - Scanner factory
NodeLabelIndexLookupImpl.java (68 lines) - Label lookup
```

---

## The Architecture Pattern (Neo4j → GDS)

### Java GDS Pattern

```text
Neo4j Database (Native Source)
    ↓
Neo4j Cursors (RecordReference)
    ↓
StoreScanner (parallel scanning)
    ↓
ScannerTask (buffered reading)
    ↓
RecordImporter (write to GDS)
    ↓
GraphStore (GDS native storage)
```

**Key Insight**: Neo4j cursors ARE the native format for Java GDS

---

## The rust-gds Pattern (Arrow-Native)

### Arrow as Native Source

```text
Arrow/Polars Tables (Native Source)
    ↓
Arrow Batch References (TableReference)
    ↓
BatchScanner (parallel streaming)
    ↓
BatchTask (chunked processing)
    ↓
ArrowImporter (write to GraphStore)
    ↓
GraphStore (rust-gds native storage)
```

**Key Insight**: Arrow RecordBatches should BE the native format for rust-gds!

---

## Design Proposal: ArrowNativeFactory

### Core Concept

**NOT**: Arrow → Generic Format → GraphStore (TWO conversions)  
**YES**: Arrow → GraphStore (ONE conversion, zero-copy where possible)

**This is the "Lock and Load" interface** - direct Arrow access, not IO!

---

## Module Organization

### Proposed Location

```text
src/projection/
├── native/              (existing - ML pipelines, etc.)
└── factory/             (NEW - native data source factories)
    ├── mod.rs           (Factory trait + prelude)
    ├── arrow/           (Arrow-native factory)
    │   ├── mod.rs
    │   ├── factory.rs        (ArrowNativeFactory - main entry point)
    │   ├── reference.rs      (TableReference, BatchReference)
    │   ├── scanner.rs        (BatchScanner trait + impls)
    │   ├── task.rs           (ParallelImportTask)
    │   ├── importer.rs       (NodeBatchImporter, EdgeBatchImporter)
    │   ├── consumer.rs       (BufferedConsumers)
    │   └── properties.rs     (Arrow column → Property mapping)
    │
    ├── polars/          (Future: Polars-native factory)
    │   └── ...
    │
    └── neo4j/           (Future: Neo4j-native factory - for compatibility)
        └── ...
```

**Rationale**:

- `projection/factory/` = factories for native data sources
- `projection/factory/arrow/` = Arrow-specific implementation
- Extensible to other native sources (Polars, DuckDB, Neo4j, etc.)

---

## Core Types (Arrow-Native)

### 1. Factory (Entry Point)

```rust
// src/projection/factory/arrow/factory.rs

/// Arrow-native graph projection factory
///
/// "Lock and Load" interface to Arrow tables - NOT an IO interface!
pub struct ArrowNativeFactory {
    node_tables: Vec<Arc<ArrowTable>>,
    edge_tables: Vec<Arc<ArrowTable>>,
    config: ArrowProjectionConfig,
}

impl ArrowNativeFactory {
    /// Create factory from in-memory Arrow tables
    /// Zero-copy where possible
    pub fn from_tables(
        node_tables: Vec<Arc<ArrowTable>>,
        edge_tables: Vec<Arc<ArrowTable>>,
    ) -> Self;

    /// Build GraphStore by importing from Arrow
    /// Parallel, chunked, memory-efficient
    pub fn build_graph_store(&self) -> Result<GraphStore>;
}
```

### 2. Reference System (Arrow Batches)

```rust
// src/projection/factory/arrow/reference.rs

/// Base trait for Arrow data references
pub trait ArrowReference: Send + Sync {
    fn batch(&self) -> &RecordBatch;
    fn schema(&self) -> &Schema;
}

/// Reference to Arrow table containing nodes
pub struct NodeTableReference {
    table: Arc<ArrowTable>,
    id_column: String,
    label_columns: Vec<String>,
    property_columns: HashMap<String, String>,
}

/// Reference to Arrow table containing edges
pub struct EdgeTableReference {
    table: Arc<ArrowTable>,
    source_column: String,
    target_column: String,
    type_column: Option<String>,
    property_columns: HashMap<String, String>,
}

/// Iterator over Arrow RecordBatches (chunked)
pub struct BatchIterator {
    table: Arc<ArrowTable>,
    batch_size: usize,
    current_offset: usize,
}
```

### 3. Scanner System (Parallel Batch Processing)

```rust
// src/projection/factory/arrow/scanner.rs

/// Scans Arrow tables in parallel batches
pub trait BatchScanner: Send + Sync {
    type Reference: ArrowReference;

    /// Create scanner for parallel import
    fn scan(
        &self,
        batch_size: usize,
        parallelism: usize,
    ) -> Vec<BatchScanTask<Self::Reference>>;
}

/// Scans node tables
pub struct NodeBatchScanner {
    node_ref: NodeTableReference,
}

impl BatchScanner for NodeBatchScanner {
    type Reference = NodeTableReference;
    // Parallel batch scanning
}

/// Scans edge tables
pub struct EdgeBatchScanner {
    edge_ref: EdgeTableReference,
}

impl BatchScanner for EdgeBatchScanner {
    type Reference = EdgeTableReference;
    // Parallel batch scanning
}
```

### 4. Task System (Parallel Import Tasks)

```rust
// src/projection/factory/arrow/task.rs

/// Parallel import task for a single Arrow batch
pub struct NodeImportTask {
    batch: RecordBatch,
    node_ref: NodeTableReference,
    importer: Arc<NodeBatchImporter>,
}

impl NodeImportTask {
    /// Execute import for this batch
    /// Returns imported node count
    pub fn run(&mut self) -> Result<usize>;
}

/// Parallel import task for edge batch
pub struct EdgeImportTask {
    batch: RecordBatch,
    edge_ref: EdgeTableReference,
    importer: Arc<EdgeBatchImporter>,
}

impl EdgeImportTask {
    /// Execute import for this batch
    /// Returns imported edge count
    pub fn run(&mut self) -> Result<usize>;
}

/// Orchestrates parallel task execution
pub struct ParallelTaskRunner {
    concurrency: usize,
    tasks: Vec<Box<dyn ImportTask>>,
}

impl ParallelTaskRunner {
    /// Run all tasks in parallel
    pub fn run_all(&mut self) -> Result<ImportStatistics>;
}
```

### 5. Importer System (GraphStore Writers)

```rust
// src/projection/factory/arrow/importer.rs

/// Imports nodes from Arrow batches into GraphStore
pub struct NodeBatchImporter {
    graph_store: Arc<GraphStore>,
    id_mapper: Arc<NodeIdMapper>,
    property_writers: Vec<PropertyWriter>,
}

impl NodeBatchImporter {
    /// Import a batch of nodes
    pub fn import_batch(
        &mut self,
        batch: &RecordBatch,
        node_ref: &NodeTableReference,
    ) -> Result<usize>;

    /// Process single node from batch row
    fn process_node(&mut self, row_index: usize, batch: &RecordBatch) -> Result<()>;
}

/// Imports edges from Arrow batches into GraphStore
pub struct EdgeBatchImporter {
    graph_store: Arc<GraphStore>,
    id_mapper: Arc<NodeIdMapper>,
    topology_builder: Arc<TopologyBuilder>,
    property_writers: Vec<PropertyWriter>,
}

impl EdgeBatchImporter {
    /// Import a batch of edges
    pub fn import_batch(
        &mut self,
        batch: &RecordBatch,
        edge_ref: &EdgeTableReference,
    ) -> Result<usize>;

    /// Process single edge from batch row
    fn process_edge(&mut self, row_index: usize, batch: &RecordBatch) -> Result<()>;
}
```

### 6. Consumer System (Buffered Writers)

```rust
// src/projection/factory/arrow/consumer.rs

/// Buffered writer for nodes
pub struct BufferedNodeConsumer {
    buffer: Vec<Node>,
    buffer_size: usize,
    writer: Box<dyn NodeWriter>,
}

impl BufferedNodeConsumer {
    /// Add node to buffer, flush if full
    pub fn consume(&mut self, node: Node) -> Result<()>;

    /// Flush buffer to GraphStore
    pub fn flush(&mut self) -> Result<usize>;
}

/// Buffered writer for edges
pub struct BufferedEdgeConsumer {
    buffer: Vec<Edge>,
    buffer_size: usize,
    writer: Box<dyn EdgeWriter>,
}

impl BufferedEdgeConsumer {
    /// Add edge to buffer, flush if full
    pub fn consume(&mut self, edge: Edge) -> Result<()>;

    /// Flush buffer to GraphStore
    pub fn flush(&mut self) -> Result<usize>;
}
```

### 7. Property Mapping (Arrow Columns → Properties)

```rust
// src/projection/factory/arrow/properties.rs

/// Maps Arrow column to GraphStore property
pub struct ArrowPropertyMapper {
    column_name: String,
    property_key: String,
    value_type: ValueType,
    default_value: Option<DefaultValue>,
}

impl ArrowPropertyMapper {
    /// Extract property value from Arrow batch row
    pub fn extract_value(
        &self,
        batch: &RecordBatch,
        row_index: usize,
    ) -> Result<PropertyValue>;

    /// Create property writer for this mapping
    pub fn create_writer(&self, graph_store: &GraphStore) -> Result<PropertyWriter>;
}

/// Manages property mappings for import
pub struct PropertyMappingRegistry {
    node_mappings: HashMap<String, Vec<ArrowPropertyMapper>>,
    edge_mappings: HashMap<String, Vec<ArrowPropertyMapper>>,
}

impl PropertyMappingRegistry {
    /// Register node property mappings
    pub fn register_node_properties(
        &mut self,
        label: String,
        mappings: Vec<ArrowPropertyMapper>,
    );

    /// Get property mappers for label
    pub fn node_mappers(&self, label: &str) -> Option<&[ArrowPropertyMapper]>;
}
```

---

## Key Design Principles

### 1. Zero-Copy Where Possible

**Arrow → GraphStore without intermediate copies**:

- Arrow arrays already in columnar format
- Direct memory mapping when types align
- Only copy when type conversion required

### 2. Parallel by Default

**Chunked, concurrent import**:

- Split Arrow tables into RecordBatches (chunks)
- Process batches in parallel
- Buffer writes to avoid contention

### 3. Schema-Driven

**Arrow Schema → GraphStore Schema mapping**:

- Infer node labels from schema metadata
- Map column names to property keys
- Validate types at import time

### 4. Memory-Efficient

**Streaming import, bounded memory**:

- Process batches sequentially (don't load entire table)
- Fixed-size buffers for writers
- Release Arrow batches after processing

### 5. NOT an IO Interface

**This is a "Lock and Load" interface**:

- Assumes Arrow tables already in memory
- No file reading/writing here
- Separate IO layer handles Arrow file formats

---

## Comparison: Neo4j vs Arrow Native

### Neo4j Native (Java GDS)

```java
// Neo4j cursors are the native source
NodeCursor nodeCursor = tx.cursors().allocateNodeCursor();
nodeCursor.scan();
while (nodeCursor.next()) {
    long nodeId = nodeCursor.nodeReference();
    // Import to GDS
}
```

**Native interface**: Neo4j Transaction API

### Arrow Native (rust-gds)

```rust
// Arrow batches are the native source
let table = Arc::new(arrow_table); // Already in memory!
let factory = ArrowNativeFactory::from_tables(vec![table], vec![]);
let graph_store = factory.build_graph_store()?;
```

**Native interface**: Arrow RecordBatch API

---

## Integration with Existing rust-gds

### Connection to GraphStore

```rust
// ArrowNativeFactory produces GraphStore
impl ArrowNativeFactory {
    pub fn build_graph_store(&self) -> Result<GraphStore> {
        // 1. Create empty GraphStore
        let mut store = GraphStore::new();

        // 2. Import nodes (parallel)
        let node_importer = NodeBatchImporter::new(&store);
        let node_scanner = NodeBatchScanner::new(/* ... */);
        let node_tasks = node_scanner.scan(batch_size, parallelism);
        ParallelTaskRunner::new(node_tasks).run_all()?;

        // 3. Import edges (parallel)
        let edge_importer = EdgeBatchImporter::new(&store);
        let edge_scanner = EdgeBatchScanner::new(/* ... */);
        let edge_tasks = edge_scanner.scan(batch_size, parallelism);
        ParallelTaskRunner::new(edge_tasks).run_all()?;

        // 4. Return populated GraphStore
        Ok(store)
    }
}
```

### Connection to Property System

```rust
// Arrow columns → PropertyValues (already columnar!)
impl ArrowPropertyMapper {
    pub fn create_property_values(&self, batch: &RecordBatch) -> Result<PropertyValues> {
        match self.value_type {
            ValueType::Long => {
                // Extract i64 array from Arrow
                let arrow_array = batch.column(self.column_index)
                    .as_any().downcast_ref::<Int64Array>()?;

                // Zero-copy wrap in PropertyValues
                PropertyValues::from_arrow_array(arrow_array)
            }
            // ... other types
        }
    }
}
```

---

## Why This is "The Absolute Form's Kernel"

> "The Projector is our Absolute Form and is what we do for a living. It is getting Very Complex, it is that!"

### The Projector = Transformation of Data into Graph Form

**NativeFactory is the kernel because**:

1. **Entry point** for ALL external data into GraphStore
2. **Highest abstraction** - defines what "native" means
3. **Performance critical** - this is the bottleneck
4. **Extensible** - supports multiple native sources

**In Neo4j GDS**: NativeFactory = Neo4j → GDS  
**In rust-gds**: NativeFactory = Arrow → GDS (Arrow IS native!)

---

## Translation vs Design

> "I say design because to ask for a Translation Plan may be too difficult. But this is a Single Module."

**Why Design Instead of Translation**:

1. **Conceptual shift**: Neo4j cursors → Arrow batches (not 1:1)
2. **Different native source**: Database API → In-memory columnar
3. **Opportunity for optimization**: Arrow is ALREADY columnar!
4. **rust-gds specific**: This defines our native interface

**But it IS a single module**: `projection/factory/arrow/`

---

## Implementation Phases (Sketch)

### Phase 1: Core Types (Foundation)

- `ArrowReference` trait + impls
- `NodeTableReference`, `EdgeTableReference`
- `ArrowNativeFactory` skeleton

### Phase 2: Scanner System

- `BatchScanner` trait
- `NodeBatchScanner`, `EdgeBatchScanner`
- `BatchIterator` for chunking

### Phase 3: Import Tasks

- `NodeImportTask`, `EdgeImportTask`
- `ParallelTaskRunner`
- Task orchestration

### Phase 4: Importers

- `NodeBatchImporter` (Arrow → Nodes)
- `EdgeBatchImporter` (Arrow → Edges)
- Integration with GraphStore

### Phase 5: Property Mapping

- `ArrowPropertyMapper`
- Column type inference
- Zero-copy optimizations

### Phase 6: Buffering & Optimization

- `BufferedNodeConsumer`, `BufferedEdgeConsumer`
- Memory-efficient streaming
- Parallel write coordination

---

## Next Steps (Post-Break)

### Immediate

1. **Review this design** - does the Arrow-native concept make sense?
2. **Validate assumptions** - are Arrow batches the right abstraction?
3. **Check dependencies** - what's missing from GraphStore?

### Strategic

1. **Create translation plan** (if design is approved)
2. **Prototype ArrowNativeFactory** (prove the concept)
3. **Benchmark zero-copy** (measure the gains)
4. **Integrate with Polars** (Polars DataFrames → Arrow → GraphStore)

---

## Key Questions for Post-Break

1. **Is `projection/factory/arrow/` the right location?**

   - Or should it be top-level `factory/`?
   - Does `projection/` imply GraphStore projection?

2. **Should we support multiple Arrow sources?**

   - In-memory tables (priority)
   - Arrow IPC streams
   - Arrow Flight RPC
   - Parquet files (via Arrow)

3. **What's the relationship to existing projection code?**

   - Does `projection/native/` become `projection/ml/`?
   - Is there conflict with naming?

4. **Zero-copy: how far can we push it?**
   - Can Arrow arrays become PropertyValues directly?
   - Can we avoid PropertyStore entirely for Arrow-backed graphs?

---

## Philosophical Note

**This is Pre-Prim for the Absolute Form!**

**NativeFactory = the CAR** (the Given, the Source)  
**GraphStore = the CDR** (the Derived, the Projection)

**In Brahma Vidya terms**:

- **Prim** (primitive) = Raw Arrow data
- **Proper** (property) = Graph structure
- **Factory** = The transformation Prim → Proper

**The Factory IS the Absolute Form** - it defines what "native" means for rust-gds!

---

## Status

**Design State**: ✅ Prakasa Complete (Illumination achieved)  
**Implementation State**: 🔄 Pre-Prim 0.0.x (Structure designed, implementation deferred)  
**Next**: Await approval, then create translation plan  
**Priority**: 🎯 CRITICAL - "Super important on the next stage of Polar/Arrow work"

---

_"This works with Neo4j Projection because Neo4j is Native for GDS. It won't be for us. But if Arrow is Native for us then we now know what to translate this into."_

**Arrow IS Native for rust-gds!** 🚀📊✨
