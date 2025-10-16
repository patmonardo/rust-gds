# ML Pipeline Architecture: FormDB Knowledge Apps

## Strategic Context

**FormDB is an ML Knowledge Apps platform**

- ML is the product (training, prediction, knowledge extraction)
- Pipelines orchestrate ML workflows
- Graph algorithms are feature sources (external)
- FormProcessor oversees the entire ML lifecycle

---

## Java GDS Pipeline Analysis

### Key Components from Java GDS

1. **Pipeline Interface** - Base abstraction

   - `nodePropertySteps()` - Graph algorithm steps
   - `featureSteps()` - Feature engineering steps
   - `validateBeforeExecution()` - Pre-flight checks

2. **TrainingPipeline** - ML training orchestration

   - `trainingParameterSpace` - Model configurations to try
   - `autoTuningConfig` - Hyperparameter tuning config
   - Supports multiple model types

3. **PipelineExecutor** - Execution engine

   - `DatasetSplits` - TRAIN, TEST, TEST_COMPLEMENT, FEATURE_INPUT
   - Executes node property steps
   - Prepares data for training

4. **PipelineTrainer** - Training orchestration

   - `run()` - Execute training
   - `setTerminationFlag()` - Cancellation support

5. **ExecutableNodePropertyStep** - Graph algorithm wrapper

   - `execute()` - Run graph algorithm (via Stub)
   - `config()` - Algorithm configuration
   - Memory estimation support

6. **Stub** - Graph algorithm interface

   - `execute()` - Call graph procedure
   - `getMemoryEstimation()` - Estimate memory
   - Decouples ML from graph implementation

7. **AutoTuningConfig** - Hyperparameter search
   - `maxTrials` - Number of configurations to try
   - Grid search / random search support

---

## Rust Pipeline Architecture: ML-First Design

### Core Principle

**Pipeline is the ML workflow orchestrator**

- Takes data from FormDB
- Optionally calls graph algorithms (stubbed)
- Engineers features
- Trains ML models
- Produces predictions

### Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                      FormDB Data                            │
│                 (Nodes, Relationships, Properties)          │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                 Pipeline Descriptor                         │
│              (Declarative ML Workflow)                      │
│                                                             │
│  - Node property steps (optional graph algorithms)         │
│  - Feature steps (engineering)                             │
│  - Training config (model, optimizer, validation)          │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              PipelineExecutor (Orchestration)               │
│                                                             │
│  1. Execute node property steps (graph algorithms)         │
│  2. Assemble features (from properties + computed)         │
│  3. Split dataset (train/validation/test)                  │
│  4. Execute training (PipelineTrainer)                     │
│  5. Evaluate model (metrics)                               │
│  6. Return trained model                                    │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ├─→ GraphProcedureStub (external)
                           │       │
                           │       └─→ PageRank, FastRP, etc.
                           │
                           ├─→ FeatureAssembler (ML core)
                           │       │
                           │       └─→ PropertyValues → Features
                           │
                           └─→ PipelineTrainer (ML core)
                                   │
                                   ├─→ Model training
                                   ├─→ Hyperparameter tuning
                                   └─→ Model evaluation
```

---

## Phase 2.3 Implementation: Pipeline-Centric

### Step 1: Pipeline Descriptor (Declarative Workflow)

**File**: `src/projection/codegen/ml/pipeline_descriptor.rs`

```rust
//! ML Pipeline descriptors - declarative workflow specifications.
//!
//! Pipelines describe complete ML workflows from data to trained models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete ML pipeline specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDescriptor {
    /// Pipeline name (for catalog)
    pub name: String,

    /// Pipeline type (classification, regression, link prediction)
    pub pipeline_type: PipelineType,

    /// Optional graph algorithm steps (feature computation)
    pub node_property_steps: Vec<NodePropertyStepDescriptor>,

    /// Feature engineering steps
    pub feature_steps: Vec<FeatureStepDescriptor>,

    /// Training configuration
    pub training_config: TrainingConfig,

    /// Auto-tuning configuration (hyperparameter search)
    pub auto_tuning_config: Option<AutoTuningConfig>,

    /// Pipeline metadata
    pub metadata: PipelineMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineType {
    /// Node classification
    NodeClassification { target_property: String },

    /// Node regression
    NodeRegression { target_property: String },

    /// Link prediction
    LinkPrediction {
        source_node_label: String,
        target_node_label: String,
    },
}

