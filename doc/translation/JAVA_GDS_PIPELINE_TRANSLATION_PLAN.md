# Java GDS Pipeline - Complete Translation Plan

**Date**: October 14, 2025  
**Status**: Planning Phase - Complete Package Analysis  
**Approach**: 1:1 translation with priority ordering

---

## Translation Status Overview

### ✅ Phase 1: Core Traits (COMPLETE - Session 1)

| Java File                         | Rust File                          | Lines | Status      | Tests |
| --------------------------------- | ---------------------------------- | ----- | ----------- | ----- |
| `FeatureStep.java`                | `feature_step.rs`                  | 40    | ✅ Complete | -     |
| `ExecutableNodePropertyStep.java` | `executable_node_property_step.rs` | 104   | ✅ Complete | -     |
| `Pipeline.java`                   | `pipeline_trait.rs`                | 195   | ✅ Complete | 1     |

**Subtotal**: 3 files, 339 lines, 75 tests passing (18 pipeline + 57 other ML)

---

## 📋 Complete Translation Plan (31 Files Total)

### Phase 2: Foundation Types (Priority 1 - Next Session)

These are simple data structures needed by executors.

#### 2.1 Value Objects

**File**: `PipelineGraphFilter.java` → `pipeline_graph_filter.rs`  
**Complexity**: ⭐ Simple  
**Lines**: ~30  
**Purpose**: Filter struct for dataset splits (node labels + relationship types)  
**Dependencies**: None  
**Java**:

```java
@ValueClass
public interface PipelineGraphFilter {
    Collection<NodeLabel> nodeLabels();
    @Value.Default
    default Collection<RelationshipType> relationshipTypes() { return List.of(); }
}
```

**Rust Strategy**: Simple struct with builder pattern

```rust
pub struct PipelineGraphFilter {
    pub node_labels: Vec<String>,
    pub relationship_types: Vec<String>,
}
```

#### 2.2 Configuration

**File**: `AutoTuningConfig.java` → `auto_tuning_config.rs`  
**Complexity**: ⭐ Simple  
**Lines**: ~50  
**Purpose**: Hyperparameter tuning configuration (max trials)  
**Dependencies**: None  
**Java**:

```java
@Configuration
public interface AutoTuningConfig extends ToMapConvertible {
    int MAX_TRIALS = 10;
    @Configuration.IntegerRange(min = 1)
    default int maxTrials() { return MAX_TRIALS; }
}
```

**Rust Strategy**: Struct with validation

```rust
pub struct AutoTuningConfig {
    max_trials: usize,
}
impl AutoTuningConfig {
    pub fn new(max_trials: usize) -> Result<Self, ValidationError> {
        if max_trials < 1 {
            return Err(ValidationError::InvalidMaxTrials);
        }
        Ok(Self { max_trials })
    }
}
```

**File**: `NodePropertyStepContextConfig.java` → `node_property_step_context_config.rs`  
**Complexity**: ⭐ Simple  
**Lines**: ~40  
**Purpose**: Context config for node property steps (context labels/types)  
**Dependencies**: None  
**Rust Strategy**: Simple struct with defaults

**CORRECTION**: DatasetSplits is a nested enum inside `PipelineExecutor.java` (not a separate file). It will be translated as part of `pipeline_executor_trait.rs` in Phase 6.

**Phase 2 Subtotal**: 4 files, ~140 lines, simple data structures

---

### Phase 3: Utility Functions (Priority 2)

**File**: `FeatureStepUtil.java` → `feature_step_util.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~80  
**Purpose**: Utility functions for feature computation  
**Dependencies**: ValueType (from types module)  
**Key Functions**:

- `property_dimension()` - Compute feature dimensions from property type
- `validate_computed_features()` - NaN checking for link features
- `throw_nan_error()` - Error formatting for NaN features

**Java**:

```java
public static int propertyDimension(NodePropertyValues nodeProperties, String propertyName) {
    int dimension = 0;
    switch (nodeProperties.valueType()) {
        case LONG:
        case DOUBLE:
            dimension = 1;
            break;
        case DOUBLE_ARRAY:
        case FLOAT_ARRAY:
            dimension = nodeProperties.doubleArrayValue(0).length;
            break;
        // ...
    }
    return dimension;
}
```

**Rust Strategy**: Module with public functions

```rust
pub fn property_dimension(
    node_properties: &dyn NodePropertyValues,
    property_name: &str,
) -> Result<usize, FeatureError> {
    match node_properties.value_type() {
        ValueType::Long | ValueType::Double => Ok(1),
        ValueType::DoubleArray | ValueType::FloatArray => {
            let arr = node_properties.double_array_value(0)?;
            Ok(arr.len())
        }
        // ...
    }
}
```

**File**: `NonEmptySetValidation.java` → `non_empty_set_validation.rs`  
**Complexity**: ⭐ Simple  
**Lines**: ~60  
**Purpose**: Validation for minimum dataset sizes  
**Key Constants**:

- MIN_SET_SIZE = 1
- MIN_TRAIN_SET_SIZE = 2
- MIN_TEST_COMPLEMENT_SET_SIZE = 3

**File**: `PipelineCompanion.java` → `pipeline_companion.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~80  
**Purpose**: Helper functions for pipeline config and validation  
**Key Functions**:

- `prepare_pipeline_config()` - Graph name handling
- `configure_auto_tuning()` - Auto-tuning setup
- `validate_main_metric()` - Metric validation for Random Forest

**Phase 3 Subtotal**: 3 files, ~220 lines, utility logic

---

### Phase 4: Concrete Step Implementations (Priority 3 - Critical)

#### 4.1 Node Property Step

**File**: `NodePropertyStep.java` → `node_property_step.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~150  
**Purpose**: Concrete implementation of ExecutableNodePropertyStep  
**Dependencies**: ExecutableNodePropertyStep trait, GdsCallableFinder (procedure registry)  
**Key Components**:

- Wraps graph algorithm procedure calls (PageRank, FastRP, Louvain, etc.)
- Stores procedure name, config, context labels/types
- Implements execute() to call actual GDS procedures

**Java Structure**:

```java
public final class NodePropertyStep implements ExecutableNodePropertyStep {
    private final GdsCallableFinder.GdsCallableDefinition callableDefinition;
    private final Map<String, Object> config;
    private final List<String> contextNodeLabels;
    private final List<String> contextRelationshipTypes;

    public NodePropertyStep(
        GdsCallableFinder.GdsCallableDefinition callableDefinition,
        Map<String, Object> config,
        List<String> contextNodeLabels,
        List<String> contextRelationshipTypes
    ) { /* ... */ }

    @Override
    public void execute(ExecutionContext executionContext, ..., Stub stub) {
        // Call procedure via stub
    }
}
```

**Rust Strategy**: Struct implementing ExecutableNodePropertyStep trait

```rust
pub struct NodePropertyStep {
    proc_name: String,
    config: HashMap<String, serde_json::Value>,
    context_node_labels: Vec<String>,
    context_relationship_types: Vec<String>,
    mutate_property: String,
}

impl ExecutableNodePropertyStep for NodePropertyStep {
    fn execute(&self, graph_store: &mut DefaultGraphStore, ...) -> Result<(), Box<dyn StdError>> {
        // Call graph algorithm (PageRank, FastRP, etc.)
        // This will require procedure registry/factory pattern
    }
    // ... other trait methods
}
```

**Challenge**: Java uses `Stub` interface for dependency injection to call procedures. Rust will need:

- Procedure registry (map of proc name → algorithm factory)
- Factory pattern to create and execute algorithms
- Integration with rust-gds algorithm implementations

#### 4.2 Stub-Powered Step (Optional - if needed)

**File**: `StubPoweredNodePropertyStep.java` → `stub_powered_node_property_step.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~150  
**Purpose**: Alternative implementation using new stub system  
**Decision**: May not need separate implementation if we design NodePropertyStep well

#### 4.3 Node Property Step Factory

**File**: `NodePropertyStepFactory.java` → `node_property_step_factory.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~170  
**Purpose**: Factory for creating NodePropertyStep instances  
**Key Functions**:

- `create_node_property_step()` - Main factory method
- Validation of procedure names
- Config parsing and normalization

**Rust Strategy**: Factory struct with static methods

```rust
pub struct NodePropertyStepFactory;

impl NodePropertyStepFactory {
    pub fn create_node_property_step(
        task_name: &str,
        config_map: HashMap<String, serde_json::Value>,
    ) -> Result<NodePropertyStep, StepCreationError> {
        // Parse task name → proc name
        // Validate config
        // Create NodePropertyStep
    }
}
```

**File**: `NodePropertyStepFactoryUsingStubs.java` → (May skip - internal Java impl detail)  
**Complexity**: ⭐⭐⭐⭐ Very Complex  
**Decision**: This is Java's internal singleton for new stub system. We may not need if we design our own factory well.

**Phase 4 Subtotal**: 2-3 files, ~320-470 lines, critical for pipeline execution

---

### Phase 5: Step Execution Infrastructure (Priority 4)

**File**: `NodePropertyStepExecutor.java` → `node_property_step_executor.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~180  
**Purpose**: Executor for list of node property steps  
**Dependencies**: NodePropertyStep, ProgressTracker  
**Key Methods**:

- `validate_node_property_steps_context_configs()` - Validate step configs
- `execute_node_property_steps()` - Execute all steps in sequence
- `cleanup_intermediate_properties()` - Remove temp properties
- `estimate_node_property_steps()` - Memory estimation

**Java Structure**:

