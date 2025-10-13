# ML Pipeline Package - Clean Stopping Point

**Date**: October 13, 2025  
**Status**: ✅ Architecture Learning Complete - 74 Tests Passing  
**Next**: 🔍 Major Architecture Review Required

---

## Quick Summary

### ✅ What We Built (All Working)

**Pipeline Orchestration** (projection/native/ml):

- `PipelineExecutor` - Orchestrates: properties → features → splits → training
- `TrainingExecutor` - Multi-candidate training with validation-based selection
- `PipelineState` - Runtime data container with dataset splits
- Feature assembly with `Transformation` trait (identity only for Phase 2.3)
- `Model` trait interface (fit, predict, evaluate)
- Integration with graph procedures (mock PageRank, FastRP, Louvain)

**Test Coverage**: 74 tests passing

- 40 tests: Pipeline infrastructure
- 15 tests: Feature assembly
- 12 tests: Training + Model trait
- 7 tests: DecisionTree stub

### ⚠️ Critical Discovery

**Model implementations don't belong here.**

`projection/native/ml/` is for **pipeline orchestration**, not **algorithm implementation**.

Actual models (DecisionTree, LogisticRegression, etc.) belong in:

- `ml-algo/` package (you mentioned this exists)
- `ml-core/` package (you mentioned this exists)

### 🚫 Blocking Issue

**PropertyValues trait** prevents model implementations from accessing node property scalars:

```rust
// This doesn't work:
let value = property.as_any().downcast_ref::<MockDouble>()?;
//                   ^^^^^^^ method doesn't exist
```

**Solution**: Use `NodePropertyValues` trait in Model interface (has `double_value()`, `long_value()` methods).

---

## Key Documents Created

1. **`doc/ML_ARCHITECTURE_CRITICAL_REVIEW.md`** ⭐ READ THIS FIRST

   - Complete architecture analysis
   - Package boundary clarification
   - PropertyValues trait issue explanation
   - 3 options for path forward
   - Architecture decisions to make

2. **`doc/ML_PACKAGE_REVIEW_OCT_13.md`**

   - Detailed progress tracking
   - Node-centric design patterns
   - Java GDS → Rust mappings
   - Code navigation guide

3. **`doc/JAVA_GDS_FEATURE_SYSTEM_ANALYSIS.md`** (600+ lines)

   - Java GDS feature patterns analysis
   - Rust trait design

4. **`doc/JAVA_GDS_TRAINING_SYSTEM_ANALYSIS.md`** (200+ lines)
   - Java GDS training patterns
   - ModelCandidate system

---

## Architecture Questions (Need Your Input)

### 1. Package Structure

**Current Understanding**:

```
rust-gds/
├── src/projection/native/ml/   # We built this (pipeline orchestration)
├── ml-algo/                    # You mentioned - algorithms live here?
└── ml-core/                    # You mentioned - core ML primitives?
```

**Questions**:

- What's currently in `ml-algo/`?
- What's currently in `ml-core/`?
- How should they interact with `projection/ml`?

### 2. Model Instantiation

How should `TrainingExecutor` get model implementations?

**Option A: ModelRegistry** (Recommended - loose coupling)

```rust
registry.register(ModelType::DecisionTree, DecisionTreeFactory);
let model = registry.create(ModelType::DecisionTree, params);
```

**Option B: Direct Imports** (Tight coupling)

```rust
match model_type {
    ModelType::DecisionTree => Box::new(DecisionTree::new()),
    // ...
}
```

### 3. PropertyValues vs NodePropertyValues

Should Model trait use:

- `PropertyValues` (current - too abstract, causes issues)
- `NodePropertyValues` (recommended - has scalar accessors)

### 4. Feature Representation in ml-algo

Should ml-algo work with:

- `HashMap<String, Arc<dyn PropertyValues>>` (current)
- `FeatureMatrix { data: Vec<Vec<f64>> }` (ml-native representation)

