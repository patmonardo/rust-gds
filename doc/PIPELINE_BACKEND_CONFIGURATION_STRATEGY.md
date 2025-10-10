# Pipeline Backend Configuration Strategy

**Date**: October 10, 2025  
**Status**: Strategic Design - Eval Macro Phase 2  
**Context**: Post-Clippy Cleanup, Pre-Migration Phase

---

## 🎯 The Core Challenge

**User's Question**:

> "The other issue it has to help us with is a general Pipeline Configuration issue. And how to wire up GraphStores with HugeArray or Virtual Threads at the Type system level. Arrow Arrays basically wiring up Type Systems for individual ML Pipelines."

**Translation**: We need the eval macro system to solve:

1. **Backend Selection** - Choose storage backend (HugeArray vs Arrow vs Sparse) per ML pipeline
2. **Thread Model** - Wire up real threads vs virtual threads at type level
3. **Type System Flexibility** - Make backend choice compile-time configurable without duplicating logic
4. **ML Pipeline Isolation** - Different algorithms can use different backends optimally

---

## 🌉 The Strategic Vision

### What We're Building

```
┌──────────────────────────────────────────────────────────────────┐
│                    ML PIPELINE CONFIGURATION                      │
│  "PageRank with Arrow backend on 8 virtual threads"             │
│  "Louvain with HugeArray backend on 32 real threads"            │
│  "NodeSimilarity with Sparse backend, single-threaded"          │
└──────────────────────────────────────────────────────────────────┘
                              ↓
                    Eval Macro System
              (Single Source, Multiple Backends)
                              ↓
        ┌─────────────────────┴─────────────────────┐
        ↓                                           ↓
┌──────────────────┐                      ┌──────────────────┐
│ Backend Policy   │                      │ Execution Policy │
│ - HugeArray      │                      │ - Real Threads   │
│ - Arrow2         │                      │ - Virtual Threads│
│ - Sparse         │                      │ - Single-thread  │
│ - Compressed     │                      │ - GPU (future)   │
└──────────────────┘                      └──────────────────┘
```

### Why This Matters for ML Pipelines

Different algorithms have **radically different** storage/execution needs:

| Algorithm             | Data Pattern               | Optimal Backend              | Thread Model                       |
| --------------------- | -------------------------- | ---------------------------- | ---------------------------------- |
| **PageRank**          | Dense reads, sparse writes | HugeArray (cursor iteration) | Real threads (I/O bound)           |
| **Louvain**           | Community clusters         | Sparse (HashMap)             | Virtual threads (many communities) |
| **BFS**               | Sequential access          | Arrow (zero-copy)            | Single-threaded (sequential)       |
| **Node Similarity**   | Dense matrix ops           | HugeArray (SIMD-friendly)    | Real threads (CPU bound)           |
| **Label Propagation** | High churn                 | Compressed (LZ4)             | Virtual threads (many labels)      |

**Key Insight**: One-size-fits-all backend = 10-100x performance loss for edge cases!

---

## 🔧 The Architecture: Backend as Type Parameter

### Current State (Inflexible)

```rust
// Hard-coded to HugeArray:
pub struct DefaultLongNodePropertyValues {
    values: HugeLongArray,  // ← Compiled-in choice!
    node_count: usize,
}

// Want Arrow? Copy-paste entire implementation:
pub struct ArrowLongNodePropertyValues {
    values: ArrowLongArray,  // ← Duplicate logic!
    node_count: usize,
}

// Want Sparse? Copy-paste again:
pub struct SparseLongNodePropertyValues {
    values: SparseLongArray,  // ← More duplication!
    node_count: usize,
}
```

