use crate::api::GraphName;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms::PathfindingAlgorithms;
use crate::applications::algorithms::pathfinding::pathfinding_algorithms_estimation_mode_business_facade::PathfindingAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::applications::algorithms::machinery::DefaultWriteToDatabase;
use crate::applications::algorithms::machinery::RequestScopedDependencies;
use crate::applications::algorithms::machinery::WriteContext;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct PathfindingAlgorithmsWriteModeBusinessFacade {
    _estimation_facade: PathfindingAlgorithmsEstimationModeBusinessFacade,
    _pathfinding_algorithms: PathfindingAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    _write_to_database: DefaultWriteToDatabase,
}

impl PathfindingAlgorithmsWriteModeBusinessFacade {
    fn new(
        estimation_facade: PathfindingAlgorithmsEstimationModeBusinessFacade,
        pathfinding_algorithms: PathfindingAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        write_to_database: DefaultWriteToDatabase,
    ) -> Self {
        Self { _estimation_facade: estimation_facade, _pathfinding_algorithms: pathfinding_algorithms, _algorithm_processing_template_convenience: algorithm_processing_template_convenience, _write_to_database: write_to_database }
    }

    pub fn create(
        _log: crate::logging::Log,
        _request_scoped_dependencies: RequestScopedDependencies,
        _write_context: WriteContext,
        estimation_facade: PathfindingAlgorithmsEstimationModeBusinessFacade,
        pathfinding_algorithms: PathfindingAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        let write_to_database = DefaultWriteToDatabase::new();
        Self::new(estimation_facade, pathfinding_algorithms, algorithm_processing_template_convenience, write_to_database)
    }

    pub fn shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::shortest_path::ShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement ShortestPath write mode")
    }

    pub fn all_pairs_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::all_pairs_shortest_path::AllPairsShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement AllPairsShortestPath write mode")
    }

    pub fn single_source_shortest_path<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::single_source_shortest_path::SingleSourceShortestPathResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement SingleSourceShortestPath write mode")
    }

    pub fn yens_k_shortest_paths<C: Config, RESULT>(&self, _graph_name: GraphName, _config: C, _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::yens_k_shortest_paths::YensKShortestPathsResult, RESULT, NodePropertiesWritten>) -> RESULT {
        todo!("Implement YensKShortestPaths write mode")
    }
}
