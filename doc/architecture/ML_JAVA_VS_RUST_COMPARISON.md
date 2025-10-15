# Java GDS vs Rust GDS ML Pipeline Comparison

## What Phase 2.2 Actually Implemented

### Executive Summary

Phase 2.2 implemented **~30% of the Java GDS Pipeline system**, focusing on:

- ✅ **Core pipeline structure** (PipelineDescriptor, PipelineState, PipelineExecutor)
- ✅ **Step abstraction** (NodePropertyStep, FeatureStep with executors)
- ✅ **Mock execution** (proves architecture works end-to-end)

**What's missing**: Algorithm registry, actual execution, auto-tuning, training, catalog, validation.

---

## Detailed Component Comparison

### 1. Pipeline Core

| Component           | Java GDS                  | Rust GDS Phase 2.2          | Status      |
| ------------------- | ------------------------- | --------------------------- | ----------- |
| Pipeline definition | `Pipeline.java` interface | `PipelineDescriptor` struct | ✅ Basic    |
| Training pipeline   | `TrainingPipeline.java`   | ❌ Not implemented          | ⏸️ Future   |
| Execution state     | Embedded in executors     | `PipelineState` struct      | ✅ Complete |
| Lifecycle           | init→compute→release      | init→step→finalize          | ✅ Similar  |
| Progress tracking   | `ProgressTracker`         | `PipelineState.progress()`  | ✅ Basic    |

**Java Pattern**:

```java
public abstract class TrainingPipeline<FEATURE_STEP extends FeatureStep>
    implements Pipeline<FEATURE_STEP> {
    protected List<ExecutableNodePropertyStep> nodePropertySteps;
    protected List<FEATURE_STEP> featureSteps;
    protected Map<TrainingMethod, List<TunableTrainerConfig>> trainingParameterSpace;
    protected AutoTuningConfig autoTuningConfig;
}
```

**Rust Pattern** (Phase 2.2):

```rust
pub struct PipelineDescriptor {
    pub name: String,
    pub steps: Vec<StepDescriptor>,
    pub config: PipelineConfig,
}

pub struct PipelineState {
    pub features: HashMap<String, Arc<dyn PropertyValues>>,
    pub phase: ExecutionPhase,
    pub steps_completed: usize,
    pub total_steps: usize,
}
```

**Gap**: No training/predict distinction, no auto-tuning, no parameter space.

---

### 2. Node Property Steps

| Component             | Java GDS                            | Rust GDS Phase 2.2           | Status       |
| --------------------- | ----------------------------------- | ---------------------------- | ------------ |
| Step descriptor       | `NodePropertyStep.java`             | `NodePropertyStepDescriptor` | ✅ Similar   |
| Executable step       | `ExecutableNodePropertyStep`        | `NodePropertyStepExecutor`   | ✅ Structure |
| Algorithm lookup      | `MutateModeAlgorithmLibrary`        | ❌ Not implemented           | ⏸️ Phase 2.3 |
| Configuration parsing | `ConfigurationParsersForMutateMode` | ❌ Not implemented           | ⏸️ Phase 2.3 |
| Context config        | `NodePropertyStepContextConfig`     | ❌ Not implemented           | ⏸️ Future    |
| Stub system           | `Stub.java`, `StubbyHolder`         | ❌ Not implemented           | ⏸️ Phase 2.3 |
| Actual execution      | Calls algorithm procedures          | Returns mock values          | ❌ Mock only |

**Java Pattern**:

```java
public interface ExecutableNodePropertyStep {
    void execute(
        ExecutionContext executionContext,
        String graphName,
        Collection<NodeLabel> nodeLabels,
        Collection<RelationshipType> relTypes,
        Concurrency trainConcurrency,
        Stub stub
    );
}
```

**Rust Pattern** (Phase 2.2):

```rust
pub struct NodePropertyStepExecutor {
    descriptor: NodePropertyStepDescriptor,
}

impl StepExecutor for NodePropertyStepExecutor {
    fn execute(&self, graph: &Arc<dyn Graph>, state: &mut PipelineState)
        -> Result<StepResult, ComputeError> {
        // Phase 2.2: Just creates mock values
        let mock_values = MockLongPropertyValues::new(graph.node_count());
        state.add_feature(name, Arc::new(mock_values));
        Ok(StepResult::success(name, "computed"))
    }
}
```

**Gap**: The Java system has a sophisticated stub/factory pattern that dispatches to ~50 different algorithms. Rust just returns mock data.

---

### 3. Algorithm Integration Architecture