/// Node property step - executes graph algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePropertyStepDescriptor {
    /// Step name (for tracking)
    pub name: String,

    /// Graph procedure to execute (e.g., "pageRank", "fastRP")
    pub procedure_name: String,

    /// Output property name (where results are stored)
    pub mutate_property: String,

    /// Procedure-specific configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Optional node label filter
    pub context_node_labels: Option<Vec<String>>,

    /// Optional relationship type filter
    pub context_relationship_types: Option<Vec<String>>,
}

/// Feature engineering step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStepDescriptor {
    /// Step name
    pub name: String,

    /// Input properties (from graph or previous steps)
    pub input_properties: Vec<String>,

    /// Feature transformation to apply
    pub transformation: FeatureTransformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureTransformation {
    /// Use property as-is
    Identity,

    /// Normalize (z-score, min-max, etc.)
    Normalize { method: NormalizationMethod },

    /// One-hot encode categorical
    OneHotEncode { categories: Vec<String> },

    /// Combine multiple properties
    Combine { method: CombineMethod },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NormalizationMethod {
    ZScore,
    MinMax,
    Robust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombineMethod {
    Concat,
    Mean,
    Sum,
}

/// Training configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Model configurations to try (hyperparameter space)
    pub model_candidates: Vec<ModelCandidate>,

    /// Dataset split configuration
    pub split_config: SplitConfig,

    /// Validation metric (for model selection)
    pub validation_metric: ValidationMetric,
}

/// Model candidate configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    /// Model type
    pub model_type: ModelType,

    /// Model-specific hyperparameters
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    DecisionTreeClassifier,
    DecisionTreeRegressor,
    LogisticRegression,
    LinearRegression,
    RandomForest,
}

/// Dataset split configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Train fraction (e.g., 0.7)
    pub train_fraction: f64,

    /// Validation fraction (e.g., 0.15)
    pub validation_fraction: f64,

    /// Test fraction (e.g., 0.15)
    pub test_fraction: f64,

    /// Random seed (reproducibility)
    pub seed: u64,

    /// Stratify by target (classification)
    pub stratify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationMetric {
    // Classification
    Accuracy,
    F1,
    AUCROC,

    // Regression
    RMSE,
    MAE,
    R2,
}

/// Auto-tuning configuration (hyperparameter search).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTuningConfig {
    /// Maximum trials (model configurations to try)
    pub max_trials: usize,

    /// Search strategy
    pub search_strategy: SearchStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchStrategy {
    /// Try all combinations
    GridSearch,

    /// Random sampling
    RandomSearch { iterations: usize },
}

/// Pipeline metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    /// Creation timestamp
    pub created_at: String,

    /// Creator
    pub created_by: String,

    /// Description
    pub description: Option<String>,
}

impl Default for SplitConfig {
    fn default() -> Self {
        Self {
            train_fraction: 0.7,
            validation_fraction: 0.15,
            test_fraction: 0.15,
            seed: 42,
            stratify: true,
        }
    }
}

impl Default for AutoTuningConfig {
    fn default() -> Self {
        Self {
            max_trials: 10,
            search_strategy: SearchStrategy::GridSearch,
        }
    }
}
```

---

### Step 2: Pipeline State (Runtime Data)

**File**: `src/projection/native/ml/pipeline_state.rs`

```rust
//! Pipeline execution state - runtime data container.

use crate::types::properties::PropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Runtime state during pipeline execution.
#[derive(Debug, Clone)]
pub struct PipelineState {
    /// Computed properties (from node property steps + original graph)
    pub properties: HashMap<String, Arc<dyn PropertyValues>>,