**Problem**: 3 backends × 8 types = 24 struct definitions! (And we haven't even added compression, mmap, GPU...)

### Target State (Flexible via Macro)

```rust
// Eval macro generates ONCE with backend parameter:
value_type_table! {
    Long => {
        subtle_type: i64,
        gross_adapters: {
            HugeArray => HugeLongArray,      // Default: in-memory pages
            Arrow => ArrowLongArray,          // Zero-copy mmap
            Sparse => SparseLongArray,        // HashMap for sparse data
            Compressed => CompressedLongArray // LZ4/Zstd for cold data
        },
        storage_hint: StorageHint::FixedWidth,
    },
    // ... other types
}
```

**Generated Output** (macro creates):

```rust
// Backend trait (unified interface):
pub trait LongBackend: Send + Sync {
    fn get(&self, index: usize) -> i64;
    fn set(&mut self, index: usize, value: i64);
    fn len(&self) -> usize;
    fn cursor(&self) -> Option<Box<dyn Iterator<Item = i64>>> {
        None  // Optional: backends can provide optimized cursors
    }
}

// HugeArray backend implementation:
impl LongBackend for HugeLongArray {
    fn get(&self, index: usize) -> i64 { self.get(index) }
    fn set(&mut self, index: usize, value: i64) { self.set(index, value) }
    fn len(&self) -> usize { self.size() }
    fn cursor(&self) -> Option<Box<dyn Iterator<Item = i64>>> {
        Some(Box::new(self.cursor()))  // ← HugeArray has optimized cursor!
    }
}

// Arrow backend implementation:
impl LongBackend for ArrowLongArray {
    fn get(&self, index: usize) -> i64 { self.value(index) }
    fn set(&mut self, _index: usize, _value: i64) {
        panic!("Arrow arrays are immutable")
    }
    fn len(&self) -> usize { self.len() }
    // No cursor - Arrow doesn't provide one
}

// Sparse backend implementation:
impl LongBackend for SparseLongArray {
    fn get(&self, index: usize) -> i64 {
        self.data.get(&index).copied().unwrap_or(self.default)
    }
    fn set(&mut self, index: usize, value: i64) {
        if value != self.default {
            self.data.insert(index, value);
        }
    }
    fn len(&self) -> usize { self.len }
    // No cursor - sparse iteration is non-contiguous
}

// Generic PropertyValues (works with ANY backend):
pub struct LongNodePropertyValues<B: LongBackend> {
    backend: B,
    node_count: usize,
    _phantom: PhantomData<B>,
}

impl<B: LongBackend> LongNodePropertyValues<B> {
    pub fn new(backend: B, node_count: usize) -> Self {
        Self {
            backend,
            node_count,
            _phantom: PhantomData,
        }
    }
}

impl<B: LongBackend> NodePropertyValues for LongNodePropertyValues<B> {
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        let idx = form_processor::checked_u64_to_usize(node_id)?;
        if idx < self.backend.len() {
            Ok(self.backend.get(idx))
        } else {
            Err(PropertyValuesError::IndexOutOfBounds {
                index: node_id,
                size: self.backend.len(),
            })
        }
    }
}
```

---

## 🔀 Pipeline Configuration: Compile-Time + Runtime

### Level 1: StorageHint (Macro Metadata)

**Purpose**: Hint to eval macro for optimal backend selection

```rust
pub enum StorageHint {
    FixedWidth,      // i64, f64 → HugeArray (dense, fixed-size)
    VariableLength,  // String → Arrow (zero-copy buffers)
    ListAsOffsets,   // [i64] → Arrow offsets (columnar)
    ColumnarStruct,  // {x, y, z} → Arrow struct
    SerializedRow,   // Complex types → bincode/serde
    Sparse,          // HashMap (explicit hint for sparse data)
    Compressed,      // LZ4/Zstd (cold data)
}
```

**Eval Macro Uses This**:

```rust
value_type_table! {
    Long => {
        subtle_type: i64,
        storage_hint: StorageHint::FixedWidth,  // ← Macro sees this
        default_backend: HugeArray,              // ← Default choice
        // Macro generates specialized code for FixedWidth patterns
    },
    StringProp => {
        subtle_type: String,
        storage_hint: StorageHint::VariableLength,  // ← Different hint
        default_backend: Arrow,                      // ← Arrow optimal for strings
        // Macro generates zero-copy buffer handling
    },
}
```

### Level 2: GraphStoreConfig (Runtime Selection)

**Purpose**: Per-instance backend selection

```rust
#[derive(Debug, Clone)]
pub struct GraphStoreBackendConfig {
    /// Node property storage backend
    pub node_properties: BackendChoice,

    /// Relationship property storage backend
    pub relationship_properties: BackendChoice,

    /// Topology storage backend
    pub topology: BackendChoice,
}

#[derive(Debug, Clone)]
pub enum BackendChoice {
    /// In-memory paged arrays (default)
    HugeArray,

    /// Zero-copy Arrow2 (mmap-friendly)
    Arrow { path: Option<PathBuf> },

    /// Sparse storage (HashMap-based)
    Sparse { load_factor: f64 },

    /// Compressed (LZ4/Zstd)
    Compressed { codec: CompressionCodec },

    /// Auto-select based on density
    Auto { density_threshold: f64 },
}

impl Default for GraphStoreBackendConfig {
    fn default() -> Self {
        Self {
            node_properties: BackendChoice::HugeArray,
            relationship_properties: BackendChoice::HugeArray,
            topology: BackendChoice::HugeArray,
        }
    }
}
```

**Usage**:

```rust
// Example 1: PageRank with HugeArray (default, optimal for dense iteration)
let config = GraphStoreConfig::builder()
    .backend(GraphStoreBackendConfig::default())
    .build()?;

// Example 2: BFS with Arrow (zero-copy, mmap-friendly)
let config = GraphStoreConfig::builder()
    .backend(GraphStoreBackendConfig {
        node_properties: BackendChoice::Arrow {
            path: Some(PathBuf::from("/mnt/graph/nodes.arrow"))
        },
        relationship_properties: BackendChoice::Arrow {
            path: Some(PathBuf::from("/mnt/graph/rels.arrow"))
        },
        topology: BackendChoice::HugeArray,  // Keep topology in memory
    })
    .build()?;

// Example 3: Louvain with Sparse (many small communities)
let config = GraphStoreConfig::builder()
    .backend(GraphStoreBackendConfig {
        node_properties: BackendChoice::Sparse { load_factor: 0.1 },
        relationship_properties: BackendChoice::HugeArray,
        topology: BackendChoice::HugeArray,
    })
    .build()?;

// Example 4: Auto-selection based on density
let config = GraphStoreConfig::builder()
    .backend(GraphStoreBackendConfig {
        node_properties: BackendChoice::Auto {
            density_threshold: 0.5  // Use HugeArray if >50% dense, else Sparse
        },
        relationship_properties: BackendChoice::Auto { density_threshold: 0.3 },
        topology: BackendChoice::HugeArray,
    })
    .build()?;
```

### Level 3: AlgorithmConfig (Per-Pipeline Execution)

**Purpose**: Algorithm-specific backend hints

```rust
#[derive(Debug, Clone)]
pub struct PageRankConfig {
    pub base: AlgoBaseConfig,
    pub max_iterations: u32,
    pub tolerance: f64,

    /// Execution config (threads, backend hints)
    pub execution: ExecutionConfig,
}

#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    /// Thread model
    pub thread_model: ThreadModel,

    /// Backend hints for intermediate results
    pub intermediate_storage: BackendChoice,

    /// Output backend
    pub output_backend: BackendChoice,
}

#[derive(Debug, Clone)]
pub enum ThreadModel {
    /// OS threads (std::thread) - best for CPU-bound work
    RealThreads { count: usize },

    /// Virtual threads (future, requires runtime) - best for I/O-bound work
    VirtualThreads { count: usize },

    /// Single-threaded (deterministic, debuggable)
    SingleThreaded,

    /// GPU (future)
    Gpu { device_id: usize },
}
```

**Usage**:

```rust
// PageRank: Dense iteration, CPU-bound → HugeArray + Real threads
let pagerank_config = PageRankConfig::builder()
    .max_iterations(20)
    .execution(ExecutionConfig {
        thread_model: ThreadModel::RealThreads { count: 8 },
        intermediate_storage: BackendChoice::HugeArray,  // Cursor-friendly
        output_backend: BackendChoice::Arrow {           // Zero-copy export
            path: Some(PathBuf::from("pagerank_scores.arrow"))
        },
    })
    .build()?;

// Louvain: Many communities, I/O-bound → Sparse + Virtual threads
let louvain_config = LouvainConfig::builder()
    .max_levels(10)
    .execution(ExecutionConfig {
        thread_model: ThreadModel::VirtualThreads { count: 1000 },  // Many communities
        intermediate_storage: BackendChoice::Sparse { load_factor: 0.05 },
        output_backend: BackendChoice::HugeArray,  // Dense final result
    })
    .build()?;
```

---

## 🏗️ Implementation Strategy

### Phase 1: Backend Trait System (Week 1)

**Goal**: Unified backend interface for all types

**Tasks**:

1. **Define backend traits** (1 day):

   ```rust
   // src/projection/backends/mod.rs
   pub trait Backend<T>: Send + Sync {
       fn get(&self, index: usize) -> T;
       fn set(&mut self, index: usize, value: T);
       fn len(&self) -> usize;

       // Optional optimizations:
       fn cursor(&self) -> Option<Box<dyn Iterator<Item = T>>> { None }
       fn slice(&self, offset: usize, length: usize) -> Option<Self> where Self: Sized { None }
   }
   ```

2. **Implement for HugeArray** (1 day):

   ```rust
   // src/projection/backends/huge_array.rs
   impl Backend<i64> for HugeLongArray { /* ... */ }
   impl Backend<f64> for HugeDoubleArray { /* ... */ }
   ```

3. **Implement for Arrow2** (1 day):

   ```rust
   // src/projection/backends/arrow.rs
   impl Backend<i64> for ArrowLongArray { /* ... */ }
   impl Backend<f64> for ArrowDoubleArray { /* ... */ }
   ```

4. **Implement for Sparse** (1 day):

   ```rust
   // src/projection/backends/sparse.rs
   pub struct SparseBackend<T> {
       data: HashMap<usize, T>,
       default: T,
       len: usize,
   }
   impl<T: Clone> Backend<T> for SparseBackend<T> { /* ... */ }
   ```

5. **Update eval macro to generate backend-agnostic code** (1 day):
   ```rust
   value_type_table! {
       Long => {
           subtle_type: i64,
           backends: [HugeArray, Arrow, Sparse],  // ← List of backends
           default_backend: HugeArray,
       },
   }
   // Macro generates: LongNodePropertyValues<B: Backend<i64>>
   ```

### Phase 2: GraphStoreConfig Integration (Week 2)

**Goal**: Runtime backend selection per GraphStore instance

**Tasks**:

1. **Add GraphStoreBackendConfig** (1 day):

   ```rust
   // src/config/backend_config.rs
   pub struct GraphStoreBackendConfig { /* ... */ }
   pub enum BackendChoice { /* ... */ }
   ```

2. **Wire into GraphStore constructor** (1 day):

   ```rust
   // src/types/graph_store/default_graph_store.rs
   impl DefaultGraphStore {
       pub fn new(config: GraphStoreConfig) -> Result<Self, GraphStoreError> {
           let backend = match config.backend.node_properties {
               BackendChoice::HugeArray => /* create HugeArray backend */,
               BackendChoice::Arrow { path } => /* create Arrow backend */,
               BackendChoice::Sparse { load_factor } => /* create Sparse backend */,
               BackendChoice::Auto { density_threshold } => /* measure density, decide */,
           };
           // ...
       }
   }
   ```

3. **Update property creation** (1 day):

   ```rust
   // src/types/properties/node/factory.rs
   pub fn create_long_property<B: Backend<i64>>(
       backend: B,
       node_count: usize,
   ) -> Arc<dyn NodePropertyValues> {
       Arc::new(LongNodePropertyValues::new(backend, node_count))
   }
   ```

4. **Add density analysis helper** (1 day):

   ```rust
   // src/config/backend_selection.rs
   pub fn analyze_density(values: &[Option<i64>]) -> f64 {
       let non_null = values.iter().filter(|v| v.is_some()).count();
       non_null as f64 / values.len() as f64
   }

   pub fn recommend_backend(density: f64) -> BackendChoice {
       if density > 0.7 {
           BackendChoice::HugeArray  // Dense → HugeArray
       } else if density > 0.1 {
           BackendChoice::Arrow { path: None }  // Medium → Arrow
       } else {
           BackendChoice::Sparse { load_factor: density }  // Sparse → HashMap
       }
   }
   ```

5. **Tests and examples** (1 day):
   ```rust
   // examples/backend_selection.rs
   fn main() {
       // Demonstrate each backend
       // Benchmark performance differences
       // Show density-based selection
   }
   ```

### Phase 3: Thread Model Integration (Week 3)

**Goal**: Wire thread model into algorithm execution

**Tasks**:

1. **Define ThreadModel enum** (1 day):

   ```rust
   // src/config/execution_config.rs
   pub enum ThreadModel { /* ... */ }
   ```

2. **Create thread pool abstraction** (2 days):

   ```rust
   // src/core/executor/mod.rs
   pub trait Executor: Send + Sync {
       fn execute<F>(&self, tasks: Vec<F>) -> Vec<F::Output>
       where
           F: FnOnce() -> F::Output + Send,
           F::Output: Send;
   }

   pub struct RealThreadExecutor { pool: ThreadPool }
   pub struct SingleThreadedExecutor;
   // pub struct VirtualThreadExecutor (future)
   ```

3. **Wire into Pregel executor** (1 day):

   ```rust
   // src/pregel/executor.rs
   impl<C: ComputeContext> PregelExecutor<C> {
       pub fn with_thread_model(mut self, model: ThreadModel) -> Self {
           self.executor = match model {
               ThreadModel::RealThreads { count } => {
                   Box::new(RealThreadExecutor::new(count))
               }
               ThreadModel::SingleThreaded => {
                   Box::new(SingleThreadedExecutor)
               }
               _ => panic!("Not implemented yet"),
           };
           self
       }
   }
   ```

4. **Update algorithm configs** (1 day):

   ```rust
   // src/config/algorithms/*.rs
   // Add ExecutionConfig field to all algorithm configs
   ```

5. **Examples and benchmarks** (1 day):
   ```rust
   // examples/thread_model_comparison.rs
   fn main() {
       // Run PageRank with 1, 4, 8, 16 threads
       // Compare real threads vs single-threaded
       // Measure overhead
   }
   ```

### Phase 4: Polish & Documentation (Week 4)

1. Update copilot-instructions.md with backend patterns
2. Create backend_selection_guide.md
3. Add integration tests for all backend combinations
4. Performance benchmarks (HugeArray vs Arrow vs Sparse)
5. Migration guide for existing code

---

## 🎓 Examples: End-to-End ML Pipelines

### Example 1: PageRank (Dense, CPU-bound)

```rust
use rust_gds::prelude::*;

fn run_pagerank() -> Result<(), Box<dyn Error>> {
    // Configure GraphStore with HugeArray backend (optimal for dense iteration)
    let graph_config = GraphStoreConfig::builder()
        .backend(GraphStoreBackendConfig {
            node_properties: BackendChoice::HugeArray,
            relationship_properties: BackendChoice::HugeArray,
            topology: BackendChoice::HugeArray,
        })
        .build()?;

    let graph = GraphStore::new(graph_config)?;

    // Configure PageRank with real threads (CPU-bound)
    let pagerank_config = PageRankConfig::builder()
        .max_iterations(20)
        .tolerance(0.0001)
        .execution(ExecutionConfig {
            thread_model: ThreadModel::RealThreads { count: 8 },
            intermediate_storage: BackendChoice::HugeArray,  // Cursor iteration
            output_backend: BackendChoice::Arrow {            // Zero-copy export
                path: Some(PathBuf::from("pagerank.arrow"))
            },
        })
        .build()?;

    // Run algorithm
    let scores = pregel::pagerank(&graph, pagerank_config)?;

    println!("PageRank complete: {} scores", scores.element_count());
    Ok(())
}
```

### Example 2: Louvain (Sparse, I/O-bound)

```rust
fn run_louvain() -> Result<(), Box<dyn Error>> {
    // Configure GraphStore with sparse backend (many small communities)
    let graph_config = GraphStoreConfig::builder()
        .backend(GraphStoreBackendConfig {
            node_properties: BackendChoice::Sparse { load_factor: 0.05 },
            relationship_properties: BackendChoice::HugeArray,
            topology: BackendChoice::HugeArray,
        })
        .build()?;

    let graph = GraphStore::new(graph_config)?;

    // Configure Louvain with virtual threads (I/O-bound, many communities)
    let louvain_config = LouvainConfig::builder()
        .max_levels(10)
        .execution(ExecutionConfig {
            thread_model: ThreadModel::VirtualThreads { count: 1000 },
            intermediate_storage: BackendChoice::Sparse { load_factor: 0.05 },
            output_backend: BackendChoice::HugeArray,  // Dense final result
        })
        .build()?;

    // Run algorithm
    let communities = pregel::louvain(&graph, louvain_config)?;

    println!("Louvain complete: {} communities", communities.element_count());
    Ok(())
}
```

### Example 3: BFS (Sequential, Zero-copy)

```rust
fn run_bfs() -> Result<(), Box<dyn Error>> {
    // Configure GraphStore with Arrow backend (zero-copy, mmap-friendly)
    let graph_config = GraphStoreConfig::builder()
        .backend(GraphStoreBackendConfig {
            node_properties: BackendChoice::Arrow {
                path: Some(PathBuf::from("/mnt/graph/nodes.arrow"))
            },
            relationship_properties: BackendChoice::Arrow {
                path: Some(PathBuf::from("/mnt/graph/rels.arrow"))
            },
            topology: BackendChoice::Arrow {
                path: Some(PathBuf::from("/mnt/graph/topology.arrow"))
            },
        })
        .build()?;

    let graph = GraphStore::new(graph_config)?;

    // Configure BFS with single-threaded (sequential, deterministic)
    let bfs_config = BfsConfig::builder()
        .start_node(0)
        .execution(ExecutionConfig {
            thread_model: ThreadModel::SingleThreaded,  // Sequential access
            intermediate_storage: BackendChoice::HugeArray,  // Small working set
            output_backend: BackendChoice::Arrow {           // Zero-copy export
                path: Some(PathBuf::from("bfs_distances.arrow"))
            },
        })
        .build()?;

    // Run algorithm
    let distances = algorithms::bfs(&graph, bfs_config)?;

    println!("BFS complete: {} distances", distances.element_count());
    Ok(())
}
```

---

## 🧭 Philosophical Alignment (Yoga Sutra 3.44)

**Sanskrit**: _"Sthūla-svarūpa-sūkṣmānvayārthavattva-saṃyamāt bhūta-jayaḥ"_

**Translation**: "By saṃyama on the gross (sthūla), essential form (svarūpa), subtle (sūkṣma), connection (anvaya), and purpose (arthavattva), mastery over the elements is attained."

**Mapping to Pipeline Configuration**:

| Sanskrit                  | Pipeline Component | Description                                       |
| ------------------------- | ------------------ | ------------------------------------------------- |
| **Sthūla** (Gross)        | Storage Backend    | Physical manifestation (HugeArray, Arrow, Sparse) |
| **Svarūpa** (Form)        | PropertyDescriptor | Essential schema (type, size, hints)              |
| **Sūkṣma** (Subtle)       | Runtime Values     | Computational representation (PrimitiveValues)    |
| **Anvaya** (Connection)   | Backend Trait      | Bridge between storage and compute                |
| **Arthavattva** (Purpose) | Pipeline Config    | Goal-oriented selection (algorithm needs)         |
| **Saṃyama** (Focus)       | Eval Macro         | Single-pointed projection from DSL                |
| **Bhūta-jayaḥ** (Mastery) | Performance        | 10-100x gains via optimal backend choice          |

**Key Insight**: The eval macro system **IS** the saṃyama practice — focused projection from a single source (DSL) to create mastery over multiple backends (elements) while preserving the essential form (schema).

Just as saṃyama requires understanding the gross, form, subtle, connection, and purpose to achieve mastery, our pipeline configuration requires understanding storage, schema, runtime, interfaces, and algorithm needs to achieve performance mastery.

---

## 📋 Action Items

### Immediate (This Session - Documentation)

- [x] Document pipeline configuration strategy (this file)
- [ ] Update EVAL_MACRO_STRATEGIC_ROLE.md with backend section
- [ ] Create backend_trait_specification.md
- [ ] Update copilot-instructions.md with backend patterns

### Phase 1 (Week 1 - Backend Traits)

- [ ] Define Backend<T> trait
- [ ] Implement Backend for HugeArray
- [ ] Implement Backend for Arrow2
- [ ] Implement Backend for Sparse
- [ ] Update eval macro to generate backend-agnostic code

### Phase 2 (Week 2 - GraphStore Integration)

- [ ] Add GraphStoreBackendConfig
- [ ] Wire backend selection into GraphStore
- [ ] Update property factories
- [ ] Add density analysis helpers
- [ ] Create examples and tests

### Phase 3 (Week 3 - Thread Model)

- [ ] Define ThreadModel enum
- [ ] Create Executor trait
- [ ] Wire into Pregel executor
- [ ] Update algorithm configs
- [ ] Benchmarks and examples

### Phase 4 (Week 4 - Polish)

- [ ] Documentation updates
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Migration guide

---

## 🎯 Success Criteria

**We'll know this is successful when**:

1. ✅ **Type Safety**: Backend choice is compile-time checked
2. ✅ **Zero Duplication**: PropertyValues logic written once, works with all backends
3. ✅ **Runtime Flexibility**: Can switch backends per GraphStore instance
4. ✅ **Algorithm Hints**: Configs can recommend backends per algorithm
5. ✅ **Performance**: 10-100x gains on edge cases (sparse graphs, zero-copy, etc.)
6. ✅ **Maintainability**: Adding new backend requires 1 trait impl, not 20 structs
7. ✅ **Testability**: Can mock backends for unit tests
8. ✅ **Observability**: Can log/profile backend usage

---

## 🔗 Related Documents

- `EVAL_MACRO_STRATEGIC_ROLE.md` - Overall eval macro architecture
- `EVAL_MACRO_MIGRATION_PLAN.md` - u64/usize migration plan
- `unified_macro_architecture.md` - Original backend abstraction design
- `config_system_implementation.md` - Configuration system patterns
- `property_values_huge_arrays_issue.md` - Why HugeArray is mandatory
- `stack_architecture.md` - Full system architecture

---

**Status**: Strategic design complete. Ready to begin Phase 1 (backend trait system) after completing u64/usize migration.
