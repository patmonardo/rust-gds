//! PageRank Algorithm Specification
//!
//! This module implements the `AlgorithmSpec` trait for PageRank.
//! It is the **Species** manifestation of the abstract **Genus** (PageRank principle).

use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, LogLevel, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use crate::config::PageRankConfig;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;

use super::computation::PageRankComputationRuntime;
use super::storage::PageRankStorageRuntime;

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
pub struct PageRankAlgorithmSpec {
    /// Name of the graph to load
    graph_name: String,
}

impl PageRankAlgorithmSpec {
    /// Create a new PageRank algorithm specification
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
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
    /// Uses the config system's `PageRankConfig` builder pattern for validation.
    /// The config system handles validation (damping factor range, positive values, etc.).
    ///
    /// **Input JSON Format**:
    /// ```json
    /// {
    ///   "dampingFactor": 0.85,
    ///   "tolerance": 1e-6,
    ///   "maxIterations": 100,
    ///   "sourceNodes": ["node1", "node2"],
    ///   "weightProperty": "weight"
    /// }
    /// ```
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Extract fields manually (since PageRankConfig doesn't derive Deserialize from define_config!)
        let mut builder = PageRankConfig::builder();
        
        if let Some(df) = input.get("dampingFactor").and_then(|v| v.as_f64()) {
            builder = builder.damping_factor(df);
        }
        
        if let Some(tol) = input.get("tolerance").and_then(|v| v.as_f64()) {
            builder = builder.tolerance(tol);
        }
        
        if let Some(max_iter) = input.get("maxIterations").and_then(|v| v.as_u64()) {
            builder = builder.max_iterations(max_iter as usize);
        }
        
        if let Some(src_nodes) = input.get("sourceNodes").and_then(|v| v.as_array()) {
            let nodes: Vec<String> = src_nodes
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            builder = builder.source_nodes(Some(nodes));
        }

        // Build and validate using config system
        let config = builder.build().map_err(|e| ConfigError::InvalidValue {
            param: "config".to_string(),
            message: format!("Config validation failed: {}", e),
        })?;

        // Return validated config as JSON (with weight_property included)
        // Note: weight_property is not in PageRankConfig yet, extract separately
        let weight_property = input
            .get("weightProperty")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(json!({
            "dampingFactor": config.damping_factor,
            "tolerance": config.tolerance,
            "maxIterations": config.max_iterations,
            "sourceNodes": config.source_nodes,
            "weightProperty": weight_property,
        }))
    }

    /// Get validation configuration
    ///
    /// Returns validators for **two-phase validation**:
    ///
    /// 1. **Before-load validation** (config only):
    ///    - Range checks (damping factor, tolerance) → Handled by `PageRankConfig::builder().build()` validation
    ///    - Required parameters → Handled by config defaults
    ///    
    ///    Note: Most validation is already done by the config system in `parse_config()`,
    ///    so before-load validators are typically not needed here.
    ///
    /// 2. **After-load validation** (config + graph):
    ///    - TODO: Validate `weightProperty` exists (if specified)
    ///    - TODO: Validate `sourceNodes` exist in graph (if specified)
    ///    - Graph must have nodes (handled by executor)
    ///
    /// **Current Status**: Returns `empty()` because:
    /// - Config system already validates parameters in `parse_config()` via `PageRankConfig::builder().build()`
    /// - Graph-specific validators (weight property, source nodes) not yet implemented
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        // TODO: Add after-load validators:
        // - PropertyExistsValidator for weightProperty (if specified)
        // - NodeExistsValidator for sourceNodes (if specified)
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
        // Extract configuration values from parsed JSON
        let damping_factor = config
            .get("dampingFactor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.85);
        
        let tolerance = config
            .get("tolerance")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0000001);
        
        let max_iterations = config
            .get("maxIterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(20) as usize;
        
        // Extract weight_property separately (not yet in PageRankConfig)
        let weight_property = config
            .get("weightProperty")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Convert source_nodes from Vec<String> to Vec<u64>
        // Source nodes in config are strings (node identifiers), need to convert to IDs
        // TODO: This should use GraphStore's node ID resolution
        let source_nodes = config
            .get("sourceNodes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().and_then(|s| s.parse::<u64>().ok()))
                    .collect::<Vec<u64>>()
            });

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

        // Create storage runtime (Real pole - knows GraphStore)
        let storage = PageRankStorageRuntime::new(graph_store, source_nodes_clone, weight_property)?;

        // Create computation runtime (Ideal pole - knows PageRank scores)
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
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "pagerank");
    }

    #[test]
    fn test_pagerank_graph_name() {
        let spec = PageRankAlgorithmSpec::new("my_graph".to_string());
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_pagerank_projection_hint() {
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }

    #[test]
    fn test_pagerank_parse_config_valid() {
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string());

        let input = json!({
            "dampingFactor": 0.9,
            "tolerance": 1e-5,
            "maxIterations": 50,
            "sourceNodes": ["0", "1"],
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
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string());

        let input = json!({
            "dampingFactor": 1.5, // Invalid: > 1
        });

        let result = spec.parse_config(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_pagerank_parse_config_defaults() {
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string());

        let input = json!({}); // Empty config

        let result = spec.parse_config(&input);
        assert!(result.is_ok());
        let config = result.unwrap();
        // Config system defaults: dampingFactor=0.85, tolerance=0.0000001, maxIterations=20
        assert_eq!(config.get("dampingFactor").unwrap().as_f64().unwrap(), 0.85);
        assert_eq!(config.get("tolerance").unwrap().as_f64().unwrap(), 0.0000001);
        assert_eq!(config.get("maxIterations").unwrap().as_u64().unwrap(), 20);
    }
}