    /// Assembled features (from feature steps)
    pub features: HashMap<String, Arc<dyn PropertyValues>>,

    /// Node IDs in dataset
    pub node_ids: Vec<u64>,

    /// Dataset splits
    pub splits: DatasetSplits,
}

/// Dataset splits for training/validation/test.
#[derive(Debug, Clone)]
pub struct DatasetSplits {
    /// Training node IDs
    pub train: Vec<u64>,

    /// Validation node IDs
    pub validation: Vec<u64>,

    /// Test node IDs
    pub test: Vec<u64>,
}

impl PipelineState {
    pub fn new(node_ids: Vec<u64>) -> Self {
        Self {
            properties: HashMap::new(),
            features: HashMap::new(),
            node_ids,
            splits: DatasetSplits {
                train: Vec::new(),
                validation: Vec::new(),
                test: Vec::new(),
            },
        }
    }

    /// Add computed property from node property step.
    pub fn add_property(&mut self, name: String, values: Arc<dyn PropertyValues>) {
        self.properties.insert(name, values);
    }

    /// Add assembled feature from feature step.
    pub fn add_feature(&mut self, name: String, values: Arc<dyn PropertyValues>) {
        self.features.insert(name, values);
    }

    /// Set dataset splits.
    pub fn set_splits(&mut self, splits: DatasetSplits) {
        self.splits = splits;
    }
}
```

---

### Step 3: Pipeline Executor (Orchestration Engine)

**File**: `src/projection/native/ml/pipeline_executor.rs` (ENHANCED)

```rust
//! Pipeline executor - orchestrates complete ML workflow.

use super::pipeline_state::{PipelineState, DatasetSplits};
use super::graph_procedure::GraphProcedureRegistry;
use super::feature_assembler::FeatureAssembler;
use super::training_executor::TrainingExecutor;
use crate::projection::codegen::ml::pipeline_descriptor::{
    PipelineDescriptor, NodePropertyStepDescriptor, SplitConfig,
};
use crate::types::GraphStore;
use std::sync::Arc;

/// Pipeline executor - runs complete ML workflow.
pub struct PipelineExecutor {
    descriptor: PipelineDescriptor,
    graph_store: Arc<GraphStore>,
    procedure_registry: Arc<GraphProcedureRegistry>,
}

impl PipelineExecutor {
    pub fn new(
        descriptor: PipelineDescriptor,
        graph_store: Arc<GraphStore>,
        procedure_registry: Arc<GraphProcedureRegistry>,
    ) -> Self {
        Self {
            descriptor,
            graph_store,
            procedure_registry,
        }
    }

    /// Execute complete pipeline: node property steps → features → training.
    pub fn execute(&self) -> Result<PipelineResult, PipelineError> {
        // Initialize state
        let node_ids = self.collect_node_ids()?;
        let mut state = PipelineState::new(node_ids);

        // Phase 1: Execute node property steps (graph algorithms)
        self.execute_node_property_steps(&mut state)?;

        // Phase 2: Assemble features
        self.assemble_features(&mut state)?;

        // Phase 3: Split dataset
        self.split_dataset(&mut state)?;

        // Phase 4: Train models (with auto-tuning if configured)
        let training_result = self.train_models(&state)?;

        Ok(PipelineResult {
            best_model: training_result.best_model,
            validation_metrics: training_result.validation_metrics,
            test_metrics: training_result.test_metrics,
            all_trials: training_result.all_trials,
        })
    }

    /// Execute node property steps (graph algorithms).
    fn execute_node_property_steps(
        &self,
        state: &mut PipelineState,
    ) -> Result<(), PipelineError> {
        for step in &self.descriptor.node_property_steps {
            self.execute_node_property_step(step, state)?;
        }
        Ok(())
    }

