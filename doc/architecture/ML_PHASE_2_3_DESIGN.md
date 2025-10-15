# ML Phase 2.3 Design: ML-First Architecture for FormDB

## The Strategic Insight 🎯

**FormDB is an ML platform with graph algorithm support, NOT a graph platform with ML support.**

This fundamentally changes our architectural priorities:

### Architecture Separation

### 1. **ML Core** (✅ PRIMARY FOCUS - Phase 2.3+)

- **Location**: `src/projection/native/ml/` (and future `src/ml/`)
- **Purpose**: Complete ML platform - models, training, evaluation, pipelines
- **Why Primary**: FormProcessor oversees ML computation species, not graph algorithms
- **Components**:
  - Model trait system (DecisionTree, LogisticRegression, RandomForest, MLP)
  - Training infrastructure (gradient descent, optimization, regularization)
  - Feature engineering (assembly, transformation, encoding, selection)
  - Evaluation metrics (accuracy, F1, RMSE, AUC, confusion matrices)
  - Pipeline orchestration (train → validate → predict)
- **Codegen Role**: Meta macro processor generates ML training code
- **FormDB Integration**: FormDB → Features → ML Training → Models → Predictions
- **Status**: ✅ **Phase 2.3 focus** - Build complete, production-ready ML architecture

### 2. **Graph Algorithms** (Stub Layer - External Concern)

- **Location**: Future external procedure execution environment
- **Purpose**: Graph analysis (PageRank, Louvain, FastRP) as feature sources
- **Architecture**: Separate execution environment, ML consumes outputs
- **Interface**: ML pipeline steps call graph procedures → get PropertyValues back
- **ML View**: "Black box" that produces PropertyValues with embeddings/features
- **Status**: ⏸️ **Stub interface only** - Real implementation is orthogonal concern

### Why ML-First Architecture

1. **FormProcessor Scope**: Oversees ML computation species, not graph traversals
2. **Codegen Target**: Meta macro processor generates ML code (training loops, model builders)
3. **Data Flow**: FormDB tables → Features → Training → Models (graph is just input)
4. **Real Value**: Prediction quality, not graph analysis
5. **Integration Point**: Graph algorithms are data sources, ML is the product

---

## FormDB + ML Architecture: The Big Picture

### FormProcessor's Role

**FormProcessor** oversees ML computation species and coordinates with FormDB:

```
┌─────────────────────────────────────────────────────────────┐
│                       FormProcessor                         │
│  (Oversees ML computation species, NOT graph algorithms)    │
└─────────────────┬───────────────────────────────────────────┘
                  │
                  │ Coordinates
                  ▼
┌─────────────────────────────────────────────────────────────┐
│                    ML Core Architecture                      │
│                                                              │
│  ┌────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ Feature        │→ │ Model Training  │→ │ Prediction  │ │
│  │ Engineering    │  │ & Optimization  │  │ Service     │ │
│  └────────────────┘  └─────────────────┘  └─────────────┘ │
│         ▲                                                    │
└─────────┼────────────────────────────────────────────────────┘
          │
          │ Consumes features from
          │
┌─────────┴────────────────────────────────────────────────────┐
│              Graph Algorithm Procedure Layer                 │
│                   (External, stubbed for now)                │
│                                                              │
│  PageRank → embeddings  │  Louvain → communities           │
│  FastRP → embeddings    │  LabelProp → labels              │
│                                                              │
│  Returns: PropertyValues (opaque to ML core)               │
└──────────────────────────────────────────────────────────────┘
```

### Data Flow: FormDB → ML

```
FormDB Tables
    │
    ├─→ Direct features (node properties, edges)
    │       │
    │       ▼
    │   Feature Assembly
    │       │
    └───────┴─→ Graph Algorithm Procedures (optional)
                    │ (e.g., PageRank, FastRP)
                    ▼
                PropertyValues (embeddings, centrality, etc.)
                    │
                    ▼
            ┌───────────────────┐
            │  Feature Matrix   │ ← Combined direct + computed features
            │  (Vec<Features>)  │
            └─────────┬─────────┘
                      │
                      ▼
            ┌───────────────────┐
            │  ML Training      │
            │  (Gradient Descent)│
            └─────────┬─────────┘
                      │
                      ▼
            ┌───────────────────┐
            │  Trained Model    │
            │  (DecisionTree,   │
            │   LogReg, etc.)   │
            └─────────┬─────────┘
                      │
                      ▼
            ┌───────────────────┐
            │  Predictions      │ → Store back in FormDB
            │  (PropertyValues) │
            └───────────────────┘
```

### Codegen Meta Macro Processor Role

The **Meta Macro Processor** generates ML training code:

1. **Model Builder Generation**

   - Generates model construction code from descriptors
   - `ModelType::DecisionTreeClassifier` → `DecisionTreeClassifier::new(...)`
   - Configures hyperparameters from descriptor

2. **Training Loop Generation**

   - Generates training loops (gradient descent iterations)
   - Handles batch processing, shuffling, early stopping
   - Produces optimized training code for specific model types

3. **Feature Pipeline Generation**

   - Generates feature assembly code from FeatureStepDescriptor
   - Optimizes feature extraction (vectorization, caching)
   - Handles missing values, normalization

4. **Evaluation Code Generation**
   - Generates metric computation code
   - Cross-validation loops
   - Confusion matrix construction

