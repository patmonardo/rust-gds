# ML Pipeline Complete Architecture

**Status**: Design Phase Complete  
**Date**: October 15, 2025

## Three Pipeline Types - Complete Picture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         GDS ML Pipeline System                              │
│                                                                             │
│  GraphStore ──▶ Pipeline ──▶ Training ──▶ Model ──▶ Predictions           │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────────────────┬──────────────────────┬──────────────────────────────┐
│  Node Classification │   Node Regression    │     Link Prediction          │
├──────────────────────┼──────────────────────┼──────────────────────────────┤
│ Predict: Node Class  │ Predict: Node Value  │ Predict: Relationship Exists │
│ Target: Node Property│ Target: Node Property│ Target: Relationship Type    │
│ Model: Classifier    │ Model: Regressor     │ Model: Classifier (Binary)   │
│ Features: Node Props │ Features: Node Props │ Features: Pair-based         │
│ Splitting: Nodes     │ Splitting: Nodes     │ Splitting: Relationships     │
│ Multi-class          │ Continuous values    │ Binary only                  │
└──────────────────────┴──────────────────────┴──────────────────────────────┘
```

## File Count Summary

| Pipeline Type             | Core Files | Training Files | Link Function Files | Total Files | Total Lines |
| ------------------------- | ---------- | -------------- | ------------------- | ----------- | ----------- |
| **Node Classification**   | 7          | 6              | -                   | 13          | ~1,500      |
| **Node Regression**       | 7          | 8              | -                   | 15          | ~1,800      |
| **Link Prediction**       | 6          | 5              | 14                  | 25          | ~2,800      |
| **Shared Infrastructure** | ~15        | -              | -                   | 15          | ~1,000      |
| **TOTAL**                 | ~35        | ~19            | 14                  | **68**      | **~7,100**  |

## Translation Progress

### ✅ Node Classification Pipeline (COMPLETE)

- **Status**: 13/13 files translated (~1,500 lines)
- **Core**: NodeClassificationTrainingPipeline, NodeClassificationPredictPipeline, Config
- **Training**: NodeClassificationTrain, TrainAlgorithm, AlgorithmFactory, ModelInfo, ToModelConverter
- **Features**: NodeFeatureProducer integration
- **Model**: Classifier-based

### ✅ Node Regression Pipeline (COMPLETE - 95%)

- **Status**: 7/8 regression files translated (~750 lines) + 7 shared classification files
- **Core**: NodeRegressionTrainingPipeline, Config
- **Training**: TrainResult, ModelInfo, ToModelConverter, TrainAlgorithm, AlgorithmFactory
- **Deferred**: NodeRegressionTrain.rs (~400 lines) - complex cross-validation
- **Model**: Regressor-based
- **Key Discovery**: Exposed complete Model::of() API through converter pattern

### 📋 Link Prediction Pipeline (PLANNED)

- **Status**: 0/25 files, comprehensive plan created
- **Phases**: 5 phases over 3 weeks (~2,800 lines)
- **Phase 1**: Core Types (6 files, ~350 lines)
- **Phase 2**: Link Functions (8 files, ~850 lines)
- **Phase 3**: Extraction System (3 files, ~330 lines)
- **Phase 4**: Pipeline Core (3 files, ~420 lines)
- **Phase 5**: Training System (5 files, ~1,200 lines)
- **Unique Features**: Pair-based features, negative sampling, relationship splitting

## Key Architectural Discoveries

### 1. Feature Extraction Patterns

**Node Pipelines** (Classification & Regression):

```rust
pub trait NodeFeatureStep {
    fn execute(&self, graph: &Graph, node_id: NodeId) -> Vec<f64>;
    fn input_node_properties(&self) -> Vec<String>;
}
```

**Link Pipeline**:

```rust
pub trait LinkFeatureStep {
    fn link_feature_appender(&self, graph: &Graph) -> Box<dyn LinkFeatureAppender>;
    fn input_node_properties(&self) -> Vec<String>;
}

