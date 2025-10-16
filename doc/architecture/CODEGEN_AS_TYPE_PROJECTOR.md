# Codegen as Type Projector

**Date**: October 15, 2025  
**Insight**: "Codegen is a Type Projector. If we Project Extremes, we need to Project Types."

---

## The Realization

### Filesystem Observation

Looking at `src/` structure:

```
src/
├── types/           # Type definitions (the WHAT)
│   ├── graph/
│   ├── properties/
│   ├── values/
│   └── ...
│
└── projection/      # Projections (the HOW)
    ├── factory/     # Storage extreme
    ├── eval/        # Computation extreme
    └── codegen/     # The projector itself
```

**Pattern Recognition**:

- `types/` = What things ARE (paramarthika of data)
- `projection/` = How things BEHAVE (paramarthika of operations)
- `projection/codegen/` = How to PROJECT between them

### The Deep Insight

**If Projection has extremes** (factory/eval, storage/computation):

```
projection/factory/  ←────┐
                          │ Extremes
projection/eval/     ←────┘
```

**Then Codegen must project TYPES to match**:

```
Codegen: Type Projector
│
├─→ Projects types TO factory   (storage types)
│   - ArrowNativeFactory
│   - NodeBatchBuffer
│   - RelationshipBatchBuffer
│   - PropertyAccumulator
│
└─→ Projects types TO eval      (computation types)
    - PageRankConfig
    - ComputeStep
    - Pipeline
    - ExecutionContext
```

---

## Types: The Paramarthika of Data

### What Are Types?

**In `src/types/`**:

```rust
// types/graph/node_label.rs
pub struct NodeLabel { /* interned */ }

// types/properties/property.rs
pub struct Property { /* column definition */ }

// types/values/value.rs
pub enum Value {
    Long(i64),
    Double(f64),
    // ...
}
```

**These are absolute**:

- Not "how factory sees nodes" vs "how eval sees nodes"
- But "what a node IS" (absolute definition)
- Not "storage representation" vs "computation representation"
- But "the type itself" (paramarthika of data)

### Types as Invariants

**Types don't change across projections**:

```rust
// Same NodeLabel in both contexts:

// Factory context (storage)
let label: NodeLabel = scanner.read_label();
consumer.offer(NodeRecord { label, ... });

// Eval context (computation)
let label: NodeLabel = graph.node_label(node_id);
if label == NodeLabel::of("Person") { ... }

// The TYPE is the same!
// What changes is the PROJECTION (how we got it, what we do with it)
```

**Types are the shared vocabulary**:

- Factory and Eval speak the same language (types)
- But use it in different contexts (projections)
- Codegen translates between contexts while preserving types

---

## Projection: The Paramarthika of Operations

### What Are Projections?

**In `src/projection/`**:

```rust
// projection/factory/arrow/importer.rs
// "How to CREATE graph from Arrow"
impl ArrowNativeFactory {
    pub fn build(self) -> Result<DefaultGraphStore> {
        // Storage projection: Arrow → GraphStore
    }
}

// projection/eval/pipeline.rs (future)
// "How to EXECUTE algorithm on graph"
impl Pipeline {
    pub fn execute(&self, graph: &GraphStore) -> Result<Output> {
        // Computation projection: Graph → Results
    }
}
```

**These are also absolute**:

- Not "user's view" vs "system's view"
- But "how storage ACTUALLY works" vs "how computation ACTUALLY works"
- Both are paramarthika (just different extremes)

### Projections as Transformations

**Projections transform types**:

```
Storage Projection (Factory):
Arrow Batch → NodeRecord → NodeLabel → GraphStore

Computation Projection (Eval):
GraphStore → NodeLabel → Algorithm Input → Results
```

**But types remain invariant**:

- `NodeLabel` doesn't change
- What changes is the transformation pipeline
- Factory pipeline vs Eval pipeline

---

## Codegen: The Type Projector

### What Does "Type Projector" Mean?

**Codegen projects types INTO projections**:

