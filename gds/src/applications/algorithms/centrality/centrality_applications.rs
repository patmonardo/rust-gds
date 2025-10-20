use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, ProgressTrackerCreator, MutateNodeProperty,
    RequestScopedDependencies, WriteContext, DefaultProgressTrackerCreator, DefaultMutateNodeProperty,
};
use crate::applications::algorithms::centrality::{
    CentralityAlgorithms, CentralityAlgorithmsEstimationModeBusinessFacade,
    CentralityAlgorithmsMutateModeBusinessFacade, CentralityAlgorithmsStatsModeBusinessFacade,
    CentralityAlgorithmsStreamModeBusinessFacade, CentralityAlgorithmsWriteModeBusinessFacade,
};

/// Main facade for centrality algorithms.
/// This is the top-level interface that provides access to all
/// execution modes for centrality algorithms.
#[derive(Clone)]
pub struct CentralityApplications {
    estimation: CentralityAlgorithmsEstimationModeBusinessFacade,
    mutation: CentralityAlgorithmsMutateModeBusinessFacade,
    stats: CentralityAlgorithmsStatsModeBusinessFacade,
    streaming: CentralityAlgorithmsStreamModeBusinessFacade,
    writing: CentralityAlgorithmsWriteModeBusinessFacade,
}

impl CentralityApplications {
    pub fn new(
        estimation: CentralityAlgorithmsEstimationModeBusinessFacade,
        mutation: CentralityAlgorithmsMutateModeBusinessFacade,
        stats: CentralityAlgorithmsStatsModeBusinessFacade,
        streaming: CentralityAlgorithmsStreamModeBusinessFacade,
        writing: CentralityAlgorithmsWriteModeBusinessFacade,
    ) -> Self {
        Self {
            estimation,
            mutation,
            stats,
            streaming,
            writing,
        }
    }

    /// Creates a new CentralityApplications instance with default dependencies.
    pub fn create(
        log: crate::logging::Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
        estimation_template: crate::applications::algorithms::machinery::AlgorithmEstimationTemplate,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        let estimation = CentralityAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        let algorithms = CentralityAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let hits_hook_generator = crate::applications::algorithms::centrality::HitsHookGenerator::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = CentralityAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            mutate_node_property.clone(),
            hits_hook_generator.clone(),
        );

        let stats = CentralityAlgorithmsStatsModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            hits_hook_generator.clone(),
        );

        let streaming = CentralityAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            hits_hook_generator.clone(),
        );

        let writing = CentralityAlgorithmsWriteModeBusinessFacade::create(
            log,
            request_scoped_dependencies,
            write_context,
            estimation.clone(),
            algorithms,
            algorithm_processing_template_convenience,
            hits_hook_generator,
        );

        Self::new(estimation, mutation, stats, streaming, writing)
    }

    /// Returns the estimation mode facade.
    pub fn estimate(&self) -> &CentralityAlgorithmsEstimationModeBusinessFacade {
        &self.estimation
    }

    /// Returns the mutation mode facade.
    pub fn mutate(&self) -> &CentralityAlgorithmsMutateModeBusinessFacade {
        &self.mutation
    }

    /// Returns the stats mode facade.
    pub fn stats(&self) -> &CentralityAlgorithmsStatsModeBusinessFacade {
        &self.stats
    }

    /// Returns the streaming mode facade.
    pub fn stream(&self) -> &CentralityAlgorithmsStreamModeBusinessFacade {
        &self.streaming
    }

    /// Returns the writing mode facade.
    pub fn write(&self) -> &CentralityAlgorithmsWriteModeBusinessFacade {
        &self.writing
    }
}
