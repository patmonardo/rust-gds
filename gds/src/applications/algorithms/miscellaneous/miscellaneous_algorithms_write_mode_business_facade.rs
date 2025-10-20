use crate::api::GraphName;
use crate::applications::algorithms::miscellaneous::{
    MiscellaneousAlgorithms, MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    ScalePropertiesWriteStep, ScalePropertiesResult,
};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, ResultBuilder, AlgorithmLabel,
    WriteToDatabase, RequestScopedDependencies, WriteContext,
};
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct MiscellaneousAlgorithmsWriteModeBusinessFacade {
    estimation_facade: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
    miscellaneous_algorithms: MiscellaneousAlgorithms,
    algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    write_to_database: Box<dyn WriteToDatabase>,
}

impl MiscellaneousAlgorithmsWriteModeBusinessFacade {
    fn new(
        estimation_facade: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
        miscellaneous_algorithms: MiscellaneousAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
        write_to_database: Box<dyn WriteToDatabase>,
    ) -> Self {
        Self { 
            estimation_facade, 
            miscellaneous_algorithms, 
            algorithm_processing_template_convenience, 
            write_to_database 
        }
    }

    pub fn create(
        _log: crate::logging::Log,
        _request_scoped_dependencies: RequestScopedDependencies,
        _write_context: WriteContext,
        estimation_facade: MiscellaneousAlgorithmsEstimationModeBusinessFacade,
        miscellaneous_algorithms: MiscellaneousAlgorithms,
        algorithm_processing_template_convenience: AlgorithmProcessingTemplateConvenience,
    ) -> Self {
        let write_to_database = WriteToDatabase::new();
        Self::new(estimation_facade, miscellaneous_algorithms, algorithm_processing_template_convenience, write_to_database)
    }

    /// Scale Properties write mode
    pub fn scale_properties<C: Config, RESULT>(
        &self,
        graph_name: GraphName,
        configuration: C,
        result_builder: impl ResultBuilder<C, ScalePropertiesResult, RESULT, NodePropertiesWritten>,
    ) -> RESULT {
        let write_step = ScalePropertiesWriteStep::new(self.write_to_database.clone(), configuration.clone());

        self.algorithm_processing_template_convenience.process_regular_algorithm_in_write_mode(
            graph_name,
            configuration,
            AlgorithmLabel::ScaleProperties,
            || self.estimation_facade.scale_properties(&configuration),
            |graph, _| self.miscellaneous_algorithms.scale_properties(graph, &configuration),
            write_step,
            result_builder
        )
    }
}
