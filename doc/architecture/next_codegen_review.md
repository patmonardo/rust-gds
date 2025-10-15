# Next Codegen Phase: Official Java GDS Sources Review

**Date:** October 11, 2025  
**Scope:** Procedure Execution Architecture + ML Pipeline Architecture  
**Duration:** 10-day codegen  
**Foundation:** Existing Projection Module

---

## 1. Java GDS Source Structure (Official Review)

### 1.1 Core Execution Framework

**Location:** `org.neo4j.gds.executor`

#### Key Classes to Translate:

```java
// Algorithm execution lifecycle
AlgorithmSpec.java           → Rust: AlgorithmDescriptor trait
ExecutionContext.java        → Rust: ExecutionContext struct
ExecutionMode.java           → Rust: ExecutionMode enum
ProcedureExecutor.java       → Rust: ProcedureExecutor struct
AlgorithmExecutor.java       → Rust: AlgorithmExecutor trait

// Configuration and validation
BaseConfig.java              → Rust: ConfigTrait (already have pattern)
ConfigurationParser.java     → Rust: FormProcessor integration
ValidationConfiguration.java → Rust: ConfigValidator trait
```

#### Key Patterns:

1. **AlgorithmSpec Pattern:**

   - Algorithm registration at compile-time
   - Configuration schema definition
   - Result type specification
   - Execution mode declaration

2. **ExecutionContext Pattern:**
   - Graph access
   - Configuration parameters
   - Progress tracking
   - Memory management
   - Transaction handling (skip for Rust - not applicable)

### 1.2 Procedure Framework

**Location:** `org.neo4j.gds.procedures`

#### Key Classes to Translate:

```java
// Procedure registration and lifecycle
ProcedureFacade.java         → Rust: ProcedureFacade struct
GraphDataScience.java        → Rust: GraphDataScience (main entry point)
ProcedureCallContext.java    → Rust: ProcedureContext struct

// Algorithm facades (one per algorithm family)
algorithms/
  centrality/
    CentralityProcedureFacade.java   → Rust procedures module
  community/
    CommunityProcedureFacade.java    → Rust procedures module
  pathfinding/
    PathFindingProcedureFacade.java  → Rust procedures module
  similarity/
    SimilarityProcedureFacade.java   → Rust procedures module
```

#### Key Patterns:

1. **Facade Pattern:**

   - Grouped algorithms by category
   - Stream/Write/Mutate/Stats modes
   - Common result formatting
   - Error handling

2. **Procedure Modes:**
   ```java
   .stream()  → Returns iterator of results
   .write()   → Writes back to graph store
   .mutate()  → Mutates in-memory graph
   .stats()   → Returns only statistics
   ```

### 1.3 ML Pipeline Architecture

**Location:** `org.neo4j.gds.ml.pipeline`

#### Key Classes to Translate:

```java
// Pipeline core
Pipeline.java                → Rust: Pipeline struct
PipelineTrainer.java         → Rust: PipelineTrainer trait
ExecutableNodePropertyStep.java → Rust: PipelineStage trait

// Node classification
NodeClassificationPipeline.java    → Rust: NodeClassificationPipeline
NodeClassificationTrainingPipeline.java
NodeClassificationPredictPipeline.java

// Link prediction
LinkPredictionPipeline.java        → Rust: LinkPredictionPipeline
LinkPredictionTrainingPipeline.java
LinkPredictionPredictPipeline.java
```

#### Key Patterns:

1. **Pipeline Stages:**

   - Feature extraction (node properties, embeddings)
   - Feature normalization
   - Model training configuration
   - Model selection (cross-validation)
   - Prediction execution

2. **Pipeline State:**
   - Training catalog (stores trained models)
   - Model parameters
   - Feature metadata
   - Split configuration (train/test/validation)

### 1.4 Algorithm Catalog

**Location:** `org.neo4j.gds.algorithms` (multiple packages)

