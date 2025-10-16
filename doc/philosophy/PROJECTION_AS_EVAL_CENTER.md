# Projection as the Center of Eval - A Warning & Recognition

**Document Type**: Philosophical Insight  
**Date**: October 15, 2025  
**Context**: User warning after TP-004 approval  
**Status**: 🚨 Recognition of Emergent Centrality

---

## 🕉️ Membership Protocol (Fichte's Method)

**This document places itself within the rust-gds Encyclopedia as**:

- **Location**: `doc/philosophy/PROJECTION_AS_EVAL_CENTER.md`
- **Category**: Philosophy (Conceptual Foundations)
- **Related**: `PROJECTION_FUNNY_BUSINESS.md`, `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md`
- **Warning**: Architectural complexity concentration risk
- **Insight**: Eval macro system + Factory system → Projection becoming central bottleneck

---

## The Warning

> "but you see this Projection package is becoming the Center of Eval! be careful LOL"

**Context**: After approving TP-004 (Native Projection → Arrow Factory translation plan)

**What the user saw**:

- `projection/codegen/` - Eval macro system (value_type_table!, functors, form processor)
- `projection/factory/` - Arrow-native factory (proposed, entry point for ALL data)
- `projection/traits/` - Projection API (ElementProjection, PropertyMapping)
- `projection/impls/` - Implementations (PropertyMappings currently open in editor)
- `projection/native/` - ML/execution pipelines

**The realization**: Projection is becoming the **center of everything**!

---

## What "Center of Eval" Means

### 1. Eval as Evaluation (Runtime)

**Projection IS the Eval center**:

- ✅ **Data ingestion** (Factory system - ALL external data enters here)
- ✅ **Type conversion** (Eval macro - value_type_table!, functors)
- ✅ **Property mapping** (PropertyMappings - column → property)
- ✅ **Schema transformation** (Arrow schema → GraphStore schema)
- ✅ **ML pipelines** (native/ml/ - algorithm execution)
- ✅ **Code generation** (codegen/ - compile-time DSL)

**Everything flows through Projection**:

```text
External Data → Factory (projection/factory/)
              → Schema mapping (projection/traits/)
              → Type conversion (projection/codegen/eval_macro)
              → Property assignment (projection/impls/)
              → GraphStore population
              → ML execution (projection/native/ml/)
```

### 2. Eval as eval! Macro (Compile-time)

**Projection CONTAINS the eval system**:

- `projection/codegen/eval_macro.rs` - The value_type_table! DSL
- `projection/codegen/functors.rs` - GrossToSubtle / SubtleToGross conversions
- `projection/codegen/form_processor.rs` - Policy surface (widening, registry)

**This IS the eval center** - compile-time code generation for the entire codebase!

---

## Why This is Dangerous (The Warning)

### 1. **Single Point of Failure**

**If Projection breaks, EVERYTHING breaks**:

- ❌ Can't load data (Factory)
- ❌ Can't convert types (Eval macro)
- ❌ Can't map properties (PropertyMappings)
- ❌ Can't run algorithms (ML pipelines)

**Projection is now a bottleneck** for the entire system!

### 2. **Cognitive Complexity**

**Too many responsibilities in one module**:

- Data ingestion (Factory)
- Schema mapping (Traits)
- Type system (Eval macro)
- Property management (Impls)
- ML execution (Native)
- Code generation (Codegen)

**This violates separation of concerns**!

### 3. **Maintenance Nightmare**

**Changes ripple everywhere**:

- Change Factory → affects data loading
- Change eval macro → recompile everything
- Change PropertyMapping → affects all projections
- Change ML pipelines → affects algorithm performance

**Too many moving parts in close proximity**!

### 4. **Onboarding Difficulty**

**New developers must understand**:

- Projection API (traits)
- Factory system (arrow)
- Eval macro system (codegen)
- Property mappings (impls)
- ML pipelines (native)

**Steep learning curve** for contributing!

---

## Historical Parallel: Neo4j GDS

### Java GDS Structure

**Neo4j GDS keeps these SEPARATE**:

```
core/                    - Core graph abstractions
graph-projection-api/    - Projection interfaces (SEPARATE!)
native-projection/       - Neo4j-specific factory (SEPARATE!)
algorithms/              - Algorithm implementations (SEPARATE!)
executor/                - Execution runtime (SEPARATE!)
```

