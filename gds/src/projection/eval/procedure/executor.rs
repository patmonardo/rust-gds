//! Procedure Executor - GDSL Runtime orchestrator for algorithm execution
//!
//! Translated from: `org.neo4j.gds.executor.ProcedureExecutor`
//! Source: ProcedureExecutor.java (~210 lines)
//!
//! This is the **MAIN ORCHESTRATOR** of the GDSL Runtime.
//! It coordinates the complete procedure execution lifecycle:
//!
//! 1. Config preprocessing & parsing
//! 2. Two-phase validation (before/after load)
//! 3. Graph loading from catalog
//! 4. Algorithm execution with timing
//! 5. Result consumption & validation
//!
//! **Java GDS Pattern**:
//! - Complex generics: <ALGO, ALGO_RESULT, CONFIG, RESULT>
//! - Separate ExecutorSpec + AlgorithmSpec
//! - Builder pattern for ComputationResult
//! - Visitor pattern for AlgorithmFactory
//!
//! **rust-gds Simplification**:
//! - Single ProcedureExecutor struct
//! - AlgorithmSpec trait handles all algorithm concerns
//! - Direct execution without factory visitor
//! - Simpler error handling with Result types

use crate::types::prelude::GraphStore;
use serde_json::Value as JsonValue;
use std::time::Instant;

// Re-export from sibling modules and codegen
use super::execution_context::ContextError;
use super::validation_config::ValidationError;
use super::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, LogLevel,
};

/// Procedure Executor - GDSL Runtime for algorithm execution
///
/// Translated from: `ProcedureExecutor<ALGO, ALGO_RESULT, CONFIG, RESULT>`
///
/// **The Fixed GDSL Runtime**:
/// - Part of `src/projection/eval/procedure/` (GDSL Runtime)
/// - NOT part of `src/procedure/` (algorithm implementations)
/// - Involved in NativeFactory codegen directly
///
/// **Orchestration Flow**:
/// ```text
/// 1. preprocess_config()    → Enhance config with context
/// 2. parse_config()         → Parse & validate JSON
/// 3. validate_before_load() → Config-only validation
/// 4. load_graph()           → Get GraphStore from catalog
/// 5. validate_after_load()  → Config + graph validation
/// 6. execute_algorithm()    → Run algorithm with timing
/// 7. consume_result()       → Transform & validate output
/// ```
pub struct ProcedureExecutor {
    /// Execution context (graph catalog, logging, metrics)
    context: ExecutionContext,

    /// Execution mode (Stream, Stats, Train, Write, Mutate)
    mode: ExecutionMode,
}

impl ProcedureExecutor {
    /// Create a new procedure executor
    ///
    /// **Parameters**:
    /// - `context`: Runtime environment (catalog, logging, metrics)
    /// - `mode`: How to return results (Stream, Stats, etc.)
    pub fn new(context: ExecutionContext, mode: ExecutionMode) -> Self {
        Self { context, mode }
    }

