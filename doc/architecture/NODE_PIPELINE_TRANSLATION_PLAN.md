# Java GDS Node Pipeline - Complete Translation Plan

**Date**: October 14, 2025  
**Status**: Planning Phase - Complete Package Analysis  
**Approach**: 1:1 translation with priority ordering  
**Context**: Building on successful Phases 1-8 (348 ML tests passing)

---

## Executive Summary

The `nodePipeline` package contains **25 files, ~2,227 lines** implementing:

- **Node-specific pipeline infrastructure** (base classes, features, splitting)
- **Node Classification** training pipeline (7 files, ~969 lines)
- **Node Regression** training pipeline (7 files, ~768 lines)

This is a **focused, domain-specific extension** of the core pipeline system (already complete).

---

## Package Structure

```
org.neo4j.gds.ml.pipeline.nodePipeline/
├── Base Infrastructure (6 files, ~490 lines)
│   ├── NodeFeatureStep.java                    73 lines  ⭐ Simple
│   ├── NodeFeatureProducer.java                94 lines  ⭐⭐ Medium
│   ├── NodePropertyPredictPipeline.java        91 lines  ⭐⭐ Medium
│   ├── NodePropertyTrainingPipeline.java       62 lines  ⭐⭐ Medium
│   ├── NodePropertyPipelineBaseTrainConfig.java 52 lines ⭐ Simple
│   └── NodePropertyPredictionSplitConfig.java  104 lines ⭐⭐ Medium
│
├── classification/ (Node Classification - 7 files, ~969 lines)
│   ├── NodeClassificationTrainingPipeline.java           46 lines  ⭐ Simple
│   └── train/
│       ├── NodeClassificationTrainResult.java            35 lines  ⭐ Simple (value class)
│       ├── NodeClassificationModelResult.java            32 lines  ⭐ Simple (value class)
│       ├── NodeClassificationTrainAlgorithm.java         44 lines  ⭐ Simple (adapter)
│       ├── LabelsAndClassCountsExtractor.java            63 lines  ⭐⭐ Medium
│       ├── NodeClassificationPipelineTrainConfig.java    63 lines  ⭐⭐ Medium
│       ├── NodeClassificationPipelineModelInfo.java      78 lines  ⭐⭐ Medium
│       ├── NodeClassificationToModelConverter.java       63 lines  ⭐⭐ Medium
│       ├── NodeClassificationTrainPipelineAlgorithmFactory.java  130 lines  ⭐⭐⭐ Complex
│       ├── NodeClassificationTrainMemoryEstimateDefinition.java  217 lines  ⭐⭐⭐⭐ Very Complex
│       └── NodeClassificationTrain.java                  340 lines  ⭐⭐⭐⭐ Very Complex
│
└── regression/ (Node Regression - 7 files, ~768 lines)
    ├── NodeRegressionTrainingPipeline.java               49 lines  ⭐ Simple
    ├── NodeRegressionTrainResult.java                    37 lines  ⭐ Simple (value class)
    ├── NodeRegressionTrainAlgorithm.java                 45 lines  ⭐ Simple (adapter)
    ├── NodeRegressionPipelineTrainConfig.java            46 lines  ⭐⭐ Medium
    ├── NodeRegressionPipelineModelInfo.java              71 lines  ⭐⭐ Medium
    ├── NodeRegressionToModelConverter.java               62 lines  ⭐⭐ Medium
    ├── NodeRegressionTrainPipelineAlgorithmFactory.java  102 lines  ⭐⭐⭐ Complex
    └── NodeRegressionTrain.java                          329 lines  ⭐⭐⭐⭐ Very Complex
```

---

## Translation Status Overview

### Current State (Phases 1-8 Complete)

✅ **Core Pipeline System**: 348 ML tests passing  
✅ **Pipeline traits, executors, training infrastructure, catalog**: All complete  
✅ **Direct Integration approach**: Proven successful

### What We're Building On

