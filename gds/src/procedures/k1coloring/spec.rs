//! K1Coloring Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringConfig {
    pub concurrency: usize,
    pub max_iterations: u64,
}

impl Default for K1ColoringConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_iterations: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K1ColoringResult {
    pub colors: Vec<u64>,
    pub ran_iterations: u64,
    pub did_converge: bool,
}

pub struct K1ColoringAlgorithmSpec {
    graph_name: String,
}

impl K1ColoringAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
