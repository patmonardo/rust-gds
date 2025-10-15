# Catalog Architecture: The Three Pillars

**Date**: October 15, 2025  
**Status**: Architectural Planning  
**Context**: Before completing Pipeline translation, need to understand Model/Pipeline/GraphStore relationships

---

## The Three Catalog Systems

Rust-GDS runtime has **three major catalog/registry systems** that form the operational foundation:

```
┌─────────────────┐
│ GraphStoreCatalog│  ← Input data: graphs with properties
└────────┬────────┘
         │ provides
         ↓
┌─────────────────┐
│ PipelineCatalog │  ← Training recipes: feature engineering + model configs
└────────┬────────┘
         │ produces
         ↓
┌─────────────────┐
│  ModelCatalog   │  ← Trained models: ready for prediction
└─────────────────┘
```

### 1. GraphStoreCatalog (Data Layer)

**Purpose**: Store and retrieve named graph projections  
**Scope**: `/core/loading/GraphStoreCatalog.java`  
**Key Operations**:

- `set(graphName, graphStore)` - Store graph projection
- `get(graphName) -> GraphStore` - Retrieve for use
- `drop(graphName)` - Remove projection
- `list() -> Stream<GraphStoreEntry>` - List available graphs

**Relationship**:

- **Consumed by Pipelines**: Pipelines read from GraphStore during training/prediction
- **Referenced by Models**: Models may store graph metadata (node labels, property schema)

**Current Status in Rust**: ✅ `DefaultGraphStore` exists, catalog not yet translated

---

### 2. PipelineCatalog (Recipe Layer)

**Purpose**: Store and retrieve ML pipeline configurations (training recipes)  
**Scope**: `/pipeline/PipelineCatalog.java` + `PipelineUserCatalog.java`  
**Java Structure**:

```java
public final class PipelineCatalog {
    private static final ConcurrentHashMap<String, PipelineUserCatalog> userCatalogs;

    static void set(String user, String pipelineName, TrainingPipeline<?> pipeline);
    static TrainingPipeline<?> get(String user, String pipelineName);
    static <PIPELINE extends TrainingPipeline<?>> PIPELINE getTyped(String user, String name, Class<PIPELINE> type);
    static TrainingPipeline<?> drop(String user, String pipelineName);
    static Stream<PipelineCatalogEntry> getAllPipelines(String user);
}
```

**Key Features**:

- **User-scoped**: Each user has isolated pipeline namespace
- **Type-safe retrieval**: `getTyped()` validates pipeline type (NodeClassification vs LinkPrediction vs NodeRegression)
- **Pipeline types map**:
  ```java
  Map<Class<?>, String> classToType = Map.of(
      NodeClassificationTrainingPipeline.class, "Node classification training pipeline",
      LinkPredictionTrainingPipeline.class, "Link prediction training pipeline"
      // NodeRegressionTrainingPipeline also exists
  );
  ```

**Relationship**:

- **Reads from GraphStoreCatalog**: Training pipelines operate on stored graphs
- **Produces Models**: Training pipeline → ModelCatalog entry
- **Multi-tenant**: User isolation for pipeline recipes

**Current Status in Rust**: ⚠️ TrainingPipeline trait exists but incomplete, no catalog yet

---

### 3. ModelCatalog (Artifact Layer)

**Purpose**: Store and retrieve trained ML models  
**Scope**: `/model-catalog-api/ModelCatalog.java` (interface) + implementations  
**Java Structure**:

```java
public interface ModelCatalog {
    // Storage
    void set(Model<?, ?, ?> model);

    // Typed retrieval
    <D, C extends ModelConfig, I extends CustomInfo> Model<D, C, I> get(
        String username, String modelName,
        Class<D> dataClass, Class<C> configClass, Class<I> infoClass
    );

    // Untyped operations
    Model<?, ?, ?> getUntypedOrThrow(String username, String modelName);
    Stream<Model<?, ?, ?>> getAllModels();

    // Lifecycle
    Model<?, ?, ?> dropOrThrow(String username, String modelName);
    Model<?, ?, ?> publish(String username, String modelName);  // Make public
    Model<?, ?, ?> store(String username, String modelName, Path modelDir);  // Persist to disk

    // Queries
    boolean exists(String username, String modelName);
    Collection<Model<?, ?, ?>> list(String username);
    long modelCount();
    boolean isEmpty();

    // Lifecycle
    void removeAllLoadedModels();
    void verifyModelCanBeStored(String username, String modelName, String modelType);

    // Events
    void registerListener(ModelCatalogListener listener);
    void unregisterListener(ModelCatalogListener listener);
}
```

