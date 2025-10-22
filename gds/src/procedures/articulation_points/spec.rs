//! Articulation Points Algorithm Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticulationPointsConfig {
    pub concurrency: usize,
}

impl Default for ArticulationPointsConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticulationPointsResult {
    pub articulation_points: Vec<u64>,
}

pub struct ArticulationPointsAlgorithmSpec {
    graph_name: String,
}

impl ArticulationPointsAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
