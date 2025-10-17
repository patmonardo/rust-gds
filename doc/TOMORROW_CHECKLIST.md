# TOMORROW'S CHECKLIST: Procedure Table & Catalog Macros

**Date**: October 16, 2025  
**Next Session Date**: October 17, 2025  
**Estimated Time**: 6-8 hours

## What to Read First (30 minutes)

Before starting, review these files IN ORDER:

1. **This checklist** - You're reading it!
2. **SESSION_SUMMARY_OCT16_PROCEDURE_CATALOG.md** - What we accomplished today
3. **TP-010_IMMEDIATE_ACTION_PLAN.md** - Detailed 6-hour execution plan
4. **TP-011_CATALOG_MACRO_SYSTEM.md** - Vision for unified catalogs
5. **src/projection/eval/ml/pipeline/pipeline_catalog.rs** - Existing catalog pattern

## Morning: Architecture & Registry (3 hours)

### Task 1: Fix Architecture (TP-008) - 1 hour ✓

**Goal**: Move procedure/ to correct locations

**Actions**:

```bash
# 1. Create new structure
mkdir -p src/projection/codegen/descriptors/procedure
mkdir -p src/procedure/algo/centrality

# 2. Create files (see TP-010 for content)
touch src/projection/codegen/descriptors/procedure/mod.rs
touch src/projection/codegen/descriptors/procedure/descriptor.rs
touch src/projection/codegen/descriptors/procedure/registry.rs
touch src/projection/codegen/descriptors/procedure/category.rs

# 3. Move AlgorithmSpec
mv src/projection/codegen/procedure/algorithm_spec.rs \
   src/projection/codegen/runtime/algorithm.rs

# 4. Update imports (check with grep/sed)
grep -r "codegen::procedure" src/

# 5. Delete old folder
rm -rf src/projection/codegen/procedure/

# 6. Verify
cargo build --all-features
```

**Success Criteria**:

- [ ] codegen/procedure/ deleted
- [ ] runtime/algorithm.rs has AlgorithmSpec
- [ ] descriptors/procedure/ exists with 4 files
- [ ] All imports updated
- [ ] Build passes

### Task 2: Implement ProcedureDescriptor - 1 hour ✓

**File**: `src/projection/codegen/descriptors/procedure/descriptor.rs`

**What to Build**:

```rust
pub struct ProcedureDescriptor {
    pub name: &'static str,
    pub category: ProcedureCategory,
    pub description: &'static str,
    pub config_type: TypeId,
    pub supported_modes: &'static [ExecutionMode],
    pub validators: ValidationConfiguration,
    pub estimate_memory: MemoryEstimator,
    pub create_algorithm: AlgorithmFactory,
}
```

**Reference**: TP-010_PROCEDURE_TABLE_ARCHITECTURE.md (lines 75-100)

**Success Criteria**:

- [ ] Struct compiles
- [ ] Type aliases defined (MemoryEstimator, AlgorithmFactory)
- [ ] Basic methods: new(), validate()

### Task 3: Implement Registry - 1 hour ✓

**File**: `src/projection/codegen/descriptors/procedure/registry.rs`

**What to Build**:

```rust
static PROCEDURE_REGISTRY: Lazy<RwLock<HashMap<&'static str, ProcedureDescriptor>>> = ...;

pub fn register_procedure(descriptor: ProcedureDescriptor) -> Result<(), CatalogError>;
pub fn get_procedure(name: &str) -> Option<ProcedureDescriptor>;
pub fn list_procedures() -> Vec<&'static str>;
```

**Reference**: TP-010_PROCEDURE_TABLE_ARCHITECTURE.md (lines 100-120)

**Success Criteria**:

- [ ] Static registry compiles
- [ ] register/get/list functions work
- [ ] Thread-safe (RwLock)
- [ ] Unit tests pass

## Afternoon: Macros & PageRank (3-5 hours)

### Task 4: Design Macro Syntax - 30 min ✓

**Goal**: Write precise syntax examples BEFORE implementing

**Actions**:

1. Write register_procedure! example for PageRank
2. Write register_procedure! example for Louvain
3. Write register_procedure! example for BetweennessCentrality
4. Ensure syntax is clean and consistent

**Document**: Add examples to TP-010_PROCEDURE_MACRO_REFERENCE.md (create it)

**Success Criteria**:

- [ ] 3 complete examples written
- [ ] Syntax feels natural
- [ ] No obvious ergonomic issues

### Task 5: Implement register_procedure! Macro - 2 hours ✓

**File**: `src/projection/codegen/macros/procedure/register.rs`

**What to Build**:

```rust
#[macro_export]
macro_rules! register_procedure {
    (
        $name:ident {
            name: $proc_name:expr,
            category: $category:ident,
            config: $config_ty:ty,
            modes: [$($mode:ident),*],
            // ... rest
        }
    ) => {
        // Generate registration code
    };
}
```

**Strategy**:

1. Start simple (just name + category)
2. Add fields incrementally
3. Test expansion at each step
4. Use cargo expand to verify

