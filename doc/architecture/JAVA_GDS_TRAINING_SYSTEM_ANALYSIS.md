# Java GDS Training System Analysis

## Purpose

Document Java GDS training patterns for ML pipelines to guide Rust implementation.
Focus: Node-centric model training with hyperparameter tuning.

## Core Java GDS Training Concepts

### 1. Training Phase

```java
// org.neo4j.gds.ml.pipeline.NodeClassificationPipeline
public class NodeClassificationPipeline {
    private List<NodePropertyStep> nodePropertySteps;
    private List<FeatureStep> featureSteps;
    private SplitConfig splitConfig;
    private List<ModelCandidate> modelCandidates;

    // Train on node features
    public Model train(Graph graph, String targetProperty) {
        // Execute node property steps
        // Assemble features
        // Split dataset (train/val/test)
        // Train each model candidate
        // Select best model based on validation metric
        return bestModel;
    }
}
```

### 2. Model Candidates

```java
// org.neo4j.gds.ml.models.ModelCandidate
public class ModelCandidate {
    private String modelType; // "LogisticRegression", "RandomForest", etc.
    private Map<String, Object> hyperparameters;

    public Model train(Features features, Labels labels) {
        // Train specific model with hyperparameters
    }
}
```

### 3. Hyperparameter Search

```java
// org.neo4j.gds.ml.training.TrainingStatistics
public class TrainingStatistics {
    private Map<ModelCandidate, Double> validationScores;
    private Map<ModelCandidate, TrainingMetrics> metrics;

    public ModelCandidate selectBestModel() {
        // Compare validation scores
        // Return best performing candidate
    }
}
```

### 4. Training Metrics

```java
// org.neo4j.gds.ml.metrics.Metric
public interface Metric {
    double compute(Labels predicted, Labels actual);
}

// Common metrics:
// - Accuracy
// - F1 Score
// - ROC AUC
// - Precision/Recall
```

### 5. Model Training Loop

```java
// Pseudo-code for training executor
for (ModelCandidate candidate : modelCandidates) {
    // 1. Train on training set
    Model model = candidate.train(trainFeatures, trainLabels);

    // 2. Evaluate on validation set
    double score = metric.compute(
        model.predict(validationFeatures),
        validationLabels
    );

    // 3. Track statistics
    statistics.record(candidate, score, metrics);
}

// 4. Select best
ModelCandidate best = statistics.selectBestModel();
Model finalModel = best.train(trainFeatures, trainLabels);

// 5. Final evaluation on test set
double testScore = metric.compute(
    finalModel.predict(testFeatures),
    testLabels
);
```

## Rust Design Patterns

### Phase 2.3 Architecture (Node-Centric)

```rust
/// Training executor for ML pipelines.
/// Node-centric: trains models to predict node properties/labels.
pub struct TrainingExecutor {
    model_candidates: Vec<ModelCandidate>,
    metric: ValidationMetric,
    statistics: TrainingStatistics,
}

impl TrainingExecutor {
    /// Train all model candidates, select best.
    pub fn train(
        &mut self,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &str,
        splits: &DatasetSplits,
    ) -> Result<Box<dyn Model>, TrainingError> {
        // 1. Extract train/validation/test node IDs
        let train_nodes = &splits.train_indices;
        let val_nodes = &splits.validation_indices;

        // 2. Get target labels for nodes
        let target_values = features.get(target)?;

        // 3. Train each candidate on train nodes
        for candidate in &self.model_candidates {
            let model = candidate.train(features, target_values, train_nodes)?;

            // 4. Evaluate on validation nodes
            let val_score = self.evaluate(&model, features, target_values, val_nodes)?;

            self.statistics.record(candidate.clone(), val_score);
        }

        // 5. Select best model
        let best_candidate = self.statistics.best_candidate()?;
        let final_model = best_candidate.train(features, target_values, train_nodes)?;

        Ok(final_model)
    }

    /// Evaluate model on nodes.
    fn evaluate(
        &self,
        model: &dyn Model,
        features: &HashMap<String, Arc<dyn PropertyValues>>,
        target: &Arc<dyn PropertyValues>,
        node_ids: &[usize],
    ) -> Result<f64, TrainingError> {
        // Predict on validation nodes
        let predictions = model.predict(features, node_ids)?;

        // Compare with actual labels
        let score = self.metric.compute(&predictions, target, node_ids)?;

        Ok(score)
    }
}
```

