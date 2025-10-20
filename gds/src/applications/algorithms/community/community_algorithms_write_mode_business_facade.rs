use crate::api::GraphName;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, WriteToDatabase, ResultBuilder,
    RequestScopedDependencies, WriteContext,
};
use crate::applications::algorithms::community::{
    CommunityAlgorithms, CommunityAlgorithmsEstimationModeBusinessFacade,
};
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

use crate::applications::algorithms::machinery::DefaultWriteToDatabase;

/// Business facade for community algorithms in write mode.
/// This provides write capabilities for community algorithms.
#[derive(Clone)]
pub struct CommunityAlgorithmsWriteModeBusinessFacade {
    _estimation_facade: CommunityAlgorithmsEstimationModeBusinessFacade,
    _community_algorithms: CommunityAlgorithms,
    _algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    _write_to_database: DefaultWriteToDatabase,
}

impl CommunityAlgorithmsWriteModeBusinessFacade {
    fn new(
        estimation_facade: CommunityAlgorithmsEstimationModeBusinessFacade,
        community_algorithms: CommunityAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        write_to_database: DefaultWriteToDatabase,
    ) -> Self {
        Self {
            _estimation_facade: estimation_facade,
            _community_algorithms: community_algorithms,
            _algorithm_processing_template_convenience: algorithm_processing_template_convenience,
            _write_to_database: write_to_database,
        }
    }

    pub fn create(
        _log: crate::logging::Log,
        _request_scoped_dependencies: RequestScopedDependencies,
        _write_context: WriteContext,
        estimation_facade: CommunityAlgorithmsEstimationModeBusinessFacade,
        community_algorithms: CommunityAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        let write_to_database = DefaultWriteToDatabase::new();

        Self::new(
            estimation_facade,
            community_algorithms,
            algorithm_processing_template_convenience,
            write_to_database,
        )
    }

    /// Executes Louvain algorithm in write mode.
    pub fn louvain<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::louvain::LouvainResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Louvain write mode")
    }

    /// Executes Leiden algorithm in write mode.
    pub fn leiden<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::leiden::LeidenResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Leiden write mode")
    }

    /// Executes Label Propagation algorithm in write mode.
    pub fn label_propagation<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::label_propagation::LabelPropagationResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Label Propagation write mode")
    }

    /// Executes Speaker Listener LPA algorithm in write mode.
    pub fn speaker_listener_lpa<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::speaker_listener_lpa::SpeakerListenerLPAResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Speaker Listener LPA write mode")
    }

    /// Executes Modularity Optimization algorithm in write mode.
    pub fn modularity_optimization<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::modularity_optimization::ModularityOptimizationResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Modularity Optimization write mode")
    }

    /// Executes Triangle Count algorithm in write mode.
    pub fn triangle_count<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::triangle_count::TriangleCountResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Triangle Count write mode")
    }

    /// Executes K-Core algorithm in write mode.
    pub fn k_core<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::k_core::KCoreResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement K-Core write mode")
    }

    /// Executes WCC algorithm in write mode.
    pub fn wcc<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::wcc::WccResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement WCC write mode")
    }

    /// Executes SCC algorithm in write mode.
    pub fn scc<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::scc::SccResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement SCC write mode")
    }

    /// Executes LCC algorithm in write mode.
    pub fn lcc<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::lcc::LccResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement LCC write mode")
    }

    /// Executes K1-Coloring algorithm in write mode.
    pub fn k1_coloring<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::k1_coloring::K1ColoringResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement K1-Coloring write mode")
    }

    /// Executes Approx Max K-Cut algorithm in write mode.
    pub fn approx_max_k_cut<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::approx_max_k_cut::ApproxMaxKCutResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement Approx Max K-Cut write mode")
    }

    /// Executes K-Means algorithm in write mode.
    pub fn k_means<C: Config, RESULT>(
        &self,
        _graph_name: GraphName,
        _config: C,
        _result_builder: impl ResultBuilder<C, crate::k_means::KMeansResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        todo!("Implement K-Means write mode")
    }
}