#### Algorithms to Implement (Priority Order):

**Week 1: Graph Algorithms (Days 1-3)**

```
✓ PageRank (already have Pregel impl)
✓ Louvain (community detection)
✓ Label Propagation
✓ Weakly Connected Components (WCC)
✓ Triangle Count
✓ Local Clustering Coefficient
✓ Degree Centrality
✓ Betweenness Centrality
```

**Week 2: Path & ML (Days 4-6)**

```
✓ Breadth-First Search (BFS)
✓ Depth-First Search (DFS)
✓ Shortest Path (Dijkstra)
✓ A* Search
✓ All Pairs Shortest Paths
✓ Node Similarity
✓ FastRP (Fast Random Projection)
✓ Node2Vec
```

**Week 2: Pipelines & Polish (Days 7-10)**

```
✓ Node Classification Pipeline
✓ Link Prediction Pipeline
✓ Graph embedding pipelines
✓ Model training infrastructure
✓ Cross-validation
✓ Form Processor unification
✓ Documentation & Examples
```

---

## 2. Rust GDS Architecture Design

### 2.1 Module Structure (New)

```
src/
├── projection/              [EXISTING - Foundation]
│   ├── mod.rs
│   ├── pipeline_descriptor.rs
│   ├── computation_descriptor.rs
│   ├── storage_descriptor.rs
│   └── form_processor.rs
│
├── procedures/              [NEW - Week 1]
│   ├── mod.rs
│   ├── registry.rs          // Procedure registration
│   ├── execution.rs         // Execution context & modes
│   ├── facade.rs            // Procedure facades
│   │
│   ├── centrality/          // Algorithm categories
│   │   ├── mod.rs
│   │   ├── pagerank.rs
│   │   ├── betweenness.rs
│   │   └── degree.rs
│   │
│   ├── community/
│   │   ├── mod.rs
│   │   ├── louvain.rs
│   │   ├── label_propagation.rs
│   │   └── wcc.rs
│   │
│   ├── pathfinding/
│   │   ├── mod.rs
│   │   ├── bfs.rs
│   │   ├── dfs.rs
│   │   ├── dijkstra.rs
│   │   └── astar.rs
│   │
│   └── similarity/
│       ├── mod.rs
│       └── node_similarity.rs
│
├── pipeline/                [NEW - Week 2]
│   ├── mod.rs
│   ├── stage.rs             // PipelineStage trait
│   ├── executor.rs          // Pipeline execution
│   ├── training.rs          // Training infrastructure
│   │
│   ├── node_classification/
│   │   ├── mod.rs
│   │   ├── pipeline.rs
│   │   ├── training.rs
│   │   └── prediction.rs
│   │
│   ├── link_prediction/
│   │   ├── mod.rs
│   │   ├── pipeline.rs
│   │   ├── training.rs
│   │   └── prediction.rs
│   │
│   └── embeddings/
│       ├── mod.rs
│       ├── fastrp.rs
│       └── node2vec.rs
│
├── pregel/                  [EXISTING - Reuse]
│   └── ... (already complete)
│
├── types/                   [EXISTING - Reuse]
│   └── ... (graph, properties, etc.)
│
└── core/                    [EXISTING - Reuse]
    └── ... (concurrency, progress, etc.)
```

### 2.2 Core Trait Definitions

#### 2.2.1 Procedure System