```java
public class NodePropertyStepExecutor<PIPELINE_CONFIG> {
    private final ExecutionContext executionContext;
    private final GraphStore graphStore;
    private final Collection<NodeLabel> nodeLabels;
    private final Collection<RelationshipType> relTypes;

    public void executeNodePropertySteps(List<ExecutableNodePropertyStep> steps) {
        progressTracker.beginSubTask("NodePropertySteps");
        for (var step : steps) {
            var expandedNodeLabels = step.featureInputNodeLabels(graphStore, nodeLabels);
            var expandedRelTypes = step.featureInputRelationshipTypes(graphStore, relTypes, ...);
            step.execute(executionContext, graphName, expandedNodeLabels, expandedRelTypes, ...);
        }
        progressTracker.endSubTask("NodePropertySteps");
    }
}
```

**Rust Strategy**: Struct with execution methods

```rust
pub struct NodePropertyStepExecutor {
    graph_store: Arc<Mutex<DefaultGraphStore>>,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
    available_rel_types: HashSet<String>,
}

impl NodePropertyStepExecutor {
    pub fn execute_node_property_steps(
        &mut self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), ExecutionError> {
        for step in steps {
            // Execute step with expanded context
            step.execute(&mut self.graph_store.lock().unwrap(), ...)?;
        }
        Ok(())
    }
}
```

**Phase 5 Subtotal**: 1 file, ~180 lines, execution infrastructure

---

### Phase 6: Pipeline Executors (Priority 5 - Core Orchestration)

#### 6.1 Base Pipeline Executor

**File**: `PipelineExecutor.java` → `pipeline_executor_trait.rs`  
**Complexity**: ⭐⭐⭐⭐ Very Complex  
**Lines**: ~120 (Java abstract class) + DatasetSplits enum (~20 lines)  
**Purpose**: Abstract base executor with template method pattern  
**Dependencies**: Pipeline trait, NodePropertyStepExecutor, PipelineGraphFilter

**Note**: This file also includes the `DatasetSplits` nested enum from the Java source (TRAIN, TEST, TEST_COMPLEMENT, FEATURE_INPUT).

**Java Structure**:

```java
public abstract class PipelineExecutor<
    PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
    PIPELINE extends Pipeline<?>,
    RESULT
> extends Algorithm<RESULT> {

    // Abstract methods for subclasses
    public abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters();
    public abstract void splitDatasets();
    protected abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits);
    protected abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps();

    // Template method
    @Override
    public RESULT compute() {
        progressTracker.beginSubTask();

        var dataSplitGraphFilters = generateDatasetSplitGraphFilters();
        var featureInputGraphFilter = dataSplitGraphFilters.get(DatasetSplits.FEATURE_INPUT);

        pipeline.validateBeforeExecution(graphStore, featureInputGraphFilter.nodeLabels());

        var nodePropertyStepExecutor = NodePropertyStepExecutor.of(...);
        nodePropertyStepExecutor.validNodePropertyStepsContextConfigs(pipeline.nodePropertySteps());

        splitDatasets();

        try {
            nodePropertyStepExecutor.executeNodePropertySteps(pipeline.nodePropertySteps());
            pipeline.validateFeatureProperties(graphStore, config.nodeLabelIdentifiers(graphStore));
            return execute(dataSplitGraphFilters);
        } finally {
            // cleanup
        }
    }
}
```

**Rust Strategy**: Trait with default template method implementation

```rust
pub trait PipelineExecutor {
    type Config;
    type Pipeline: Pipeline;
    type Result;

    // Abstract methods for implementations
    fn generate_dataset_split_graph_filters(&self) -> HashMap<DatasetSplits, PipelineGraphFilter>;
    fn split_datasets(&mut self) -> Result<(), ExecutionError>;
    fn execute(&self, data_splits: &HashMap<DatasetSplits, PipelineGraphFilter>) -> Result<Self::Result, ExecutionError>;
    fn get_available_rel_types_for_node_property_steps(&self) -> HashSet<String>;

    // Template method with default implementation
    fn compute(&mut self) -> Result<Self::Result, ExecutionError> {
        let data_split_graph_filters = self.generate_dataset_split_graph_filters();
        let feature_input_filter = data_split_graph_filters.get(&DatasetSplits::FeatureInput)
            .ok_or(ExecutionError::MissingFeatureInputFilter)?;

        // Validate pipeline
        self.pipeline().validate_before_execution(
            &self.graph_store(),
            &feature_input_filter.node_labels,
        )?;

        // Create and execute node property steps
        let mut executor = NodePropertyStepExecutor::new(...);
        executor.execute_node_property_steps(self.pipeline().node_property_steps())?;

        // Validate feature properties
        self.pipeline().validate_feature_properties(&self.graph_store(), ...)?;

        // Split datasets
        self.split_datasets()?;

        // Execute pipeline-specific logic
        let result = self.execute(&data_split_graph_filters)?;

        // Cleanup
        self.cleanup()?;

        Ok(result)
    }

    // Helper methods
    fn pipeline(&self) -> &Self::Pipeline;
    fn graph_store(&self) -> &DefaultGraphStore;
    fn cleanup(&mut self) -> Result<(), ExecutionError> { Ok(()) }
}
```

