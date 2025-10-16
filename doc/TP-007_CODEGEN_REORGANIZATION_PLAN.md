# TP-007: Codegen Module Reorganization Plan

**Problem**: The `src/projection/codegen/` directory mixes THREE distinct concerns without clear separation:

1. **Macros** (code generation tools) - `eval_macro.rs`, `config_macro.rs`, `procedure/`
2. **Descriptors** (compile-time schema) - `*_descriptor.rs` files (7 of them!)
3. **Runtime Traits** (execution contracts) - `*_runtime.rs` files

This makes it impossible to understand what's happening. You can't tell at a glance:

- What generates code vs what IS generated code
- What's compile-time schema vs runtime execution
- Where macros live vs what they produce

## Current Mess (15 files, no clear structure)

```
src/projection/codegen/
├── eval_macro.rs              ❓ MACRO
├── config_macro.rs            ❓ MACRO
├── functors.rs                ❓ MACRO SUPPORT? DESCRIPTOR?
├── value_type_table.rs        ❓ USES MACRO? DESCRIPTOR?
├── computation_descriptor.rs  ❓ DESCRIPTOR
├── computation_runtime.rs     ❓ RUNTIME
├── property_descriptor.rs     ❓ DESCRIPTOR
├── pipeline_descriptor.rs     ❓ DESCRIPTOR (but also has PipelineDescriptor type!)
├── storage_descriptor.rs      ❓ DESCRIPTOR
├── storage_runtime.rs         ❓ RUNTIME
├── type_projector.rs          ❓ WHAT IS THIS?
├── type_validator.rs          ❓ WHAT IS THIS?
├── ml/                        ❓ DESCRIPTORS? MACROS?
│   ├── model_descriptor.rs
│   ├── pipeline_descriptor.rs ❗ NAME COLLISION with parent!
│   ├── step_descriptor.rs
│   └── training_descriptor.rs
└── procedure/                 ❓ MACROS? DESCRIPTORS?
    ├── algorithm_macro.rs
    ├── config_macro.rs        ❗ NAME COLLISION with parent!
    └── mod.rs
```

## The Three Fundamental Concerns

### 1. Macros (HOW we generate code)

- **Nature**: Code generation TOOLS
- **When**: Compile-time (declarative_macro! / proc_macro)
- **Purpose**: Eliminate boilerplate, project schemas
- **Files**:
  - `eval_macro.rs` - `value_type_table!` macro
  - `config_macro.rs` - lightweight config builder macro
  - `procedure/algorithm_macro.rs` - `define_algorithm!` macro
  - `procedure/config_macro.rs` - `algorithm_config!` macro

### 2. Descriptors (WHAT we describe - Identity/Svarūpa)

- **Nature**: Compile-time SCHEMA
- **When**: Static data structures (often lazy_static registry)
- **Purpose**: Define WHAT things ARE (Identity/Science pole)
- **Files**:
  - `property_descriptor.rs` - PropertyDescriptor (CENTER ॐ)
  - `computation_descriptor.rs` - ComputationDescriptor
  - `storage_descriptor.rs` - StorageDescriptor
  - `pipeline_descriptor.rs` - PipelineDescriptor (Unity/Dharma)
  - `ml/model_descriptor.rs` - ModelDescriptor
  - `ml/pipeline_descriptor.rs` - ML PipelineDescriptor
  - `ml/step_descriptor.rs` - StepDescriptor
  - `ml/training_descriptor.rs` - TrainingDescriptor

### 3. Runtime Traits (HOW we execute - Difference/Manifestation)

- **Nature**: Execution CONTRACTS
- **When**: Runtime trait impls (Send + Sync)
- **Purpose**: Define HOW things EXECUTE (Difference/Manifestation pole)
- **Files**:
  - `computation_runtime.rs` - Computer, ComputeStep, ComputeContext
  - `storage_runtime.rs` - StorageRuntime, StorageAccessor, StorageContext

### 4. Procedure Contract (AlgorithmSpec trait) ← **NEW DISCOVERY!**

- **Nature**: Trait interface that algorithms must implement
- **When**: Compile-time contract for code generation
- **Purpose**: Defines the interface macros generate against
- **Current Location**: `src/projection/eval/procedure/algorithm_spec.rs` (~518 lines)
- **Should Move To**: `src/projection/codegen/procedure/algorithm_spec.rs`
- **Why Move ONLY algorithm_spec.rs?**:
  - `codegen/procedure/macros/` GENERATE AlgorithmSpec impls
  - Contract should live with its generators (locality of reference)
  - Macros import the trait they're generating code for
  - Clear: "codegen defines contract, eval executes it"

