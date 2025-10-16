# GAMMA Action: IO vs Loading Separation Strategy

**Document Type**: Action Plan  
**Date**: October 15, 2025  
**Status**: 🎯 Critical Architectural Decision  
**Context**: User insight: "Java GDS conflates Import/Export IO and Loading"

---

## The Problem (User Quote)

> "Right we have core/io and core/loading. they intertwine badly, it is a difficult architecture that will have to fix. Java GDS conflates Import and Export IO and Loading."

## The Strategy (User Direction)

> "I say we translate the Native Projection here, the NativeFactory and see how it relates to Core/IO and Core/LOADING which we have in TS and Java but not Rust"

---

## Java GDS Conflation (The Mess)

```
Java GDS Structure:
core/io/              ← Import (read) + Export (write) MIXED
core/loading/         ← In-memory construction BUT overlaps with io/
native-projection/    ← Neo4j DB → GDS (uses both io + loading)
```

**Problems**:

1. `io/` does TWO things (import AND export - conflated!)
2. `loading/` overlaps with `io/` (unclear boundary)
3. `native-projection/` bridges both (complex dependencies)
4. **Result**: "Intertwine badly" - hard to understand, hard to modify

---

## rust-gds Clean Separation (The Fix)

```
rust-gds Structure (Proposed):
projection/factory/   ← Arrow/Polars → GraphStore (Entry point) [TP-004]
core/loading/         ← Batch/Sort/Compress (Pure in-memory) [Translate on-demand]
io/import/            ← Files → Arrow (File reading only) [Future]
io/export/            ← GraphStore → Files (File writing only) [Future]
```

**Benefits**:

1. **Factory** = Entry point (clear!)
2. **Loading** = In-memory machinery (no file IO!)
3. **Import** = Read files → Arrow (separate!)
4. **Export** = Write files from GraphStore (separate!)
5. **Result**: Clean boundaries, easy to reason about

---

## The GAMMA Strategy: Translate & Observe

### Step 1: Translate NativeFactory (TP-004) ✅ FIRST

**What**: Create `projection/factory/arrow/`
**Input**: Arrow tables (ALREADY IN MEMORY!)
**Output**: GraphStore (in-memory)
**Defer**: File IO (assume tables provided)

**Why this order?**:

- Factory reveals what it needs from Loading
- No premature translation of unused code
- Clear separation: Factory = Projection, NOT IO

### Step 2: Observe Loading Dependencies 🔍 THEN

**After Factory execution, ask**:

- What `core/loading/` components did we need?
- Which batch buffers? (Nodes, Relationships)
- Which sorting? (Radix sort)
- Which compression? (AdjacencyBuffer, delta encoding)

**Translate on-demand**:

- Only translate Loading pieces Factory requires
- Port TS → Rust as needed
- Document dependencies

### Step 3: Add IO Later 📁 FINALLY

**Only after Factory + Loading working**:

- Add `io/import/` for Parquet/CSV → Arrow
- Add `io/export/` for GraphStore → Parquet/CSV
- Keep boundaries clean!

---

## Current Status: What We Have

### In TS (Translated from Java)

✅ `NodesBatchBuffer.ts` (node collection)
✅ `RelationshipsBatchBuffer.ts` (relationship collection + radix sort)
✅ `AdjacencyBuffer.ts` (compression orchestration)
✅ `AdjacencyPreAggregation.ts` (duplicate merging)
✅ `Nodes.ts`, `SingleTypeRelationships.ts` (final structures)
✅ `RecordScannerTask.ts` (parallel execution interface)
✅ Hook interfaces (PostLoadETLHook, PostLoadValidationHook)

### In Java (Reference)

✅ Complete `core/loading/` package (~20 classes)
✅ Complete `native-projection/` package (~70 classes)
✅ Reference architecture (conflated but functional)

### In Rust (Missing!)

❌ No `projection/factory/` (TP-004 will create!)
❌ No `core/loading/` equivalent (translate on-demand!)
❌ No `io/` separation (future work!)

---

## Decision Tree: What to Translate When

### NOW (Week 1-2: TP-004 Phases 1-4)

**Translate**: Factory skeleton

- `projection/factory/mod.rs` (Factory trait)
- `projection/factory/arrow/factory.rs` (ArrowNativeFactory)
- `projection/factory/arrow/config.rs` (Config)
- `projection/factory/arrow/reference.rs` (TableReference)

### SOON (Week 2-3: TP-004 Phases 5-6)

