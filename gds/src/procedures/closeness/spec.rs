//! Closeness Centrality Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosenessCentralityConfig {
    pub concurrency: usize,
    pub wasserman_faust: bool,
}

impl Default for ClosenessCentralityConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            wasserman_faust: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosenessCentralityResult {
    pub centralities: Vec<f64>,
}

pub struct ClosenessCentralityAlgorithmSpec {
    graph_name: String,
}

impl ClosenessCentralityAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
