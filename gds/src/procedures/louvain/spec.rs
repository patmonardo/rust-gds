use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainConfig {
    pub concurrency: usize,
}

impl Default for LouvainConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainResult {
    pub data: Vec<u64>,
}

pub struct LouvainAlgorithmSpec {
    graph_name: String,
}

impl LouvainAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
