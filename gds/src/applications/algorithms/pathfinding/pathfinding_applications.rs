use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, RequestScopedDependencies, WriteContext,
    DefaultProgressTrackerCreator, DefaultMutateNodeProperty,
};
use crate::applications::algorithms::pathfinding::{
    PathfindingAlgorithms, PathfindingAlgorithmsEstimationModeBusinessFacade,
    PathfindingAlgorithmsMutateModeBusinessFacade, PathfindingAlgorithmsStatsModeBusinessFacade,
    PathfindingAlgorithmsStreamModeBusinessFacade, PathfindingAlgorithmsWriteModeBusinessFacade,
};

/// Main facade for pathfinding algorithms.
#[derive(Clone)]
pub struct PathfindingApplications {
    estimation: PathfindingAlgorithmsEstimationModeBusinessFacade,
    mutation: PathfindingAlgorithmsMutateModeBusinessFacade,
    stats: PathfindingAlgorithmsStatsModeBusinessFacade,
    streaming: PathfindingAlgorithmsStreamModeBusinessFacade,
    writing: PathfindingAlgorithmsWriteModeBusinessFacade,
}

impl PathfindingApplications {
    pub fn new(
        estimation: PathfindingAlgorithmsEstimationModeBusinessFacade,
        mutation: PathfindingAlgorithmsMutateModeBusinessFacade,
        stats: PathfindingAlgorithmsStatsModeBusinessFacade,
        streaming: PathfindingAlgorithmsStreamModeBusinessFacade,
        writing: PathfindingAlgorithmsWriteModeBusinessFacade,
    ) -> Self {
        Self { estimation, mutation, stats, streaming, writing }
    }

    pub fn create(
        log: crate::logging::Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
        estimation_template: crate::applications::algorithms::machinery::AlgorithmEstimationTemplate,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        let estimation = PathfindingAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        let algorithms = PathfindingAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = PathfindingAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            mutate_node_property.clone(),
        );
        let stats = PathfindingAlgorithmsStatsModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let streaming = PathfindingAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let writing = PathfindingAlgorithmsWriteModeBusinessFacade::create(
            log,
            request_scoped_dependencies,
            write_context,
            estimation.clone(),
            algorithms,
            algorithm_processing_template_convenience,
        );

        Self::new(estimation, mutation, stats, streaming, writing)
    }

    pub fn estimate(&self) -> &PathfindingAlgorithmsEstimationModeBusinessFacade { &self.estimation }
    pub fn mutate(&self) -> &PathfindingAlgorithmsMutateModeBusinessFacade { &self.mutation }
    pub fn stats(&self) -> &PathfindingAlgorithmsStatsModeBusinessFacade { &self.stats }
    pub fn stream(&self) -> &PathfindingAlgorithmsStreamModeBusinessFacade { &self.streaming }
    pub fn write(&self) -> &PathfindingAlgorithmsWriteModeBusinessFacade { &self.writing }
}