pub trait LinkFeatureAppender {
    fn append_features(&self, source: NodeId, target: NodeId, features: &mut [f64], offset: usize);
    fn dimension(&self) -> usize;
    fn is_symmetric(&self) -> bool;
}
```

**Key Difference**: Node features extract from single nodes; link features extract from node pairs.

### 2. Data Splitting Patterns

**Node Pipelines**:

```rust
// Split nodes into train/test/validation
NodeSplitter::split(
    graph: &Graph,
    test_fraction: f64,
    validation_folds: usize
) -> NodeSplitResult
```

**Link Pipeline**:

```rust
// Split relationships + generate negative samples
LinkPredictionRelationshipSampler::split_relationships(
    graph_store: &GraphStore,
    config: &LinkPredictionSplitConfig,  // includes negative sampling ratios
    // ...
) -> RelationshipSplitResult {
    train_positive: Graph,
    train_negative: Graph,  // Generated non-existent relationships
    test_positive: Graph,
    test_negative: Graph,
    // ...
}
```

**Key Difference**: Link prediction requires negative sampling (generating non-existent relationships for binary classification).

### 3. Model API Pattern (Discovered from Node Regression)

```rust
Model::of(
    gds_version: String,           // "2.5.0"
    model_type: String,            // "NodeClassification", "NodeRegression", "LinkPrediction"
    graph_schema: GraphSchema,     // Training graph structure
    data: DATA,                    // ClassifierData, RegressorData, LinkPredictorData
    train_config: CONFIG,          // Pipeline-specific config
    custom_info: INFO,             // Pipeline-specific metadata (ModelInfo)
)
```

**All Three Pipelines Use Same Pattern**:

- Node Classification: `Model<ClassifierData, NodeClassificationTrainPipelineConfig, NodeClassificationPipelineModelInfo>`
- Node Regression: `Model<RegressorData, NodeRegressionTrainPipelineConfig, NodeRegressionPipelineModelInfo>`
- Link Prediction: `Model<ClassifierData, LinkPredictionTrainConfig, LinkPredictionModelInfo>`

### 4. Training Pipeline Pattern

**All Three Pipelines Follow This Flow**:

```
1. Node Property Steps (ExecutableNodePropertyStep[])
   ↓ Execute mutating graph algorithms (PageRank, FastRP, etc.)

2. Feature Steps (NodePropertyStep[] or LinkFeatureStep[])
   ↓ Extract features from (augmented) graph

3. Data Splitting (Nodes or Relationships)
   ↓ Create train/test/validation sets

4. RandomSearch (Hyperparameter Tuning)
   ↓ Try multiple TrainerConfig candidates

5. Cross-Validation (K-Fold)
   ↓ Evaluate each candidate on validation sets

6. Model Selection (Best Metric)
   ↓ Choose best TrainerConfig

7. Final Training (Full Train Set)
   ↓ Train final model on all training data

8. Test Evaluation
   ↓ Evaluate on held-out test set

9. Model Creation
   ↓ Model::of(...) with trained model + metadata
