# Java GDS Pipeline Architecture Analysis

**Date**: October 14, 2025  
**Context**: Planning proper translation of Java GDS Pipeline system to Rust

## Current State

### What We Have (Speculative/Form)

- `projection/native/form/` - Speculative Form Processor implementation
- Pipeline executor, state, step executor, training executor
- **Status**: All commented out, needs rewrite for proper Java GDS patterns

### What We Need (Java GDS ML Pipeline)

- `projection/native/ml/pipeline/` - Proper Java GDS Pipeline translation
- **Status**: Currently active but needs proper architecture

## Java GDS Pipeline Architecture

### Hierarchy Overview

```
Pipeline<FEATURE_STEP>                    (Base interface)
├── TrainingPipeline<FEATURE_STEP>        (Training-specific)
│   ├── NodePropertyTrainingPipeline      (Node property prediction)
│   │   ├── NodeClassificationTrainingPipeline
│   │   └── NodeRegressionTrainingPipeline
│   └── LinkPredictionTrainingPipeline    (Link prediction)
└── PredictPipeline                       (Inference)
    ├── NodePropertyPredictPipeline
    └── LinkPredictionPredictPipeline
```

### Core Abstraction: Pipeline<FEATURE_STEP>

**Location**: `pipeline/src/main/java/org/neo4j/gds/ml/pipeline/Pipeline.java`

**Key Methods**:

- `nodePropertySteps()` - List of graph algorithm steps
- `featureSteps()` - List of feature extraction steps
- `featureProperties()` - Derived feature property names
- `validateBeforeExecution(graphStore, nodeLabels)` - Pre-execution validation
- `validateFeatureProperties(graphStore, nodeLabels)` - Post-step validation
- `specificValidateBeforeExecution(graphStore)` - Subclass-specific validation

**Pattern**: Template method pattern with validation hooks

### Node Pipeline Sophistication

**Node Classification Pipeline**:

```
NodeClassificationTrainingPipeline
├── nodePropertySteps: List<ExecutableNodePropertyStep>
├── featureSteps: List<NodeFeatureStep>
├── splitConfig: NodePropertyPredictionSplitConfig
├── parameterSpace: Map<String, List<Map<String, Object>>>
└── trainingParameterSpace: Map<String, List<Map<String, Object>>>
```

**Node Regression Pipeline**:

```
NodeRegressionTrainingPipeline
├── Similar structure to classification
└── Different metrics (RMSE vs Accuracy)
```

**Key Sophistication**:

1. **Hyperparameter Search Space** - `parameterSpace` for model tuning
2. **Split Configuration** - Train/Test/Validation splits with strategies
3. **Feature Engineering** - NodeFeatureStep with various transformations
4. **Metrics** - Task-specific evaluation metrics

### Link Pipeline Sophistication

**Link Prediction Pipeline**:

```
LinkPredictionTrainingPipeline
├── nodePropertySteps: List<ExecutableNodePropertyStep>
├── featureSteps: List<LinkFeatureStep>      // Different from NodeFeatureStep!
├── splitConfig: LinkPredictionSplitConfig   // Different split strategy!
├── negativeClassWeight: double              // Class imbalance handling
└── relationshipWeightProperty: Optional<String>
```

**Key Sophistication**:

1. **LinkFeatureStep** - Operates on relationship pairs, not nodes
2. **Negative Sampling** - Generate negative examples for training
3. **Class Weighting** - Handle imbalanced positive/negative examples
4. **Relationship Types** - Train/Test edge types separate from feature graph

**Link Feature Extraction**:

- Hadamard product of node embeddings
- L2 distance between node embeddings
- Cosine similarity
- Element-wise operations on node features

### Executor Architecture

**PipelineExecutor<PIPELINE_CONFIG, PIPELINE, RESULT>**:

```java
abstract class PipelineExecutor {
    enum DatasetSplits {
        TRAIN,           // Training data
        TEST,            // Test data
        TEST_COMPLEMENT, // Everything not in test (train + validation)
        FEATURE_INPUT    // Full graph for feature computation
    }

    // Abstract methods subclasses implement
    abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters();
    abstract void splitDatasets();
    abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits);
    abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps();

    // Template method
    RESULT compute() {
        1. Generate dataset splits
        2. Validate pipeline
        3. Execute node property steps (feature engineering)
        4. Validate feature properties exist
        5. Execute model training/prediction
        6. Cleanup intermediate properties
        7. Return result
    }
}
```