**Challenge**: Java uses inheritance (abstract class). Rust will use trait with default methods for template method pattern.

#### 6.2 Predict Pipeline Executor

**File**: `PredictPipelineExecutor.java` → `predict_pipeline_executor_trait.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~90  
**Purpose**: Base executor for prediction (no training, just inference)  
**Difference from PipelineExecutor**: No dataset splitting, simpler flow

**Java Structure**:

```java
public abstract class PredictPipelineExecutor<
    PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
    PIPELINE extends Pipeline<?>,
    RESULT
> extends Algorithm<RESULT> {

    protected abstract RESULT execute();
    protected abstract PipelineGraphFilter nodePropertyStepFilter();

    @Override
    public RESULT compute() {
        progressTracker.beginSubTask();
        PipelineGraphFilter nodePropertyStepFilter = nodePropertyStepFilter();
        pipeline.validateBeforeExecution(graphStore, nodePropertyStepFilter.nodeLabels());

        var nodePropertyStepExecutor = NodePropertyStepExecutor.of(...);
        nodePropertyStepExecutor.executeNodePropertySteps(pipeline.nodePropertySteps());
        pipeline.validateFeatureProperties(graphStore, ...);

        return execute();
    }
}
```

**Rust Strategy**: Similar trait pattern to PipelineExecutor but simpler

```rust
pub trait PredictPipelineExecutor {
    type Config;
    type Pipeline: Pipeline;
    type Result;

    fn execute(&self) -> Result<Self::Result, ExecutionError>;
    fn node_property_step_filter(&self) -> PipelineGraphFilter;

    fn compute(&mut self) -> Result<Self::Result, ExecutionError> {
        let filter = self.node_property_step_filter();
        self.pipeline().validate_before_execution(&self.graph_store(), &filter.node_labels)?;

        let mut executor = NodePropertyStepExecutor::new(...);
        executor.execute_node_property_steps(self.pipeline().node_property_steps())?;

        self.pipeline().validate_feature_properties(&self.graph_store(), ...)?;

        self.execute()
    }

    fn pipeline(&self) -> &Self::Pipeline;
    fn graph_store(&self) -> &DefaultGraphStore;
}
```

**Phase 6 Subtotal**: 2 files, ~210 lines, core orchestration traits

---

### Phase 7: Training Infrastructure (Priority 6)

**File**: `PipelineTrainer.java` → `pipeline_trainer_trait.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~30  
**Purpose**: Simple trait for training execution  
**Java**:

```java
public interface PipelineTrainer<RESULT> {
    void setTerminationFlag(TerminationFlag terminationFlag);
    RESULT run();
}
```

**Rust Strategy**: Simple trait

```rust
pub trait PipelineTrainer {
    type Result;
    fn set_termination_flag(&mut self, flag: TerminationFlag);
    fn run(&mut self) -> Result<Self::Result, TrainingError>;
}
```

**File**: `PipelineTrainAlgorithm.java` → `pipeline_train_algorithm.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~70  
**Purpose**: Abstract algorithm for pipeline training  
**Dependencies**: PipelineTrainer, Pipeline, ResultToModelConverter

**File**: `ResultToModelConverter.java` → `result_to_model_converter_trait.rs`  
**Complexity**: ⭐ Simple  
**Lines**: ~30  
**Purpose**: Trait for converting training results to model catalog entries  
**Java**:

```java
public interface ResultToModelConverter<MODEL, RESULT> {
    MODEL toModel(RESULT result, GraphSchema originalSchema);
}
```

**Rust Strategy**: Simple trait

```rust
pub trait ResultToModelConverter {
    type Model;
    type Result;
    fn to_model(&self, result: Self::Result, original_schema: GraphSchema) -> Self::Model;
}
```

**File**: `TrainingPipeline.java` → `training_pipeline_trait.rs`  
**Complexity**: ⭐⭐⭐⭐ Very Complex  
**Lines**: ~170  
**Purpose**: Base class for training pipelines with hyperparameter space  
**Key Components**:

- Extends Pipeline trait
- Manages training parameter space (map of TrainingMethod → configs)
- Auto-tuning configuration
- Training parameter validation

**Phase 7 Subtotal**: 4 files, ~300 lines, training infrastructure

---

### Phase 8: Catalog and Registry (Priority 7)

**File**: `PipelineCatalog.java` → `pipeline_catalog.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~140  
**Purpose**: Global registry for named pipelines (per-user)  
**Key Methods**:

- `set()` - Register pipeline
- `get()` - Retrieve pipeline
- `exists()` - Check existence
- `drop()` - Remove pipeline
- `get_all_pipelines()` - List user's pipelines

