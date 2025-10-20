use crate::api::GraphName;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, ResultBuilder, AlgorithmLabel,
};
use crate::applications::algorithms::metadata::{RelationshipsWritten, Algorithm};
use crate::applications::algorithms::similarity::{
    SimilarityAlgorithms, SimilarityAlgorithmsEstimationModeBusinessFacade,
};
use crate::applications::algorithms::similarity::results::*;
use crate::applications::algorithms::similarity::write_steps::*;
use crate::config::base_types::Config;
use std::collections::HashMap;

/// Business facade for similarity algorithms in write mode.
/// This provides write capabilities for similarity algorithms.
#[derive(Clone)]
pub struct SimilarityAlgorithmsWriteModeBusinessFacade {
    estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
    similarity_algorithms: SimilarityAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    write_relationship_service: WriteRelationshipService,
}

impl SimilarityAlgorithmsWriteModeBusinessFacade {
    pub fn new(
        estimation_facade: SimilarityAlgorithmsEstimationModeBusinessFacade,
        similarity_algorithms: SimilarityAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        write_relationship_service: WriteRelationshipService,
    ) -> Self {
        Self {
            estimation_facade,
            similarity_algorithms,
            algorithm_processing_template_convenience,
            write_relationship_service,
        }
    }

    /// Execute FilteredKNN in write mode
    pub fn filtered_knn<C: Config + Clone, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, FilteredKnnResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let write_step = FilteredKnnWriteStep::create(
            configuration.clone(),
            should_compute_similarity_distribution,
            self.write_relationship_service.clone(),
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_write_mode(
            graph_name,
            configuration,
            Algorithm::FilteredKNN,
            || self.estimation_facade.filtered_knn(&configuration),
            |graph, _| self.similarity_algorithms.filtered_knn(graph, &configuration),
            write_step,
            result_builder,
        )
    }

    /// Execute FilteredNodeSimilarity in write mode
    pub fn filtered_node_similarity<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, NodeSimilarityResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let write_step = FilteredNodeSimilarityWriteStep::create(
            self.write_relationship_service.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_write_mode(
            graph_name,
            configuration,
            AlgorithmLabel::FilteredNodeSimilarity,
            || self.estimation_facade.filtered_node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.filtered_node_similarity(graph, &configuration),
            write_step,
            result_builder,
        )
    }

    /// Execute KNN in write mode
    pub fn knn<C: Config + Clone, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, KnnResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let write_step = KnnWriteStep::create(
            configuration.clone(),
            should_compute_similarity_distribution,
            self.write_relationship_service.clone(),
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_write_mode(
            graph_name,
            configuration,
            AlgorithmLabel::KNN,
            || self.estimation_facade.knn(&configuration),
            |graph, _| self.similarity_algorithms.knn(graph, &configuration),
            write_step,
            result_builder,
        )
    }

    /// Execute NodeSimilarity in write mode
    pub fn node_similarity<C: Config + Clone, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, NodeSimilarityResult, RESULT, (RelationshipsWritten, HashMap<String, String>)>,
        should_compute_similarity_distribution: bool,
    ) -> RESULT {
        let write_step = NodeSimilarityWriteStep::create(
            self.write_relationship_service.clone(),
            configuration.clone(),
            should_compute_similarity_distribution,
        );

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_write_mode(
            graph_name,
            configuration,
            AlgorithmLabel::NodeSimilarity,
            || self.estimation_facade.node_similarity(&configuration),
            |graph, _| self.similarity_algorithms.node_similarity(graph, &configuration),
            write_step,
            result_builder,
        )
    }
}