**Executor Hierarchy**:

```
PipelineExecutor
├── NodeClassificationPipelineTrainExecutor
├── NodeRegressionPipelineTrainExecutor
├── LinkPredictionPipelineTrainExecutor
├── NodeClassificationPipelineStreamExecutor
├── NodeRegressionPipelineStreamExecutor
└── LinkPredictionPipelineStreamExecutor
```

### Split Strategies

**Node Property Split**:

- Stratified by target property
- Holdout method (train/test/validation)
- K-fold cross-validation support

**Link Prediction Split**:

- Edge-level splitting (not node-level!)
- Negative sampling strategy
- Maintain graph connectivity
- Separate feature graph from train/test edges

### Feature Steps

**NodeFeatureStep**:

```java
interface NodeFeatureStep {
    String name();
    List<String> inputNodeProperties();
    Map<String, Object> config();
}
```

**LinkFeatureStep**:

```java
interface LinkFeatureStep {
    String name();
    List<String> inputNodeProperties();
    LinkFeatureExtractor extractor();
}
```

**Key Difference**: Link features operate on _pairs_ of nodes, not individual nodes!

## Rust Translation Strategy

### Module Structure

```
src/projection/native/ml/
├── pipeline/
│   ├── mod.rs                          // Pipeline trait
│   ├── training_pipeline.rs           // TrainingPipeline trait
│   ├── predict_pipeline.rs            // PredictPipeline trait
│   ├── node/
│   │   ├── mod.rs
│   │   ├── classification_pipeline.rs
│   │   ├── regression_pipeline.rs
│   │   └── feature_step.rs
│   └── link/
│       ├── mod.rs
│       ├── prediction_pipeline.rs
│       ├── feature_step.rs
│       └── negative_sampler.rs
├── executor/
│   ├── mod.rs                          // PipelineExecutor trait
│   ├── node_classification_executor.rs
│   ├── node_regression_executor.rs
│   └── link_prediction_executor.rs
├── split/
│   ├── mod.rs
│   ├── node_split.rs                   // Stratified node splits
│   └── link_split.rs                   // Edge-level splits
└── features/
    ├── mod.rs
    ├── node_features.rs
    └── link_features.rs
```

### Key Translation Patterns

**1. Pipeline Trait Hierarchy**:

```rust
pub trait Pipeline {
    type FeatureStep: FeatureStep;

    fn node_property_steps(&self) -> &[ExecutableNodePropertyStep];
    fn feature_steps(&self) -> &[Self::FeatureStep];
    fn validate_before_execution(&self, graph_store: &GraphStore, node_labels: &[NodeLabel]);
}

pub trait TrainingPipeline: Pipeline {
    type SplitConfig: SplitConfig;
    type Metrics: Metrics;

    fn split_config(&self) -> &Self::SplitConfig;
    fn parameter_space(&self) -> &HashMap<String, Vec<serde_json::Value>>;
}

pub trait NodePropertyPipeline: TrainingPipeline {
    type FeatureStep = NodeFeatureStep;
    // Node-specific methods
}

pub trait LinkPredictionPipeline: TrainingPipeline {
    type FeatureStep = LinkFeatureStep;
    fn negative_class_weight(&self) -> f64;
    fn relationship_weight_property(&self) -> Option<&str>;
}
```

**2. Executor Template Method**:

```rust
pub trait PipelineExecutor {
    type Config;
    type Pipeline;
    type Result;

    fn generate_dataset_splits(&self) -> HashMap<DatasetSplit, PipelineGraphFilter>;
    fn split_datasets(&mut self);
    fn execute_impl(&self, splits: &HashMap<DatasetSplit, PipelineGraphFilter>) -> Self::Result;

    // Template method
    fn compute(&mut self) -> Self::Result {
        let splits = self.generate_dataset_splits();
        self.validate_pipeline(&splits);
        self.execute_node_property_steps(&splits);
        self.split_datasets();
        let result = self.execute_impl(&splits);
        self.cleanup();
        result
    }
}
```

