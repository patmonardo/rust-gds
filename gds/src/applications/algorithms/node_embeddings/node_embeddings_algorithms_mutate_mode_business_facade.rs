use crate::api::GraphName;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms::NodeEmbeddingsAlgorithms;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms_estimation_mode_business_facade::NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::applications::algorithms::machinery::DefaultMutateNodeProperty;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

/// Business facade for node embeddings algorithms in mutate mode.
/// This provides mutation capabilities for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsAlgorithmsMutateModeBusinessFacade {
    _estimation: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    _algorithms: NodeEmbeddingsAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    _mutate_node_property: DefaultMutateNodeProperty,
}

impl NodeEmbeddingsAlgorithmsMutateModeBusinessFacade {
    pub fn new(
        estimation: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
        algorithms: NodeEmbeddingsAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        Self {
            _estimation: estimation,
            _algorithms: algorithms,
            _algorithm_processing_template_convenience: algorithm_processing_template_convenience,
            _mutate_node_property: mutate_node_property,
        }
    }

    /// Executes FastRP algorithm in mutate mode.
    pub fn fast_rp<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::fast_rp::FastRPResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement FastRP mutate mode")
    }

    /// Executes GraphSage algorithm in mutate mode.
    pub fn graph_sage<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::graph_sage::GraphSageResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement GraphSage mutate mode")
    }

    /// Executes Node2Vec algorithm in mutate mode.
    pub fn node2vec<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::node2vec::Node2VecResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Node2Vec mutate mode")
    }

    /// Executes RandomWalk algorithm in mutate mode.
    pub fn random_walk<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::random_walk::RandomWalkResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement RandomWalk mutate mode")
    }
}
