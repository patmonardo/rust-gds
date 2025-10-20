use crate::core::loading::GraphResources;
use crate::applications::algorithms::machinery::AlgorithmProcessingTimings;
use crate::applications::algorithms::machinery::{ResultBuilder, StatsResultBuilder, StreamResultBuilder};

/// Result Renderer - renders results from algorithm execution
pub trait ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, SIDE_EFFECT_METADATA> {
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<SIDE_EFFECT_METADATA>,
    ) -> RESULT_TO_CALLER;
}

/// Mutate Result Renderer - renders results for mutate mode
pub struct MutateResultRenderer<
    RESULT_FROM_ALGORITHM,
    RESULT_TO_CALLER,
    MUTATE_METADATA,
    CONFIG: crate::config::base_types::Config,
> {
    configuration: CONFIG,
    result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA>>,
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA, CONFIG: crate::config::base_types::Config>
    MutateResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA, CONFIG>
{
    pub fn new(
        configuration: CONFIG,
        result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA>>,
    ) -> Self {
        Self {
            configuration,
            result_builder,
        }
    }
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA, CONFIG: crate::config::base_types::Config>
    ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA>
    for MutateResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, MUTATE_METADATA, CONFIG>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<MUTATE_METADATA>,
    ) -> RESULT_TO_CALLER {
        self.result_builder.build(
            graph_resources.graph.clone(),
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}

/// Write Result Renderer - renders results for write mode
pub struct WriteResultRenderer<
    RESULT_FROM_ALGORITHM,
    RESULT_TO_CALLER,
    WRITE_METADATA,
    CONFIG: crate::config::base_types::Config,
> {
    configuration: CONFIG,
    result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA>>,
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA, CONFIG: crate::config::base_types::Config>
    WriteResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA, CONFIG>
{
    pub fn new(
        configuration: CONFIG,
        result_builder: Box<dyn ResultBuilder<CONFIG, RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA>>,
    ) -> Self {
        Self {
            configuration,
            result_builder,
        }
    }
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA, CONFIG: crate::config::base_types::Config>
    ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA>
    for WriteResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, WRITE_METADATA, CONFIG>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<WRITE_METADATA>,
    ) -> RESULT_TO_CALLER {
        self.result_builder.build(
            graph_resources.graph.clone(),
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}

/// Stats Result Renderer - renders results for stats mode
pub struct StatsResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> {
    stats_result_builder: Box<dyn StatsResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>,
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> StatsResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> {
    pub fn new(stats_result_builder: Box<dyn StatsResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>) -> Self {
        Self { stats_result_builder }
    }
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> ResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER, ()>
    for StatsResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> RESULT_TO_CALLER {
        self.stats_result_builder.build(
            graph_resources.graph.clone(),
            result,
            timings,
        )
    }
}

/// Stream Result Renderer - renders results for stream mode
pub struct StreamResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> {
    result_builder: Box<dyn StreamResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>,
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> StreamResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> {
    pub fn new(result_builder: Box<dyn StreamResultBuilder<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>>) -> Self {
        Self { result_builder }
    }
}

impl<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER> ResultRenderer<RESULT_FROM_ALGORITHM, Vec<RESULT_TO_CALLER>, ()>
    for StreamResultRenderer<RESULT_FROM_ALGORITHM, RESULT_TO_CALLER>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
        _timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> Vec<RESULT_TO_CALLER> {
        self.result_builder.build(
            graph_resources.graph.clone(),
            graph_resources.graph_store.clone(),
            result,
        )
    }
}
