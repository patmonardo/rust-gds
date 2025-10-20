// Placeholder config module for similarity algorithms
// This would contain configuration traits and structs for similarity algorithms

use crate::config::base_types::Config;
use crate::concurrency::Concurrency;

/// Base configuration trait for FilteredKNN algorithm
pub trait FilteredKnnBaseConfig: Config + Clone {
    fn max_iterations(&self) -> usize;
    fn concurrency(&self) -> Concurrency;
    fn seed_target_nodes(&self) -> bool;
    fn to_memory_estimation_parameters(&self) -> MemoryEstimationParameters;
}

/// Base configuration trait for FilteredNodeSimilarity algorithm
pub trait FilteredNodeSimilarityBaseConfig: Config + Clone {
    fn use_components(&self) -> UseComponents;
    fn source_node_filter(&self) -> NodeFilter;
    fn target_node_filter(&self) -> NodeFilter;
    fn to_parameters(&self) -> NodeSimilarityParameters;
    fn concurrency(&self) -> Concurrency;
    fn to_memory_estimate_parameters(&self) -> MemoryEstimationParameters;
}

/// Base configuration trait for KNN algorithm
pub trait KnnBaseConfig: Config + Clone {
    fn to_parameters(&self) -> KnnParameters;
    fn max_iterations(&self) -> usize;
    fn concurrency(&self) -> Concurrency;
    fn to_memory_estimation_parameters(&self) -> MemoryEstimationParameters;
}

/// Base configuration trait for NodeSimilarity algorithm
pub trait NodeSimilarityBaseConfig: Config + Clone {
    fn use_components(&self) -> UseComponents;
    fn to_parameters(&self) -> NodeSimilarityParameters;
    fn concurrency(&self) -> Concurrency;
    fn to_memory_estimate_parameters(&self) -> MemoryEstimationParameters;
}

/// Configuration for mutate mode operations
pub struct FilteredKnnMutateConfig;
pub struct FilteredNodeSimilarityMutateConfig;
pub struct KnnMutateConfig;
pub struct NodeSimilarityMutateConfig;

/// Configuration for write mode operations
pub struct FilteredKnnWriteConfig;
pub struct FilteredNodeSimilarityWriteConfig;
pub struct KnnWriteConfig;
pub struct NodeSimilarityWriteConfig;

/// Placeholder types
#[derive(Clone)]
pub struct UseComponents;

impl UseComponents {
    pub fn compute_components(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct NodeFilter;

impl NodeFilter {
    pub fn to_node_filter(&self, _graph: &crate::api::Graph) -> Self {
        NodeFilter
    }

    pub fn allow_everything() -> Self {
        NodeFilter
    }
}

#[derive(Clone)]
pub struct NodeSimilarityParameters;

#[derive(Clone)]
pub struct KnnParameters;

impl KnnParameters {
    pub fn finalize(&self, _node_count: i64) -> Self {
        self.clone()
    }

    pub fn node_property_specs(&self) -> NodePropertySpecs {
        NodePropertySpecs
    }
}

#[derive(Clone)]
pub struct MemoryEstimationParameters;

#[derive(Clone)]
pub struct NodePropertySpecs;
