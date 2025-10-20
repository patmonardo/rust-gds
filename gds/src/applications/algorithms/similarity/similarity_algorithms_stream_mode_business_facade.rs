use crate::api::GraphName;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, StreamResultBuilder,
};
use crate::applications::algorithms::metadata::Algorithm;
use crate::applications::algorithms::similarity::{
    SimilarityAlgorithms, SimilarityAlgorithmsEstimationModeBusinessFacade,
};
use crate::applications::algorithms::similarity::results::*;
use crate::config::base_types::Config;

/// Business facade for similarity algorithms in stream mode.
/// This provides stream capabilities for similarity algorithms.
#[derive(Clone)]
pub struct SimilarityAlgorithmsStreamModeBusinessFacade {
    estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
    similarity_algorithms: SimilarityAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl SimilarityAlgorithmsStreamModeBusinessFacade {
    pub fn new(
        estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
        similarity_algorithms: SimilarityAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        Self {
            estimation_facade,
            similarity_algorithms,
            algorithm_processing_template_convenience,
        }
    }

    /// Execute FilteredKNN in stream mode
    pub fn filtered_knn<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl StreamResultBuilder<FilteredKnnResult, RESULT>,
    ) -> Vec<RESULT> {
        self.algorithm_processing_template_convenience.process_regular_algorithm_in_stream_mode(
            graph_name,
            configuration,
            Algorithm::FilteredKNN,
            || self.estimation_facade.filtered_knn(&configuration),
            |graph, _| self.similarity_algorithms.filtered_knn(graph, &configuration),
            result_builder,
        )
    }

    /// Execute FilteredNodeSimilarity in stream mode
    pub fn filtered_node_similarity<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl StreamResultBuilder<NodeSimilarityResult, RESULT>,
    ) -> Vec<RESULT> {
        self.algorithm_processing_template_convenience.process_regular_algorithm_in_stream_mode(
            graph_name,
            configuration,
            Algorithm::FilteredNodeSimilarity,
            || self.estimation_facade.filtered_node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.filtered_node_similarity(graph, &configuration),
            result_builder,
        )
    }

    /// Execute KNN in stream mode
    pub fn knn<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl StreamResultBuilder<KnnResult, RESULT>,
    ) -> Vec<RESULT> {
        self.algorithm_processing_template_convenience.process_regular_algorithm_in_stream_mode(
            graph_name,
            configuration,
            Algorithm::KNN,
            || self.estimation_facade.knn(&configuration),
            |graph, _| self.similarity_algorithms.knn(graph, &configuration),
            result_builder,
        )
    }

    /// Execute NodeSimilarity in stream mode
    pub fn node_similarity<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl StreamResultBuilder<NodeSimilarityResult, RESULT>,
    ) -> Vec<RESULT> {
        self.algorithm_processing_template_convenience.process_regular_algorithm_in_stream_mode(
            graph_name,
            configuration,
            Algorithm::NodeSimilarity,
            || self.estimation_facade.node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.node_similarity(graph, &configuration),
            result_builder,
        )
    }
}