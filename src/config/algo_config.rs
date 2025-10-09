//! Algorithm configuration types and builders

use super::base_types::{AlgoBaseConfig, ConcurrencyConfig, Config, IterationsConfig};
use super::validation::{ConfigError, ConfigValidation};
use crate::projection::{NodeLabel, RelationshipType};

/// PageRank algorithm configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PageRankConfig {
    pub base: AlgoBaseConfig,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub damping_factor: f64,
    pub source_nodes: Option<Vec<String>>,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            max_iterations: 20,
            tolerance: 0.0000001,
            damping_factor: 0.85,
            source_nodes: None,
        }
    }
}

impl Config for PageRankConfig {}

impl ConcurrencyConfig for PageRankConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl IterationsConfig for PageRankConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        Some(self.tolerance)
    }
}

impl PageRankConfig {
    pub fn builder() -> PageRankConfigBuilder {
        PageRankConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_positive(self.max_iterations as f64, "maxIterations")?;
        ConfigValidation::validate_range(self.damping_factor, 0.0, 1.0, "dampingFactor")?;
        ConfigValidation::validate_positive(self.tolerance, "tolerance")?;
        Ok(())
    }
}

/// Builder for PageRank configuration
#[derive(Debug, Default)]
pub struct PageRankConfigBuilder {
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
    damping_factor: Option<f64>,
    source_nodes: Option<Vec<String>>,
}

impl PageRankConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = Some(iterations);
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn damping_factor(mut self, factor: f64) -> Self {
        self.damping_factor = Some(factor);
        self
    }

    pub fn source_nodes(mut self, nodes: Vec<String>) -> Self {
        self.source_nodes = Some(nodes);
        self
    }

    pub fn build(self) -> Result<PageRankConfig, ConfigError> {
        let defaults = PageRankConfig::default();

        let config = PageRankConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            max_iterations: self.max_iterations.unwrap_or(defaults.max_iterations),
            tolerance: self.tolerance.unwrap_or(defaults.tolerance),
            damping_factor: self.damping_factor.unwrap_or(defaults.damping_factor),
            source_nodes: self.source_nodes.or(defaults.source_nodes),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Louvain algorithm configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LouvainConfig {
    pub base: AlgoBaseConfig,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub include_intermediate_communities: bool,
    pub seed_property: Option<String>,
    pub gamma: f64,
    pub theta: f64,
}

impl Default for LouvainConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            max_iterations: 10,
            tolerance: 0.0001,
            include_intermediate_communities: false,
            seed_property: None,
            gamma: 1.0,
            theta: 0.01,
        }
    }
}

impl Config for LouvainConfig {}

impl ConcurrencyConfig for LouvainConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl IterationsConfig for LouvainConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        Some(self.tolerance)
    }
}

impl LouvainConfig {
    pub fn builder() -> LouvainConfigBuilder {
        LouvainConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_positive(self.max_iterations as f64, "maxIterations")?;
        ConfigValidation::validate_positive(self.tolerance, "tolerance")?;
        ConfigValidation::validate_range(self.gamma, 0.0, 10.0, "gamma")?;
        ConfigValidation::validate_range(self.theta, 0.0, 1.0, "theta")?;
        Ok(())
    }
}

/// Builder for Louvain configuration
#[derive(Debug, Default)]
pub struct LouvainConfigBuilder {
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
    include_intermediate_communities: Option<bool>,
    seed_property: Option<String>,
    gamma: Option<f64>,
    theta: Option<f64>,
}

impl LouvainConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = Some(iterations);
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn include_intermediate_communities(mut self, include: bool) -> Self {
        self.include_intermediate_communities = Some(include);
        self
    }

    pub fn seed_property(mut self, property: String) -> Self {
        self.seed_property = Some(property);
        self
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    pub fn theta(mut self, theta: f64) -> Self {
        self.theta = Some(theta);
        self
    }

    pub fn build(self) -> Result<LouvainConfig, ConfigError> {
        let defaults = LouvainConfig::default();

        let config = LouvainConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            max_iterations: self.max_iterations.unwrap_or(defaults.max_iterations),
            tolerance: self.tolerance.unwrap_or(defaults.tolerance),
            include_intermediate_communities: self
                .include_intermediate_communities
                .unwrap_or(defaults.include_intermediate_communities),
            seed_property: self.seed_property.or(defaults.seed_property),
            gamma: self.gamma.unwrap_or(defaults.gamma),
            theta: self.theta.unwrap_or(defaults.theta),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Node Similarity algorithm configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeSimilarityConfig {
    pub base: AlgoBaseConfig,
    pub similarity_cutoff: f64,
    pub degree_cutoff: usize,
    pub top_k: usize,
    pub bottom_k: usize,
}

impl Default for NodeSimilarityConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            similarity_cutoff: 0.0,
            degree_cutoff: 1,
            top_k: 10,
            bottom_k: 10,
        }
    }
}

impl Config for NodeSimilarityConfig {}

