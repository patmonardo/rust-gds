use crate::api::GraphName;
use crate::applications::algorithms::miscellaneous::{
    MiscellaneousAlgorithms, MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    CollapsePathMutateStep, IndexInverseMutateStep, ScalePropertiesMutateStep, ToUndirectedMutateStep,
    ScalePropertiesResult,
};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, ResultBuilder, AlgorithmLabel,
};
use crate::applications::algorithms::machinery::DefaultMutateNodeProperty;
use crate::applications::algorithms::metadata::{NodePropertiesWritten, RelationshipsWritten};
use crate::core::loading::SingleTypeRelationships;
use crate::config::base_types::Config;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MiscellaneousAlgorithmsMutateModeBusinessFacade {
    estimation: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    algorithms: MiscellaneousAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    mutate_node_property: DefaultMutateNodeProperty,
}

impl MiscellaneousAlgorithmsMutateModeBusinessFacade {
    pub fn new(
        estimation: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
        algorithms: MiscellaneousAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        mutate_node_property: DefaultMutateNodeProperty,
    ) -> Self {
        Self { 
            estimation, 
            algorithms, 
            algorithm_processing_template_convenience, 
            mutate_node_property 
        }
    }

    /// Collapse Path mutate mode
    pub fn collapse_path<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, SingleTypeRelationships, RESULT, ()>,
    ) -> RESULT {
        let mutate_step = CollapsePathMutateStep;

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            AlgorithmLabel::CollapsePath,
            || self.estimation.collapse_path(),
            |_, graph_store| self.algorithms.collapse_path(graph_store, &configuration),
            mutate_step,
            result_builder
        )
    }

    /// Index Inverse mutate mode
    pub fn index_inverse<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, HashMap<String, SingleTypeRelationships>, RESULT, ()>,
    ) -> RESULT {
        let mutate_step = IndexInverseMutateStep;

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            AlgorithmLabel::IndexInverse,
            || self.estimation.index_inverse(&configuration),
            |graph, graph_store| self.algorithms.index_inverse(graph, graph_store, &configuration),
            mutate_step,
            result_builder
        )
    }

    /// Scale Properties mutate mode
    pub fn scale_properties<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, ScalePropertiesResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        let mutate_step = ScalePropertiesMutateStep::new(self.mutate_node_property.clone(), configuration.clone());

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            AlgorithmLabel::ScaleProperties,
            || self.estimation.scale_properties(&configuration),
            |graph, _| self.algorithms.scale_properties(graph, &configuration),
            mutate_step,
            result_builder
        )
    }

    /// To Undirected mutate mode
    pub fn to_undirected<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, SingleTypeRelationships, RESULT, RelationshipsWritten>,
    ) -> RESULT {
        let mutate_step = ToUndirectedMutateStep;

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_mutate_mode(
            graph_name,
            configuration,
            AlgorithmLabel::ToUndirected,
            || self.estimation.to_undirected(&configuration),
            |_, graph_store| self.algorithms.to_undirected(graph_store, &configuration),
            mutate_step,
            result_builder
        )
    }
}