**Key Insight**: Codegen produces _efficient, specialized_ ML code from high-level descriptors. The ML architecture must be _codegen-friendly_ with clear, compositional abstractions.

### What FormProcessor Needs from ML Architecture

1. **Computation Species for ML**

   ```rust
   ComputationSpecies::MLTraining
   ComputationSpecies::MLPrediction
   ComputationSpecies::MLEvaluation
   ```

2. **Progress Tracking**

   - Training epoch progress
   - Validation metric updates
   - Early stopping signals

3. **Resource Management**

   - Memory limits for feature matrices
   - Concurrent model training (multiple models in AutoML)
   - Model storage/caching

4. **Error Propagation**

   - Training failures → FormProcessor
   - Invalid features → Pipeline error
   - Model convergence issues

5. **Descriptor-Driven**
   - All ML work specified by descriptors
   - No "hidden" configuration
   - Serializable, reproducible pipelines

---

## What Java GDS Actually Has (For Reference)

### ML Models Package Structure

```
ml-algo/src/main/java/org/neo4j/gds/ml/
├── models/
│   ├── Classifier.java              // Core ML model interface
│   ├── Features.java                 // Feature vector representation
│   ├── TrainerConfig.java           // Training configuration
│   ├── automl/                      // Auto-ML hyperparameter tuning
│   ├── linearregression/            // Linear regression
│   ├── logisticregression/          // Logistic regression
│   ├── randomforest/                // Random forest
│   └── mlp/                         // Multi-layer perceptron
├── decisiontree/
│   ├── DecisionTreeClassifierTrainer.java
│   ├── DecisionTreeRegressorTrainer.java
│   ├── DecisionTreePredictor.java
│   └── TreeNode.java
├── gradientdescent/
│   ├── Training.java                // Core training loop
│   ├── Objective.java               // Loss functions
│   └── TrainingStopper.java         // Early stopping
├── metrics/
│   ├── Metric.java                  // Model evaluation metrics
│   ├── classification/              // Accuracy, F1, etc.
│   └── regression/                  // RMSE, MAE, etc.
├── splitting/
│   ├── StratifiedKFoldSplitter.java // Cross-validation
│   └── TrainingExamplesSplit.java   // Train/test split
└── training/
    ├── TrainingStatistics.java
    └── CrossValidator.java
```

### Pipeline Training Package

```
pipeline/src/main/java/org/neo4j/gds/ml/pipeline/
├── nodePipeline/
│   ├── classification/
│   │   ├── NodeClassificationTrain.java
│   │   └── NodeClassificationTrainingPipeline.java
│   └── regression/
│       ├── NodeRegressionTrain.java
│       └── NodeRegressionTrainingPipeline.java
├── linkPipeline/
│   ├── train/
│   │   ├── LinkPredictionTrain.java
│   │   └── LinkPredictionTrainingPipeline.java
└── PipelineTrainer.java             // Generic trainer interface
```

---

## Phase 2.3 Proposal: Complete ML Core Architecture

### Goal

Build **production-ready ML architecture** as FormDB's primary computation platform.

### ML-First Scope (Phase 2.3)

Phase 2.3 builds a **complete, production-grade ML system**:

1. ✅ **ML Model Trait System** - Extensible abstraction for all ML models

   - Base `Model` trait (train, predict, evaluate)
   - `Classifier` and `Regressor` sub-traits
   - Serialization/deserialization support
   - Model metadata and versioning

2. ✅ **Feature Engineering** - Full feature pipeline

   - Feature assembly from PropertyValues
   - Feature transformation (normalization, encoding, scaling)
   - Feature selection and dimensionality reduction
   - Missing value handling
   - **Key**: Codegen-friendly feature descriptors

3. ✅ **Training Infrastructure** - Complete training system

   - Gradient descent with multiple optimizers (SGD, Adam, RMSprop)
   - Loss functions (cross-entropy, MSE, MAE, Huber)
   - Regularization (L1, L2, ElasticNet)
   - Early stopping and convergence detection
   - Learning rate schedules
   - **Key**: Codegen generates optimized training loops

4. ✅ **Model Evaluation** - Comprehensive metrics

   - Classification: accuracy, precision, recall, F1, AUC-ROC
   - Regression: RMSE, MAE, R², adjusted R²
   - Confusion matrices
   - Cross-validation (K-fold, stratified)
   - **Key**: Evaluation integrated into training pipeline

5. ✅ **ML Models** - Real implementations (not stubs!)

   - DecisionTree (classification + regression)
   - LogisticRegression
   - LinearRegression (with regularization)
   - Start with these three, extensible for RandomForest, MLP, etc.

6. ✅ **Pipeline Training** - End-to-end ML pipeline

   - Train → Validate → Test workflow
   - Hyperparameter tuning support (grid search, random search)
   - Model persistence and loading
   - **Key**: FormProcessor can orchestrate training

7. ✅ **FormProcessor Integration**
   - ComputationDescriptor for ML species
   - Progress tracking hooks
   - Resource management integration
   - Error propagation

### Graph Algorithm Stub Layer (Minimal Interface)

Graph algorithms are **external procedure calls** that ML consumes:

```rust
/// Stub interface for graph algorithm procedures.
/// Real implementation lives in separate procedure execution environment.
pub trait GraphProcedure: Send + Sync {
    /// Execute procedure and return PropertyValues.
    fn execute(&self, graph: &Graph, config: ProcedureConfig)
        -> Result<Arc<dyn PropertyValues>, ProcedureError>;

    /// Procedure name (e.g., "pageRank", "fastRP")
    fn name(&self) -> &str;
}

/// Phase 2.3: Mock implementations return random PropertyValues
/// Phase 3.x: Real procedure execution environment integration
```

