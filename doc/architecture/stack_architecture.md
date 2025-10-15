# Rust-GDS Stack Architecture

**Document Status**: Living Document  
**Last Updated**: October 5, 2025  
**Phase**: Post-PureGraphStore, Pre-CoreGraphStore

---

## Executive Summary

Rust-GDS is a high-performance graph data science library that serves as the execution engine for a TypeScript-based DSL (GDSL). The architecture translates Neo4j's battle-tested GDS design from Java to Rust, leveraging Arrow/Polars for columnar storage and providing seamless integration with TypeScript through nodejs-polars.

**Key Innovation**: A three-layer stack where TypeScript provides an expressive DSL, Rust provides performance, and Polars provides zero-copy data exchange.

---

## The Full Stack

```
┌─────────────────────────────────────────────────────────┐
│  Application Layer                                      │
│  • User code in TypeScript                              │
│  • Graph algorithms, projections, queries               │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│  GDSL Layer (TS-GDS)                                    │
│  • TypeScript DSL for graph operations                  │
│  • Type-safe projection specifications                  │
│  • Algorithm composition                                │
│  • @Logic and @Model processors                         │
└──────────────────────┬──────────────────────────────────┘
                       │ nodejs-polars bridge
                       │ (zero-copy via Arrow)
┌──────────────────────▼──────────────────────────────────┐
│  Rust-GDS Execution Layer (this codebase)              │
│                                                          │
│  ┌────────────────────────────────────────────┐        │
│  │  PureGraphStore (current, Oct 2025)        │        │
│  │  • In-memory property graph                │        │
│  │  • Arc-based zero-copy views               │        │
│  │  • Snapshot semantics                      │        │
│  │  • Validates API surface                   │        │
│  └────────────────────────────────────────────┘        │
│                                                          │
│  ┌────────────────────────────────────────────┐        │
│  │  CoreGraphStore (planned, Oct 2025)        │        │
│  │  • Arrow-backed columnar storage           │        │
│  │  • SuperBlock format (mmap-able)           │        │
│  │  • Polars DataFrame integration            │        │
│  │  • Production-grade persistence            │        │
│  └────────────────────────────────────────────┘        │
│                                                          │
│  ┌────────────────────────────────────────────┐        │
│  │  PregelComputer (future)                   │        │
│  │  • Bulk Synchronous Parallel model         │        │
│  │  • Virtual thread execution                │        │
│  │  • Message-passing infrastructure          │        │
│  │  • Graph algorithms (PageRank, etc.)       │        │
│  └────────────────────────────────────────────┘        │
└─────────────────────────────────────────────────────────┘
```

---

## Component Details

### 1. GDSL (TypeScript GDS)

**Purpose**: Expressive, type-safe DSL for graph data science

**Features**:

- AI-generated from Neo4j GDS (Java) → TypeScript translation
- Graph projection DSL (node filters, relationship filters, properties)
- Algorithm composition and chaining
- Integration with @Logic/@Model processors
- Type safety throughout the pipeline

**Role**:

- User-facing API (familiar to web developers)
- Generates execution plans
- Delegates heavy computation to Rust layer

**Limitations & Solutions**:

- ❌ Performance: Large graphs slow in pure TypeScript
  - ✅ Solution: Delegate to Rust via nodejs-polars
- ❌ Memory: V8 heap limits for large datasets
  - ✅ Solution: Columnar storage in Rust (Arrow format)
- ❌ Concurrency: Single-threaded JavaScript
  - ✅ Solution: Rust parallel execution with Rayon

---

### 2. nodejs-polars Bridge

**Purpose**: Zero-copy data exchange between TypeScript and Rust

**Mechanism**:

- Polars DataFrames use Arrow format (language-agnostic)
- No serialization overhead (direct memory sharing)
- PropertyValues ↔ Arrow columns (zero-copy conversions)

**Data Flow**:

```
GDSL Query → Execution Plan (TS)
           ↓
Polars DataFrame (Arrow format)
           ↓
Rust-GDS PropertyValues (Arc<dyn NodePropertyValues>)
           ↓
Graph Algorithms (Rust)
           ↓
Result PropertyValues
           ↓
Polars DataFrame (Arrow format)
           ↓
GDSL Results (TS)
```

---

### 3. Rust-GDS Execution Layer

#### 3.1 PureGraphStore (Current, October 2025)

**Status**: ✅ Complete, validated via integration tests

**Architecture**:

- In-memory graph store with property management
- Arc-based sharing (zero-copy views)
- Immutable Graph snapshots
- Builder pattern for copy-on-write mutations
- Triadic property system (graph/node/relationship)

**Key Abstractions**:

```rust
Property         = Schema + PropertyValues (columns)
PropertyStore    = HashMap<String, Property>
GraphStore       = Container (schema, topology, properties)
Graph            = Immutable view/projection of GraphStore
Cursors          = Traversal iterators over topology
```

**Validated Behaviors** (via integration tests):

- ✅ Hierarchical key organization (top-level, label-scoped, type-scoped)
- ✅ Property selection semantics (auto-select vs. conservative fallback)
- ✅ Arc-based zero-copy sharing across multiple views
- ✅ Snapshot semantics (views capture state at creation time)
- ✅ GraphStore as container (schema holder, topology container, property manager)
- ✅ ResultStore semantics (GraphStore as pipeline output)

**Purpose**:

- Validate API design before columnar implementation
- Prove Arc-sharing and snapshot semantics work
- Establish contracts for nodejs-polars integration
- Serve as reference implementation

---

#### 3.2 CoreGraphStore (Planned, Week of October 6, 2025)

**Status**: 🔜 Next major milestone

**Vision**:

- Arrow-backed columnar property storage
- SuperBlock format (manifest + segments + columnar layout)
- Memory-mapped file support (large graphs on disk)
- Zero-copy integration with Polars DataFrames
- Write-Ahead Log (WAL) + compaction for durability

**SuperBlock Architecture**:

```
SuperBlock/
├── manifest.json       # Schema, version, segment index
├── nodes/
│   ├── id_map.arrow
│   └── properties/
│       ├── age.arrow
│       └── salary.arrow
└── relationships/
    ├── KNOWS/
    │   ├── topology.arrow
    │   └── properties/
    │       └── weight.arrow
    └── WORKS_AT/
        ├── topology.arrow
        └── properties/
            └── tenure.arrow
```

**Key Transitions** (PureGraphStore → CoreGraphStore):

- `Vec<T>` → `arrow2::PrimitiveArray<T>`
- `HashMap` → mmap segments with manifest
- In-memory only → Disk-backed with caching
- Manual Arc → Arrow reference counting

**Benefits**:

- Massive graphs (billions of nodes) via mmap
- Instant startup (no loading, direct mmap)
- Polars native integration (Arrow format)
- Compression (Arrow's encoding schemes)

---

#### 3.3 PregelComputer (Future)

**Status**: 🔮 Vision stage

**Inspiration**: Neo4j GDS's Pregel API + Google Pregel paper

**Model**: Bulk Synchronous Parallel (BSP)

- Supersteps (compute → message → barrier)
- Virtual threads per node (lightweight)
- Message passing between neighbors
- Global aggregation (convergence detection)

**Target Algorithms**:

- PageRank, Label Propagation
- Community detection (Louvain, Leiden)
- Centrality (Betweenness, Closeness)
- Path algorithms (Shortest Path, A\*)

**Integration**:

- Operates on Graph views (read-only topology)
- Writes results as PropertyValues
- Exposes via GDSL as high-level operations

---

## Architectural Layers (Neo4j GDS Heritage)

Based on Neo4j GDS design, the system has conceptual layers:

### Layer 1: Projection Algebra (Future)

- **What**: Schema DSL for declaring graph projections
- **Components**: Node filters, relationship filters, property selections
- **Output**: Projection specifications (data structure)

### Layer 2: NativeFactory (Future)

- **What**: Smart drivers/interpreters for projection specs
- **Role**: Translates Projection Algebra → executable GraphFactory
- **Vision**: AI-assisted generation from small component set

### Layer 3: GraphFactory (Future)

- **What**: Executors that build graphs from specs
- **Role**: Reads data sources, applies projections, produces GraphStore

### Layer 4: GraphStore as Container (✅ Validated)

- **What**: Holder of projected data
- **Components**: Schema metadata, topology, property orchestration
- **Tests**: `tests/graphstore_as_container.rs`
- **Key Insight**: GraphStore is polymorphic (source/intermediate/result contexts)

### Layer 5: Projection in Execution (✅ Validated)

- **What**: Observable runtime behavior (queries, traversals)
- **Tests**:
  - `tests/property_key_distribution.rs` (hierarchical organization)
  - `tests/property_selection_semantics.rs` (ADR 0005 validation)
  - `tests/arc_sharing_and_snapshots.rs` (zero-copy semantics)
- **Key Insight**: What you can _do_ with projected graphs

**Current Status**: Layers 4 and 5 are validated. Layers 1-3 are future work (likely informed by GDSL requirements).

---

## Design Principles (GDS → Rust Translation)

### 1. Immutability via Arc

- **Java GDS**: Immutable graph catalog, expensive copies
- **Rust-GDS**: Arc-based sharing, cheap clones
- **Benefit**: Zero-copy views, snapshot semantics

### 2. Columnar Storage

- **Java GDS**: HugeArrays for large graphs
- **Rust-GDS**: Arrow arrays (Polars-native)
- **Benefit**: Memory efficiency, compression, mmap support

### 3. Type Safety

- **Java GDS**: Runtime type checking, reflection
- **Rust-GDS**: Compile-time trait bounds
- **Benefit**: Zero-cost abstractions, early error detection

### 4. Projection as First-Class Concept

- **Java GDS**: Graph projections create new catalog entries
- **Rust-GDS**: Graph views via Arc (no data duplication)
- **Benefit**: Cheap projections, multiple views of same data

### 5. Builder Pattern for Mutations

- **Java GDS**: Mutable builders, immutable results
- **Rust-GDS**: Same pattern (copy-on-write via builders)
- **Benefit**: Clear mutation boundaries, safe concurrency

---

## Integration Test Suite (Validation Layer)

### Current Coverage (October 5, 2025)

**Layer 5: Projection in Execution**

- ✅ `property_key_distribution.rs` (283 lines)

  - Hierarchical key organization (top-level, label-scoped, type-scoped)
  - Label intersection, type union
  - PropertyValues wrapping inspection

- ✅ `property_selection_semantics.rs` (206 lines)

  - ADR 0005 validation
  - Single property auto-selection
  - Multiple properties conservative fallback
  - Per-type property selection

- ✅ `arc_sharing_and_snapshots.rs` (304 lines)
  - Arc strong count tracking
  - Multiple views share columns (zero-copy)
  - Graph views as snapshots
  - Filtered views share topology
  - Properties persist after store mutation

**Layer 4: GraphStore as Container**

- ✅ `graphstore_as_container.rs` (NEW, Oct 5)
  - GraphStore as schema holder
  - GraphStore as topology container
  - GraphStore as property manager
  - Graph as immutable projection
  - ResultStore semantics
  - Container metadata and provenance

**Total**: 4 integration test suites, ~1000 lines, 17 test cases

---

## Current Milestone: Dust Settling (October 5, 2025)

**Context**: Three-day intensive PureGraphStore development sprint (Oct 2-5)

**Completed**:

- ✅ Core property system (Property, PropertyStore, PropertyValues)
- ✅ Builder ergonomics (convenience methods)
- ✅ ADR 0005 (property selection semantics)
- ✅ All 6 examples enhanced with educational commentary
- ✅ 4 comprehensive integration test suites
- ✅ 124 unit tests + 17 integration tests passing
- ✅ Architectural uncertainties documented

**Current Phase**: Review and consolidation

- Reading the codebase "hot off the press"
- Validating design decisions
- Prioritizing next steps
- Preparing for CoreGraphStore milestone

**Next Week Goal**: Begin CoreGraphStore (Arrow/SuperBlock)

- Likely start: October 6, 2025 (tomorrow)
- Focus: Arrow-backed PropertyValues implementation
- Reference: PureGraphStore API as contract

---

## Development Philosophy

### Human + AI Collaboration

**Challenge**: One person cannot rethink a complete ML architecture

**Solution**: Human + AI Codex translating proven designs

- **Human**: Domain knowledge, GDS understanding, design decisions
- **AI**: Rust idioms, pattern translation, test generation
- **Result**: Neo4j GDS (Java) → Rust-native implementation

**Process**:

1. Java GDS → AI → TypeScript GDS (GDSL)
2. GDSL requirements → Drive Rust-GDS design
3. Integration tests → Validate contracts
4. Iterate → Refine based on nodejs-polars needs

### Translation Over Invention

**Not**: Inventing new graph architecture from scratch  
**But**: Translating battle-tested Neo4j GDS to Rust

**Benefits**:

- Proven at scale (Neo4j production use)
- Community understanding (GDS users know concepts)
- Clear roadmap (GDS features as guide)
- Known pitfalls (learn from GDS evolution)

---

## Roadmap

### Immediate (Week of Oct 6, 2025)

- [ ] CoreGraphStore foundation
- [ ] Arrow-backed PropertyValues
- [ ] SuperBlock manifest design
- [ ] Polars DataFrame conversion utilities

### Near-Term (October 2025)

- [ ] Mmap support for large graphs
- [ ] WAL + compaction for writes
- [ ] Projection Algebra initial design
- [ ] NativeFactory prototype

### Medium-Term (Q4 2025)

- [ ] PregelComputer foundation
- [ ] Basic algorithms (PageRank, Label Propagation)
- [ ] GDSL → Rust-GDS full integration
- [ ] Performance benchmarks vs. Neo4j GDS

### Long-Term (2026+)

- [ ] Advanced algorithms (community detection, centrality)
- [ ] Distributed execution (multi-node graphs)
- [ ] Streaming graph support (temporal graphs)
- [ ] Production deployment patterns

---

## References

### Neo4j GDS (Java)

- **Source**: Neo4j Graph Data Science Library
- **Concepts**: Projection, ResultStore, Graph Catalog, Pregel
- **Architecture**: Layers 1-5 conceptual model

### TypeScript GDS (GDSL)

- **Location**: `ts-gds/` directory
- **Purpose**: DSL layer for Rust-GDS
- **Integration**: nodejs-polars bridge

### Documentation

- `doc/pure_graphstore_checklist.md` - Pre-CoreGraphStore readiness
- `doc/adr0005_property_selection.md` - Property selection semantics
- `examples/*.rs` - Educational examples (6 files)
- `tests/*.rs` - Integration test suites (4 files)

---

## Conclusion

Rust-GDS is not a research project but a **translation effort**: bringing Neo4j GDS's proven architecture to Rust with modern tooling (Arrow, Polars, TypeScript DSL). The PureGraphStore phase validated the API surface; CoreGraphStore will provide production-grade performance; PregelComputer will enable graph algorithms at scale.

The three-layer stack (GDSL → nodejs-polars → Rust-GDS) provides the best of all worlds: TypeScript expressiveness, Polars efficiency, and Rust performance. Integration tests ensure the layers interoperate correctly.

**Status**: Foundation complete. Ready for CoreGraphStore.

---

**End of Document**