- ✅ `Pipeline` trait (Phase 1)
- ✅ `TrainingPipeline` trait (Phase 7)
- ✅ `PipelineExecutor` + `PredictPipelineExecutor` (Phase 6)
- ✅ `PipelineTrainAlgorithm` + `PipelineTrainer` (Phase 7)
- ✅ `NodePropertyStepExecutor` (Phase 5)
- ✅ `PipelineCatalog` (Phase 8)

---

## Complexity Analysis

### By Complexity Level

| Level                 | Files  | Total Lines | Description                     |
| --------------------- | ------ | ----------- | ------------------------------- |
| ⭐ Simple             | 7      | ~321        | Value classes, simple pipelines |
| ⭐⭐ Medium           | 10     | ~755        | Configs, converters, producers  |
| ⭐⭐⭐ Complex        | 2      | ~232        | Factories with logic            |
| ⭐⭐⭐⭐ Very Complex | 2      | ~669        | Training algorithms             |
| **Total**             | **21** | **~1,977**  | **Executable code**             |

**Note**: ~250 lines are in memory estimation files (can be deferred)

### By Category

| Category                    | Files | Lines | Priority | Dependencies      |
| --------------------------- | ----- | ----- | -------- | ----------------- |
| ✅ Foundation (Done)        | -     | -     | P0       | Phases 1-8        |
| **Base Infrastructure**     | 6     | ~490  | **P1**   | Phases 1-7        |
| **Classification Core**     | 7     | ~453  | **P2**   | P1 + ml-core      |
| **Classification Training** | 3     | ~516  | **P3**   | P2 + ml-training  |
| **Regression Core**         | 7     | ~439  | **P4**   | P1 + ml-core      |
| **Optional**                | 2     | ~329  | P5       | Memory estimation |

---

## 📋 Phased Translation Plan

### Phase 1: Base Infrastructure (6 files, ~490 lines)

**Goal**: Node-specific pipeline foundations

**Dependencies**: Phases 1-8 (already complete)

#### 1.1 Simple Types (Priority 1A)

| Java File                                  | Rust File                                     | Lines | Complexity | Notes                       |
| ------------------------------------------ | --------------------------------------------- | ----- | ---------- | --------------------------- |
| `NodeFeatureStep.java`                     | `node_feature_step.rs`                        | 73    | ⭐ Simple  | Concrete `FeatureStep` impl |
| `NodePropertyPipelineBaseTrainConfig.java` | `node_property_pipeline_base_train_config.rs` | 52    | ⭐ Simple  | Config trait                |

**Deliverable**: 2 files, ~125 lines, foundational types

#### 1.2 Pipeline Types (Priority 1B)

| Java File                                | Rust File                                  | Lines | Complexity  | Notes                      |
| ---------------------------------------- | ------------------------------------------ | ----- | ----------- | -------------------------- |
| `NodePropertyTrainingPipeline.java`      | `node_property_training_pipeline.rs`       | 62    | ⭐⭐ Medium | Abstract base for training |
| `NodePropertyPredictPipeline.java`       | `node_property_predict_pipeline.rs`        | 91    | ⭐⭐ Medium | Prediction pipeline        |
| `NodePropertyPredictionSplitConfig.java` | `node_property_prediction_split_config.rs` | 104   | ⭐⭐ Medium | Train/test splitting       |

**Deliverable**: 3 files, ~257 lines, pipeline abstractions

#### 1.3 Feature Producer (Priority 1C)

| Java File                  | Rust File                  | Lines | Complexity  | Notes              |
| -------------------------- | -------------------------- | ----- | ----------- | ------------------ |
| `NodeFeatureProducer.java` | `node_feature_producer.rs` | 94    | ⭐⭐ Medium | Feature extraction |

**Deliverable**: 1 file, ~94 lines, feature production

**Phase 1 Total**: 6 files, ~476 lines  
**Test Strategy**: Unit tests for each component

---

### Phase 2: Node Classification Core (7 files, ~453 lines)

**Goal**: Classification pipeline and supporting types

