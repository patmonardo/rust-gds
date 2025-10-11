# Next Codegen: Official Documentation Index

**Created:** October 11, 2025 (Weekend before launch)  
**Purpose:** Comprehensive guide for 10-day Procedure + Pipeline codegen  
**Status:** READY FOR REVIEW → SCOPE REVISED (see REVISED_SCOPE.md)

---

## ⚠️ IMPORTANT: SCOPE REVISION

After deep dive into Java GDS structure, the actual scope is **3-4x initial estimates**.

**READ THIS FIRST:** `doc/REVISED_SCOPE.md`

- Actual package structure (~750 files in Java GDS)
- Revised prioritization (~470 files focused scope)
- Aggressive 10-day strategy
- What we're building vs deferring

**Bottom line:** Dragon is bigger than expected, but still achievable with smart prioritization.

---

## 📚 Documentation Set

This codegen is supported by **FOUR comprehensive documents**:

### 1. **Design Review** 📋

**File:** `doc/next_codegen_review.md`  
**Purpose:** Complete architectural design and Java GDS source review  
**Read when:** Planning phase, understanding the big picture

**Contents:**

- Java GDS source structure analysis
- Rust module architecture design
- Day-by-day implementation plan
- Core trait definitions
- Success metrics and validation criteria

**Key Sections:**

