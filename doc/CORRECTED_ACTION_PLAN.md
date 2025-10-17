# CORRECTED ACTION PLAN: Computation Registries & Storage Stores

**Date**: October 16, 2025  
**Architecture**: Computation/Storage Duality  
**Insight**: Registries (computation) vs Stores (storage)

## The Fundamental Distinction

```
COMPUTATION SIDE          |  STORAGE SIDE
--------------------------|---------------------------
Descriptors (WHAT)        |  Descriptors (WHAT)
Runtime (HOW executes)    |  Factories (HOW creates)
REGISTRIES (lookup)       |  STORES/CATALOGS (lookup)
                          |
Examples:                 |  Examples:
- AlgorithmRegistry       |  - GraphStore
- ProcedureRegistry       |  - GraphCatalog
- PipelineRegistry        |  - PropertyStore
- ModelRegistry           |  - BackendFactory
```

## Phase 1: Build Registry Macro System (2-3 hours)

### Step 1.1: Create Module Structure (15 min)

```bash
# Create new modules
mkdir -p src/registry
mkdir -p src/catalog

# Create module files
touch src/registry/mod.rs
touch src/registry/procedure.rs
touch src/registry/algorithm.rs
touch src/registry/pipeline.rs
touch src/registry/model.rs

touch src/catalog/mod.rs
touch src/catalog/graph.rs
```

### Step 1.2: Implement define_registry! Macro (1.5 hours)

**File**: `src/projection/codegen/macros/registry.rs`

**What it generates**:

```rust
// INPUT: define_registry! call
define_registry! {
    ProcedureRegistry {
        key: &'static str,
        value: ProcedureDescriptor,
    }
}

// OUTPUT: Generated code
static PROCEDURE_REGISTRY: Lazy<RwLock<HashMap<&'static str, ProcedureDescriptor>>> = ...;

pub fn register(name: &'static str, descriptor: ProcedureDescriptor) -> Result<(), RegistryError>;
pub fn get(name: &str) -> Option<ProcedureDescriptor>;
pub fn list() -> Vec<&'static str>;
pub fn remove(name: &str) -> Option<ProcedureDescriptor>;
```

**Test**: Create dummy registry, verify register/get/list works

### Step 1.3: Create ProcedureDescriptor (1 hour)

**File**: `src/projection/codegen/descriptors/procedure/descriptor.rs`

**Structure**:

```rust
pub struct ProcedureDescriptor {
    /// Algorithm name (e.g., "pagerank")
    pub name: &'static str,

    /// Category (Centrality, Community, etc.)
    pub category: ProcedureCategory,

    /// Description
    pub description: &'static str,

    /// Config type ID
    pub config_type: TypeId,

    /// Execution modes supported
    pub modes: &'static [ExecutionMode],

    /// Memory estimator function
    pub estimate_memory: MemoryEstimator,

    /// Validation configuration
    pub validators: ValidationConfiguration,
}
```

**Reference**: Java GDS AlgorithmSpec pattern

## Phase 2: Build ProcedureRegistry (1-2 hours)

### Step 2.1: Apply Macro to Create Registry (30 min)

**File**: `src/registry/procedure.rs`

```rust
use crate::projection::codegen::macros::define_registry;
use crate::projection::codegen::descriptors::procedure::ProcedureDescriptor;

define_registry! {
    ProcedureRegistry {
        key: &'static str,
        value: ProcedureDescriptor,

        validate: |descriptor| {
            // Ensure name is not empty
            // Check modes are valid
            // Verify config type
        },
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_lookup() {
        let descriptor = ProcedureDescriptor { /* dummy */ };

        register("test_algo", descriptor).unwrap();

        let found = get("test_algo");
        assert!(found.is_some());
    }
}
```

### Step 2.2: Build register_procedure! Helper Macro (1 hour)

**File**: `src/projection/codegen/macros/procedure/register.rs`

**Syntax**:

```rust
register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        description: "Computes PageRank centrality scores",
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],

        memory_estimate: |config: &PageRankConfig, graph: &dyn GraphStore| {
            let nodes = graph.node_count();
            MemoryEstimate::from_bytes(nodes * 16) // 2 f64 arrays
        },

        validators: ValidationConfiguration::new()
            .add_before_load(RangeValidator::new("dampingFactor", 0.0, 1.0))
            .add_after_load(GraphNotEmptyValidator),
    }
}
```

**Expands to**:

```rust
const _: () = {
    #[ctor::ctor]
    fn register_pagerank_procedure() {
        let descriptor = ProcedureDescriptor {
            name: "pagerank",
            category: ProcedureCategory::Centrality,
            description: "Computes PageRank centrality scores",
            config_type: TypeId::of::<PageRankConfig>(),
            modes: &[
                ExecutionMode::Stream,
                ExecutionMode::Stats,
                ExecutionMode::Write,
                ExecutionMode::Mutate,
            ],
            estimate_memory: |config, graph| {
                let nodes = graph.node_count();
                MemoryEstimate::from_bytes(nodes * 16)
            },
            validators: ValidationConfiguration::new()
                .add_before_load(RangeValidator::new("dampingFactor", 0.0, 1.0))
                .add_after_load(GraphNotEmptyValidator),
        };

        registry::procedure::register("pagerank", descriptor)
            .expect("Failed to register PageRank");
    }
};
```

## Phase 3: Migrate PipelineCatalog → PipelineRegistry (1 hour)

### Step 3.1: Rename and Move (20 min)

```bash
# Current location
src/projection/eval/ml/pipeline/pipeline_catalog.rs

# Target location
src/registry/pipeline.rs

# Strategy: Copy, apply macro, test, then delete old
```

### Step 3.2: Apply define_registry! (20 min)

**Before** (hand-written):

```rust
pub struct PipelineCatalog {
    pipelines_by_name: HashMap<String, PipelineCatalogEntry>,
    // ...
}

impl PipelineCatalog {
    pub fn register(&mut self, name: String, pipeline: PipelineDescriptor) { ... }
    pub fn get(&self, name: &str) -> Option<PipelineCatalogEntry> { ... }
    // ... lots of boilerplate
}
```

**After** (macro-generated):

```rust
use crate::projection::codegen::macros::define_registry;
use crate::projection::codegen::descriptors::ml::PipelineDescriptor;

define_registry! {
    PipelineRegistry {
        key: String,
        value: PipelineDescriptor,

        validate: |pipeline| {
            if pipeline.steps.is_empty() {
                return Err("Pipeline must have at least one step");
            }
            Ok(())
        },
    }
}
```

### Step 3.3: Update All Imports (20 min)

```bash
# Find all uses
grep -r "PipelineCatalog" src/

# Replace with sed
find src -name "*.rs" -exec sed -i 's/PipelineCatalog/PipelineRegistry/g' {} +

# Verify
cargo build --all-features
```

## Phase 4: Build GraphCatalog (Storage Side) (30 min)

### Step 4.1: Create GraphCatalog (Storage Catalog)

**File**: `src/catalog/graph.rs`

```rust
use crate::types::prelude::GraphStore;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// GraphCatalog - Named graph storage (STORAGE side)
static GRAPH_CATALOG: Lazy<RwLock<HashMap<String, Arc<dyn GraphStore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn register(name: String, graph: Arc<dyn GraphStore>) -> Result<(), CatalogError> {
    let mut catalog = GRAPH_CATALOG.write().unwrap();

    if catalog.contains_key(&name) {
        return Err(CatalogError::AlreadyExists(name));
    }

    log::info!("Registering graph '{}' with {} nodes, {} edges",
               name, graph.node_count(), graph.edge_count());

    catalog.insert(name, graph);
    Ok(())
}

pub fn get(name: &str) -> Option<Arc<dyn GraphStore>> {
    let catalog = GRAPH_CATALOG.read().unwrap();
    catalog.get(name).cloned()
}

pub fn list() -> Vec<String> {
    let catalog = GRAPH_CATALOG.read().unwrap();
    catalog.keys().cloned().collect()
}

pub fn remove(name: &str) -> Option<Arc<dyn GraphStore>> {
    let mut catalog = GRAPH_CATALOG.write().unwrap();

    if let Some(graph) = catalog.remove(name) {
        log::info!("Removed graph '{}'", name);
        Some(graph)
    } else {
        None
    }
}
```

**Note**: This is STORAGE side (catalog), not COMPUTATION side (registry)!

## Phase 5: Stub PageRank Algorithm (1 hour)

### Step 5.1: Create Algorithm Structure (30 min)

**File**: `src/procedure/algo/centrality/pagerank.rs`

```rust
use crate::config::algo_config::PageRankConfig;
use crate::projection::codegen::runtime::algorithm::AlgorithmSpec;
use crate::registry::procedure;
use crate::types::prelude::*;

/// PageRank algorithm implementation
pub struct PageRank {
    config: PageRankConfig,
    graph: Arc<dyn GraphStore>,
}

impl AlgorithmSpec for PageRank {
    type Output = Vec<(NodeId, f64)>;

    fn name(&self) -> &str {
        "pagerank"
    }

    fn graph_name(&self) -> &str {
        // TODO: Get from config or context
        "default"
    }

    fn execute(
        &self,
        context: &ExecutionContext,
    ) -> Result<Self::Output, ExecutionError> {
        // Stub implementation
        let node_count = self.graph.node_count();
        let scores = vec![1.0 / node_count as f64; node_count];

        Ok((0..node_count)
            .map(|i| (i as NodeId, scores[i]))
            .collect())
    }
}
```

