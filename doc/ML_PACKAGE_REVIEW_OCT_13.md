# ML Package Review - October 13, 2025

## Current Status: 67 Tests Passing

### ✅ Completed (Tasks 1-8)

**Infrastructure (40 tests):**

1. **Pipeline Descriptor** (3 tests) - Training config, model candidates, splits
2. **Pipeline Executor** (6 tests) - Core orchestration with builder pattern
3. **Pipeline State** (9 tests) - Runtime data, dataset splits
4. **Graph Procedure Stub** (10 tests) - Mock PageRank/FastRP/Louvain
5. **Pipeline Executor Core** (12 tests) - Full orchestration flow

**Features (15 tests):** 6. **Feature Assembly** (15 tests) - Node-centric transformations, Phase 2.3 identity only

**Training & Models (12 tests):** 7. **Training Executor** (7 tests) - Hyperparameter search, validation-based selection 8. **Model Trait System** (5 tests) - Model trait, ModelError, ModelMetadata, MockModel

### ⚠️ In Progress

**Task 9: DecisionTreeClassifier** - Created but has PropertyValues trait casting issues

- Problem: PropertyValues doesn't implement Any trait, can't downcast to MockDoublePropertyValues
- Blocker: Need trait redesign OR simplified Phase 2.3 approach

## Architecture Summary

### Node-Centric Design (Consistent Throughout)

```rust
// All ML operates on node features/properties/IDs
features: HashMap<String, Arc<dyn PropertyValues>>  // Node features
target: Arc<dyn PropertyValues>                      // Node labels
node_ids: Vec<usize>                                 // Train/val/test nodes

// Model API
model.fit(&features, &target, &train_nodes)
model.predict(&features, &test_nodes) -> Vec<f64>
```

### Module Structure

```
src/projection/
├── codegen/ml/               # Descriptors (Java GDS mapping)
│   ├── pipeline_descriptor.rs  # PipelineDescriptor, ModelType, etc.
│   └── mod.rs
└── native/ml/                # Runtime (execution)
    ├── features/             # Feature transformations
    │   ├── validation.rs
    │   ├── transformation.rs
    │   └── assembler.rs
    ├── models/               # ML models
    │   ├── model_trait.rs
    │   └── decision_tree.rs  # ⚠️ WIP
    ├── pipeline_executor.rs  # Main orchestrator
    ├── pipeline_state.rs     # Runtime data
    ├── training_executor.rs  # Training & tuning
    ├── graph_procedure.rs    # GDS procedure stubs
    └── mock_property_values.rs  # Test helpers
```

### Key Patterns Captured from Java GDS

1. **Pipeline Descriptor Pattern**

   - Java: `GraphSagePipeline`, `NodeClassificationPipeline`
   - Rust: `PipelineDescriptor` with `PipelineType` enum

2. **Model Candidate Pattern**

   - Java: `ModelCandidate` with hyperparameters
   - Rust: `ModelCandidate { model_type, params }`

3. **Training with Validation**

   - Java: Train multiple candidates → validate → select best
   - Rust: `TrainingExecutor::train()` → `TrainingStatistics`

4. **Feature Transformation**

   - Java: `FeatureStep` interface
   - Rust: `Transformation` trait

5. **Model Interface**
   - Java: `Model.fit()`, `.predict()`
   - Rust: `Model` trait with fit/predict/evaluate

## Critical Issue: PropertyValues Trait System

### Problem

```rust
// This doesn't work:
let mock = prop.as_any().downcast_ref::<MockDoublePropertyValues>()?;
// Error: PropertyValues doesn't have as_any() method
```

### Root Cause

- `PropertyValues` trait doesn't inherit from `Any`
- Can't downcast `Arc<dyn PropertyValues>` to concrete types
- Blocks model implementations from extracting scalar values

### Options

**Option A: Add as_any() to PropertyValues** (Invasive)

```rust
pub trait PropertyValues: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;  // Add this
    fn value_type(&self) -> ValueType;
    fn element_count(&self) -> usize;
}
```

- PRO: Enables downcasting everywhere
- CON: Modifies core trait, affects all implementations

**Option B: Phase 2.3 Simplified Models** (Pragmatic)

```rust
// Store feature vectors directly, bypass PropertyValues
pub struct DecisionTreeClassifier {
    feature_data: Option<Vec<Vec<f64>>>,  // Cached during fit()
    // ...
}

impl Model for DecisionTreeClassifier {
    fn fit(&mut self, features: &HashMap<...>, target: &Arc<...>, node_ids: &[usize]) {
        // Extract once, store internally
        self.feature_data = Some(extract_to_vec(features, node_ids));
        // Train on cached data
    }
}
```

- PRO: Works now, no trait changes
- CON: Less general, Phase 2.5 refactor needed

**Option C: NodePropertyValues Specialization** (Middle Ground)

```rust
// Use NodePropertyValues trait which has double_value(), long_value()
// Require features to implement NodePropertyValues, not just PropertyValues
fn fit(
    &mut self,
    features: &HashMap<String, Arc<dyn NodePropertyValues>>,  // More specific
    // ...
)
```

- PRO: Type-safe access to node properties
- CON: Changes Model trait signature

## Recommendation: Option B for Phase 2.3

**Rationale:**

- User emphasized: "we have a lot to learn", "Not going to be the final product"
- Phase 2.3 goal: Pattern capture, not perfection
- Can refactor in Phase 2.5 when we understand full PropertyValues usage

**Implementation:**

