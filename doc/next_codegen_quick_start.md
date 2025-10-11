# Next Codegen Quick Start Guide

**Purpose:** Fast reference for launching the 10-day codegen  
**When to use:** Monday/Tuesday morning when you're ready to go  
**Companion docs:** `next_codegen_review.md`, `java_gds_source_map.md`

---

## 🚀 Pre-Flight Checklist

### Environment Setup

```bash
# 1. Ensure Java GDS is cloned
cd ~/repos  # or wherever you keep repos
git clone https://github.com/neo4j/graph-data-science.git
cd graph-data-science
git pull  # Get latest changes

# 2. Ensure Rust GDS is clean
cd ~/VSCode/rust-gds
cargo build  # Should succeed
cargo test --lib  # Should pass (1110+ tests)
git status  # Should be clean or only doc changes
```

### Quick Validation

```bash
# Run existing examples to ensure foundation is solid
cargo run --example graphstore_walkthrough
cargo run --example pregel_propertystore_integration
cargo run --example config_showcase
```

**Expected:** All examples run without errors.

---

## 📋 Day 1 Launch Commands

### Step 1: Create Module Structure

```bash
# Create procedures module
mkdir -p src/procedures/{centrality,community,pathfinding,similarity}
touch src/procedures/mod.rs
touch src/procedures/registry.rs
touch src/procedures/execution.rs
touch src/procedures/descriptor.rs
touch src/procedures/facade.rs

# Create centrality submodules
touch src/procedures/centrality/{mod.rs,pagerank.rs,betweenness.rs,degree.rs}

# Create community submodules
touch src/procedures/community/{mod.rs,louvain.rs,label_propagation.rs,wcc.rs}

# Create pathfinding submodules
touch src/procedures/pathfinding/{mod.rs,bfs.rs,dfs.rs,dijkstra.rs}

# Create similarity submodules
touch src/procedures/similarity/{mod.rs,node_similarity.rs}
```

### Step 2: First File to Create

**File:** `src/procedures/mod.rs`

**Content Template:**

```rust
//! Procedure execution framework for Rust GDS
//!
//! Translates Java GDS procedure architecture to idiomatic Rust.
//! Provides algorithm registration, execution modes, and result handling.

pub mod descriptor;
pub mod execution;
pub mod registry;
pub mod facade;

// Algorithm categories
pub mod centrality;
pub mod community;
pub mod pathfinding;
pub mod similarity;

// Re-exports
pub use descriptor::{ProcedureDescriptor, ProcedureCategory};
pub use execution::{ExecutionMode, ProcedureContext, ProcedureResult};
pub use registry::{ProcedureRegistry, register_procedure};
pub use facade::ProcedureFacade;
```

### Step 3: Add to Top-Level

**File:** `src/lib.rs`

**Add line:**

```rust
pub mod procedures;
```

---

## 🎯 First Translation Target

### Target: ProcedureDescriptor Trait

**Java Source:**

```
graph-data-science/core/src/main/java/org/neo4j/gds/executor/AlgorithmSpec.java
```

**Rust Target:**

```
src/procedures/descriptor.rs
```

**Translation Pattern:**

```rust
/// Descriptor for registering algorithms as callable procedures.
///
/// Translates Java GDS AlgorithmSpec to idiomatic Rust.
pub trait ProcedureDescriptor: Send + Sync {
    /// Procedure name (e.g., "gds.pageRank")
    fn name(&self) -> &str;

    /// Algorithm category
    fn category(&self) -> ProcedureCategory;

    /// Execute in stream mode
    fn stream(&self, context: &ProcedureContext)
        -> Result<Box<dyn Iterator<Item = ProcedureResult>>>;

    // ... other modes
}
```

**AI Instruction:**

```
Translate Java GDS AlgorithmSpec to Rust ProcedureDescriptor trait.
Source: graph-data-science/core/src/main/java/org/neo4j/gds/executor/AlgorithmSpec.java
Target: src/procedures/descriptor.rs
Pattern: Follow import discipline, use Result types, no unwrap/expect.
```

