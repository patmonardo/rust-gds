# GraphStore Loading - Quick Reference Guide

**Document Type**: Technical Specification (Quick Reference)  
**Date**: October 15, 2025  
**Status**: 📝 Notes for GAMMA Study  
**Source**: TypeScript translations from Java GDS core/loading/

---

## 🕉️ Membership Protocol

- **Location**: `doc/specifications/GRAPHSTORE_LOADING_QUICK_REFERENCE.md`
- **Category**: Specifications (Quick Reference)
- **Purpose**: Guide for understanding GraphStore loading architecture
- **Context**: Part of Projection (NOT IO!) - in-memory graph construction

---

## Key Insight: Projection vs IO

**This is Projection** (in-memory graph construction):

```
External Data (already loaded) → GraphStore (in-memory)
```

**This is NOT IO** (file reading/writing):

```
Files/DB → Memory (that's IO, different layer!)
```

**Java GDS Structure**:

- `core/loading/` = Projection (graph construction) ✅ This!
- `io/` = File/DB operations ❌ Not this!

---

## The Loading Pipeline (5 Stages)

### Stage 1: Batch Buffering

**Files**: `NodesBatchBuffer.ts`, `RelationshipsBatchBuffer.ts`

**Purpose**: Collect records into memory batches

**Key concepts**:

- Fixed-size buffers (100K records typical)
- Pre-allocated arrays for performance
- Property references stored alongside IDs
- Label tokens for multi-label nodes

**Node buffer**:

```
[nodeId₀, nodeId₁, ..., nodeIdₙ]
[labelTokens₀, labelTokens₁, ..., labelTokensₙ]
[propertyRef₀, propertyRef₁, ..., propertyRefₙ]
```

**Relationship buffer**:

```
[src₀, tgt₀, src₁, tgt₁, ..., srcₙ, tgtₙ]  (interleaved pairs)
[relationshipRef₀, ref₁, ..., refₙ]
[propertyRef₀, prop₁, ..., propₙ]
```

---

### Stage 2: Radix Sorting

**File**: `RelationshipsBatchBuffer.ts` (sortBySource/sortByTarget methods)

**Purpose**: Ultra-fast sorting for adjacency list construction

**Why radix sort?**: O(n) time complexity vs O(n log n) for comparison sorts

**Sort targets**:

- **By source** → outgoing adjacency lists (forward relationships)
- **By target** → incoming adjacency lists (reverse relationships)

**Working arrays** (pre-allocated):

```rust
buffer_copy: Vec<u64>              // Spare array for sorting
histogram: Vec<usize>              // Radix buckets
relationship_refs_copy: Vec<u64>  // Parallel sort
property_refs_copy: Vec<T>        // Parallel sort
```

---

### Stage 3: Pre-Aggregation

**File**: `AdjacencyPreAggregation.ts`

**Purpose**: Merge duplicate relationships by aggregating properties

**Problem**: Multiple edges between same node pair

```
A → B (weight: 5)
A → B (weight: 3)
```

**Solution**: Aggregate into single edge

```
A → B (weight: 8)  [if aggregation = SUM]
```

**Key mechanism**:

- Mark duplicates with `IGNORE_VALUE` (LONG_MIN)
- Aggregate properties into first occurrence
- Downstream processing filters IGNORE_VALUE

**Aggregations supported**:

- `NONE` - Keep all parallel edges
- `SUM`, `MAX`, `MIN`, `COUNT`, `SINGLE` - Merge strategy

---

### Stage 4: Adjacency Compression

**File**: `AdjacencyBuffer.ts`

**Purpose**: Compress sorted adjacency lists for memory efficiency

**Compression strategies** (feature toggles):

1. **Compressed** (default): Delta + VarLong encoding
2. **Uncompressed**: Raw arrays (debug/testing)
3. **Packed**: Bit-packing for small node IDs
4. **Mixed**: Adaptive (packed for dense, compressed for sparse)

**Key insight**: Node IDs are often sequential or clustered

```
Targets: [100, 101, 105, 109]
Deltas:  [100, +1, +4, +4]  (smaller numbers!)
VarLong: Encode small numbers in fewer bytes
```

