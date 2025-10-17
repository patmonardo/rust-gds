# TP-011: Unified Catalog Macro System

**Date**: October 16, 2025  
**Status**: Vision & Design Phase  
**Insight**: All our "Stores" (Catalogs) follow the same pattern - macros can unify them!

## The Big Realization

We have **multiple catalog systems** that all do the same thing:

- Register entities in a static table
- Look up by name/key
- List all registered entries
- Validate on registration
- Thread-safe access

## Current Catalog Systems

### 1. **Procedure Catalog** (Designing in TP-010)

```rust
// What we're building
static PROCEDURE_REGISTRY: Lazy<RwLock<HashMap<&'static str, ProcedureDescriptor>>> = ...;

register_procedure!(PageRank { ... });
let proc = get_procedure("pagerank");
```

### 2. **Pipeline Catalog** (ML - Already Built!)

```rust
// What we have in eval/ml/
pub struct PipelineCatalog {
    pipelines: HashMap<String, PipelineDescriptor>,
}

impl PipelineCatalog {
    pub fn register(&mut self, pipeline: PipelineDescriptor) { ... }
    pub fn get(&self, name: &str) -> Option<&PipelineDescriptor> { ... }
    pub fn list(&self) -> Vec<&str> { ... }
}
```

### 3. **GraphStore Catalog** (Needed!)

```rust
// What we need
static GRAPH_CATALOG: Lazy<RwLock<HashMap<String, Arc<dyn GraphStore>>>> = ...;

catalog::register_graph("my_graph", graph_store);
let graph = catalog::get_graph("my_graph")?;
let all_graphs = catalog::list_graphs();
```

### 4. **Model Catalog** (ML - Partially Built)

```rust
// What we have in eval/ml/models/
pub struct ModelCatalog {
    models: HashMap<String, TrainedModel>,
}

// What we need: macro-based registration
register_model!(LogisticRegression { ... });
```

### 5. **More Catalogs** (Future)

- **Projection Catalog**: Named graph projections
- **Computation Catalog**: BSP/Pregel computations
- **Transform Catalog**: Graph transformations
- **Feature Catalog**: ML feature extractors

## The Pattern: All Catalogs Are The Same!

Every catalog needs:

1. **Static Storage**

   ```rust
   static CATALOG: Lazy<RwLock<HashMap<Key, Value>>> = ...;
   ```

2. **Registration**

   ```rust
   pub fn register(key: Key, value: Value) -> Result<(), Error> {
       let mut catalog = CATALOG.write().unwrap();
       catalog.insert(key, value);
   }
   ```

3. **Lookup**

   ```rust
   pub fn get(key: &Key) -> Option<Value> {
       let catalog = CATALOG.read().unwrap();
       catalog.get(key).cloned()
   }
   ```

4. **Listing**

   ```rust
   pub fn list() -> Vec<Key> {
       let catalog = CATALOG.read().unwrap();
       catalog.keys().cloned().collect()
   }
   ```

5. **Optional: Validation, Lifecycle, Dependencies**

## The Macro Solution: `define_catalog!`

### Vision: One Macro to Rule Them All

```rust
define_catalog! {
    /// Procedure Catalog - All registered algorithms
    ProcedureCatalog {
        key: &'static str,
        value: ProcedureDescriptor,

        validate: |descriptor| {
            // Ensure name matches key
            // Check config type is valid
            // Verify modes are supported
        },

        on_register: |name, descriptor| {
            log::info!("Registered procedure: {}", name);
        },
    }
}
```

**What This Generates**:

- Static `PROCEDURE_CATALOG: Lazy<RwLock<HashMap<&'static str, ProcedureDescriptor>>>`
- `register_procedure(name, descriptor) -> Result<(), CatalogError>`
- `get_procedure(name) -> Option<ProcedureDescriptor>`
- `list_procedures() -> Vec<&'static str>`
- Optional hooks: validate, on_register, on_remove

### Example: All Our Catalogs Unified

