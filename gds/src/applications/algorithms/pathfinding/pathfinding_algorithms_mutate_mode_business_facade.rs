use crate::api::GraphName;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms::PathfindingAlgorithms;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms_estimation_mode_business_facade::PathfindingAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::applications::algorithms::machinery::DefaultMutateNodeProperty;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct PathfindingAlgorithmsMutateModeBusinessFacade {
    _estimation: PathfindingAlgorithmsEstimationModeBusinessFacade,
    _algorithms: PathfindingAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    _mutate_node_property: DefaultMutateNodeProperty,
}

impl PathfindingAlgorithmsMutateModeBusinessFacade {
    pub fn new(
        estimation: PathfindingAlgorithmsEstimationModeBusinessFacade,
        algorithms: PathfindingAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        Self { _estimation: estimation, _algorithms: algorithms, _algorithm_processing_template_convenience: algorithm_processing_template_convenience, _mutate_node_property: mutate_node_property }
    }

    pub fn shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::shortest_path::ShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement ShortestPath mutate mode")
    }

    pub fn all_pairs_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::all_pairs_shortest_path::AllPairsShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement AllPairsShortestPath mutate mode")
    }

    pub fn single_source_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::single_source_shortest_path::SingleSourceShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement SingleSourceShortestPath mutate mode")
    }

    pub fn yens_k_shortest_paths<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::yens_k_shortest_paths::YensKShortestPathsResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement YensKShortestPaths mutate mode")
    }
}