**Generic Model Structure**:

```java
class Model<DATA, CONFIG extends ModelConfig, CUSTOM_INFO extends CustomInfo> {
    DATA data;                    // Actual trained model (weights, trees, etc.)
    CONFIG trainConfig;           // Training configuration
    CUSTOM_INFO customInfo;       // Pipeline-specific metadata
    ModelType modelType;          // NodeClassification, LinkPrediction, NodeRegression
    ZonedDateTime creationTime;
    String creator;
}
```

**Key Features**:

- **Generic over 3 type parameters**: `Model<D, C, I>`
  - `D`: Model data (e.g., `Classifier`, `Regressor`, `LinkPredictor`)
  - `C`: Training config (e.g., `NodeClassificationPipelineTrainConfig`)
  - `I`: Custom info (e.g., `NodeClassificationPipelineModelInfo` with feature importance)
- **Persistence**: `store()` saves to disk, reload later
- **Publishing**: Private models (user-scoped) can be made public
- **Event-driven**: Listeners notified on model add/drop

**Relationship**:

- **Produced by Pipelines**: `TrainingPipeline.train() -> Model`
- **References GraphStore**: Models may store graph schema/labels used during training
- **Used for Prediction**: Prediction pipelines load models from catalog

**Current Status in Rust**: ❌ **Nothing translated yet** - this is Priority 1

---

## Data Flow: The Complete ML Lifecycle

```
1. PROJECTION PHASE
   User → gds.graph.project() → GraphStoreCatalog.set("myGraph", graphStore)

2. PIPELINE CREATION PHASE
   User → gds.beta.pipeline.nodeClassification.create("myPipeline")
        → PipelineCatalog.set(user, "myPipeline", new NodeClassificationTrainingPipeline())
   User → pipeline.addNodeProperty(...)  // Add feature engineering steps
   User → pipeline.addFeature(...)        // Add feature selection
   User → pipeline.addTrainerConfig(...)  // Add model candidates

3. TRAINING PHASE
   User → gds.beta.pipeline.nodeClassification.train("myGraph", {pipeline: "myPipeline", ...})
        → Pipeline = PipelineCatalog.get(user, "myPipeline")
        → GraphStore = GraphStoreCatalog.get("myGraph")
        → Model = Pipeline.train(GraphStore, config)
        → ModelCatalog.set(Model)

4. PREDICTION PHASE
   User → gds.beta.pipeline.nodeClassification.predict("myGraph", {model: "myModel", ...})
        → Model = ModelCatalog.get(user, "myModel")
        → GraphStore = GraphStoreCatalog.get("myGraph")
        → Predictions = Model.predict(GraphStore, config)

5. PERSISTENCE PHASE
   User → gds.model.store("myModel", {storeDir: "/path"})
        → ModelCatalog.store(user, "myModel", path)
```

---

## Translation Priority & Strategy

### Priority 1: Model System Foundation ⭐⭐⭐

**Why First**: Models are the **output** of pipelines. Understanding Model structure will:

1. Clarify what Pipeline.train() must produce
2. Define ModelConverter interface (Pipeline-specific → generic Model)
3. Identify gaps in our current Pipeline trait

**Files to Translate** (in order):

1. `Model.java` - Generic model container (data/config/customInfo)
2. `ModelConfig.java` - Base config interface
3. `CustomInfo` types - Pipeline-specific metadata (ModelInfo classes)
4. `ModelCatalog.java` - Storage/retrieval interface
5. Model-specific types:
   - `Classifier.java` (for NodeClassification)
   - `Regressor.java` (for NodeRegression)
   - `LinkPredictor.java` (for LinkPrediction)