```rust
// src/procedures/mod.rs

/// Procedure descriptor - registers algorithms as callable procedures
pub trait ProcedureDescriptor: Send + Sync {
    /// Procedure name (e.g., "gds.pageRank")
    fn name(&self) -> &str;

    /// Algorithm category
    fn category(&self) -> ProcedureCategory;

    /// Configuration schema
    fn config_schema(&self) -> &dyn ConfigSchema;

    /// Execute in stream mode (returns iterator)
    fn stream(&self, context: &ProcedureContext)
        -> Result<Box<dyn Iterator<Item = ProcedureResult>>>;

    /// Execute in write mode (writes to graph store)
    fn write(&self, context: &ProcedureContext)
        -> Result<WriteResult>;

    /// Execute in mutate mode (mutates in-memory graph)
    fn mutate(&self, context: &ProcedureContext)
        -> Result<MutateResult>;

    /// Execute in stats mode (returns only statistics)
    fn stats(&self, context: &ProcedureContext)
        -> Result<StatsResult>;
}

/// Execution mode for procedures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Stream,  // Return results as iterator
    Write,   // Write results back to store
    Mutate,  // Mutate in-memory graph
    Stats,   // Return statistics only
}

/// Procedure category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcedureCategory {
    Centrality,
    Community,
    PathFinding,
    Similarity,
    Embedding,
    Utility,
}

/// Execution context passed to procedures
pub struct ProcedureContext {
    graph: Arc<dyn Graph>,
    config: Box<dyn Config>,
    progress: Option<Arc<LeafTask>>,
    mode: ExecutionMode,
}

/// Result from procedure execution
pub struct ProcedureResult {
    node_id: usize,
    value: PrimitiveValue,
    metadata: HashMap<String, PrimitiveValue>,
}
```

#### 2.2.2 Pipeline System

```rust
// src/pipeline/mod.rs

/// Pipeline stage - transforms data in ML pipelines
pub trait PipelineStage: Send + Sync {
    /// Stage name for debugging
    fn name(&self) -> &str;

    /// Execute this stage
    fn execute(&self, context: &PipelineContext)
        -> Result<PipelineStageResult>;

    /// Estimate memory usage
    fn estimate_memory(&self, context: &PipelineContext)
        -> MemoryRange;
}

/// Pipeline - chain of stages
pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStage>>,
    config: PipelineConfig,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }

    pub fn train(&self, context: &PipelineContext)
        -> Result<TrainedPipeline> {
        // Execute training stages
    }

    pub fn predict(&self, context: &PipelineContext)
        -> Result<PredictionResult> {
        // Execute prediction with trained model
    }
}

/// Context for pipeline execution
pub struct PipelineContext {
    graph: Arc<dyn Graph>,
    config: PipelineConfig,
    state: PipelineState,
    progress: Option<Arc<LeafTask>>,
}

/// Mutable state during pipeline execution
pub struct PipelineState {
    features: HashMap<String, Arc<dyn NodePropertyValues>>,
    models: HashMap<String, TrainedModel>,
    splits: Option<DataSplit>,
}
```

#### 2.2.3 Form Processor Unification

```rust
// src/projection/form_processor.rs

/// Universal form descriptor - unifies Procedures, Pipelines, Computations
pub trait FormDescriptor: Send + Sync {
    /// Input schema (what this form consumes)
    fn input_schema(&self) -> FormSchema;

    /// Output schema (what this form produces)
    fn output_schema(&self) -> FormSchema;

    /// Process the form transformation
    fn process(&self, context: &FormContext) -> Result<FormOutput>;

    /// Estimate resource requirements
    fn estimate_resources(&self, context: &FormContext)
        -> ResourceEstimate;
}

// Blanket implementations
impl<T: ProcedureDescriptor> FormDescriptor for T { ... }
impl<T: PipelineStage> FormDescriptor for T { ... }
impl<T: ComputationDescriptor> FormDescriptor for T { ... }
```

### 2.3 Registration Pattern (Using `inventory`)

