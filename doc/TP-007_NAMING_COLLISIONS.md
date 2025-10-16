# TP-007 URGENT: Naming Collisions Review

**Date**: October 16, 2025  
**Status**: ⚠️ NOT CLEAN - DO NOT COMMIT YET  
**Issue**: Multiple naming collisions discovered

---

## Problems Found

### 1. PipelineDescriptor Name Collision (CRITICAL!)

**Two different types with same name:**

```
descriptors/pipeline.rs:
  pub struct PipelineDescriptor { ... }  // General computation pipeline (267 lines)

descriptors/ml/pipeline.rs:
  pub struct PipelineDescriptor { ... }  // ML-specific pipeline (373 lines)
```

**Current "Fix" (Inadequate)**:

```rust
// descriptors/ml/mod.rs line 30
pub use pipeline::{
    PipelineDescriptor as MLPipelineDescriptor,  // Aliased
```

**Why This Is Still Confusing**:

- Two files named `pipeline.rs` in different directories
- Both define `PipelineDescriptor` struct
- Alias only fixes exports, not internal confusion
- Unclear which is the "real" pipeline descriptor
- What IS a pipeline? Computation flow? ML workflow? Both?

---

### 2. Procedure Duplicates (FIXED!)

**Was:**

```
codegen/procedure/algorithm_macro.rs  ❌ duplicate
codegen/procedure/config_macro.rs     ❌ duplicate
codegen/macros/procedure/algorithm.rs ✅ original
codegen/macros/procedure/config.rs    ✅ original
```

**Status**: ✅ **FIXED** - Deleted duplicates from codegen/procedure/

---

## The Root Question: What IS a Pipeline?

### Option A: General Computation Pipeline (descriptors/pipeline.rs)

**Purpose**: Describes a general computation workflow

- 267 lines of philosophy about Dharma and unity
- "Both Storage and Computation poles"
- "The CENTER of the Five-Fold Brahmachakra"

**Types Defined**:

- `PipelineDescriptor` only (single struct)

**Usage**: ???

### Option B: ML Pipeline (descriptors/ml/pipeline.rs)

**Purpose**: Describes ML training workflows

- 373 lines of practical ML types
- Node classification, regression, link prediction
- Training config, model candidates, validation

**Types Defined**:

- `PipelineDescriptor`
- `PipelineType`
- `TrainingConfig`
- `ModelCandidate`
- `ModelType`
- `SplitConfig`
- `ValidationMetric`
- `PipelineConfig`
- `AutoTuningConfig`
- `SearchStrategy`
- `ValidationConfig`
- `PipelineMetadata`
- `PipelineDescriptorBuilder`

**Usage**: ML algorithm implementations

---

## Analysis: Which One is Real?

### Evidence for ML Pipeline Being the Real One

1. **More complete** - 12 types vs 1 type
2. **More practical** - Actual ML workflow types
3. **Has builder pattern** - PipelineDescriptorBuilder
4. **Well documented** - Clear purpose
5. **Has metadata** - Creation time, user, etc.
6. **Maps to Java GDS** - References org.neo4j.gds.ml.pipeline

### Evidence for General Pipeline Being Philosophical

1. **Mostly documentation** - Philosophy about Dharma
2. **One struct** - Just PipelineDescriptor
3. **Vague purpose** - "Unity that projects into extremes"
4. **No clear usage** - Where is this used?
5. **Re-exports from property** - Not self-contained

---

## Proposed Solution

### Option 1: Rename General Pipeline (Recommended)

```
descriptors/pipeline.rs → descriptors/computation_flow.rs
  PipelineDescriptor → ComputationFlowDescriptor
```

**Rationale**:

- ML Pipeline is the PRIMARY use case
- ML Pipeline should own the "Pipeline" name
- General pipeline is about computation flow, call it that
- Clear separation: ML workflows vs computation flows

### Option 2: Rename ML Pipeline

```
descriptors/ml/pipeline.rs → descriptors/ml/training_workflow.rs
  PipelineDescriptor → TrainingWorkflowDescriptor
```

**Rationale**:

- Keeps "pipeline" for general concept
- ML pipeline is specifically about training
- More specific name

**Downside**: ML Pipeline is more developed and practical

### Option 3: Delete General Pipeline

```
rm descriptors/pipeline.rs
```

**Rationale**:

- If it's not used anywhere, delete it
- ML Pipeline is the only real implementation
- Simplifies codebase

**Question**: Where is general PipelineDescriptor actually used?

---

## Investigation Needed

### 1. Check Usage of General PipelineDescriptor

```bash
grep -r "PipelineDescriptor" src/ --include="*.rs" | grep -v "ml/pipeline.rs" | grep -v "descriptors/pipeline.rs"
```

**Question**: Does anything import/use the general PipelineDescriptor?

### 2. Check Re-exports

```bash
# Check if it's re-exported from codegen/mod.rs
grep "PipelineDescriptor" src/projection/codegen/mod.rs

# Check if it's used in examples
grep "PipelineDescriptor" examples/
```

### 3. Check Test Coverage

```bash
# Are there tests for general PipelineDescriptor?
grep "PipelineDescriptor" src/projection/codegen/descriptors/pipeline.rs | grep "test"
```

---

## Similar Issues to Check

Based on the pattern, we should check for other collisions:

### Check for Other Duplicates

```bash
# Find duplicate struct names
find src/projection/codegen -name "*.rs" -exec grep "^pub struct" {} + | \
  awk '{print $3}' | sort | uniq -d

# Find duplicate enum names
find src/projection/codegen -name "*.rs" -exec grep "^pub enum" {} + | \
  awk '{print $3}' | sort | uniq -d
```

---

## Decision Matrix

| Option                         | Pros                                 | Cons                      | Effort |
| ------------------------------ | ------------------------------------ | ------------------------- | ------ |
| Rename General→ComputationFlow | Clear separation, ML owns "Pipeline" | Break imports if used     | Medium |
| Rename ML→TrainingWorkflow     | Keep general concept                 | ML more developed         | Medium |
| Delete General                 | Simplest if unused                   | Lost work if needed later | Low    |
| Keep Both Aliased              | No code changes                      | Confusing forever         | Zero   |

---

## Recommendation

**Step 1**: Find usage of general PipelineDescriptor

- If used nowhere → **DELETE** descriptors/pipeline.rs
- If used somewhere → **RENAME** to ComputationFlowDescriptor

**Step 2**: Remove alias from ml/mod.rs

```rust
// Before
pub use pipeline::{ PipelineDescriptor as MLPipelineDescriptor, ... };

// After (if general pipeline deleted)
pub use pipeline::PipelineDescriptor;  // No alias needed!
```

**Step 3**: Update imports in codegen/mod.rs

---

## Next Actions

**DO NOT COMMIT** until we resolve:

1. ✅ Check usage of descriptors/pipeline.rs
2. ✅ Decide: Delete, Rename, or Keep?
3. ✅ Check for other naming collisions
4. ✅ Fix imports
5. ✅ Verify build + tests
6. ✅ THEN commit

---

## User's Original Observation

> "We have descriptors under ml for pipeline and we have pipeline in descriptors folder so both descriptors and descriptors/ml define a pipeline. One has to go."

**Status**: ✅ **ABSOLUTELY CORRECT**

The user identified a real problem that I missed. This needs to be resolved before committing.

---

**Current State**: ⚠️ BLOCKING ISSUES REMAIN  
**Action Needed**: Investigate and decide on pipeline naming  
**Status**: DO NOT COMMIT YET