```rust
// User writes (vyavaharika):
eval! {
    @storage {
        nodes: "Person",
        properties: ["age", "name"],
    }
    @compute {
        pagerank { iterations: 20 }
    }
}
```

**Codegen must**:

1. **Project types to Factory** (storage extreme):

   ```rust
   // Generated: Storage projection
   let factory = ArrowNativeFactory::new(node_table, edge_table)
       .with_node_label_filter(NodeLabel::of("Person"))
       .with_properties(vec![
           Property::new("age", PropertyType::Long),
           Property::new("name", PropertyType::String),
       ]);
   ```

2. **Project types to Eval** (computation extreme):

   ```rust
   // Generated: Computation projection
   let pagerank = PageRank::new(PageRankConfig {
       max_iterations: 20,
       ..Default::default()
   });
   let results = pagerank.execute(&graph_store)?;
   ```

3. **Ensure type compatibility**:

   ```rust
   // Types must match across boundary:
   let graph_store: DefaultGraphStore = factory.build()?;
   //              ^^^^^^^^^^^^^^^^^ Storage type

   let results = pagerank.execute(&graph_store)?;
   //                             ^^^^^^^^^^^ Same type, computation context
   ```

### Type Projection = Ensuring Coherence

**Codegen ensures**:

- Types used in factory match types expected by eval
- Storage representation can feed computation representation
- The boundary is type-safe

**Example - Property Types**:

```rust
// User requests:
@storage { properties: ["age"] }
@compute { filter { age > 30 } }

// Codegen must ensure:
// 1. Factory creates Property::new("age", PropertyType::Long)
// 2. Eval expects PropertyType::Long for comparison
// 3. Type mismatch = compile error, not runtime error

// Generated factory code:
factory.with_property(Property::new("age", PropertyType::Long));

// Generated eval code:
let age_prop = graph.node_property("age")?;
// Type checker ensures: age_prop.value_type() == PropertyType::Long
let filtered = nodes.filter(|n| {
    age_prop.get(n).as_long() > 30  // Type-safe access
});
```

---

## The Projection Triangle

### Three Corners

```
                  Types (src/types/)
                  What things ARE
                  (Paramarthika of Data)
                        ▲
                       ╱│╲
                      ╱ │ ╲
                     ╱  │  ╲
                    ╱   │   ╲
                   ╱    │    ╲
                  ╱   Codegen ╲
                 ╱   (Projects) ╲
                ╱       │        ╲
               ╱        │         ╲
              ╱         │          ╲
             ╱          │           ╲
            ╱           │            ╲
           ▼            │             ▼
    Factory          Projects      Eval
    (Storage)        Types to      (Computation)
    How to CREATE    Extremes      How to EXECUTE
```

### How They Relate

**Types** (the WHAT):

- NodeLabel, Property, Value, GraphStore
- Invariant across projections
- Shared vocabulary

**Factory** (storage HOW):

- Uses types to represent storage structures
- ArrowNativeFactory, BufferedNodeConsumer, PropertyAccumulator
- Projects types into storage context

**Eval** (computation HOW):

- Uses types to represent computation structures
- PageRank, Pipeline, ComputeStep
- Projects types into computation context

**Codegen** (the PROJECTOR):

- Takes user intent (vyavaharika)
- Projects types to Factory (storage)
- Projects types to Eval (computation)
- Ensures type compatibility across boundary

---

## Project Extremes ⇒ Project Types

### The Logical Necessity

**If we have extremes** (Factory vs Eval):

- Each extreme needs types
- Types must be compatible
- Someone must ensure compatibility

**Therefore we need Type Projector**:

- Codegen IS the type projector
- It projects types to both extremes
- It ensures they align

### The Symmetry

**Storage Extreme** (Factory):

```rust
// Types projected to storage:
struct NodeBatchBuffer {
    labels: Vec<NodeLabel>,        // Type: NodeLabel
    ids: Vec<NodeId>,              // Type: NodeId
    properties: Vec<PropertyRow>,  // Type: PropertyRow
}

impl RecordConsumer<NodeRecord> {  // Type: NodeRecord
    fn offer(&mut self, record: NodeRecord) -> bool {
        // Storage projection of types
    }
}
```