    /// Execute an algorithm following the complete procedure lifecycle
    ///
    /// Translated from: `RESULT compute(String graphName, Map<String, Object> configuration)`
    ///
    /// **Java GDS Flow**:
    /// ```java
    /// public RESULT compute(String graphName, Map<String, Object> configuration) {
    ///     algoSpec.preProcessConfig(configuration, executionContext);
    ///     CONFIG config = executorSpec.configParser(...).processInput(configuration);
    ///     var validator = executorSpec.validator(algoSpec.validationConfig(...));
    ///     validator.validateConfigsBeforeLoad(graphProjectConfig, config);
    ///     graphStore = graphCreation.graphStore();
    ///     validator.validateConfigWithGraphStore(graphStore, graphProjectConfig, config);
    ///     ALGO algo = newAlgorithm(graph, graphStore, config);
    ///     ALGO_RESULT result = executeAlgorithm(builder, algo, ...);
    ///     return algoSpec.computationResultConsumer().consume(computationResult, ...);
    /// }
    /// ```
    ///
    /// **rust-gds Simplification**:
    /// - Single generic parameter `A: AlgorithmSpec`
    /// - Direct execution without factory/visitor
    /// - Result-based error handling
    pub fn compute<A: AlgorithmSpec>(
        &mut self,
        algorithm: &mut A,
        config_input: &JsonValue,
    ) -> Result<A::Output, ExecutorError> {
        let algo_name = algorithm.name().to_string();
        let graph_name = algorithm.graph_name().to_string();

        self.context.log(
            LogLevel::Info,
            &format!("Starting procedure: {} on graph: {}", algo_name, graph_name),
        );

        // Step 1: Preprocess configuration (ML pipelines, model params, etc.)
        let preprocess_start = Instant::now();
        let mut config = config_input.clone();
        algorithm.preprocess_config(&mut config, &self.context)?;
        let preprocess_time = preprocess_start.elapsed();

        self.context.log(
            LogLevel::Debug,
            &format!("Config preprocessing took: {:?}", preprocess_time),
        );

        // Step 2: Parse configuration
        let config = algorithm.parse_config(&config)?;

        self.context
            .log(LogLevel::Debug, &format!("Config parsed: {}", config));

        // Step 3: Get validation configuration
        let validation = algorithm.validation_config(&self.context);

        // Step 4: Validate BEFORE graph load (config only)
        validation.validate_before_load(&config)?;

        self.context
            .log(LogLevel::Debug, "Before-load validation passed");

        // Step 5: Load graph from catalog
        let load_start = Instant::now();
        let graph_store = self.context.load_graph(&graph_name)?;
        let load_time = load_start.elapsed();

        self.context.log(
            LogLevel::Info,
            &format!(
                "Graph loaded: {} nodes, {} rels ({}ms)",
                graph_store.node_count(),
                graph_store.relationship_count(),
                load_time.as_millis()
            ),
        );

        // Check for empty graph
        if graph_store.node_count() == 0 {
            self.context
                .log(LogLevel::Warn, "Graph is empty, skipping computation");

            let empty_result = ComputationResult::new(
                // Algorithm needs to handle empty case
                algorithm
                    .execute(graph_store.as_ref(), &config, &self.context)?
                    .into_result(),
                std::time::Duration::ZERO,
            )
            .with_preprocess_time(preprocess_time)
            .mark_graph_empty();

            return algorithm
                .consume_result(empty_result, &self.mode)
                .map_err(ExecutorError::Consumer);
        }

        // Step 6: Validate AFTER graph load (config + graph)
        validation.validate_after_load(graph_store.as_ref(), &config)?;

        self.context
            .log(LogLevel::Debug, "After-load validation passed");

        // Step 7: Execute algorithm with timing
        let compute_start = Instant::now();

        self.context.log(
            LogLevel::Info,
            &format!("Executing algorithm: {}", algo_name),
        );

        let computation_result = algorithm.execute(graph_store.as_ref(), &config, &self.context)?;
        let compute_time = compute_start.elapsed();

        self.context.log(
            LogLevel::Info,
            &format!(
                "Algorithm completed: {} (compute: {}ms, total: {}ms)",
                algo_name,
                compute_time.as_millis(),
                (preprocess_time + load_time + compute_time).as_millis()
            ),
        );

        // Record metrics
        self.context.record_timing(
            &format!("algorithm.{}", algo_name),
            compute_time.as_millis() as u64,
        );

        // Step 8: Consume result (transform, validate, return)
        let consume_start = Instant::now();
        let output = algorithm.consume_result(computation_result, &self.mode)?;
        let consume_time = consume_start.elapsed();

        self.context.log(
            LogLevel::Debug,
            &format!("Result consumption took: {}ms", consume_time.as_millis()),
        );

        self.context.log(
            LogLevel::Info,
            &format!("Procedure completed: {}", algo_name),
        );

        Ok(output)
    }

    /// Get a reference to the execution context
    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    /// Get a mutable reference to the execution context
    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }

    /// Get the execution mode
    pub fn mode(&self) -> ExecutionMode {
        self.mode
    }

    /// Set the execution mode
    pub fn set_mode(&mut self, mode: ExecutionMode) {
        self.mode = mode;
    }
}

