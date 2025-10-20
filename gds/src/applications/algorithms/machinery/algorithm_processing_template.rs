use crate::api::{Graph, GraphStore, GraphName};
use crate::applications::algorithms::machinery::{
    AlgorithmLabel, MutateStep, WriteStep, ResultBuilder, StreamResultBuilder, StatsResultBuilder,
    RequestScopedDependencies, WriteContext, DimensionTransformer,
};
use crate::config::base_types::Config;
use crate::mem::MemoryEstimation;
use crate::core::loading::{PostLoadValidationHook, PostLoadETLHook};
use std::collections::HashMap;

/// The core Algorithm Processing Template interface.
/// This defines the ISA (Instruction Set Architecture) for our Platonic Form Processor.
/// All algorithms follow the same execution pattern:
/// 1. Load data
/// 2. Compute algorithm  
/// 3. Process any side effects (mutate/write)
/// 4. Render a result
pub trait AlgorithmProcessingTemplate {
    /// Process algorithm for write mode with full configuration
    fn process_algorithm_for_write<
        CONFIG: Config,
        RESULT_TO_CALLER,
        RESULT_FROM_ALGORITHM,
        WRITE_METADATA,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: CONFIG,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> RESULT_FROM_ALGORITHM,
        write_step: Box<dyn WriteStep<RESULT_FROM_ALGORITHM, WRITE_METADATA>>,
        result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA>>,
    ) -> RESULT_TO_CALLER;

    /// Process algorithm for mutate mode with full configuration
    fn process_algorithm_for_mutate<
        CONFIG: Config,
        RESULT_TO_CALLER,
        RESULT_FROM_ALGORITHM,
        MUTATE_METADATA,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: CONFIG,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> RESULT_FROM_ALGORITHM,
        mutate_step: Box<dyn MutateStep<RESULT_FROM_ALGORITHM, MUTATE_METADATA>>,
        result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA>>,
    ) -> RESULT_TO_CALLER;

    /// Process algorithm for stream mode with full configuration
    fn process_algorithm_for_stream<
        CONFIG: Config,
        RESULT_TO_CALLER,
        RESULT_FROM_ALGORITHM,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: CONFIG,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> RESULT_FROM_ALGORITHM,
        result_builder: Box<dyn StreamResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>,
    ) -> Vec<RESULT_TO_CALLER>;

    /// Process algorithm for stats mode with full configuration
    fn process_algorithm_for_stats<
        CONFIG: Config,
        RESULT_TO_CALLER,
        RESULT_FROM_ALGORITHM,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: CONFIG,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> RESULT_FROM_ALGORITHM,
        result_builder: Box<dyn StatsResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>,
    ) -> RESULT_TO_CALLER;

    /// Process algorithm with any side effects - the core template method
    fn process_algorithm_and_any_side_effects<
        CONFIG: Config,
        RESULT_TO_CALLER,
        RESULT_FROM_ALGORITHM,
        SIDE_EFFECT_METADATA,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: CONFIG,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> RESULT_FROM_ALGORITHM,
        side_effect: Option<Box<dyn SideEffect<RESULT_FROM_ALGORITHM, SIDE_EFFECT_METADATA>>>,
        result_renderer: Box<dyn ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, SIDE_EFFECT_METADATA>>,
    ) -> RESULT_TO_CALLER;
}

// Placeholder for Computation trait
pub trait Computation<RESULT> {
    fn compute(&self, graph: Graph, graph_store: GraphStore) -> RESULT;
}

// Placeholder for SideEffect trait
pub trait SideEffect<RESULT_FROM_ALGORITHM, METADATA> {
    fn process(
        &self,
        graph_resources: &crate::core::loading::GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
    ) -> Option<METADATA>;
}

// Placeholder for ResultRenderer trait
pub trait ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, SIDE_EFFECT_METADATA> {
    fn render(
        &self,
        graph_resources: &crate::core::loading::GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        timings: crate::applications::algorithms::machinery::AlgorithmProcessingTimings,
        metadata: Option<SIDE_EFFECT_METADATA>,
    ) -> RESULT_TO_CALLER;
}
