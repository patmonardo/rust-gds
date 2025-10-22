//! KSpanningTree Specification

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeConfig {
    pub source_node: usize,
    pub k: u64,
    pub objective: String, // "min" or "max"
}

impl Default for KSpanningTreeConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            k: 1,
            objective: "min".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpanningTreeResult {
    pub parent: Vec<i64>,
    pub cost_to_parent: Vec<f64>,
    pub total_cost: f64,
    pub root: u64,
}

pub struct KSpanningTreeAlgorithmSpec {
    graph_name: String,
}

impl KSpanningTreeAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