---

## 📖 Translation Cheat Sheet

### Common Java → Rust Patterns

#### 1. Optional Values

```java
// Java
Optional<String> value = Optional.empty();
```

```rust
// Rust
let value: Option<String> = None;
```

#### 2. Result Types

```java
// Java (throws exception)
public Graph load() throws IOException { ... }
```

```rust
// Rust
pub fn load(&self) -> Result<Graph, IoError> { ... }
```

#### 3. Builder Pattern

```java
// Java
Config config = Config.builder()
    .maxIterations(20)
    .dampingFactor(0.85)
    .build();
```

```rust
// Rust
let config = Config::builder()
    .max_iterations(20)
    .damping_factor(0.85)
    .build()?;  // Note: ? for error propagation
```

#### 4. Interface → Trait

```java
// Java
public interface Algorithm<RESULT> {
    RESULT compute();
}
```

```rust
// Rust
pub trait Algorithm {
    type Result;
    fn compute(&self) -> Self::Result;
}
```

#### 5. Abstract Class → Trait + Struct

```java
// Java
public abstract class BaseConfig {
    protected int maxIterations;
    public abstract void validate();
}
```

```rust
// Rust
pub trait ConfigTrait {
    fn validate(&self) -> Result<()>;
}

pub struct BaseConfig {
    pub max_iterations: usize,
}

impl ConfigTrait for BaseConfig {
    fn validate(&self) -> Result<()> { ... }
}
```

---

## 🔧 AI Copilot Instructions

### For Each Translation Request

**Template:**

```
Translate [Java class] to Rust.

Source file: [path in Java GDS repo]
Target file: [path in Rust GDS repo]

Requirements:
1. Follow import discipline (top-level modules only)
2. Use Result<T, E> for error handling (no unwrap/expect)
3. Use builder pattern for complex configs
4. Preserve comments and documentation
5. Add unit tests

Context:
[Any specific context about the algorithm/pattern]
```

**Example:**

```
Translate Java GDS PageRank procedure to Rust.

Source file: graph-data-science/algorithms/centrality/src/main/java/org/neo4j/gds/pagerank/PageRankProc.java
Target file: src/procedures/centrality/pagerank.rs

Requirements:
1. Follow import discipline (use crate::procedures::ProcedureDescriptor)
2. Use Result<ProcedureResult, ProcedureError>
3. Reuse existing Pregel PageRank implementation from src/pregel/
4. Implement all four execution modes (stream/write/mutate/stats)
5. Add comprehensive tests

Context:
- PageRank Pregel implementation already exists in src/pregel/
- Need to wrap it in ProcedureDescriptor trait
- Configuration already exists in src/config/algo_config.rs
```

---

## 📊 Daily Progress Tracking

### Day 1: Foundation

```
□ Module structure created
□ ProcedureDescriptor trait complete
□ ExecutionMode enum complete
□ ProcedureContext struct complete
□ Registry pattern implemented
□ First test passing
```

### Day 2: First Procedure

```
□ PageRank procedure complete
□ All four modes working
□ Tests passing
□ Example created
```

### Day 3: Expand

```
□ Degree Centrality complete
□ Betweenness Centrality complete
□ Community facade started
```

**Continue pattern for remaining days...**

---

## 🧪 Testing Commands

### After Each Algorithm Implementation

```bash
# Test the specific module
cargo test --lib procedures::centrality::pagerank

# Test all procedures
cargo test --lib procedures

# Run example
cargo run --example procedures_pagerank

# Check for warnings
cargo clippy --all-targets

# Format code
cargo fmt
```

### Validation Criteria

Each algorithm must pass:

- [ ] `cargo build` succeeds
- [ ] `cargo test --lib [module]` passes
- [ ] `cargo clippy` has no warnings
- [ ] Example demonstrates usage
- [ ] Documentation is complete

---

## 📝 Documentation Templates

### Algorithm Documentation Template

