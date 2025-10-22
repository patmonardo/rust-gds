//! Triangle Count Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleCountConfig {
    pub concurrency: usize,
    pub max_degree: u64,
}

impl Default for TriangleCountConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_degree: u64::MAX,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleCountResult {
    pub local_triangles: Vec<u64>,
    pub global_triangles: u64,
}

pub struct TriangleCountAlgorithmSpec {
    graph_name: String,
}

impl TriangleCountAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
