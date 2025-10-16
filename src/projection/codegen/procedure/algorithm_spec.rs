//! Algorithm Specification - Contract for algorithm implementations
//!
//! Translated from: `org.neo4j.gds.executor.AlgorithmSpec`
//! Source: AlgorithmSpec.java (~60 lines)
//!
//! This trait defines the interface that all algorithms must implement
//! to work with the ProcedureExecutor (GDSL Runtime).
//!
//! **Key Architecture**:
//! - Algorithms live in `src/procedure/algo/` (extensible content)
//! - AlgorithmSpec trait is part of GDSL Runtime (fixed infrastructure)
//! - Executor calls these methods to orchestrate algorithm execution

use crate::types::prelude::GraphStore;
use serde_json::Value as JsonValue;
use std::time::Duration;

// Import types from eval/procedure (execution runtime)
// NOTE: AlgorithmSpec moved to codegen/procedure but still needs runtime types
use crate::projection::eval::procedure::{
    ComputationResult, ExecutionContext, ExecutionMode, ValidationConfiguration,
};

/// Algorithm Specification - Contract for algorithm implementations
///
/// Translated from: `AlgorithmSpec<ALGO, ALGO_RESULT, CONFIG, RESULT, ALGO_FACTORY>`
///
/// **Java GDS Pattern**:
/// - Complex 5-type-parameter interface
/// - AlgorithmFactory creates Algorithm instances
/// - ComputationResultConsumer processes results
/// - NewConfigFunction parses config
///
/// **Rust Simplification**:
/// - Single trait with associated type `Output`
/// - No separate Factory (algorithms create themselves)
/// - No separate Consumer (consume_result method)
/// - Config parsing via parse_config method
///
/// **Usage Pattern**:
/// ```ignore
/// struct PageRankAlgorithm {
///     graph_name: String,
///     config: PageRankConfig,
/// }
///
/// impl AlgorithmSpec for PageRankAlgorithm {
///     type Output = Vec<(NodeId, f64)>;
///     
///     fn name(&self) -> &str { "pagerank" }
///     fn graph_name(&self) -> &str { &self.graph_name }
///     // ... implement other methods
/// }
/// ```
pub trait AlgorithmSpec: Send + Sync {
    /// Algorithm output type
    ///
    /// This is the raw result produced by the algorithm.
    /// Examples: Vec<(NodeId, f64)> for PageRank, Vec<Community> for Louvain
    type Output: Send + Sync;

    /// Algorithm name (for logging, catalog, error messages)
    ///
    /// Translated from: `String name()`
    fn name(&self) -> &str;

    /// Graph name to load from catalog
    ///
    /// **New in rust-gds**: Separates graph specification from config
    fn graph_name(&self) -> &str;

    /// Projection hint for AdaptiveProjector
    ///
    /// **New in rust-gds**: Guides storage backend selection
    /// - Dense: PageRank, NodeSimilarity (cursor iteration)
    /// - Columnar: BFS, export pipelines (zero-copy)
    /// - Sparse: Louvain, Label Propagation (HashMap-based)
    /// - Auto: Let AdaptiveProjector analyze and decide
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Auto
    }

    /// Pre-process configuration (optional enhancement)
    ///
    /// Translated from: `void preProcessConfig(Map<String, Object> userInput, ExecutionContext)`
    ///
    /// **Java GDS Use Case**: ML pipelines collect "missing" parameters from trained models
    ///
    /// **rust-gds Use Case**: Enhance config with defaults, context-specific values
    fn preprocess_config(
        &mut self,
        config: &mut JsonValue,
        _context: &ExecutionContext,
    ) -> Result<(), ConfigError> {
        // Default: no preprocessing
        let _ = config; // Suppress unused warning
        Ok(())
    }

    /// Parse configuration from JSON
    ///
    /// Translated from: `NewConfigFunction<CONFIG> newConfigFunction()`
    ///
    /// **Java GDS Pattern**: Returns a function that creates CONFIG from Map
    /// **rust-gds Pattern**: Direct method that parses and validates config
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError>;

    /// Get validation configuration
    ///
    /// Translated from: `ValidationConfiguration<CONFIG> validationConfig(ExecutionContext)`
    ///
    /// Returns validators for two-phase validation:
    /// - Before load: config-only validation
    /// - After load: config + graph validation
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    ///
    /// Translated from: Algorithm execution through AlgorithmFactory + Algorithm.compute()
    ///
    /// **Java GDS Pattern**:
    /// ```java
    /// ALGO_FACTORY factory = algorithmFactory(context);
    /// ALGO algo = factory.build(graph, config, ...);
    /// ALGO_RESULT result = algo.compute();
    /// ```
    ///
    /// **rust-gds Pattern**: Direct execution method
    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError>;

    /// Consume result and produce output
    ///
    /// Translated from: `ComputationResultConsumer<ALGO, ALGO_RESULT, CONFIG, RESULT>`
    ///
    /// **Java GDS Pattern**: Separate consumer interface
    /// **rust-gds Pattern**: Method on AlgorithmSpec
    ///
    /// **THIS IS WHERE TYPEVALIDATOR COMES IN!**
    /// Validate the result structure before returning to user.
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError>;

    /// Should release progress task? (default true)
    ///
    /// Translated from: `boolean releaseProgressTask()`
    ///
    /// **Java GDS**: Controls progress tracking cleanup
    /// **rust-gds**: Future use for progress tracking
    fn release_progress_task(&self) -> bool {
        true
    }
}

