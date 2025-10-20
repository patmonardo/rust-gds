use crate::api::GraphName;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms::NodeEmbeddingsAlgorithms;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms_estimation_mode_business_facade::NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::config::base_types::Config;

/// Business facade for node embeddings algorithms in stream mode.
/// This provides streaming capabilities for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsAlgorithmsStreamModeBusinessFacade {
    _estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    _node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl NodeEmbeddingsAlgorithmsStreamModeBusinessFacade {
    pub fn new(
        estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
        node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        Self {
            _estimation_facade: estimation_facade,
            _node_embeddings_algorithms: node_embeddings_algorithms,
            _algorithm_processing_template_convenience: algorithm_processing_template_convenience,
        }
    }

    /// Executes FastRP algorithm in stream mode.
    pub fn fast_rp<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::fast_rp::FastRPResult, RESULT>,
    ) -> RESULT {
        todo!("Implement FastRP stream mode")
    }

    /// Executes GraphSage algorithm in stream mode.
    pub fn graph_sage<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::graph_sage::GraphSageResult, RESULT>,
    ) -> RESULT {
        todo!("Implement GraphSage stream mode")
    }

    /// Executes Node2Vec algorithm in stream mode.
    pub fn node2vec<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::node2vec::Node2VecResult, RESULT>,
    ) -> RESULT {
        todo!("Implement Node2Vec stream mode")
    }

    /// Executes RandomWalk algorithm in stream mode.
    pub fn random_walk<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StreamResultBuilder<crate::random_walk::RandomWalkResult, RESULT>,
    ) -> RESULT {
        todo!("Implement RandomWalk stream mode")
    }
}