**Memory savings**: 2-10x compression typical

**Structure**:

```
ChunkedAdjacencyLists[]  (one per page)
  ├─ Page 0: nodes [0..pageSize)
  ├─ Page 1: nodes [pageSize..2*pageSize)
  └─ Page N: nodes [N*pageSize..)
```

---

### Stage 5: Final Graph Construction

**Files**: `Nodes.ts`, `SingleTypeRelationships.ts`

**Purpose**: Assemble final immutable GraphStore

**Components**:

**Nodes**:

```rust
struct Nodes {
    id_map: IdMap,                        // Original ID → Internal ID
    schema: NodeSchema,                    // Labels + properties metadata
    property_store: NodePropertyStore,     // Actual property values
}
```

**Relationships** (per type):

```rust
struct SingleTypeRelationships {
    topology: Topology,                    // Adjacency lists (compressed)
    properties: RelationshipPropertyStore, // Property values (compressed)
    schema_entry: RelationshipSchemaEntry, // Type metadata
}
```

---

## Paging System

**File**: `AdjacencyBufferPaging.ts`

**Purpose**: Distribute nodes across pages for parallel processing

**Interface**:

```typescript
pageId(source: number): number      // Which page?
localId(source: number): number     // Position within page?
sourceNodeId(localId, pageId): number  // Reconstruct global ID
```

**Why paging?**:

- Parallel compression (one page per thread)
- Memory locality (process page-at-a-time)
- Scalability (handle graphs larger than RAM)

**Typical page size**: 128K-1M nodes per page

---

## Task System

**File**: `RecordScannerTask.ts`

**Purpose**: Parallel execution with progress tracking

**Pattern**:

```typescript
interface RecordScannerTask {
  run(): void;
  recordsImported(): number;
  propertiesImported(): number;
}
```

**Usage**:

- One task per thread
- Each task processes a chunk/page
- Aggregate statistics at end

---

## Hooks for Extension

**Files**: `PostLoadETLHook.ts`, `PostLoadValidationHook.ts`

**Purpose**: Injection points for custom post-processing

**ETL Hook** (transformations):

```typescript
interface PostLoadETLHook {
  onGraphStoreLoaded(graphStore: GraphStore): void;
}
```

**Validation Hook** (checks):

```typescript
interface PostLoadValidationHook {
  onGraphStoreLoaded(graphStore: GraphStore): void;
  onGraphLoaded(graph: Graph): void;
}
```

**Use cases**:

- Feature engineering (GraphSage embeddings)
- Consistency checks (algorithm requirements)
- Index building (spatial, text search)

---

## Memory Estimation

**Pattern** (throughout all loading components):

```typescript
static memoryEstimation(
  dimensions: GraphDimensions,
  concurrency: Concurrency
): MemoryEstimation
```

**Why critical?**:

- Pre-allocate correct buffer sizes
- Avoid OOM during import
- Choose appropriate compression

**Estimation factors**:

- Node count
- Relationship count (total + per type)
- Average degree
- Property count
- Concurrency (threads)

---

## Key Architectural Patterns

### 1. Batch + Sort + Compress Pipeline

```
Raw Data → Batch Buffer → Radix Sort → Pre-Aggregate → Compress → Final Graph
(Stage 1)   (Stage 1)     (Stage 2)    (Stage 3)       (Stage 4)   (Stage 5)
```

### 2. Paged Processing

```
Global node space [0..N)
  ↓
Divide into pages [0..P₀), [P₀..P₁), ..., [Pₙ..N)
  ↓
Process each page in parallel
  ↓
Combine into final graph
```

### 3. Dual-Pass for Relationships

```
Pass 1: Sort by source → Build outgoing adjacency lists
Pass 2: Sort by target → Build incoming adjacency lists (if needed)
```

### 4. Property Parallelism

```
Node/Edge data
  ↓
IDs + Property References stored together
  ↓
Property values extracted/converted in parallel
  ↓
Stored in separate property stores
```

---

## Integration with Arrow Factory (GAMMA Context)

