//! PageRank Algorithm Specification
//!
//! This module implements the `AlgorithmSpec` trait for PageRank iteration.
//! It is the **Species** manifestation of the abstract **Genus** (PageRank principle).
//!
//! ## Path Knowledge
//!
//! PageRank IS the Path by which network potential (Prajna) becomes network knowledge (Jnana):
//! - **Prajna pole**: Distributed edge weights and initial scores (infinite potential)
//! - **Jnana pole**: Aggregated node scores (infinite knowledge)
//! - **Dharma (Functor)**: Message-passing that relates them (the stroke of the Path)

use crate::config::PageRankConfig;
use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, ProjectionHint, ValidationConfiguration,
};
use crate::types::graph::Degrees;
use crate::types::prelude::GraphStore;
use crate::types::properties::relationship::traits::RelationshipIterator;
use serde_json::Value as JsonValue;
use std::time::Instant;

use super::computation::PageRankComputationRuntime;
use super::storage::PageRankStorageRuntime;

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
// Algorithm Specification
// ============================================================================

/// PageRank Algorithm Specification
///
/// This is the **Species** - concrete manifestation of the PageRank algorithm.
/// It implements the `AlgorithmSpec` trait required by `ProcedureExecutor`.
///
/// ## The Path
///
/// Each iteration of PageRank IS one breath of the Path:
/// 1. Validator apprehends current scores (what IS the form?)
/// 2. Projector reveals duality (storage scores ↔ message-passing)
/// 3. Functor converts (PropertyValues → computation)
/// 4. Computation: scores propagate via edges
/// 5. Return: new scores become next iteration's Prajna
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
// AlgorithmSpec Implementation
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
    ///
    /// PageRank needs efficient edge traversal and score updates,
    /// so dense arrays with cursor-based iteration are preferred.
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON configuration into PageRankConfig
    fn parse_config(&self, json: &JsonValue) -> Result<JsonValue, ConfigError> {
        // For now, echo back the JSON; full deserialization would happen here
        Ok(json.clone())
    }

    /// Validation configuration (validators for input data)
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the PageRank algorithm
    ///
    /// Follows Java GDS PageRankComputation.compute() logic via Pregel iteration.
    fn execute<G>(
        &self,
        graph_store: &G,
        _config: &JsonValue,
        _context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError>
    where
        G: GraphStore + Degrees + RelationshipIterator,
    {
        let start = Instant::now();

        let node_count = graph_store.node_count() as usize;
        if node_count == 0 {
            return Err(AlgorithmError::Execution("Graph has no nodes".to_string()));
        }

        // Initial score: alpha = (1 - dampingFactor)
        // This matches Java GDS: initialValue() returns alpha for source nodes
        let alpha = 1.0 - self.config.damping_factor;
        let mut current_scores = vec![alpha; node_count];
        let mut deltas = vec![alpha; node_count]; // Track deltas for convergence

        let storage = PageRankStorageRuntime::new(graph_store);
        let compute =
            PageRankComputationRuntime::new(self.config.damping_factor, self.config.tolerance);

        for iteration in 0..self.config.max_iterations {
            // Validate current scores
            storage.validate_scores(&current_scores)?;

            // Extract messages from all nodes based on current deltas
            // (In Java: sendToNeighbors(delta / degree))
            let incoming_messages = storage.extract_messages(&deltas)?;

            // Accumulate messages at each node
            let mut new_scores = vec![0.0; node_count];
            compute.accumulate_scores(&incoming_messages, &mut new_scores)?;

            // Apply damping: newScore = (1-d)*sum + d*initialScore
            // (In Java: setNodeValue(rank + delta) where delta = dampingFactor * sum)
            let mut new_deltas = vec![0.0; node_count];
            for node_id in 0..node_count {
                let dampened = self.config.damping_factor * new_scores[node_id];
                new_scores[node_id] = alpha + dampened;
                new_deltas[node_id] = dampened; // Delta for next iteration
            }

            // Check convergence: all deltas < tolerance
            let max_delta = new_deltas.iter().fold(0.0f64, |a, &b| a.max(b.abs()));

            if iteration > 0 && max_delta < self.config.tolerance {
                let result = PageRankComputationResult {
                    scores: new_scores,
                    iterations: iteration + 1,
                    converged: true,
                    residual: max_delta,
                    execution_time_ms: start.elapsed().as_millis(),
                };
                return Ok(ComputationResult::new(result, start.elapsed()));
            }

            current_scores = new_scores;
            deltas = new_deltas;
        }

        let result = PageRankComputationResult {
            scores: current_scores,
            iterations: self.config.max_iterations,
            converged: false,
            residual: 0.0,
            execution_time_ms: start.elapsed().as_millis(),
        };

        Ok(ComputationResult::new(result, start.elapsed()))
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
