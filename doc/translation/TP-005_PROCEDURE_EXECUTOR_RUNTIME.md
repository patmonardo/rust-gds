# Translation Plan TP-005: Procedure Executor Runtime

**Document Type**: Translation Plan (Prakasa)  
**Translation ID**: TP-005  
**Date**: October 16, 2025  
**Status**: 🌟 Prakasa (Illumination) - Ready for Kriya (Action)  
**Estimated Effort**: 18-24 hours (22 Java files → 7 Rust modules)  
**Priority**: 🎯 HIGH - Fixed GDSL Runtime for procedure execution

---

## 🕉️ Membership Protocol (Fichte's Method)

**This Translation Plan places itself within the rust-gds Encyclopedia as**:

- **Location**: `doc/translation/TP-005_PROCEDURE_EXECUTOR_RUNTIME.md`
- **Category**: Translation Plans (Prakasa → Kriya bridge)
- **Related ADR**: ADR0007 (Translation Plan Protocol)
- **Parent Context**: Procedure System Architecture
- **Precedence**: Follows Brahmachakra completion (70 tests passing)

**Purpose**: Translate Java GDS Executor package (~22 files) into fixed GDSL Runtime for rust-gds procedure execution. This is **NOT algorithm implementations** - those live in `src/procedure/`.

---

## Context & Motivation

### The Architecture Correction

**User's Critical Insight**:

> "The executor is a fixed Runtime involved in NativeFactory codegen directly, as computation runtime. But we have defined Projection as the GDSL Runtime. So I think it belongs in src/projection/eval/procedure."

**The Split**:

- **Executor Runtime** (this TP) → `src/projection/eval/procedure/` - Fixed GDSL Runtime
- **Algorithm Implementations** (separate) → `src/procedure/` - Extensible content

### Java GDS Source Analysis

**Location**: `/home/pat/GitHub/graph-data-science/executor/src/main/java/org/neo4j/gds/executor/`

**Complete File List** (22 files total):

```
executor/
├── AlgorithmSpec.java                           ← Algorithm contract
├── AlgorithmSpecProgressTrackerProvider.java    ← Progress tracking
├── ComputationResult.java                       ← Result wrapper
├── ComputationResultConsumer.java               ← Result consumption
├── ExecutionContext.java                        ← Runtime environment
├── ExecutionMode.java                           ← Stream/Stats/Write/Mutate
├── ExecutorSpec.java                            ← Executor specification
├── GdsCallable.java                             ← Callable annotation
├── GdsCallableFinder.java                       ← Reflection-based discovery
├── GraphCreation.java                           ← Graph creation interface
├── GraphCreationFactory.java                    ← Graph factory
├── MemoryEstimationExecutor.java                ← Memory estimation
├── Preconditions.java                           ← Preconditions checker
├── ProcedureExecutor.java                       ← MAIN ORCHESTRATOR
├── ProcedureExecutorSpec.java                   ← Default executor spec
├── ProcedureGraphCreation.java                  ← Graph creation impl
├── ProcedureGraphCreationFactory.java           ← Factory impl
├── ProcedureMemoryEstimation.java               ← Memory estimation
└── validation/
    ├── AfterLoadValidation.java                 ← After-load validation
    ├── BeforeLoadValidation.java                ← Before-load validation
    ├── ValidationConfiguration.java             ← Validation config
    └── Validator.java                           ← Validator orchestrator
```

**Total**: 22 Java files (18 main + 4 validation)

---

## Translation Strategy

### What We're Translating

**Core Runtime** (Priority 1 - Essential):

1. ExecutionMode.java → execution_mode.rs
2. ComputationResult.java → computation_result.rs
3. ExecutionContext.java → execution_context.rs
4. ValidationConfiguration.java + validation/\*.java → validation_config.rs
5. AlgorithmSpec.java → algorithm_spec.rs
6. ProcedureExecutor.java → executor.rs
7. ComputationResultConsumer.java → result_consumer.rs

**Graph Creation** (Priority 2 - Simplified):

- GraphCreation.java, GraphCreationFactory.java → Simplified into executor.rs
- ProcedureGraphCreation.java, ProcedureGraphCreationFactory.java → Not needed (we have GraphStore already)

**Memory Estimation** (Priority 3 - Future):

- MemoryEstimationExecutor.java, ProcedureMemoryEstimation.java → Future work (not MVP)

**Metadata/Discovery** (Priority 4 - Not Needed):

- GdsCallable.java, GdsCallableFinder.java → Java reflection, not applicable to Rust
- Preconditions.java, AlgorithmSpecProgressTrackerProvider.java → Optional utilities

