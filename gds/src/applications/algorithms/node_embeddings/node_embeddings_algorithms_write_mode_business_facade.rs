use crate::api::GraphName;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms::NodeEmbeddingsAlgorithms;
use crate::applications::algorithms::node_embeddings::node_embeddings_algorithms_estimation_mode_business_facade::NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade;
use crate::applications::algorithms::machinery::algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
use crate::applications::algorithms::machinery::DefaultWriteToDatabase;
use crate::applications::algorithms::machinery::RequestScopedDependencies;
use crate::applications::algorithms::machinery::WriteContext;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

/// Business facade for node embeddings algorithms in write mode.
/// This provides write capabilities for node embeddings algorithms.
#[derive(Clone)]
pub struct NodeEmbeddingsAlgorithmsWriteModeBusinessFacade {
    _estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
    _node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    _write_to_database: DefaultWriteToDatabase,
}

impl NodeEmbeddingsAlgorithmsWriteModeBusinessFacade {
    fn new(
        estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
        node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        write_to_database: DefaultWriteToDatabase,
    ) -> Self {
        Self {
            _estimation_facade: estimation_facade,
            _node_embeddings_algorithms: node_embeddings_algorithms,
            _algorithm_processing_template_convenience: algorithm_processing_template_convenience,
            _write_to_database: write_to_database,
        }
    }

    pub fn create(
        _log: crate::logging::Log,
        _request_scoped_dependencies: RequestScopedDependencies,
        _write_context: WriteContext,
        estimation_facade: NodeEmbeddingsAlgorithmsEstimationModeBusinessFacade,
        node_embeddings_algorithms: NodeEmbeddingsAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        let write_to_database = DefaultWriteToDatabase::new();

        Self::new(
            estimation_facade,
            node_embeddings_algorithms,
            algorithm_processing_template_convenience,
            write_to_database,
        )
    }

    /// Executes FastRP algorithm in write mode.
    pub fn fast_rp<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::fast_rp::FastRPResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement FastRP write mode")
    }

    /// Executes GraphSage algorithm in write mode.
    pub fn graph_sage<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::graph_sage::GraphSageResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement GraphSage write mode")
    }

    /// Executes Node2Vec algorithm in write mode.
    pub fn node2vec<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::node2vec::Node2VecResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Node2Vec write mode")
    }

    /// Executes RandomWalk algorithm in write mode.
    pub fn random_walk<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl crate::applications::algorithms::machinery::ResultBuilder<C, crate::random_walk::RandomWalkResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement RandomWalk write mode")
    }
}
