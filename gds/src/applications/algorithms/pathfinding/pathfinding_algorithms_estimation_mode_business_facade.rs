use crate::applications::algorithms::machinery::AlgorithmEstimationTemplate;
use crate::mem::MemoryEstimationResult;
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct PathfindingAlgorithmsEstimationModeBusinessFacade {
    _algorithm_estimation_template: AlgorithmEstimationTemplate,
}

impl PathfindingAlgorithmsEstimationModeBusinessFacade {
    pub fn new(algorithm_estimation_template: AlgorithmEstimationTemplate) -> Self {
        Self { _algorithm_estimation_template: algorithm_estimation_template }
    }

    pub fn shortest_path<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement ShortestPath memory estimation")
    }

    pub fn all_pairs_shortest_path<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement AllPairsShortestPath memory estimation")
    }

    pub fn single_source_shortest_path<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement SingleSourceShortestPath memory estimation")
    }

    pub fn yens_k_shortest_paths<C: Config>(&self, _config: &C) -> MemoryEstimationResult {
        todo!("Implement YensKShortestPaths memory estimation")
    }
}
