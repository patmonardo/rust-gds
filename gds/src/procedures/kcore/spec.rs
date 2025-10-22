//! KCore Specification
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreConfig {
    pub concurrency: usize,
}

impl Default for KCoreConfig {
    fn default() -> Self {
        Self { concurrency: 4 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KCoreResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
}

pub struct KCoreAlgorithmSpec {
    graph_name: String,
}

impl KCoreAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