**No graph algorithm implementation in Phase 2.3** - just the stub interface.

---

## Architecture Design

### 1. ML Model Trait (Core Abstraction)

**File**: `src/projection/native/ml/model.rs`

```rust
//! ML model traits and core abstractions.
//!
//! Provides the foundation for classifiers, regressors, and other ML models
//! used in pipeline training and prediction.

use std::sync::Arc;

/// Features for a single element (node or link).
/// Wraps a Vec<f64> for type safety and future optimizations.
#[derive(Debug, Clone)]
pub struct Features {
    values: Vec<f64>,
}

impl Features {
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn dimension(&self) -> usize {
        self.values.len()
    }
}

/// Target value for supervised learning.
#[derive(Debug, Clone)]
pub enum Target {
    /// Classification target (class label)
    Class(i64),
    /// Regression target (continuous value)
    Value(f64),
    /// Multi-class with probabilities
    Probabilities(Vec<f64>),
}

/// Training example: features + target.
#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub features: Features,
    pub target: Target,
}

/// Core ML model trait for supervised learning.
pub trait Model: Send + Sync {
    /// Train the model on examples.
    fn train(&mut self, examples: &[TrainingExample]) -> Result<TrainingStats, ModelError>;

    /// Predict on new features.
    fn predict(&self, features: &Features) -> Result<Prediction, ModelError>;

    /// Model type name.
    fn name(&self) -> &str;

    /// Number of features expected.
    fn feature_dimension(&self) -> usize;
}

/// Prediction result from a model.
#[derive(Debug, Clone)]
pub enum Prediction {
    /// Classification: class label + confidence
    Class { label: i64, confidence: f64 },
    /// Regression: continuous value
    Value(f64),
    /// Multi-class: probabilities per class
    Probabilities(Vec<f64>),
}

/// Training statistics.
#[derive(Debug, Clone)]
pub struct TrainingStats {
    pub epochs: usize,
    pub final_loss: f64,
    pub training_time_ms: u64,
}

/// Model training/prediction errors.
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Training failed: {0}")]
    TrainingFailed(String),

    #[error("Prediction failed: {0}")]
    PredictionFailed(String),

    #[error("Invalid features: expected {expected}, got {actual}")]
    InvalidFeatureDimension { expected: usize, actual: usize },

    #[error("Invalid training data: {0}")]
    InvalidTrainingData(String),
}
```

### 2. Feature Assembly

**File**: `src/projection/native/ml/feature_assembler.rs`

```rust
//! Feature assembly from pipeline state.
//!
//! Converts PropertyValues stored in PipelineState into Features for ML models.

use super::model::{Features, Target, TrainingExample};
use super::pipeline_executor::PipelineState;
use crate::types::properties::PropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::sync::Arc;

/// Assembles features from pipeline state for a single node.
pub struct FeatureAssembler {
    /// Feature names in order
    feature_names: Vec<String>,
}

impl FeatureAssembler {
    pub fn new(feature_names: Vec<String>) -> Self {
        Self { feature_names }
    }

    /// Assemble features for a node from pipeline state.
    pub fn assemble_node_features(
        &self,
        node_id: u64,
        state: &PipelineState,
    ) -> Result<Features, AssemblyError> {
        let mut values = Vec::new();

        for feature_name in &self.feature_names {
            let property_values = state.features.get(feature_name)
                .ok_or_else(|| AssemblyError::MissingFeature(feature_name.clone()))?;

            // Extract values based on property type
            if let Some(node_props) = property_values.as_any()
                .downcast_ref::<dyn NodePropertyValues>()
            {
                match property_values.value_type() {
                    ValueType::Long => {
                        let val = node_props.long_value(node_id)? as f64;
                        values.push(val);
                    }
                    ValueType::Double => {
                        let val = node_props.double_value(node_id)?;
                        values.push(val);
                    }
                    ValueType::DoubleArray => {
                        let arr = node_props.double_array_value(node_id)?;
                        values.extend(arr);
                    }
                    _ => return Err(AssemblyError::UnsupportedType(property_values.value_type())),
                }
            }
        }

        Ok(Features::new(values))
    }

    /// Assemble training examples for all nodes.
    pub fn assemble_training_examples(
        &self,
        node_ids: &[u64],
        state: &PipelineState,
        target_property: &str,
    ) -> Result<Vec<TrainingExample>, AssemblyError> {
        let target_values = state.features.get(target_property)
            .ok_or_else(|| AssemblyError::MissingFeature(target_property.to_string()))?;

        let mut examples = Vec::with_capacity(node_ids.len());

        for &node_id in node_ids {
            let features = self.assemble_node_features(node_id, state)?;

            // Extract target
            let target = if let Some(node_props) = target_values.as_any()
                .downcast_ref::<dyn NodePropertyValues>()
            {
                match target_values.value_type() {
                    ValueType::Long => Target::Class(node_props.long_value(node_id)?),
                    ValueType::Double => Target::Value(node_props.double_value(node_id)?),
                    _ => return Err(AssemblyError::UnsupportedTargetType),
                }
            } else {
                return Err(AssemblyError::InvalidTarget);
            };

            examples.push(TrainingExample { features, target });
        }

        Ok(examples)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssemblyError {
    #[error("Missing feature: {0}")]
    MissingFeature(String),

    #[error("Unsupported property type: {0:?}")]
    UnsupportedType(ValueType),

    #[error("Unsupported target type")]
    UnsupportedTargetType,

    #[error("Invalid target")]
    InvalidTarget,

    #[error("Property access error: {0}")]
    PropertyError(#[from] PropertyValuesError),
}
```