**Java Structure**:

```java
public final class PipelineCatalog {
    private static final ConcurrentHashMap<String, PipelineUserCatalog> userCatalogs = new ConcurrentHashMap<>();

    public static void set(String user, String pipelineName, TrainingPipeline<?> pipeline) { ... }
    public static TrainingPipeline<?> get(String user, String pipelineName) { ... }
    public static <PIPELINE extends TrainingPipeline<?>> PIPELINE getTyped(String user, String pipelineName, Class<PIPELINE> expectedClass) { ... }
}
```

**Rust Strategy**: Singleton with thread-safe storage

```rust
lazy_static! {
    static ref PIPELINE_CATALOG: Mutex<HashMap<String, HashMap<String, Box<dyn TrainingPipeline>>>> =
        Mutex::new(HashMap::new());
}

pub struct PipelineCatalog;

impl PipelineCatalog {
    pub fn set(user: &str, pipeline_name: &str, pipeline: Box<dyn TrainingPipeline>) {
        let mut catalog = PIPELINE_CATALOG.lock().unwrap();
        catalog.entry(user.to_string())
            .or_insert_with(HashMap::new)
            .insert(pipeline_name.to_string(), pipeline);
    }

    pub fn get(user: &str, pipeline_name: &str) -> Option<Box<dyn TrainingPipeline>> { ... }
}
```

**Phase 8 Subtotal**: 1 file, ~140 lines, registry infrastructure

---

### Phase 9: Stub System (Priority 8 - Complex)

#### 9.1 Stub Interface

**File**: `Stub.java` → `stub_trait.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~40  
**Purpose**: Interface for procedure execution (dependency injection)  
**Java**:

```java
public interface Stub {
    MemoryEstimation getMemoryEstimation(AlgorithmsProcedureFacade facade, String username, Map<String, Object> configuration);
    void execute(AlgorithmsProcedureFacade facade, String graphName, Map<String, Object> configuration);
}
```

**Rust Strategy**: Trait for algorithm execution

```rust
pub trait Stub {
    fn get_memory_estimation(
        &self,
        username: &str,
        configuration: &HashMap<String, serde_json::Value>,
    ) -> Result<MemoryEstimation, MemoryEstimationError>;

    fn execute(
        &self,
        graph_store: &mut DefaultGraphStore,
        configuration: &HashMap<String, serde_json::Value>,
    ) -> Result<(), ExecutionError>;
}
```

#### 9.2 Stub Implementations

**File**: `StubbyHolder.java` → `stub_registry.rs`  
**Complexity**: ⭐⭐⭐⭐ Very Complex  
**Lines**: ~150 (+ 40+ stub implementations)  
**Purpose**: Registry mapping algorithm names to stub implementations  
**Dependencies**: All stub implementations (PageRankStub, FastRPStub, LouvainStub, etc.)

**Decision**: This is a massive file (40+ stub implementations in Java). We have two options:

**Option A**: Implement full stub system

- Translate all 40+ stub implementations
- Create registry matching Java
- Pros: Complete parity
- Cons: 1500+ lines of boilerplate

**Option B**: Direct algorithm integration

- Skip stub abstraction
- NodePropertyStep directly calls rust-gds algorithms
- Pros: Simpler, more Rusty
- Cons: Different from Java architecture

**Recommendation**: Start with Option B (direct integration), add stub system later if needed for extensibility.

**Phase 9 Subtotal**: 1-2 files, ~40-190 lines (or skip stubs entirely)

---

### Phase 10: Algorithm Support (Priority 9 - Infrastructure)

**File**: `CanonicalProcedureName.java` → `canonical_procedure_name.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~70  
**Purpose**: Normalize procedure names (gds.pageRank.mutate variants)  
**Java**:

```java
public static CanonicalProcedureName parse(String rawInput) {
    var normalisedInput = rawInput.toLowerCase(Locale.ROOT);
    normalisedInput = !normalisedInput.startsWith("gds.") ? formatWithLocale("gds.%s", normalisedInput) : normalisedInput;
    normalisedInput = !normalisedInput.endsWith(".mutate") ? formatWithLocale("%s.mutate", normalisedInput) : normalisedInput;
    // Remove .mutate suffix
    return new CanonicalProcedureName(normalisedInput, rawInput);
}
```

**File**: `MutateModeAlgorithmLibrary.java` → `mutate_mode_algorithm_library.rs`  
**Complexity**: ⭐⭐⭐ Complex  
**Lines**: ~120  
**Purpose**: Map canonical procedure names to Algorithm enum values  
**Dependencies**: Algorithm enum (metadata), CanonicalProcedureName

**File**: `ConfigurationParsersForMutateMode.java` → `configuration_parsers_for_mutate_mode.rs`  
**Complexity**: ⭐⭐⭐⭐ Very Complex  
**Lines**: ~150  
**Purpose**: Map Algorithm enum to config parsers  
**Decision**: This is Java-specific (reflects class structure). In Rust, we'd use match on Algorithm enum.

