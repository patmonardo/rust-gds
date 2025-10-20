use crate::api::GraphName;
use crate::applications::algorithms::miscellaneous::{
    MiscellaneousAlgorithms, MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    ScalePropertiesResult,
};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, StatsResultBuilder, AlgorithmLabel,
};
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct MiscellaneousAlgorithmsStatsModeBusinessFacade {
    estimation_facade: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    miscellaneous_algorithms: MiscellaneousAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
}

impl MiscellaneousAlgorithmsStatsModeBusinessFacade {
    pub fn new(
        estimation_facade: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
        miscellaneous_algorithms: MiscellaneousAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        Self { 
            estimation_facade, 
            miscellaneous_algorithms, 
            algorithm_processing_template_convenience 
        }
    }

    /// Scale Properties stats mode
    pub fn scale_properties<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl StatsResultBuilder<ScalePropertiesResult, RESULT>,
    ) -> RESULT {
        self.algorithm_processing_template_convenience.process_regular_algorithm_in_stats_mode(
            graph_name,
            configuration,
            AlgorithmLabel::ScaleProperties,
            || self.estimation_facade.scale_properties(&configuration),
            |graph, _| self.miscellaneous_algorithms.scale_properties(graph, &configuration),
            result_builder
        )
    }
}
