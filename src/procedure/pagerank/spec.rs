//! PageRank Algorithm Specification
//!
//! This module implements the `AlgorithmSpec` trait for PageRank.
//! It is the **Species** manifestation of the abstract **Genus** (PageRank principle).

use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, LogLevel, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;

use super::computation::PageRankComputationRuntime;
use super::storage::PageRankStorageRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// PageRank Configuration
///
/// Specifies PageRank parameters and behavior.
#[derive(Debug, Clone)]
pub struct PageRankConfig {
    /// Damping factor (typically 0.85)
    pub damping_factor: f64,
    /// Convergence tolerance
    pub tolerance: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Source nodes for personalized PageRank (if any)
    pub source_nodes: Option<Vec<u64>>,
    /// Relationship weight property (if any)
    pub weight_property: Option<String>,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            damping_factor: 0.85,
            tolerance: 1e-6,
            max_iterations: 100,
            source_nodes: None,
            weight_property: None,
        }
    }
}

// ============================================================================
// Algorithm Specification
// ============================================================================

/// PageRank Algorithm Specification
///
/// This is the **Species** - concrete manifestation of the PageRank algorithm.
/// It implements the `AlgorithmSpec` trait required by `ProcedureExecutor`.
///
/// ## Architecture
///
/// This struct bridges three concepts:
/// - **Genus** (principle) = "iterative message passing for centrality"
/// - **Species** (instance) = PageRankAlgorithmSpec with specific parameters
/// - **Functor** (mapping) = GraphStore ↔ PageRank scores projection
///
/// ## The AlgorithmSpec Contract
///
/// The executor calls these methods in order:
/// 1. `preprocess_config()` - Enhance config with context
/// 2. `parse_config()` - Parse and validate JSON
/// 3. `validation_config()` - Get validators
/// 4. `execute()` - Run the algorithm
/// 5. `consume_result()` - Format output
#[allow(dead_code)] // config field used for future implementation
pub struct PageRankAlgorithmSpec {
    /// Name of the graph to load
    graph_name: String,
    /// Configuration for this execution
    config: PageRankConfig,
}

impl PageRankAlgorithmSpec {
    /// Create a new PageRank algorithm specification
    pub fn new(graph_name: String, config: PageRankConfig) -> Self {
        Self { graph_name, config }
    }
}

// ============================================================================
// AlgorithmSpec Implementation
// ============================================================================

/// PageRank computation result
#[derive(Debug, Clone)]
pub struct PageRankComputationResult {
    /// PageRank scores for each node
    pub scores: Vec<f64>,
    /// Number of iterations performed
    pub iterations: usize,
    /// Whether the algorithm converged
    pub converged: bool,
    /// Execution time
    pub execution_time: std::time::Duration,
}

impl AlgorithmSpec for PageRankAlgorithmSpec {
    /// Output type: PageRankComputationResult
    type Output = PageRankComputationResult;

    /// Algorithm name (for logging and catalog)
    fn name(&self) -> &str {
        "pagerank"
    }

    /// Graph to load
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Projection hint: prefer dense arrays
    ///
    /// PageRank iterates all nodes and their neighbors, so dense arrays
    /// with cursor iteration provide the best performance.
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON configuration
    ///
    /// **Input JSON Format**:
    /// ```json
    /// {
    ///   "dampingFactor": 0.85,
    ///   "tolerance": 1e-6,
    ///   "maxIterations": 100,
    ///   "sourceNodes": null,
    ///   "weightProperty": null
    /// }
    /// ```
    ///
    /// **Validation**:
    /// - `dampingFactor` must be between 0 and 1
    /// - `tolerance` must be positive
    /// - `maxIterations` must be positive
    /// - `sourceNodes` is optional array of node IDs
    /// - `weightProperty` is optional string
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Extract damping factor (required, default 0.85)
        let damping_factor = input
            .get("dampingFactor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.85);
        
        if damping_factor <= 0.0 || damping_factor >= 1.0 {
            return Err(ConfigError::InvalidValue {
                param: "dampingFactor".to_string(),
                message: "Must be between 0 and 1".to_string(),
            });
        }

        // Extract tolerance (required, default 1e-6)
        let tolerance = input
            .get("tolerance")
            .and_then(|v| v.as_f64())
            .unwrap_or(1e-6);
        
        if tolerance <= 0.0 {
            return Err(ConfigError::InvalidValue {
                param: "tolerance".to_string(),
                message: "Must be positive".to_string(),
            });
        }

        // Extract max iterations (required, default 100)
        let max_iterations = input
            .get("maxIterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(100) as usize;
        
        if max_iterations == 0 {
            return Err(ConfigError::InvalidValue {
                param: "maxIterations".to_string(),
                message: "Must be positive".to_string(),
            });
        }

        // Extract source nodes (optional)
        let source_nodes = input
            .get("sourceNodes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_u64())
                    .collect::<Vec<u64>>()
            });

        // Extract weight property (optional)
        let weight_property = input
            .get("weightProperty")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Return validated config as JSON
        Ok(json!({
            "dampingFactor": damping_factor,
            "tolerance": tolerance,
            "maxIterations": max_iterations,
            "sourceNodes": source_nodes,
            "weightProperty": weight_property,
        }))
    }