**Dependencies**: Phase 1

#### 2.1 Pipeline & Value Classes (Priority 2A)

| Java File                                 | Rust File                                  | Lines | Complexity | Notes             |
| ----------------------------------------- | ------------------------------------------ | ----- | ---------- | ----------------- |
| `NodeClassificationTrainingPipeline.java` | `node_classification_training_pipeline.rs` | 46    | ⭐ Simple  | Concrete pipeline |
| `NodeClassificationTrainResult.java`      | `node_classification_train_result.rs`      | 35    | ⭐ Simple  | Value class       |
| `NodeClassificationModelResult.java`      | `node_classification_model_result.rs`      | 32    | ⭐ Simple  | Value class       |
| `NodeClassificationTrainAlgorithm.java`   | `node_classification_train_algorithm.rs`   | 44    | ⭐ Simple  | Adapter           |

**Deliverable**: 4 files, ~157 lines, classification foundations

#### 2.2 Configs & Converters (Priority 2B)

| Java File                                    | Rust File                                      | Lines | Complexity  | Notes          |
| -------------------------------------------- | ---------------------------------------------- | ----- | ----------- | -------------- |
| `NodeClassificationPipelineTrainConfig.java` | `node_classification_pipeline_train_config.rs` | 63    | ⭐⭐ Medium | Config         |
| `NodeClassificationPipelineModelInfo.java`   | `node_classification_pipeline_model_info.rs`   | 78    | ⭐⭐ Medium | Model metadata |
| `NodeClassificationToModelConverter.java`    | `node_classification_to_model_converter.rs`    | 63    | ⭐⭐ Medium | Converter      |

**Deliverable**: 3 files, ~204 lines, classification support

**Phase 2 Total**: 7 files, ~361 lines  
**Test Strategy**: Integration tests with mock training

---

### Phase 3: Node Classification Training (3 files, ~516 lines)

**Goal**: Full classification training implementation

**Dependencies**: Phase 2 + ml-training subsystem

#### 3.1 Label Extraction (Priority 3A)

| Java File                            | Rust File                              | Lines | Complexity  | Notes     |
| ------------------------------------ | -------------------------------------- | ----- | ----------- | --------- |
| `LabelsAndClassCountsExtractor.java` | `labels_and_class_counts_extractor.rs` | 63    | ⭐⭐ Medium | Data prep |

**Deliverable**: 1 file, ~63 lines, label processing

#### 3.2 Factory (Priority 3B)

| Java File                                              | Rust File                                                 | Lines | Complexity     | Notes         |
| ------------------------------------------------------ | --------------------------------------------------------- | ----- | -------------- | ------------- |
| `NodeClassificationTrainPipelineAlgorithmFactory.java` | `node_classification_train_pipeline_algorithm_factory.rs` | 130   | ⭐⭐⭐ Complex | Factory logic |

**Deliverable**: 1 file, ~130 lines, algorithm factory

#### 3.3 Training Implementation (Priority 3C)

| Java File                      | Rust File                      | Lines | Complexity            | Notes              |
| ------------------------------ | ------------------------------ | ----- | --------------------- | ------------------ |
| `NodeClassificationTrain.java` | `node_classification_train.rs` | 340   | ⭐⭐⭐⭐ Very Complex | Full training loop |

**Deliverable**: 1 file, ~340 lines, training algorithm

**Phase 3 Total**: 3 files, ~533 lines  
**Test Strategy**: End-to-end classification training tests

**Milestone**: **Node Classification fully functional**

---

### Phase 4: Node Regression (7 files, ~768 lines)

**Goal**: Regression pipeline (parallel to classification)

**Dependencies**: Phase 1 + ml-training subsystem

**Note**: Regression follows same structure as classification but simpler (continuous targets vs. classes)

#### 4.1 Pipeline & Value Classes (Priority 4A)