### 3. Simple ML Model (DecisionTreeStub)

**File**: `src/projection/native/ml/models/decision_tree.rs`

```rust
//! Simple decision tree for Phase 2.3.
//!
//! Stub implementation to prove ML training works in pipeline.
//! Real implementation will come later.

use super::super::model::{
    Features, Model, ModelError, Prediction, Target, TrainingExample, TrainingStats,
};
use std::time::Instant;

/// Simple decision tree stub.
/// Phase 2.3: Just learns mean/mode, proves training works.
/// Phase 2.4+: Real decision tree with splitting.
pub struct DecisionTreeStub {
    name: String,
    feature_dim: usize,
    /// Learned prediction (mean for regression, mode for classification)
    prediction: Option<Prediction>,
}

impl DecisionTreeStub {
    pub fn new(name: String, feature_dim: usize) -> Self {
        Self {
            name,
            feature_dim,
            prediction: None,
        }
    }
}

impl Model for DecisionTreeStub {
    fn train(&mut self, examples: &[TrainingExample]) -> Result<TrainingStats, ModelError> {
        let start = Instant::now();

        if examples.is_empty() {
            return Err(ModelError::InvalidTrainingData("No training examples".into()));
        }

        // Validate feature dimensions
        for ex in examples {
            if ex.features.dimension() != self.feature_dim {
                return Err(ModelError::InvalidFeatureDimension {
                    expected: self.feature_dim,
                    actual: ex.features.dimension(),
                });
            }
        }

        // Phase 2.3: Learn mean/mode (proves training works)
        // Phase 2.4+: Actual tree building with splitting
        self.prediction = match &examples[0].target {
            Target::Class(_) => {
                // Classification: find mode
                let mut counts = std::collections::HashMap::new();
                for ex in examples {
                    if let Target::Class(label) = ex.target {
                        *counts.entry(label).or_insert(0) += 1;
                    }
                }
                let mode = counts.iter().max_by_key(|(_, &count)| count).unwrap().0;
                Some(Prediction::Class {
                    label: *mode,
                    confidence: 1.0,
                })
            }
            Target::Value(_) => {
                // Regression: compute mean
                let sum: f64 = examples
                    .iter()
                    .filter_map(|ex| {
                        if let Target::Value(v) = ex.target {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .sum();
                Some(Prediction::Value(sum / examples.len() as f64))
            }
            Target::Probabilities(_) => {
                return Err(ModelError::TrainingFailed(
                    "Probability targets not yet supported".into(),
                ));
            }
        };

        let elapsed = start.elapsed();

        Ok(TrainingStats {
            epochs: 1,
            final_loss: 0.0, // Stub: no actual loss computation
            training_time_ms: elapsed.as_millis() as u64,
        })
    }

    fn predict(&self, features: &Features) -> Result<Prediction, ModelError> {
        if features.dimension() != self.feature_dim {
            return Err(ModelError::InvalidFeatureDimension {
                expected: self.feature_dim,
                actual: features.dimension(),
            });
        }

        self.prediction
            .clone()
            .ok_or_else(|| ModelError::PredictionFailed("Model not trained".into()))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dim
    }
}
```

### 4. Training Descriptor

**File**: `src/projection/codegen/ml/training_descriptor.rs`

```rust
//! ML training configuration descriptors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Training configuration for ML pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDescriptor {
    /// Model type to train
    pub model_type: ModelType,

    /// Target property name (what to predict)
    pub target_property: String,

    /// Feature properties to use
    pub feature_properties: Vec<String>,

    /// Training/validation split configuration
    pub split_config: SplitConfig,

    /// Model-specific parameters
    pub model_config: HashMap<String, serde_json::Value>,
}

/// Type of ML model to train.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    /// Decision tree classifier
    DecisionTreeClassifier,

    /// Decision tree regressor
    DecisionTreeRegressor,

    /// Logistic regression
    LogisticRegression,

    /// Random forest
    RandomForest,
}

/// Dataset split configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Fraction for training (0.0 to 1.0)
    pub train_fraction: f64,

    /// Fraction for validation (0.0 to 1.0)
    pub validation_fraction: f64,

    /// Fraction for test (0.0 to 1.0)
    pub test_fraction: f64,

    /// Random seed for reproducibility
    pub seed: u64,
}

impl Default for SplitConfig {
    fn default() -> Self {
        Self {
            train_fraction: 0.7,
            validation_fraction: 0.15,
            test_fraction: 0.15,
            seed: 42,
        }
    }
}
```

### 5. Pipeline Training Executor

**File**: `src/projection/native/ml/training_executor.rs`