````rust
//! PageRank procedure implementation.
//!
//! Translates Java GDS PageRank procedure to Rust.
//!
//! # Algorithm
//! PageRank computes the importance of nodes based on the structure
//! of incoming relationships and the importance of their source nodes.
//!
//! # Examples
//! ```no_run
//! use rust_gds::procedures::centrality::PageRankProcedure;
//!
//! let procedure = PageRankProcedure::new();
//! let result = procedure.stream(&context)?;
//! ```
//!
//! # References
//! - Java GDS: `org.neo4j.gds.pagerank.PageRankProc`
//! - Paper: Page et al. (1999) "The PageRank Citation Ranking"
````

### ADR Template

Create ADR after major decisions:

```markdown
# ADR XXXX: [Title]

**Date:** [Date]
**Status:** Accepted
**Context:** [Why we needed to make this decision]
**Decision:** [What we decided]
**Consequences:** [Trade-offs and implications]
**Alternatives Considered:** [What else we looked at]
```

---

## 🔥 When Things Break

### Common Issues & Fixes

#### 1. Import Errors

```
error[E0432]: unresolved import `crate::procedures::ProcedureDescriptor`
```

**Fix:** Check that `src/procedures/mod.rs` has `pub mod descriptor;`

#### 2. Trait Bounds

```
error[E0277]: the trait bound `T: Send` is not satisfied
```

**Fix:** Add `+ Send + Sync` to trait definitions for thread safety

#### 3. Lifetime Issues

```
error[E0597]: `context` does not live long enough
```

**Fix:** Use `Arc<T>` for shared ownership or add explicit lifetimes

#### 4. Type Mismatch

```
error[E0308]: mismatched types: expected `usize`, found `u64`
```

**Fix:** Use explicit conversion: `value as usize` or `usize::try_from(value)?`

### Debug Commands

```bash
# Verbose error messages
RUST_BACKTRACE=1 cargo build

# Show expansion of macros
cargo expand --lib procedures::centrality::pagerank

# Check specific error code
rustc --explain E0597
```

---

## 🎯 Success Milestones

### End of Day 1

- ✅ Module structure complete
- ✅ Traits defined
- ✅ Registry pattern working
- ✅ First test passing

### End of Day 3

- ✅ 3+ procedures implemented
- ✅ All execution modes working
- ✅ Examples demonstrating usage

### End of Day 5

- ✅ 8+ algorithms complete
- ✅ Community + Centrality + Pathfinding working
- ✅ Test coverage >90%

### End of Day 7

- ✅ Pipeline foundation complete
- ✅ First ML pipeline working
- ✅ Integration tests passing

### End of Day 10

- ✅ 20+ algorithms implemented
- ✅ ML pipelines operational
- ✅ Form Processor unification complete
- ✅ Documentation published
- ✅ Ready for community release

---

## 🌟 Motivation Reminders

When you hit the wall (you will):

1. **Remember:** The foundation is solid. You've already built Pregel, Projection, Properties.
2. **Remember:** You're not translating randomly - you're discovering the true abstraction.
3. **Remember:** Every algorithm you implement makes the next one easier.
4. **Remember:** The Dragon grows with each codegen. This is evolution.
5. **Remember:** Coffee, rain, and code. The perfect weekend prep led to this.

---

## 📞 Quick Reference Links

**Rust GDS Docs:**

- Import discipline: `.github/copilot-instructions.md`
- Projection philosophy: `doc/projection_philosophy.md`
- Pregel architecture: `doc/PREGEL_ARCHITECTURE.md`

**Java GDS Sources:**

- Executor: `core/src/main/java/org/neo4j/gds/executor/`
- Procedures: `proc/facade/src/main/java/org/neo4j/gds/procedures/`
- ML Pipelines: `ml/ml-algo/src/main/java/org/neo4j/gds/ml/`

**This Codegen:**

- Design doc: `doc/next_codegen_review.md`
- Source mapping: `doc/java_gds_source_map.md`
- This guide: `doc/next_codegen_quick_start.md`

---

**Status:** READY TO LAUNCH 🚀  
**Next Action:** Monday/Tuesday morning - run Pre-Flight Checklist and GO!