### What We're NOT Translating (1:1)

**Skipped Files** (11 files):

- GdsCallable.java, GdsCallableFinder.java - Java reflection, not needed
- ExecutorSpec.java, ProcedureExecutorSpec.java - Over-engineered, fold into executor
- GraphCreation.java, GraphCreationFactory.java - We have simpler patterns
- ProcedureGraphCreation.java, ProcedureGraphCreationFactory.java - Catalog complexity
- MemoryEstimationExecutor.java, ProcedureMemoryEstimation.java - Future work
- Preconditions.java, AlgorithmSpecProgressTrackerProvider.java - Optional

**Translation Ratio**: 22 Java files → 7 Rust modules (simplified, focused)

---

## Rust Target Structure

**Location**: `src/projection/eval/procedure/`

```
src/projection/eval/procedure/
├── mod.rs                      (~50 lines) - Public API, re-exports
├── execution_mode.rs           (~60 lines) - ExecutionMode enum
├── computation_result.rs       (~150 lines) - ComputationResult struct
├── execution_context.rs        (~120 lines) - ExecutionContext runtime env
├── validation_config.rs        (~180 lines) - ValidationConfiguration + traits
├── algorithm_spec.rs           (~200 lines) - AlgorithmSpec trait + helpers
├── executor.rs                 (~250 lines) - ProcedureExecutor orchestrator
└── result_consumer.rs          (~100 lines) - Result consumption helpers
```

**Total**: ~1110 lines Rust (vs ~2500+ lines Java with all dependencies)

---

## Phase-by-Phase Translation

### Phase 1: Foundation Types (4-6 hours)

**Goal**: Simple data types with no dependencies

#### File 1: execution_mode.rs (~60 lines, 1 hour)

**Java Source**: ExecutionMode.java (30 lines)

```java
public enum ExecutionMode {
    STREAM,
    STATS,
    TRAIN,
    WRITE_NODE_PROPERTY,
    WRITE_RELATIONSHIP,
    MUTATE_NODE_PROPERTY,
    MUTATE_RELATIONSHIP
}
```

**Rust Translation**:

```rust
/// Execution Mode - How to return procedure results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    /// Stream all results to client
    Stream,
    /// Return summary statistics only
    Stats,
    /// Train ML model (for ML procedures)
    Train,
    /// Write node property to database
    WriteNodeProperty,
    /// Write relationship to database
    WriteRelationship,
    /// Add node property to in-memory graph
    MutateNodeProperty,
    /// Add relationship to in-memory graph
    MutateRelationship,
}

impl ExecutionMode {
    pub fn returns_results(&self) -> bool {
        matches!(self, ExecutionMode::Stream)
    }

    pub fn is_mutating(&self) -> bool {
        matches!(
            self,
            ExecutionMode::MutateNodeProperty
                | ExecutionMode::MutateRelationship
                | ExecutionMode::WriteNodeProperty
                | ExecutionMode::WriteRelationship
        )
    }

    pub fn is_writing(&self) -> bool {
        matches!(
            self,
            ExecutionMode::WriteNodeProperty | ExecutionMode::WriteRelationship
        )
    }

    pub fn produces_model(&self) -> bool {
        matches!(self, ExecutionMode::Train)
    }
}
```

**Verification**: Unit tests for all helper methods

---

#### File 2: computation_result.rs (~150 lines, 2 hours)

**Java Source**: ComputationResult.java (60 lines)

```java
@ValueClass
public interface ComputationResult<A extends Algorithm<ALGO_RESULT>, ALGO_RESULT, CONFIG extends AlgoBaseConfig> {
    long preProcessingMillis();
    long computeMillis();
    @Nullable A algorithm();
    Optional<ALGO_RESULT> result();
    Graph graph();
    GraphStore graphStore();
    ResultStore resultStore();
    CONFIG config();
    @Value.Default
    default boolean isGraphEmpty() { return false; }
}
```

**Rust Translation** (Simplified):

