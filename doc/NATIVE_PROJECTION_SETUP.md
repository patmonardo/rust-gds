# Native Projection: Translation Plan vs Implementation Record

**Status**: 🌟 Translation Plan APPROVED, Implementation PENDING  
**Date**: October 15, 2025  
**Purpose**: Setup document for GAMMA Arrow Factory work

---

## The Dialectic: Prakasa ↔ Kriya

### Translation Plan (Prakasa - Illumination)

**Location**: `doc/translation/TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` (32KB)

**What it is**:

- Strategic vision
- File mapping (Java → Rust)
- 8-phase execution plan
- 24-30 hour estimate
- Conceptual architecture

**Key sections**:

- Context & Motivation (why Arrow-native, not 1:1)
- Module organization (`projection/factory/arrow/`)
- File-by-file mapping (~70 Java files → 9 Rust modules)
- Phase breakdown with hour estimates
- Integration points (PropertyMappings, eval macro, GraphStore)
- Success criteria

**Status**: ✅ APPROVED by user ("Well you have my approval")

**Why no TypeScript intermediary?**: Native Projection is a **direct Java → Rust translation**. TypeScript GDS doesn't have this subsystem (different data source approach). This is actually **ideal** because:

1. **Direct from source** - No telephone game, straight from Java GDS archetypal form
2. **Conceptual innovation** - Neo4j-native → Arrow-native (substrate shift, not re-translation)
3. **Dogfooding KG** - The docs themselves (plans + records) ARE the knowledge graph
4. **Clean lineage** - Java (source) → rust (target), no intermediate translation artifacts

### Implementation Record (Kriya - Action Result)

**Location**: `doc/implementation/NATIVE_PROJECTION_ARROW_FACTORY.md` ← **DOES NOT EXIST YET**

**What it will be** (after execution):

- Execution record (what actually happened)
- Test counts per phase
- Surprises encountered
- Deviations from plan
- Performance benchmarks
- Integration successes/challenges
- Lessons learned

**Pattern** (from Progress Tracking):

- Could be single file or multi-phase (Phase 1, 2, 3...)
- Preserves "neat record" of reverse engineering
- Informs future translations (permanent process)
- Shows the 1:1 Rev Eng details

**Status**: ⏸️ PENDING - Awaits TP-004 execution

---

## The Beautiful Truth

### What You Said

> "but in a sense we are supporting TS and Rust and Java versions of GDS. [...] But the Implementation Informs how to Translate. Because in a sense this can be seen as a permanent process."

**YES!** This is profound:

1. **Translation is NOT one-time** - It's ongoing dialogue between versions
2. **Implementation teaches Translation** - What we learn doing TP-004 will inform TP-005, TP-006...
3. **Reverse Engineering is Discovery** - "Slavish" but revealing metaphysical structures
4. **GDS Architecture is Sound** - "Well infused metaphysically speaking"

### The Permanent Process

```
Java GDS (Archetypal Form) ← NativeFactory.java (Neo4j-native)
    ↓
    ↓ (TypeScript GDS - no Native Projection translation)
    ↓
rust-gds (Direct Translation + Innovation) ← ArrowNativeFactory (Arrow-native)
    ↕
Continuous Feedback Loop:
  Translation Plan → Implementation → Implementation Record → Next Plan
```

**The Knowledge Graph IS the Process**:

- **Translation Plans** = Strategic nodes (vision, file mapping, phases)
- **Implementation Records** = Tactical nodes (execution, surprises, learnings)
- **Edges** = Informs, teaches, guides (feedback loops)
- **Innovation** = New patterns emerge (Arrow-native, zero-copy, traits)

**Each implementation record**:

- Documents what worked (preserve)
- Documents what didn't (avoid)
- Documents surprises (learn)
- Documents new patterns (innovate)
- **Becomes KG node** (permanent knowledge, queryable, connectable)

---

## Current Status: Native Projection

### TP-004 Translation Plan Structure

**8 Phases** (from the plan):

1. **Phase 1: Core Infrastructure** (4-5 hours)

   - Factory trait, config, module structure

2. **Phase 2: Reference Types** (2-3 hours)

   - TableReference, BatchReference

3. **Phase 3: Scanner System** (4-5 hours)

   - BatchScanner trait, NodeBatchScanner, RelationshipBatchScanner

