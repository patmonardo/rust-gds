//! Bridges Algorithm Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgesConfig {
    pub concurrency: usize,
}

impl Default for BridgesConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgesResult {
    pub bridges: Vec<(u64, u64)>,
}

pub struct BridgesAlgorithmSpec {
    graph_name: String,
}

impl BridgesAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
