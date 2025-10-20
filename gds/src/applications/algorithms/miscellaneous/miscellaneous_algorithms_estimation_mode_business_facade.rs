use crate::applications::algorithms::machinery::AlgorithmEstimationTemplate;
use crate::mem::{MemoryEstimation, MemoryEstimationResult};
use crate::config::base_types::Config;

#[derive(Clone)]
pub struct MiscellaneousAlgorithmsEstimationModeBusinessFacade {
    algorithm_estimation_template: AlgorithmEstimationTemplate,
}

impl MiscellaneousAlgorithmsEstimationModeBusinessFacade {
    pub fn new(algorithm_estimation_template: AlgorithmEstimationTemplate) -> Self {
        Self { algorithm_estimation_template }
    }

    /// Collapse Path memory estimation - not implemented
    pub fn collapse_path(&self) -> MemoryEstimation {
        panic!("Memory estimation not implemented for CollapsePath")
    }

    /// Index Inverse memory estimation
    pub fn index_inverse<C: Config>(&self, configuration: &C) -> MemoryEstimation {
        // Placeholder for InverseRelationshipsMemoryEstimateDefinition
        MemoryEstimation::new("IndexInverse", 1000)
    }

    /// Index Inverse memory estimation with result
    pub fn index_inverse_with_result<C: Config>(
        &self,
        configuration: &C,
        graph_name_or_configuration: &str,
    ) -> MemoryEstimationResult {
        let memory_estimation = self.index_inverse(configuration);
        self.algorithm_estimation_template.estimate(
            configuration,
            graph_name_or_configuration,
            memory_estimation
        )
    }

    /// Scale Properties memory estimation
    pub fn scale_properties<C: Config>(&self, configuration: &C) -> MemoryEstimation {
        // Placeholder for ScalePropertiesMemoryEstimateDefinition
        MemoryEstimation::new("ScaleProperties", 2000)
    }

    /// Scale Properties memory estimation with result
    pub fn scale_properties_with_result<C: Config>(
        &self,
        configuration: &C,
        graph_name_or_configuration: &str,
    ) -> MemoryEstimationResult {
        let memory_estimation = self.scale_properties(configuration);
        self.algorithm_estimation_template.estimate(
            configuration,
            graph_name_or_configuration,
            memory_estimation
        )
    }

    /// To Undirected memory estimation
    pub fn to_undirected<C: Config>(&self, configuration: &C) -> MemoryEstimation {
        // Placeholder for ToUndirectedMemoryEstimateDefinition
        MemoryEstimation::new("ToUndirected", 1500)
    }

    /// To Undirected memory estimation with result
    pub fn to_undirected_with_result<C: Config>(
        &self,
        configuration: &C,
        graph_name_or_configuration: &str,
    ) -> MemoryEstimationResult {
        let memory_estimation = self.to_undirected(configuration);
        self.algorithm_estimation_template.estimate(
            configuration,
            graph_name_or_configuration,
            memory_estimation
        )
    }
}
