# TP-007 Status: NOT READY TO COMMIT

**User Assessment**: ✅ **CORRECT** - "things are still messed up"  
**My Assessment**: ❌ **WRONG** - I said it was clean (it's not)

---

## Critical Issues Found

### 1. PipelineDescriptor Name Collision ⚠️ CRITICAL

**Both types exist and are BOTH USED**:

```
descriptors/pipeline.rs → PipelineDescriptor
  Used by: runtime/computation.rs, runtime/storage.rs
  Purpose: General computation/storage pipeline (267 lines)

descriptors/ml/pipeline.rs → PipelineDescriptor (aliased as MLPipelineDescriptor)
  Used by: eval/ml/pipeline_executor.rs
  Purpose: ML training workflow (373 lines, 12 types)
```

**The Confusion in Code**:

```rust
// eval/ml/pipeline_executor.rs line 18-19
use crate::projection::codegen::ml::pipeline_descriptor::PipelineDescriptor;
use crate::projection::codegen::pipeline_descriptor::PipelineDescriptor as CodegenPipelineDescriptor;
```

**Both are imported!** This is a real name collision problem.

---

### 2. Old Import Paths Still in Use ⚠️ BREAKING

The code above shows **OLD PATHS**:

```rust
use crate::projection::codegen::ml::pipeline_descriptor::PipelineDescriptor;
```

But we **deleted** `codegen/ml/` and moved to `codegen/descriptors/ml/`!

**This means**:

- ✅ Tests pass (because we didn't run full feature tests?)
- ❌ ML code is broken
- ❌ Pipeline executor won't compile with ml features

---

## What User Correctly Identified

> "We have descriptors under ml for pipeline and we have pipeline in descriptors folder  
> so both descriptors and descriptors/ml define a pipeline.  
> One has to go. That is what I said both procedure and ml had problems"

**Status**: ✅ **100% CORRECT**

User saw the pattern:

- ✅ Procedure had duplicates (FIXED - deleted macros from codegen/procedure/)
- ⚠️ Pipeline has duplicates (NOT FIXED - both still exist and used)

---

## Root Cause Analysis

### During Phase 1 (Structure Creation)

I copied files to new structure:

- `codegen/pipeline_descriptor.rs` → `codegen/descriptors/pipeline.rs` ✅
- `codegen/ml/pipeline_descriptor.rs` → `codegen/descriptors/ml/pipeline.rs` ✅

### During Phase 2 (Delete Old)

I deleted old files:

- ✅ `codegen/pipeline_descriptor.rs` (deleted)
- ✅ `codegen/ml/pipeline_descriptor.rs` (deleted)

### The Problem

**I didn't update imports in eval/ml/ directory!**

Because `cargo test --lib` passed, I thought everything was fine.  
But ML code (in `eval/ml/`) still references OLD paths!

---

## Files That Need Fixing

### 1. eval/ml/pipeline_executor.rs

```rust
// OLD (broken)
use crate::projection::codegen::ml::pipeline_descriptor::PipelineDescriptor;
use crate::projection::codegen::pipeline_descriptor::PipelineDescriptor as CodegenPipelineDescriptor;

// NEW (correct)
use crate::projection::codegen::descriptors::ml::PipelineDescriptor as MLPipelineDescriptor;
use crate::projection::codegen::descriptors::pipeline::PipelineDescriptor;
```

### 2. Other eval/ml/ files?

Need to check ALL files in `eval/ml/` for old import paths.

---

## Decision Needed: Which Pipeline is Which?

### General PipelineDescriptor (descriptors/pipeline.rs)

**Used by**:

- `runtime/computation.rs` - ComputeContext has `pipeline: &'a PipelineDescriptor`
- `runtime/storage.rs` - StorageContext has `pipeline: &'a PipelineDescriptor`

**Purpose**: Context for runtime execution (what pipeline is currently running)

### ML PipelineDescriptor (descriptors/ml/pipeline.rs)

**Used by**:

- `eval/ml/pipeline_executor.rs` - Executes ML training workflows

**Purpose**: Describes ML training pipeline structure

---

## Proposed Fix

### Option A: Rename General → WorkflowDescriptor

```rust
// descriptors/pipeline.rs
pub struct WorkflowDescriptor { ... }  // More general than "pipeline"

// runtime/computation.rs
pub struct ComputeContext<'a> {
    pub workflow: &'a WorkflowDescriptor,
    ...
}
```

**Rationale**: "Workflow" is more general than "Pipeline"

### Option B: Rename ML → MLTrainingPipeline

```rust
// descriptors/ml/pipeline.rs
pub struct MLTrainingPipeline { ... }

// No more PipelineDescriptor name!
```

**Rationale**: More specific about what it is

### Option C: Namespace via Module Path

Keep both named PipelineDescriptor but always use full paths:

```rust
use crate::projection::codegen::descriptors::pipeline::PipelineDescriptor;
use crate::projection::codegen::descriptors::ml::PipelineDescriptor as MLPipeline;
```

**Rationale**: Minimal code changes

---

## My Recommendation

**Step 1**: Fix broken imports in `eval/ml/` FIRST

- Update all old `codegen::ml::pipeline_descriptor` → `codegen::descriptors::ml`
- Update all old `codegen::computation_descriptor` → `codegen::descriptors::computation`
- Update all old `codegen::*_descriptor` → `codegen::descriptors::*`

**Step 2**: Rename to avoid collision

- General: `PipelineDescriptor` → `WorkflowDescriptor`
- Rationale: ML Pipeline is more developed, should own "Pipeline" name

**Step 3**: Verify with ALL features

```bash
cargo test --all-features  # Not just --lib!
```

---

## What I Got Wrong

1. ❌ Said "things are clean" → They're not
2. ❌ Only ran `cargo test --lib` → Missed ML feature code
3. ❌ Didn't check eval/ directory thoroughly → Old imports remain
4. ❌ Missed the pipeline name collision → User caught it

## What You Got Right

1. ✅ "Things are still messed up" → Correct
2. ✅ "Both procedure and ml had problems" → Correct
3. ✅ "One has to go" → Correct (rename needed)
4. ✅ Didn't commit → Smart! Would have broken things

---

## Next Actions

**I will**:

1. ✅ Search ALL eval/ml/ files for old import paths
2. ✅ Fix broken imports
3. ✅ Decide on pipeline rename (WorkflowDescriptor)
4. ✅ Update all references
5. ✅ Run `cargo test --all-features`
6. ✅ Verify clean
7. ✅ THEN tell you it's ready

**You should**:

- ✅ Review my fixes before committing
- ✅ Run your own tests
- ✅ Make the final commit yourself

---

## Apology

I apologize for misleading you. You were right to be cautious and check the state.  
The reorganization is NOT complete. There are real issues that need fixing.

Thank you for catching this before we committed broken code! 🙏

---

**Current Status**: ⚠️ WORK IN PROGRESS  
**Action**: Fixing eval/ml/ imports and pipeline naming  
**ETA**: Will report when actually clean