```rust
// Simplified DecisionTreeClassifier for Phase 2.3
pub struct DecisionTreeClassifier {
    tree: Option<TreeNode>,
    feature_names: Vec<String>,
    // Cache training data to avoid PropertyValues complexity
    cached_features: Option<Vec<Vec<f64>>>,
    metadata: ModelMetadata,
}

// Helper function outside Model trait
fn extract_mock_features(
    features: &HashMap<String, Arc<dyn PropertyValues>>,
    node_ids: &[usize],
) -> Result<Vec<Vec<f64>>, ModelError> {
    // TODO Phase 2.5: Generic extraction
    // For now: Document that Phase 2.3 requires MockDoublePropertyValues
    unimplemented!("Phase 2.3: Use test helpers to create Vec<Vec<f64>> directly")
}
```

## Next Steps (Priority Order)

### IMMEDIATE: Complete Task 9 (DecisionTreeClassifier)

1. **Option B.1**: Simplify to accept `Vec<Vec<f64>>` directly in tests
2. **Option B.2**: Add helper that works with MockDoublePropertyValues specifically
3. Document: "Phase 2.3: Test-only, Phase 2.5: Production PropertyValues support"
4. Target: 8-9 decision tree tests passing

### HIGH: Task 10 - End-to-End Pipeline Test

- Goal: Validate full flow works with actual graph
- Steps:
  1. Create small graph (Karate Club - 34 nodes)
  2. Run mock PageRank (populate node properties)
  3. Assemble features (identity transformation)
  4. Split dataset (train/val/test)
  5. Train DecisionTreeClassifier
  6. Evaluate on test set
- Success: 1 comprehensive integration test passing

### MEDIUM: Task 11 - Pipeline Catalog

- Goal: Named pipeline storage/retrieval
- Pattern: Java GDS `PipelineCatalog`
- APIs: `add()`, `get()`, `drop()`, `list()`
- Success: 5-6 catalog tests passing

### FUTURE: Phase 2.5 Enhancements

1. **PropertyValues Integration**

   - Decide on Option A vs C (trait redesign)
   - Implement generic extraction for all PropertyValues types
   - Support embedding properties (DoubleArray)

2. **Full Feature Transformations**

   - Implement NormalizeTransformation (currently returns identity)
   - Implement CombineTransformation
   - Add L1/L2 normalization
   - Add StandardScaler pattern

3. **Advanced Models**

   - LogisticRegression
   - RandomForest
   - GradientBoosting
   - Neural network (linear layers)

4. **Cross-Validation**

   - K-fold splits
   - Stratified sampling
   - Multiple validation metrics

5. **Production Features**
   - Model serialization (save/load)
   - Feature importance
   - Hyperparameter optimization (Bayesian)
   - Early stopping
   - Model ensembles

## Code Navigation Guide

### To Add a New Model

1. Create `src/projection/native/ml/models/your_model.rs`
2. Implement `Model` trait (fit, predict, evaluate)
3. Add to `models/mod.rs` exports
4. Add ModelType variant if needed (in codegen/ml/pipeline_descriptor.rs)
5. Write 5-8 tests (creation, fit, predict, evaluate, metadata)

### To Add a New Transformation

1. Add to `src/projection/native/ml/features/transformation.rs`
2. Implement `Transformation` trait (transform, name)
3. Add tests in same file
4. Use in `DefaultFeatureAssembler`

### To Add a New Algorithm Step

1. Create mock in `graph_procedure.rs` (Phase 2.3)
2. Add ProcedureCategory variant if needed
3. Register in `create_mock_registry()`
4. Add to pipeline_executor's execute_node_property_steps()
5. Implement actual algorithm in Phase 2.5 (src/algorithms/)

### To Run Tests

```bash
# All ML tests
cargo test --lib native::ml

# Specific module
cargo test --lib native::ml::features
cargo test --lib native::ml::models
cargo test --lib native::ml::training_executor

# Specific test
cargo test --lib test_decision_tree_fit
```

## Key Learnings

1. **Node-Centric Design Works**

   - Consistent API across all modules
   - HashMap<String, PropertyValues> for features
   - Vec<usize> for node IDs
   - Natural fit for graph ML

2. **Java GDS Patterns Translate Well**

   - Pipeline descriptors map cleanly
   - Model candidates enable auto-tuning
   - Feature transformations are composable
   - Validation-based model selection works

3. **Phase 2.3 Strategy Is Sound**

   - Identity transformations sufficient for architecture
   - Mock procedures enable testing without algorithms
   - Simplified implementations reveal design issues early

4. **PropertyValues Abstraction Needs Work**

   - Too abstract for ML model implementations
   - Downcasting issues block concrete usage
   - Need either: trait redesign OR usage pattern shift

5. **Testing Strategy Working**
   - MockPropertyValues enable unit tests
   - Small graphs (4 nodes) sufficient for logic testing
   - Integration tests reveal orchestration issues

## Metrics

- **Total ML Tests**: 67 passing
- **Code Coverage**: ~85% (estimated)
- **Module Count**: 10 files in ml/ directory
- **Lines of Code**: ~3500 in ml module
- **Documentation**: 2 Java GDS analysis docs (800+ lines)
- **Time Invested**: ~8 hours
- **Tasks Complete**: 8/11 (73%)

## Decision Point: PropertyValues Trait

**Question for User:** Should we:

1. Modify PropertyValues trait to add as_any()? (Invasive, enables all models)
2. Keep Phase 2.3 simple with test-only models? (Quick, refactor later)
3. Change Model trait to require NodePropertyValues? (Middle ground, type-safe)

**My Recommendation**: Option 2 for now. Complete Tasks 9-11 with simplified approach, then revisit trait design in Phase 2.5 when we have more usage examples.