```rust
// In src/catalog/mod.rs

use crate::projection::codegen::macros::define_catalog;

define_catalog! {
    /// Procedure Catalog
    ProcedureCatalog {
        key: &'static str,
        value: ProcedureDescriptor,
    }
}

define_catalog! {
    /// Pipeline Catalog
    PipelineCatalog {
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

define_catalog! {
    /// GraphStore Catalog
    GraphCatalog {
        key: String,
        value: Arc<dyn GraphStore>,

        on_register: |name, graph| {
            log::info!("Registered graph '{}' with {} nodes",
                      name, graph.node_count());
        },

        on_remove: |name| {
            log::info!("Removed graph '{}'", name);
        },
    }
}

define_catalog! {
    /// Model Catalog
    ModelCatalog {
        key: String,
        value: TrainedModel,

        validate: |model| {
            model.validate_trained()
        },
    }
}
```

## The Registration Macros: Catalog-Specific

Each catalog gets its own registration helper:

### `register_procedure!` (Macro)

```rust
register_procedure! {
    PageRank {
        name: "pagerank",
        category: Centrality,
        config: PageRankConfig,
        modes: [Stream, Stats, Write, Mutate],
        // ... rest of descriptor
    }
}
```

Expands to:

```rust
const _: () = {
    #[ctor::ctor]
    fn register_pagerank_procedure() {
        let descriptor = ProcedureDescriptor {
            name: "pagerank",
            category: ProcedureCategory::Centrality,
            // ... construct descriptor
        };

        ProcedureCatalog::register("pagerank", descriptor)
            .expect("Failed to register PageRank procedure");
    }
};
```

### `register_pipeline!` (Macro)

```rust
register_pipeline! {
    NodeClassificationPipeline {
        name: "node-classification-pipeline",
        type: NodeClassification { target_property: "label" },
        steps: [
            node_property("degree"),
            feature("fastRP", { dimension: 128 }),
        ],
        models: [
            logistic_regression(),
            random_forest(),
        ],
    }
}
```

### `register_graph!` (Function - Simple)

```rust
// This one doesn't need a macro - it's simple enough
graph_catalog::register("my_graph", graph_store)?;
```

## The Catalog Module Structure

```
src/catalog/
├── mod.rs                  # Public API, re-exports
├── macros.rs               # define_catalog! macro
├── error.rs                # CatalogError type
├── procedure.rs            # ProcedureCatalog (macro-generated)
├── pipeline.rs             # PipelineCatalog (macro-generated)
├── graph.rs                # GraphCatalog (macro-generated)
└── model.rs                # ModelCatalog (macro-generated)
```

### Public API (`src/catalog/mod.rs`)

```rust
//! Catalog System - Unified registration and lookup for all entities
//!
//! All catalogs follow the same pattern:
//! - Static, thread-safe storage (RwLock<HashMap>)
//! - register(), get(), list(), remove() operations
//! - Optional validation and lifecycle hooks
//!
//! Catalogs are macro-generated using `define_catalog!` for consistency.

pub mod error;
pub mod procedure;
pub mod pipeline;
pub mod graph;
pub mod model;

// Re-export catalog modules
pub use procedure::ProcedureCatalog;
pub use pipeline::PipelineCatalog;
pub use graph::GraphCatalog;
pub use model::ModelCatalog;

// Re-export error type
pub use error::CatalogError;
```

## Implementation Strategy

### Phase 1: Extract Pattern (1 hour)

1. Review existing catalog code (PipelineCatalog in eval/ml/)
2. Identify common operations
3. Design trait `Catalog<K, V>`
4. Document pattern

### Phase 2: Build `define_catalog!` Macro (2 hours)

1. Implement basic macro (key, value, storage)
2. Add validation hooks
3. Add lifecycle hooks (on_register, on_remove)
4. Test with simple example

### Phase 3: Generate Catalogs (2 hours)

1. ProcedureCatalog (use macro)
2. PipelineCatalog (migrate existing code)
3. GraphCatalog (new)
4. ModelCatalog (new)

### Phase 4: Registration Macros (2 hours)

