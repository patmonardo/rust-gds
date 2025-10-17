# FIRST PRINCIPLES: Computation/Storage Duality

**Date**: October 16, 2025  
**Insight**: Getting back to the fundamental architecture we designed

## The Core Duality

```
┌─────────────────────────────────────────────────────────┐
│                    CODEGEN SYSTEM                        │
│                                                          │
│  ┌────────────────────┐      ┌────────────────────┐    │
│  │   COMPUTATION      │      │     STORAGE        │    │
│  │   (Process)        │      │     (State)        │    │
│  └────────────────────┘      └────────────────────┘    │
│           │                           │                 │
│  ┌────────┴────────┐         ┌───────┴────────┐       │
│  │  DESCRIPTORS    │         │  DESCRIPTORS   │       │
│  │  (WHAT)         │         │  (WHAT)        │       │
│  │  - Identity     │         │  - Identity    │       │
│  └────────┬────────┘         └───────┬────────┘       │
│           │                           │                 │
│  ┌────────┴────────┐         ┌───────┴────────┐       │
│  │  RUNTIME        │         │  FACTORIES     │       │
│  │  (HOW)          │         │  (HOW)         │       │
│  │  - Execution    │         │  - Creation    │       │
│  └────────┬────────┘         └───────┬────────┘       │
│           │                           │                 │
│  ┌────────┴────────┐         ┌───────┴────────┐       │
│  │  REGISTRIES     │         │  STORES        │       │
│  │  (Lookup)       │         │  (Lookup)      │       │
│  └─────────────────┘         └────────────────┘       │
└─────────────────────────────────────────────────────────┘
```

## COMPUTATION SIDE (Left Side)

### Layer 1: Descriptors (WHAT the computation IS)

- **ComputationDescriptor** - Defines computation identity
  - Name, ID, Species (BSP, DataFlow, etc.)
  - Pattern (VertexCentric, EdgeCentric, etc.)
  - Configuration schema
  - Memory requirements

### Layer 2: Runtime (HOW it EXECUTES)

- **Computer trait** - Runtime execution contract
  - init(), compute(), terminate()
  - Message passing
  - Superstep iteration

### Layer 3: Registries (LOOKUP tables)

- **ComputationRegistry** - Maps name → ComputationDescriptor
- **ComputerFactory** - Creates Computer instances from descriptors
- **AlgorithmRegistry** - Maps algorithm name → Algorithm implementation

**Examples**:

- PageRank Descriptor → PageRank Computer
- Louvain Descriptor → Louvain Computer
- BFS Descriptor → BFS Computer

## STORAGE SIDE (Right Side)

### Layer 1: Descriptors (WHAT the storage IS)

- **StorageDescriptor** - Defines storage identity
  - Backend type (HugeArray, Arrow, Sparse)
  - Schema (columns, types)
  - Compression hints
  - Access patterns

### Layer 2: Factories (HOW it's CREATED)

- **StorageRuntimeFactory** - Creates storage instances
- **ArrayBackendFactory** - Creates array backends
- **PropertyValuesFactory** - Creates property columns

### Layer 3: Stores (LOOKUP catalogs)

- **GraphStore** - The graph itself (nodes, edges, properties)
- **PropertyStore** - Column storage
- **GraphCatalog** - Maps name → GraphStore instance

**Examples**:

- HugeArray Descriptor → HugeArray Backend
- Arrow Descriptor → Arrow Backend
- Sparse Descriptor → HashMap Backend

## The Key Insight: Different Terminology!

You're right - I was conflating everything as "Catalogs". The correct terms are:

### COMPUTATION SIDE: **REGISTRIES**

- **Why**: Registries hold execution blueprints
- **Pattern**: Name → Descriptor → Runtime instance
- **Examples**:
  - Algorithm Registry (PageRank, Louvain, etc.)
  - Computation Registry (BSP patterns, DataFlow patterns)
  - Procedure Registry (GDSL procedures)
  - Model Registry (ML models)

### STORAGE SIDE: **STORES** or **FACTORIES**

- **Why**: Stores/Factories create and hold data
- **Pattern**: Schema → Factory → Storage instance
- **Examples**:
  - Graph Store (the actual graph data)
  - Property Store (column storage)
  - Backend Factory (creates HugeArray, Arrow, etc.)
  - Graph Catalog (named graph instances)

## What We Built (Mapping to Architecture)

