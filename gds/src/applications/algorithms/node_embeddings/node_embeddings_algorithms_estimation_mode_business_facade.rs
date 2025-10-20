use crate::applications::algorithms::machinery::AlgorithmEstimationTemplate;
use crate::mem::MemoryEstimationResult;
use crate::config::base_types::Config;

/// Business facade for node embeddings algorithms in estimation mode.
/// This provides memory estimation capabilities for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade {
    _algorithm_estimation_template: AlgorithmEstimationTemplate,
}

impl NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade {
    pub fn new(algorithm_estimation_template: AlgorithmEstimationTemplate) -> Self {
        Self {
            _algorithm_estimation_template: algorithm_estimation_template,
        }
    }

    /// Estimates memory for FastRP algorithm.
    pub fn fast_rp<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement FastRP memory estimation")
    }

    /// Estimates memory for GraphSage algorithm.
    pub fn graph_sage<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement GraphSage memory estimation")
    }

    /// Estimates memory for Node2Vec algorithm.
    pub fn node2vec<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement Node2Vec memory estimation")
    }

    /// Estimates memory for RandomWalk algorithm.
    pub fn random_walk<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement RandomWalk memory estimation")
    }
}