```rust
// src/procedures/registry.rs

/// Procedure registry entry
pub struct ProcedureRegistration {
    pub name: &'static str,
    pub category: ProcedureCategory,
    pub factory: fn() -> Box<dyn ProcedureDescriptor>,
}

// Global registry using inventory crate
inventory::collect!(ProcedureRegistration);

// Register a procedure (used in each algorithm module)
#[macro_export]
macro_rules! register_procedure {
    ($name:expr, $category:expr, $type:ty) => {
        inventory::submit! {
            ProcedureRegistration {
                name: $name,
                category: $category,
                factory: || Box::new(<$type>::new()),
            }
        }
    };
}

// Usage in algorithm modules:
// register_procedure!("gds.pageRank", ProcedureCategory::Centrality, PageRankProcedure);
```

---

## 3. Implementation Strategy

### 3.1 Day-by-Day Plan

#### **Days 1-2: Procedure Foundation**

- [ ] Create `src/procedures/` module structure
- [ ] Implement `ProcedureDescriptor` trait
- [ ] Implement `ProcedureContext` and execution modes
- [ ] Implement procedure registry using `inventory`
- [ ] Create `ProcedureFacade` for centrality algorithms

#### **Day 3: First Procedures**

- [ ] Translate PageRank as procedure (reuse Pregel impl)
- [ ] Translate Degree Centrality
- [ ] Translate Betweenness Centrality
- [ ] Create comprehensive examples
- [ ] Write tests for procedure execution

#### **Days 4-5: Community & Path Algorithms**

- [ ] Translate Louvain
- [ ] Translate Label Propagation
- [ ] Translate WCC
- [ ] Translate BFS/DFS
- [ ] Translate Dijkstra shortest path
- [ ] Create pathfinding facade

#### **Days 6-7: Pipeline Foundation**

- [ ] Create `src/pipeline/` module structure
- [ ] Implement `PipelineStage` trait
- [ ] Implement `Pipeline` struct with builder
- [ ] Implement training infrastructure
- [ ] Implement feature extraction stages

#### **Days 8-9: ML Pipelines**

- [ ] Translate Node Classification Pipeline
- [ ] Translate Link Prediction Pipeline
- [ ] Implement FastRP embeddings
- [ ] Implement Node2Vec (if time allows)
- [ ] Create model training/prediction flow

#### **Day 10: Unification & Polish**

- [ ] Implement `FormDescriptor` blanket impls
- [ ] Create comprehensive examples (one per algorithm)
- [ ] Write integration tests
- [ ] Update documentation
- [ ] Write ADRs for major decisions

### 3.2 Reuse Strategy

**Already Implemented (Reuse):**

- ✅ Pregel framework → Use for PageRank, Label Propagation, WCC
- ✅ Property system → Use for storing algorithm results
- ✅ Progress tracking → Use LeafTask for all algorithms
- ✅ Concurrency primitives → Use for parallel execution
- ✅ Graph abstraction → Use existing Graph trait

**New Implementation Required:**

- ❌ Procedure registry and execution modes
- ❌ Pipeline stage composition
- ❌ Model training infrastructure
- ❌ Feature extraction stages
- ❌ Algorithm facades (one per category)

---

## 4. Java GDS Files to Review (Priority Order)

### 4.1 Must Review First (Foundation)

**Executor Framework:**

```
neo4j-graph-data-science/
  core/src/main/java/org/neo4j/gds/
    executor/
      AlgorithmSpec.java                 ← Core abstraction
      ExecutionContext.java              ← Context pattern
      ProcedureExecutor.java             ← Execution lifecycle

    procedures/
      GraphDataScience.java              ← Main entry point
      ProcedureFacade.java               ← Facade pattern
```

### 4.2 Algorithm Examples (Study Pattern)

**Centrality:**

```
algorithms/centrality/
  BetweennessCentrality.java
  DegreeCentrality.java
```

**Community:**

```
algorithms/community/
  Louvain.java
  LabelPropagation.java
  WCC.java
```

### 4.3 ML Pipeline (Advanced)

**Pipeline Core:**

```
ml/ml-core/src/main/java/org/neo4j/gds/ml/
  pipeline/
    Pipeline.java
    PipelineTrainer.java
    ExecutableNodePropertyStep.java

  nodePropertyPrediction/
    NodeClassificationPipeline.java
    NodeClassificationTrainingPipeline.java
```