### Node-Centric Training Statistics

```rust
/// Track training results for model selection.
pub struct TrainingStatistics {
    results: Vec<TrainingResult>,
}

pub struct TrainingResult {
    candidate: ModelCandidate,
    validation_score: f64,
    training_time_ms: u64,
}

impl TrainingStatistics {
    pub fn record(&mut self, candidate: ModelCandidate, score: f64) {
        self.results.push(TrainingResult {
            candidate,
            validation_score: score,
            training_time_ms: 0, // TODO: track time
        });
    }

    pub fn best_candidate(&self) -> Result<&ModelCandidate, TrainingError> {
        self.results
            .iter()
            .max_by(|a, b| a.validation_score.partial_cmp(&b.validation_score).unwrap())
            .map(|r| &r.candidate)
            .ok_or(TrainingError::NoCandidates)
    }
}
```

## Implementation Plan

### Day 7: Training Executor Core

1. Create `training_executor.rs`
2. Define `TrainingExecutor` struct
3. Implement `train()` - train all candidates, select best
4. Implement `evaluate()` - validation scoring
5. Create `TrainingStatistics` for tracking results
6. Define `TrainingError` enum
7. Add 5-6 unit tests

### Day 8: Hyperparameter Search (Future)

1. Grid search implementation
2. Random search implementation
3. Cross-validation support
4. Early stopping

## Key Differences from Java GDS

1. **Node IDs as indices**: Rust uses Vec indices (usize) for node IDs, Java uses long
2. **Trait objects**: `Box<dyn Model>` instead of Java's interface polymorphism
3. **Result types**: Rust's Result<T, E> for error handling vs Java exceptions
4. **Ownership**: Rust's Arc for shared node features vs Java references
5. **No reflection**: Rust needs explicit trait implementations vs Java's runtime introspection

## Phase 2.3 Simplifications

- Single validation metric (Accuracy)
- No cross-validation (just train/val/test split)
- No early stopping
- No model serialization yet
- Simplified hyperparameter specification
- Node-centric only (no relationship prediction)

## Phase 2.5 Enhancements

- Multiple metrics support
- K-fold cross-validation
- Grid/random search
- Model checkpointing
- Distributed training
- Link prediction support

## Testing Strategy

```rust
#[test]
fn test_train_single_candidate() {
    // Setup: mock features, target, splits
    let mut executor = TrainingExecutor::new(vec![candidate], ValidationMetric::Accuracy);

    // Train
    let model = executor.train(&features, "label", &splits).unwrap();

    // Verify model returned
    assert!(model.is_trained());
}

#[test]
fn test_select_best_model() {
    // Setup: multiple candidates
    let candidates = vec![
        candidate_lr(),
        candidate_rf()
    ];

    let mut executor = TrainingExecutor::new(candidates, ValidationMetric::Accuracy);

    // Train - should pick best
    let model = executor.train(&features, "label", &splits).unwrap();

    // Verify best selected
    assert_eq!(executor.statistics().best_score(), expected_best);
}
```

## Node-Centric Examples

```rust
// Example: Train node classifier on Karate Club graph
let graph = create_karate_graph();

// Node properties: PageRank, degree, clustering coefficient
let node_props = execute_node_properties(&graph);

// Node features: normalized properties
let node_features = assemble_features(&node_props);

// Node labels: club membership (0 or 1)
let node_labels = graph.node_property("club");

// Split nodes
let splits = DatasetSplits::from_fractions(&node_ids, 0.6, 0.2, 0.2);

// Train node classifier
let mut executor = TrainingExecutor::new(candidates, ValidationMetric::F1);
let model = executor.train(&node_features, "club", &splits)?;

// Predict on test nodes
let predictions = model.predict(&node_features, &splits.test_indices)?;
```

## Summary

Java GDS training executor provides:

1. Model candidate training with hyperparameters
2. Validation-based model selection
3. Training statistics tracking
4. Multiple metric support
5. Node-centric classification/regression

Rust implementation captures these patterns with:

- `TrainingExecutor` for orchestration
- `TrainingStatistics` for tracking
- `ModelCandidate` from descriptor
- Node ID-based train/val/test splits
- Trait-based Model abstraction

Phase 2.3: Simple single-metric training
Phase 2.5: Full hyperparameter search