**Each has clear boundaries and responsibilities!**

### rust-gds Current Structure

**rust-gds MERGES them**:

```
projection/
├── traits/       (≈ graph-projection-api)
├── impls/        (≈ graph-projection-api)
├── factory/      (≈ native-projection) ← NEW, adding to pile
├── codegen/      (≈ ???) ← UNIQUE TO RUST-GDS
├── native/       (≈ algorithms + executor) ← CONFUSING NAME
└── ...
```

**Everything under one roof** - convenience OR danger?

---

## Why It Happened (Architectural Drift)

### Phase 1: Projection API (Original)

**Scope**: Traits and implementations for projecting graphs
**Files**: `traits/`, `impls/`
**Purpose**: Define what a projection IS

### Phase 2: Eval Macro System (Addition)

**Scope**: Compile-time code generation for property types
**Files**: `codegen/`
**Purpose**: Generate type conversions at compile time
**Why here?**: "Projection deals with properties, eval macro generates property code"

### Phase 3: ML Pipelines (Addition)

**Scope**: Algorithm execution and form processing
**Files**: `native/`
**Purpose**: Run ML algorithms
**Why here?**: "Algorithms work on projections, native execution is local"

### Phase 4: Factory System (Proposed)

**Scope**: Data ingestion from external sources
**Files**: `factory/` (TP-004)
**Purpose**: Entry point for ALL external data
**Why here?**: "Factory creates projections, projections are in projection/"

**Each addition made sense in isolation, but together they create centralization!**

---

## The "be careful LOL" Decoded

**The LOL is not dismissive** - it's recognition of **inevitable complexity**!

**Translation**:

> "I see what's happening. Projection is becoming the heart of the system. This is both:
>
> - **Necessary** (Projection IS the core abstraction)
> - **Dangerous** (Too much responsibility in one place)
>
> Be careful you don't make it unmaintainable. But I know you know this. Hence LOL."

**The warning is loving** - like a master warning an apprentice about a difficult technique!

---

## What to Be Careful About

### 1. **Module Boundaries**

**Keep internal structure clear**:

- `projection/traits/` - PUBLIC API (stable)
- `projection/impls/` - IMPLEMENTATIONS (can change)
- `projection/factory/` - ENTRY POINTS (extension points)
- `projection/codegen/` - COMPILE-TIME (magical, fragile)
- `projection/native/` - EXECUTION (performance-critical)

**Document what depends on what!**

### 2. **Avoid Circular Dependencies**

**Current risk**:

- Factory depends on PropertyMappings
- PropertyMappings depends on PropertyMapping (trait)
- PropertyMapping depends on eval macro (for types)
- Eval macro generates code for Factory

**Solution**: Keep dependency graph acyclic (DAG)!

### 3. **Clear Extension Points**

**Make it obvious where to add new features**:

- New data source? → `projection/factory/`
- New property type? → `projection/codegen/value_type_table!`
- New algorithm? → `projection/native/ml/`
- New projection type? → `projection/traits/`

**Extension points are the AI entry points!**

### 4. **Comprehensive Documentation**

**Because complexity is inevitable**:

- Module-level README.md (existing! ✅)
- Architecture diagrams (missing!)
- Dependency graphs (missing!)
- Onboarding guide (missing!)

**The more complex, the more docs needed!**

---

## Counterargument: Why Centralization is GOOD

### 1. **Projection IS the Core Abstraction**

**Everything in GDS is about projections**:

- Load data → Project into graph form
- Map properties → Project columns to properties
- Run algorithms → Project graph through computation
- Export results → Project internal to external format

**Projection is not just ONE thing - it's THE thing!**

### 2. **Rust Enables Safe Complexity**

**Features that make centralization manageable**:

- ✅ **Strong types** - Can't mix up abstractions
- ✅ **Traits** - Clear contracts between layers
- ✅ **Modules** - Enforced boundaries
- ✅ **Compile-time checks** - Errors caught early

**Rust makes complex systems tractable!**

### 3. **Eval Macro REQUIRES Centralization**

**Compile-time code generation needs**:

