//! Betweenness Centrality Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetweennessCentralityConfig {
    pub concurrency: usize,
}

impl Default for BetweennessCentralityConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetweennessCentralityResult {
    pub centralities: Vec<f64>,
}

pub struct BetweennessCentralityAlgorithmSpec {
    graph_name: String,
}

impl BetweennessCentralityAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
