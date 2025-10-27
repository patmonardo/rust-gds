//! GraphStore configuration types
//!
//! Configuration for the GraphStore runtime system including memory management,
//! caching strategy, and computation settings.

use super::base_types::ConcurrencyConfig;
use super::validation::ConfigValidation;
use super::collections_config::{CollectionsBackend, CollectionsConfig, CollectionsConfigBuilder};
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
    /// Properties configuration for Collections-backed property storage
    ///
    /// This config drives the selection of Collections backends for different property levels.
    /// It replaces the deprecated PropertyStoreConfig with a Collections-first approach.
    pub struct GraphStorePropertiesConfig {
        validate = |cfg: &GraphStorePropertiesConfig| {
            ConfigValidation::validate_positive(cfg.huge_array_threshold as f64, "hugeArrayThreshold")?;
            Ok(())
        },
        /// Default backend for node properties
        default_node_backend: CollectionsBackend = CollectionsBackend::Vec,
        /// Default backend for relationship properties
        default_relationship_backend: CollectionsBackend = CollectionsBackend::Vec,
        /// Default backend for graph properties
        default_graph_backend: CollectionsBackend = CollectionsBackend::Vec,
        /// Element count threshold for switching Vec -> Huge (10M elements)
        huge_array_threshold: usize = 10_000_000,
        /// Enable automatic backend selection based on size
        enable_adaptive_backend: bool = true,
        /// Prefer Arrow when available (for future Arrow backend)
        prefer_arrow: bool = false,
        // Note: optimization_level is part of CollectionsConfig's PerformanceConfig
        // We'll set sensible defaults when creating CollectionsConfig instances
    }
);

define_config!(
    pub struct GraphStoreConfig {
        validate = |cfg: &GraphStoreConfig| {
            cfg.memory.validate()?;
            cfg.cache.validate()?;
            cfg.compute.validate()?;
            cfg.properties.validate()?;
            Ok(())
        },
        memory: GraphStoreMemoryConfig = GraphStoreMemoryConfig::default(),
        cache: GraphStoreCacheConfig = GraphStoreCacheConfig::default(),
        compute: GraphStoreComputeConfig = GraphStoreComputeConfig::default(),
        properties: GraphStorePropertiesConfig = GraphStorePropertiesConfig::default(),
    }
);

impl GraphStoreConfig {
    /// Creates a CollectionsConfig for node properties of type T
    ///
    /// Uses the store's properties config to select the appropriate backend,
    /// with adaptive selection based on element count if enabled.
    ///
    /// # Example
    /// ```ignore
    /// let config = GraphStoreConfig::default();
    /// let node_config: CollectionsConfig<i64> = config.node_collections_config(1_000_000);
    /// // Uses Vec backend for 1M elements
    /// ```
    pub fn node_collections_config<T>(&self, element_count: usize) -> CollectionsConfig<T> {
        let backend = self.select_backend(
            self.properties.default_node_backend,
            element_count,
        );

        CollectionsConfigBuilder::<T>::new()
            .with_backend(backend)
            .build()
    }

    /// Creates a CollectionsConfig for relationship properties of type T
    ///
    /// Similar to node_collections_config but uses relationship-specific defaults.
    pub fn relationship_collections_config<T>(&self, element_count: usize) -> CollectionsConfig<T> {
        let backend = self.select_backend(
            self.properties.default_relationship_backend,
            element_count,
        );

        CollectionsConfigBuilder::<T>::new()
            .with_backend(backend)
            .build()
    }

    /// Creates a CollectionsConfig for graph properties of type T
    ///
    /// Graph properties are typically small (single value), so adaptive
    /// backend selection is less relevant.
    pub fn graph_collections_config<T>(&self, _element_count: usize) -> CollectionsConfig<T> {
        let backend = self.properties.default_graph_backend;

        CollectionsConfigBuilder::<T>::new()
            .with_backend(backend)
            .build()
    }

