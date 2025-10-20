use crate::applications::algorithms::machine_learning::machine_learning_algorithms::MachineLearningAlgorithms;
use crate::applications::algorithms::machine_learning::machine_learning_algorithms_estimation_mode_business_facade::MachineLearningAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machine_learning::machine_learning_algorithms_mutate_mode_business_facade::MachineLearningAlgorithmsMutateModeBusinessFacade;
use crate::applications::algorithms::machine_learning::machine_learning_algorithms_stream_mode_business_facade::MachineLearningAlgorithmsStreamModeBusinessFacade;
use crate::applications::algorithms::machine_learning::machine_learning_algorithms_write_mode_business_facade::MachineLearningAlgorithmsWriteModeBusinessFacade;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, RequestScopedDependencies, WriteContext,
    DefaultProgressTrackerCreator,
};

/// Main facade for machine learning algorithms.
/// This is the top-level interface that provides access to all
/// execution modes for machine learning algorithms.
#[derive(Clone)]
pub struct MachineLearningApplications {
    estimation: MachineLearningAlgorithmsEstimationModeBusinessFacade,
    mutation: MachineLearningAlgorithmsMutateModeBusinessFacade,
    streaming: MachineLearningAlgorithmsStreamModeBusinessFacade,
    writing: MachineLearningAlgorithmsWriteModeBusinessFacade,
}

impl MachineLearningApplications {
    pub fn new(
        estimation: MachineLearningAlgorithmsEstimationModeBusinessFacade,
        mutation: MachineLearningAlgorithmsMutateModeBusinessFacade,
        streaming: MachineLearningAlgorithmsStreamModeBusinessFacade,
        writing: MachineLearningAlgorithmsWriteModeBusinessFacade,
    ) -> Self {
        Self {
            estimation,
            mutation,
            streaming,
            writing,
        }
    }

    /// Creates a new MachineLearningApplications instance with default dependencies.
    pub fn create(
        _log: crate::logging::Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        let estimation_template = crate::applications::algorithms::machinery::AlgorithmEstimationTemplate::new();
        let estimation = MachineLearningAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        
        let algorithms = MachineLearningAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = MachineLearningAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        
        let streaming = MachineLearningAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        
        let writing = MachineLearningAlgorithmsWriteModeBusinessFacade::new(
            estimation.clone(),
            algorithms,
            algorithm_processing_template_convenience,
        );

        Self::new(estimation, mutation, streaming, writing)
    }

    pub fn estimate(&self) -> &MachineLearningAlgorithmsEstimationModeBusinessFacade {
        &self.estimation
    }

    pub fn mutate(&self) -> &MachineLearningAlgorithmsMutateModeBusinessFacade {
        &self.mutation
    }

    pub fn stream(&self) -> &MachineLearningAlgorithmsStreamModeBusinessFacade {
        &self.streaming
    }

    pub fn write(&self) -> &MachineLearningAlgorithmsWriteModeBusinessFacade {
        &self.writing
    }
}
