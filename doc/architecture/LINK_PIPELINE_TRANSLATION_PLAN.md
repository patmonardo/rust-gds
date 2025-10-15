# LinkPipeline Translation Plan

**Document Type**: Translation Plan (TP-001)  
**Status**: ✅ Executed (Kriya Complete - see TC-001)  
**Date**: October 15, 2025  
**Goal**: Complete third pipeline type (Link Prediction) to understand full ML Pipeline architecture before Model/Catalog systems  
**Total Files**: 25 Java files (~2,800 lines)

---

## Membership Protocol - Location within Encyclopedia

**This document locates itself as follows:**

```text
rust-gds Encyclopedia of Software Translations
│
└─ Translation Plans/ (TP) ← THIS DOCUMENT RESIDES HERE
   ├─ LINK_PIPELINE_TRANSLATION_PLAN.md (TP-001) ← YOU ARE HERE
   └─ GRAPH_PROJECTION_API_TRANSLATION_PLAN.md (TP-002)
```

**Location Justification** (Fichte's Protocol):

1. **This is Translation Plan #001** - First comprehensive translation plan
2. **This is Prakasa Stage** - Illumination written before execution
3. **This has been executed** - Kriya completed (see TC-001)
4. **This established the pattern** - Template for future translation plans

**Related Documents**:

- **Produces**: `LINK_PIPELINE_TRANSLATION_COMPLETE.md` (TC-001, complete)
- **Follows pattern**: `TRANSLATION_WORKFLOW_TEMPLATE.md` (universal method)
- **Located via**: `ENCYCLOPEDIA_INDEX.md` (master index)

---

## Executive Summary

LinkPipeline represents the third and final pipeline type in GDS ML Pipeline system. Unlike Node Classification and Node Regression which predict **node properties**, Link Prediction predicts **relationship existence** between node pairs. This requires fundamentally different feature engineering (pair-based features), data splitting (relationship-based splits with negative sampling), and training strategies.

**Key Architectural Differences from Node Pipelines**:

1. **Pair-Based Features**: LinkFeatureStep operates on (source, target) pairs instead of single nodes
2. **Negative Sampling**: Training requires generating non-existent relationships for binary classification
3. **Relationship Splitting**: Train/test/validation splits operate on relationships, not nodes
4. **Link Functions**: Mathematical operations on node property pairs (Hadamard, Cosine, L2, SameCategory)
5. **Binary Classification Only**: Link prediction is always binary (exists/doesn't exist), unlike node classification (multi-class)

## File Inventory (25 files, ~2,800 lines)

### Phase 1: Core Interfaces and Types (6 files, ~350 lines)

**1.1 LinkFeatureStep.java** (35 lines)

- Interface extending `FeatureStep`
- Core method: `LinkFeatureAppender linkFeatureAppender(Graph graph)`
- Generates pair-based features instead of node features
- Pattern: Similar to NodeFeatureStep but operates on pairs

**1.2 LinkFeatureAppender.java** (44 lines)

- Interface for appending features for a link (source, target) pair
- Core method: `appendFeatures(long source, long target, double[] linkFeatures, int offset)`
- Returns `int dimension()` - feature count
- `boolean isSymmetric()` - optimization for undirected relationships
- Pattern: Pair-based feature extraction

**1.3 ExpectedSetSizes.java** (36 lines)

- Value class for split size calculations
- Fields: testSize, featureInputSize, trainSize, testComplementSize, validationFoldSize
- Used by LinkPredictionSplitConfig validation
- Simple data holder, no behavior

**1.4 FeaturesAndLabels.java** (35 lines) [train/]

- Value class pairing Features with HugeIntArray labels
- Fields: `Features features()`, `HugeIntArray labels()`
- Method: `long size()` - returns features.size()
- Pattern: Training data container

**1.5 LinkPredictionTrainResult.java** (31 lines) [train/]

- Value class with `Classifier classifier()`, `TrainingStatistics trainingStatistics()`
- Identical pattern to NodeRegressionTrainResult
- Simple result container

**1.6 LinkPredictionModelInfo.java** (65 lines)

- CustomInfo implementation for Link Prediction models
- Fields: bestParameters (TrainerConfig), metrics (Map), pipeline (LinkPredictionPredictPipeline)
- Static factory: `of(testMetrics, outerTrainMetrics, bestCandidate, pipeline)`
- Pattern: Identical to NodeRegressionPipelineModelInfo but with LinkPredictionPredictPipeline

### Phase 2: Link Functions (8 files, ~850 lines)

**2.1 HadamardFeatureStep.java** (161 lines) [linkfunctions/]

- Element-wise multiplication of node property vectors: `v1[i] * v2[i]`
- Most common link feature - captures feature interaction
- Supports: double[], float[], long[], long, double properties
- Inner class: HadamardFeatureAppenderFactory with property type dispatching

**2.2 CosineFeatureStep.java** (241 lines) [linkfunctions/]

- Cosine similarity: `dot(v1, v2) / (norm(v1) * norm(v2))`
- Measures angular similarity between vectors
- Complex: Partial L2 computation with norm caching
- Inner classes: PartialL2WithNormsComputer, CosineComputationResult

**2.3 L2FeatureStep.java** (175 lines) [linkfunctions/]

- L2 distance: `sqrt(sum((v1[i] - v2[i])^2))`
- Measures Euclidean distance between vectors
- Supports same property types as Hadamard
- Inner class: L2LinkFeatureAppenderFactory

**2.4 SameCategoryStep.java** (96 lines) [linkfunctions/]

- Binary feature: 1 if properties equal, 0 otherwise
- Works with any property type (long, double, arrays)
- Useful for categorical node properties
- Returns multiple binary features (one per input property)

**2.5 LinkFeatureStepConfiguration.java** (76 lines) [linkfunctions/]

- Configuration interface for link feature steps
- Core field: `List<String> nodeProperties()`
- Complex validation: `fromObject()` converter with detailed error messages
- Pattern: Shared config for all link function types

**2.6 AbstractLinkFeatureAppenderFactory.java** (69 lines) [linkfunctions/]

- Abstract factory for creating property-type-specific appenders
- Protected abstract methods for each property type
- Method: `createAppenders(Graph, List<String>)` - creates appender array
- Pattern: Factory with type dispatching

**2.7 SinglePropertyFeatureAppender.java** (38 lines) [linkfunctions/]

- Abstract base class for single-property appenders
- Fields: NodePropertyValues props, int dimension
- Simple base implementation of LinkFeatureAppender

**2.8 UnionLinkFeatureAppender.java** (68 lines) [linkfunctions/]

- Combines multiple LinkFeatureAppenders into one
- Manages offset calculation across appenders
- NaN validation after feature computation
- Pattern: Composite appender

### Phase 3: Feature Extraction System (3 files, ~330 lines)

**3.1 LinkFeatureStepFactory.java** (85 lines)

- Enum factory for creating LinkFeatureStep instances
- Values: HADAMARD, COSINE, L2, SAME_CATEGORY
- Pattern: Same as NodePropertyStepFactory
- Static method: `parse(String name, Map<String, Object> config)`

**3.2 LinkFeatureExtractor.java** (125 lines)

- Core feature extraction orchestrator
- Static factory: `of(Graph, List<LinkFeatureStep>)` - creates extractor
- Static method: `extractFeatures(Graph, steps, concurrency, progressTracker, terminationFlag)` - full extraction
- Parallel extraction using DegreePartition and RunWithConcurrency
- Returns Features object

**3.3 BatchLinkFeatureExtractor.java** (67 lines)

- Runnable for parallel link feature extraction
- Fields: extractor, partition, graph, relationshipOffset, linkFeatures, progressTracker
- Run method: Iterates partition, extracts features per relationship
- Pattern: Parallel extraction worker

### Phase 4: Pipeline Core (3 files, ~420 lines)

**4.1 LinkPredictionTrainingPipeline.java** (134 lines)

- Extends TrainingPipeline<LinkFeatureStep>
- Static fields: PIPELINE_TYPE = "Link prediction training pipeline", MODEL_TYPE = "LinkPrediction"
- Key difference: `LinkPredictionSplitConfig splitConfig` (not in node pipelines)
- Methods: type(), featurePipelineDescription(), additionalEntries(), splitConfig()
- Validation: Split config validation specific to link prediction

**4.2 LinkPredictionPredictPipeline.java** (83 lines)

- Implements Pipeline<LinkFeatureStep>
- Static field: EMPTY = new LinkPredictionPredictPipeline(List.of(), List.of())
- Fields: nodePropertySteps (ExecutableNodePropertyStep[]), featureSteps (LinkFeatureStep[])
- Static factories: `from(Pipeline<LinkFeatureStep>)`, `from(Stream<ExecutableNodePropertyStep>, Stream<LinkFeatureStep>)`
- Note: Does NOT extend TrainingPipeline - this is predict-only subset

**4.3 LinkPredictionSplitConfig.java** (207 lines)

- Configuration interface with complex validation
- Key fields: validationFolds (default 3), testFraction (default 0.1), trainFraction (default 0.1)
- **NEW**: negativeClassWeight, testRelationshipType, trainRelationshipType, featureInputRelationshipType
- Complex validation: relationship count validation, split size calculation
- Method: `expectedSetSizes(GraphDimensions, Collection<RelationshipType>)` - calculates expected split sizes
- ElementTypeValidator integration for relationship type validation

### Phase 5: Training System (5 files, ~1,200 lines)

**5.1 LinkPredictionTrainConfig.java** (151 lines) [train/]

- Extends TrainBaseConfig, GraphNameConfig, RandomSeedConfig
- Key field: `double negativeClassWeight()` (default 1.0)
- Fields: pipeline (String), targetRelationshipType (String), sourceNodeLabel/targetNodeLabel (optional)
- Complex validation: relationship type validation, ElementTypeValidator integration
- Internal method: `internalTargetRelationshipType()` - converts string to RelationshipType

**5.2 LinkPredictionTrain.java** (410 lines) [train/]

- **LARGEST FILE** - Core training algorithm
- Inner class: `TrainData` - holds training graph, features, labels
- Key method: `compute()` - orchestrates full training pipeline
- Training loop: RandomSearch over TrainerConfig candidates
- Cross-validation: StratifiedKFoldSplitter for validation
- Metric evaluation: ModelSpecificMetricsHandler with SignedProbabilities
- Pattern: Identical to NodeRegressionTrain but with link-specific data preparation

**5.3 LinkPredictionRelationshipSampler.java** (303 lines) [train/]

- **CRITICAL** - Handles relationship splitting and negative sampling
- Static method: `splitRelationships(...)` - creates train/test/validation splits
- Negative sampling: Generates non-existent relationships for binary classification
- Uses EdgeSplitter and NegativeSampler from ml-core
- Complex validation: Ensures minimum set sizes for all splits
- Progress tracking: Detailed tasks for each split phase

**5.4 LinkFeaturesAndLabelsExtractor.java** (144 lines) [train/]

- Static method: `extractFeaturesAndLabels(...)` - extracts features for relationship pairs
- Parallel extraction using DegreePartition
- Label assignment: POSITIVE (1) for existing, NEGATIVE (0) for non-existing
- Memory estimation: Detailed estimation for feature storage
- Pattern: Similar to node feature extraction but pair-based

**5.5 LinkPredictionTrainPipelineExecutor.java** (243 lines) [train/]

- Extends PipelineExecutor
- Inner class: `LinkPredictionTrainPipelineResult` - extends CatalogModelContainer
- Key method: `compute()` - orchestrates pipeline execution
- Steps: Node property steps → Link feature steps → Relationship sampling → Training → Model creation
- Model creation: Calls Model.of() with Classifier data
- Progress tracking: Multi-phase progress tasks

## Translation Strategy

### Order of Translation

**Week 1: Foundation (Phases 1-2, ~1,200 lines)**

1. Phase 1.1-1.6: Core interfaces and types (6 files, ~350 lines) - **Day 1-2**
2. Phase 2.1-2.4: Link functions (4 main files, ~670 lines) - **Day 3-4**
3. Phase 2.5-2.8: Link function infrastructure (4 files, ~180 lines) - **Day 5**

**Week 2: Extraction and Pipeline (Phases 3-4, ~750 lines)** 4. Phase 3.1-3.3: Feature extraction system (3 files, ~330 lines) - **Day 6-7** 5. Phase 4.1-4.3: Pipeline core (3 files, ~420 lines) - **Day 8-9**

**Week 3: Training System (Phase 5, ~1,200 lines)** 6. Phase 5.1: LinkPredictionTrainConfig (151 lines) - **Day 10** 7. Phase 5.4: LinkFeaturesAndLabelsExtractor (144 lines) - **Day 11** 8. Phase 5.3: LinkPredictionRelationshipSampler (303 lines) - **Day 12-13** 9. Phase 5.5: LinkPredictionTrainPipelineExecutor (243 lines) - **Day 14** 10. Phase 5.2: LinkPredictionTrain (410 lines) - **Day 15** (LAST, most complex)

### Key Dependencies to Stub

**From ml-core**:

- `EdgeSplitter` - Splits relationships into train/test sets
- `UndirectedEdgeSplitter` - Handles undirected relationship splitting
- `NegativeSampler` - Generates negative examples for binary classification

**From ml-models**:

- `Classifier` - Binary classifier (LogisticRegression, RandomForest, etc.)
- `ClassifierTrainer` - Trains binary classifiers
- `ClassifierTrainerFactory` - Creates classifier trainers
- `ClassifierData` - Trained classifier model data

**From ml-training**:

- `CrossValidation` - K-fold cross-validation
- `StratifiedKFoldSplitter` - Stratified K-fold splitting
- `RandomSearch` - Hyperparameter search

**From ml-metrics**:

- `LinkMetric` - Link-specific metrics (AUCPR, ROC_AUC)
- `SignedProbabilities` - Predictions with confidence scores

**From ml-splitting** (NEW):

- Split strategies specific to relationships
- Train/test/validation relationship partitioning

## Key Architectural Patterns

### 1. Pair-Based Feature Pattern

```rust
pub trait LinkFeatureAppender {
    fn append_features(&self, source: NodeId, target: NodeId, link_features: &mut [f64], offset: usize);
    fn dimension(&self) -> usize;
    fn is_symmetric(&self) -> bool { true }
}
```

**Difference from Node Features**: Operates on (source, target) pairs instead of single nodes.

### 2. Link Function Pattern

Each link function (Hadamard, Cosine, L2, SameCategory) follows this pattern:

```rust
pub struct HadamardFeatureStep {
    node_properties: Vec<String>,
}

impl LinkFeatureStep for HadamardFeatureStep {
    fn link_feature_appender(&self, graph: &Graph) -> Box<dyn LinkFeatureAppender> {
        // Create property-type-specific appenders
        // Combine into UnionLinkFeatureAppender
    }
}
```

### 3. Relationship Splitting Pattern

```rust
pub struct LinkPredictionRelationshipSampler;

impl LinkPredictionRelationshipSampler {
    pub fn split_relationships(
        graph_store: &GraphStore,
        config: &LinkPredictionSplitConfig,
        // ... other params
    ) -> SplitResult {
        // 1. Split existing relationships into train/test/validation
        // 2. Generate negative samples for each split
        // 3. Validate minimum set sizes
        // 4. Return training-ready relationship sets
    }
}
```

**Key Difference**: Node pipelines split nodes; link pipelines split relationships AND generate negatives.

### 4. Training Pipeline Pattern

```rust
pub struct LinkPredictionTrain {
    pipeline: LinkPredictionTrainingPipeline,
    config: LinkPredictionTrainConfig,
    // ...
}

impl LinkPredictionTrain {
    pub fn compute(&self) -> LinkPredictionTrainResult {
        // 1. Execute node property steps
        // 2. Split relationships + negative sampling
        // 3. Extract link features for train/test/validation sets
        // 4. RandomSearch over TrainerConfig candidates
        // 5. Cross-validation for each candidate
        // 6. Select best model
        // 7. Return Classifier + TrainingStatistics
    }
}
```

## Critical Design Questions

### 1. How to Generalize Pipeline<F: FeatureStep>?

**Current Node Pipeline Pattern**:

```rust
pub struct NodeClassificationTrainingPipeline {
    feature_steps: Vec<NodePropertyStep>,  // Node-specific
    // ...
}
```

**Link Pipeline Pattern**:

```rust
pub struct LinkPredictionTrainingPipeline {
    feature_steps: Vec<LinkFeatureStep>,  // Link-specific
    // ...
}
```

**Options**:

- **A. Keep Specialized**: Maintain separate NodePropertyStep and LinkFeatureStep types
  - Pro: Type safety, no trait object overhead
  - Con: Some code duplication in pipeline infrastructure
- **B. Generalize with Trait**: `Pipeline<F: FeatureStep>`
  - Pro: Shared pipeline infrastructure
  - Con: Trait object complexity, harder to understand

**Recommendation**: **Option A** (Keep Specialized)

- Type safety is more valuable than abstraction here
- Node and Link features have fundamentally different semantics
- Duplication is minimal and localized

### 2. How to Handle Negative Sampling?

**Java Approach**: Integrated into LinkPredictionRelationshipSampler
**Rust Options**:

- **A. Same as Java**: Integrate into sampler
- **B. Separate Service**: Create NegativeSamplingService
- **C. Strategy Pattern**: Make negative sampling strategy pluggable

**Recommendation**: **Option A** (Same as Java)

- Negative sampling is integral to link prediction splitting
- No other pipeline types need it
- Matches Java architecture for easier translation

### 3. Should LinkFeatureAppender be Trait Object or Generic?

**Options**:

- **A. Trait Object**: `Box<dyn LinkFeatureAppender>`
  - Pro: Easy to store heterogeneous appenders
  - Con: Virtual dispatch overhead
- **B. Enum Dispatch**: `enum LinkFeatureAppender { Hadamard(...), Cosine(...), ... }`
  - Pro: No virtual dispatch, better performance
  - Con: Closed set of appenders, harder to extend

**Recommendation**: **Option A** (Trait Object)

- Flexibility for future extension
- Performance not critical (feature extraction is I/O bound)
- Matches Java architecture

## Comparison: Node vs Link Pipelines

| Aspect                 | Node Pipeline                          | Link Pipeline                                     |
| ---------------------- | -------------------------------------- | ------------------------------------------------- |
| **Prediction Target**  | Node property                          | Relationship existence                            |
| **Feature Extraction** | Single node properties                 | Pair-based node property combinations             |
| **Data Splitting**     | Nodes into train/test                  | Relationships into train/test + negative sampling |
| **Feature Step Type**  | NodePropertyStep                       | LinkFeatureStep                                   |
| **Training Type**      | Classification or Regression           | Classification only (binary)                      |
| **Split Config**       | Simple train/test/validation fractions | Complex with negative sampling ratios             |
| **Model Type**         | Classifier or Regressor                | Classifier only                                   |
| **Metrics**            | Class-specific or regression           | Link-specific (AUCPR, ROC_AUC)                    |

## Success Criteria

1. ✅ All 25 LinkPipeline files translated to Rust
2. ✅ Zero compilation errors (with placeholders for ml-core/ml-models/ml-training)
3. ✅ Unit tests for each file (basic construction and method calls)
4. ✅ Design document comparing all three pipeline types
5. ✅ Clear identification of shared vs specialized patterns
6. ✅ Model API requirements updated with Link Prediction specifics

## Next Steps After LinkPipeline

1. **Create Pipeline Comparison Document**

   - Compare Node Classification, Node Regression, Link Prediction
   - Identify generalizable patterns
   - Recommend shared trait hierarchy

2. **Update Model Requirements**

   - Add ClassifierData requirements
   - Document Link Prediction ModelInfo structure
   - Update Model::of() API documentation

3. **Plan Model System Translation**

   - Model<DATA, CONFIG, INFO> core structure
   - ModelConfig/CustomInfo/ResultToModelConverter traits
   - Model data types: RegressorData, ClassifierData, LinkPredictorData (if different)

4. **Plan Catalog Systems**
   - PipelineCatalog (simpler, user-scoped)
   - ModelCatalog (complex, cross-user, persistence)
   - GraphStoreCatalog (may defer until after Procedures)

## Estimated Effort

- **Phase 1 (Core Types)**: 2 days (~350 lines, 6 files)
- **Phase 2 (Link Functions)**: 3 days (~850 lines, 8 files)
- **Phase 3 (Extraction)**: 2 days (~330 lines, 3 files)
- **Phase 4 (Pipeline Core)**: 2 days (~420 lines, 3 files)
- **Phase 5 (Training)**: 6 days (~1,200 lines, 5 files)
- **Testing & Documentation**: 2 days

**Total**: ~3 weeks (15 work days) for complete LinkPipeline translation

## Risk Assessment

**Low Risk**:

- Core types (Phase 1) - Simple value classes and interfaces
- Link functions (Phase 2) - Well-defined mathematical operations
- Feature extraction (Phase 3) - Parallel to node feature extraction

**Medium Risk**:

- Pipeline core (Phase 4) - Split config validation is complex
- LinkPredictionTrainPipelineExecutor (Phase 5.5) - Model integration

**High Risk**:

- LinkPredictionRelationshipSampler (Phase 5.3) - Negative sampling integration with ml-core
- LinkPredictionTrain (Phase 5.2) - Large file with many dependencies

**Mitigation**:

- Use placeholder types for ml-core dependencies (EdgeSplitter, NegativeSampler)
- Translate smaller files first to establish patterns
- Defer LinkPredictionTrain until end (like NodeRegressionTrain)
- Test each phase before moving to next

---

**Ready to Execute**: This plan provides complete roadmap for LinkPipeline translation. Begin with Phase 1 (Core Types) to establish foundation.