    /// Execute single node property step.
    fn execute_node_property_step(
        &self,
        step: &NodePropertyStepDescriptor,
        state: &mut PipelineState,
    ) -> Result<(), PipelineError> {
        // Get procedure from registry (stub for now)
        let procedure = self.procedure_registry
            .get(&step.procedure_name)
            .ok_or_else(|| PipelineError::UnknownProcedure(step.procedure_name.clone()))?;

        // Execute procedure
        let result = procedure.execute(
            self.graph_store.clone(),
            &step.config,
        )?;

        // Store result in state
        state.add_property(step.mutate_property.clone(), result);

        Ok(())
    }

    /// Assemble features from properties.
    fn assemble_features(
        &self,
        state: &mut PipelineState,
    ) -> Result<(), PipelineError> {
        let assembler = FeatureAssembler::new(
            self.descriptor.feature_steps.clone(),
        );

        for feature_step in &self.descriptor.feature_steps {
            let feature_values = assembler.assemble_feature(
                feature_step,
                state,
            )?;

            state.add_feature(feature_step.name.clone(), feature_values);
        }

        Ok(())
    }

    /// Split dataset into train/validation/test.
    fn split_dataset(
        &self,
        state: &mut PipelineState,
    ) -> Result<(), PipelineError> {
        let config = &self.descriptor.training_config.split_config;

        let splits = if config.stratify {
            self.stratified_split(&state.node_ids, config)?
        } else {
            self.random_split(&state.node_ids, config)?
        };

        state.set_splits(splits);

        Ok(())
    }

    /// Train models (with auto-tuning if configured).
    fn train_models(
        &self,
        state: &PipelineState,
    ) -> Result<TrainingResult, PipelineError> {
        let training_executor = TrainingExecutor::new(
            self.descriptor.training_config.clone(),
            self.descriptor.auto_tuning_config.clone(),
        );

        training_executor.execute(state)
    }

    // Helper methods

    fn collect_node_ids(&self) -> Result<Vec<u64>, PipelineError> {
        // Collect all node IDs from graph store
        // Filter by node labels if specified
        Ok(self.graph_store.node_ids().collect())
    }

    fn random_split(
        &self,
        node_ids: &[u64],
        config: &SplitConfig,
    ) -> Result<DatasetSplits, PipelineError> {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;

        let mut rng = rand::rngs::StdRng::seed_from_u64(config.seed);
        let mut shuffled = node_ids.to_vec();
        shuffled.shuffle(&mut rng);

        let train_size = (node_ids.len() as f64 * config.train_fraction) as usize;
        let val_size = (node_ids.len() as f64 * config.validation_fraction) as usize;

        Ok(DatasetSplits {
            train: shuffled[..train_size].to_vec(),
            validation: shuffled[train_size..train_size + val_size].to_vec(),
            test: shuffled[train_size + val_size..].to_vec(),
        })
    }

    fn stratified_split(
        &self,
        node_ids: &[u64],
        config: &SplitConfig,
    ) -> Result<DatasetSplits, PipelineError> {
        // TODO: Implement stratified splitting
        // For now, fall back to random split
        self.random_split(node_ids, config)
    }
}

/// Pipeline execution result.
pub struct PipelineResult {
    /// Best model (from auto-tuning)
    pub best_model: Box<dyn crate::projection::eval::ml::model::Model>,

    /// Validation metrics (for best model)
    pub validation_metrics: MetricsResult,

    /// Test metrics (final evaluation)
    pub test_metrics: MetricsResult,

    /// All trials (if auto-tuning enabled)
    pub all_trials: Vec<TrialResult>,
}

pub struct MetricsResult {
    pub accuracy: Option<f64>,
    pub f1: Option<f64>,
    pub auc_roc: Option<f64>,
    pub rmse: Option<f64>,
    pub mae: Option<f64>,
    pub r2: Option<f64>,
}

pub struct TrialResult {
    pub model_type: String,
    pub params: std::collections::HashMap<String, serde_json::Value>,
    pub validation_score: f64,
    pub training_time_ms: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Unknown graph procedure: {0}")]
    UnknownProcedure(String),

    #[error("Feature assembly error: {0}")]
    FeatureAssembly(String),