### Where Arrow Factory Fits

**Current pipeline**:

```
??? → Batch Buffers → Sort → Compress → GraphStore
```

**With Arrow Factory**:

```
Arrow Tables → Arrow Factory → Batch Buffers → Sort → Compress → GraphStore
   (TP-004)        (NEW!)         (Reuse!)      (Reuse!) (Reuse!)   (Done!)
```

### What Arrow Factory Must Produce

**For nodes**:

```rust
// Arrow Factory produces:
Vec<NodesBatchBuffer<PropertyRef>>

// Which feeds existing pipeline:
NodesBatchBuffer → IdMap + NodePropertyStore → Nodes
```

**For relationships**:

```rust
// Arrow Factory produces:
Vec<RelationshipsBatchBuffer<PropertyRef>>

// Which feeds existing pipeline:
RelationshipsBatchBuffer → Sort → PreAggregate → Compress → SingleTypeRelationships
```

### Critical Integration Points

1. **Property References**:

   - Arrow: Column indices + value references
   - Need: Convert to PropertyRef format expected by buffers

2. **Batch Filling**:

   - Arrow: RecordBatch iteration
   - Need: Fill NodesBatchBuffer/RelationshipsBatchBuffer efficiently

3. **Label Tokens**:

   - Arrow: Schema metadata or column values
   - Need: NodeLabelTokenSet creation

4. **Paging**:
   - Arrow: Already chunked (RecordBatches)
   - Need: Map Arrow batches to pages consistently

---

## Rust Translation Priorities

**HIGH** (needed for Arrow Factory):

1. `RecordsBatchBuffer` (base class)
2. `NodesBatchBuffer<T>` (node collection)
3. `RelationshipsBatchBuffer<T>` (relationship collection)
4. `AdjacencyBufferPaging` (paging interface)
5. `RecordScannerTask` (parallel execution interface)

**MEDIUM** (optimization layer): 6. `AdjacencyBuffer` (compression orchestration) 7. `AdjacencyPreAggregation` (duplicate merging) 8. Radix sort implementation (performance)

**LOW** (final assembly): 9. `Nodes` structure (may already exist?) 10. `SingleTypeRelationships` structure (may already exist?) 11. ETL/Validation hooks (extension points)

---

## Quick Reference: Class Relationships

```
RecordsBatchBuffer (abstract base)
  ↓
├─ NodesBatchBuffer<PROPERTY_REF>
│   └─ Used by: NodeImporter
│
└─ RelationshipsBatchBuffer<PROPERTY_REF>
    ├─ Provides: sortBySource()
    ├─ Provides: sortByTarget()
    └─ Used by: RelationshipImporter
        ↓
    AdjacencyPreAggregation (merges duplicates)
        ↓
    AdjacencyBuffer (compression)
        ├─ Uses: AdjacencyBufferPaging
        ├─ Creates: ChunkedAdjacencyLists
        └─ Produces: CompressedAdjacencyList
            ↓
    SingleTypeRelationships (final)
```

---

## Key Differences: Java vs Rust

**Java** (GDS):

- Mutable buffers with concurrent writes
- `ReentrantLock` for thread safety
- GC handles cleanup
- Virtual dispatch for compressor selection

**Rust** (rust-gds):

- Move semantics for ownership transfer
- `Mutex<T>` or `RwLock<T>` for shared mutation
- RAII for automatic cleanup
- Trait objects or generics for compression strategy

**Translation challenges**:

- Java's `long[]` → Rust's `Vec<u64>` or `Vec<i64>`
- Concurrent writes → Need careful `Arc<Mutex<>>` or channels
- Property references → Generic type parameter `<PROPERTY_REF>`
- Radix sort → Port carefully (bitwise ops sensitive)

---

## Status

**Understanding**: ✅ Loading pipeline architecture mapped  
**Integration**: 🔄 Arrow Factory → Batch Buffers connection point identified  
**Translation**: ⏸️ Awaiting GAMMA execution  
**Priority**: 🎯 Critical for TP-004 success

---

_Notes for GAMMA study - understand before building!_