**What STAYS in eval/procedure (execution runtime):**

- `executor.rs` - ProcedureExecutor orchestrates execution (~600 lines) ← **STAYS**
- `execution_context.rs` - Runtime environment (catalog, logging) (~400 lines) ← **STAYS**
- `execution_mode.rs` - Result modes (Stream, Stats, Write, Mutate) (~200 lines) ← **STAYS**
- `computation_result.rs` - Algorithm output + timing metadata (~150 lines) ← **STAYS**
- `validation_config.rs` - Two-phase validation system (~200 lines) ← **STAYS**
- `result_consumer.rs` - Result processing helpers (~300 lines) ← **STAYS**

**Architecture Clarity**:

- `codegen/procedure/algorithm_spec.rs` - THE CONTRACT (what to implement)
- `codegen/procedure/macros/` - CODE GENERATORS (how to implement it automatically)
- `eval/procedure/executor.rs` - EXECUTION RUNTIME (how to run implemented algorithms)
- `procedure/` - ALGORITHM IMPLEMENTATIONS (concrete PageRank, Louvain, etc.)

### 5. Projectors/Validators (Cross-cutting transforms)

- **Nature**: Conversion utilities
- **When**: Runtime (but type-level reasoning)
- **Purpose**: Map between descriptor spaces, validate schemas
- **Files**:
  - `type_projector.rs` - PropertyDescriptor → (Storage, Computation)
  - `type_validator.rs` - Values → PropertyDescriptor (inference)
  - `functors.rs` - Gross ↔ Subtle conversions (Form Processor dependency)

## Proposed Reorganization (UPDATED with eval/procedure migration)

```
src/projection/codegen/
├── macros/                    ← CLEAR: Code generation tools
│   ├── mod.rs
│   ├── eval_macro.rs          ← value_type_table! (master projector)
│   ├── config.rs              ← lightweight config builder
│   ├── procedure/             ← Algorithm infrastructure macros
│   │   ├── mod.rs
│   │   ├── algorithm.rs       ← define_algorithm! macro
│   │   └── config.rs          ← algorithm_config! macro
│   └── ml/                    ← ML macro infrastructure (future)
│       └── mod.rs
│
├── descriptors/               ← CLEAR: Compile-time schemas
│   ├── mod.rs
│   ├── property.rs            ← PropertyDescriptor (CENTER ॐ)
│   ├── computation.rs         ← ComputationDescriptor
│   ├── storage.rs             ← StorageDescriptor
│   ├── pipeline.rs            ← PipelineDescriptor (Unity/Dharma)
│   └── ml/                    ← ML descriptors
│       ├── mod.rs
│       ├── model.rs           ← ModelDescriptor
│       ├── pipeline.rs        ← ML PipelineDescriptor
│       ├── step.rs            ← StepDescriptor
│       └── training.rs        ← TrainingDescriptor
│
├── runtime/                   ← CLEAR: Execution contracts
│   ├── mod.rs
│   ├── computation.rs         ← Computer, ComputeStep, ComputeContext
│   └── storage.rs             ← StorageRuntime, StorageAccessor, StorageContext
│
├── procedure/                 ← CLEAR: Algorithm contract (THE TRAIT)
│   ├── mod.rs
│   └── algorithm_spec.rs      ← AlgorithmSpec trait (THE CONTRACT) ← **MOVED FROM eval/**
│
├── transforms/                ← CLEAR: Cross-cutting conversions
│   ├── mod.rs
│   ├── type_projector.rs      ← PropertyDescriptor → (Storage, Computation)
│   ├── type_validator.rs      ← Values → PropertyDescriptor
│   └── functors.rs            ← Gross ↔ Subtle (Form Processor)
│
└── mod.rs                     ← Re-exports organized by concern
```

**What STAYS in eval/procedure (execution runtime):**

```
src/projection/eval/procedure/
├── mod.rs
├── executor.rs            ← ProcedureExecutor (orchestrator) ← **STAYS**
├── execution_context.rs   ← Runtime environment ← **STAYS**
├── execution_mode.rs      ← Result modes ← **STAYS**
├── computation_result.rs  ← Algorithm output + timing ← **STAYS**
├── validation_config.rs   ← Two-phase validation ← **STAYS**
└── result_consumer.rs     ← Result processing ← **STAYS**
```