**Java GDS Has** (Not in Rust yet):

#### MutateModeAlgorithmLibrary

```java
// Maps procedure names to Algorithm enum
private final Map<CanonicalProcedureName, Algorithm> knownAlgorithms;

// Example mappings:
gds.pagerank -> Algorithm.PAGE_RANK
gds.louvain -> Algorithm.LOUVAIN
gds.fastrp -> Algorithm.FAST_RP
```

#### Stub System

```java
// Each algorithm has a stub that knows how to:
// 1. Estimate memory
// 2. Execute the algorithm
// 3. Validate configuration

public interface Stub {
    MemoryEstimation getMemoryEstimation(...);
    void execute(AlgorithmsProcedureFacade facade, String graphName, Map config);
}
```

#### StubbyHolder

Centralizes ~50 algorithm stubs:

- `PageRankStub`, `LouvainStub`, `FastRPStub`, `KnnStub`, etc.
- Each stub knows algorithm-specific behavior

**Rust GDS Phase 2.2**: ❌ None of this exists yet

**Phase 2.3 Will Need**:

```rust
// Proposed architecture
pub struct AlgorithmRegistry {
    algorithms: HashMap<String, Box<dyn AlgorithmExecutor>>,
}

pub trait AlgorithmExecutor {
    fn execute(&self, graph: &Arc<dyn Graph>, config: &HashMap<String, Value>)
        -> Result<Arc<dyn PropertyValues>, ComputeError>;
    fn estimate_memory(&self, graph: &Arc<dyn Graph>) -> MemoryEstimation;
}
```

---

### 4. Feature Steps

| Component          | Java GDS                              | Rust GDS Phase 2.2      | Status       |
| ------------------ | ------------------------------------- | ----------------------- | ------------ |
| Step descriptor    | `FeatureStep.java`                    | `FeatureStepDescriptor` | ✅ Similar   |
| Executor           | Embedded in pipeline                  | `FeatureStepExecutor`   | ✅ Structure |
| Property dimension | `FeatureStepUtil.propertyDimension()` | ❌ Not implemented      | ⏸️ Phase 2.3 |
| NaN validation     | `validateComputedFeatures()`          | ❌ Not implemented      | ⏸️ Future    |
| Property reading   | `graph.nodeProperties(name)`          | ❌ Not implemented      | ⏸️ Phase 2.3 |
| Actual computation | Reads from graph                      | Returns mock embeddings | ❌ Mock only |

**Java Pattern**:

```java
public static int propertyDimension(Graph graph, String nodeProperty) {
    return propertyDimension(graph.nodeProperties(nodeProperty), nodeProperty);
}

public static int propertyDimension(NodePropertyValues nodeProperties, String propertyName) {
    int dimension = 0;
    // Logic to detect scalar vs array vs embedding
    switch (nodeProperties.valueType()) {
        case DOUBLE -> dimension = 1;
        case DOUBLE_ARRAY -> dimension = nodeProperties.doubleArrayValue(0).length;
        // ...
    }
    return dimension;
}
```

**Rust Pattern** (Phase 2.2):

```rust
fn compute_feature(&self, graph: &Arc<dyn Graph>, _state: &PipelineState)
    -> Result<Arc<dyn PropertyValues>, ComputeError> {
    // Phase 2.2: Just creates mock embeddings
    let dimension = self.descriptor.target_dimension.unwrap_or(128);
    let mock_values = MockEmbeddingPropertyValues::new(graph.node_count(), dimension);
    Ok(Arc::new(mock_values))
}
```

**Gap**: No actual property reading, no dimension detection, no feature assembly.

---

### 5. Auto-Tuning & Training

| Component          | Java GDS                      | Rust GDS Phase 2.2 | Status    |
| ------------------ | ----------------------------- | ------------------ | --------- |
| Auto-tuning config | `AutoTuningConfig.java`       | ❌ Not implemented | ⏸️ Future |
| Training pipeline  | `PipelineTrainAlgorithm.java` | ❌ Not implemented | ⏸️ Future |
| Pipeline trainer   | `PipelineTrainer.java`        | ❌ Not implemented | ⏸️ Future |
| Model converter    | `ResultToModelConverter.java` | ❌ Not implemented | ⏸️ Future |
| Dataset splits     | `DatasetSplits` enum          | ❌ Not implemented | ⏸️ Future |

**Java Has**:

```java
@Configuration
public interface AutoTuningConfig {
    int MAX_TRIALS = 10;

    @Configuration.IntegerRange(min = 1)
    default int maxTrials() { return MAX_TRIALS; }
}

public abstract class PipelineTrainAlgorithm<RESULT, MODEL, CONFIG> {
    private final PipelineTrainer<RESULT> pipelineTrainer;
    private final ResultToModelConverter<MODEL, RESULT> converter;

    public MODEL compute() {
        RESULT trainingResult = pipelineTrainer.run();
        return converter.toModel(trainingResult, graphStore.schema());
    }
}
```

**Rust Phase 2.2**: Nothing related to training exists yet.

---

### 6. Pipeline Catalog

| Component          | Java GDS                      | Rust GDS Phase 2.2 | Status    |
| ------------------ | ----------------------------- | ------------------ | --------- |
| Pipeline storage   | `PipelineCatalog.java`        | ❌ Not implemented | ⏸️ Future |
| User catalogs      | `PipelineUserCatalog`         | ❌ Not implemented | ⏸️ Future |
| Pipeline retrieval | `get()`, `exists()`, `list()` | ❌ Not implemented | ⏸️ Future |
| Type mapping       | `classToType` map             | ❌ Not implemented | ⏸️ Future |

**Java Pattern**:

```java
public final class PipelineCatalog {
    private static final ConcurrentHashMap<String, PipelineUserCatalog> userCatalogs;

    public static void set(String user, String pipelineName, TrainingPipeline<?> pipeline);
    public static <T extends TrainingPipeline<?>> T get(String user, String pipelineName);
    public static boolean exists(String user, String pipelineName);
    public static Stream<PipelineCatalogEntry> list(String user);
}
```

**Rust Phase 2.2**: No catalog system exists.

---

## Architecture: What Was Based on Java?

### Directly Inspired by Java ✅

1. **Pipeline Structure**

   - Java: `Pipeline<FEATURE_STEP>` interface
   - Rust: `PipelineDescriptor` struct
   - **Match**: ~70% - Similar concept, simpler implementation

2. **Step Pattern**

   - Java: `ExecutableNodePropertyStep`, `FeatureStep` interfaces
   - Rust: `NodePropertyStepExecutor`, `FeatureStepExecutor` structs
   - **Match**: ~60% - Similar execution pattern, but no stub system

3. **Step Descriptors**

   - Java: `NodePropertyStep.java` with config maps
   - Rust: `NodePropertyStepDescriptor` with typed fields
   - **Match**: ~80% - Very similar structure

4. **State Tracking**
   - Java: Embedded in `PipelineExecutor.graphStore`
   - Rust: Explicit `PipelineState` struct
   - **Match**: ~50% - Different approach, but same goal

### Not Yet Implemented (But Planned) ⏸️

5. **Algorithm Registry**

   - Java: `MutateModeAlgorithmLibrary` + `StubbyHolder`
   - Rust: **Planned for Phase 2.3**
   - **Match**: 0% currently

6. **Training System**

   - Java: `PipelineTrainAlgorithm`, `PipelineTrainer`, auto-tuning
   - Rust: **Planned for later phases**
   - **Match**: 0% currently

7. **Pipeline Catalog**
   - Java: `PipelineCatalog` with user catalogs
   - Rust: **Future work**
   - **Match**: 0% currently

---

## What Phase 2.2 Achieved

### The Good ✅

1. **Architectural Foundation**

   - Pipeline/Step/Executor pattern established
   - PropertyValues abstraction works
   - Plugin architecture proven (Computer trait)
   - Clean separation: descriptors (WHAT) vs executors (HOW)

2. **Execution Flow**

   - Init→Execute→Finalize lifecycle works
   - State management functional
   - Progress tracking operational
   - Feature storage validated

3. **Testing**
   - 18 tests, 100% passing
   - Mock PropertyValues deterministic and correct
   - Pipeline validation works

### The Gap ❌

1. **No Actual Algorithms**

   - Everything returns mock data
   - No algorithm registry
   - No real PageRank, Louvain, FastRP, etc.

2. **No Training**

   - No auto-tuning
   - No model training
   - No dataset splitting
   - No cross-validation

3. **No Catalog**

   - No pipeline persistence
   - No user management
   - No pipeline retrieval

4. **Missing Java Features**
   - Stub system for algorithm dispatch
   - Configuration parsers per algorithm
   - Memory estimation integration
   - Canonical procedure name parsing
   - Context configuration (nodeLabels, relTypes)

---

## Phase 2.3 Roadmap (Based on Java)

To reach Java GDS parity, Phase 2.3+ needs:

### Immediate (Phase 2.3)

1. **AlgorithmRegistry**

   ```rust
   pub struct AlgorithmRegistry {
       algorithms: HashMap<String, Box<dyn AlgorithmExecutor>>,
   }

   impl AlgorithmRegistry {
       fn new() -> Self {
           let mut registry = HashMap::new();
           registry.insert("pageRank".to_string(), Box::new(PageRankExecutor));
           registry.insert("louvain".to_string(), Box::new(LouvainExecutor));
           // ... ~50 more algorithms
           Self { algorithms: registry }
       }
   }
   ```

2. **AlgorithmExecutor Trait**

   ```rust
   pub trait AlgorithmExecutor: Send + Sync {
       fn execute(&self, graph: &Arc<dyn Graph>, config: &NodePropertyStepConfig)
           -> Result<Arc<dyn PropertyValues>, ComputeError>;
       fn estimate_memory(&self, graph: &Arc<dyn Graph>) -> MemoryEstimation;
       fn validate_config(&self, config: &NodePropertyStepConfig) -> Result<(), ConfigError>;
   }
   ```

3. **First Real Algorithms**

   - PageRank (already exists in codebase)
   - Louvain (community detection)
   - FastRP (embeddings)

4. **Feature Property Reading**
   ```rust
   fn compute_feature(&self, graph: &Arc<dyn Graph>, state: &PipelineState) {
       // Read from graph PropertyStore
       let property_values = graph.node_properties(&self.descriptor.source_properties[0])?;
       // Assemble into feature vector
       let features = assemble_features(property_values, &self.descriptor);
       state.add_feature(self.descriptor.name.clone(), features);
   }
   ```

### Short-term (Phase 2.4)

5. **Training Pipeline**

   - TrainingPipelineDescriptor
   - Auto-tuning configuration
   - Parameter space definition

6. **Model Training**
   - PipelineTrainer trait
   - Cross-validation
   - Model evaluation metrics

### Medium-term (Phase 2.5+)

7. **Pipeline Catalog**

   - Persistence layer
   - User-specific catalogs
   - Pipeline versioning

8. **Advanced Features**
   - Memory estimation
   - Progress tracking improvements
   - Validation service
   - Configuration parsers

---

## Java vs Rust Design Differences

### Java Approach

- **Dynamic dispatch**: Procedure facade + stubs
- **Runtime flexibility**: Reflective configuration parsing
- **Catalog-centric**: Pipelines stored and retrieved by name
- **User-aware**: Per-user defaults and limits

### Rust Approach (Current)

- **Static dispatch**: Direct algorithm calls (future)
- **Compile-time safety**: Typed configurations
- **Descriptor-centric**: Pipelines as serializable data
- **Context-aware**: ExecutionContext pattern (future)

### Better Patterns in Rust

1. **Type safety**: Descriptors are strongly typed
2. **Ownership**: Clear Arc usage for PropertyValues
3. **Error handling**: Result types instead of exceptions
4. **Trait-based**: Cleaner abstractions than Java interfaces

---

## Conclusion

### What Phase 2.2 Delivered

**~30% of Java GDS ML Pipeline**, specifically:

- ✅ Core pipeline structure (PipelineDescriptor, PipelineState)
- ✅ Step abstraction (NodePropertyStep, FeatureStep)
- ✅ Execution infrastructure (PipelineExecutor, StepExecutor)
- ✅ Mock implementation proving architecture works

### What's Still Missing

**~70% of Java GDS ML Pipeline**, specifically:

- ❌ Algorithm registry and execution (~40 algorithms)
- ❌ Training system (auto-tuning, cross-validation)
- ❌ Pipeline catalog (persistence, user management)
- ❌ Feature extraction from properties
- ❌ Memory estimation
- ❌ Advanced validation

### Was This Based on Java?

**Yes, heavily inspired**, but:

- ✅ Architectural patterns directly borrowed (Pipeline/Step/Executor)
- ✅ Lifecycle model similar (init→execute→finalize)
- ✅ Descriptor structure mirrors Java
- ❌ Implementation details differ (no stubs yet, simpler state)
- ❌ Only ~30% of Java functionality implemented

### Bottom Line

Phase 2.2 successfully **translated the core Java GDS Pipeline architecture to Rust** with:

- Clean, idiomatic Rust code
- Proven execution model
- Strong type safety
- Solid foundation for Phase 2.3

But it's **mock-only** - the real work (algorithm registry, actual execution, training) starts in Phase 2.3.
