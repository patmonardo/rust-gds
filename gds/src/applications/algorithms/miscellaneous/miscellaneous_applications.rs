use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, RequestScopedDependencies, WriteContext,
    DefaultProgressTrackerCreator, DefaultMutateNodeProperty,
};
use crate::applications::algorithms::miscellaneous::{
    MiscellaneousAlgorithms, MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    MiscellaneousAlgorithmsMutateModeBusinessFacade, MiscellaneousAlgorithmsStatsModeBusinessFacade,
    MiscellaneousAlgorithmsStreamModeBusinessFacade, MiscellaneousAlgorithmsWriteModeBusinessFacade,
};

#[derive(Clone)]
pub struct MiscellaneousApplications {
    estimation: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    mutation: MiscellaneousAlgorithmsMutateModeBusinessFacade,
    stats: MiscellaneousAlgorithmsStatsModeBusinessFacade,
    streaming: MiscellaneousAlgorithmsStreamModeBusinessFacade,
    writing: MiscellaneousAlgorithmsWriteModeBusinessFacade,
}

impl MiscellaneousApplications {
    pub fn new(
        estimation: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
        mutation: MiscellaneousAlgorithmsMutateModeBusinessFacade,
        stats: MiscellaneousAlgorithmsStatsModeBusinessFacade,
        streaming: MiscellaneousAlgorithmsStreamModeBusinessFacade,
        writing: MiscellaneousAlgorithmsWriteModeBusinessFacade,
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
        let estimation = MiscellaneousAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        let algorithms = MiscellaneousAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = MiscellaneousAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            mutate_node_property.clone(),
        );
        let stats = MiscellaneousAlgorithmsStatsModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let streaming = MiscellaneousAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let writing = MiscellaneousAlgorithmsWriteModeBusinessFacade::create(
            log,
            request_scoped_dependencies,
            write_context,
            estimation.clone(),
            algorithms,
            algorithm_processing_template_convenience,
        );

        Self::new(estimation, mutation, stats, streaming, writing)
    }

    pub fn estimate(&self) -> &MiscellaneousAlgorithmsEstimationModeBusinessFacade { &self.estimation }
    pub fn mutate(&self) -> &MiscellaneousAlgorithmsMutateModeBusinessFacade { &self.mutation }
    pub fn stats(&self) -> &MiscellaneousAlgorithmsStatsModeBusinessFacade { &self.stats }
    pub fn stream(&self) -> &MiscellaneousAlgorithmsStreamModeBusinessFacade { &self.streaming }
    pub fn write(&self) -> &MiscellaneousAlgorithmsWriteModeBusinessFacade { &self.writing }
}