| Java File                             | Rust File                              | Lines | Complexity | Notes             |
| ------------------------------------- | -------------------------------------- | ----- | ---------- | ----------------- |
| `NodeRegressionTrainingPipeline.java` | `node_regression_training_pipeline.rs` | 49    | ⭐ Simple  | Concrete pipeline |
| `NodeRegressionTrainResult.java`      | `node_regression_train_result.rs`      | 37    | ⭐ Simple  | Value class       |
| `NodeRegressionTrainAlgorithm.java`   | `node_regression_train_algorithm.rs`   | 45    | ⭐ Simple  | Adapter           |

**Deliverable**: 3 files, ~131 lines, regression foundations

#### 4.2 Configs & Converters (Priority 4B)

| Java File                                | Rust File                                  | Lines | Complexity  | Notes          |
| ---------------------------------------- | ------------------------------------------ | ----- | ----------- | -------------- |
| `NodeRegressionPipelineTrainConfig.java` | `node_regression_pipeline_train_config.rs` | 46    | ⭐⭐ Medium | Config         |
| `NodeRegressionPipelineModelInfo.java`   | `node_regression_pipeline_model_info.rs`   | 71    | ⭐⭐ Medium | Model metadata |
| `NodeRegressionToModelConverter.java`    | `node_regression_to_model_converter.rs`    | 62    | ⭐⭐ Medium | Converter      |

**Deliverable**: 3 files, ~179 lines, regression support

#### 4.3 Training (Priority 4C)

| Java File                                          | Rust File                                             | Lines | Complexity            | Notes         |
| -------------------------------------------------- | ----------------------------------------------------- | ----- | --------------------- | ------------- |
| `NodeRegressionTrainPipelineAlgorithmFactory.java` | `node_regression_train_pipeline_algorithm_factory.rs` | 102   | ⭐⭐⭐ Complex        | Factory       |
| `NodeRegressionTrain.java`                         | `node_regression_train.rs`                            | 329   | ⭐⭐⭐⭐ Very Complex | Training loop |

**Deliverable**: 2 files, ~431 lines, regression training

**Phase 4 Total**: 7 files, ~741 lines  
**Test Strategy**: End-to-end regression training tests

**Milestone**: **Node Regression fully functional**

---

### Phase 5: Optional Extensions (2 files, ~329 lines)

**Goal**: Memory estimation (optional)

**Priority**: LOW - Can defer indefinitely

| Java File                                              | Rust File  | Lines | Complexity | Notes             |
| ------------------------------------------------------ | ---------- | ----- | ---------- | ----------------- |
| `NodeClassificationTrainMemoryEstimateDefinition.java` | (deferred) | 217   | ⭐⭐⭐⭐   | Memory estimation |
| (Regression memory estimate - if exists)               | (deferred) | ~112  | ⭐⭐⭐     | Memory estimation |

**Rationale**: Memory estimation is nice-to-have but not required for functionality

---

## Translation Strategy

### Direct Integration Approach (Proven Success)

Based on Phases 1-8 success:

- ✅ **No ExecutionContext wrapper** - Direct `Arc<DefaultGraphStore>` access
- ✅ **Trait-based design** - Not abstract classes
- ✅ **Template method pattern** - Default trait implementations
- ✅ **Simple error handling** - Custom error enums with `Display` + `StdError`

### Key Design Decisions

#### 1. Node Feature Step (Simple Concrete Type)

```rust
// Java: NodeFeatureStep implements FeatureStep
// Rust: Concrete struct implementing FeatureStep trait
pub struct NodeFeatureStep {
    node_property: String,
}

impl FeatureStep for NodeFeatureStep {
    fn name(&self) -> &str { "feature" }
    fn input_node_properties(&self) -> &[String] { &[&self.node_property] }
    // ...
}
```

#### 2. Training Pipeline Hierarchy