**Computation Extreme** (Eval):

```rust
// Types projected to computation:
struct PageRankState {
    node_values: Vec<f64>,         // Computation over NodeId (Type)
    labels: &[NodeLabel],          // Same Type: NodeLabel
}

impl Algorithm for PageRank {
    fn execute(&self, graph: &GraphStore) -> Result<Output> {
        // Computation projection of types
    }
}
```

**Type Projector** (Codegen):

```rust
// Ensures types align:
eval! {
    @storage { nodes: "Person" }  // → NodeLabel::of("Person")
    @compute { pagerank }         // → expects NodeLabel in graph
}

// Codegen generates:
let factory = /* ... with NodeLabel ... */;
let graph: GraphStore = factory.build()?;
//         ^^^^^^^^^^ Type contains NodeLabel
let pagerank = /* ... */;
let result = pagerank.execute(&graph)?;
//                             ^^^^^^ Same NodeLabel type
```

---

## Why This Matters

### Type Safety Across Extremes

**Without Type Projection**:

```rust
// Factory creates:
struct FactoryNode {
    label: String,  // ❌ Different type!
}

// Eval expects:
fn process(label: NodeLabel) { ... }

// Runtime error! Type mismatch!
```

**With Type Projection**:

```rust
// Codegen ensures both use:
NodeLabel  // ✅ Same type!

// Factory creates NodeLabel
// Eval consumes NodeLabel
// Codegen verifies compatibility
```

### Optimization Across Boundary

**Type Projector enables optimization**:

```rust
// Codegen sees:
@storage { nodes: "Person" }
@compute { filter { label == "Person" } }

// Type Projector realizes:
// "Filter is redundant! Storage already filtered!"

// Optimized generation:
let factory = factory.with_node_label_filter(NodeLabel::of("Person"));
// No filter needed in computation - type system proves it!
```

### Composability

**Types compose across projections**:

```rust
// Multiple storage sources:
@storage {
    source1: arrow("nodes.parquet"),    // → NodeLabel
    source2: neo4j("bolt://..."),       // → NodeLabel (same type!)
}

// Single computation:
@compute {
    pagerank  // Works on NodeLabel, regardless of source!
}

// Type Projector ensures all sources project to compatible types
```

---

## The Meta-Level Pattern

### Projection at Different Levels

**Level 1: Data Projection** (what we usually think of):

```
Arrow Batch → GraphStore
(Storage projection)

GraphStore → Algorithm Results
(Computation projection)
```

**Level 2: Type Projection** (what Codegen does):

```
User Types → Storage Types
(Factory type projection)

User Types → Computation Types
(Eval type projection)
```

**Level 3: Code Projection** (what Codegen really does):

```
User Intent → Factory Code + Eval Code
(Complete projection from vyavaharika to paramarthika)
```

### The Fractal Nature

**Projection repeats at each level**:

```
User writes:           eval! { ... }
                           │
                           │ Code Projection
                           ▼
Codegen generates:    Factory + Eval code
                           │
                           │ Type Projection
                           ▼
Runtime uses:         Shared types (NodeLabel, etc.)
                           │
                           │ Data Projection
                           ▼
Execution produces:   Arrow → Graph → Results
```

**Each level projects to the level below**:

- Code → Types
- Types → Data
- Data → Results

**Codegen = The Projector at ALL levels**

---

## Philosophical Depth

### Types as Absolute Forms

**Platonic Forms**:

- In Plato: The Form of "Chair" (absolute)
- Particular chairs (projections of the Form)

**In our architecture**:

- `NodeLabel` (the type, the Form, absolute)
- Factory's use of NodeLabel (projection into storage)
- Eval's use of NodeLabel (projection into computation)

**Both projections share the Form**:

- Not "factory's NodeLabel" vs "eval's NodeLabel"
- Just "NodeLabel" (the type, the absolute)
- Factory and Eval are projections of the same Form