**Outcome**: Clear understanding of what Pipeline converters must produce

---

### Priority 2: Complete Regression Pipeline ⭐⭐

**Files Remaining**:

1. `NodeRegressionTrain.java` - Core training algorithm
2. `NodeRegressionTrainAlgorithm.java` - Wrapper
3. `NodeRegressionTrainPipelineAlgorithmFactory.java` - Factory
4. `NodeRegressionToModelConverter.java` - Pipeline → Model converter
5. `NodeRegressionPipelineModelInfo.java` - Custom metadata
6. `NodeRegressionTrainResult.java` - Training output

**Benefit**: Completing regression will expose **all** Model/Pipeline integration points

---

### Priority 3: PipelineCatalog Translation ⭐

**Why After Model**: PipelineCatalog needs complete TrainingPipeline trait, which needs Model types

**Files**:

1. `PipelineCatalog.java` - Main catalog interface
2. `PipelineUserCatalog.java` - Per-user storage
3. `PipelineCatalogEntry.java` - Catalog entry metadata

**Simpler than ModelCatalog**: Just stores TrainingPipeline references, no generic complexity

---

### Priority 4: LinkPipeline Investigation ⭐

**Scope**: Understand if LinkPipeline is:

- **Specialized**: Completely different from Node pipelines (link features, pair sampling)
- **Generalizable**: Can we abstract a "CombinatoricPipeline" for node pairs?

**Files to Review**:

- `LinkPredictionTrainingPipeline.java`
- `LinkFeatureStep.java` (vs NodeFeatureStep)
- `LinkFeatureExtractor.java` (pair-based features)
- `LinkPredictionRelationshipSampler.java` (negative sampling)

**Question**: Is this a third specialized pipeline, or can we generalize node/link patterns?

---

### Priority 5: GraphStoreCatalog Integration

**When**: After Model and Pipeline catalogs understood

**Focus**:

- How GraphStore lifecycle relates to Pipeline/Model lifecycle
- Graph schema validation in pipelines
- Reference counting / graph cleanup

---

## Open Questions

1. **Model Persistence**: Do we need disk serialization now, or placeholder it?

   - Java uses Arrow IPC for model weights
   - Could defer to "Phase 7: Persistence"

2. **Multi-tenancy**: Rust doesn't have built-in user context like Neo4j

   - User string in catalog keys?
   - Separate catalog per "session"?

3. **Concurrency**: Java uses ConcurrentHashMap

   - Rust: `Arc<RwLock<HashMap>>` or `dashmap`?

4. **Event System**: ModelCatalogListener pattern

   - Rust: callbacks, channels, or trait objects?

5. **LinkPipeline Generalization**: Can we abstract node/link patterns?
   - Or keep three separate pipeline types?

---

## Next Steps (Today's Plan)

1. ✅ **Document this architecture** (this file)
2. 🔄 **Translate Model foundation** (Model.java, ModelConfig.java)
3. 🔄 **Complete Regression pipeline** (remaining 4-6 files)
4. 🔄 **Surface all Model/Pipeline API gaps** during translation
5. 📝 **Create ModelCatalog design** based on findings
6. 📝 **Decide on LinkPipeline strategy** (generalize vs specialize)

**Goal**: By end of today, have clear picture of Model/Pipeline/GraphStore relationships and APIs before attempting full integration.

---

## Reference Files

**Java Source Locations**:

- `/graph-data-science/model-catalog-api/src/main/java/org/neo4j/gds/core/model/`
- `/graph-data-science/pipeline/src/main/java/org/neo4j/gds/ml/pipeline/`
- `/graph-data-science/core/src/main/java/org/neo4j/gds/core/loading/GraphStoreCatalog.java`

**Rust Target Locations** (proposed):

- `src/projection/native/model/` - Model system
- `src/projection/native/ml/pipeline/` - Pipeline system (existing)
- `src/types/graph_store/` - GraphStore (existing, add catalog)