    /// Selects the appropriate backend based on element count and config
    ///
    /// If adaptive backend is enabled and element count exceeds the threshold,
    /// switches from Vec to Huge. In the future, this will also handle Arrow
    /// selection when prefer_arrow is true.
    fn select_backend(&self, default_backend: CollectionsBackend, element_count: usize) -> CollectionsBackend {
        // Future: Check prefer_arrow first
        if self.properties.prefer_arrow {
            // TODO: When Arrow backend is ready, return CollectionsBackend::Arrow
            // For now, fall through to size-based selection
        }

        // Adaptive backend selection based on size
        if self.properties.enable_adaptive_backend
            && element_count > self.properties.huge_array_threshold
        {
            // Large collections should use Huge backend for memory efficiency
            match default_backend {
                CollectionsBackend::Vec | CollectionsBackend::Std => CollectionsBackend::Huge,
                other => other, // Respect explicit backend choices
            }
        } else {
            default_backend
        }
    }
}

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

    #[test]
    fn test_properties_config_default() {
        let config = GraphStorePropertiesConfig::default();
        assert_eq!(config.default_node_backend, CollectionsBackend::Vec);
        assert_eq!(config.default_relationship_backend, CollectionsBackend::Vec);
        assert_eq!(config.default_graph_backend, CollectionsBackend::Vec);
        assert_eq!(config.huge_array_threshold, 10_000_000);
        assert!(config.enable_adaptive_backend);
        assert!(!config.prefer_arrow);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_properties_config_builder() {
        let config = GraphStorePropertiesConfig::builder()
            .default_node_backend(CollectionsBackend::Huge)
            .huge_array_threshold(5_000_000)
            .prefer_arrow(true)
            .build()
            .unwrap();

        assert_eq!(config.default_node_backend, CollectionsBackend::Huge);
        assert_eq!(config.huge_array_threshold, 5_000_000);
        assert!(config.prefer_arrow);
    }

    #[test]
    fn test_graphstore_config_with_properties() {
        let properties = GraphStorePropertiesConfig::builder()
            .default_node_backend(CollectionsBackend::Huge)
            .enable_adaptive_backend(true)
            .build()
            .unwrap();

        let config = GraphStoreConfig::builder()
            .properties(properties)
            .build()
            .unwrap();

        assert_eq!(config.properties.default_node_backend, CollectionsBackend::Huge);
        assert!(config.properties.enable_adaptive_backend);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_collections_config_creation_small() {
        let config = GraphStoreConfig::default();
        
        // Small collection (1M elements) should use Vec
        let node_config: CollectionsConfig<i64> = config.node_collections_config(1_000_000);
        assert_eq!(node_config.backend.primary, CollectionsBackend::Vec);
    }

    #[test]
    fn test_collections_config_creation_large() {
        let config = GraphStoreConfig::default();
        
        // Large collection (20M elements) should auto-switch to Huge
        let node_config: CollectionsConfig<i64> = config.node_collections_config(20_000_000);
        assert_eq!(node_config.backend.primary, CollectionsBackend::Huge);
    }

    #[test]
    fn test_collections_config_adaptive_disabled() {
        let config = GraphStoreConfig::builder()
            .properties(
                GraphStorePropertiesConfig::builder()
                    .enable_adaptive_backend(false)
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap();
        
        // Even with 20M elements, should stay Vec when adaptive is disabled
        let node_config: CollectionsConfig<i64> = config.node_collections_config(20_000_000);
        assert_eq!(node_config.backend.primary, CollectionsBackend::Vec);
    }

    #[test]
    fn test_collections_config_per_level() {
        let config = GraphStoreConfig::builder()
            .properties(
                GraphStorePropertiesConfig::builder()
                    .default_node_backend(CollectionsBackend::Huge)
                    .default_relationship_backend(CollectionsBackend::Vec)
                    .default_graph_backend(CollectionsBackend::Vec)
                    .enable_adaptive_backend(false)
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap();
        
        // Each level uses its configured backend
        let node_config: CollectionsConfig<i64> = config.node_collections_config(1000);
        let rel_config: CollectionsConfig<f64> = config.relationship_collections_config(1000);
        let graph_config: CollectionsConfig<f64> = config.graph_collections_config(1);
        
        assert_eq!(node_config.backend.primary, CollectionsBackend::Huge);
        assert_eq!(rel_config.backend.primary, CollectionsBackend::Vec);
        assert_eq!(graph_config.backend.primary, CollectionsBackend::Vec);
    }
}
