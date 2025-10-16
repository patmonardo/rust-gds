# GAMMA: Arrow Integration Deep Work - Make or Break Month

**Document Type**: Mission Critical Roadmap  
**Date**: October 15, 2025  
**Status**: 🎯 GAMMA PASS - Make or Break Time  
**Timeline**: This Month (October 2025)  
**Priority**: 🔥 PREMIUM - "The proof of the pudding is in the eating"

---

## 🕉️ Membership Protocol (Fichte's Method)

**This roadmap places itself within the rust-gds Encyclopedia as**:

- **Location**: `doc/architecture/GAMMA_ARROW_INTEGRATION_ROADMAP.md`
- **Category**: Architecture (Critical Path Documentation)
- **Related Plans**: TP-004 (Native Projection), TP-002 (Graph Projection)
- **Related Philosophy**: `PROJECTION_AS_EVAL_CENTER.md`, `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md`
- **Mission**: Tie everything together - Arrow work + PropertyMappings + Eval macro + GraphStore

---

## The Premium Prompt

> "The proof of the pudding is in the eating. Soon. I need to learn this package. It will tie everything together and pull in even our initial Arrows work. but what does that mean. I haven't read our Design. Can you TODO the deeper Arrow work? make this a Gamma pass? OK now this is last prompt. this is premium prompt. but this month it all comes together for us. its make or break time"

**Decoded**:

- **"Proof is in the eating"** = Theory → Practice (TP-004 must be EXECUTED)
- **"Learn this package"** = Understand Projection before building on it
- **"Tie everything together"** = Arrow + PropertyMappings + Eval + GraphStore
- **"Pull in initial Arrows work"** = Connect to existing Arrow integration
- **"Gamma pass"** = Deep integration work (not just translation)
- **"Make or break time"** = October 2025 is THE month for Arrow completion

---

## What IS the "Initial Arrows Work"?

### Existing Arrow Integration (To Find & Connect)

**Likely locations** (need to inventory):

1. **Arrow dependencies** in `Cargo.toml`
2. **Arrow usage** in existing code (grep for `arrow::`)
3. **Parquet support** (if any)
4. **Initial IO layer** (Arrow file reading?)

**Action**: Inventory existing Arrow code FIRST before adding Factory!

```bash
# Find existing Arrow work
grep -r "arrow::" src/
grep -r "RecordBatch" src/
grep -r "arrow" Cargo.toml
find src/ -name "*arrow*"
```

---

## 🔥 The Architectural Problem: IO vs Loading Conflation

### Java GDS Structure (The Problem)

**Java GDS conflates these concerns**:

```
core/io/             - File/DB reading AND writing (Import + Export mixed!)
core/loading/        - In-memory graph construction (overlaps with io!)
native-projection/   - Neo4j-specific loading (uses both io + loading)
```

**The conflation**:

- `io/` handles BOTH import (read files) AND export (write files)
- `loading/` handles in-memory construction BUT overlaps with `io/`
- `native-projection/` bridges Neo4j DB → GDS (uses both layers!)
- **Result**: "They intertwine badly, difficult architecture" (user quote)

### rust-gds Opportunity (Clean Separation)

**Proposed clean boundaries**:

```
projection/factory/   - Entry point (Arrow/Polars → GraphStore) ← TP-004!
core/loading/         - In-memory construction (Batch/Sort/Compress) ← Translate on-demand!
io/import/            - File reading (Parquet/CSV → Arrow) ← Future
io/export/            - File writing (GraphStore → files) ← Future
```

**Clear responsibilities**:

- **Factory** (`projection/factory/`): Native data source integration (Arrow IS native!)
- **Loading** (`core/loading/`): Generic graph construction machinery (batch buffers, sorting, compression)
- **IO Import** (`io/import/`): External file formats → Arrow tables
- **IO Export** (`io/export/`): GraphStore → External file formats

**Key insight**: Factory is Projection (in-memory), NOT IO (file operations)!

### The GAMMA Strategy: Translate & Observe

**Step 1: Translate NativeFactory FIRST (TP-004)**