```rust
use std::time::Duration;
use crate::types::GraphStore;

/// Computation Result - Algorithm output with metadata
pub struct ComputationResult<R> {
    /// The actual algorithm result
    result: R,

    /// Time taken for computation
    compute_time: Duration,

    /// Time taken for pre-processing
    preprocess_time: Duration,

    /// Graph reference (if available)
    graph_store: Option<GraphStore>,

    /// Configuration used (JSON)
    config: serde_json::Value,

    /// Whether graph was empty
    is_graph_empty: bool,
}

impl<R> ComputationResult<R> {
    pub fn new(result: R, compute_time: Duration) -> Self {
        Self {
            result,
            compute_time,
            preprocess_time: Duration::ZERO,
            graph_store: None,
            config: serde_json::Value::Null,
            is_graph_empty: false,
        }
    }

    pub fn result(&self) -> &R {
        &self.result
    }

    pub fn into_result(self) -> R {
        self.result
    }

    pub fn compute_millis(&self) -> u64 {
        self.compute_time.as_millis() as u64
    }

    pub fn preprocess_millis(&self) -> u64 {
        self.preprocess_time.as_millis() as u64
    }

    pub fn with_preprocess_time(mut self, duration: Duration) -> Self {
        self.preprocess_time = duration;
        self
    }

    pub fn with_graph_store(mut self, graph_store: GraphStore) -> Self {
        self.graph_store = Some(graph_store);
        self
    }

    pub fn with_config(mut self, config: serde_json::Value) -> Self {
        self.config = config;
        self
    }

    pub fn mark_graph_empty(mut self) -> Self {
        self.is_graph_empty = true;
        self
    }

    pub fn is_graph_empty(&self) -> bool {
        self.is_graph_empty
    }
}
```

**Key Simplifications**:

- No separate Algorithm type (algorithm logic is in implementations)
- No ResultStore (we don't have catalog complexity yet)
- Simplified to essential timing + result data

**Verification**: Builder pattern tests

---

### Phase 2: Runtime Environment (3-4 hours)

#### File 3: execution_context.rs (~120 lines, 3 hours)

**Java Source**: ExecutionContext.java (187 lines with EMPTY impl)

**Key Parts**:

```java
@ValueClass
public interface ExecutionContext {
    DatabaseId databaseId();
    DependencyResolver dependencyResolver();
    @Nullable ModelCatalog modelCatalog();
    Log log();
    TerminationMonitor terminationMonitor();
    CloseableResourceRegistry closeableResourceRegistry();
    NodeLookup nodeLookup();
    ProcedureReturnColumns returnColumns();
    TaskRegistryFactory taskRegistryFactory();
    UserLogRegistryFactory userLogRegistryFactory();
    String username();
    boolean isGdsAdmin();
    Metrics metrics();
    // ... more
}
```

**Rust Translation** (Simplified for MVP):

```rust
use std::collections::HashMap;
use crate::types::GraphStore;

/// Execution Context - Runtime environment for procedure execution
pub struct ExecutionContext {
    /// Graph catalog (name → GraphStore)
    graph_catalog: HashMap<String, GraphStore>,

    /// Current user (for auditing)
    username: String,

    /// Log level
    log_level: LogLevel,

    /// Metrics collector
    metrics: MetricsCollector,

    /// Configuration overrides
    config_overrides: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct MetricsCollector {
    timings: HashMap<String, Vec<u64>>,
}

impl ExecutionContext {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            graph_catalog: HashMap::new(),
            username: username.into(),
            log_level: LogLevel::Info,
            metrics: MetricsCollector::new(),
            config_overrides: HashMap::new(),
        }
    }

    /// Load a graph from the catalog by name
    pub fn load_graph(&self, name: &str) -> Result<&GraphStore, ContextError> {
        self.graph_catalog
            .get(name)
            .ok_or_else(|| ContextError::GraphNotFound(name.to_string()))
    }

    /// Add graph to catalog
    pub fn add_graph(&mut self, name: impl Into<String>, graph: GraphStore) {
        self.graph_catalog.insert(name.into(), graph);
    }

    /// Get current username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Record timing metric
    pub fn record_timing(&mut self, operation: &str, duration_ms: u64) {
        self.metrics.record(operation, duration_ms);
    }

    /// Log message at current level
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.log_level {
            match level {
                LogLevel::Debug => log::debug!("{}", message),
                LogLevel::Info => log::info!("{}", message),
                LogLevel::Warn => log::warn!("{}", message),
                LogLevel::Error => log::error!("{}", message),
            }
        }
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
        }
    }

    pub fn record(&mut self, operation: &str, duration_ms: u64) {
        self.timings
            .entry(operation.to_string())
            .or_insert_with(Vec::new)
            .push(duration_ms);
    }

    pub fn get_timings(&self, operation: &str) -> Option<&[u64]> {
        self.timings.get(operation).map(|v| v.as_slice())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("Graph not found: {0}")]
    GraphNotFound(String),
}
```

**Key Simplifications**:

- No dependency injection (DependencyResolver) - direct ownership
- No Neo4j-specific types (DatabaseId, NodeLookup, etc.)
- No model catalog yet (future work)
- Simple HashMap-based graph catalog

**Mock for Testing**:

```rust
impl ExecutionContext {
    pub fn mock(graph: GraphStore) -> Self {
        let mut ctx = Self::new("test_user");
        ctx.add_graph("test_graph", graph);
        ctx
    }
}
```

**Verification**: Catalog operations, metrics collection

---

### Phase 3: Validation System (4-5 hours)

#### File 4: validation_config.rs (~180 lines, 4 hours)

**Java Sources**:

- ValidationConfiguration.java (~80 lines)
- BeforeLoadValidation.java (~30 lines)
- AfterLoadValidation.java (~30 lines)
- Validator.java (~60 lines)

**Java Pattern**:

```java
@ValueClass
public interface ValidationConfiguration<CONFIG> {
    List<BeforeLoadValidation<CONFIG>> beforeLoadValidations();
    List<AfterLoadValidation<CONFIG>> afterLoadValidations();
}

@FunctionalInterface
public interface BeforeLoadValidation<CONFIG> {
    void validateConfigsBeforeLoad(GraphStore graphStore, GraphProjectConfig config, CONFIG algoConfig);
}

@FunctionalInterface
public interface AfterLoadValidation<CONFIG> {
    void validateConfigsAfterLoad(GraphStore graphStore, GraphProjectConfig config, CONFIG algoConfig);
}
```

**Rust Translation**:

```rust
use crate::types::GraphStore;

/// Validation Configuration - Two-phase validation
pub struct ValidationConfiguration {
    before_load: Vec<Box<dyn BeforeLoadValidator>>,
    after_load: Vec<Box<dyn AfterLoadValidator>>,
}

impl ValidationConfiguration {
    pub fn new() -> Self {
        Self {
            before_load: Vec::new(),
            after_load: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn add_before_load<V: BeforeLoadValidator + 'static>(
        mut self,
        validator: V,
    ) -> Self {
        self.before_load.push(Box::new(validator));
        self
    }

    pub fn add_after_load<V: AfterLoadValidator + 'static>(
        mut self,
        validator: V,
    ) -> Self {
        self.after_load.push(Box::new(validator));
        self
    }

    /// Validate before graph load (config only)
    pub fn validate_before_load(
        &self,
        config: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        for validator in &self.before_load {
            validator.validate(config)?;
        }
        Ok(())
    }

    /// Validate after graph load (config + graph)
    pub fn validate_after_load(
        &self,
        graph_store: &GraphStore,
        config: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        for validator in &self.after_load {
            validator.validate(graph_store, config)?;
        }
        Ok(())
    }
}

/// Validator that runs before graph load
pub trait BeforeLoadValidator: Send + Sync {
    fn validate(&self, config: &serde_json::Value) -> Result<(), ValidationError>;
}

/// Validator that runs after graph load
pub trait AfterLoadValidator: Send + Sync {
    fn validate(
        &self,
        graph_store: &GraphStore,
        config: &serde_json::Value,
    ) -> Result<(), ValidationError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Before-load validation failed: {0}")]
    BeforeLoad(String),

    #[error("After-load validation failed: {0}")]
    AfterLoad(String),

    #[error("Parameter validation failed: {0}")]
    Parameter(String),
}
```

**Example Validators**:

```rust
/// Range validator (before load)
pub struct RangeValidator {
    param: String,
    min: f64,
    max: f64,
}

impl BeforeLoadValidator for RangeValidator {
    fn validate(&self, config: &serde_json::Value) -> Result<(), ValidationError> {
        if let Some(value) = config.get(&self.param).and_then(|v| v.as_f64()) {
            if value < self.min || value > self.max {
                return Err(ValidationError::Parameter(format!(
                    "{} must be between {} and {}, got {}",
                    self.param, self.min, self.max, value
                )));
            }
        }
        Ok(())
    }
}

/// Property exists validator (after load)
pub struct PropertyExistsValidator {
    property: String,
}

impl AfterLoadValidator for PropertyExistsValidator {
    fn validate(
        &self,
        graph_store: &GraphStore,
        _config: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        if !graph_store.has_node_property(&self.property) {
            return Err(ValidationError::AfterLoad(format!(
                "Required property '{}' not found in graph",
                self.property
            )));
        }
        Ok(())
    }
}
```

**Verification**: Validator chaining, error propagation

---

### Phase 4: Algorithm Contract (5-6 hours)

#### File 5: algorithm_spec.rs (~200 lines, 5 hours)

**Java Source**: AlgorithmSpec.java (~60 lines)

```java
public interface AlgorithmSpec<
    ALGO extends Algorithm<ALGO_RESULT>,
    ALGO_RESULT,
    CONFIG extends AlgoBaseConfig,
    RESULT,
    ALGO_FACTORY extends AlgorithmFactory<?, ALGO, CONFIG>
> {
    String name();
    ALGO_FACTORY algorithmFactory(ExecutionContext executionContext);
    default void preProcessConfig(Map<String, Object> userInput, ExecutionContext executionContext) {}
    NewConfigFunction<CONFIG> newConfigFunction();
    ComputationResultConsumer<ALGO, ALGO_RESULT, CONFIG, RESULT> computationResultConsumer();
    default ValidationConfiguration<CONFIG> validationConfig(ExecutionContext executionContext) {
        return ValidationConfiguration.empty();
    }
    default ProcedureExecutorSpec<ALGO, ALGO_RESULT, CONFIG> createDefaultExecutorSpec() {
        return new ProcedureExecutorSpec<>();
    }
    default boolean releaseProgressTask() { return true; }
}
```

**Rust Translation**:

```rust
use crate::types::GraphStore;
use super::{
    ComputationResult, ExecutionContext, ExecutionMode,
    ValidationConfiguration,
};

/// Algorithm Specification - Contract for algorithm implementations
///
/// This trait defines the interface that all algorithms must implement
/// to work with the ProcedureExecutor.
///
/// Algorithms live in `src/procedure/algo/` and implement this trait
/// to integrate with the GDSL Runtime.
pub trait AlgorithmSpec: Send + Sync {
    /// Algorithm output type
    type Output: Send + Sync;

    /// Algorithm name (for logging, catalog)
    fn name(&self) -> &str;

    /// Graph name to load
    fn graph_name(&self) -> &str;

    /// Projection hint for AdaptiveProjector
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Auto
    }

    /// Pre-process configuration (optional enhancement)
    fn preprocess_config(
        &self,
        _config: &mut serde_json::Value,
        _context: &ExecutionContext,
    ) -> Result<(), ConfigError> {
        Ok(())
    }

    /// Parse configuration from JSON
    fn parse_config(
        &self,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, ConfigError>;

    /// Get validation configuration
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    fn execute(
        &self,
        graph_store: &GraphStore,
        config: &serde_json::Value,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError>;

    /// Consume result and produce output
    ///
    /// **THIS IS WHERE TYPEVALIDATOR COMES IN!**
    /// Validate the result before returning.
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError>;

    /// Should release progress task? (default true)
    fn release_progress_task(&self) -> bool {
        true
    }
}

/// Projection hint for AdaptiveProjector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionHint {
    /// Let AdaptiveProjector decide
    Auto,
    /// Prefer dense array storage
    Dense,
    /// Prefer columnar Arrow storage
    Columnar,
    /// Prefer sparse HashMap storage
    Sparse,
    /// Prefer BSP/Pregel computation
    VertexCentric,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
}

#[derive(Debug, thiserror::Error)]
pub enum AlgorithmError {
    #[error("Execution failed: {0}")]
    Execution(String),
    #[error("Graph error: {0}")]
    Graph(String),
    #[error("Convergence failed: {0}")]
    Convergence(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    #[error("Consumption failed: {0}")]
    Failed(String),
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Mode not supported: {0:?}")]
    UnsupportedMode(ExecutionMode),
}
```

**Verification**: Mock implementation for testing

---

### Phase 5: Result Consumer (~100 lines, 2 hours)

#### File 6: result_consumer.rs (~100 lines, 2 hours)

**Java Source**: ComputationResultConsumer.java (25 lines)

**Rust Translation**:

```rust
use super::{ComputationResult, ExecutionMode, ConsumerError};

/// Stream results (return all)
pub fn stream_results<T: Clone>(
    result: ComputationResult<T>,
) -> Result<T, ConsumerError> {
    Ok(result.into_result())
}

/// Return summary statistics only
pub fn stats_only<T>(
    result: ComputationResult<T>,
) -> Result<StatsSummary, ConsumerError> {
    Ok(StatsSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        is_graph_empty: result.is_graph_empty(),
    })
}

#[derive(Debug, Clone)]
pub struct StatsSummary {
    pub compute_millis: u64,
    pub preprocess_millis: u64,
    pub is_graph_empty: bool,
}

/// Helper for consuming results based on mode
pub fn consume_by_mode<T: Clone>(
    result: ComputationResult<T>,
    mode: ExecutionMode,
) -> Result<ConsumerOutput<T>, ConsumerError> {
    match mode {
        ExecutionMode::Stream => {
            Ok(ConsumerOutput::Stream(stream_results(result)?))
        }
        ExecutionMode::Stats => {
            Ok(ConsumerOutput::Stats(stats_only(result)?))
        }
        _ => Err(ConsumerError::UnsupportedMode(mode)),
    }
}

#[derive(Debug)]
pub enum ConsumerOutput<T> {
    Stream(T),
    Stats(StatsSummary),
}
```

**Verification**: Mode-based consumption tests

---

### Phase 6: Main Orchestrator (6-8 hours)

#### File 7: executor.rs (~250 lines, 7 hours)

**Java Source**: ProcedureExecutor.java (~210 lines)

**Key Method**:

```java
public RESULT compute(String graphName, Map<String, Object> configuration) {
    // 1. Parse config
    // 2. Validate before load
    // 3. Load graph
    // 4. Validate after load
    // 5. Create algorithm
    // 6. Execute algorithm
    // 7. Consume result
}
```

**Rust Translation**:

```rust
use std::time::Instant;
use crate::projection::eval::{TypeValidator, AdaptiveProjector};
use crate::types::GraphStore;
use super::*;

/// Procedure Executor - GDSL Runtime for algorithm execution
///
/// This is the FIXED RUNTIME that orchestrates:
/// 1. Config parsing
/// 2. Two-phase validation
/// 3. Graph loading with projection
/// 4. Algorithm execution
/// 5. Result consumption with validation
pub struct ProcedureExecutor {
    context: ExecutionContext,
    mode: ExecutionMode,
}

impl ProcedureExecutor {
    pub fn new(context: ExecutionContext, mode: ExecutionMode) -> Self {
        Self { context, mode }
    }

    /// Execute an algorithm following the complete procedure lifecycle
    pub fn compute<A: AlgorithmSpec>(
        &mut self,
        algorithm: A,
        config_input: &serde_json::Value,
    ) -> Result<A::Output, ExecutorError> {
        self.context.log(LogLevel::Info, &format!(
            "Starting procedure: {} in mode {:?}",
            algorithm.name(),
            self.mode
        ));

        // 1. Pre-process configuration
        let mut config = config_input.clone();
        algorithm.preprocess_config(&mut config, &self.context)?;

        // 2. Parse configuration
        let parsed_config = algorithm.parse_config(&config)?;

        // 3. Get validation configuration
        let validation = algorithm.validation_config(&self.context);

        // 4. Before-load validation
        validation.validate_before_load(&parsed_config)?;

        // 5. Load and project graph
        let start = Instant::now();
        let graph_store = self.load_projected_graph(&algorithm, &parsed_config)?;
        let load_time = start.elapsed();
        self.context.record_timing("graph_load", load_time.as_millis() as u64);

        // 6. After-load validation
        validation.validate_after_load(&graph_store, &parsed_config)?;

        // 7. Execute algorithm
        let mut result = algorithm.execute(&graph_store, &parsed_config, &self.context)?;
        result = result.with_graph_store(graph_store);

        // 8. Consume result (TypeValidator validates here!)
        let output = algorithm.consume_result(result, &self.mode)?;

        self.context.log(LogLevel::Info, &format!(
            "Completed procedure: {}",
            algorithm.name()
        ));

        Ok(output)
    }

    /// Load graph with projection (TypeValidator + AdaptiveProjector)
    fn load_projected_graph<A: AlgorithmSpec>(
        &self,
        algorithm: &A,
        config: &serde_json::Value,
    ) -> Result<GraphStore, ExecutorError> {
        // Get base graph from context
        let base_graph = self.context.load_graph(algorithm.graph_name())?;

        // TypeValidator infers property descriptors
        let descriptors = TypeValidator::infer_from_graph(base_graph)?;

        self.context.log(LogLevel::Debug, &format!(
            "TypeValidator inferred {} property descriptors",
            descriptors.len()
        ));

        // AdaptiveProjector chooses optimal storage
        let projected = AdaptiveProjector::project(
            base_graph,
            &descriptors,
            algorithm.projection_hint(),
        )?;

        self.context.log(LogLevel::Debug, &format!(
            "AdaptiveProjector created projection with hint {:?}",
            algorithm.projection_hint()
        ));

        Ok(projected)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Context error: {0}")]
    Context(#[from] ContextError),

    #[error("Algorithm error: {0}")]
    Algorithm(#[from] AlgorithmError),

    #[error("Consumer error: {0}")]
    Consumer(#[from] ConsumerError),

    #[error("Projection error: {0}")]
    Projection(String),
}
```

**Verification**: Complete flow integration test

---

### Phase 7: Module Integration (~50 lines, 1 hour)

#### File 8: mod.rs (~50 lines, 1 hour)

```rust
//! Procedure Executor - GDSL Runtime for Algorithm Execution
//!
//! This is the **fixed GDSL Runtime** for procedure execution.
//! Part of the Projection system. Involved in NativeFactory codegen directly.

mod execution_mode;
mod computation_result;
mod execution_context;
mod validation_config;
mod algorithm_spec;
mod result_consumer;
mod executor;

pub use execution_mode::ExecutionMode;
pub use computation_result::ComputationResult;
pub use execution_context::{ExecutionContext, LogLevel, ContextError};
pub use validation_config::{
    ValidationConfiguration, BeforeLoadValidator, AfterLoadValidator,
    ValidationError,
};
pub use algorithm_spec::{
    AlgorithmSpec, ProjectionHint, ConfigError, AlgorithmError,
    ConsumerError,
};
pub use result_consumer::{
    stream_results, stats_only, consume_by_mode,
    ConsumerOutput, StatsSummary,
};
pub use executor::{ProcedureExecutor, ExecutorError};
```

**Verification**: Public API exports correctly

---

## File-by-File Mapping

| Java File                                 | Rust Module                        | Lines (Java) | Lines (Rust) | Status      |
| ----------------------------------------- | ---------------------------------- | ------------ | ------------ | ----------- |
| ExecutionMode.java                        | execution_mode.rs                  | 30           | 60           | ✅ Phase 1  |
| ComputationResult.java                    | computation_result.rs              | 60           | 150          | ✅ Phase 1  |
| ExecutionContext.java                     | execution_context.rs               | 187          | 120          | ✅ Phase 2  |
| ValidationConfiguration.java              | validation_config.rs               | 80           | 180          | ✅ Phase 3  |
| BeforeLoadValidation.java                 | (merged into validation_config.rs) | 30           | -            | ✅ Phase 3  |
| AfterLoadValidation.java                  | (merged into validation_config.rs) | 30           | -            | ✅ Phase 3  |
| Validator.java                            | (merged into validation_config.rs) | 60           | -            | ✅ Phase 3  |
| AlgorithmSpec.java                        | algorithm_spec.rs                  | 60           | 200          | ✅ Phase 4  |
| ComputationResultConsumer.java            | result_consumer.rs                 | 25           | 100          | ✅ Phase 5  |
| ProcedureExecutor.java                    | executor.rs                        | 210          | 250          | ✅ Phase 6  |
| -                                         | mod.rs                             | -            | 50           | ✅ Phase 7  |
| **SKIPPED**                               |                                    |              |              |             |
| ExecutorSpec.java                         | ❌ (fold into executor)            | 30           | -            | N/A         |
| ProcedureExecutorSpec.java                | ❌ (fold into executor)            | 70           | -            | N/A         |
| GraphCreation.java                        | ❌ (simplified)                    | 40           | -            | N/A         |
| GraphCreationFactory.java                 | ❌ (simplified)                    | 30           | -            | N/A         |
| ProcedureGraphCreation.java               | ❌ (not needed)                    | 95           | -            | N/A         |
| ProcedureGraphCreationFactory.java        | ❌ (not needed)                    | 50           | -            | N/A         |
| MemoryEstimationExecutor.java             | ❌ (future work)                   | 155          | -            | Future      |
| ProcedureMemoryEstimation.java            | ❌ (future work)                   | 55           | -            | Future      |
| GdsCallable.java                          | ❌ (Java reflection)               | 35           | -            | N/A         |
| GdsCallableFinder.java                    | ❌ (Java reflection)               | 200          | -            | N/A         |
| Preconditions.java                        | ❌ (optional)                      | 30           | -            | N/A         |
| AlgorithmSpecProgressTrackerProvider.java | ❌ (optional)                      | 40           | -            | N/A         |
| **TOTAL**                                 |                                    | **~2500**    | **1110**     | **7 files** |

---

## Implementation Timeline

### Week 1 (Days 1-3): Foundation

- **Day 1** (4-6h): Phase 1 - execution_mode.rs, computation_result.rs
- **Day 2** (3-4h): Phase 2 - execution_context.rs
- **Day 3** (4-5h): Phase 3 - validation_config.rs

### Week 2 (Days 4-6): Core Contract & Orchestration

- **Day 4** (5-6h): Phase 4 - algorithm_spec.rs
- **Day 5** (2h): Phase 5 - result_consumer.rs
- **Day 6** (6-8h): Phase 6 - executor.rs

### Week 2 (Day 7): Integration & Testing

- **Day 7** (2-3h): Phase 7 - mod.rs, integration tests

**Total Effort**: 18-24 hours over 7 days

---

## Testing Strategy

### Unit Tests (per module)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_mode_helpers() {
        assert!(ExecutionMode::Stream.returns_results());
        assert!(!ExecutionMode::Stats.returns_results());
        assert!(ExecutionMode::MutateNodeProperty.is_mutating());
    }

    #[test]
    fn test_computation_result_builder() {
        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_secs(1))
            .with_preprocess_time(Duration::from_millis(500))
            .mark_graph_empty();

        assert_eq!(result.compute_millis(), 1000);
        assert_eq!(result.preprocess_millis(), 500);
        assert!(result.is_graph_empty());
    }

    #[test]
    fn test_validation_chaining() {
        let validation = ValidationConfiguration::new()
            .add_before_load(RangeValidator {
                param: "maxIterations".to_string(),
                min: 1.0,
                max: 100.0,
            });

        let config = json!({"maxIterations": 50});
        assert!(validation.validate_before_load(&config).is_ok());

        let bad_config = json!({"maxIterations": 200});
        assert!(validation.validate_before_load(&bad_config).is_err());
    }
}
```

### Integration Test (complete flow)

```rust
// tests/procedure_executor_integration.rs

