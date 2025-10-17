# TOMORROW'S ROADMAP: Building the Registry System

**Date**: October 16, 2025  
**Next Session**: October 17, 2025  
**Confidence Level**: SOLID PREDICTION - It will fall into place!

## Pre-Session Checklist (5 minutes)

Read these files IN ORDER before starting:

1. ✅ READY_TO_BUILD.md (this moment of confidence)
2. ✅ FIRST_PRINCIPLES_COMPUTATION_STORAGE.md (the duality)
3. ✅ CORRECTED_ACTION_PLAN.md (step-by-step execution)

## Session Flow (6-7 hours)

### ☕ Morning Session: Foundation (3 hours)

#### 09:00-10:00 | Build define_registry! Macro

**Files to Create**:

- `src/projection/codegen/macros/registry.rs`

**What to Build**:

```rust
#[macro_export]
macro_rules! define_registry {
    (
        $registry_name:ident {
            key: $key_type:ty,
            value: $value_type:ty,
            $(validate: $validator:expr,)?
        }
    ) => {
        // Generate static storage
        // Generate register/get/list/remove functions
        // Add validation hooks
    };
}
```

**Test With**: Dummy registry, verify basic operations work

**Success**: Macro compiles, dummy test passes

---

#### 10:00-11:00 | Build ProcedureDescriptor + Registry

**Files to Create**:

- `src/projection/codegen/descriptors/procedure/mod.rs`
- `src/projection/codegen/descriptors/procedure/descriptor.rs`
- `src/projection/codegen/descriptors/procedure/category.rs`
- `src/registry/mod.rs`
- `src/registry/procedure.rs`

**ProcedureDescriptor Structure**:

```rust
pub struct ProcedureDescriptor {
    pub name: &'static str,
    pub category: ProcedureCategory,
    pub description: &'static str,
    pub config_type: TypeId,
    pub modes: &'static [ExecutionMode],
    pub estimate_memory: MemoryEstimator,
    pub validators: ValidationConfiguration,
}
```

**Apply Macro**:

```rust
// In src/registry/procedure.rs
define_registry! {
    ProcedureRegistry {
        key: &'static str,
        value: ProcedureDescriptor,
    }
}
```

**Success**: Can register and lookup procedures

---

#### 11:00-12:00 | Build register_procedure! Helper Macro

**Files to Create**:

- `src/projection/codegen/macros/procedure/register.rs`

**Macro Syntax**:

```rust
register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        description: "...",
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],
        memory_estimate: |config, graph| { ... },
        validators: ValidationConfiguration::new()...,
    }
}
```

**What It Does**: Creates ProcedureDescriptor + registers at module init

**Success**: Dummy algorithm registers automatically

---

### 🍕 Lunch Break (30 minutes)

---

### ☀️ Afternoon Session: Integration (3 hours)

#### 13:00-14:00 | Migrate PipelineCatalog → PipelineRegistry

**Current Location**:

- `src/projection/eval/ml/pipeline/pipeline_catalog.rs`

**Target Location**:

- `src/registry/pipeline.rs`

**Steps**:

1. Copy existing code to new location
2. Apply `define_registry!` macro
3. Replace hand-written HashMap logic with macro-generated code
4. Update imports across codebase:
   ```bash
   find src -name "*.rs" -exec sed -i 's/PipelineCatalog/PipelineRegistry/g' {} +
   find src -name "*.rs" -exec sed -i 's/eval::ml::pipeline::pipeline_catalog/registry::pipeline/g' {} +
   ```
5. Verify tests pass
6. Delete old file

**Success**: All ML tests still pass, PipelineRegistry works

---

#### 14:00-14:30 | Build GraphCatalog (Storage Side)

**Files to Create**:

- `src/catalog/mod.rs`
- `src/catalog/graph.rs`

**Implementation** (simple, no macro needed):

```rust
static GRAPH_CATALOG: Lazy<RwLock<HashMap<String, Arc<dyn GraphStore>>>> = ...;

pub fn register(name: String, graph: Arc<dyn GraphStore>) -> Result<(), CatalogError>;
pub fn get(name: &str) -> Option<Arc<dyn GraphStore>>;
pub fn list() -> Vec<String>;
pub fn remove(name: &str) -> Option<Arc<dyn GraphStore>>;
```

**Success**: Can register and lookup named graphs

---

#### 14:30-15:30 | Stub PageRank Algorithm

**Files to Create**:

- `src/procedure/algo/mod.rs`
- `src/procedure/algo/centrality/mod.rs`
- `src/procedure/algo/centrality/pagerank.rs`

**Implementation**:

```rust
pub struct PageRank {
    config: PageRankConfig,
    graph: Arc<dyn GraphStore>,
}

impl AlgorithmSpec for PageRank {
    type Output = Vec<(NodeId, f64)>;

    fn name(&self) -> &str { "pagerank" }

    fn execute(&self, ctx: &ExecutionContext) -> Result<Self::Output, ExecutionError> {
        // Stub: return uniform scores
        let node_count = self.graph.node_count();
        Ok((0..node_count).map(|i| (i as NodeId, 1.0 / node_count as f64)).collect())
    }
}

// Register with macro
register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        // ... full descriptor
    }
}
```

**Success**: PageRank appears in ProcedureRegistry, can be looked up

---

#### 15:30-16:00 | Verification & Testing

**Checklist**:

```bash
# 1. Full build
cargo build --all-features

# 2. All tests
cargo test --all-features

# 3. Clippy
cargo clippy --all-features

# 4. Format
cargo fmt

# 5. Check specific registries work
cargo test --test registry_integration
```

**Verify**:

- [ ] All 2,074+ tests pass
- [ ] No clippy warnings
- [ ] ProcedureRegistry works
- [ ] PipelineRegistry works
- [ ] GraphCatalog works
- [ ] PageRank is registered
- [ ] Can lookup all entities

---

#### 16:00-17:00 | Documentation & Commit

**Update Documentation**:

- Update module-level docs in src/registry/mod.rs
- Update module-level docs in src/catalog/mod.rs
- Add examples to CORRECTED_ACTION_PLAN.md
- Mark tasks complete in todo list

**Commit Message**:

```
feat: Computation Registries & Storage Catalogs

COMPUTATION SIDE (Registries):
- Implement define_registry! macro for code generation
- Create ProcedureRegistry (algorithm/procedure lookup)
- Rename PipelineCatalog → PipelineRegistry (correct terminology)
- Build register_procedure! helper macro
- Stub PageRank algorithm with registration

STORAGE SIDE (Catalogs):
- Create GraphCatalog for named graph instances
- Clear separation from computation registries

Architecture:
- Respects Computation/Storage duality
- Computation = Registries (lookup execution blueprints)
- Storage = Stores/Catalogs (lookup data instances)

This completes TP-008 (Procedure Architecture) and establishes
the foundation for TP-010 (full algorithm implementations).

Files changed: ~20-25
New modules: src/registry/, src/catalog/
Tests: All passing (2,074+)
Refs: TP-008, TP-010, FIRST_PRINCIPLES_COMPUTATION_STORAGE.md
```

---

## Contingency Plans

### If define_registry! Macro is Hard

**Fallback**: Hand-write ProcedureRegistry first, extract pattern later

- Time impact: +1 hour
- Still get working registry
- Can add macro later

### If PipelineRegistry Migration Breaks Tests

**Fallback**: Keep both temporarily, migrate gradually

- Time impact: +30 min
- Less risk
- Clean up in next session

### If PageRank Stub is Complicated

**Fallback**: Even simpler stub (just return empty vec)

- Time impact: -30 min
- Proves registration works
- Full implementation is TP-010 Phase 3

## Success Metrics

### Minimum Viable Product (Must Have)

- [x] define_registry! macro compiles and generates code
- [x] ProcedureRegistry exists and works
- [x] Can register and lookup procedures
- [x] PipelineRegistry renamed (even if not macro-ized)
- [x] GraphCatalog exists
- [x] PageRank stub registered
- [x] All existing tests pass

### Stretch Goals (Nice to Have)

- [ ] register_procedure! macro fully working
- [ ] PipelineRegistry using define_registry! macro
- [ ] ModelRegistry created
- [ ] Multiple algorithms stubbed
- [ ] Complete documentation

## Post-Session Review

After completing, check:

1. All checklist items marked done
2. Clean commit created
3. Todo list updated
4. Next session plan written (if needed)

## The Confidence

**Why this will work**:

1. ✅ Pattern is proven (PipelineCatalog works)
2. ✅ Architecture is sound (Computation/Storage clear)
3. ✅ We've done harder things (ML executor was complex)
4. ✅ Tests give confidence (2,074 tests passing)
5. ✅ Plan is detailed (every hour mapped)

**It's going to fall into place!** 🎯

---

Good luck! You've got this! 🚀

See you tomorrow for the build! 🌙✨