**File**: `ValidationService.java` → `validation_service.rs`  
**Complexity**: ⭐⭐ Medium  
**Lines**: ~70  
**Purpose**: Validate algorithm configurations  
**Dependencies**: ConfigurationParsersForMutateMode

**Phase 10 Subtotal**: 3-4 files, ~310-380 lines, algorithm registry infrastructure

---

## 📊 Translation Priority Matrix

### Must-Have (Blocks Everything Else)

1. ✅ **Phase 1: Core Traits** - DONE
2. **Phase 2: Foundation Types** (PipelineGraphFilter, configs) - CORRECTED: removed DatasetSplits (nested enum)
3. **Phase 4: NodePropertyStep** (concrete step implementation)
4. **Phase 5: NodePropertyStepExecutor** (step execution)

### Core Functionality (Enables Basic Pipelines)

5. **Phase 6: PipelineExecutor** (orchestration)
6. **Phase 3: Utility Functions** (feature utils, validation)

### Training Support (Enables Full ML)

7. **Phase 7: Training Infrastructure** (PipelineTrainer, TrainingPipeline)
8. **Phase 8: Catalog** (pipeline registry)

### Optional/Advanced

9. **Phase 10: Algorithm Support** (procedure name parsing, algorithm library)
10. **Phase 9: Stub System** (only if needed for extensibility)

---

## 🎯 Recommended Translation Sequence

### Session 2: Foundation (Priority 1)

**Goal**: Simple data structures needed by executors

**CORRECTED**: Removed dataset_splits.rs (nested enum in PipelineExecutor, not separate file)

1. `pipeline_graph_filter.rs` (30 lines) - Struct
2. `auto_tuning_config.rs` (50 lines) - Config struct
3. `node_property_step_context_config.rs` (40 lines) - Config struct

**Deliverable**: 3 files, ~120 lines, foundation types complete

### Session 3: Utilities (Priority 2)

**Goal**: Helper functions for validation and feature computation

1. `feature_step_util.rs` (80 lines) - Feature utilities
2. `non_empty_set_validation.rs` (60 lines) - Validation
3. `pipeline_companion.rs` (80 lines) - Pipeline helpers

**Deliverable**: 3 files, ~220 lines, utility functions complete

### Session 4: Concrete Steps (Priority 3 - Critical)

**Goal**: Implement actual executable steps

**Challenge**: Requires algorithm integration strategy decision

**Option A: Stub-based (matches Java)**

1. `stub_trait.rs` (40 lines)
2. Design stub implementations for key algorithms
3. `node_property_step.rs` (150 lines)
4. `node_property_step_factory.rs` (170 lines)

**Option B: Direct integration (simpler)**

1. Design algorithm registry/factory
2. `node_property_step.rs` (150 lines) - calls algorithms directly
3. `node_property_step_factory.rs` (170 lines)

**Deliverable**: 2-4 files, ~360-530 lines, executable steps working

**Decision Point**: User must decide stub vs direct integration approach

### Session 5: Step Execution (Priority 4)

**Goal**: Execute lists of steps

1. `node_property_step_executor.rs` (180 lines)

**Deliverable**: 1 file, ~180 lines, step execution infrastructure

### Session 6: Pipeline Executors (Priority 5)

**Goal**: Core orchestration logic

1. `pipeline_executor_trait.rs` (120 lines) - Template method pattern
2. `predict_pipeline_executor_trait.rs` (90 lines) - Prediction executor

**Deliverable**: 2 files, ~210 lines, orchestration complete

**Milestone**: Basic pipeline execution working end-to-end

### Session 7: Training Infrastructure (Priority 6)

**Goal**: Enable ML training

1. `pipeline_trainer_trait.rs` (30 lines)
2. `result_to_model_converter_trait.rs` (30 lines)
3. `training_pipeline_trait.rs` (170 lines)
4. `pipeline_train_algorithm.rs` (70 lines)

**Deliverable**: 4 files, ~300 lines, training infrastructure complete

### Session 8: Catalog (Priority 7)

**Goal**: Pipeline registry

1. `pipeline_catalog.rs` (140 lines)

**Deliverable**: 1 file, ~140 lines, registry complete

**Milestone**: Complete pipeline system functional

### Session 9+: Advanced Features (Optional)

- Canonical procedure name parsing
- Algorithm library
- Full stub system (if needed)
- Hyperparameter optimization
- Advanced validation

---

## 📈 Complexity Analysis

### By Complexity Level

| Level                 | Files  | Total Lines | Description                      |
| --------------------- | ------ | ----------- | -------------------------------- |
| ⭐ Simple             | 8      | ~340        | Data structures, simple traits   |
| ⭐⭐ Medium           | 7      | ~500        | Utilities, configs, simple logic |
| ⭐⭐⭐ Complex        | 9      | ~1,350      | Executors, factories, registries |
| ⭐⭐⭐⭐ Very Complex | 7      | ~1,110      | Training, stub system, parsers   |
| **Total**             | **31** | **~3,300**  | **Complete package**             |