    #[error("Training error: {0}")]
    Training(String),

    #[error("Graph procedure error: {0}")]
    GraphProcedure(String),
}

// Stub implementations for now
struct TrainingResult {
    best_model: Box<dyn crate::projection::eval::ml::model::Model>,
    validation_metrics: MetricsResult,
    test_metrics: MetricsResult,
    all_trials: Vec<TrialResult>,
}
```

---

### Step 4: Graph Procedure Stub (External Interface)

**File**: `src/projection/native/ml/graph_procedure.rs`

```rust
//! Graph procedure interface - stub for external graph algorithms.
//!
//! Graph algorithms are external to ML core. This provides minimal interface
//! for ML pipelines to call graph procedures and get results.

use crate::types::properties::PropertyValues;
use crate::types::GraphStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Graph procedure interface - external graph algorithm.
pub trait GraphProcedure: Send + Sync {
    /// Execute procedure on graph store.
    fn execute(
        &self,
        graph_store: Arc<GraphStore>,
        config: &HashMap<String, serde_json::Value>,
    ) -> Result<Arc<dyn PropertyValues>, GraphProcedureError>;

    /// Procedure name (e.g., "pageRank", "fastRP").
    fn name(&self) -> &str;

    /// Estimate memory usage.
    fn estimate_memory(
        &self,
        graph_store: &GraphStore,
        config: &HashMap<String, serde_json::Value>,
    ) -> Result<usize, GraphProcedureError>;
}

/// Registry of graph procedures.
pub struct GraphProcedureRegistry {
    procedures: HashMap<String, Arc<dyn GraphProcedure>>,
}

impl GraphProcedureRegistry {
    pub fn new() -> Self {
        Self {
            procedures: HashMap::new(),
        }
    }

    /// Register procedure.
    pub fn register(&mut self, procedure: Arc<dyn GraphProcedure>) {
        self.procedures.insert(procedure.name().to_string(), procedure);
    }

