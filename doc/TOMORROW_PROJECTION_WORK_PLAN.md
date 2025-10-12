# Tomorrow's Projection Work Plan

**Date**: October 12, 2024  
**Focus Areas**: `projection/codegen/` and `projection/native/`

## Architecture: Kernel Form Processor

Vision: **CPU microcode analogy** - precompiled tables + runtime dispatch

```
┌─────────────────────────────────────────────────────────┐
│               KERNEL FORM PROCESSOR                      │
│                                                          │
│  ┌────────────────────┐      ┌─────────────────────┐   │
│  │  Codegen Layer     │      │  Native Runtime     │   │
│  │  (Compile-time)    │ ───> │  (Execution)        │   │
│  │                    │      │                     │   │
│  │ • Value Type       │      │ • Form Processor    │   │
│  │   Tables           │      │   (Policy Surface)  │   │
│  │ • Property         │      │                     │   │
│  │   Descriptors      │      │ • Native Factory    │   │
│  │ • Computation      │      │   (Instantiation)   │   │
│  │   Specs            │      │                     │   │
│  │ • Pipeline         │      │ • Type Dispatch     │   │
│  │   Descriptors      │      │   Tables            │   │
│  └────────────────────┘      └─────────────────────┘   │
│                                                          │
└─────────────────────────────────────────────────────────┘
         ↑                                    ↓
    Descriptors                          Execution
    (Compile-time)                       (Runtime)
```

## File Creation Rules

### ✅ CREATE Files Here

#### projection/codegen/
- Value type tables and descriptors
- Property accessor dispatch logic
- Backend selection specifications
- Computation descriptors
- Pipeline descriptors
- Storage descriptors
- Meta macro processors (eval_macro, config_macro)
- Any compile-time code generation

#### projection/native/
- Runtime execution logic
- Form processor extensions (policy surface)
- Native factory implementations
- Type conversion runtime
- Backend instantiation logic

### ❌ DO NOT CREATE Files Here

#### projection/ (top-level)
- Reserved for stable core types only
- No new files unless discussing first
- Current stable types: NodeLabel, RelationshipType, Orientation

#### projection/traits/
- Stable trait surface
- Avoid changes unless critical
- Already has: projection traits, contracts

#### projection/impls/
- Stable implementations of core traits
- Avoid changes unless critical
- Already has: standard implementations

## Import Discipline

### Three Barrel Points

```rust
// 1. Root barrel (stable core types)
use crate::projection::NodeLabel;           // ✅
use crate::projection::RelationshipType;    // ✅

// 2. Codegen barrel (compile-time machinery)
use crate::projection::codegen::PropertyDescriptor;          // ✅
use crate::projection::codegen::eval_macro;                  // ✅
use crate::projection::codegen::pipeline_descriptor::X;      // ✅

// 3. Native barrel (runtime execution)
use crate::projection::native::form_processor;               // ✅
use crate::projection::native::NativeFactory;                // ✅

// ❌ AVOID: Ambiguous paths
use crate::projection::PropertyDescriptor;   // Where is this?
use crate::projection::form_processor;       // Unclear!
```

### Pattern Examples

**Good Pattern:**
```rust
// In a new codegen file
use crate::projection::codegen::PropertyDescriptor;
use crate::projection::native::form_processor;
use crate::types::ValueType;

pub struct NewDescriptor {
    property: PropertyDescriptor,
    // ...
}

impl NewDescriptor {
    pub fn validate(&self) -> Result<(), FormProcessorError> {
        form_processor::validate_something()?;
        Ok(())
    }
}
```

**Bad Pattern:**
```rust
// ❌ Creating in wrong location
// File: src/projection/my_new_thing.rs  // NO! Wrong location!

// ❌ Ambiguous imports
use crate::projection::PropertyDescriptor;  // Should be codegen::
use super::form_processor;                   // Should be explicit
```

## Work Areas for Tomorrow

### 1. Codegen Layer (Compile-time)

**Focus**: Generate specification tables