```rust
//! ML model training executor for pipelines.

use super::feature_assembler::FeatureAssembler;
use super::model::Model;
use super::models::decision_tree::DecisionTreeStub;
use super::pipeline_executor::PipelineState;
use crate::projection::codegen::ml::training_descriptor::{ModelType, TrainingDescriptor};

/// Trains an ML model using features from pipeline state.
pub struct TrainingExecutor {
    descriptor: TrainingDescriptor,
}

impl TrainingExecutor {
    pub fn new(descriptor: TrainingDescriptor) -> Self {
        Self { descriptor }
    }

    /// Execute training on pipeline state.
    pub fn execute(
        &self,
        state: &PipelineState,
        node_ids: &[u64],
    ) -> Result<TrainingResult, TrainingError> {
        // 1. Assemble features
        let assembler = FeatureAssembler::new(self.descriptor.feature_properties.clone());
        let examples = assembler.assemble_training_examples(
            node_ids,
            state,
            &self.descriptor.target_property,
        )?;

        // 2. Split dataset
        let (train_examples, val_examples) = self.split_dataset(&examples)?;

        // 3. Create model
        let feature_dim = examples[0].features.dimension();
        let mut model = self.create_model(feature_dim)?;

        // 4. Train
        let stats = model.train(&train_examples)?;

        // 5. Validate (if validation set exists)
        let validation_score = if !val_examples.is_empty() {
            Some(self.evaluate_model(&model, &val_examples)?)
        } else {
            None
        };

        Ok(TrainingResult {
            model: Box::new(model),
            training_stats: stats,
            validation_score,
        })
    }

    fn create_model(&self, feature_dim: usize) -> Result<Box<dyn Model>, TrainingError> {
        match self.descriptor.model_type {
            ModelType::DecisionTreeClassifier => {
                Ok(Box::new(DecisionTreeStub::new(
                    "DecisionTreeClassifier".to_string(),
                    feature_dim,
                )))
            }
            ModelType::DecisionTreeRegressor => {
                Ok(Box::new(DecisionTreeStub::new(
                    "DecisionTreeRegressor".to_string(),
                    feature_dim,
                )))
            }
            _ => Err(TrainingError::UnsupportedModel(
                format!("{:?}", self.descriptor.model_type)
            )),
        }
    }

    fn split_dataset(
        &self,
        examples: &[TrainingExample],
    ) -> Result<(Vec<TrainingExample>, Vec<TrainingExample>), TrainingError> {
        let train_size = (examples.len() as f64 * self.descriptor.split_config.train_fraction) as usize;

        Ok((
            examples[..train_size].to_vec(),
            examples[train_size..].to_vec(),
        ))
    }

    fn evaluate_model(
        &self,
        model: &dyn Model,
        examples: &[TrainingExample],
    ) -> Result<f64, TrainingError> {
        // Phase 2.3: Simple accuracy/RMSE
        // Phase 2.4+: Full metrics (F1, AUC, etc.)
        let mut correct = 0;
        for ex in examples {
            let pred = model.predict(&ex.features)?;
            match (&pred, &ex.target) {
                (Prediction::Class { label, .. }, Target::Class(target)) => {
                    if label == target {
                        correct += 1;
                    }
                }
                _ => {}
            }
        }
        Ok(correct as f64 / examples.len() as f64)
    }
}

pub struct TrainingResult {
    pub model: Box<dyn Model>,
    pub training_stats: TrainingStats,
    pub validation_score: Option<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum TrainingError {
    #[error("Feature assembly error: {0}")]
    AssemblyError(#[from] AssemblyError),

    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),
}
```

---

## Phase 2.3 Implementation Plan: ML-First

This plan builds **production-grade ML architecture**, not prototypes.

### Layer 1: Core ML Foundations (Days 1-2)

**Goal**: Trait system and abstractions for ML models

#### 1.1 Model Trait System

- [ ] `src/projection/native/ml/model.rs`
  - Base `Model` trait (train, predict, evaluate, serialize)
  - `Classifier` sub-trait (with class probabilities)
  - `Regressor` sub-trait (with prediction intervals)
  - Features, Target, TrainingExample, Prediction types
  - ModelMetadata (version, training date, hyperparameters)
  - **Codegen hook**: `#[derive(ModelBuilder)]` for descriptor → model

#### 1.2 Training Infrastructure

- [ ] `src/projection/native/ml/training/mod.rs`
  - `Optimizer` trait (SGD, Adam, RMSprop)
  - `LossFunction` trait (CrossEntropy, MSE, MAE, Huber)
  - `Regularization` (L1, L2, ElasticNet)
  - `TrainingConfig` (epochs, batch_size, learning_rate, etc.)
  - `TrainingLoop` - generic training iteration
  - **Codegen hook**: Generate optimized training loops per model type

#### 1.3 Evaluation System

- [ ] `src/projection/native/ml/metrics/mod.rs`
  - Classification metrics: `Accuracy`, `Precision`, `Recall`, `F1Score`, `AUCROC`
  - Regression metrics: `RMSE`, `MAE`, `R2Score`, `AdjustedR2`
  - `ConfusionMatrix`
  - `CrossValidator` (K-fold, stratified)
  - `MetricTracker` - track metrics over training
  - **FormProcessor hook**: Progress tracking integration

### Layer 2: Feature Engineering (Days 3-4)

**Goal**: Transform PropertyValues → ML-ready features

#### 2.1 Feature Assembly

- [ ] `src/projection/native/ml/features/assembler.rs`
  - `FeatureAssembler` - extract from PropertyValues
  - Handle missing values (imputation strategies)
  - Handle different value types (long, double, double-array)
  - **Codegen hook**: Generate specialized assembly for known schemas