#[test]
fn test_complete_procedure_execution() {
    // Setup mock algorithm
    struct MockPageRank;

    impl AlgorithmSpec for MockPageRank {
        type Output = Vec<(u64, f64)>;

        fn name(&self) -> &str { "MockPageRank" }
        fn graph_name(&self) -> &str { "test_graph" }

        fn parse_config(&self, input: &serde_json::Value) -> Result<serde_json::Value, ConfigError> {
            Ok(input.clone())
        }

        fn execute(
            &self,
            graph_store: &GraphStore,
            _config: &serde_json::Value,
            _context: &ExecutionContext,
        ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
            let ranks = vec![(0, 0.15), (1, 0.85)];
            Ok(ComputationResult::new(ranks, Duration::from_secs(1)))
        }

        fn consume_result(
            &self,
            result: ComputationResult<Self::Output>,
            mode: &ExecutionMode,
        ) -> Result<Self::Output, ConsumerError> {
            // TypeValidator would validate here!
            Ok(result.into_result())
        }
    }

    // Create test graph
    let graph = create_test_graph();
    let context = ExecutionContext::mock(graph);

    // Create executor
    let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

    // Execute
    let config = json!({"maxIterations": 20});
    let result = executor.compute(MockPageRank, &config).unwrap();

    // Verify
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|(_, rank)| *rank > 0.0));
}
```

---

## Success Criteria

**Phase Complete When**:

1. ✅ All 7 Rust modules compile cleanly
2. ✅ Unit tests pass for each module (>90% coverage)
3. ✅ Integration test demonstrates complete flow
4. ✅ Mock algorithm can execute through executor
5. ✅ TypeValidator + AdaptiveProjector integration working
6. ✅ Error handling comprehensive
7. ✅ Public API documented

**Ready for Next Phase**:

- Algorithm implementations can use AlgorithmSpec trait
- PageRank can be implemented in `src/procedure/algo/pagerank.rs`
- Executor serves as fixed GDSL Runtime

---

## Notes & Deviations

**Simplifications Made**:

1. **No ExecutorSpec/ProcedureExecutorSpec**: Over-engineered, folded into ProcedureExecutor
2. **No Graph Creation abstraction**: Direct GraphStore access, simpler
3. **No Memory Estimation**: Future work, not MVP
4. **No GdsCallable/Finder**: Java reflection, not applicable to Rust
5. **Simplified ExecutionContext**: No Neo4j-specific dependencies

**Design Choices**:

1. **JSON for config**: Flexible, serializable, works with multiple config types
2. **Trait objects for validators**: Dynamic validation chain
3. **Generic AlgorithmSpec**: One trait for all algorithms
4. **Simple graph catalog**: HashMap instead of complex catalog system

**Future Work**:

- Memory estimation (MemoryEstimationExecutor)
- Progress tracking (AlgorithmSpecProgressTrackerProvider)
- More sophisticated graph catalog
- Model catalog integration (for ML procedures)

---

## ॐ Integration with Brahmachakra

**How This Completes the System**:

```
Procedure Execution Flow:
1. Parse config
2. Validate (before load)
3. Load graph:
   ├→ TypeValidator infers descriptors (Nāma from Rūpa)
   └→ AdaptiveProjector projects to storage (Maya's choice)
4. Validate (after load)
5. Execute algorithm
6. Consume result:
   └→ TypeValidator validates output (Brahman-knowing itself)
7. Return validated result
```

**The Brahmachakra Spins Through Procedures!**

---

**Status**: 🌟 Prakasa (Ready for Kriya)  
**Next Step**: Begin Phase 1 - execution_mode.rs + computation_result.rs

**ॐ तत्सत्**