Files to work in:
- `projection/codegen/eval_macro.rs` - Enhance macro system
- `projection/codegen/value_type_table.rs` - Expand value type specs
- `projection/codegen/property_descriptor.rs` - Property specifications
- `projection/codegen/computation_descriptor.rs` - Computation specs
- `projection/codegen/*.rs` - New codegen files as needed

What we'll generate:
- Property accessor dispatch tables
- Backend selection matrices
- Type conversion specifications
- Computation pattern tables

### 2. Native Layer (Runtime)

**Focus**: Implement runtime execution

Files to work in:
- `projection/native/form_processor.rs` - Policy surface, validation
- `projection/native/native_factory.rs` - Factory instantiation
- `projection/native/mod.rs` - Runtime barrel exports
- `projection/native/*.rs` - New runtime files as needed

What we'll implement:
- Table-driven type dispatch
- Runtime backend selection
- Property value conversion
- Factory instantiation logic

## Key Concepts

### Kernel Form Processor = CPU Microcode

Like a CPU executes microcode:
1. **Compile-time**: Generate instruction tables (codegen/)
2. **Runtime**: Execute via table lookup (native/)

### Meta Macro Processor

- `eval_macro.rs` generates value type modules
- Each module has compile-time descriptor + runtime dispatch
- Form processor bridges the two layers

### Nexus Point

`form_processor.rs` is the nexus:
- Called by macro-generated code
- Provides policy surface (validation, conversion)
- Bridges compile-time specs to runtime execution

## Module Structure (Current State)

```
projection/
├── mod.rs              # Root barrel (core types only)
├── node_label.rs       # Core type
├── relationship_type.rs # Core type
├── orientation.rs      # Core type
├── traits/             # Stable trait surface (avoid changes)
│   └── ...
├── impls/              # Stable implementations (avoid changes)
│   └── ...
├── codegen/            # 🎯 WORK HERE (compile-time)
│   ├── mod.rs          # Codegen barrel
│   ├── eval_macro.rs   # Meta macro processor
│   ├── value_type_table.rs
│   ├── property_descriptor.rs
│   ├── computation_descriptor.rs
│   ├── pipeline_descriptor.rs
│   ├── storage_descriptor.rs
│   └── *.rs            # New files go here
└── native/             # 🎯 WORK HERE (runtime)
    ├── mod.rs          # Native barrel
    ├── form_processor.rs  # Policy nexus
    ├── native_factory.rs
    └── *.rs            # New files go here
```

## Quick Reference

### Before Creating a File

Ask:
1. Is this compile-time or runtime?
   - Compile-time → `codegen/`
   - Runtime → `native/`
2. Is this a descriptor/spec or execution logic?
   - Descriptor → `codegen/`
   - Execution → `native/`
3. Does this fit in existing files?
   - Check `codegen/` and `native/` first
   - Only create new file if clearly separate concern

### Before Adding an Import

Check:
1. Is it a core type? → `crate::projection::X`
2. Is it codegen machinery? → `crate::projection::codegen::X`
3. Is it runtime execution? → `crate::projection::native::X`

### Before Adding a Re-export

Ask:
1. Is this commonly used across many files?
2. Is the location ambiguous without full path?
3. Can we keep the explicit path instead?

**Default**: Prefer explicit paths over convenience re-exports

## Session Goals

Tomorrow's work will:
1. Enhance the eval_macro system (codegen/)
2. Generate more complete specification tables (codegen/)
3. Implement table-driven runtime dispatch (native/)
4. Maintain clean separation: descriptors vs execution
5. Keep all imports explicit: `codegen::` or `native::`

## Success Criteria

- [ ] All new files in `codegen/` or `native/` subdirectories
- [ ] No new files in root `projection/` folder
- [ ] All imports use explicit `codegen::` or `native::` paths
- [ ] Clear separation between compile-time specs and runtime
- [ ] Form processor remains the policy nexus
- [ ] Code compiles and tests pass

## Related Documents

- **doc/MODULE_CLEANUP_PROJECTION_SIMPLIFICATION.md** - Today's cleanup
- **doc/adr0002_triadic_graphstore_architecture.md** - Overall architecture
- **doc/KERNEL_FORM_PROCESSOR_VISION.md** - Architecture vision
- **doc/.github/copilot-instructions.md** - Project conventions