#### 2.2 Feature Transformation

- [ ] `src/projection/native/ml/features/transformation.rs`
  - `Normalizer` (min-max, z-score, robust)
  - `Encoder` (one-hot, label encoding, target encoding)
  - `Scaler` (standardization)
  - `FeaturePipeline` - chain transformations
  - **Codegen hook**: Generate vectorized transformation code

#### 2.3 Feature Selection

- [ ] `src/projection/native/ml/features/selection.rs`
  - `VarianceThreshold` - remove low-variance features
  - `CorrelationFilter` - remove correlated features
  - `FeatureImportance` - model-based selection
  - **FormProcessor hook**: Report feature importance

### Layer 3: ML Models (Days 5-7)

**Goal**: Real ML models, not stubs

#### 3.1 Decision Tree

- [ ] `src/projection/native/ml/models/decision_tree/mod.rs`
  - `DecisionTreeClassifier` - real implementation
  - `DecisionTreeRegressor`
  - Tree building with splitting (Gini, Entropy, MSE)
  - Pruning support (pre-pruning, post-pruning)
  - Tree visualization/serialization
  - **Codegen hook**: Generate optimized split evaluation

#### 3.2 Linear Models

- [ ] `src/projection/native/ml/models/linear/logistic_regression.rs`
  - `LogisticRegression` with gradient descent
  - Support L1/L2 regularization
  - Multi-class (one-vs-rest, softmax)
- [ ] `src/projection/native/ml/models/linear/linear_regression.rs`
  - `LinearRegression` with normal equation and gradient descent
  - Ridge, Lasso, ElasticNet variants
  - **Codegen hook**: Generate matrix operations for known dimensions

#### 3.3 Model Registry

- [ ] `src/projection/native/ml/models/registry.rs`
  - `ModelRegistry` - register model types
  - `ModelFactory` - create models from descriptors
  - Model versioning and compatibility
  - **FormProcessor hook**: Track deployed models

### Layer 4: Training Orchestration (Days 8-9)

**Goal**: End-to-end training pipelines

#### 4.1 Training Descriptors

- [ ] `src/projection/codegen/ml/training_descriptor.rs`
  - `TrainingDescriptor` - complete training config
  - `ModelType` enum (extensible)
  - `OptimizationConfig` (optimizer, learning rate, etc.)
  - `ValidationConfig` (split, cross-validation)
  - **Codegen input**: Descriptors drive code generation

#### 4.2 Training Executor

- [ ] `src/projection/native/ml/training_executor.rs`
  - `TrainingExecutor` - orchestrate training
  - Dataset splitting (train/val/test, stratified)
  - Early stopping logic
  - Model checkpointing
  - Training resumption
  - **FormProcessor hook**: Report training progress

#### 4.3 Hyperparameter Tuning

- [ ] `src/projection/native/ml/tuning/mod.rs`
  - `GridSearch` - exhaustive parameter search
  - `RandomSearch` - random sampling
  - `ParamGrid` - parameter space definition
  - **FormProcessor hook**: Parallel model training

### Layer 5: FormProcessor Integration (Day 10)

**Goal**: Wire ML architecture into FormProcessor

#### 5.1 Computation Species

- [ ] `src/projection/codegen/computation_descriptor.rs`
  - Add `ComputationSpecies::MLTraining`
  - Add `ComputationSpecies::MLPrediction`
  - Add `ComputationSpecies::MLEvaluation`
  - Register ML species with FormProcessor

#### 5.2 Progress Tracking

- [ ] `src/projection/native/ml/progress.rs`
  - `TrainingProgress` - epoch, loss, metrics
  - `ProgressReporter` trait - send progress to FormProcessor
  - Integration with existing progress system

#### 5.3 Resource Management

- [ ] `src/projection/native/ml/resources.rs`
  - `MemoryEstimator` - estimate feature matrix size
  - `ResourceLimits` - enforce limits
  - Integration with FormProcessor resource management

### Layer 6: Graph Procedure Stub (Minimal)

**Goal**: Minimal interface for graph algorithms

#### 6.1 Procedure Interface

- [ ] `src/projection/native/ml/graph_procedure.rs`
  - `GraphProcedure` trait - stub interface
  - `MockGraphProcedure` - return random PropertyValues
  - **Phase 3.x**: Real procedure execution integration

### Layer 7: Testing & Documentation (Ongoing)

#### 7.1 Unit Tests

- [ ] Tests for each model type
- [ ] Tests for optimizers and loss functions
- [ ] Tests for feature transformations
- [ ] Tests for evaluation metrics

#### 7.2 Integration Tests

- [ ] `tests/ml/end_to_end_classification.rs`
- [ ] `tests/ml/end_to_end_regression.rs`
- [ ] `tests/ml/cross_validation.rs`
- [ ] `tests/ml/hyperparameter_tuning.rs`

#### 7.3 Documentation

- [ ] `doc/ML_PHASE_2_3_COMPLETE.md` - completion report
- [ ] `doc/ML_MODEL_GUIDE.md` - how to add new models
- [ ] `doc/ML_CODEGEN_GUIDE.md` - codegen integration
- [ ] `examples/ml_training_showcase.rs` - comprehensive example

---

## Success Criteria

1. ✅ **ML Model Trait** - Clean abstraction for supervised learning
2. ✅ **Feature Assembly** - Convert PropertyValues → Features
3. ✅ **Training Works** - DecisionTreeStub trains on examples
4. ✅ **Pipeline Integration** - Training executes after feature extraction
5. ✅ **Test Coverage** - 10+ new tests, all passing

