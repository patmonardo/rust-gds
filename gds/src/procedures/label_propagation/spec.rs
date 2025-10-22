//! Label Propagation Specification

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropConfig {
    pub max_iterations: u64,
    pub concurrency: usize,
    pub seed_property: Option<String>,
    pub node_weight_property: Option<String>,
}

impl Default for LabelPropConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            concurrency: 4,
            seed_property: None,
            node_weight_property: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPropResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
}

pub struct LabelPropAlgorithmSpec {
    graph_name: String,
}

impl LabelPropAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
