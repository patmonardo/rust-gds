# Module Rename: projection::native → projection::eval

**Date**: October 15, 2025  
**Type**: Refactoring  
**Status**: ✅ Complete

---

## Rationale

### The Problem

**`projection/native/`** was a confusing name:

- Originally translated from Java GDS "native projection" concept
- With `projection/factory/` now top-level, "native" became meaningless
- Didn't describe what the module actually contains

### What It Actually Contains

**Execution Runtime Components**:

- ML pipelines (PageRank, Louvain, etc.)
- Form evaluators (Eval macro system)
- Procedure executors (stored procedure analogs)
- Kernel tasks (micro-services for graph operations)

### The Solution

**Rename to `projection/eval/`** - clearly signals:

- Evaluation/execution runtime
- Home of the Eval macro system
- Pipeline executors
- Not I/O (distinguishes from `factory`)

---

## Architecture Clarification

### Before Rename

```
src/projection/
├── factory/         # Data ingestion
├── codegen/         # Code generation
├── native/          # ??? (confusing!)
├── traits/          # Core abstractions
└── impls/           # Implementations
```

### After Rename

```
src/projection/
├── factory/         # Data ingestion (CAR - given data)
├── eval/            # Execution runtime (CDR - derived computations)
├── codegen/         # Code generation utilities
├── traits/          # Core abstractions
└── impls/           # Implementations
```

**Clear Separation**:

- `factory` = External data → GraphStore (Arrow, Neo4j, Polars)
- `eval` = Computations on GraphStore (ML, Form, Procedures)
- `codegen` = Utilities for generating code

---

## Changes Made

### 1. Directory Rename

```bash
mv src/projection/native src/projection/eval
```

### 2. Import Updates

**Bulk replace across all source files**:

```bash
find src -name "*.rs" -type f -exec sed -i 's/projection::native/projection::eval/g' {} +
find doc tests -name "*.md" -o -name "*.rs" | xargs sed -i 's/projection::native/projection::eval/g'
```

**Files affected**: ~50 Rust files, ~20 documentation files

### 3. Module Documentation Update

**`src/projection/mod.rs`** updated with:

- Clear GDSL Runtime description
- Module separation explanation (factory vs eval vs codegen)
- Updated import guidance

---

## Migration Strategy

**No symlink needed!** - Single atomic rename:

1. ✅ Rename directory
2. ✅ Update all imports
3. ✅ Update documentation
4. ✅ Verify compilation

**Result**: Clean, immediate transition with no compatibility period needed.

---

## Verification

### Compilation Status

```bash
cargo check --lib
```

**Result**: ✅ Clean compilation (2 unused import warnings unrelated to rename)

### Test Status

```bash
cargo test --lib --features arrow
```

**Result**: ✅ 1797 tests passing, 1 pre-existing failure (unrelated to rename)

### Import Verification

```bash
grep -r "projection::native" src/
```

**Result**: ✅ Zero matches (all updated to `projection::eval`)

---

## Impact

### Breaking Changes

**For internal code**: None (all imports updated in single commit)

**For external users**:

- Change: `use rust_gds::projection::native::...` → `use rust_gds::projection::eval::...`
- Scope: Public API paths for ML/Form modules
- Migration: Simple find-replace

### Benefits

1. **Clarity** - Name matches purpose (evaluation/execution)
2. **Architecture** - Clear separation (factory/eval/codegen)
3. **Documentation** - GDSL Runtime description makes sense
4. **Consistency** - Aligns with "Eval macro" terminology

---

## Module Purpose (Clarified)

### projection::factory

**Purpose**: Data ingestion (CAR - given data)

**Contents**:

- Arrow-native factory (Phase 1-7 complete!)
- Future: Neo4j connector, Polars connector, etc.
- Scanner → Consumer → Importer → Accumulator → GraphStore

**Example**: `ArrowNativeFactory::new(node_table, edge_table).build()`

### projection::eval

**Purpose**: Execution runtime (CDR - derived computations)

**Contents**:

- ML pipelines (PageRank, Louvain, NodeSimilarity, etc.)
- Form evaluators (Eval macro system)
- Procedure executors (stored procedure analogs)
- Graph operations (algorithms, transformations)

**Example**: `PipelineExecutor::new().execute(pipeline, graph)`

### projection::codegen

**Purpose**: Code generation utilities

**Contents**:

- Computation descriptors
- Storage descriptors
- Eval macro implementation
- Value type tables

**Example**: `eval! { ... }` macro expansion

---

## Updated Import Patterns

### Before

```rust
use rust_gds::projection::native::ml::PipelineExecutor;
use rust_gds::projection::native::form_processor;
```

### After

```rust
use rust_gds::projection::eval::ml::PipelineExecutor;
use rust_gds::projection::eval::form_processor;
```

---

## Documentation Updates

### Files Updated

1. `src/projection/mod.rs` - Module-level documentation
2. All ML/Form documentation files - Import paths
3. Test files - Import paths
4. Examples (if any) - Import paths

### New Guidance

**Module comments now clearly state**:

- `projection` = GDSL Runtime
- `factory` = Data ingestion (CAR)
- `eval` = Execution (CDR)
- `codegen` = Code generation utilities

---

## Next Steps

### Immediate

✅ Rename complete  
✅ All imports updated  
✅ Documentation updated  
✅ Compilation verified

### Phase 8 (Integration)

Continue Arrow Factory work with clear module structure:

- `projection::factory::arrow` - Data ingestion
- `projection::eval::ml` - Algorithms on imported data
- Clean separation of concerns!

---

## Summary

**What**: Renamed `projection/native/` → `projection/eval/`

**Why**:

- "native" was confusing (native to what?)
- "eval" clearly describes purpose (execution runtime)
- Aligns with "Eval macro" terminology
- Distinguishes from "factory" (ingestion)

**How**: Single atomic rename with bulk import updates

**Impact**:

- Breaking change for external users (simple find-replace)
- Clarifies architecture significantly
- No internal disruption (all in one commit)

**Result**: ✅ Clean, clear module structure with obvious purpose for each component

---

**The projection module is now the GDSL Runtime with clear separation:**

- 🏭 **factory** - Build GraphStores from external data
- ⚙️ **eval** - Execute computations on GraphStores
- 🔧 **codegen** - Generate execution code

Much better! 🎯