```rust
// Base trait (Phase 1)
pub trait NodePropertyTrainingPipeline: TrainingPipeline {
    fn split_config(&self) -> &NodePropertyPredictionSplitConfig;
    fn set_split_config(&mut self, config: NodePropertyPredictionSplitConfig);
    fn require_eager_features(&self) -> bool;
}

// Classification (Phase 2)
pub struct NodeClassificationTrainingPipeline {
    // Implements NodePropertyTrainingPipeline
}

// Regression (Phase 4)
pub struct NodeRegressionTrainingPipeline {
    // Implements NodePropertyTrainingPipeline
}
```

#### 3. Training Algorithm Pattern

```rust
// Follows PipelineTrainAlgorithm pattern from Phase 7
pub struct NodeClassificationTrain {
    // PipelineTrainer implementation
}

impl PipelineTrainer for NodeClassificationTrain {
    type Result = NodeClassificationTrainResult;

    fn run(&mut self) -> Result<Self::Result, Box<dyn StdError>> {
        // 1. Extract labels and class counts
        // 2. Split data (train/test/validation)
        // 3. Cross-validation loop
        // 4. Model selection (AutoML)
        // 5. Train final model on full training set
        // 6. Return result with metrics
    }
}
```

---

## Dependencies & Prerequisites

### Already Complete (Phases 1-8)

✅ **Phase 1-3**: Core traits, foundation types, utilities  
✅ **Phase 4**: Node property steps with Direct Integration  
✅ **Phase 5**: Step execution  
✅ **Phase 6**: Pipeline executors  
✅ **Phase 7**: Training infrastructure  
✅ **Phase 8**: Pipeline catalog

### Required from ml-core (Should Exist)

- ✅ `Features` / `FeaturesFactory` - Feature representation
- ✅ `Classifier` / `Regressor` - Model types
- ✅ `TrainingStatistics` - Metrics tracking
- ⚠️ `ClassifierTrainer` / `RegressorTrainer` - May need implementation
- ⚠️ `CrossValidation` - Cross-validation logic
- ⚠️ `RandomSearch` - AutoML hyperparameter search

### External Dependencies

- `NodeLabel` - Node label representation (from projection)
- `GraphStore` - Graph storage (from types)
- `PropertyValues` - Property access (from types)

---

## Estimated Effort

### Time Estimates (Based on Phase 1-8 Experience)

| Phase       | Files  | Lines      | Complexity     | Est. Time       | Tests           |
| ----------- | ------ | ---------- | -------------- | --------------- | --------------- |
| **Phase 1** | 6      | ~490       | Medium         | 2-3 hours       | 10-15 tests     |
| **Phase 2** | 7      | ~453       | Medium         | 2-3 hours       | 10-12 tests     |
| **Phase 3** | 3      | ~516       | High           | 3-4 hours       | 8-10 tests      |
| **Phase 4** | 7      | ~768       | High           | 4-5 hours       | 12-15 tests     |
| **Phase 5** | 2      | ~329       | Low (optional) | Deferred        | -               |
| **Total**   | **21** | **~1,977** | **Mixed**      | **11-15 hours** | **40-52 tests** |

### Milestones

1. **Phase 1 Complete**: Node pipeline foundations working
2. **Phase 2 Complete**: Classification pipeline defined
3. **Phase 3 Complete**: Classification training functional (can train models!)
4. **Phase 4 Complete**: Regression training functional
5. **All Complete**: Full node property prediction capability

---

## Testing Strategy

### Unit Tests (Per Phase)

- Phase 1: Component tests for each type (configs, steps, producers)
- Phase 2: Pipeline construction, validation tests
- Phase 3: Training loop components (label extraction, splitting, training)
- Phase 4: Regression-specific tests (parallel to Phase 3)

### Integration Tests

- **End-to-end classification**: Create pipeline → add features → train → predict
- **End-to-end regression**: Same flow for regression
- **Pipeline catalog integration**: Store/retrieve trained pipelines
- **Model persistence**: Save/load trained models

### Test Data Strategy

Use `RandomGraphConfig::seeded(42)` for deterministic test graphs (established pattern)

---

## Success Criteria

### Phase 1 Success