- Access to type definitions (value_type_table!)
- Access to property mappings (PropertyMapping trait)
- Access to storage descriptors (StorageHint)
- Coordination across layers

**Can't be decentralized - it's literally generating code FOR other modules!**

### 4. **Single Source of Truth**

**Benefits**:

- ✅ One place to look for projection logic
- ✅ Consistent patterns across features
- ✅ Easier to reason about entire system
- ✅ No duplicate abstractions

**Centralization can be a feature, not a bug!**

---

## The Middle Path (Pragmatic Approach)

### Accept Centralization, Manage Complexity

**Don't fight it**:

- Projection WILL be central (it's the core abstraction)
- Factory SHOULD be in projection/ (creates projections)
- Eval macro SHOULD be in projection/ (generates property code)

**But manage it**:

1. **Clear internal structure** (subdirectories with clear purposes)
2. **Strong boundaries** (traits between layers)
3. **Extensive documentation** (README.md in every submodule)
4. **Explicit dependencies** (no hidden coupling)
5. **Extension points** (AI entry points clearly marked)

### Trust the Process (Pre-Prim Strategy)

**Current state**: Pre-Prim 0.0.x (structure complete, implementation deferred)

**This means**:

- ✅ Design the structure now (TP-004 approved!)
- ⏸️ Defer implementation (wait for return)
- 🔍 Observe patterns (see what emerges)
- 🔄 Refactor if needed (when Prim 0.1.x)

**Pre-Prim allows architectural experimentation!**

---

## Action Items (Post-Execution)

### After TP-004 Execution

1. **Create Architecture Diagram**

   - Show all `projection/` subdirectories
   - Show dependencies between modules
   - Show data flow (external → GraphStore)
   - Highlight extension points

2. **Document Internal Structure**

   - README.md for `projection/factory/`
   - README.md for `projection/codegen/`
   - README.md for `projection/native/`
   - Update main `projection/README.md`

3. **Identify Refactoring Candidates**

   - Is `projection/native/` actually "execution"?
   - Should eval macro be top-level `codegen/`?
   - Should Factory be top-level `factory/`?
   - Document options, defer decision

4. **Create Onboarding Guide**
   - "Understanding rust-gds Projection System"
   - Start with simple: traits → impls
   - Then complex: eval macro → factory
   - Then advanced: ML pipelines

---

## Philosophical Reflection

### The Absolute Form's Necessity

**User insight**:

> "The Projector is our Absolute Form and is what we do for a living."

**This explains why centralization is inevitable**:

- Projection is not a module - it's the METHOD
- Not just code - it's the PRACTICE
- Not just abstraction - it's the WORK ITSELF

**Of course it's central - it's THE CENTER!**

### The Danger of the Absolute

**But Absolutes are dangerous**:

- ✅ Necessary (can't avoid)
- 🚨 Fragile (single point of failure)
- 📚 Complex (steep learning curve)
- 🔧 Powerful (enables everything)

**"Be careful LOL" = Recognition of this paradox!**

### The LOL as Wisdom

**The "LOL" is not casual** - it's:

- Recognition (I see what you're doing)
- Acceptance (This is inevitable)
- Warning (But watch out)
- Humor (Because what else can you do?)

**Dharmic Science in action**:

- Theory (Projection is central)
- Approval (Go ahead with TP-004)
- Warning (Be mindful of complexity)
- Trust (You'll figure it out)

---

## Conclusion

**The Warning**: Projection is becoming the Center of Eval
**The Reality**: Projection IS the Center (by necessity)
**The Challenge**: Manage complexity without losing power
**The Strategy**: Clear boundaries + strong types + extensive docs
**The Philosophy**: Accept the Absolute, but be mindful of its weight

**Safe travels!** When you return, we'll **prove** the theory through execution! 🙏✨

---

## Status

**Recognition**: ✅ Projection centrality acknowledged  
**Warning**: 🚨 Complexity concentration risk documented  
**Strategy**: 📋 Management approach defined  
**Next**: ⏸️ Await return for TP-004 execution

---

_"The Absolute Form is central by necessity, dangerous by nature, and manageable by design."_

🕉️ **Tat Tvam Asi** - Projection IS That! 🚀