/// Executor Error - Errors from procedure execution orchestration
///
/// Aggregates errors from all execution phases:
/// - Config parsing/validation
/// - Graph loading
/// - Algorithm execution
/// - Result consumption
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Context error: {0}")]
    Context(#[from] ContextError),

    #[error("Algorithm execution error: {0}")]
    Algorithm(#[from] AlgorithmError),

    #[error("Result consumption error: {0}")]
    Consumer(#[from] ConsumerError),

    #[error("Orchestration failed: {0}")]
    Orchestration(String),

    #[error("Empty graph: cannot execute algorithm on empty graph")]
    EmptyGraph,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::{DefaultGraphStore, RandomGraphConfig};
    use std::sync::Arc;
    use std::time::Duration;

    // Mock algorithm for testing
    struct TestAlgorithm {
        name: String,
        graph_name: String,
    }

    impl AlgorithmSpec for TestAlgorithm {
        type Output = Vec<u64>;

        fn name(&self) -> &str {
            &self.name
        }

        fn graph_name(&self) -> &str {
            &self.graph_name
        }

        fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
            Ok(input.clone())
        }

        fn execute<G: GraphStore>(
            &self,
            graph_store: &G,
            _config: &JsonValue,
            _context: &ExecutionContext,
        ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
            // Mock: return node count as result
            let result = vec![graph_store.node_count() as u64];
            Ok(ComputationResult::new(result, Duration::from_millis(100)))
        }

        fn consume_result(
            &self,
            result: ComputationResult<Self::Output>,
            mode: &ExecutionMode,
        ) -> Result<Self::Output, ConsumerError> {
            match mode {
                ExecutionMode::Stream => Ok(result.into_result()),
                _ => Err(ConsumerError::UnsupportedMode(*mode)),
            }
        }
    }

    fn create_test_context() -> ExecutionContext {
        let config = RandomGraphConfig::default().with_seed(42);
        let graph = Arc::new(DefaultGraphStore::random(&config).unwrap());
        ExecutionContext::mock(graph)
    }

    #[test]
    fn test_executor_creation() {
        let context = create_test_context();
        let executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        assert_eq!(executor.mode(), ExecutionMode::Stream);
    }

    #[test]
    fn test_executor_set_mode() {
        let context = create_test_context();
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        executor.set_mode(ExecutionMode::Stats);
        assert_eq!(executor.mode(), ExecutionMode::Stats);
    }

    #[test]
    fn test_compute_success() {
        let context = create_test_context();
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        let mut algorithm = TestAlgorithm {
            name: "test_algo".to_string(),
            graph_name: "test_graph".to_string(),
        };

        let config = serde_json::json!({});
        let result = executor.compute(&mut algorithm, &config);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compute_graph_not_found() {
        let context = ExecutionContext::new("test_user");
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        let mut algorithm = TestAlgorithm {
            name: "test_algo".to_string(),
            graph_name: "nonexistent".to_string(),
        };

        let config = serde_json::json!({});
        let result = executor.compute(&mut algorithm, &config);

        assert!(result.is_err());
        match result {
            Err(ExecutorError::Context(ContextError::GraphNotFound(name))) => {
                assert_eq!(name, "nonexistent");
            }
            _ => panic!("Expected GraphNotFound error"),
        }
    }

    #[test]
    fn test_compute_unsupported_mode() {
        let context = create_test_context();
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::WriteNodeProperty);

        let mut algorithm = TestAlgorithm {
            name: "test_algo".to_string(),
            graph_name: "test_graph".to_string(),
        };

        let config = serde_json::json!({});
        let result = executor.compute(&mut algorithm, &config);

        assert!(result.is_err());
        match result {
            Err(ExecutorError::Consumer(_)) => {} // Expected
            _ => panic!("Expected ConsumerError"),
        }
    }

    #[test]
    fn test_context_access() {
        let context = create_test_context();
        let username = context.username().to_string();

        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        assert_eq!(executor.context().username(), username);

        // Test mutable access
        executor.context_mut().set_log_level(LogLevel::Debug);
    }

    #[test]
    fn test_executor_error_conversion() {
        let config_err = ConfigError::MissingParameter("test".to_string());
        let executor_err: ExecutorError = config_err.into();
        assert!(matches!(executor_err, ExecutorError::Config(_)));

        let validation_err = ValidationError::Parameter("test".to_string());
        let executor_err: ExecutorError = validation_err.into();
        assert!(matches!(executor_err, ExecutorError::Validation(_)));
    }

    #[test]
    fn test_compute_with_validation() {
        use super::super::validation_config::{RangeValidator, ValidationConfiguration};

        struct ValidatedAlgorithm {
            name: String,
            graph_name: String,
        }

        impl AlgorithmSpec for ValidatedAlgorithm {
            type Output = Vec<u64>;

            fn name(&self) -> &str {
                &self.name
            }

            fn graph_name(&self) -> &str {
                &self.graph_name
            }

            fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
                Ok(input.clone())
            }

            fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
                ValidationConfiguration::new().add_before_load(RangeValidator::new(
                    "maxIterations",
                    1.0,
                    100.0,
                ))
            }

            fn execute<G: GraphStore>(
                &self,
                graph_store: &G,
                _config: &JsonValue,
                _context: &ExecutionContext,
            ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
                let result = vec![graph_store.node_count() as u64];
                Ok(ComputationResult::new(result, Duration::from_millis(50)))
            }

            fn consume_result(
                &self,
                result: ComputationResult<Self::Output>,
                mode: &ExecutionMode,
            ) -> Result<Self::Output, ConsumerError> {
                match mode {
                    ExecutionMode::Stream => Ok(result.into_result()),
                    _ => Err(ConsumerError::UnsupportedMode(*mode)),
                }
            }
        }

        let context = create_test_context();
        let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);

        let mut algorithm = ValidatedAlgorithm {
            name: "validated_algo".to_string(),
            graph_name: "test_graph".to_string(),
        };

        // Valid config
        let config = serde_json::json!({"maxIterations": 20});
        let result = executor.compute(&mut algorithm, &config);
        assert!(result.is_ok());

        // Invalid config (out of range)
        let config = serde_json::json!({"maxIterations": 200});
        let result = executor.compute(&mut algorithm, &config);
        assert!(result.is_err());
    }
}