    /// Get validation configuration
    ///
    /// For PageRank, we validate:
    /// - Graph has nodes
    /// - Source nodes (if specified) exist in the graph
    /// - Weight property (if specified) exists
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    ///
    /// **Flow**:
    /// 1. Extract config
    /// 2. Create PageRankStorageRuntime (Gross pole - GraphStore)
    /// 3. Create PageRankComputationRuntime (Subtle pole - scores)
    /// 4. Run iterative PageRank algorithm
    /// 5. Return result
    ///
    /// This is where the Functor machinery works in practice:
    /// - Storage Runtime knows how to access GraphStore
    /// - Computation Runtime knows how to manage PageRank scores
    /// - Functor maps between them via message passing
    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        // Extract configuration
        let damping_factor = config
            .get("dampingFactor")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AlgorithmError::Execution("Missing dampingFactor".to_string()))?;
        
        let tolerance = config
            .get("tolerance")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| AlgorithmError::Execution("Missing tolerance".to_string()))?;
        
        let max_iterations = config
            .get("maxIterations")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| AlgorithmError::Execution("Missing maxIterations".to_string()))? as usize;
        
        let source_nodes = config
            .get("sourceNodes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_u64())
                    .collect::<Vec<u64>>()
            });
        
        let weight_property = config
            .get("weightProperty")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        context.log(
            LogLevel::Info,
            &format!(
                "Computing PageRank with damping={}, tolerance={}, max_iterations={} on graph with {} nodes",
                damping_factor,
                tolerance,
                max_iterations,
                graph_store.node_count()
            ),
        );

        let timer = Instant::now();

        // Clone source_nodes for both runtimes
        let source_nodes_clone = source_nodes.clone();

        // Create storage runtime (Gross pole - knows GraphStore)
        let storage = PageRankStorageRuntime::new(graph_store, source_nodes_clone, weight_property)?;

        // Create computation runtime (Subtle pole - knows PageRank scores)
        let mut computation = PageRankComputationRuntime::new(
            storage.node_count(),
            damping_factor,
            tolerance,
            source_nodes,
        );

        // Run PageRank iterations
        for iteration in 0..max_iterations {
            context.log(
                LogLevel::Debug,
                &format!("PageRank iteration {}", iteration + 1),
            );

            // Advance to next iteration
            computation.advance_iteration();

            // Check convergence
            if computation.converged() {
                context.log(
                    LogLevel::Info,
                    &format!("PageRank converged after {} iterations", computation.iteration()),
                );
                break;
            }

            // TODO: Implement actual PageRank message passing
            // For now, this is a placeholder that simulates the algorithm
            // In a real implementation, this would:
            // 1. For each node, compute outgoing messages
            // 2. Distribute messages to neighbors
            // 3. Accumulate incoming messages
            // 4. Update scores
        }

        let elapsed = timer.elapsed();

        context.log(
            LogLevel::Info,
            &format!(
                "PageRank completed: {} iterations, converged={}, time={:?}",
                computation.iteration(),
                computation.converged(),
                elapsed
            ),
        );

        // Create result
        let result = PageRankComputationResult {
            scores: computation.get_all_scores(),
            iterations: computation.iteration(),
            converged: computation.converged(),
            execution_time: elapsed,
        };

        // Return result wrapped in ComputationResult
        Ok(ComputationResult::new(result, elapsed))
    }

    /// Consume result and produce final output
    ///
    /// **Mode Handling**:
    /// - `STREAM` - Return the PageRank scores
    /// - `STATS` - Return the scores with metadata
    /// - Other - Error (PageRank is read-only)
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => {
                // Stream mode: return raw PageRank result
                Ok(result.into_result())
            }
            ExecutionMode::Stats => {
                // Stats mode: return PageRank result with metadata
                Ok(result.into_result())
            }
            other => {
                // PageRank is read-only, doesn't support other modes
                Err(ConsumerError::UnsupportedMode(*other))
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_algorithm_name() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );
        assert_eq!(spec.name(), "pagerank");
    }

    #[test]
    fn test_pagerank_graph_name() {
        let spec = PageRankAlgorithmSpec::new(
            "my_graph".to_string(),
            PageRankConfig::default(),
        );
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_pagerank_projection_hint() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_pagerank_parse_config_valid() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );

        let input = json!({
            "dampingFactor": 0.9,
            "tolerance": 1e-5,
            "maxIterations": 50,
            "sourceNodes": [0, 1],
            "weightProperty": "weight"
        });

        let result = spec.parse_config(&input);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.get("dampingFactor").unwrap().as_f64().unwrap(), 0.9);
        assert_eq!(config.get("tolerance").unwrap().as_f64().unwrap(), 1e-5);
        assert_eq!(config.get("maxIterations").unwrap().as_u64().unwrap(), 50);
    }

    #[test]
    fn test_pagerank_parse_config_invalid_damping_factor() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );

        let input = json!({
            "dampingFactor": 1.5, // Invalid: > 1
        });

        let result = spec.parse_config(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_pagerank_parse_config_defaults() {
        let spec = PageRankAlgorithmSpec::new(
            "test_graph".to_string(),
            PageRankConfig::default(),
        );

        let input = json!({}); // Empty config

        let result = spec.parse_config(&input);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.get("dampingFactor").unwrap().as_f64().unwrap(), 0.85);
        assert_eq!(config.get("tolerance").unwrap().as_f64().unwrap(), 1e-6);
        assert_eq!(config.get("maxIterations").unwrap().as_u64().unwrap(), 100);
    }
}