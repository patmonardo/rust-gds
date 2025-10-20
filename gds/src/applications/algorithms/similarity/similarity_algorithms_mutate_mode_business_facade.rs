use crate::api::GraphName;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, ResultBuilder,
};
use crate::applications::algorithms::metadata::{RelationshipsWritten, Algorithm};
use crate::applications::algorithms::similarity::{
    SimilarityAlgorithms, SimilarityAlgorithmsEstimationModeBusinessFacade,
};
use crate::applications::algorithms::similarity::results::*;
use crate::applications::algorithms::similarity::mutate_steps::*;
use crate::config::base_types::Config;
use crate::logging::Log;
use std::collections::HashMap;

/// Business facade for similarity algorithms in mutate mode.
/// This provides mutate capabilities for similarity algorithms.
#[derive(Clone)]
pub struct SimilarityAlgorithmsMutateModeBusinessFacade {
    log: Log,
    estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
    similarity_algorithms: SimilarityAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl SimilarityAlgorithmsMutateModeBusinessFacade {
    pub fn new(
        log: Log,
        estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
        similarity_algorithms: SimilarityAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        Self {
            log,
            estimation_facade,
            similarity_algorithms,
            algorithm_processing_template_convenience,
        }
    }

    /// Execute FilteredKNN in mutate mode
    pub fn filtered_knn<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, FilteredKnnResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let mutate_step = FilteredKnnMutateStep::create(
            self.log.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            Algorithm::FilteredKNN,
            || self.estimation_facade.filtered_knn(&configuration),
            |graph, _| self.similarity_algorithms.filtered_knn(graph, &configuration),
            mutate_step,
            result_builder,
        )
    }

    /// Execute FilteredNodeSimilarity in mutate mode
    pub fn filtered_node_similarity<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, NodeSimilarityResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let mutate_step = FilteredNodeSimilarityMutateStep::create(
            self.log.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            Algorithm::FilteredNodeSimilarity,
            || self.estimation_facade.filtered_node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.filtered_node_similarity(graph, &configuration),
            mutate_step,
            result_builder,
        )
    }

    /// Execute KNN in mutate mode
    pub fn knn<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, KnnResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let mutate_step = KnnMutateStep::create(
            self.log.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            Algorithm::KNN,
            || self.estimation_facade.knn(&configuration),
            |graph, _| self.similarity_algorithms.knn(graph, &configuration),
            mutate_step,
            result_builder,
        )
    }

    /// Execute NodeSimilarity in mutate mode
    pub fn node_similarity<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, NodeSimilarityResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let mutate_step = NodeSimilarityMutateStep::create(
            self.log.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            Algorithm::NodeSimilarity,
            || self.estimation_facade.node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.node_similarity(graph, &configuration),
            mutate_step,
            result_builder,
        )
    }
}