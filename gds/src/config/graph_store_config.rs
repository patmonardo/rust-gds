//! GraphStore configuration types
//!
//! Configuration for the GraphStore runtime system including memory management,
//! caching strategy, and computation settings.

use super::base_types::ConcurrencyConfig;
use super::validation::ConfigValidation;
use super::property_store_config::PropertyStoreConfig;
use crate::define_config;

define_config!(
    pub struct GraphStoreMemoryConfig {
        validate = |cfg: &GraphStoreMemoryConfig| {
            ConfigValidation::validate_positive(cfg.max_memory_bytes as f64, "maxMemoryBytes")?;
            ConfigValidation::validate_range(cfg.gc_threshold_ratio, 0.0, 1.0, "gcThresholdRatio")?;
            if let Some(ref path) = cfg.offload_path {
                ConfigValidation::validate_path(path)?;
            }
            Ok(())
        },
        max_memory_bytes: usize = 4 * 1024 * 1024 * 1024, // 4GB
        enable_memory_tracking: bool = true,
        gc_threshold_ratio: f64 = 0.8,
        allow_disk_offload: bool = false,
        offload_path: Option<String> = None,
    }
);

define_config!(
    pub struct GraphStoreCacheConfig {
        validate = |cfg: &GraphStoreCacheConfig| {
            if cfg.enable_node_cache {
                ConfigValidation::validate_positive(cfg.node_cache_size as f64, "nodeCacheSize")?;
            }
            if cfg.enable_relationship_cache {
                ConfigValidation::validate_positive(
                    cfg.relationship_cache_size as f64,
                    "relationshipCacheSize",
                )?;
            }
            if cfg.enable_property_cache {
                ConfigValidation::validate_positive(
                    cfg.property_cache_size as f64,
                    "propertyCacheSize",
                )?;
            }
            Ok(())
        },
        enable_node_cache: bool = true,
        enable_relationship_cache: bool = true,
        enable_property_cache: bool = true,
        node_cache_size: usize = 10000,
        relationship_cache_size: usize = 100000,
        property_cache_size: usize = 50000,
        cache_eviction_strategy: CacheEvictionStrategy = CacheEvictionStrategy::Lru,
    }
);

/// Cache eviction strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CacheEvictionStrategy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Random eviction
    Random,
}

define_config!(
    pub struct GraphStoreComputeConfig {
        validate = |cfg: &GraphStoreComputeConfig| {
            ConfigValidation::validate_positive(cfg.concurrency as f64, "concurrency")?;
            ConfigValidation::validate_positive(cfg.batch_size as f64, "batchSize")?;
            ConfigValidation::validate_positive(cfg.task_queue_size as f64, "taskQueueSize")?;
            ConfigValidation::validate_positive(cfg.worker_pool_size as f64, "workerPoolSize")?;
            Ok(())
        },
        concurrency: usize = num_cpus::get(),
        batch_size: usize = 10000,
        enable_parallel_execution: bool = true,
        task_queue_size: usize = 1000,
        worker_pool_size: usize = num_cpus::get(),
        enable_work_stealing: bool = true,
        computation_timeout_secs: Option<u64> = None,
    }
);

impl ConcurrencyConfig for GraphStoreComputeConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

define_config!(
    pub struct GraphStoreConfig {
        validate = |cfg: &GraphStoreConfig| {
            cfg.memory.validate()?;
            cfg.cache.validate()?;
            cfg.compute.validate()?;
            Ok(())
        },
        memory: GraphStoreMemoryConfig = GraphStoreMemoryConfig::default(),
        cache: GraphStoreCacheConfig = GraphStoreCacheConfig::default(),
        compute: GraphStoreComputeConfig = GraphStoreComputeConfig::default(),
        properties: PropertyStoreConfig = PropertyStoreConfig::default(),
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_config_default() {
        let config = GraphStoreMemoryConfig::default();
        assert_eq!(config.max_memory_bytes, 4 * 1024 * 1024 * 1024);
        assert!(config.enable_memory_tracking);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_memory_config_builder() {
        let config = GraphStoreMemoryConfig::builder()
            .max_memory_bytes(8 * 1024 * 1024 * 1024)
            .gc_threshold_ratio(0.9)
            .allow_disk_offload(true)
            .offload_path(Some(String::from("/tmp/offload")))
            .build()
            .unwrap();

        assert_eq!(config.max_memory_bytes, 8 * 1024 * 1024 * 1024);
        assert_eq!(config.gc_threshold_ratio, 0.9);
        assert!(config.allow_disk_offload);
    }

    #[test]
    fn test_cache_config_default() {
        let config = GraphStoreCacheConfig::default();
        assert!(config.enable_node_cache);
        assert_eq!(config.node_cache_size, 10000);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_cache_config_builder() {
        let config = GraphStoreCacheConfig::builder()
            .node_cache_size(50000)
            .cache_eviction_strategy(CacheEvictionStrategy::Lfu)
            .build()
            .unwrap();

        assert_eq!(config.node_cache_size, 50000);
        assert_eq!(config.cache_eviction_strategy, CacheEvictionStrategy::Lfu);
    }

    #[test]
    fn test_compute_config_default() {
        let config = GraphStoreComputeConfig::default();
        assert!(config.enable_parallel_execution);
        assert!(config.enable_work_stealing);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_compute_config_builder() {
        let config = GraphStoreComputeConfig::builder()
            .concurrency(16)
            .batch_size(5000)
            .computation_timeout_secs(Some(300))
            .build()
            .unwrap();

        assert_eq!(config.concurrency, 16);
        assert_eq!(config.batch_size, 5000);
        assert_eq!(config.computation_timeout_secs, Some(300));
    }

    #[test]
    fn test_complete_graphstore_config() {
        let memory = GraphStoreMemoryConfig::builder()
            .max_memory_bytes(16 * 1024 * 1024 * 1024)
            .build()
            .unwrap();

        let cache = GraphStoreCacheConfig::builder()
            .node_cache_size(100000)
            .build()
            .unwrap();

        let compute = GraphStoreComputeConfig::builder()
            .concurrency(32)
            .build()
            .unwrap();

        let config = GraphStoreConfig::builder()
            .memory(memory)
            .cache(cache)
            .compute(compute)
            .build()
            .unwrap();

        assert_eq!(config.memory.max_memory_bytes, 16 * 1024 * 1024 * 1024);
        assert_eq!(config.cache.node_cache_size, 100000);
        assert_eq!(config.compute.concurrency, 32);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_gc_threshold() {
        let result = GraphStoreMemoryConfig::builder()
            .gc_threshold_ratio(1.5)
            .build();

        assert!(result.is_err());
    }
}
