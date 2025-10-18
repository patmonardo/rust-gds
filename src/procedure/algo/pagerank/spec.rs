//! PageRank Algorithm Specification
//!
//! This module is a STUB while we clarify Executor/Algorithm architecture.
//!
//! BLOCKED: PageRank needs Pregel framework, but AlgorithmSpec::execute()
//! signature restricts to G: GraphStore, while Pregel needs G: Graph.
//!
//! See: doc/SESSION_CONTEXT_OCT_18_MODEL_FEATURE.md
//!
//! When Executor architecture is clarified, this will be re-implemented.

use crate::config::PageRankConfig;
use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::Value as JsonValue;

// ============================================================================
// Computation Result
// ============================================================================

/// Result of a PageRank execution
#[derive(Debug, Clone)]
pub struct PageRankComputationResult {
    /// Final scores indexed by node ID
    pub scores: Vec<f64>,
    /// Number of iterations executed
    pub iterations: usize,
    /// Whether convergence criterion was met
    pub converged: bool,
    /// Final residual error (change in last iteration)
    pub residual: f64,
    /// Execution time (milliseconds)
    pub execution_time_ms: u128,
}

// ============================================================================
// Algorithm Specification (STUB)
// ============================================================================

/// PageRank Algorithm Specification (STUB)
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

    /// Access the configuration
    pub fn config(&self) -> &PageRankConfig {
        &self.config
    }
}

// ============================================================================
// AlgorithmSpec Implementation (STUB - returns error for now)
// ============================================================================

impl AlgorithmSpec for PageRankAlgorithmSpec {
    /// Output type: PageRank scores
    type Output = PageRankComputationResult;

    /// Algorithm name (for logging and catalog)
    fn name(&self) -> &str {
        "pagerank"
    }

    /// Graph to load
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Projection hint: dense arrays with cursor iteration
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON configuration into PageRankConfig
    fn parse_config(&self, json: &JsonValue) -> Result<JsonValue, ConfigError> {
        Ok(json.clone())
    }

    /// Validation configuration (validators for input data)
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the PageRank algorithm
    ///
    /// STUB: Returns error until Executor/Algorithm architecture is clarified.
    /// Real implementation requires Pregel integration.
    fn execute<G>(
        &self,
        _graph_store: &G,
        _config: &JsonValue,
        _context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError>
    where
        G: GraphStore,
    {
        Err(AlgorithmError::Execution(
            "PageRank execution blocked: awaiting Executor/Algorithm architecture clarification. \
             See doc/SESSION_CONTEXT_OCT_18_MODEL_FEATURE.md"
                .to_string(),
        ))
    }

    /// Consume result and return formatted output
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        _mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        Ok(result.into_result())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_spec_name() {
        let spec = PageRankAlgorithmSpec::new("test_graph".to_string(), PageRankConfig::default());
        assert_eq!(spec.name(), "pagerank");
    }

    #[test]
    fn test_pagerank_spec_graph_name() {
        let spec = PageRankAlgorithmSpec::new("my_graph".to_string(), PageRankConfig::default());
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_pagerank_result_structure() {
        let result = PageRankComputationResult {
            scores: vec![0.1, 0.2, 0.3],
            iterations: 5,
            converged: true,
            residual: 0.00001,
            execution_time_ms: 42,
        };
        assert_eq!(result.scores.len(), 3);
        assert!(result.converged);
    }
}
