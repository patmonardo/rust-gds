use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, RequestScopedDependencies, WriteContext,
    DefaultProgressTrackerCreator, DefaultMutateNodeProperty,
};
use crate::applications::algorithms::node_embeddings::{
    NodeEmbeddingsAlgorithms, NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    NodeEmbeddingsAlgorithmsMutateModeBusinessFacade, NodeEmbeddingsAlgorithmsStatsModeBusinessFacade,
    NodeEmbeddingsAlgorithmsStreamModeBusinessFacade, NodeEmbeddingsAlgorithmsWriteModeBusinessFacade,
    NodeEmbeddingsAlgorithmsTrainModeBusinessFacade,
};

/// Main facade for node embeddings algorithms.
/// This is the top-level interface that provides access to all
/// execution modes for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsApplications {
    estimation: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    mutation: NodeEmbeddingsAlgorithmsMutateModeBusinessFacade,
    stats: NodeEmbeddingsAlgorithmsStatsModeBusinessFacade,
    streaming: NodeEmbeddingsAlgorithmsStreamModeBusinessFacade,
    writing: NodeEmbeddingsAlgorithmsWriteModeBusinessFacade,
    training: NodeEmbeddingsAlgorithmsTrainModeBusinessFacade,
}

impl NodeEmbeddingsApplications {
    pub fn new(
        estimation: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
        mutation: NodeEmbeddingsAlgorithmsMutateModeBusinessFacade,
        stats: NodeEmbeddingsAlgorithmsStatsModeBusinessFacade,
        streaming: NodeEmbeddingsAlgorithmsStreamModeBusinessFacade,
        writing: NodeEmbeddingsAlgorithmsWriteModeBusinessFacade,
        training: NodeEmbeddingsAlgorithmsTrainModeBusinessFacade,
    ) -> Self {
        Self {
            estimation,
            mutation,
            stats,
            streaming,
            writing,
            training,
        }
    }

    /// Creates a new NodeEmbeddingsApplications instance with default dependencies.
    pub fn create(
        log: crate::logging::Log,
        request_scoped_dependencies: RequestScopedDependencies,
        write_context: WriteContext,
        estimation_template: crate::applications::algorithms::machinery::AlgorithmEstimationTemplate,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        progress_tracker_creator: DefaultProgressTrackerCreator,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        let estimation = NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade::new(estimation_template);
        let algorithms = NodeEmbeddingsAlgorithms::new(
            progress_tracker_creator.clone(),
            request_scoped_dependencies.termination_flag().clone(),
        );

        let mutation = NodeEmbeddingsAlgorithmsMutateModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
            mutate_node_property.clone(),
        );
        let stats = NodeEmbeddingsAlgorithmsStatsModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let streaming = NodeEmbeddingsAlgorithmsStreamModeBusinessFacade::new(
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let writing = NodeEmbeddingsAlgorithmsWriteModeBusinessFacade::create(
            log,
            request_scoped_dependencies,
            write_context,
            estimation.clone(),
            algorithms.clone(),
            algorithm_processing_template_convenience.clone(),
        );
        let training = NodeEmbeddingsAlgorithmsTrainModeBusinessFacade::new(
            algorithms,
        );

        Self::new(estimation, mutation, stats, streaming, writing, training)
    }

    pub fn estimate(&self) -> &NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade {
        &self.estimation
    }

    pub fn mutate(&self) -> &NodeEmbeddingsAlgorithmsMutateModeBusinessFacade {
        &self.mutation
    }

    pub fn stats(&self) -> &NodeEmbeddingsAlgorithmsStatsModeBusinessFacade {
        &self.stats
    }

    pub fn stream(&self) -> &NodeEmbeddingsAlgorithmsStreamModeBusinessFacade {
        &self.streaming
    }

    pub fn write(&self) -> &NodeEmbeddingsAlgorithmsWriteModeBusinessFacade {
        &self.writing
    }

    pub fn train(&self) -> &NodeEmbeddingsAlgorithmsTrainModeBusinessFacade {
        &self.training
    }
}