### ✅ COMPUTATION SIDE (Mostly Complete)

1. **Descriptors** ✓

   - ComputationDescriptor (codegen/descriptors/computation.rs)
   - ML Descriptors (pipeline, model, step, training)
   - PipelineDescriptor is a COMPUTATION descriptor!

2. **Runtime** ✓

   - Computer trait (codegen/runtime/computation.rs)
   - Procedure executor (eval/procedure/)
   - ML training executor (eval/ml/training_executor.rs)

3. **Registries** ⚠️ (What we're building!)
   - ProcedureRegistry (need to build - TP-010)
   - AlgorithmRegistry (part of ProcedureRegistry)
   - PipelineCatalog (exists but should be PipelineRegistry!)
   - ModelRegistry (need to build)

### ✅ STORAGE SIDE (Mostly Complete)

1. **Descriptors** ✓

   - StorageDescriptor (codegen/descriptors/storage.rs)
   - PropertyDescriptor (codegen/descriptors/property.rs)
   - Backend descriptors (ArrayBackend enum, etc.)

2. **Factories** ✓

   - StorageRuntimeFactory (codegen/runtime/storage.rs)
   - NativeFactory (types/factory/)
   - Backend factories (huge_array, arrow, sparse)

3. **Stores** ✓
   - GraphStore trait (types/graph_store/)
   - DefaultGraphStore (the implementation)
   - PropertyValues (column stores)
   - Graph Catalog (need to build properly)

## The Macro System (Corrected)

### For COMPUTATION SIDE: **define_registry!**

```rust
define_registry! {
    /// Algorithm Registry - All registered algorithms
    AlgorithmRegistry {
        key: &'static str,
        value: AlgorithmDescriptor,

        lookup: |name| {
            // Registry lookup logic
        },
    }
}

define_registry! {
    /// Procedure Registry - GDSL procedures
    ProcedureRegistry {
        key: &'static str,
        value: ProcedureDescriptor,
    }
}

define_registry! {
    /// Pipeline Registry - ML training pipelines
    PipelineRegistry {  // NOT PipelineCatalog!
        key: String,
        value: PipelineDescriptor,
    }
}

define_registry! {
    /// Model Registry - Trained ML models
    ModelRegistry {
        key: String,
        value: TrainedModelDescriptor,
    }
}
```

### For STORAGE SIDE: **define_store!** or **define_factory!**

```rust
define_factory! {
    /// Backend Factory - Creates array backends
    BackendFactory {
        descriptor: StorageDescriptor,
        creates: Box<dyn ArrayBackend>,

        variants: [HugeArray, Arrow, Sparse],
    }
}

define_store! {
    /// Graph Catalog - Named graph instances
    GraphCatalog {
        key: String,
        value: Arc<dyn GraphStore>,

        lifecycle: {
            on_register: |name, graph| {
                log::info!("Registered graph: {} ({} nodes)",
                          name, graph.node_count());
            },
            on_remove: |name| {
                log::info!("Dropped graph: {}", name);
            },
        }
    }
}
```

## The Corrected Module Organization

```
src/projection/codegen/
├── descriptors/              # WHAT things ARE (Identity)
│   ├── computation.rs        # COMPUTATION descriptors
│   ├── storage.rs            # STORAGE descriptors
│   ├── property.rs           # Property schemas (center)
│   ├── ml/                   # ML computation descriptors
│   │   ├── pipeline.rs       # PipelineDescriptor (COMPUTATION!)
│   │   ├── model.rs
│   │   ├── step.rs
│   │   └── training.rs
│   └── procedure/            # NEW: Procedure descriptors
│       ├── descriptor.rs     # ProcedureDescriptor
│       └── category.rs
│
├── runtime/                  # HOW things EXECUTE/CREATE
│   ├── computation.rs        # COMPUTATION runtime (Computer trait)
│   ├── storage.rs            # STORAGE runtime (factories)
│   └── algorithm.rs          # NEW: AlgorithmSpec trait
│
├── macros/                   # Code generation tools
│   ├── define_registry.rs    # COMPUTATION registries macro
│   ├── define_store.rs       # STORAGE stores macro
│   ├── define_factory.rs     # STORAGE factories macro
│   └── procedure/            # Procedure-specific macros
│       ├── register.rs       # register_procedure! macro
│       └── define.rs         # define_algorithm! macro
│
└── transforms/               # Conversion utilities

src/registry/                 # NEW: All COMPUTATION registries
├── mod.rs
├── procedure.rs              # ProcedureRegistry
├── algorithm.rs              # AlgorithmRegistry
├── pipeline.rs               # PipelineRegistry (moved from eval/ml/)
└── model.rs                  # ModelRegistry

src/catalog/                  # NEW: All STORAGE catalogs/stores
├── mod.rs
└── graph.rs                  # GraphCatalog (named graphs)
```

## The Naming Convention

### COMPUTATION SIDE

- **Descriptor** suffix (AlgorithmDescriptor, PipelineDescriptor)
- **Runtime** suffix (ComputationRuntime, Computer)
- **Registry** suffix (AlgorithmRegistry, ProcedureRegistry)
- **register\_** prefix for macros (register_procedure!, register_algorithm!)

### STORAGE SIDE

- **Descriptor** suffix (StorageDescriptor, PropertyDescriptor)
- **Factory** suffix (StorageRuntimeFactory, BackendFactory)
- **Store** or **Catalog** suffix (GraphStore, PropertyStore, GraphCatalog)
- **create\_** prefix for factory methods

## What This Means for Our Work

### Rename "PipelineCatalog" → "PipelineRegistry"

```rust
// BEFORE (wrong - storage terminology)
PipelineCatalog::register(pipeline);

// AFTER (correct - computation terminology)
PipelineRegistry::register(pipeline);
```

### Separate Registry and Catalog modules

```rust
// COMPUTATION (registries)
use rust_gds::registry::{
    ProcedureRegistry,
    AlgorithmRegistry,
    PipelineRegistry,
    ModelRegistry,
};

// STORAGE (catalogs/stores)
use rust_gds::catalog::{
    GraphCatalog,
};

use rust_gds::types::{
    GraphStore,      // The storage contract
    PropertyStore,   // Column storage
};
```

## The Macro Strategy (Corrected)

### Phase 1: Build Foundation Macros (2 hours)

1. **define_registry!** - For computation registries

   - Generates: static HashMap, register/get/list functions
   - For: Algorithm, Procedure, Pipeline, Model registries

2. **define_store!** - For storage catalogs (simpler)
   - Generates: instance management, lifecycle hooks
   - For: GraphCatalog

### Phase 2: Build Specific Macros (2 hours)

3. **register_procedure!** - Register algorithms
4. **register_pipeline!** - Register ML pipelines
5. **register_model!** - Register trained models

### Phase 3: Migrate Existing Code (2 hours)

6. Rename PipelineCatalog → PipelineRegistry
7. Move to src/registry/ module
8. Apply define_registry! macro
9. Update all imports

## Tomorrow's Corrected Plan

### Morning: Get Architecture Right (3 hours)

1. **Create registry/ and catalog/ modules** (30 min)

   ```bash
   mkdir -p src/registry src/catalog
   ```

2. **Implement define_registry! macro** (1.5 hours)

   - Pattern for computation registries
   - Test with dummy registry

3. **Build ProcedureRegistry** (1 hour)
   - Create ProcedureDescriptor
   - Apply define_registry! macro
   - Add register_procedure! helper macro

### Afternoon: Migration & Cleanup (3 hours)

4. **Rename & migrate PipelineCatalog** (1 hour)

   - Rename to PipelineRegistry
   - Move to src/registry/pipeline.rs
   - Update all imports in eval/ml/

5. **Build GraphCatalog** (1 hour)

   - New src/catalog/graph.rs
   - Simple named graph storage
   - Lifecycle hooks

6. **Stub PageRank** (1 hour)
   - Basic algorithm structure
   - Register with ProcedureRegistry
   - Verify lookup works

## Success Criteria (Corrected)

✅ **Terminology Fixed**:

- COMPUTATION side uses Registry
- STORAGE side uses Store/Catalog/Factory
- Clear separation in code

✅ **Macros Working**:

- define_registry! generates computation registries
- define_store! generates storage catalogs (if needed)
- register_procedure! registers algorithms

✅ **Module Structure Clean**:

- src/registry/ for computation registries
- src/catalog/ for storage catalogs
- Clear re-exports

✅ **Existing Code Migrated**:

- PipelineRegistry (not Catalog)
- All imports updated
- Tests still pass

Thank you for bringing me back to first principles! The Computation/Storage duality is fundamental, and the terminology matters. Let's build this correctly! 🎯