    /// Get procedure by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn GraphProcedure>> {
        self.procedures.get(name).cloned()
    }

    /// Create registry with mock procedures (for testing).
    pub fn with_mocks() -> Self {
        let mut registry = Self::new();

        // Register mock procedures
        registry.register(Arc::new(MockPageRankProcedure));
        registry.register(Arc::new(MockFastRPProcedure));

        registry
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphProcedureError {
    #[error("Procedure execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Memory estimation not implemented")]
    MemoryEstimationNotImplemented,
}

// Mock procedures for testing

struct MockPageRankProcedure;

impl GraphProcedure for MockPageRankProcedure {
    fn execute(
        &self,
        graph_store: Arc<GraphStore>,
        _config: &HashMap<String, serde_json::Value>,
    ) -> Result<Arc<dyn PropertyValues>, GraphProcedureError> {
        // Return mock PageRank scores
        use crate::projection::eval::ml::mock_property_values::MockDoublePropertyValues;

        let node_count = graph_store.node_count();
        let values: Vec<f64> = (0..node_count)
            .map(|i| 1.0 / node_count as f64 + (i as f64 * 0.01))
            .collect();

        Ok(Arc::new(MockDoublePropertyValues::new(values)))
    }

    fn name(&self) -> &str {
        "pageRank"
    }

    fn estimate_memory(
        &self,
        graph_store: &GraphStore,
        _config: &HashMap<String, serde_json::Value>,
    ) -> Result<usize, GraphProcedureError> {
        Ok(graph_store.node_count() * 8) // 8 bytes per double
    }
}

struct MockFastRPProcedure;

impl GraphProcedure for MockFastRPProcedure {
    fn execute(
        &self,
        graph_store: Arc<GraphStore>,
        config: &HashMap<String, serde_json::Value>,
    ) -> Result<Arc<dyn PropertyValues>, GraphProcedureError> {
        // Return mock embeddings
        use crate::projection::eval::ml::mock_property_values::MockEmbeddingPropertyValues;

        let embedding_dim = config
            .get("embeddingDimension")
            .and_then(|v| v.as_u64())
            .unwrap_or(128) as usize;

        let node_count = graph_store.node_count();

        Ok(Arc::new(MockEmbeddingPropertyValues::new(
            node_count,
            embedding_dim,
        )))
    }

    fn name(&self) -> &str {
        "fastRP"
    }

    fn estimate_memory(
        &self,
        graph_store: &GraphStore,
        config: &HashMap<String, serde_json::Value>,
    ) -> Result<usize, GraphProcedureError> {
        let embedding_dim = config
            .get("embeddingDimension")
            .and_then(|v| v.as_u64())
            .unwrap_or(128) as usize;

        Ok(graph_store.node_count() * embedding_dim * 8)
    }
}
```

---

## Key Design Decisions

### 1. Pipeline is the ML Orchestrator

**Decision**: Pipeline owns the complete ML workflow, not just configuration.

**Why**:

- FormDB is ML platform - pipeline is the primary abstraction
- Users think in terms of pipelines, not individual steps
- Reproducibility requires capturing entire workflow

### 2. Graph Procedures are External

**Decision**: Graph algorithms are external services called via stub interface.

**Why**:

- ML doesn't depend on graph algorithm implementations
- Separate execution environments (ML vs graph procedures)
- Can swap graph backends without changing ML code

### 3. Descriptor-Driven Everything

**Decision**: All pipeline operations defined by descriptors.

**Why**:

- Codegen target - meta macro processor generates from descriptors
- Serializable - pipelines can be saved/loaded/shared
- FormProcessor visibility - can inspect pipeline without execution

### 4. State Object Pattern

**Decision**: PipelineState holds all runtime data.

**Why**:

- Clear data flow - each phase updates state
- Testable - can mock state for unit tests
- Resumable - can checkpoint and resume from state

---

## Phase 2.3 Priorities (Pipeline-Focused)

### What to Build First

1. **Pipeline Descriptor** (Day 1) ✅
   - Complete specification of ML workflow
   - Serializable, shareable, reproducible
2. **Pipeline State** (Day 1) ✅
   - Runtime data container
   - Properties, features, splits
3. **Graph Procedure Stub** (Day 2) ✅
   - Minimal interface for graph algorithms
   - Mock implementations for testing
4. **Pipeline Executor** (Days 3-4)
   - Orchestrate complete workflow
   - Node property steps → Features → Training
5. **Feature Assembler** (Days 5-6)
   - Transform properties → ML features
   - Handle transformations (normalize, encode, etc.)
6. **Training Executor** (Days 7-8)
   - Train models with auto-tuning
   - Model selection, evaluation
7. **Integration Tests** (Days 9-10)
   - End-to-end pipeline tests
   - Validate complete workflow

---

## Success Criteria

### Pipeline Architecture Complete When:

1. ✅ **Can define ML workflow declaratively**

   - PipelineDescriptor captures complete workflow
   - Serializable to/from JSON

2. ✅ **Can execute node property steps**

   - Call graph procedures (mocked)
   - Store results in state

3. ✅ **Can assemble features**

   - Transform properties to ML features
   - Handle normalization, encoding

4. ✅ **Can split dataset**

   - Random and stratified splitting
   - Train/validation/test splits

5. ✅ **Can train models**

   - Execute training on splits
   - Auto-tune hyperparameters

6. ✅ **Can evaluate models**

   - Compute metrics on validation/test
   - Select best model

7. ✅ **End-to-end test passes**
   - Full pipeline: data → trained model
   - All phases working together

---

## Next Steps

**Ready to implement Pipeline-focused Phase 2.3:**

1. Start with pipeline descriptors (declarative specs)
2. Build pipeline executor (orchestration)
3. Implement graph procedure stubs (external interface)
4. Add feature assembly (property → feature transformation)
5. Integrate training (model training + tuning)
6. Write end-to-end tests (complete workflow)

**This architecture makes FormDB an ML Knowledge Apps platform** with pipelines as the primary abstraction!
