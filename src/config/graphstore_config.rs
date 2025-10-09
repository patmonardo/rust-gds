//! GraphStore configuration types
//!
//! Configuration for the GraphStore runtime system including memory management,
//! caching strategy, and computation settings.

use super::base_types::{ConcurrencyConfig, Config};
use super::validation::{ConfigError, ConfigValidation};

/// Memory management configuration for GraphStore
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphStoreMemoryConfig {
    pub max_memory_bytes: usize,
    pub enable_memory_tracking: bool,
    pub gc_threshold_ratio: f64,
    pub allow_disk_offload: bool,
    pub offload_path: Option<String>,
}

impl Default for GraphStoreMemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            enable_memory_tracking: true,
            gc_threshold_ratio: 0.8,
            allow_disk_offload: false,
            offload_path: None,
        }
    }
}

impl Config for GraphStoreMemoryConfig {}

impl GraphStoreMemoryConfig {
    pub fn builder() -> GraphStoreMemoryConfigBuilder {
        GraphStoreMemoryConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.max_memory_bytes as f64, "maxMemoryBytes")?;
        ConfigValidation::validate_range(self.gc_threshold_ratio, 0.0, 1.0, "gcThresholdRatio")?;
        if let Some(ref path) = self.offload_path {
            ConfigValidation::validate_path(path)?;
        }
        Ok(())
    }
}

/// Builder for GraphStoreMemoryConfig
#[derive(Debug, Default)]
pub struct GraphStoreMemoryConfigBuilder {
    max_memory_bytes: Option<usize>,
    enable_memory_tracking: Option<bool>,
    gc_threshold_ratio: Option<f64>,
    allow_disk_offload: Option<bool>,
    offload_path: Option<String>,
}

impl GraphStoreMemoryConfigBuilder {
    pub fn max_memory_bytes(mut self, bytes: usize) -> Self {
        self.max_memory_bytes = Some(bytes);
        self
    }

    pub fn max_memory_gb(mut self, gb: usize) -> Self {
        self.max_memory_bytes = Some(gb * 1024 * 1024 * 1024);
        self
    }

    pub fn enable_memory_tracking(mut self, enable: bool) -> Self {
        self.enable_memory_tracking = Some(enable);
        self
    }

    pub fn gc_threshold_ratio(mut self, ratio: f64) -> Self {
        self.gc_threshold_ratio = Some(ratio);
        self
    }

    pub fn allow_disk_offload(mut self, allow: bool) -> Self {
        self.allow_disk_offload = Some(allow);
        self
    }

    pub fn offload_path(mut self, path: String) -> Self {
        self.offload_path = Some(path);
        self
    }