---

## What This Achieves

### Phase 2.3 Deliverables

1. **ML Foundation** - Model trait, Features, TrainingExample
2. **Training Infrastructure** - TrainingExecutor, dataset splitting
3. **Simple Model** - DecisionTreeStub (proves training works)
4. **Pipeline Training** - Full pipeline can train models

### What's Still Deferred

1. **GDS Algorithms** - NodePropertyStep still returns mocks
2. **Real ML Models** - DecisionTree, LogisticRegression implementations
3. **Metrics** - Accuracy, F1, RMSE, AUC
4. **Cross-Validation** - K-fold splitting
5. **Auto-Tuning** - Hyperparameter search

---

## Key Design Decisions

### 1. **ComputationDescriptor Integration**

**Question**: Should ML training use ComputationDescriptor?

**Answer**: Yes, but subtly:

```rust
// ML training is a computation species
let training_computation = ComputationDescriptor::new(
    100, // Reserved range for ML
    "MLTraining",
    ComputationSpecies::MapReduce, // Training is map-reduce-ish
    ComputationPattern::Global,     // Operates on whole dataset
);
```

But we don't need this in Phase 2.3 - it's for later when we want:

- Progress tracking integration
- Resource management
- Distributed training

### 2. **Feature Assembly Strategy**

**Approach**: Read from PipelineState, not directly from graph.

**Rationale**:

- Features are already extracted in pipeline steps
- Allows complex feature engineering
- Matches Java GDS pattern

### 3. **Model Storage**

**Approach**: Return `Box<dyn Model>` from training.

**Future**: Store in model catalog (Phase 2.4+).

### 4. **Stub vs Real**

**Phase 2.3**: DecisionTreeStub (mean/mode)  
**Phase 2.4+**: Real decision tree with splitting

**Rationale**: Prove architecture works before implementing complex algorithms.

---

## Files to Create (Phase 2.3)

### New Files (8)

1. `src/projection/native/ml/model.rs` (Model trait, Features, Target)
2. `src/projection/native/ml/feature_assembler.rs` (Feature assembly)
3. `src/projection/native/ml/training_executor.rs` (Training executor)
4. `src/projection/native/ml/models/mod.rs` (Models module)
5. `src/projection/native/ml/models/decision_tree.rs` (DecisionTreeStub)
6. `src/projection/codegen/ml/training_descriptor.rs` (Training config)
7. `tests/ml/training_integration_test.rs` (Integration tests)
8. `doc/ML_PHASE_2_3_COMPLETE.md` (Completion report)

### Modified Files (2)

9. `src/projection/native/ml/mod.rs` (Add exports)
10. `src/projection/codegen/ml/mod.rs` (Add training descriptor)

---

## ML-First Architecture: Updated Success Criteria

### Production-Grade ML (Not Prototypes)

Phase 2.3 now delivers **complete ML platform**, not just proof-of-concept:

1. ✅ **Real ML Models** - DecisionTree (with real splitting), LogisticRegression, LinearRegression
2. ✅ **Training Infrastructure** - Gradient descent, optimizers, loss functions, regularization
3. ✅ **Feature Engineering** - Assembly, transformation, normalization, encoding
4. ✅ **Evaluation System** - Classification/regression metrics, cross-validation
5. ✅ **FormProcessor Integration** - Progress tracking, resource management, error propagation
6. ✅ **Codegen Hooks** - All operations descriptor-driven for meta macro processor
7. ✅ **Comprehensive Testing** - 50+ unit tests, 10+ integration tests, >90% coverage

### What This Achieves for FormDB

1. **FormDB can do production ML** - Not demos, real training and prediction
2. **Codegen target complete** - Meta macro processor has full ML surface to generate
3. **FormProcessor integration** - ML is a first-class computation species
4. **Graph-agnostic** - Graph algorithms are external data sources, not core dependency

### What's Deferred (Separate Concerns)

1. **Graph Algorithm Implementation** - Separate procedure execution environment
2. **Advanced ML Models** - RandomForest, GradientBoosting (extensible for Phase 3.x)
3. **AutoML** - Hyperparameter search (foundation in place)
4. **Distributed Training** - Multi-node (single-node production-ready first)

---

## Key Design Decisions: ML-First for FormDB

### 1. ML is Primary, Graph is Secondary

**Old thinking**: "Graph platform with ML support"  
**New thinking**: "ML platform with graph feature support"

**Impact**:

- ML architecture gets production-grade implementation in Phase 2.3
- Graph algorithms are stub interface, real implementation deferred
- FormProcessor oversees ML, not graph traversals
- Codegen targets ML training, not graph procedures

### 2. Real Implementations from Day 1

**Old plan**: DecisionTreeStub (mean/mode) → Real tree later  
**New plan**: Real DecisionTree with splitting in Phase 2.3

**Rationale**:

- Can't validate ML architecture with fake algorithms
- Training convergence issues reveal design problems
- Performance matters (O(n log n) splits)
- FormDB needs production ML now, not later

### 3. FormProcessor Integration Now

**Old plan**: ComputationDescriptor later  
**New plan**: Full FormProcessor integration in Phase 2.3

**Why**:

- Progress tracking essential (training is long-running)
- Resource management prevents OOM
- Error propagation for debugging
- FormProcessor needs to see ML as computation species