/// Projection Hint - Guides AdaptiveProjector storage selection
///
/// **New in rust-gds**: Helps executor choose optimal storage backend
///
/// Different algorithms have different access patterns:
/// - **Cursor iteration** (PageRank) → Dense arrays
/// - **Zero-copy export** (BFS to file) → Columnar Arrow
/// - **Sparse updates** (Louvain) → HashMap
/// - **Message passing** (Pregel) → Vertex-centric BSP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionHint {
    /// Let AdaptiveProjector decide based on graph density
    Auto,

    /// Prefer dense array storage (HugeArray)
    /// **Best for**: Cursor iteration, most nodes accessed
    Dense,

    /// Prefer columnar Arrow storage
    /// **Best for**: Zero-copy export, mmap-friendly, read-heavy
    Columnar,

    /// Prefer sparse HashMap storage
    /// **Best for**: Sparse graphs, selective access, many updates
    Sparse,

    /// Prefer BSP/Pregel vertex-centric computation
    /// **Best for**: Message-passing algorithms, distributed computation
    VertexCentric,
}

impl ProjectionHint {
    /// Get human-readable description
    pub fn description(&self) -> &str {
        match self {
            ProjectionHint::Auto => "automatic selection based on graph density",
            ProjectionHint::Dense => "dense array storage (cursor iteration)",
            ProjectionHint::Columnar => "columnar Arrow storage (zero-copy)",
            ProjectionHint::Sparse => "sparse HashMap storage (selective access)",
            ProjectionHint::VertexCentric => "vertex-centric BSP/Pregel computation",
        }
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Configuration Error - Errors from config parsing/validation
///
/// Translated from various config-related exceptions in Java GDS
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid value for {param}: {message}")]
    InvalidValue { param: String, message: String },

    #[error("Type mismatch for {param}: expected {expected}, got {actual}")]
    TypeMismatch {
        param: String,
        expected: String,
        actual: String,
    },

    #[error("Preprocessing failed: {0}")]
    Preprocessing(String),
}

/// Algorithm Error - Errors from algorithm execution
///
/// Translated from various algorithm execution exceptions
#[derive(Debug, thiserror::Error)]
pub enum AlgorithmError {
    #[error("Execution failed: {0}")]
    Execution(String),

    #[error("Graph error: {0}")]
    Graph(String),

    #[error("Convergence failed after {iterations} iterations: {message}")]
    Convergence { iterations: usize, message: String },

    #[error("Computation timeout after {duration:?}")]
    Timeout { duration: Duration },

