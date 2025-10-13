# ML Architecture Shift: FormDB-First Design

## The Fundamental Shift

### Before

**Thinking**: Graph platform (like Neo4j GDS) that adds ML capabilities

```
Graph Algorithms (Primary)
    ├── PageRank, Louvain, FastRP, etc.
    └── ML (Secondary, uses graph algorithm outputs)
        ├── Pipeline orchestration
        └── Simple models
```

**Problems**:

- Graph algorithms unclear scope for FormDB
- ML treated as afterthought
- Unclear what FormProcessor actually oversees
- Codegen target ambiguous

### After

**Thinking**: ML platform (FormDB) that uses graph algorithms as feature sources

```
ML Core (Primary - FormProcessor oversees this)
    ├── Model trait system
    ├── Training infrastructure
    ├── Feature engineering
    ├── Evaluation metrics
    └── Graph Procedures (Secondary, external data sources)
        └── Stub interface for PageRank, FastRP, etc.
```

**Benefits**:

- Clear scope: ML is the product
- FormProcessor role clear: oversees ML computation species
- Codegen target clear: generate ML training code
- Graph algorithms are separate concern (procedure execution environment)

---

## What This Means for Phase 2.3

### Old Phase 2.3 Plan (~800 lines)

- Model trait (prototype)
- Feature assembly (basic)
- DecisionTreeStub (learns mean/mode - not real)
- Training executor (basic)
- 10+ tests

**Problem**: Prototype that doesn't validate real ML architecture

### New Phase 2.3 Plan (~8000 lines)

- **Complete model trait system** (Classifier, Regressor sub-traits, serialization)
- **Real ML models** (DecisionTree with splitting, LogisticRegression, LinearRegression)
- **Training infrastructure** (optimizers, loss functions, regularization, early stopping)
- **Feature engineering pipeline** (assembly, transformation, normalization, encoding)
- **Comprehensive evaluation** (classification/regression metrics, cross-validation)
- **FormProcessor integration** (progress tracking, resource management, computation species)
- **Codegen hooks** (descriptor-driven, generation-friendly)
- **50+ tests** (production coverage)

**Result**: Production-grade ML platform for FormDB

---

## Key Design Principles

### 1. ML-First

**Principle**: ML is the core product, graph algorithms are data sources.

**Implementation**:

```rust
// ML is first-class, complete implementation
pub mod ml {
    pub mod models { /* DecisionTree, LogReg, LinReg, ... */ }
    pub mod training { /* Optimizers, loss, regularization */ }
    pub mod features { /* Assembly, transformation, selection */ }
    pub mod metrics { /* Comprehensive evaluation */ }
}

// Graph procedures are minimal stub interface
pub mod graph_procedures {
    pub trait GraphProcedure {
        fn execute(&self, ...) -> PropertyValues;
    }
    // Real implementation is separate concern
}
```

### 2. FormProcessor-Integrated

**Principle**: FormProcessor oversees ML computation, needs visibility and control.

**Implementation**:

```rust
// ML computation species
impl ComputationDescriptor {
    pub const ML_TRAINING: ComputationSpecies = ComputationSpecies::MLTraining;
    pub const ML_PREDICTION: ComputationSpecies = ComputationSpecies::MLPrediction;
    pub const ML_EVALUATION: ComputationSpecies = ComputationSpecies::MLEvaluation;
}

// Progress tracking
pub trait ProgressReporter {
    fn report_epoch(&self, epoch: usize, loss: f64);
    fn report_validation(&self, metrics: &Metrics);
}

// Resource management
pub struct ResourceLimits {
    max_feature_matrix_bytes: usize,
    max_training_time: Duration,
}
```

### 3. Codegen-Driven

**Principle**: Meta macro processor generates optimized ML code from descriptors.

**Implementation**:

```rust
// All ML operations descriptor-driven
#[derive(ModelBuilder)]  // Codegen hook
pub struct DecisionTreeDescriptor {
    max_depth: usize,
    min_samples_split: usize,
    criterion: SplitCriterion,
}

// Descriptors enable reproducible pipelines
pub struct TrainingDescriptor {
    model: ModelDescriptor,
    optimization: OptimizationConfig,
    validation: ValidationConfig,
}

// Codegen produces specialized code
// Generated: DecisionTreeClassifier::new_optimized_for(descriptor)
```

### 4. Production-Quality

**Principle**: Real implementations, not stubs or proofs-of-concept.

**Implementation**:

- DecisionTree: Real splitting with Gini/Entropy/MSE impurity
- LogisticRegression: Real gradient descent with line search
- LinearRegression: Normal equation + gradient descent variants
- Training converges, metrics improve, models serialize

**Why**: Can't validate architecture with fake algorithms.

---

## FormDB Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                          FormDB                             │
│                      (Data Storage)                         │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ├─→ Direct Features (node properties)
                               │       │
                               │       ▼
                               │   Feature Assembly
                               │       │
                               └───────┴─→ Graph Procedures (optional)
                                           │ PageRank, FastRP, etc.
                                           │ (External, stubbed for now)
                                           ▼
                                       PropertyValues
                                           │
                                           ▼
                               ┌───────────────────────┐
                               │   Feature Matrix      │
                               │   (Combined features) │
                               └───────────┬───────────┘
                                           │
                                           ▼
                               ┌───────────────────────┐
                               │   ML Training         │ ← FormProcessor
                               │   (Gradient Descent)  │   oversees
                               └───────────┬───────────┘
                                           │
                                           ▼
                               ┌───────────────────────┐
                               │   Trained Model       │
                               │   (DecisionTree, etc.)│
                               └───────────┬───────────┘
                                           │
                                           ▼
                               ┌───────────────────────┐
                               │   Predictions         │ → Store in FormDB
                               │   (PropertyValues)    │
                               └───────────────────────┘