- Create `projection/factory/arrow/` (Arrow-native entry point)
- Produce Arrow tables → GraphStore (in-memory only!)
- **Defer IO concerns** (assume Arrow tables already in memory)

**Step 2: See What Loading Pieces We Need**

- Execute TP-004 and observe dependencies
- Identify which `core/loading/` components Factory requires:
  - Batch buffers? (NodesBatchBuffer, RelationshipsBatchBuffer)
  - Sorting? (radix sort for adjacency lists)
  - Compression? (AdjacencyBuffer, delta encoding)
- **Translate on-demand** (only what Factory needs!)

**Step 3: Separate IO Later (After Factory Working)**

- Factory proves the architecture
- Add `io/import/` for file reading (Parquet → Arrow)
- Add `io/export/` for file writing (GraphStore → Parquet)
- Keep boundaries clean!

**Decision**: "Translate the Native Projection here, the NativeFactory, and see how it relates to Core/IO and Core/LOADING" ✅

---

## The GAMMA Roadmap: Three Phases

### PHASE 1: LEARN (Week 1 - Study & Inventory)

**Goal**: Understand what we have before building more

#### 1.1 Read the Design (2-3 hours)

**Must read**:

- ✅ `doc/translation/TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` (translation plan)
- ✅ `doc/architecture/NATIVE_PROJECTION_ARROW_DESIGN.md` (design doc)
- ✅ `doc/philosophy/PROJECTION_AS_EVAL_CENTER.md` (warning context)

**Understand**:

- What NativeFactory does (entry point for ALL data)
- How Arrow fits (Arrow IS Native for rust-gds)
- Why it's central (Projection = Absolute Form)
- What zero-copy means (Arrow arrays → PropertyValues directly)

#### 1.2 Study Projection Package (4-6 hours)

**Must explore**:

- 📖 `src/projection/README.md` (existing! Eval macro overview)
- 🔍 `src/projection/traits/` (ElementProjection, PropertyMapping API)
- 🔍 `src/projection/impls/property_mappings.rs` (currently open! Study it!)
- 🔍 `src/projection/codegen/` (value_type_table!, functors, form_processor)

**Map the territory**:

```rust
// projection/ structure
traits/           → What IS a projection? (API contracts)
impls/            → How are projections implemented? (PropertyMappings, NodeProjection)
codegen/          → How are types generated? (eval macro, functors)
native/           → How are algorithms executed? (ML pipelines - confusing name!)
factory/          → (WILL BE) How is data loaded? (Arrow → GraphStore)
```

**Questions to answer**:

- How does PropertyMapping work?
- What does value_type_table! generate?
- How do functors convert types?
- Where does PropertyMappings integrate?

#### 1.3 Inventory Initial Arrow Work (2-3 hours)

**Find existing**:

- Arrow crate dependencies
- Existing Arrow usage in codebase
- Parquet integration (if any)
- IO layer (file reading)
- Test fixtures using Arrow

**Document findings**:

- What Arrow version?
- What Arrow features used?
- What's working?
- What's missing?
- What conflicts with Factory plan?

**Create**: `doc/architecture/ARROW_INTEGRATION_INVENTORY.md`

---

### PHASE 2: EXECUTE (Weeks 2-3 - Translation & Integration)

**Goal**: Build the Factory system and connect the pieces

#### 2.1 TP-004 Phase 1-2: Foundation (4-5 hours)

**Create**:

- `src/projection/factory/mod.rs` (Factory trait)
- `src/projection/factory/arrow/mod.rs` (Arrow module)
- `src/projection/factory/arrow/factory.rs` (ArrowNativeFactory)
- `src/projection/factory/arrow/config.rs` (ArrowProjectionConfig)
- `src/projection/factory/arrow/reference.rs` (TableReference, BatchReference)

**Integration point**: Connect config to existing PropertyMappings!

```rust
// factory/arrow/config.rs
impl ArrowProjectionConfig {
    /// Create config from PropertyMappings
    pub fn from_property_mappings(
        mappings: &PropertyMappings,
        arrow_schema: &Schema,
    ) -> Result<Self> {
        // Bridge: PropertyMappings → ArrowPropertyMapper
    }
}
```