    #[error("Invalid graph structure: {0}")]
    InvalidGraph(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

/// Consumer Error - Errors from result consumption
///
/// Translated from result consumer exceptions
#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    #[error("Consumption failed: {0}")]
    Failed(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Mode not supported: {0:?}")]
    UnsupportedMode(ExecutionMode),

    #[error("Result transformation failed: {0}")]
    Transformation(String),

    #[error("Output formatting failed: {0}")]
    Formatting(String),
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get required parameter from config
pub fn get_required_param<T: serde::de::DeserializeOwned>(
    config: &JsonValue,
    param: &str,
) -> Result<T, ConfigError> {
    let value = config
        .get(param)
        .ok_or_else(|| ConfigError::MissingParameter(param.to_string()))?;

    serde_json::from_value(value.clone()).map_err(|_| ConfigError::TypeMismatch {
        param: param.to_string(),
        expected: std::any::type_name::<T>().to_string(),
        actual: format!("{:?}", value),
    })
}

/// Get optional parameter from config with default
pub fn get_optional_param<T: serde::de::DeserializeOwned>(
    config: &JsonValue,
    param: &str,
    default: T,
) -> Result<T, ConfigError> {
    match config.get(param) {
        Some(value) => {
            serde_json::from_value(value.clone()).map_err(|_| ConfigError::TypeMismatch {
                param: param.to_string(),
                expected: std::any::type_name::<T>().to_string(),
                actual: format!("{:?}", value),
            })
        }
        None => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Mock algorithm for testing
    struct MockAlgorithm {
        name: String,
        graph_name: String,
        hint: ProjectionHint,
    }

    impl AlgorithmSpec for MockAlgorithm {
        type Output = Vec<u64>;

        fn name(&self) -> &str {
            &self.name
        }

        fn graph_name(&self) -> &str {
            &self.graph_name
        }

        fn projection_hint(&self) -> ProjectionHint {
            self.hint
        }

        fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
            // Simple mock: just return input
            Ok(input.clone())
        }

        fn execute<G: GraphStore>(
            &self,
            _graph_store: &G,
            _config: &JsonValue,
            _context: &ExecutionContext,
        ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
            // Mock execution
            Ok(ComputationResult::new(
                vec![1, 2, 3],
                Duration::from_millis(100),
            ))
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

    #[test]
    fn test_mock_algorithm_basic() {
        let algo = MockAlgorithm {
            name: "test_algo".to_string(),
            graph_name: "test_graph".to_string(),
            hint: ProjectionHint::Auto,
        };

        assert_eq!(algo.name(), "test_algo");
        assert_eq!(algo.graph_name(), "test_graph");
        assert_eq!(algo.projection_hint(), ProjectionHint::Auto);
        assert!(algo.release_progress_task());
    }

    #[test]
    fn test_projection_hints() {
        assert_eq!(ProjectionHint::Auto, ProjectionHint::Auto);
        assert_ne!(ProjectionHint::Dense, ProjectionHint::Sparse);

        // Test descriptions
        assert!(ProjectionHint::Auto.description().contains("automatic"));
        assert!(ProjectionHint::Dense.description().contains("dense"));
        assert!(ProjectionHint::Columnar.description().contains("columnar"));
        assert!(ProjectionHint::Sparse.description().contains("sparse"));
        assert!(ProjectionHint::VertexCentric
            .description()
            .contains("vertex"));
    }

    #[test]
    fn test_config_error_types() {
        let err = ConfigError::MissingParameter("maxIterations".to_string());
        assert!(err.to_string().contains("maxIterations"));

        let err = ConfigError::InvalidValue {
            param: "tolerance".to_string(),
            message: "negative value".to_string(),
        };
        assert!(err.to_string().contains("tolerance"));
        assert!(err.to_string().contains("negative"));
    }

    #[test]
    fn test_algorithm_error_types() {
        let err = AlgorithmError::Convergence {
            iterations: 100,
            message: "did not converge".to_string(),
        };
        assert!(err.to_string().contains("100"));
        assert!(err.to_string().contains("did not converge"));

        let err = AlgorithmError::Timeout {
            duration: Duration::from_secs(30),
        };
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_consumer_error_types() {
        let err = ConsumerError::UnsupportedMode(ExecutionMode::Train);
        assert!(err.to_string().contains("Train"));

        let err = ConsumerError::Validation("invalid result structure".to_string());
        assert!(err.to_string().contains("Validation"));
    }

    #[test]
    fn test_validation_config_empty() {
        let algo = MockAlgorithm {
            name: "test".to_string(),
            graph_name: "g".to_string(),
            hint: ProjectionHint::Auto,
        };

        let ctx = ExecutionContext::empty();
        let validation = algo.validation_config(&ctx);
        assert!(validation.is_empty());
    }

    #[test]
    fn test_parse_config() {
        let algo = MockAlgorithm {
            name: "test".to_string(),
            graph_name: "g".to_string(),
            hint: ProjectionHint::Auto,
        };

        let config = json!({"maxIterations": 20});
        let result = algo.parse_config(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_preprocess_config_default() {
        let mut algo = MockAlgorithm {
            name: "test".to_string(),
            graph_name: "g".to_string(),
            hint: ProjectionHint::Auto,
        };

        let mut config = json!({"param": "value"});
        let ctx = ExecutionContext::empty();

        // Default implementation does nothing
        let result = algo.preprocess_config(&mut config, &ctx);
        assert!(result.is_ok());
        assert_eq!(config, json!({"param": "value"}));
    }

    #[test]
    fn test_consume_result_stream() {
        let algo = MockAlgorithm {
            name: "test".to_string(),
            graph_name: "g".to_string(),
            hint: ProjectionHint::Auto,
        };

        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_millis(100));
        let output = algo.consume_result(result, &ExecutionMode::Stream);

        assert!(output.is_ok());
        assert_eq!(output.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_consume_result_unsupported_mode() {
        let algo = MockAlgorithm {
            name: "test".to_string(),
            graph_name: "g".to_string(),
            hint: ProjectionHint::Auto,
        };

        let result = ComputationResult::new(vec![1, 2, 3], Duration::from_millis(100));
        let output = algo.consume_result(result, &ExecutionMode::Stats);

        assert!(output.is_err());
        match output {
            Err(ConsumerError::UnsupportedMode(mode)) => {
                assert_eq!(mode, ExecutionMode::Stats);
            }
            _ => panic!("Expected UnsupportedMode error"),
        }
    }
}
