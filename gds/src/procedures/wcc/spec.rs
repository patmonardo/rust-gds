//! WCC Algorithm Specification
use serde::{Deserialize, Serialize};
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::{AlgorithmSpec, AlgorithmError};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore as _;
use super::storage::WccStorageRuntime;
use super::computation::WccComputationRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccConfig {
    pub concurrency: usize,
}

impl Default for WccConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccResult {
    pub components: Vec<u64>,
    pub component_count: usize,
}

pub struct WccAlgorithmSpec {
    graph_name: String,
}

impl WccAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

define_algorithm_spec! {
    name: "wcc",
    output_type: WccResult,
    projection_hint: Dense,
    modes: [Stream, Stats],
    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: WccConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        // Create runtimes
        let storage = WccStorageRuntime::new(parsed_config.concurrency);
        let mut computation = WccComputationRuntime::new();

        // Undirected view over all types by default
        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        let result = storage.compute_wcc(&mut computation, graph_view.as_ref());
        Ok(WccResult { components: result.components, component_count: result.component_count })
    }
}