```

---

## Codegen Meta Macro Processor Role

### What Codegen Generates

1. **Model Builders**

   ```rust
   // From descriptor:
   DecisionTreeDescriptor { max_depth: 10, criterion: Gini }

   // Generate:
   impl DecisionTreeClassifier {
       pub fn new_with_max_depth_10_gini() -> Self {
           // Specialized, optimized implementation
       }
   }
   ```

2. **Training Loops**

   ```rust
   // From training descriptor:
   TrainingDescriptor {
       optimizer: Adam { lr: 0.001 },
       loss: CrossEntropy,
       epochs: 100,
   }

   // Generate:
   pub fn train_with_adam_cross_entropy(data: &[TrainingExample]) {
       // Optimized training loop
       // Unrolled, vectorized, cache-friendly
   }
   ```

3. **Feature Pipelines**

   ```rust
   // From feature descriptor:
   FeatureDescriptor {
       sources: ["age", "income", "pagerank_score"],
       transforms: [Normalize, OneHot],
   }

   // Generate:
   pub fn extract_features_optimized(state: &PipelineState) -> Vec<Features> {
       // Vectorized feature extraction
       // Cached, minimal allocations
   }
   ```

4. **Evaluation Code**

   ```rust
   // From evaluation descriptor:
   EvaluationDescriptor {
       metrics: [Accuracy, F1, AUCROC],
       cross_validation: KFold { k: 5 },
   }

   // Generate:
   pub fn evaluate_with_5fold_cv(model: &Model) -> EvaluationResults {
       // Optimized evaluation
       // Parallel fold execution
   }
   ```

### Why This Matters

- **Performance**: Generated code is specialized, fast
- **Reproducibility**: Descriptor → same generated code → same results
- **Observability**: FormProcessor sees descriptors, tracks progress
- **Extensibility**: Add new models/optimizers by writing descriptors

---

## What FormProcessor Sees

```rust
pub enum ComputationEvent {
    // ML training events
    TrainingStarted {
        model_type: ModelType,
        training_samples: usize,
        validation_samples: usize,
    },

    EpochCompleted {
        epoch: usize,
        loss: f64,
        metrics: Metrics,
        duration: Duration,
    },

    ValidationCompleted {
        metrics: Metrics,
    },

    TrainingCompleted {
        final_metrics: Metrics,
        total_duration: Duration,
    },

    TrainingFailed {
        error: TrainingError,
        epoch: usize,
    },

    // Resource events
    FeatureMatrixAllocated {
        size_bytes: usize,
    },

    ResourceLimitApproaching {
        current: usize,
        limit: usize,
    },
}
```

FormProcessor can:

- Track training progress (epoch, loss, metrics)
- Enforce resource limits
- Kill runaway training
- Report errors with context
- Coordinate multiple model training (AutoML)

---

## Phase 2.3 Deliverables Summary

### What Gets Built

1. **Complete ML Core** (~8000 lines)

   - Model trait system (base Model, Classifier, Regressor)
   - 3 real ML models (DecisionTree, LogisticRegression, LinearRegression)
   - Training infrastructure (optimizers, loss, regularization)
   - Feature engineering (assembly, transformation, selection)
   - Evaluation metrics (classification, regression, cross-validation)

2. **FormProcessor Integration** (~400 lines)

   - ML computation species
   - Progress tracking
   - Resource management
   - Error propagation

3. **Codegen Hooks** (throughout)

   - Descriptor-driven design
   - Clear generation points
   - Compositional abstractions

4. **Graph Procedure Stub** (~100 lines)

   - Minimal interface
   - Mock implementations
   - Real implementation deferred

5. **Comprehensive Tests** (~2000 lines)
   - 50+ unit tests
   - 10+ integration tests
   - > 90% coverage

### What Gets Deferred

- **Graph algorithm implementation** (separate procedure execution environment)
- **Advanced ML models** (RandomForest, GradientBoosting - extensible)
- **AutoML** (hyperparameter search - foundation in place)
- **Distributed training** (single-node production-ready first)

---

## Decision Points

### ✅ Confirmed

1. ML is primary focus for Phase 2.3
2. Real implementations, not stubs
3. FormProcessor integration in Phase 2.3
4. Codegen-driven architecture
5. Graph algorithms are separate concern

### ❓ For Review

1. **Model scope**: DecisionTree + Linear models sufficient?
2. **Optimizer scope**: SGD + Adam, or add more?
3. **Feature transforms**: Which are must-have vs nice-to-have?
4. **Module location**: `projection/native/ml/` or top-level `src/ml/`?
5. **Timeline**: 10 days reasonable for production ML?

---

## Next Steps

1. **Get approval** on scope and approach
2. **Start with Layer 1** (Core ML Foundations)
3. **Iterate with tests** - Validate each component
4. **Integrate FormProcessor** - Wire progress tracking early
5. **Document as we go** - Codegen guide, model guide

**Goal**: FormDB has production-grade ML platform, ready for real workloads.