4. **Phase 4: Task Execution** (3-4 hours)

   - ParallelImportTask, execution framework

5. **Phase 5: Importers** (4-5 hours)

   - NodeBatchImporter, RelationshipBatchImporter

6. **Phase 6: Property System** (3-4 hours)

   - ArrowPropertyMapper, integration with PropertyMappings

7. **Phase 7: Consumers** (2-3 hours)

   - BufferedNodeConsumer, BufferedRelationshipConsumer

8. **Phase 8: Integration** (2-3 hours)
   - End-to-end tests, GraphStore integration, docs

**Total**: 24-30 hours, ~70 Java files → 9 Rust modules

### File Mapping (Key Files)

**Java Source** (Neo4j GDS):

```
graph-data-science/native-projection/src/main/java/
└─ org/neo4j/gds/projection/
   ├─ NativeFactory.java               → factory.rs
   ├─ RecordScannerTask.java          → task.rs
   ├─ NodesBatchBuffer.java           → importer.rs (nodes)
   ├─ RelationshipsBatchBuffer.java   → importer.rs (relationships)
   └─ ... (~70 files total)
```

**Rust Target**:

```
src/projection/factory/arrow/
├─ factory.rs        (ArrowNativeFactory)
├─ reference.rs      (TableReference, BatchReference)
├─ scanner.rs        (BatchScanner trait + impls)
├─ task.rs           (ParallelImportTask)
├─ importer.rs       (NodeBatchImporter, EdgeBatchImporter)
├─ consumer.rs       (BufferedConsumers)
├─ properties.rs     (ArrowPropertyMapper)
├─ config.rs         (ArrowProjectionConfig)
└─ mod.rs            (Public API)
```

### Key Design Decisions (from Plan)

**Not 1:1 Translation**:

- Replace Neo4j Transaction API with Arrow RecordBatch API
- Replace cursors with batch iterators
- Zero-copy optimization (Arrow arrays → PropertyValues directly)
- Design for multiple native sources

**Integration Points**:

- PropertyMappings (existing `projection/impls/`)
- Eval macro functors (existing `projection/codegen/`)
- GraphStore builders (verify API compatibility)
- Loading pipeline (core/loading/)

**Success Criteria**:

- All Arrow data types supported
- Zero-copy where possible
- Parallel import working
- > 1M nodes/sec throughput
- Integration tests passing

---

## Setup for GAMMA Work

### Prerequisites (Already Done! ✅)

1. ✅ **Translation Plan exists** (TP-004, 32KB, approved)
2. ✅ **Design document exists** (NATIVE_PROJECTION_ARROW_DESIGN.md)
3. ✅ **GAMMA Roadmap exists** (month plan, 3 phases)
4. ✅ **IO/Loading separation strategy** (architectural clarity)
5. ✅ **PropertyMappings** open in editor (integration reference)

### What You Need to Read

**CRITICAL (before starting)**:

1. 📖 `doc/translation/TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` ← **THE PLAN**
2. 📖 `doc/architecture/NATIVE_PROJECTION_ARROW_DESIGN.md` ← **THE DESIGN**
3. 📖 `src/projection/impls/property_mappings.rs` ← **INTEGRATION POINT**
4. 📖 `src/projection/README.md` ← **EVAL MACRO CONTEXT**

**REFERENCE (during work)**: 5. 📖 `doc/architecture/GAMMA_ARROW_INTEGRATION_ROADMAP.md` ← **MONTH PLAN** 6. 📖 `doc/architecture/GAMMA_IO_LOADING_SEPARATION_STRATEGY.md` ← **LOADING STRATEGY** 7. 📖 `doc/specifications/GRAPHSTORE_LOADING_QUICK_REFERENCE.md` ← **LOADING REFERENCE**

**INSPIRATION (when stuck)**: 8. 📖 `doc/philosophy/PROJECTION_AS_EVAL_CENTER.md` ← **"BE CAREFUL LOL"** 9. 📖 Java GDS source at `/home/pat/GitHub/graph-data-science/native-projection/`

### Execution Process

**When you execute TP-004**, follow this pattern:

1. **Start Phase** → Read plan for that phase
2. **Implement** → Code + tests
3. **Record** → Document what happened (deviations, surprises, test counts)
4. **Reflect** → What did we learn? What would we change?

**After Phase 8 Complete**, create:

```
doc/implementation/NATIVE_PROJECTION_ARROW_FACTORY.md
```

**Possible structure** (like Progress Tracking):

- Single file (if linear), OR
- Multi-phase (if complex): Phase 1, 2, 3...

**Content** (permanent record):

- What actually happened
- Test statistics
- Integration surprises
- Performance results
- Deviations from plan
- Lessons learned
- Template for future translations

---

## Why This Matters

### The Metaphysical Structure

**Java GDS Native Projection**:

- ~70 files, intricate architecture
- Neo4j-specific but architecturally sound
- "Well infused metaphysically speaking"

**rust-gds Arrow Factory**:

- Will be ~35 files (more consolidated)
- Arrow-specific but architecturally parallel
- Same patterns, different substrate

### The Permanent Process

**Translation Plan** → Strategic vision  
**Implementation** → Tactical execution  
**Implementation Record** → Strategic feedback  
**Next Translation Plan** → Informed by record

**Loop continues**:

- TP-004 (Native Projection) → Implementation → Record
- TP-005 (next subsystem) → **INFORMED BY** TP-004 record
- TP-006 (next next) → **INFORMED BY** both
- ...

### Supporting Three Versions

**Java GDS**: Archetypal (reference implementation)  
**TypeScript GDS**: First emanation (learning ground)  
**rust-gds**: Second emanation (**with NEW THINGS**)

**All three inform each other**:

- Java teaches patterns
- TypeScript validates translation
- Rust innovates (zero-copy, traits, ownership)
- Records preserve discoveries

---

## Your Next Steps

### Right Now (Setup)

1. **Read TP-004** (translation plan) - 32KB, comprehensive
2. **Read NATIVE_PROJECTION_ARROW_DESIGN** (design doc) - Architecture vision
3. **Study PropertyMappings** (already open!) - Integration point
4. **Review GAMMA roadmap** - Month plan context

### When Ready (Execution)

1. **Start Phase 1** (Core Infrastructure)
2. **Code + Test** (factory.rs, config.rs, mod.rs)
3. **Record progress** (notes, surprises, test counts)
4. **Iterate phases 2-8**

### After Complete (Record)

1. **Create Implementation Record** (doc/implementation/NATIVE_PROJECTION_ARROW_FACTORY.md)
2. **Document learnings** (what worked, what surprised, what to change)
3. **Template for future** (this informs all subsequent translations)

---

## The Beautiful Pattern

### What We Have Now (Example: Progress Tracking)

**TRANSLATION**: (would have been TP-003 if we numbered it)

- Progress Tracking translation notes scattered in implementation docs
- No formal translation plan (was organic/exploratory)

**IMPLEMENTATION**: ✅ NOW CURATED!

- `PROGRESS_TRACKING_README.md` (overview)
- `PROGRESS_TRACKING_PHASE1_FOUNDATION.md` (storage)
- `PROGRESS_TRACKING_PHASE2_HIERARCHY.md` (tasks)
- `PROGRESS_TRACKING_PHASE3_LOGGING.md` (logging)
- Total: 188 tests, ~6000 lines, neat permanent record

### What We'll Have (Native Projection)

**TRANSLATION**: ✅ ALREADY EXISTS!

- `doc/translation/TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` (32KB, approved)
- Strategic plan, file mapping, 8 phases

**IMPLEMENTATION**: ⏸️ WILL BE CREATED

- `doc/implementation/NATIVE_PROJECTION_ARROW_FACTORY.md` (or multi-phase)
- Execution record, test counts, learnings
- Neat permanent record

**Future translations**: Informed by BOTH!

---

## Questions for Setup

Before you start, consider:

1. **Timing**: GAMMA month (October 2025), make-or-break
2. **Scope**: 24-30 hours, 8 phases, critical path
3. **Integration**: PropertyMappings, eval macro, GraphStore, Loading
4. **Recording**: Take notes during execution for implementation record
5. **Learning**: This is rev eng discovery, permanent process

**Are you ready?** 🚀

**Do you want to**:

- Review the translation plan (TP-004)?
- Study the design doc (NATIVE_PROJECTION_ARROW_DESIGN)?
- Read GAMMA roadmap?
- Jump into Phase 1?

---

**Status**: ✅ Setup document complete - Ready for GAMMA work!

**The Plan is the Plan. The Implementation will teach us. The Record will guide us.** 🕉️
