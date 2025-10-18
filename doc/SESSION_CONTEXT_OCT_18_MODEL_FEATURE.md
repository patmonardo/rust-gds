# Session Context: October 18, 2025 - Model & Feature API Translation

**Status**: Starting isolated 1:1 Java translation (PageRank deferred due to Executor/Algorithm architectural complexity)

**Session Goal**: Build Model and Feature APIs following 1:1 Java GDS translation without cross-cutting concerns

---

## Why This Approach?

### ✅ Advantages of Starting with Model/Feature APIs

- **Isolated**: No dependencies on Executor, Algorithm, or Pregel
- **Clean**: 1:1 translation from Java GDS source
- **Self-contained**: No cross-cutting architectural questions
- **Progressive**: Builds platform understanding incrementally

### ❌ Why PageRank Was Blocked

```
AlgorithmSpec::execute() signature:
    fn execute<G: GraphStore>(&self, graph_store: &G, ...)
                              ^^^^

Pregel needs Graph API (degree, stream_relationships, edge iteration)
GraphStore provides CRUD (node_count, node_labels, properties)

MISMATCH:
- Executor passes &G: GraphStore (CRUD interface)
- PageRank needs &G: Graph (analytic interface)
- These compose differently than Java
- Architectural confusion → blocking work

DECISION: Return to this when full platform comprehension exists
```

---

## Current Codebase State (Oct 17 snapshot)

### ✅ Completed

- `src/types/` - Core property graph types
- `src/pregel/` - Pregel framework (computation, messaging, execution)
- `src/projection/` - GraphStore projection system
- `src/config/` - Configuration system
- `src/ml/` - Partial ML pipeline infrastructure
- `src/procedure/algo/sum/` - Reference algorithm (working)

### ⏸️ In Progress/Blocked

- `src/procedure/algo/pagerank/` - Partially implemented (blocked on Executor/Algorithm)
- `src/ml/model/` - Needs implementation (Model API)
- `src/ml/feature/` - Needs implementation (Feature API)

### ❌ Not Started

- Model trait and implementations
- Feature extraction system
- Feature composition operators
- Training pipeline integration

---

## Translation Discipline (Copilot Instructions)

**Remember**:

> Translate EXACTLY what is in the source file. Do not add helpful extensions, convenience implementations, or any other additions unless explicitly requested. A translation request means a literal 1:1 mapping of the source material to idiomatic Rust.

### Process

1. User points to Java GDS source file
2. I read it completely
3. I translate 1:1 to Rust (idiomatic, not mechanical)
4. I ask clarifying questions ONLY if blocked
5. No assumptions, no architecture innovation

---

## Next Steps

### Ready to Start

User will provide Java GDS source file paths for:

1. Model trait definition
2. Feature trait definition
3. Feature extraction implementations
4. Feature composition operators

### File Locations (Expected)

```
Java GDS:
/home/pat/GitHub/graph-data-science/ml/src/main/java/org/neo4j/gds/ml/model/
/home/pat/GitHub/graph-data-science/ml/src/main/java/org/neo4j/gds/ml/features/

Rust Target:
src/ml/model/ (trait, implementations)
src/ml/feature/ (extraction, composition)
```

---

## Reference: Architecture Overview

### Layers (From Most Isolated to Most Cross-Cutting)

```
MOST ISOLATED (start here):
├─ Model & Feature APIs  ← We are here
│  (self-contained value types and extractors)
│
├─ ML Pipeline
│  (orchestrates models, features, training)
│
├─ Procedure (Algorithm) System
│  (PageRank, other graph algorithms)
│  - Uses Pregel for execution
│  - Bridges to GraphStore/Graph
│
MOST CROSS-CUTTING (return here later)
└─ Executor & AlgorithmSpec
   (connects everything: config → validation → execution → result)
```

### Why Model/Feature First

- Zero dependencies upward (nothing depends on them yet)
- Clear Java reference (canonical implementation)
- Builds understanding of platform patterns
- Can be integrated into pipeline when ready

---

## Key References

### Documentation

- **START_HERE.md** - Master navigation
- **TRANSLATION_WORKFLOW_TEMPLATE.md** - How to plan translations
- **adr0007_translation_plan_protocol.md** - Documentation structure
- **java_gds_source_map.md** - Java GDS package → Rust module mapping

### Code

- **Sum algorithm** (`src/procedure/algo/sum/spec.rs`) - Reference for simple algorithm structure
- **Config system** (`src/config/`) - Pattern for configuration traits
- **ML pipeline** (`src/ml/`) - Where Model/Feature will integrate

---

## Vocabulary (Session-Specific)

**Prakasa** - Illumination phase (planning, analysis)  
**Kriya** - Action phase (implementation)  
**Krama** - Recording phase (documentation)  
**1:1 Translation** - Exact Java → Rust mapping (idiomatic, no extensions)  
**Cross-cutting Concern** - Something affecting multiple layers (Executor is one)  
**Isolated Translation** - Self-contained, no upstream dependencies (Model/Feature)

---

## Ready Signal

✅ Codebase compiles  
✅ Documentation scavenged and organized  
✅ Copilot discipline reset (1:1 translation mode)  
✅ Isolated translation scope (Model/Feature APIs)  
✅ Waiting for Java GDS source file pointers

**Awaiting next instruction** → User provides Java GDS Model/Feature source file paths

---

**Last Updated**: October 18, 2025  
**Compiled**: ✅ Yes  
**Ready**: ✅ Yes (awaiting Java source files)