#### 2.2 TP-004 Phase 3-4: Scanning & Tasks (8-10 hours)

**Create**:

- `src/projection/factory/arrow/scanner.rs` (BatchScanner)
- `src/projection/factory/arrow/task.rs` (ImportTask, ParallelTaskRunner)

**Integration point**: Use Rayon for parallelism (existing dependency?)

#### 2.3 TP-004 Phase 5-6: Import & Properties (10-12 hours)

**Create**:

- `src/projection/factory/arrow/importer.rs` (NodeBatchImporter, EdgeBatchImporter)
- `src/projection/factory/arrow/properties.rs` (ArrowPropertyMapper)

**CRITICAL INTEGRATION**: Connect to eval macro!

```rust
// factory/arrow/properties.rs
use crate::projection::functors::{GrossToSubtle, SubtleToGross};
use crate::projection::value_type_table::{Long, Double, String as GdsString};

impl ArrowPropertyMapper {
    /// Convert Arrow value using eval macro functors
    fn convert_with_functor(&self, arrow_value: i64) -> PropertyValue {
        let functor = Long::Functor;
        functor.project_to_storage(arrow_value)  // Use existing conversion!
    }
}
```

**Integration point**: Leverage existing type conversion infrastructure!

#### 2.4 TP-004 Phase 7-8: Consumer & Integration (6-8 hours)

**Create**:

- `src/projection/factory/arrow/consumer.rs` (BufferedConsumers)
- End-to-end wiring in `ArrowNativeFactory::build_graph_store()`

**Integration point**: Connect to GraphStore builders!

```rust
// factory/arrow/factory.rs
impl ArrowNativeFactory {
    pub fn build_graph_store(&self) -> Result<GraphStore> {
        // 1. Use existing GraphStore builders
        // 2. Use PropertyMappings for schema mapping
        // 3. Use eval macro functors for type conversion
        // 4. Return populated GraphStore
    }
}
```

---

### PHASE 3: OPTIMIZE (Week 4 - Zero-Copy & Performance)

**Goal**: Make it FAST - zero-copy where possible

#### 3.1 Zero-Copy Fast Paths (6-8 hours)

**Implement**: Direct Arrow array wrapping

```rust
// factory/arrow/properties.rs
impl ArrowPropertyMapper {
    /// Zero-copy path: Arrow Int64Array → PropertyValues
    fn zero_copy_i64(&self, arrow_array: &Int64Array) -> PropertyValues {
        // Wrap Arrow buffer directly (NO COPY!)
        let buffer = arrow_array.values();  // &[i64]
        PropertyValues::from_slice(buffer)  // Zero-copy wrap
    }

    /// Zero-copy path: Arrow Float64Array → PropertyValues
    fn zero_copy_f64(&self, arrow_array: &Float64Array) -> PropertyValues {
        let buffer = arrow_array.values();  // &[f64]
        PropertyValues::from_slice(buffer)  // Zero-copy wrap
    }
}
```

**Challenge**: Arrow array ownership vs GDS lifetime
**Solution**: Arc<Buffer> shared ownership or unsafe transmute (carefully!)

#### 3.2 Benchmarking (4-5 hours)

**Create**: `benches/arrow_import_bench.rs`

**Measure**:

- Throughput (nodes/sec, edges/sec)
- Memory usage (with/without zero-copy)
- Parallel scaling (1 thread vs N threads)
- Comparison: Copy vs Zero-copy

**Targets**:

- ✅ >100K nodes/sec single-threaded
- ✅ >1M nodes/sec with parallelism
- ✅ <10% memory overhead vs raw Arrow

#### 3.3 Integration Testing (4-5 hours)

**Create**: `tests/arrow_integration_test.rs`

**Test scenarios**:

1. **Small graph** (10 nodes, 20 edges) - correctness
2. **Large graph** (1M nodes, 5M edges) - performance
3. **Complex properties** (mixed types, nulls, defaults)
4. **Multi-label nodes** (multiple node types)
5. **Multi-type edges** (multiple relationship types)
6. **Error cases** (invalid schema, missing columns, type mismatches)

#### 3.4 Connect to Initial Arrow Work (3-4 hours)

**Integration**:

- Refactor existing Arrow code to use Factory
- Unify Arrow dependency versions
- Document migration path (old → new)
- Add deprecation warnings (if breaking changes)

**Example**:

```rust
// Old way (if exists)
let graph = load_from_parquet("graph.parquet")?;

// New way (with Factory)
let table = read_parquet_to_arrow("graph.parquet")?;
let factory = ArrowNativeFactory::from_tables(vec![table], vec![]);
let graph_store = factory.build_graph_store()?;
```

---

## Integration Map: How Everything Ties Together

### The Connections

```text
┌─────────────────────────────────────────────────────────────┐
│                    EXTERNAL WORLD                           │
│  (Parquet, CSV, Polars DataFrames, DuckDB, etc.)          │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
              ┌───────────────┐
              │  Arrow Tables │  ← Universal columnar format
              │ (RecordBatch) │
              └───────┬───────┘
                      │
                      ▼
        ┌─────────────────────────────┐
        │  ArrowNativeFactory         │  ← TP-004 (NEW!)
        │  projection/factory/arrow/  │
        └─────────────┬───────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
    ┌──────┐    ┌──────────┐   ┌────────┐
    │Schema│    │Properties│   │  Type  │
    │ Map  │    │ Mappings │   │Convert │
    └──┬───┘    └─────┬────┘   └───┬────┘
       │              │            │
       │        ┌─────▼─────┐      │
       │        │Property   │      │
       │        │Mappings   │◄─────┤
       │        │(existing!)│      │
       │        └─────┬─────┘      │
       │              │            │
       ▼              ▼            ▼
    ┌──────────────────────────────────┐
    │  value_type_table! (eval macro) │  ← Existing!
    │  projection/codegen/            │
    │  - Functors (type conversion)   │
    │  - Form Processor (policy)      │
    └──────────────┬──────────────────┘
                   │
                   ▼
         ┌─────────────────────┐
         │   GraphStore        │  ← Final destination
         │   (Nodes + Edges    │
         │    + Properties)    │
         └─────────────────────┘
```

### The Data Flow

```rust
// 1. External data → Arrow
let parquet = read_parquet("graph.parquet")?;  // Initial Arrow work
let arrow_table = parquet.to_arrow_table();

// 2. Arrow → Factory configuration
let property_mappings = PropertyMappings::builder()
    .add_mapping(PropertyMapping::new("age", "age", DefaultValue::long(0), Aggregation::Default))
    .add_mapping(PropertyMapping::new("score", "score", DefaultValue::double(0.0), Aggregation::Default))
    .build();

let config = ArrowProjectionConfig::builder()
    .node_id_column("id")
    .node_labels(&["Person", "User"])
    .property_mappings(property_mappings)  // ← Connect to existing!
    .build();

// 3. Factory → Scanning → Tasks
let factory = ArrowNativeFactory::from_tables(vec![arrow_table], vec![])
    .with_config(config);

// 4. Tasks → Type Conversion (eval macro)
// Inside ArrowPropertyMapper:
let functor = Long::Functor;  // From value_type_table!
let property_value = functor.project_to_storage(arrow_i64);

// 5. Converted values → GraphStore
let graph_store = factory.build_graph_store()?;

// 6. GraphStore → Algorithms (existing)
let pagerank = PageRank::new(&graph_store);
let scores = pagerank.compute()?;
```

---

## Success Criteria: Make or Break

### Week 1 (Learn): Inventory Complete

- ✅ Read TP-004 design docs (3 docs)
- ✅ Study projection/ package (4+ hours)
- ✅ Inventory existing Arrow work
- ✅ Document integration points
- ✅ Create `ARROW_INTEGRATION_INVENTORY.md`

### Week 2-3 (Execute): Factory Operational