### Step 5.2: Register PageRank (30 min)

```rust
// At end of pagerank.rs

use crate::projection::codegen::macros::procedure::register_procedure;

register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        description: "Computes PageRank centrality scores for all nodes",
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],

        memory_estimate: |config, graph| {
            let nodes = graph.node_count();
            // 2 f64 arrays (scores + deltas) + degree array (usize)
            MemoryEstimate::from_bytes(nodes * (2 * 8 + 8))
        },

        validators: ValidationConfiguration::new()
            .add_before_load(RangeValidator::new("dampingFactor", 0.0, 1.0))
            .add_before_load(RangeValidator::new("tolerance", 0.0, 1.0))
            .add_before_load(RangeValidator::new("maxIterations", 1.0, 1000.0))
            .add_after_load(GraphNotEmptyValidator),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_registered() {
        // Verify PageRank is in registry
        let descriptor = registry::procedure::get("pagerank");
        assert!(descriptor.is_some());
        assert_eq!(descriptor.unwrap().name, "pagerank");
    }
}
```

## Module Organization (Final)

```
src/
├── registry/                    # COMPUTATION REGISTRIES
│   ├── mod.rs
│   ├── procedure.rs             # ProcedureRegistry
│   ├── algorithm.rs             # AlgorithmRegistry (alias/re-export?)
│   ├── pipeline.rs              # PipelineRegistry (moved from eval/ml/)
│   └── model.rs                 # ModelRegistry
│
├── catalog/                     # STORAGE CATALOGS/STORES
│   ├── mod.rs
│   └── graph.rs                 # GraphCatalog (named graphs)
│
├── procedure/                   # Algorithm implementations
│   ├── mod.rs
│   ├── core/                    # Shared utilities
│   └── algo/
│       ├── centrality/
│       │   └── pagerank.rs      # PageRank algorithm
│       ├── community/
│       └── pathfinding/
│
└── projection/codegen/
    ├── descriptors/
    │   ├── computation.rs       # ComputationDescriptor
    │   ├── storage.rs           # StorageDescriptor
    │   ├── procedure/           # ProcedureDescriptor
    │   └── ml/                  # Pipeline/Model descriptors
    │
    ├── runtime/
    │   ├── computation.rs       # Computer trait
    │   ├── storage.rs           # StorageRuntimeFactory
    │   └── algorithm.rs         # AlgorithmSpec trait
    │
    └── macros/
        ├── registry.rs          # define_registry! macro
        ├── store.rs             # define_store! macro (optional)
        └── procedure/
            ├── register.rs      # register_procedure! macro
            └── define.rs        # define_algorithm! macro (later)
```

## Success Criteria (Corrected)

✅ **Terminology Correct**:

- [ ] COMPUTATION side: Registries (procedure, algorithm, pipeline, model)
- [ ] STORAGE side: Stores/Catalogs (graph catalog)
- [ ] Clear distinction in code and docs

✅ **Macros Working**:

- [ ] define_registry! generates computation registries
- [ ] register_procedure! registers algorithms
- [ ] PipelineRegistry using define_registry!

✅ **Structure Clean**:

- [ ] src/registry/ for computation registries
- [ ] src/catalog/ for storage catalogs
- [ ] Clear separation and re-exports

✅ **Code Migrated**:

- [ ] PipelineCatalog → PipelineRegistry
- [ ] ProcedureRegistry created
- [ ] PageRank stubbed and registered
- [ ] All tests pass

## Timeline (Revised)

- **Phase 1** (Registry macro): 2-3 hours
- **Phase 2** (ProcedureRegistry): 1-2 hours
- **Phase 3** (Migrate Pipeline): 1 hour
- **Phase 4** (GraphCatalog): 30 min
- **Phase 5** (PageRank stub): 1 hour

**Total**: 5.5-7.5 hours

## Key Insights

1. **Computation vs Storage** is fundamental
2. **Registries** (computation) ≠ **Catalogs** (storage)
3. **PipelineDescriptor** is a COMPUTATION descriptor (goes in registry!)
4. **GraphStore** is STORAGE (goes in catalog!)
5. Macros should respect this distinction

Tomorrow we build it right! 🎯