### By Category

| Category              | Files | Lines   | Priority |
| --------------------- | ----- | ------- | -------- |
| ✅ Core Traits (Done) | 3     | 339     | P0       |
| Foundation Types      | 4     | 140     | P1       |
| Utilities             | 3     | 220     | P2       |
| Concrete Steps        | 2-4   | 360-530 | P3       |
| Step Execution        | 1     | 180     | P4       |
| Pipeline Executors    | 2     | 210     | P5       |
| Training              | 4     | 300     | P6       |
| Catalog               | 1     | 140     | P7       |
| Algorithm Support     | 3-4   | 310-380 | P8       |
| Stub System           | 1-2   | 40-190  | P9       |

---

## 🔑 Key Design Decisions

### Decision 1: Stub System vs Direct Integration

**Java Approach**: Uses Stub interface for dependency injection

- Pros: Extensible, testable with mocks
- Cons: Extra abstraction layer, 40+ stub implementations

**Rust Options**:

**A) Full Stub System (Java parity)**

```rust
pub trait Stub {
    fn execute(&self, graph_store: &mut GraphStore, config: &Config) -> Result<()>;
}

pub struct PageRankStub;
impl Stub for PageRankStub { /* ... */ }
// + 40 more stubs
```

**B) Direct Integration (simpler)**

```rust
pub struct NodePropertyStep {
    algorithm: AlgorithmType,
    config: HashMap<String, Value>,
}

impl NodePropertyStep {
    fn execute(&self, graph_store: &mut GraphStore) -> Result<()> {
        match self.algorithm {
            AlgorithmType::PageRank => {
                let config = PageRankConfig::from_map(&self.config)?;
                let result = page_rank(graph_store, config)?;
                graph_store.add_node_property(&self.mutate_property, result);
            }
            // ... other algorithms
        }
    }
}
```

**Recommendation**: Start with **Option B** (direct integration)

- Simpler implementation
- More Rust-idiomatic
- Can add stub abstraction later if needed
- Saves ~1,500 lines of boilerplate

### Decision 2: Pipeline Executor Pattern

**Java**: Abstract class with template method
**Rust**: Trait with default methods

```rust
pub trait PipelineExecutor {
    // Abstract methods (must implement)
    fn generate_dataset_split_graph_filters(&self) -> HashMap<DatasetSplits, PipelineGraphFilter>;
    fn split_datasets(&mut self) -> Result<()>;
    fn execute(&self, splits: &HashMap<DatasetSplits, PipelineGraphFilter>) -> Result<Self::Result>;

    // Template method (default implementation)
    fn compute(&mut self) -> Result<Self::Result> {
        // Orchestrate: validate → execute steps → split → execute
    }
}
```

### Decision 3: Configuration Handling

**Java**: Uses @Configuration annotation processor
**Rust**: Manual struct definitions with validation

```rust
pub struct NodePropertyStepConfig {
    pub mutate_property: String,
    pub context_node_labels: Vec<String>,
    pub context_relationship_types: Vec<String>,
    // Algorithm-specific config as HashMap
    pub config: HashMap<String, serde_json::Value>,
}

impl NodePropertyStepConfig {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validation logic
    }
}
```

### Decision 4: Pipeline Catalog Storage

**Java**: Static ConcurrentHashMap
**Rust**: lazy_static with Mutex

```rust
lazy_static! {
    static ref PIPELINE_CATALOG: Mutex<HashMap<String, HashMap<String, Box<dyn TrainingPipeline>>>> =
        Mutex::new(HashMap::new());
}
```

**Alternative**: Use parking_lot::RwLock for better performance

---

## 📝 Implementation Notes

### Module Structure

```
src/projection/native/ml/pipeline/
├── mod.rs                                  # Module exports
├── feature_step.rs                         # ✅ Done
├── executable_node_property_step.rs        # ✅ Done
├── pipeline_trait.rs                       # ✅ Done
├── pipeline_graph_filter.rs                # TODO: Session 2
├── auto_tuning_config.rs                   # TODO: Session 2
├── node_property_step_context_config.rs    # TODO: Session 2
├── feature_step_util.rs                    # TODO: Session 3
├── non_empty_set_validation.rs             # TODO: Session 3
├── pipeline_companion.rs                   # TODO: Session 3
├── node_property_step.rs                   # TODO: Session 4 (Critical!)
├── node_property_step_factory.rs           # TODO: Session 4
├── node_property_step_executor.rs          # TODO: Session 5
├── pipeline_executor_trait.rs              # TODO: Session 6
├── predict_pipeline_executor_trait.rs      # TODO: Session 6
├── pipeline_trainer_trait.rs               # TODO: Session 7
├── result_to_model_converter_trait.rs      # TODO: Session 7
├── training_pipeline_trait.rs              # TODO: Session 7
├── pipeline_train_algorithm.rs             # TODO: Session 7
├── pipeline_catalog.rs                     # TODO: Session 8
├── canonical_procedure_name.rs             # TODO: Session 9 (optional)
├── mutate_mode_algorithm_library.rs        # TODO: Session 9 (optional)
└── stub_trait.rs                           # TODO: Session 9 (optional)
```

