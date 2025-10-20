use crate::api::GraphName;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms::PathfindingAlgorithms;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms_estimation_mode_business_facade::PathfindingAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct PathfindingAlgorithmsStreamModeBusinessFacade {
    _estimation_facade: PathfindingAlgorithmsEstimationModeBusinessFacade,
    _pathfinding_algorithms: PathfindingAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl PathfindingAlgorithmsStreamModeBusinessFacade {
    pub fn new(
        estimation_facade: PathfindingAlgorithmsEstimationModeBusinessFacade,
        pathfinding_algorithms: PathfindingAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        Self { _estimation_facade: estimation_facade, _pathfinding_algorithms: pathfinding_algorithms, _algorithm_processing_template_convenience: algorithm_processing_template_convenience }
    }

    pub fn shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::shortest_path::ShortestPathResult, RESULT>) -> RESULT {
        todo!("Implement ShortestPath stream mode")
    }

    pub fn all_pairs_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::all_pairs_shortest_path::AllPairsShortestPathResult, RESULT>) -> RESULT {
        todo!("Implement AllPairsShortestPath stream mode")
    }

    pub fn single_source_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::single_source_shortest_path::SingleSourceShortestPathResult, RESULT>) -> RESULT {
        todo!("Implement SingleSourceShortestPath stream mode")
    }

    pub fn yens_k_shortest_paths<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::yens_k_shortest_paths::YensKShortestPathsResult, RESULT>) -> RESULT {
        todo!("Implement YensKShortestPaths stream mode")
    }
}