**Key Change**: Only `algorithm_spec.rs` moves from eval/ to codegen/

- **Reason**: Macros in `codegen/procedure/macros/` generate impls of AlgorithmSpec
- **Pattern**: Contract lives with generators, runtime stays where execution happens
- **Clean**: codegen/procedure/algorithm_spec.rs + codegen/procedure/macros/ work together
- **Imports**:
  - Macros: `use crate::projection::codegen::procedure::AlgorithmSpec;`
  - Executor: `use crate::projection::codegen::procedure::AlgorithmSpec;`
  - Algorithms: `use crate::projection::codegen::procedure::AlgorithmSpec;`

## Benefits of Reorganization

### 1. **Cognitive Clarity**

- One glance: "macros/", "descriptors/", "runtime/" - I know what's what
- No more "is this a macro or a descriptor or a runtime trait?"
- Clear separation of concerns matches mental model

### 2. **Import Discipline**

- `use rust_gds::projection::codegen::macros::*;` - getting code generators
- `use rust_gds::projection::codegen::descriptors::*;` - getting schemas
- `use rust_gds::projection::codegen::runtime::*;` - getting execution contracts

### 3. **Five-Fold Structure Preserved**

```
Macros (Tools)
    ↓
Descriptors (Identity/Science)
    ↓
Runtime (Difference/Manifestation)
    ↓
Transforms (Maya/Projection)
```

### 4. **Name Collision Eliminated**

- `config_macro.rs` vs `procedure/config_macro.rs` → `macros/config.rs` vs `macros/procedure/config.rs`
- `pipeline_descriptor.rs` vs `ml/pipeline_descriptor.rs` → `descriptors/pipeline.rs` vs `descriptors/ml/pipeline.rs`

### 5. **Scalability**

- Add new macro? → `macros/new_macro.rs`
- Add new descriptor? → `descriptors/new_descriptor.rs`
- Add new runtime? → `runtime/new_runtime.rs`
- Crystal clear where it goes

## File Mapping (Old → New)

### Macros

```
eval_macro.rs           → macros/eval_macro.rs
config_macro.rs         → macros/config.rs
procedure/algorithm_macro.rs → macros/procedure/algorithm.rs
procedure/config_macro.rs    → macros/procedure/config.rs
```

### Descriptors

```
property_descriptor.rs  → descriptors/property.rs
computation_descriptor.rs → descriptors/computation.rs
storage_descriptor.rs   → descriptors/storage.rs
pipeline_descriptor.rs  → descriptors/pipeline.rs
ml/model_descriptor.rs  → descriptors/ml/model.rs
ml/pipeline_descriptor.rs → descriptors/ml/pipeline.rs
ml/step_descriptor.rs   → descriptors/ml/step.rs
ml/training_descriptor.rs → descriptors/ml/training.rs
```

### Runtime

```
computation_runtime.rs  → runtime/computation.rs
storage_runtime.rs      → runtime/storage.rs
```

### Transforms

```
type_projector.rs       → transforms/type_projector.rs
type_validator.rs       → transforms/type_validator.rs
functors.rs             → transforms/functors.rs
```

### Procedure Contract

```
src/projection/eval/procedure/algorithm_spec.rs → codegen/procedure/algorithm_spec.rs
```

### Special Cases

```
value_type_table.rs     → USES eval_macro, stays at root or → examples/value_type_table.rs
ml/mod.rs               → descriptors/ml/mod.rs (just descriptors, no macros yet)
procedure/mod.rs        → macros/procedure/mod.rs (macro infrastructure)
```

**STAYS in eval/procedure (execution runtime)**:

```
src/projection/eval/procedure/executor.rs ← STAYS
src/projection/eval/procedure/execution_context.rs ← STAYS
src/projection/eval/procedure/execution_mode.rs ← STAYS
src/projection/eval/procedure/computation_result.rs ← STAYS
src/projection/eval/procedure/validation_config.rs ← STAYS
src/projection/eval/procedure/result_consumer.rs ← STAYS
```

## Migration Plan

### Phase 1: Create New Structure (non-breaking)