    pub fn build(self) -> Result<GraphStoreMemoryConfig, ConfigError> {
        let defaults = GraphStoreMemoryConfig::default();

        let config = GraphStoreMemoryConfig {
            max_memory_bytes: self.max_memory_bytes.unwrap_or(defaults.max_memory_bytes),
            enable_memory_tracking: self
                .enable_memory_tracking
                .unwrap_or(defaults.enable_memory_tracking),
            gc_threshold_ratio: self
                .gc_threshold_ratio
                .unwrap_or(defaults.gc_threshold_ratio),
            allow_disk_offload: self
                .allow_disk_offload
                .unwrap_or(defaults.allow_disk_offload),
            offload_path: self.offload_path.or(defaults.offload_path),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Cache configuration for GraphStore
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphStoreCacheConfig {
    pub enable_node_cache: bool,
    pub enable_relationship_cache: bool,
    pub enable_property_cache: bool,
    pub node_cache_size: usize,
    pub relationship_cache_size: usize,
    pub property_cache_size: usize,
    pub cache_eviction_strategy: CacheEvictionStrategy,
}

impl Default for GraphStoreCacheConfig {
    fn default() -> Self {
        Self {
            enable_node_cache: true,
            enable_relationship_cache: true,
            enable_property_cache: true,
            node_cache_size: 10000,
            relationship_cache_size: 100000,
            property_cache_size: 50000,
            cache_eviction_strategy: CacheEvictionStrategy::Lru,
        }
    }
}

impl Config for GraphStoreCacheConfig {}

impl GraphStoreCacheConfig {
    pub fn builder() -> GraphStoreCacheConfigBuilder {
        GraphStoreCacheConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.enable_node_cache {
            ConfigValidation::validate_positive(self.node_cache_size as f64, "nodeCacheSize")?;
        }
        if self.enable_relationship_cache {
            ConfigValidation::validate_positive(
                self.relationship_cache_size as f64,
                "relationshipCacheSize",
            )?;
        }
        if self.enable_property_cache {
            ConfigValidation::validate_positive(
                self.property_cache_size as f64,
                "propertyCacheSize",
            )?;
        }
        Ok(())
    }
}

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

/// Builder for GraphStoreCacheConfig
#[derive(Debug, Default)]
pub struct GraphStoreCacheConfigBuilder {
    enable_node_cache: Option<bool>,
    enable_relationship_cache: Option<bool>,
    enable_property_cache: Option<bool>,
    node_cache_size: Option<usize>,
    relationship_cache_size: Option<usize>,
    property_cache_size: Option<usize>,
    cache_eviction_strategy: Option<CacheEvictionStrategy>,
}

impl GraphStoreCacheConfigBuilder {
    pub fn enable_node_cache(mut self, enable: bool) -> Self {
        self.enable_node_cache = Some(enable);
        self
    }

    pub fn enable_relationship_cache(mut self, enable: bool) -> Self {
        self.enable_relationship_cache = Some(enable);
        self
    }

    pub fn enable_property_cache(mut self, enable: bool) -> Self {
        self.enable_property_cache = Some(enable);
        self
    }

    pub fn node_cache_size(mut self, size: usize) -> Self {
        self.node_cache_size = Some(size);
        self
    }

    pub fn relationship_cache_size(mut self, size: usize) -> Self {
        self.relationship_cache_size = Some(size);
        self
    }

    pub fn property_cache_size(mut self, size: usize) -> Self {
        self.property_cache_size = Some(size);
        self
    }

    pub fn cache_eviction_strategy(mut self, strategy: CacheEvictionStrategy) -> Self {
        self.cache_eviction_strategy = Some(strategy);
        self
    }

    pub fn build(self) -> Result<GraphStoreCacheConfig, ConfigError> {
        let defaults = GraphStoreCacheConfig::default();

        let config = GraphStoreCacheConfig {
            enable_node_cache: self.enable_node_cache.unwrap_or(defaults.enable_node_cache),
            enable_relationship_cache: self
                .enable_relationship_cache
                .unwrap_or(defaults.enable_relationship_cache),
            enable_property_cache: self
                .enable_property_cache
                .unwrap_or(defaults.enable_property_cache),
            node_cache_size: self.node_cache_size.unwrap_or(defaults.node_cache_size),
            relationship_cache_size: self
                .relationship_cache_size
                .unwrap_or(defaults.relationship_cache_size),
            property_cache_size: self
                .property_cache_size
                .unwrap_or(defaults.property_cache_size),
            cache_eviction_strategy: self
                .cache_eviction_strategy
                .unwrap_or(defaults.cache_eviction_strategy),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Computation configuration for GraphStore operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphStoreComputeConfig {
    pub concurrency: usize,
    pub batch_size: usize,
    pub enable_parallel_execution: bool,
    pub task_queue_size: usize,
    pub worker_pool_size: usize,
    pub enable_work_stealing: bool,
    pub computation_timeout_secs: Option<u64>,
}

impl Default for GraphStoreComputeConfig {
    fn default() -> Self {
        Self {
            concurrency: num_cpus::get(),
            batch_size: 10000,
            enable_parallel_execution: true,
            task_queue_size: 1000,
            worker_pool_size: num_cpus::get(),
            enable_work_stealing: true,
            computation_timeout_secs: None,
        }
    }
}

impl Config for GraphStoreComputeConfig {}

impl ConcurrencyConfig for GraphStoreComputeConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl GraphStoreComputeConfig {
    pub fn builder() -> GraphStoreComputeConfigBuilder {
        GraphStoreComputeConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_positive(self.batch_size as f64, "batchSize")?;
        ConfigValidation::validate_positive(self.task_queue_size as f64, "taskQueueSize")?;
        ConfigValidation::validate_positive(self.worker_pool_size as f64, "workerPoolSize")?;
        Ok(())
    }
}

/// Builder for GraphStoreComputeConfig
#[derive(Debug, Default)]
pub struct GraphStoreComputeConfigBuilder {
    concurrency: Option<usize>,
    batch_size: Option<usize>,
    enable_parallel_execution: Option<bool>,
    task_queue_size: Option<usize>,
    worker_pool_size: Option<usize>,
    enable_work_stealing: Option<bool>,
    computation_timeout_secs: Option<u64>,
}

impl GraphStoreComputeConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub fn enable_parallel_execution(mut self, enable: bool) -> Self {
        self.enable_parallel_execution = Some(enable);
        self
    }

    pub fn task_queue_size(mut self, size: usize) -> Self {
        self.task_queue_size = Some(size);
        self
    }

    pub fn worker_pool_size(mut self, size: usize) -> Self {
        self.worker_pool_size = Some(size);
        self
    }

    pub fn enable_work_stealing(mut self, enable: bool) -> Self {
        self.enable_work_stealing = Some(enable);
        self
    }

    pub fn computation_timeout_secs(mut self, secs: u64) -> Self {
        self.computation_timeout_secs = Some(secs);
        self
    }

    pub fn build(self) -> Result<GraphStoreComputeConfig, ConfigError> {
        let defaults = GraphStoreComputeConfig::default();

        let config = GraphStoreComputeConfig {
            concurrency: self.concurrency.unwrap_or(defaults.concurrency),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            enable_parallel_execution: self
                .enable_parallel_execution
                .unwrap_or(defaults.enable_parallel_execution),
            task_queue_size: self.task_queue_size.unwrap_or(defaults.task_queue_size),
            worker_pool_size: self.worker_pool_size.unwrap_or(defaults.worker_pool_size),
            enable_work_stealing: self
                .enable_work_stealing
                .unwrap_or(defaults.enable_work_stealing),
            computation_timeout_secs: self
                .computation_timeout_secs
                .or(defaults.computation_timeout_secs),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Complete GraphStore configuration combining all subsystems
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphStoreConfig {
    pub memory: GraphStoreMemoryConfig,
    pub cache: GraphStoreCacheConfig,
    pub compute: GraphStoreComputeConfig,
}

impl Config for GraphStoreConfig {}

impl GraphStoreConfig {
    pub fn builder() -> GraphStoreConfigBuilder {
        GraphStoreConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        self.memory.validate()?;
        self.cache.validate()?;
        self.compute.validate()?;
        Ok(())
    }
}

/// Builder for complete GraphStoreConfig
#[derive(Debug, Default)]
pub struct GraphStoreConfigBuilder {
    memory: Option<GraphStoreMemoryConfig>,
    cache: Option<GraphStoreCacheConfig>,
    compute: Option<GraphStoreComputeConfig>,
}

impl GraphStoreConfigBuilder {
    pub fn memory(mut self, config: GraphStoreMemoryConfig) -> Self {
        self.memory = Some(config);
        self
    }

    pub fn cache(mut self, config: GraphStoreCacheConfig) -> Self {
        self.cache = Some(config);
        self
    }

    pub fn compute(mut self, config: GraphStoreComputeConfig) -> Self {
        self.compute = Some(config);
        self
    }

    pub fn build(self) -> Result<GraphStoreConfig, ConfigError> {
        let defaults = GraphStoreConfig::default();

        let config = GraphStoreConfig {
            memory: self.memory.unwrap_or(defaults.memory),
            cache: self.cache.unwrap_or(defaults.cache),
            compute: self.compute.unwrap_or(defaults.compute),
        };

        config.validate()?;
        Ok(config)
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
            .max_memory_gb(8)
            .gc_threshold_ratio(0.9)
            .allow_disk_offload(true)
            .offload_path(String::from("/tmp/offload"))
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
            .computation_timeout_secs(300)
            .build()
            .unwrap();

        assert_eq!(config.concurrency, 16);
        assert_eq!(config.batch_size, 5000);
        assert_eq!(config.computation_timeout_secs, Some(300));
    }

    #[test]
    fn test_complete_graphstore_config() {
        let memory = GraphStoreMemoryConfig::builder()
            .max_memory_gb(16)
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
