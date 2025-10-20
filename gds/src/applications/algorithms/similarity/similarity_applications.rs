use crate::applications::algorithms::machinery::{
    AlgorithmEstimationTemplate, AlgorithmProcessingTemplateConvenience,
    RequestScopedDependencies, WriteContext, DefaultProgressTrackerCreator,
};
use crate::applications::algorithms::similarity::{
    SimilarityAlgorithms, SimilarityAlgorithmsEstimationModeBusinessFacade,
    SimilarityAlgorithmsMutateModeBusinessFacade, SimilarityAlgorithmsStatsModeBusinessFacade,
    SimilarityAlgorithmsStreamModeBusinessFacade, SimilarityAlgorithmsWriteModeBusinessFacade,
};
use crate::applications::algorithms::similarity::write_steps::WriteRelationshipService;
use crate::logging::Log;

/// Main facade for similarity algorithms.
/// This is the top-level interface that provides access to all
/// execution modes for similarity algorithms.
#[derive(Clone)]
pub struct SimilarityApplications {
    estimation_mode_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
    mutate_mode_facade: SimilarityAlgorithmsMutateModeBusinessFacade,
    stats_mode_facade: SimilarityAlgorithmsStatsModeBusinessFacade,
    stream_mode_facade: SimilarityAlgorithmsStreamModeBusinessFacade,
    write_mode_facade: SimilarityAlgorithmsWriteModeBusinessFacade,
}

impl SimilarityApplications {
    /// Creates a new SimilarityApplications instance with default dependencies.
    pub fn create(
        log: Log,
        request_scoped_dependencies: RequestScopedDependencies,
        algorithm_estimation_template: AlgorithmEstimationTemplate,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        write_context: WriteContext,
    ) -> Self {
        let estimation_mode_facade = SimilarityAlgorithmsEstimationModeBusinessFacade::new(
            algorithm_estimation_template.clone(),
        );
        
        let similarity_algorithms = SimilarityAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutate_mode_facade = SimilarityAlgorithmsMutateModeBusinessFacade::new(
            log.clone(),
            estimation_mode_facade.clone(),
            similarity_algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );

        let stats_mode_facade = SimilarityAlgorithmsStatsModeBusinessFacade::new(
            estimation_mode_facade.clone(),
            similarity_algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );

        let stream_mode_facade = SimilarityAlgorithmsStreamModeBusinessFacade::new(
            estimation_mode_facade.clone(),
            similarity_algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );

        let write_relationship_service = WriteRelationshipService::new();

        let write_mode_facade = SimilarityAlgorithmsWriteModeBusinessFacade::new(
            estimation_mode_facade.clone(),
            similarity_algorithms,
            algorithm_processing_template_convenience,
            write_relationship_service,
        );

        Self {
            estimation_mode_facade,
            mutate_mode_facade,
            stats_mode_facade,
            stream_mode_facade,
            write_mode_facade,
        }
    }

    /// Access to estimation mode operations
    pub fn estimate(&self) -> &SimilarityAlgorithmsEstimationModeBusinessFacade {
        &self.estimation_mode_facade
    }

    /// Access to mutate mode operations
    pub fn mutate(&self) -> &SimilarityAlgorithmsMutateModeBusinessFacade {
        &self.mutate_mode_facade
    }

    /// Access to stats mode operations
    pub fn stats(&self) -> &SimilarityAlgorithmsStatsModeBusinessFacade {
        &self.stats_mode_facade
    }

    /// Access to stream mode operations
    pub fn stream(&self) -> &SimilarityAlgorithmsStreamModeBusinessFacade {
        &self.stream_mode_facade
    }

    /// Access to write mode operations
    pub fn write(&self) -> &SimilarityAlgorithmsWriteModeBusinessFacade {
        &self.write_mode_facade
    }
}