```

## Link Function Mathematics

Link prediction introduces **4 mathematical operations** on node property pairs:

### 1. Hadamard (Element-wise Product)

```
Input: v1 = [a, b, c], v2 = [x, y, z]
Output: [a*x, b*y, c*z]
Use Case: Feature interaction - which dimensions correlate?
```

### 2. Cosine Similarity

```
Input: v1, v2
Output: dot(v1, v2) / (norm(v1) * norm(v2))
Use Case: Angular similarity - do vectors point in same direction?
```

### 3. L2 Distance

```
Input: v1, v2
Output: sqrt(sum((v1[i] - v2[i])^2))
Use Case: Euclidean distance - how far apart are vectors?
```

### 4. Same Category

```
Input: v1 (categorical), v2 (categorical)
Output: 1 if v1 == v2, else 0
Use Case: Categorical similarity - same community, label, etc.?
```

**Node pipelines don't need these** - they work with single node properties directly.

## Negative Sampling (Link Prediction Only)

**Problem**: In real graphs, we observe relationships that **exist**. To train a binary classifier (exists/doesn't exist), we need **negative examples** (non-existent relationships).

**Solution**: `NegativeSampler` generates negative examples by:

1. Random pair sampling: Pick random (source, target) pairs that don't have a relationship
2. Degree-aware sampling: Bias towards nodes with similar degrees to positive examples
3. Validation: Ensure negatives don't exist in any train/test/validation set

**Configuration**:

```rust
LinkPredictionSplitConfig {
    test_fraction: 0.1,           // 10% relationships for test
    train_fraction: 0.1,          // 10% relationships for train
    validation_folds: 3,          // 3-fold cross-validation
    negative_sampling_ratio: 1.0, // 1:1 ratio (1 negative per positive)
}
```

**Result**:

- Train set: 10% positive + 10% negative (from sampled non-existent)
- Test set: 10% positive + 10% negative
- Validation sets: Stratified K-fold on train set

## Shared Infrastructure Requirements

All three pipelines depend on shared infrastructure:

### ml-core (Graph ML Core)

- `Features`, `FeaturesFactory` - Feature storage and manipulation
- `EdgeSplitter`, `UndirectedEdgeSplitter` - Relationship splitting
- `NegativeSampler` - Negative example generation for link prediction
- `LocalIdMap` - Node ID mapping for subgraphs
- `BatchQueue` - Mini-batch training

### ml-models (ML Models)

- `Classifier`, `ClassifierData` - Classification models (LogisticRegression, RandomForest, etc.)
- `Regressor`, `RegressorData` - Regression models (LinearRegression, etc.)
- `ClassifierTrainer`, `RegressorTrainer` - Model training
- `ClassifierTrainerFactory`, `RegressorTrainerFactory` - Trainer creation
- `TrainerConfig` - Hyperparameter configuration

### ml-training (Training Infrastructure)

- `CrossValidation` - K-fold cross-validation
- `StratifiedKFoldSplitter` - Stratified splitting (preserves class distribution)
- `RandomSearch` - Hyperparameter search
- `TrainingStatistics` - Training metrics and history

### ml-metrics (Evaluation Metrics)

- `Metric` - Base metric trait
- `ClassificationMetric` - Accuracy, F1, Precision, Recall, etc.
- `RegressionMetrics` - RMSE, MAE, R², etc.
- `LinkMetric` - AUCPR, ROC_AUC (for link prediction)
- `ModelCandidateStats` - Candidate evaluation results
- `SignedProbabilities` - Predictions with confidence

### pipeline-shared (Pipeline Infrastructure)

- `ExecutableNodePropertyStep` - Mutating graph algorithms
- `NodeFeatureProducer` - Node feature extraction orchestration
- `PipelineExecutor` - Pipeline execution framework
- `NodeSplitter` - Node-based train/test splitting

## Critical Design Decisions

### 1. Specialized vs Generic Pipeline Types

**Decision**: Keep specialized pipeline types (NodeClassification, NodeRegression, LinkPrediction)

**Rationale**:

- Node and Link features have fundamentally different semantics (single node vs pair)
- Splitting strategies differ significantly (nodes vs relationships + negative sampling)
- Type safety is more valuable than abstraction
- Minimal code duplication (only pipeline infrastructure)

### 2. Model API Generalization

**Decision**: Use `Model<DATA, CONFIG, INFO>` generic pattern

**Rationale**:

- All three pipelines follow same Model creation pattern
- DATA varies: ClassifierData, RegressorData
- CONFIG varies: Pipeline-specific train config
- INFO varies: Pipeline-specific ModelInfo
- Clean separation of concerns

### 3. Trait Hierarchy

**Proposed Hierarchy**:

```rust
pub trait FeatureStep {
    fn name(&self) -> &str;
    fn input_node_properties(&self) -> Vec<String>;
    fn to_map(&self) -> Map<String, Object>;
}

pub trait NodeFeatureStep: FeatureStep {
    // Node-specific feature extraction
}

pub trait LinkFeatureStep: FeatureStep {
    fn link_feature_appender(&self, graph: &Graph) -> Box<dyn LinkFeatureAppender>;
}

pub trait Pipeline<F: FeatureStep> {
    fn node_property_steps(&self) -> &[ExecutableNodePropertyStep];
    fn feature_steps(&self) -> &[F];
    fn to_map(&self) -> Map<String, Object>;
}
```

## Next Steps

### Immediate (This Week)

1. ✅ **COMPLETE**: Create LinkPipeline translation plan
2. 📋 **START**: Translate LinkPipeline Phase 1 (Core Types) - 6 files, ~350 lines

### Short Term (Next 2 Weeks)

3. Translate LinkPipeline Phases 2-5 - remaining 19 files, ~2,450 lines
4. Create Pipeline Comparison Document - identify generalization opportunities
5. Update MODEL_SYSTEM_REQUIREMENTS with Link Prediction specifics

### Medium Term (Next Month)

6. Translate Model<DATA, CONFIG, INFO> core system
7. Translate PipelineCatalog (simpler, user-scoped)
8. Translate ModelCatalog (complex, cross-user, persistence)

### Long Term (Next Quarter)

9. Complete ml-core infrastructure (EdgeSplitter, NegativeSampler)
10. Complete ml-models infrastructure (Classifier, Regressor, Trainers)
11. Complete ml-training infrastructure (CrossValidation, RandomSearch)
12. Complete ml-metrics infrastructure (all metric types)
13. Integration testing of complete ML Pipeline system

## Success Metrics

- ✅ Node Classification: 13/13 files complete
- ✅ Node Regression: 7/8 files complete (95%)
- 📋 Link Prediction: 0/25 files complete (0%)
- **Total Progress**: 20/56 pipeline files (36%)
- **Lines Translated**: ~2,250 / ~6,100 pipeline lines (37%)

**When Complete**:

- All three pipeline types fully translated
- Complete understanding of ML Pipeline architecture
- Model API requirements fully documented
- Ready to build Model and Catalog systems
- Foundation for GDS Procedures

---

**This document provides the complete architectural picture of GDS ML Pipeline system.**