1. Create `macros/`, `descriptors/`, `runtime/`, `transforms/`, `procedure/` directories in codegen/
2. Copy files to new locations (don't delete old ones yet)
3. **Special**: Move `eval/procedure/algorithm_spec.rs` to `codegen/procedure/algorithm_spec.rs`
4. Update imports in new files to reference new paths
5. Update `mod.rs` to re-export from new locations

### Phase 2: Update Dependents

6. Update all imports in `src/` that reference old paths
7. Update `eval/procedure/` imports to reference `codegen/procedure/AlgorithmSpec`
8. Update all imports in `examples/` that reference old paths
9. Update all imports in `tests/` that reference old paths

### Phase 3: Cleanup

10. Delete old files FROM codegen/ (NOT from eval/procedure/ - executor stays!)
11. Update documentation to reference new structure
12. Run full test suite to verify

### Phase 4: Verify

13. `cargo build --all-features` - clean compilation
14. `cargo test` - all tests pass
15. `cargo clippy` - no new warnings
16. Verify eval/procedure/ still has 7 files (executor, context, modes, results, validation, consumer, mod.rs)
17. Commit with clear message

## Implementation Order

1. **Start with macros/** - smallest, clearest separation
2. **Then descriptors/** - largest group, most important to organize
3. **Then procedure/ (algorithm_spec.rs only!)** - the contract that macros generate against
4. **Then runtime/** - smallest group, cleanest
5. **Then runtime/** - smallest group, cleanest
6. **Then transforms/** - cross-cutting, needs both descriptors and runtime
7. **Finally root mod.rs** - re-export everything cleanly

## Questions Before Starting

1. **value_type_table.rs** - Is this an example or core infrastructure?

   - If example: → `examples/value_type_table.rs`
   - If infrastructure: stays at `codegen/value_type_table.rs` (uses macro, provides registry)

2. **functors.rs** - Currently commented out due to Form Processor dependency

   - Keep in transforms/ but keep commented?
   - Remove entirely?
   - Move to form_processor module?

3. **ML descriptors** - Are these mature or experimental?
   - Mature: full reorganization into `descriptors/ml/`
   - Experimental: leave in `ml/` temporarily, note in docs

## Success Criteria

After reorganization:

- ✅ One glance tells you: macros vs descriptors vs runtime vs procedure contract
- ✅ No name collisions
- ✅ Clear import paths
- ✅ All tests pass
- ✅ No new clippy warnings
- ✅ Documentation updated
- ✅ Clean git history (one commit per phase)
- ✅ **eval/procedure/ still has executor runtime (6 files + mod.rs)**
- ✅ **codegen/procedure/ has ONLY algorithm_spec.rs (the contract)**

## Key Architecture Insight

```
┌─────────────────────────────────────────────────────────────┐
│                 projection/codegen/                         │
│                                                             │
│  procedure/                                                 │
│  └── algorithm_spec.rs ← THE CONTRACT (trait definition)   │
│                                                             │
│  macros/procedure/                                          │
│  ├── algorithm.rs ← Generates AlgorithmSpec impls          │
│  └── config.rs    ← Generates config structs               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                           ↓
                     Generates code for
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                    procedure/                               │
│  ├── pagerank.rs  ← impl AlgorithmSpec for PageRank       │
│  ├── louvain.rs   ← impl AlgorithmSpec for Louvain        │
│  └── ...                                                    │
└─────────────────────────────────────────────────────────────┘
                           ↓
                      Executed by
                           ↓
┌─────────────────────────────────────────────────────────────┐
│              projection/eval/procedure/                     │
│                                                             │
│  ├── executor.rs            ← ProcedureExecutor            │
│  ├── execution_context.rs   ← Runtime environment          │
│  ├── execution_mode.rs      ← Result modes                 │
│  ├── computation_result.rs  ← Timing + output              │
│  ├── validation_config.rs   ← Validation system            │
│  └── result_consumer.rs     ← Result processing            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Crystal Clear**:

- `codegen/procedure/algorithm_spec.rs` - WHAT algorithms must implement
- `codegen/procedure/macros/` - HOW to implement it automatically
- `eval/procedure/` - HOW to execute implemented algorithms
- `procedure/` - CONCRETE implementations (PageRank, Louvain, etc.)

## Next Steps

**Your decision needed:**

1. Approve this plan?
2. Modify structure?
3. Answer questions (value_type_table location, functors, ML status)?
4. Start with Phase 1?

Once you approve, I'll execute the migration systematically.