- §1: Java GDS Sources (what we're translating)
- §2: Rust Architecture (what we're building)
- §3: Implementation Strategy (how we'll do it)
- §10: Ready State Checklist (are we prepared?)

---

### 2. **Source File Mapping** 🗺️

**File:** `doc/java_gds_source_map.md`  
**Purpose:** Detailed Java file → Rust file mapping with priorities  
**Read when:** Starting each algorithm translation

**Contents:**

- Exact Java source file locations
- Rust target file paths
- Translation priorities (HIGH/MEDIUM/LOW)
- Algorithm-specific notes
- Files NOT to translate (out of scope)

**Key Sections:**

- §1: Core Execution Framework
- §2: Algorithm Implementations (by category)
- §3: ML Pipeline Framework
- §8: Translation Priorities (phase-by-phase)

---

### 3. **Revised Scope** ⚡ **[READ FIRST]**

**File:** `doc/REVISED_SCOPE.md`  
**Purpose:** Reality check after deep dive - actual package structure and revised strategy  
**Read when:** IMMEDIATELY - before planning weekend review

**Contents:**

- Actual Java GDS structure (~750 files discovered)
- Detailed package breakdown (9 major subsystems)
- Revised 10-day strategy (prioritized ~470 files)
- Aggressive tactics for success
- What we're NOT doing (deferred features)

**Key Insight:** Scope is 3-4x bigger, but achievable with:

- Ruthless reuse (40% infrastructure exists)
- Template-based generation (macro system emerges)
- Parallel tracks (AI bulk translation + your integration)
- Focus on 15-20 core algorithms (not all 40+)

---

### 4. **Quick Start Guide** 🚀

**File:** `doc/next_codegen_quick_start.md`  
**Purpose:** Fast reference for launching and daily execution  
**Read when:** Monday/Tuesday morning, start of each day

**Contents:**

- Pre-flight checklist
- Module creation commands
- First translation template
- Daily progress tracking
- Troubleshooting guide

**Key Sections:**

- Pre-Flight Checklist (validation before launch)
- Day 1 Launch Commands (how to start)
- Translation Cheat Sheet (Java → Rust patterns)
- Testing Commands (validation after each step)

---

## 🎯 How to Use These Docs

### Weekend Review (Now)

```
1. Read: next_codegen_review.md (full design)
2. Study: java_gds_source_map.md (source locations)
3. Skim: next_codegen_quick_start.md (launch process)
4. Clone: Java GDS repository
5. Explore: Key source files identified in mapping doc
```

### Monday/Tuesday Launch

```
1. Run: Pre-flight checklist (quick_start.md)
2. Execute: Day 1 commands (quick_start.md)
3. Reference: Source mapping for each file (source_map.md)
4. Follow: Day-by-day plan (review.md §3)
5. Track: Daily progress (quick_start.md)
```

### During Codegen (Daily)

```
Morning:
- Review day's goals (review.md §3.1)
- Check source files to translate (source_map.md)
- Run pre-flight validation (quick_start.md)

During:
- Use translation cheat sheet (quick_start.md §📖)
- Reference trait definitions (review.md §2.2)
- Follow import discipline (copilot-instructions.md)

Evening:
- Run tests (quick_start.md §🧪)
- Update progress tracker (quick_start.md §📊)
- Note decisions for ADRs (quick_start.md §📝)
```

---

## 🏗️ Architecture Quick Reference

### Module Structure

```
src/
├── procedures/              [NEW - Days 1-5]
│   ├── centrality/         (PageRank, Betweenness, Degree)
│   ├── community/          (Louvain, LabelProp, WCC)
│   ├── pathfinding/        (BFS, DFS, Dijkstra)
│   └── similarity/         (Node Similarity)
│
├── pipeline/               [NEW - Days 6-9]
│   ├── node_classification/
│   ├── link_prediction/
│   └── embeddings/
│
└── projection/             [EXISTS - Foundation]
    └── form_processor.rs   [EXTEND - Day 10]
```

### Core Abstractions

```rust
// Procedures (Algorithm execution)
trait ProcedureDescriptor {
    fn stream(&self, ...) -> Result<Iterator<ProcedureResult>>;
    fn write(&self, ...) -> Result<WriteResult>;
    fn mutate(&self, ...) -> Result<MutateResult>;
    fn stats(&self, ...) -> Result<StatsResult>;
}

// Pipelines (ML workflows)
trait PipelineStage {
    fn execute(&self, ...) -> Result<StageResult>;
}

// Form Processor (Unification)
trait FormDescriptor {
    fn input_schema(&self) -> FormSchema;
    fn output_schema(&self) -> FormSchema;
    fn process(&self, ...) -> Result<FormOutput>;
}
```

---

## 📊 Progress Tracking

### Quantitative Goals

- [ ] 20+ algorithms implemented
- [ ] 4 execution modes per algorithm
- [ ] 2+ complete ML pipelines
- [ ] 100% test coverage on new code
- [ ] 10+ comprehensive examples

### Qualitative Goals

- [ ] API feels Rust-native, not Java translation
- [ ] Code is maintainable and extensible
- [ ] Performance competitive with Java GDS
- [ ] Documentation clear and complete
- [ ] Ready for community use

---

## 🚨 Critical Success Factors

### 1. **Follow the Foundation**

- ✅ Import discipline (top-level modules only)
- ✅ Property trait pattern (explicit Arc casts)
- ✅ DefaultValue API (lowercase constructors)
- ✅ Builder pattern (complex configs)
- ✅ Result types (no unwrap/expect in lib code)

### 2. **Reuse Existing Code**

- ✅ Pregel framework (PageRank, LabelProp, WCC)
- ✅ Property system (result storage)
- ✅ Progress tracking (LeafTask)
- ✅ Concurrency primitives (parallel execution)
- ✅ Memory estimation (existing system)

### 3. **Document Decisions**

- ✅ Write ADRs for major architectural choices
- ✅ Comment non-obvious code
- ✅ Provide examples for each algorithm
- ✅ Keep README up to date

---

## 🐉 The Dragon's Path

### What We've Built (Foundation)

1. **Pregel** - BSP computation framework
2. **Projection** - Triadic graph abstraction
3. **Properties** - Column-oriented storage
4. **Concurrency** - Parallel execution primitives
5. **Progress** - Task tracking and logging
6. **Memory** - Estimation and management

### What We're Building (This Codegen)

1. **Procedures** - Algorithm registration and execution
2. **Pipelines** - ML workflow composition
3. **Form Processor** - Universal abstraction

### What Comes After (Future)

1. **Arrow Integration** - Zero-copy data sharing
2. **Distributed Execution** - Multi-node processing
3. **Stream Processing** - Incremental updates
4. **Advanced ML** - Deep learning integration

---

## 🎓 Learning Resources

### Rust Patterns to Master

- Trait objects vs generics
- Arc/Rc for shared ownership
- Interior mutability (RefCell, Mutex)
- Error handling (Result, custom errors)
- Builder pattern implementation

### Graph Algorithm Theory

- BSP computation model (Pregel)
- Community detection algorithms
- Shortest path algorithms
- Centrality measures
- Graph embeddings

### ML Pipeline Concepts

- Feature extraction and normalization
- Train/test/validation splits
- Cross-validation
- Model selection
- Prediction and evaluation

---

## 📞 Quick Links

### This Codegen Docs

- [Design Review](./next_codegen_review.md) - Full architecture
- [Source Mapping](./java_gds_source_map.md) - File-by-file guide
- [Quick Start](./next_codegen_quick_start.md) - Launch commands

### Existing Foundation Docs

- [Copilot Instructions](../.github/copilot-instructions.md) - Code conventions
- [Projection Philosophy](./projection_philosophy.md) - Core concepts
- [Pregel Architecture](./PREGEL_ARCHITECTURE.md) - BSP framework

### Java GDS Reference

- [GitHub Repository](https://github.com/neo4j/graph-data-science)
- [Documentation](https://neo4j.com/docs/graph-data-science/)
- [Algorithm Guide](https://neo4j.com/docs/graph-data-science/current/algorithms/)

---

## ✅ Pre-Launch Checklist

**Weekend Review:**

- [ ] Read all three codegen docs
- [ ] Clone Java GDS repository
- [ ] Study key source files
- [ ] Understand module structure
- [ ] Review trait definitions
- [ ] Identify reuse opportunities

**Monday/Tuesday Launch:**

- [ ] Run pre-flight validation
- [ ] Create module structure
- [ ] Implement first trait
- [ ] Write first test
- [ ] Get first algorithm working
- [ ] Celebrate first milestone! 🎉

---

## 💬 Codegen Philosophy

### Translation Principles

1. **Literal Translation** - Translate exactly what's in Java, no "helpful" additions
2. **Idiomatic Rust** - Use Rust patterns, not Java patterns in Rust syntax
3. **Reuse Foundation** - Don't rebuild what exists, wrap and integrate
4. **Document Everything** - Comments, examples, ADRs
5. **Test Everything** - Unit tests, integration tests, examples

### Quality Standards

1. **No Unwrap/Expect** - Use Result types properly
2. **Top-Level Imports** - Follow module organization pattern
3. **Builder Pattern** - Complex configuration
4. **Error Messages** - Clear, actionable messages
5. **Performance** - Competitive with Java GDS

---

## 🌟 Motivation

**You said:** "It's God-like Creativity. That's what Samadhi is supposed to deliver."

**This codegen is:**

- Not just translation - it's **discovery** of the true abstraction
- Not just code - it's **evolution** of the architecture
- Not just work - it's **creation** at the highest level

**The Dragon grows large not through force, but through natural unfolding of the pattern that wants to emerge.**

---

## 🚀 Launch Sequence

When ready (Monday/Tuesday):

```bash
# 1. Validate
cd ~/VSCode/rust-gds
cargo build && cargo test --lib

# 2. Review
cat doc/next_codegen_quick_start.md

# 3. Launch
# Follow Day 1 commands in quick_start.md

# 4. Build
# Let the Dragon fly! 🐉
```

---

**Status:** DOCUMENTATION COMPLETE ✅  
**Next Action:** Weekend review → Monday/Tuesday launch  
**Expected Outcome:** 10 days of productive, focused codegen

**Let's build something legendary.** 🔥