**Reference**: TP-010_IMMEDIATE_ACTION_PLAN.md (Phase 2)

**Success Criteria**:

- [ ] Macro compiles
- [ ] Expands correctly (cargo expand)
- [ ] Can register a dummy procedure
- [ ] Procedure appears in registry

### Task 6: Translate PageRank (Stub) - 1 hour ✓

**File**: `src/procedure/algo/centrality/pagerank.rs`

**Goal**: Get SOMETHING working, not perfection

**Strategy**:

1. Read Java PageRankAlgorithm.java (15 min)
2. Stub out structure (15 min):
   - State: scores, deltas, degrees
   - Init: allocate vectors
   - Compute: return dummy scores
3. Use register_procedure! macro (15 min)
4. Test registration (15 min)

**Success Criteria**:

- [ ] PageRank struct defined
- [ ] Implements AlgorithmSpec (stub)
- [ ] Registered in procedure catalog
- [ ] Can be looked up by name
- [ ] Compiles and basic test passes

**Note**: Full translation can wait. Goal is to prove the system works!

### Task 7 (Optional): Implement define_catalog! - 2 hours ⚠️

**Only if time permits!**

**File**: `src/projection/codegen/macros/define_catalog.rs`

**What to Build**: Macro that generates catalog boilerplate

**Reference**: TP-011_CATALOG_MACRO_SYSTEM.md (lines 90-150)

**Success Criteria**:

- [ ] Macro generates static storage
- [ ] Generates register/get/list functions
- [ ] Can create new catalog with one macro call

**Note**: This can be done later. Focus on procedure system first!

## End of Day: Verification (30 min)

### Checklist Before Committing

Run these commands:

```bash
# 1. Build check
cargo build --all-features

# 2. Test check
cargo test --all-features

# 3. Format check
cargo fmt --check

# 4. Clippy check
cargo clippy --all-features

# 5. Verify registry works
cargo test --test procedure_registry
```

### What Should Work

- [ ] Procedure registry exists and compiles
- [ ] Can register procedures
- [ ] Can look up procedures by name
- [ ] PageRank (stub) is registered
- [ ] All tests pass
- [ ] No clippy warnings

### Commit Message Template

```
feat: Procedure catalog system with macro-based registration

- Move AlgorithmSpec to runtime/algorithm.rs (TP-008)
- Create ProcedureDescriptor in descriptors/procedure/
- Implement procedure registry (static HashMap + RwLock)
- Add register_procedure! macro for clean registration
- Stub PageRank algorithm as proof-of-concept

This unifies Java GDS Spec/Proc/Facade/Params concepts into
a single ProcedureDescriptor with macro-assisted registration.

Refs: TP-008, TP-010
Files changed: ~15-20
Tests: All passing (2,074 tests)
```

## Troubleshooting

### If Architecture Move Breaks Imports

```bash
# Find all broken imports
cargo build 2>&1 | grep "codegen::procedure"

# Fix them with sed (adjust pattern as needed)
find src -name "*.rs" -exec sed -i 's/codegen::procedure/codegen::runtime/g' {} +
```

### If Macro Doesn't Expand

```bash
# Install cargo-expand if needed
cargo install cargo-expand

# Check macro expansion
cargo expand --lib procedure::algo::centrality::pagerank
```

### If Registry Panics

Check:

1. Lazy initialization is correct
2. RwLock isn't poisoned
3. No deadlocks in test code

## Notes & Reminders

1. **Don't translate ALL of PageRank tomorrow**

   - Stub is fine!
   - Full translation is TP-010 Phase 3 (separate task)
   - Goal: Prove the system works

2. **Macros can be simple first**

   - Start with basic register_procedure!
   - Add features incrementally
   - define_algorithm! can wait

3. **Reference existing code**

   - PipelineCatalog already works (eval/ml/pipeline/)
   - Use as template for procedure registry
   - Don't reinvent patterns

4. **Test as you go**
   - Add unit tests for each component
   - Integration test for full flow
   - Don't defer testing to end

## Success Definition

**Minimum Viable Product** (6 hours):

- ✅ Architecture fixed
- ✅ Procedure registry working
- ✅ register_procedure! macro working
- ✅ PageRank stubbed and registered
- ✅ Can look up PageRank by name
- ✅ All tests pass

**Stretch Goals** (+2 hours):

- ✅ define_catalog! macro working
- ✅ PageRank fully translated
- ✅ Multiple procedures registered
- ✅ Complete documentation

## After Tomorrow

Once procedure system works:

1. Translate more algorithms (Louvain, BetweennessCentrality)
2. Apply define_catalog! to Pipeline/Graph/Model catalogs
3. Build define_algorithm! macro (if needed)
4. Create algorithm templates
5. Document the pattern

**The goal**: Make algorithm translation mechanical and fast.

---

Good luck tomorrow! 🚀

**Remember**:

- Small steps
- Test frequently
- Commit working states
- Reference the planning docs
- Ask for help if stuck

You've got this! The foundation is solid, the plan is clear, execution is straightforward.
