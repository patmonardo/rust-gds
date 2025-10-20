//! Algorithm configuration types and builders

use super::base_types::{AlgoBaseConfig, ConcurrencyConfig, IterationsConfig};
use super::pregel_config::PregelRuntimeConfig;
use super::validation::{validate_positive, validate_range};
use crate::core::utils::partition::Partitioning;
use crate::define_config;

define_config!(
    pub struct PageRankConfig {
        validate = |cfg: &PageRankConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            validate_range(cfg.damping_factor, 0.0, 1.0, "dampingFactor")?;
            validate_positive(cfg.tolerance, "tolerance")?;
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        max_iterations: usize = 20,
        tolerance: f64 = 0.0000001,
        damping_factor: f64 = 0.85,
        source_nodes: Option<Vec<String>> = None,
    }
);

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

impl PregelRuntimeConfig for PageRankConfig {
    fn is_asynchronous(&self) -> bool {
        false
    }

    fn partitioning(&self) -> Partitioning {
        Partitioning::Range
    }

    fn track_sender(&self) -> bool {
        false
    }
}

define_config!(
    pub struct LouvainConfig {
        validate = |cfg: &LouvainConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            validate_positive(cfg.tolerance, "tolerance")?;
            validate_range(cfg.gamma, 0.0, 10.0, "gamma")?;
            validate_range(cfg.theta, 0.0, 1.0, "theta")?;
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        max_iterations: usize = 10,
        tolerance: f64 = 0.0001,
        include_intermediate_communities: bool = false,
        seed_property: Option<String> = None,
        gamma: f64 = 1.0,
        theta: f64 = 0.01,
    }
);

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

define_config!(
    pub struct NodeSimilarityConfig {
        validate = |cfg: &NodeSimilarityConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            validate_range(cfg.similarity_cutoff, 0.0, 1.0, "similarityCutoff")?;
            validate_positive(cfg.degree_cutoff as f64, "degreeCutoff")?;
            validate_positive(cfg.top_k as f64, "topK")?;
            validate_positive(cfg.bottom_k as f64, "bottomK")?;
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        similarity_cutoff: f64 = 0.0,
        degree_cutoff: usize = 1,
        top_k: usize = 10,
        bottom_k: usize = 10,
    }
);

impl ConcurrencyConfig for NodeSimilarityConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

define_config!(
    pub struct BetweennessCentralityConfig {
        validate = |cfg: &BetweennessCentralityConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            if let Some(size) = cfg.sampling_size {
                validate_positive(size as f64, "samplingSize")?;
            }
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        sampling_size: Option<usize> = None,
        sampling_seed: Option<u64> = None,
    }
);

impl ConcurrencyConfig for BetweennessCentralityConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
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
            .sampling_size(Some(100))
            .sampling_seed(Some(42))
            .build()
            .unwrap();

        assert_eq!(config.sampling_size, Some(100));
        assert_eq!(config.sampling_seed, Some(42));
    }
}