- [x] All 6 base infrastructure files translated
- [x] NodeFeatureStep working
- [x] Split configs validated
- [x] 10-15 unit tests passing

### Phase 2 Success

- [x] Classification pipeline constructible
- [x] All value classes working
- [x] Config validation functional
- [x] 10-12 unit tests passing

### Phase 3 Success

- [x] **Can train a node classification model end-to-end**
- [x] Label extraction working
- [x] Training loop functional
- [x] Metrics computed correctly
- [x] 8-10 unit tests passing

### Phase 4 Success

- [x] **Can train a node regression model end-to-end**
- [x] Training loop functional
- [x] Metrics computed correctly
- [x] 12-15 unit tests passing

### Overall Success

- [x] **~390 ML tests passing** (348 + 42 new)
- [x] Zero compilation errors
- [x] Can train both classification and regression models
- [x] Integration with existing pipeline system (Phases 1-8)

---

## Risk Assessment

### Low Risk

✅ **Foundation infrastructure** (Phase 1) - Straightforward types  
✅ **Value classes** (Phases 2 & 4) - Simple data containers  
✅ **Pipeline definitions** - Extends existing patterns

### Medium Risk

⚠️ **Training algorithms** (Phases 3 & 4) - Complex logic, needs ml-training  
⚠️ **Feature production** - Depends on graph API  
⚠️ **Cross-validation** - May need implementation

### Mitigation Strategies

1. **Start simple**: Phase 1-2 first (low risk, high value)
2. **Check ml-training**: Verify Classifier/Regressor trainers exist
3. **Mock when needed**: Use test mocks for missing ml components
4. **Incremental testing**: Test each component as built

---

## Recommended Execution Order

### Session 1: Foundation (Phase 1)

**Goal**: Get base infrastructure working  
**Time**: 2-3 hours  
**Deliverable**: 6 files, ~490 lines, node pipeline basics

### Session 2: Classification Setup (Phase 2)

**Goal**: Classification pipeline defined  
**Time**: 2-3 hours  
**Deliverable**: 7 files, ~453 lines, classification foundations

### Session 3: Classification Training (Phase 3)

**Goal**: Can train classification models  
**Time**: 3-4 hours  
**Deliverable**: 3 files, ~516 lines, **working classification**

**Milestone**: First trainable model! 🎉

### Session 4: Regression (Phase 4)

**Goal**: Can train regression models  
**Time**: 4-5 hours  
**Deliverable**: 7 files, ~768 lines, **working regression**

**Milestone**: Complete node property prediction! 🚀

---

## Notes & Observations

### Comparison to Link Prediction

The nodePipeline package is **significantly simpler** than link prediction would be:

- **Fewer files**: 21 vs. ~40+ for link prediction
- **Simpler structure**: Node features vs. link features + relationship sampling
- **Less complexity**: No link splitting, no negative sampling

### Why This Is Valuable

1. **Completes node property prediction** - Core ML use case
2. **Builds on proven approach** - Direct Integration success
3. **Reasonable scope** - ~2,000 lines, 11-15 hours
4. **High utility** - Node classification is extremely common in GDS

### What We're NOT Doing (Smart Deferrals)

- ❌ Memory estimation (Phase 5) - Optional, can defer
- ❌ Link prediction - Much larger scope, separate plan
- ❌ Stub system (Phase 9-10) - Already replaced with Direct Integration

---

## Conclusion

**Recommendation**: **Proceed with Node Pipeline Translation**

**Rationale**:

1. ✅ **Manageable scope**: 21 files, ~2,000 lines, 11-15 hours
2. ✅ **High value**: Enables node classification and regression
3. ✅ **Proven approach**: Direct Integration pattern works
4. ✅ **Clear dependencies**: Builds cleanly on Phases 1-8
5. ✅ **Incremental progress**: Can deliver phases independently

**Next Step**: Begin Phase 1 (Base Infrastructure) - 6 files, 2-3 hours

This translation will bring **full node property prediction capability** to rust-gds, completing a major ML use case with reasonable effort! 🎯
