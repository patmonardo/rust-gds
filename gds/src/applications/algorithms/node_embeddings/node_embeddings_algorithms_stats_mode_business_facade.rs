use crate::api::GraphName;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms::NodeEmbeddingsAlgorithms;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms_estimation_mode_business_facade::NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::config::base_types::Config;

/// Business facade for node embeddings algorithms in stats mode.
/// This provides statistics capabilities for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsAlgorithmsStatsModeBusinessFacade {
    _estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    _node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl NodeEmbeddingsAlgorithmsStatsModeBusinessFacade {
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

    /// Executes FastRP algorithm in stats mode.
    pub fn fast_rp<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StatsResultBuilder<crate::fast_rp::FastRPResult, RESULT>,
    ) -> RESULT {
        todo!("Implement FastRP stats mode")
    }

    /// Executes GraphSage algorithm in stats mode.
    pub fn graph_sage<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StatsResultBuilder<crate::graph_sage::GraphSageResult, RESULT>,
    ) -> RESULT {
        todo!("Implement GraphSage stats mode")
    }

    /// Executes Node2Vec algorithm in stats mode.
    pub fn node2vec<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StatsResultBuilder<crate::node2vec::Node2VecResult, RESULT>,
    ) -> RESULT {
        todo!("Implement Node2Vec stats mode")
    }

    /// Executes RandomWalk algorithm in stats mode.
    pub fn random_walk<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::StatsResultBuilder<crate::random_walk::RandomWalkResult, RESULT>,
    ) -> RESULT {
        todo!("Implement RandomWalk stats mode")
    }
}
