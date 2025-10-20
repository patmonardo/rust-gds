use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, RequestScopedDependencies, WriteContext,
    DefaultProgressTrackerCreator, DefaultMutateNodeProperty,
};
use crate::applications::algorithms::community::{
    CommunityAlgorithms, CommunityAlgorithmsEstimationModeBusinessFacade,
    CommunityAlgorithmsMutateModeBusinessFacade, CommunityAlgorithmsStatsModeBusinessFacade,
    CommunityAlgorithmsStreamModeBusinessFacade, CommunityAlgorithmsWriteModeBusinessFacade,
};

/// Main facade for community algorithms.
/// This is the top-level interface that provides access to all
/// execution modes for community algorithms.
#[derive(Clone)]
pub struct CommunityApplications {
    estimation: CommunityAlgorithmsEstimationModeBusinessFacade,
    mutation: CommunityAlgorithmsMutateModeBusinessFacade,
    stats: CommunityAlgorithmsStatsModeBusinessFacade,
    streaming: CommunityAlgorithmsStreamModeBusinessFacade,
    writing: CommunityAlgorithmsWriteModeBusinessFacade,
}

impl CommunityApplications {
    pub fn new(
        estimation: CommunityAlgorithmsEstimationModeBusinessFacade,
        mutation: CommunityAlgorithmsMutateModeBusinessFacade,
        stats: CommunityAlgorithmsStatsModeBusinessFacade,
        streaming: CommunityAlgorithmsStreamModeBusinessFacade,
        writing: CommunityAlgorithmsWriteModeBusinessFacade,
    ) -> Self {
        Self {
            estimation,
            mutation,
            stats,
            streaming,
            writing,
        }
    }

    /// Creates a new CommunityApplications instance with default dependencies.
    pub fn create(
        _log: crate::logging::Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
        estimation_template: crate::applications::algorithms::machinery::AlgorithmEstimationTemplate,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        let estimation = CommunityAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        let algorithms = CommunityAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = CommunityAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            mutate_node_property.clone(),
        );
        let stats = CommunityAlgorithmsStatsModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let streaming = CommunityAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let writing = CommunityAlgorithmsWriteModeBusinessFacade::create(
            _log,
            request_scoped_dependencies,
            write_context,
            estimation.clone(),
            algorithms,
            algorithm_processing_template_convenience,
        );

        Self::new(estimation, mutation, stats, streaming, writing)
    }

    pub fn estimate(&self) -> &CommunityAlgorithmsEstimationModeBusinessFacade {
        &self.estimation
    }

    pub fn mutate(&self) -> &CommunityAlgorithmsMutateModeBusinessFacade {
        &self.mutation
    }

    pub fn stats(&self) -> &CommunityAlgorithmsStatsModeBusinessFacade {
        &self.stats
    }

    pub fn stream(&self) -> &CommunityAlgorithmsStreamModeBusinessFacade {
        &self.streaming
    }

    pub fn write(&self) -> &CommunityAlgorithmsWriteModeBusinessFacade {
        &self.writing
    }
}