### Testing Strategy

**Unit Tests** (per file):

- Data structures: Builders, validation
- Utilities: Function behavior
- Configs: Parsing, validation

**Integration Tests** (per phase):

- Phase 4: NodePropertyStep execution with mock algorithms
- Phase 5: NodePropertyStepExecutor with multiple steps
- Phase 6: Full pipeline execution end-to-end
- Phase 7: Training pipeline with mock trainer

**End-to-End Test** (final):

- Create pipeline
- Add node property steps (PageRank, FastRP)
- Add feature steps
- Execute training
- Verify results

### Documentation Standard

Each file must include:

1. Module-level doc with Java source reference
2. Complete Java code snippet showing original
3. Rust translation with design notes
4. Method-level docs referencing Java equivalents
5. Usage examples where applicable

---

## 🎉 Success Metrics

### Phase Completion Criteria

**Phase 2 (Foundation)**:

- ✅ All data structures compile
- ✅ Builder patterns work
- ✅ Validation logic correct
- ✅ Tests: 10+

**Phase 3 (Utilities)**:

- ✅ Feature dimension calculation works
- ✅ NaN validation works
- ✅ Tests: 15+

**Phase 4 (Concrete Steps)**:

- ✅ Can create NodePropertyStep instances
- ✅ Factory validation works
- ✅ Integration with algorithm registry
- ✅ Tests: 20+

**Phase 5 (Step Execution)**:

- ✅ Can execute multiple steps in sequence
- ✅ Context expansion works
- ✅ Cleanup works
- ✅ Tests: 10+

**Phase 6 (Executors)**:

- ✅ Template method pattern works
- ✅ Dataset splitting works
- ✅ Full orchestration works
- ✅ Tests: 15+

**Phase 7 (Training)**:

- ✅ Training pipeline trait works
- ✅ Hyperparameter space works
- ✅ Model conversion works
- ✅ Tests: 20+

**Phase 8 (Catalog)**:

- ✅ Thread-safe registry works
- ✅ Per-user isolation works
- ✅ CRUD operations work
- ✅ Tests: 10+

### Final Success Criteria

- ✅ All 31 files translated (or decision made to skip some)
- ✅ 100+ pipeline tests passing
- ✅ Zero compilation errors
- ✅ Acceptable warnings only
- ✅ End-to-end pipeline execution works
- ✅ Documentation complete with Java references
- ✅ Ready for node/link pipeline specialization

---

## 📅 Estimated Timeline

**Conservative Estimate** (1-2 sessions per phase):

- Phase 2 (Foundation): 1 session (~2 hours)
- Phase 3 (Utilities): 1 session (~2 hours)
- Phase 4 (Concrete Steps): 2 sessions (~4 hours) - **Critical decision point**
- Phase 5 (Step Execution): 1 session (~2 hours)
- Phase 6 (Executors): 2 sessions (~4 hours)
- Phase 7 (Training): 2 sessions (~4 hours)
- Phase 8 (Catalog): 1 session (~2 hours)
- Phases 9-10 (Optional): 1-2 sessions (~2-4 hours)

**Total**: 11-13 sessions, ~22-26 hours

**Aggressive Estimate** (batch sessions):

- Sessions 2-3: Foundation + Utilities (~4 hours)
- Sessions 4-5: Concrete Steps + Execution (~6 hours)
- Sessions 6-7: Executors + Training (~6 hours)
- Session 8: Catalog (~2 hours)

**Total**: 4-5 large sessions, ~18-20 hours

---

## 🚀 Next Steps (Session 2)

**Ready to implement**: Phase 2 - Foundation Types (CORRECTED)

**Note**: DatasetSplits removed (nested enum in PipelineExecutor, not separate file)

1. Create `pipeline_graph_filter.rs` (30 lines)
2. Create `auto_tuning_config.rs` (50 lines)
3. Create `node_property_step_context_config.rs` (40 lines)

**Deliverable**: 3 simple files, ~120 lines, tests passing

**After that**: Session 3 - Utilities, then critical Session 4 decision on stub vs direct integration.

---

**Translation Plan Status**: ✅ COMPLETE  
**Current Phase**: ✅ Phase 1 Complete (3/31 files, 339/3300 lines)  
**Next Phase**: Phase 2 - Foundation Types (4 files, ~140 lines)  
**Ready to proceed**: Yes - clear priorities and sequences defined
