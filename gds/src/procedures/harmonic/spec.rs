//! Harmonic Centrality Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonicConfig {
    pub concurrency: usize,
}

impl Default for HarmonicConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonicResult {
    pub centralities: Vec<f64>,
}

pub struct HarmonicAlgorithmSpec {
    graph_name: String,
}

impl HarmonicAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