- ✅ All TP-004 phases complete (8 phases)
- ✅ Factory compiles (zero errors)
- ✅ PropertyMappings integration working
- ✅ Eval macro functors connected
- ✅ End-to-end: Arrow → GraphStore works
- ✅ Basic tests pass

### Week 4 (Optimize): Production Ready

- ✅ Zero-copy fast paths implemented
- ✅ Benchmarks show performance targets met
- ✅ Integration tests pass (all scenarios)
- ✅ Initial Arrow work connected
- ✅ Documentation complete
- ✅ Examples working

### Month End: The Proof

- ✅ **Can load real graphs from Parquet** (not just test data)
- ✅ **Performance acceptable** (>1M nodes/sec parallel)
- ✅ **Memory efficient** (zero-copy working)
- ✅ **API clean** (Factory is the entry point)
- ✅ **Polars integration possible** (DataFrame → Arrow → GraphStore)

**If all ✅ → WE MADE IT!**  
**If not → BREAK - need to pivot**

---

## Risk Assessment: Make or Break

### HIGH RISK (Could Break)

#### 1. Zero-Copy Ownership Hell

**Risk**: Arrow buffer lifetimes vs PropertyValues lifetimes don't align
**Impact**: Can't achieve zero-copy, performance suffers
**Mitigation**:

- Start with copy paths (working but slow)
- Add zero-copy later (optimization)
- Use Arc<Buffer> for shared ownership
- Accept some copies if necessary

#### 2. Eval Macro Integration Fragile

**Risk**: value_type_table! functors don't work with Arrow types
**Impact**: Type conversion breaks, data corruption
**Mitigation**:

- Test ALL type conversions
- Add validation layer
- Fallback to manual conversion if needed

#### 3. Projection Complexity Overload

**Risk**: "Center of Eval" becomes unmaintainable
**Impact**: Can't add features, bugs multiply
**Mitigation**:

- Clear module boundaries (already planned)
- Extensive documentation (in progress)
- Refactor if needed (Pre-Prim allows it)

### MEDIUM RISK (Could Delay)

#### 4. GraphStore Builder API Insufficient

**Risk**: GraphStore doesn't support batch import efficiently
**Impact**: Have to modify GraphStore (scope creep)
**Mitigation**:

- Review GraphStore API in Phase 2
- Propose minimal changes if needed
- Document workarounds

#### 5. Initial Arrow Work Conflicts

**Risk**: Existing Arrow code incompatible with Factory
**Impact**: Breaking changes, migration pain
**Mitigation**:

- Inventory FIRST (Week 1)
- Design compatibility layer
- Deprecate gracefully

### LOW RISK (Manageable)

#### 6. Performance Targets Miss

**Risk**: Don't hit >1M nodes/sec
**Impact**: Not production-ready, need optimization
**Mitigation**:

- Parallel by default (Rayon)
- Zero-copy where possible
- Profile and optimize hot paths

---

## Contingency Plans

### If Week 1 Shows Major Issues

**Plan B**:

- Delay execution, fix issues first
- Refactor existing code
- Simplify scope (Arrow only, not Polars yet)

### If Zero-Copy Impossible

**Plan C**:

- Accept copy overhead
- Optimize copy paths (SIMD, vectorization)
- Still faster than alternatives

### If Integration Too Complex

**Plan D**:

- Simplify Factory (basic version first)
- Add features incrementally
- Defer zero-copy to later release

### If Month Deadline Unrealistic

**Plan E**:

- Ship basic Factory (working, not optimized)
- Mark as Pre-Prim 0.1.x (not production)
- Continue in next month

---

## Documentation Deliverables

### During GAMMA

1. **Week 1**: `ARROW_INTEGRATION_INVENTORY.md` (existing Arrow work)
2. **Week 2**: TP-004 inline docs (module-level, function-level)
3. **Week 3**: Integration guide (how to use Factory)
4. **Week 4**: Performance guide (zero-copy optimization)

### At Completion