impl ConcurrencyConfig for NodeSimilarityConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl NodeSimilarityConfig {
    pub fn builder() -> NodeSimilarityConfigBuilder {
        NodeSimilarityConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_range(self.similarity_cutoff, 0.0, 1.0, "similarityCutoff")?;
        ConfigValidation::validate_positive(self.degree_cutoff as f64, "degreeCutoff")?;
        ConfigValidation::validate_positive(self.top_k as f64, "topK")?;
        ConfigValidation::validate_positive(self.bottom_k as f64, "bottomK")?;
        Ok(())
    }
}

/// Builder for Node Similarity configuration
#[derive(Debug, Default)]
pub struct NodeSimilarityConfigBuilder {
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    similarity_cutoff: Option<f64>,
    degree_cutoff: Option<usize>,
    top_k: Option<usize>,
    bottom_k: Option<usize>,
}

impl NodeSimilarityConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn similarity_cutoff(mut self, cutoff: f64) -> Self {
        self.similarity_cutoff = Some(cutoff);
        self
    }

    pub fn degree_cutoff(mut self, cutoff: usize) -> Self {
        self.degree_cutoff = Some(cutoff);
        self
    }

    pub fn top_k(mut self, k: usize) -> Self {
        self.top_k = Some(k);
        self
    }

    pub fn bottom_k(mut self, k: usize) -> Self {
        self.bottom_k = Some(k);
        self
    }

    pub fn build(self) -> Result<NodeSimilarityConfig, ConfigError> {
        let defaults = NodeSimilarityConfig::default();

        let config = NodeSimilarityConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            similarity_cutoff: self.similarity_cutoff.unwrap_or(defaults.similarity_cutoff),
            degree_cutoff: self.degree_cutoff.unwrap_or(defaults.degree_cutoff),
            top_k: self.top_k.unwrap_or(defaults.top_k),
            bottom_k: self.bottom_k.unwrap_or(defaults.bottom_k),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Betweenness Centrality algorithm configuration
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BetweennessCentralityConfig {
    pub base: AlgoBaseConfig,
    pub sampling_size: Option<usize>,
    pub sampling_seed: Option<u64>,
}

impl Config for BetweennessCentralityConfig {}

impl ConcurrencyConfig for BetweennessCentralityConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl BetweennessCentralityConfig {
    pub fn builder() -> BetweennessCentralityConfigBuilder {
        BetweennessCentralityConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        if let Some(size) = self.sampling_size {
            ConfigValidation::validate_positive(size as f64, "samplingSize")?;
        }
        Ok(())
    }
}

/// Builder for Betweenness Centrality configuration
#[derive(Debug, Default)]
pub struct BetweennessCentralityConfigBuilder {
    concurrency: Option<usize>,
    node_labels: Option<Vec<NodeLabel>>,
    relationship_types: Option<Vec<RelationshipType>>,
    sampling_size: Option<usize>,
    sampling_seed: Option<u64>,
}

impl BetweennessCentralityConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn node_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn relationship_types(mut self, types: Vec<RelationshipType>) -> Self {
        self.relationship_types = Some(types);
        self
    }

    pub fn sampling_size(mut self, size: usize) -> Self {
        self.sampling_size = Some(size);
        self
    }

    pub fn sampling_seed(mut self, seed: u64) -> Self {
        self.sampling_seed = Some(seed);
        self
    }

    pub fn build(self) -> Result<BetweennessCentralityConfig, ConfigError> {
        let defaults = BetweennessCentralityConfig::default();

        let config = BetweennessCentralityConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: self.node_labels.unwrap_or(defaults.base.node_labels),
                relationship_types: self
                    .relationship_types
                    .unwrap_or(defaults.base.relationship_types),
            },
            sampling_size: self.sampling_size.or(defaults.sampling_size),
            sampling_seed: self.sampling_seed.or(defaults.sampling_seed),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_default() {
        let config = PageRankConfig::default();
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.damping_factor, 0.85);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_pagerank_builder() {
        let config = PageRankConfig::builder()
            .max_iterations(30)
            .damping_factor(0.9)
            .tolerance(0.001)
            .build()
            .unwrap();

        assert_eq!(config.max_iterations, 30);
        assert_eq!(config.damping_factor, 0.9);
        assert_eq!(config.tolerance, 0.001);
    }

    #[test]
    fn test_pagerank_invalid_damping_factor() {
        let result = PageRankConfig::builder().damping_factor(1.5).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_louvain_default() {
        let config = LouvainConfig::default();
        assert_eq!(config.max_iterations, 10);
        assert_eq!(config.gamma, 1.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_node_similarity_default() {
        let config = NodeSimilarityConfig::default();
        assert_eq!(config.top_k, 10);
        assert_eq!(config.similarity_cutoff, 0.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_betweenness_centrality_builder() {
        let config = BetweennessCentralityConfig::builder()
            .sampling_size(100)
            .sampling_seed(42)
            .build()
            .unwrap();

        assert_eq!(config.sampling_size, Some(100));
        assert_eq!(config.sampling_seed, Some(42));
    }
}