1. `register_procedure!`
2. `register_pipeline!`
3. Test auto-registration at startup

### Phase 5: Integration (1 hour)

1. Update existing code to use catalogs
2. Replace ad-hoc registries
3. Verify all tests pass

**Total**: 8 hours

## Benefits of Unified Catalog System

### 1. **Consistency**

- All catalogs work the same way
- Same API: register, get, list, remove
- Same error handling
- Same thread-safety guarantees

### 2. **Discoverability**

- Single `catalog::` module for all lookups
- IDE autocomplete works perfectly
- Clear documentation in one place

### 3. **Reduced Boilerplate**

- `define_catalog!` generates 100+ lines per catalog
- Registration macros eliminate ceremony
- Focus on entity definition, not storage

### 4. **Type Safety**

- Compile-time validation of catalog types
- No runtime type casting needed
- Generic over key/value types

### 5. **Lifecycle Management**

- Hooks for validation, logging, metrics
- Easy to add cleanup logic
- Debugging and monitoring built-in

### 6. **Testing**

- Mock catalogs for testing
- Clear/reset for test isolation
- Inspect catalog state easily

## Example Usage (The Vision)

### Application Startup

```rust
fn main() {
    // Initialize catalog system
    catalog::init();

    // Catalogs auto-populated via #[ctor] from register_procedure! macros

    // List what's available
    println!("Procedures: {:?}", catalog::procedure::list());
    println!("Pipelines: {:?}", catalog::pipeline::list());
    println!("Graphs: {:?}", catalog::graph::list());
    println!("Models: {:?}", catalog::model::list());
}
```

### User Workflow

```rust
// 1. Load graph
let graph = RandomGraphConfig::default().build()?;
catalog::graph::register("social_network", graph)?;

// 2. Create ML pipeline
register_pipeline! {
    MyPipeline {
        name: "friend-recommender",
        type: LinkPrediction { ... },
        steps: [ ... ],
        models: [ ... ],
    }
}

// 3. Run algorithm
let pagerank_proc = catalog::procedure::get("pagerank")?;
let results = pagerank_proc.execute(&graph, config)?;

// 4. Train model
let pipeline = catalog::pipeline::get("friend-recommender")?;
let model = pipeline.train(&graph)?;
catalog::model::register("friend-model-v1", model)?;

// 5. Use model
let model = catalog::model::get("friend-model-v1")?;
let predictions = model.predict(&graph)?;
```

## Tomorrow's Work

### Review src/procedure/ Work (30 min)

- Check what we built today
- Identify what's missing
- Document current state

### Design define_procedure! Macro (2 hours)

- Precise syntax design
- Example usage for 3 algorithms
- Implementation sketch

### Design define_catalog! Macro (2 hours)

- Extract pattern from existing catalogs
- Design macro syntax
- Plan generated code structure

### Build First Catalog (1 hour)

- Implement ProcedureCatalog using macro
- Test registration and lookup
- Verify thread safety

## The Big Picture

```
Unified Catalog System
    ↓
All "Stores" Use Same Pattern
    ↓
define_catalog! Macro Generates Boilerplate
    ↓
Specific Registration Macros
    (register_procedure!, register_pipeline!, etc.)
    ↓
Clean, Consistent API
    ↓
Easy to Add New Entity Types
```

This is the **Projection** principle in action:

- One pattern (Catalog) projects into many forms (Procedure, Pipeline, Graph, Model)
- Macros are the projection mechanism
- Clean, consistent runtime APIs

## Success Criteria

✅ **Single Pattern**:

- All catalogs use `define_catalog!`
- Same API surface
- Same guarantees (thread-safe, validated)

✅ **Clean Registration**:

- Entity-specific macros (register_procedure!, etc.)
- Auto-registration at startup
- No manual catalog management

✅ **Discoverable**:

- Single catalog:: namespace
- Clear documentation
- IDE support works perfectly

✅ **Tested**:

- Unit tests for each catalog
- Integration tests for registration
- Thread-safety verified

Let's unify the catalog system! 🚀