---

## Code Status

### ✅ Production Ready (Keep)

- `src/projection/native/ml/pipeline_executor.rs`
- `src/projection/native/ml/pipeline_state.rs`
- `src/projection/native/ml/training_executor.rs`
- `src/projection/native/ml/graph_procedure.rs`
- `src/projection/native/ml/features/*` (all files)
- `src/projection/native/ml/models/model_trait.rs`
- `src/projection/codegen/ml/pipeline_descriptor.rs`

### ⚠️ Temporary/Learning (Remove or Move)

- `src/projection/native/ml/models/decision_tree_stub.rs` - Just for learning, real impl goes in ml-algo

### 🔧 Needs Changes

- `src/projection/native/ml/models/model_trait.rs` - Change PropertyValues → NodePropertyValues

---

## Recommended Next Steps

### Immediate (Before Proceeding)

1. **Review Documents** (30 min)

   - Read `doc/ML_ARCHITECTURE_CRITICAL_REVIEW.md`
   - Review package structure questions
   - Check ml-algo and ml-core current state

2. **Architecture Decisions** (30 min)
   - Clarify package boundaries
   - Decide on ModelRegistry vs Direct imports
   - Decide PropertyValues vs NodePropertyValues
   - Plan feature representation

### After Architecture Review

3. **Implement ModelRegistry** (2 hours)

   - Create `ModelFactory` trait
   - Create `ModelRegistry` in projection/ml
   - Update `TrainingExecutor` to use registry

4. **Fix Model Trait** (30 min)

   - Change signature to use `NodePropertyValues`
   - Update tests

5. **First Real Model in ml-algo** (4 hours)

   - Implement DecisionTree in ml-algo/tree/
   - Register with ModelRegistry
   - Integration test

6. **End-to-End Test** (2 hours)
   - Graph creation
   - Property computation
   - Feature assembly
   - Model training
   - Prediction

---

## Test Commands

```bash
# All ML tests (74 passing)
cargo test --lib native::ml

# Specific modules
cargo test --lib native::ml::pipeline_executor
cargo test --lib native::ml::training_executor
cargo test --lib native::ml::features
cargo test --lib native::ml::models

# With output
cargo test --lib native::ml -- --nocapture
```

---

## What We Proved

✅ **Java GDS patterns translate to Rust successfully**

- Pipeline descriptors work
- Training with validation works
- Feature transformations work
- Model trait abstraction works

✅ **Node-centric design is solid**

- HashMap<String, PropertyValues> for features
- Vec<usize> for node IDs
- Consistent throughout all modules

✅ **Orchestration layer is clean**

- Clear separation of concerns
- Testable components
- Good error handling

⚠️ **Need to clarify package boundaries**

- Where do models live?
- How do packages interact?
- What's the role of ml-algo and ml-core?

---

## Bottom Line

**We successfully learned the architecture** by building the pipeline orchestration layer. 74 tests pass, patterns are clear, code is clean.

**Now we need architectural guidance** before implementing actual ML algorithms. The DecisionTree stub serves its purpose - it shows the Model trait works, but real implementations belong elsewhere.

**Read `doc/ML_ARCHITECTURE_CRITICAL_REVIEW.md` for full analysis** and to make informed decisions about the path forward.

---

## Contact Points for Review

**Key Files to Review**:

1. `doc/ML_ARCHITECTURE_CRITICAL_REVIEW.md` - Full analysis
2. `src/projection/native/ml/pipeline_executor.rs` - Main orchestrator
3. `src/projection/native/ml/training_executor.rs` - Training logic
4. `src/projection/native/ml/models/model_trait.rs` - Model interface

**Key Questions**:

1. What's in ml-algo currently?
2. What's in ml-core currently?
3. How should packages interact?
4. PropertyValues vs NodePropertyValues?

**Ready for**: Architecture review meeting / decision-making session.