---

## 5. Validation Criteria

### 5.1 Completion Checklist

Each algorithm must have:

- [ ] Procedure descriptor implementation
- [ ] All four execution modes (stream/write/mutate/stats)
- [ ] Configuration struct with validation
- [ ] Comprehensive unit tests
- [ ] Integration test with real graph
- [ ] Example in `examples/` directory
- [ ] Documentation with usage

### 5.2 API Quality Standards

Must follow:

- [ ] Import discipline (top-level modules only)
- [ ] Property trait pattern (explicit Arc casts)
- [ ] DefaultValue API (lowercase constructors)
- [ ] Builder pattern for complex configs
- [ ] Error handling with Result types
- [ ] Progress tracking integration

### 5.3 Performance Baselines

Measure:

- [ ] Single-threaded vs parallel speedup
- [ ] Memory usage vs graph size
- [ ] Compilation time (keep incremental builds fast)
- [ ] Test suite runtime (should stay under 10s)

---

## 6. Documentation Requirements

### 6.1 ADRs to Write

Required decision records:

- `adr000X_procedure_execution_architecture.md`
- `adr000X_execution_modes_design.md`
- `adr000X_ml_pipeline_architecture.md`
- `adr000X_form_processor_unification.md`
- `adr000X_algorithm_registration_pattern.md`

### 6.2 Examples to Create

One example per category:

- `examples/procedures_pagerank.rs` - Simple procedure execution
- `examples/procedures_all_modes.rs` - Stream/Write/Mutate/Stats
- `examples/pipeline_node_classification.rs` - Full ML pipeline
- `examples/pipeline_link_prediction.rs` - Link prediction
- `examples/form_processor_unified.rs` - Form abstraction demo

---

## 7. Risk Assessment

### 7.1 Known Challenges

**Technical:**

- Pipeline state management (mutable state during training)
- Model serialization (if we want to save/load models)
- Cross-validation implementation (complex splitting logic)
- Memory estimation for large pipelines

**Design:**

- Balancing abstraction vs performance
- Keeping compilation times reasonable
- Avoiding trait object overhead where possible
- Maintaining ergonomic APIs

### 7.2 Mitigation Strategies

- Start simple, add complexity only when needed
- Use feature flags to isolate heavy dependencies
- Profile early and often
- Keep examples minimal to catch API friction

---

## 8. Success Metrics

### 8.1 Quantitative

- [ ] 20+ algorithms implemented
- [ ] 4 execution modes per algorithm
- [ ] 2+ complete ML pipelines
- [ ] 100% test coverage on new code
- [ ] Documentation for every public API

### 8.2 Qualitative

- [ ] API feels "Rust-native" not "Java translation"
- [ ] Examples are clear and educational
- [ ] Code is maintainable (future devs can extend easily)
- [ ] Performance is competitive with Java GDS
- [ ] Community would be excited to use this

---

## 9. Post-Codegen Review Plan

After 10 days:

1. **Retrospective** - What worked, what didn't
2. **Performance Analysis** - Benchmark vs Java GDS
3. **API Refinement** - Collect friction points
4. **Documentation Pass** - Ensure everything is explained
5. **Community Preparation** - Blog post? Release notes?

---

## 10. Ready State Checklist

Before launching codegen on Monday/Tuesday:

- [ ] This document reviewed and approved
- [ ] Java GDS sources cloned and studied
- [ ] Key patterns identified and understood
- [ ] Module structure agreed upon
- [ ] Day-by-day plan validated
- [ ] Success criteria defined
- [ ] Risk mitigation strategies in place

---

**Status:** DRAFT - Ready for review  
**Next Step:** Weekend review, Monday/Tuesday launch  
**Expected Outcome:** Complete Procedure + Pipeline architecture in 10 days