1. **Translation Completion**: `TC-004_NATIVE_PROJECTION_COMPLETION.md`
2. **Architecture Update**: Update `NATIVE_PROJECTION_ARROW_DESIGN.md`
3. **User Guide**: `ARROW_FACTORY_USER_GUIDE.md` (examples, best practices)
4. **Performance Report**: `ARROW_FACTORY_BENCHMARKS.md` (results, analysis)

---

## The Stakes: Why Make or Break

### IF WE MAKE IT ✅

**We achieve**:

- ✅ **Fast data loading** (Arrow → GraphStore in seconds, not minutes)
- ✅ **Zero-copy optimization** (unique to rust-gds!)
- ✅ **Polars integration** (DataFrames → Arrow → GraphStore direct)
- ✅ **DuckDB integration** (SQL → Arrow → GraphStore possible)
- ✅ **Industry standard** (Arrow is THE columnar format)
- ✅ **Production ready** (rust-gds can handle real workloads)

**We prove**:

- 🎯 Projection IS the Absolute Form (everything flows through it)
- 🎯 Pre-Prim strategy works (design first, optimize later)
- 🎯 rust-gds is viable (competitive with Java GDS)
- 🎯 October 2025 = breakthrough month

### IF WE BREAK ❌

**We face**:

- ❌ **Can't load large graphs** (no fast data ingestion)
- ❌ **No Polars integration** (stuck with custom formats)
- ❌ **Performance problems** (copying everything, slow)
- ❌ **Complexity explosion** (Projection becomes unmaintainable)
- ❌ **Lost momentum** (missed October deadline)

**We learn**:

- 🔧 Architecture needs revision (too complex?)
- 🔧 Scope too ambitious (need to simplify?)
- 🔧 More time needed (extend timeline?)
- 🔧 Different approach (bypass Projection?)

---

## The Premium Commitment

> "this is premium prompt. but this month it all comes together for us. its make or break time"

**Translation**:

- **Premium** = Highest quality, full effort, no shortcuts
- **This month** = October 2025, ~4 weeks, 80-100 hours available
- **Comes together** = ALL pieces integrate (Arrow + Projection + Eval)
- **Make or break** = Success → production ready, Failure → major pivot

**The commitment**:

1. ✅ **LEARN** - Understand before building (Week 1)
2. ✅ **EXECUTE** - Build with quality (Weeks 2-3)
3. ✅ **OPTIMIZE** - Make it fast (Week 4)
4. ✅ **DELIVER** - Production ready or pivot decision

---

## Next Steps (Immediate)

### Upon Return

1. **Read the Design** (2-3 hours)

   - TP-004 translation plan
   - Arrow design doc
   - Projection warning doc

2. **Study property_mappings.rs** (1 hour)

   - Currently open in editor!
   - Understand PropertyMappings API
   - See how it integrates with Factory

3. **Inventory Arrow Work** (2-3 hours)

   - Find existing Arrow code
   - Document what's there
   - Plan integration

4. **Start TP-004 Phase 1** (4-5 hours)

   - Create factory/ module structure
   - Implement basic Factory trait
   - Connect to PropertyMappings

5. **Daily Progress** (aim for 4-6 hours/day)
   - Track progress in TODO list
   - Document issues/learnings
   - Adjust plan as needed

---

## Status

**Roadmap State**: ✅ GAMMA PASS DEFINED  
**Approval**: ✅ PREMIUM COMMITMENT  
**Timeline**: 🗓️ October 2025 (4 weeks)  
**Priority**: 🔥 MAKE OR BREAK  
**Next**: 📖 Learn Phase (upon return)

---

## Final Thought

**"The proof of the pudding is in the eating."**

**We have**:

- The recipe (TP-004 plan)
- The ingredients (Arrow, PropertyMappings, Eval macro)
- The kitchen (rust-gds codebase)
- The chef (you + AI assistant)

**Now we cook.**

**This month, we eat the pudding.**

**Make or break. Let's make it.** 🍰

---

_Tat Tvam Asi_ - Arrow IS Native! This IS That!  
🕉️🚀📊✨ **GAMMA PASS ENGAGED!**