**3. Link Feature Pattern**:

```rust
pub trait LinkFeatureExtractor {
    fn extract(&self, source_features: &[f64], target_features: &[f64]) -> Vec<f64>;
}

pub struct HadamardFeatureExtractor;
impl LinkFeatureExtractor for HadamardFeatureExtractor {
    fn extract(&self, source: &[f64], target: &[f64]) -> Vec<f64> {
        source.iter().zip(target).map(|(s, t)| s * t).collect()
    }
}

pub struct L2DistanceFeatureExtractor;
impl LinkFeatureExtractor for L2DistanceFeatureExtractor {
    fn extract(&self, source: &[f64], target: &[f64]) -> Vec<f64> {
        let dist = source.iter().zip(target)
            .map(|(s, t)| (s - t).powi(2))
            .sum::<f64>()
            .sqrt();
        vec![dist]
    }
}
```

## Key Sophistication to Preserve

### 1. Type-Level Differentiation

- NodeFeatureStep vs LinkFeatureStep (different operations)
- NodeSplitConfig vs LinkSplitConfig (different strategies)

### 2. Hyperparameter Search

- Parameter space exploration
- Cross-validation
- Model selection

### 3. Dataset Splitting

- **Node**: Stratified by target, preserves class distribution
- **Link**: Edge-level, maintains connectivity, negative sampling

### 4. Feature Engineering Pipeline

- Sequential node property steps
- Feature extraction from properties
- Transformations (normalization, standardization)

### 5. Validation Hooks

- Pre-execution validation (properties exist)
- Post-step validation (features computed)
- Graph structure validation

## Next Steps

1. **Create base Pipeline trait** - Core abstraction
2. **Implement NodePropertyPipeline** - Classification + Regression
3. **Implement LinkPredictionPipeline** - With negative sampling
4. **Create PipelineExecutor trait** - Template method pattern
5. **Implement split strategies** - Node stratified, Link edge-level
6. **Port feature extractors** - Node features, Link features
7. **Add hyperparameter search** - Grid/random search
8. **Testing** - Unit tests for each component

## Design Decisions

**Use Rust traits for Java interfaces**:

- `Pipeline` trait = Java `Pipeline<FEATURE_STEP>`
- Associated types for FEATURE_STEP (NodeFeatureStep vs LinkFeatureStep)

**Use trait objects for dynamic dispatch**:

- `Box<dyn FeatureExtractor>` for runtime feature selection
- `Arc<dyn Model>` for trained models

**Use builder pattern for pipelines**:

- `NodeClassificationPipeline::builder()`
- Add steps fluently: `.add_node_property_step(...).add_feature_step(...)`

**Leverage type system**:

- Compile-time distinction between Node and Link pipelines
- Type-safe split configurations
- Generic executors: `Executor<NodeClassificationPipeline>`

## Questions to Resolve

1. **How much hyperparameter search to include?**

   - Grid search, random search, Bayesian optimization?
   - Start with grid search, defer advanced methods?

2. **Split strategy complexity?**

   - Full cross-validation or just train/test?
   - Start simple, add complexity as needed?

3. **Link negative sampling?**

   - Random negative sampling or more sophisticated?
   - Degree-aware sampling? Temporal constraints?

4. **Model zoo scope?**
   - Just logistic regression to start?
   - Add random forest, gradient boosting later?

## Conclusion

Java GDS Pipeline is **highly sophisticated**:

- 2-level hierarchy (Node vs Link)
- 2-stage pipeline (Property Steps → Feature Steps → Training)
- Type-safe feature extractors
- Flexible split strategies
- Hyperparameter search

Our Rust translation should:

- ✅ Preserve type-level differentiation (traits + associated types)
- ✅ Use template method pattern (compute() orchestration)
- ✅ Start with core abstractions (Pipeline, Executor)
- ✅ Add sophistication incrementally (splits, features, search)
- ✅ Leverage Rust's type system (compile-time safety)

**Ready to translate!** 🚀
