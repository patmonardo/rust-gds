//! Sum Algorithm Specification
//!
//! This module implements the `AlgorithmSpec` trait for Sum aggregation.
//! It demonstrates the Storage ↔ Computation Functor machinery.

use crate::projection::eval::procedure::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, LogLevel, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;

use super::computation::SumComputationRuntime;
use super::storage::SumStorageRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Sum Aggregation Configuration
///
/// Specifies what property to sum and how to handle the output.
#[derive(Debug, Clone)]
pub struct SumConfig {
    /// The property key to aggregate
    pub property_key: String,
    /// Optional weight property (for weighted sum)
    pub weight_property: Option<String>,
}

// ============================================================================
// Algorithm Specification
// ============================================================================

/// Sum Aggregation Algorithm Specification
///
/// This implements the `AlgorithmSpec` trait required by `ProcedureExecutor`.
pub struct SumAlgorithmSpec {
    /// Name of the graph to load
    graph_name: String,
    /// Configuration for this execution
    config: SumConfig,
}

impl SumAlgorithmSpec {
    /// Create a new Sum algorithm specification
    pub fn new(graph_name: String, config: SumConfig) -> Self {
        Self { graph_name, config }
    }
}

// ============================================================================
// AlgorithmSpec Implementation
// ============================================================================

impl AlgorithmSpec for SumAlgorithmSpec {
    /// Output type: f64 (the sum value)
    type Output = f64;

    /// Algorithm name (for logging and catalog)
    fn name(&self) -> &str {
        "sum"
    }

    /// Graph to load
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Projection hint: prefer dense arrays
    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    /// Parse JSON configuration
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Extract property_key (required)
        let property_key = input
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConfigError::MissingParameter("property_key".to_string()))?
            .to_string();

        // Extract weight_property (optional)
        let weight_property = input
            .get("weight_property")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Return validated config as JSON
        Ok(json!({
            "property_key": property_key,
            "weight_property": weight_property,
        }))
    }

    /// Get validation configuration
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    /// Execute the algorithm
    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        // Extract configuration
        let property_key = config
            .get("property_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing property_key".to_string()))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Computing sum for property: {} on graph with {} nodes",
                property_key,
                graph_store.node_count()
            ),
        );

        let timer = Instant::now();

        // Create storage runtime (Gross pole - knows PropertyValues)
        let storage = SumStorageRuntime::new(graph_store, property_key)?;

        // Create computation runtime (Subtle pole - knows accumulation)
        let mut computation = SumComputationRuntime::new();

        // Iterate all nodes and accumulate
        let node_count = graph_store.node_count();
        for node_id in 0..node_count as u32 {
            // **FUNCTOR IN ACTION**:
            // Project from Storage (Gross/PropertyValues)
            // to Computation (Subtle/accumulation)
            let value = storage.get_node_value(node_id)?;
            computation.add_value(value);
        }

        let elapsed = timer.elapsed();

        context.log(
            LogLevel::Info,
            &format!(
                "Sum computed: {} (over {} nodes in {:?})",
                computation.sum(),
                computation.count(),
                elapsed
            ),
        );

        // Return result wrapped in ComputationResult
        Ok(ComputationResult::new(computation.sum(), elapsed))
    }

    /// Consume result and produce final output
    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => Ok(result.into_result()),
            ExecutionMode::Stats => Ok(result.into_result()),
            other => Err(ConsumerError::UnsupportedMode(*other)),
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
    fn test_sum_algorithm_name() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.name(), "sum");
    }

    #[test]
    fn test_sum_graph_name() {
        let spec = SumAlgorithmSpec::new(
            "my_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.graph_name(), "my_graph");
    }

    #[test]
    fn test_sum_projection_hint() {
        let spec = SumAlgorithmSpec::new(
            "test_graph".to_string(),
            SumConfig {
                property_key: "value".to_string(),
                weight_property: None,
            },
        );
        assert_eq!(spec.projection_hint(), ProjectionHint::Dense);
    }
}