### 4. Codegen-Driven from Start

**Old plan**: Hand-written ML code  
**New plan**: Descriptor → Codegen → Optimized ML code

**Why**:

- Meta macro processor needs ML surface to generate
- Descriptors enable reproducible pipelines
- Generated code is specialized and fast
- Clear what/how separation

---

## Files to Create: Updated for Production ML

### Core ML Foundation (~2500 lines)

1. `src/projection/native/ml/model.rs` - Model trait system (200 lines)
2. `src/projection/native/ml/training/mod.rs` - Training infrastructure (400 lines)
3. `src/projection/native/ml/training/optimizer.rs` - SGD, Adam (300 lines)
4. `src/projection/native/ml/training/loss.rs` - Loss functions (200 lines)
5. `src/projection/native/ml/metrics/mod.rs` - Evaluation metrics (400 lines)

### Feature Engineering (~800 lines)

6. `src/projection/native/ml/features/assembler.rs` - Feature assembly (250 lines)
7. `src/projection/native/ml/features/transformation.rs` - Normalization, encoding (350 lines)
8. `src/projection/native/ml/features/selection.rs` - Feature selection (200 lines)

### ML Models (~1500 lines)

9. `src/projection/native/ml/models/decision_tree/mod.rs` - Real DecisionTree (600 lines)
10. `src/projection/native/ml/models/linear/logistic_regression.rs` - LogReg (400 lines)
11. `src/projection/native/ml/models/linear/linear_regression.rs` - LinReg (300 lines)
12. `src/projection/native/ml/models/registry.rs` - Model registry (200 lines)

### Training Orchestration (~600 lines)

13. `src/projection/codegen/ml/training_descriptor.rs` - Training config (200 lines)
14. `src/projection/native/ml/training_executor.rs` - Training orchestration (300 lines)
15. `src/projection/native/ml/tuning/mod.rs` - Hyperparameter tuning (100 lines)

### FormProcessor Integration (~400 lines)

16. `src/projection/native/ml/progress.rs` - Progress tracking (150 lines)
17. `src/projection/native/ml/resources.rs` - Resource management (150 lines)
18. `src/projection/codegen/computation_descriptor.rs` - ML species (100 lines, modify existing)

### Graph Procedure Stub (~100 lines)

19. `src/projection/native/ml/graph_procedure.rs` - Minimal stub interface

### Tests (~2000 lines)

20. `tests/ml/models/` - Model unit tests (800 lines)
21. `tests/ml/training/` - Training unit tests (600 lines)
22. `tests/ml/integration/` - End-to-end tests (600 lines)

### Documentation

23. `doc/ML_PHASE_2_3_COMPLETE.md` - Completion report
24. `doc/ML_MODEL_GUIDE.md` - How to add new models
25. `doc/ML_CODEGEN_GUIDE.md` - Codegen integration guide
26. `examples/ml_training_showcase.rs` - Comprehensive example

**Total**: ~8000 lines of production ML infrastructure

---

## Implementation Strategy

### Phase 2.3a: Core Foundation (Days 1-3)

- Model trait system
- Training infrastructure (optimizers, loss functions)
- Feature assembly
- Basic metrics

**Deliverable**: Can train simple model (logistic regression) end-to-end

### Phase 2.3b: Real Models (Days 4-6)

- DecisionTree with real splitting
- LogisticRegression with gradient descent
- LinearRegression with regularization

**Deliverable**: Three production-ready models

### Phase 2.3c: Advanced Features (Days 7-8)

- Feature transformation pipeline
- Cross-validation
- Hyperparameter tuning
- Model registry

**Deliverable**: Complete ML platform

### Phase 2.3d: FormProcessor Integration (Days 9-10)

- Progress tracking
- Resource management
- Computation species registration
- Error propagation

**Deliverable**: FormProcessor-orchestrated ML training

---

## Questions for Review

### Strategic Questions

1. **Scope confirmation**: Build production ML in Phase 2.3, defer graph algorithms? ✅
2. **FormProcessor integration**: In Phase 2.3 or later? ✅ NOW
3. **Real vs stub**: Real ML models in Phase 2.3? ✅ REAL

### Technical Questions

4. **Model scope**: DecisionTree + LinearModels sufficient for Phase 2.3?
5. **Optimizer scope**: SGD + Adam enough, or add more?
6. **Feature transforms**: Which transformations are must-have vs nice-to-have?
7. **Codegen hooks**: Where exactly should codegen plug in?

### Architecture Questions

8. **Module location**: Keep in `projection/native/ml/` or move to top-level `src/ml/`?
9. **Graph procedure interface**: Is minimal stub interface sufficient?
10. **Progress reporting**: What granularity does FormProcessor need?

---

## Ready to Implement?

**If approved**, Phase 2.3 will deliver:

- **~8000 lines** of production ML infrastructure (not ~800 prototype lines)
- **3 real ML models** (DecisionTree, LogisticRegression, LinearRegression)
- **Complete training system** (optimizers, loss functions, metrics)
- **Feature engineering pipeline** (assembly, transformation, selection)
- **FormProcessor integration** (progress, resources, computation species)
- **Codegen-ready architecture** (descriptors, hooks, compositional design)
- **50+ tests** (unit + integration, >90% coverage)

**ETA**: 10 days for production-grade ML platform

**First step**: Confirm scope and answer review questions above.