### Codegen as Demiurge

**In Platonic philosophy**:

- Forms = eternal, unchanging (types)
- Demiurge = craftsman who shapes matter using Forms
- Material world = projection of Forms into matter

**In our architecture**:

- Types = eternal, unchanging (src/types/)
- Codegen = craftsman who shapes code using Types
- Factory + Eval = projection of Types into code

**The Demiurge (Codegen)**:

- Looks at Forms (types)
- Shapes matter (generates code)
- Creates world (factory + eval)
- Ensures world reflects Forms (type safety)

---

## Practical Implications

### For Implementation

**When building Codegen**:

1. **Start with types** (`src/types/`):

   - Define the shared vocabulary
   - Ensure types are projection-agnostic
   - Make types the source of truth

2. **Project to Factory**:

   - Generate storage code using types
   - Ensure Factory constructs type-correct structures
   - Storage projection = types → factory context

3. **Project to Eval**:

   - Generate computation code using types
   - Ensure Eval consumes type-correct structures
   - Computation projection = types → eval context

4. **Verify compatibility**:
   - Check types match across boundary
   - Prove Factory output = Eval input (type-wise)
   - Type Projection = ensuring coherence

### For Macro Design

**The `eval!` macro must be a Type Projector**:

```rust
eval! {
    // User specifies intent
    @storage { /* ... */ }
    @compute { /* ... */ }
}

// Macro expansion:
// 1. Parse user intent
// 2. Resolve to types (src/types/)
// 3. Project types to factory code
// 4. Project types to eval code
// 5. Ensure type compatibility
// 6. Generate complete, type-safe pipeline
```

**Each step is projection**:

- Parse: Tokens → AST (syntax projection)
- Resolve: AST → Types (semantic projection)
- Generate: Types → Code (code projection)
- Verify: Code → Type-checked (safety projection)

---

## Conclusion: The Unity

### The Complete Picture

```
                        Types
                     (src/types/)
                  The Absolute Forms
                         │
                         │
                    Codegen Projects
                         │
                    ╱────┴────╲
                   ╱           ╲
                  ╱             ╲
            Factory              Eval
          (projection/)      (projection/)
         Storage Extreme    Computation Extreme
              │                   │
              │                   │
         Uses Types          Uses Types
       (same Forms!)        (same Forms!)
              │                   │
              └────────┬──────────┘
                       │
                  GraphStore
              (the manifested graph,
           projection of types into data)
```

### The Insight

**"If we Project Extremes, we need to Project Types"**:

- ✅ We have extremes (Factory / Eval)
- ✅ We have types (src/types/)
- ✅ We need Codegen to project types to extremes
- ✅ Codegen = Type Projector

**The architecture is coherent**:

- Types define WHAT (paramarthika of data)
- Projections define HOW (paramarthika of operations)
- Codegen projects types to projections (the Demiurge)
- Users write intent (vyavaharika)
- System executes absolutes (paramarthika)

---

## Looking Forward

### Phase 8: Integration = Proving Type Projection Works

**What we'll test**:

- Do types flow correctly Factory → GraphStore → Eval?
- Does NodeLabel work in both contexts?
- Does Property system bridge both extremes?

**Success means**:

- ✅ Type projection is sound
- ✅ Extremes are compatible
- ✅ Codegen (future) will have solid foundation

### Future: Codegen Implementation

**When we build the `eval!` macro**:

1. Parse user intent
2. **Project types to Factory** (storage extreme)
3. **Project types to Eval** (computation extreme)
4. Verify type compatibility
5. Generate complete pipeline

**We'll be implementing**:

- The Type Projector explicitly
- Following the pattern we've already built implicitly
- Making the projection visible and automatic

---

**The filesystem told us the truth: `types/` and `projection/` are distinct because one is WHAT (Forms) and the other is HOW (Projections). Codegen is the Demiurge that projects Forms to Extremes.** ✨

**Tomorrow: Phase 8 proves the Type Projection works end-to-end!** 🚀