**Translate**: Loading pieces Factory needs

- Likely: Batch buffers (Node + Relationship)
- Likely: Property mapping (Arrow columns → Properties)
- Maybe: Sorting (if not in rust-gds already)
- Maybe: Compression (if not in rust-gds already)

### LATER (Week 4: TP-004 Phase 8)

**Translate**: Optimization layers

- Zero-copy paths (Arrow array → PropertyValues direct)
- Parallel task execution (if not using Rayon directly)
- Advanced compression (if performance requires)

### FUTURE (Post-GAMMA)

**Translate**: IO separation

- `io/import/` - File reading (Parquet, CSV, JSON)
- `io/export/` - File writing (Parquet, CSV, JSON)
- Keep Factory pure (no file IO knowledge!)

---

## Key Architectural Insight

**Factory is Projection, NOT IO!**

```text
WRONG mental model:
  Factory reads files → GraphStore
  (conflates IO + Projection)

RIGHT mental model:
  IO reads files → Arrow tables (in-memory)
  Factory projects Arrow → GraphStore
  (clean separation!)
```

**Why this matters**:

- Factory can work with ANY Arrow source (files, network, generated, Polars)
- IO layer is pluggable (Parquet, CSV, JSON, Arrow IPC, Flight)
- Testing easier (provide Arrow tables directly, skip file IO)
- Performance better (Arrow is the interchange format)

---

## Action Items (Immediate)

### Upon Return from Travel

1. **Start TP-004 Phase 1** (4-5 hours)

   - Create `projection/factory/` module structure
   - Implement Factory trait (generic entry point)
   - Implement ArrowNativeFactory skeleton
   - **Assume Arrow tables already in memory!**

2. **Document What's Needed** (ongoing)

   - As Factory development proceeds, note:
     - "Need batch buffer for nodes"
     - "Need sorting for relationships"
     - "Need property mapping"
   - Track in TODO comments or separate file

3. **Translate On-Demand** (Phases 2-6)

   - When Factory needs Loading component:
     - Check if rust-gds has equivalent
     - If not, translate from TS (or Java if needed)
     - Port minimal version (no over-engineering)

4. **Validate Separation** (Phase 8)
   - Verify Factory has NO file IO code
   - Verify Loading has NO file IO code
   - Defer IO layer to future work

---

## Success Criteria

### Week 1-2: Factory Skeleton

✅ Factory trait defined
✅ ArrowNativeFactory compiles
✅ Config system working
✅ **No file IO code in Factory!**

### Week 2-3: Factory Functional

✅ Factory produces GraphStore from Arrow tables
✅ Loading components integrated (as needed)
✅ PropertyMappings connected
✅ **Still no file IO in Factory!**

### Week 4: Factory Optimized

✅ Zero-copy paths working
✅ Performance acceptable (>1M nodes/sec)
✅ Integration tests pass
✅ **Factory is pure Projection!**

### Post-GAMMA: IO Separation

✅ `io/import/` added (Parquet → Arrow)
✅ `io/export/` added (GraphStore → Parquet)
✅ Factory unchanged (no file IO ever!)
✅ **Clean architecture validated!**

---

## Reference Documents

**Must read (GAMMA Week 1)**:

- ✅ `TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` (translation plan)
- ✅ `NATIVE_PROJECTION_ARROW_DESIGN.md` (design doc)
- ✅ `GRAPHSTORE_LOADING_QUICK_REFERENCE.md` (loading architecture)
- ✅ `PROJECTION_AS_EVAL_CENTER.md` (complexity warning)

**Reference (as needed)**:

- TS files: `NodesBatchBuffer.ts`, `RelationshipsBatchBuffer.ts`, etc.
- Java source: `/home/pat/GitHub/graph-data-science/native-projection/`
- Java source: `/home/pat/GitHub/graph-data-science/core/src/main/java/org/neo4j/gds/core/loading/`

---

## Status

**Strategy**: ✅ DEFINED - Translate NativeFactory first, observe Loading needs  
**Separation**: 🎯 CRITICAL - Factory = Projection (no IO!), Loading = In-memory (no IO!)  
**Timeline**: 🗓️ October 2025 (GAMMA month)  
**Priority**: 🔥 MAKE OR BREAK

---

_"Translate the Native Projection here, the NativeFactory and see how it relates to Core/IO and Core/LOADING"_ ✅

**Let the Factory reveal what it needs! Translate on-demand!** 🎯
