//! Collections Partitioning Extensions
//!
//! Repackages GDS Partitioning utilities as Collections Extensions for the Collections First approach.
//! This provides sophisticated parallel processing capabilities for ML workloads.

use crate::collections::traits::Collections;
use crate::config::Extension;
use crate::core::utils::partition::{
    Partition, DegreePartition, IteratorPartition, LazyDegreePartitionIterator,
    PartitionUtils, Partitioning, DegreeFunction
};
use std::marker::PhantomData;

/// Partitioning strategy for ML Collections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MLPartitioningStrategy {
    /// Range-based partitioning for uniform workloads
    Range,
    /// Degree-based partitioning for relationship-heavy workloads
    Degree,
    /// Memory-aware partitioning for large datasets
    MemoryAware,
    /// Load-balanced partitioning for ML training
    LoadBalanced,
    /// Cache-friendly partitioning for performance
    CacheOptimized,
    /// Auto-select best strategy based on data characteristics
    Auto,
}

impl MLPartitioningStrategy {
    pub fn from_gds_partitioning(partitioning: Partitioning) -> Self {
        match partitioning {
            Partitioning::Range => MLPartitioningStrategy::Range,
            Partitioning::Degree => MLPartitioningStrategy::Degree,
            Partitioning::Auto => MLPartitioningStrategy::Auto,
        }
    }
    
    pub fn to_gds_partitioning(self) -> Partitioning {
        match self {
            MLPartitioningStrategy::Range => Partitioning::Range,
            MLPartitioningStrategy::Degree => Partitioning::Degree,
            MLPartitioningStrategy::MemoryAware => Partitioning::Degree, // Use degree as fallback
            MLPartitioningStrategy::LoadBalanced => Partitioning::Degree,
            MLPartitioningStrategy::CacheOptimized => Partitioning::Range,
            MLPartitioningStrategy::Auto => Partitioning::Auto,
        }
    }
}

/// Partitioning extension trait for Collections
pub trait PartitioningSupport<T> {
    /// Create partitions for parallel processing
    fn create_partitions(&self, concurrency: usize) -> Result<Vec<Partition>, PartitioningError>;
    
    /// Create degree-aware partitions
    fn create_degree_partitions(
        &self, 
        concurrency: usize, 
        degrees: Box<dyn DegreeFunction>
    ) -> Result<Vec<DegreePartition>, PartitioningError>;
    
    /// Create iterator-based partitions for custom processing
    fn create_iterator_partitions<F>(
        &self, 
        concurrency: usize, 
        iterator_factory: F
    ) -> Result<Vec<IteratorPartition<impl Iterator<Item = usize>>>, PartitioningError>
    where
        F: Fn() -> Box<dyn Iterator<Item = usize>>;
    
    /// Create lazy streaming partitions
    fn create_lazy_partitions(
        &self, 
        concurrency: usize, 
        degrees: Box<dyn DegreeFunction>
    ) -> Result<LazyDegreePartitionIterator, PartitioningError>;
    
    /// Process partitions in parallel
    fn process_partitions<F, R>(
        &self, 
        partitions: Vec<Partition>, 
        processor: F
    ) -> Result<Vec<R>, PartitioningError>
    where
        F: Fn(Partition) -> R + Send + Sync,
        R: Send;
    
    /// Get optimal partitioning strategy for this collection
    fn optimal_partitioning_strategy(&self) -> MLPartitioningStrategy;
    
    /// Estimate partition load balancing
    fn estimate_partition_load(&self, partitions: &[Partition]) -> PartitionLoadEstimate;
    
    /// Enable partition-aware processing
    fn enable_partitioning(&mut self, strategy: MLPartitioningStrategy) -> Result<(), PartitioningError>;
    
    /// Disable partitioning
    fn disable_partitioning(&mut self);
    
    /// Check if partitioning is enabled
    fn is_partitioning_enabled(&self) -> bool;
}

/// Partition load estimation
#[derive(Debug, Clone)]
pub struct PartitionLoadEstimate {
    pub total_work: usize,
    pub partition_count: usize,
    pub average_load: f64,
    pub load_variance: f64,
    pub is_balanced: bool,
    pub recommendations: Vec<String>,
}

/// Partitioning configuration
#[derive(Debug, Clone)]
pub struct PartitioningConfig {
    pub strategy: MLPartitioningStrategy,
    pub concurrency: usize,
    pub min_batch_size: Option<usize>,
    pub max_partition_size: Option<usize>,
    pub enable_load_balancing: bool,
    pub enable_memory_optimization: bool,
    pub enable_cache_optimization: bool,
}

impl Default for PartitioningConfig {
    fn default() -> Self {
        Self {
            strategy: MLPartitioningStrategy::Auto,
            concurrency: num_cpus::get(),
            min_batch_size: Some(100),
            max_partition_size: None,
            enable_load_balancing: true,
            enable_memory_optimization: true,
            enable_cache_optimization: false,
        }
    }
}

/// Partition-aware collection wrapper
pub struct PartitionAwareCollection<T, C>
where
    C: Collections<T>,
{
    inner: C,
    partitioning_config: PartitioningConfig,
    is_partitioning_enabled: bool,
    _phantom: PhantomData<T>,
}

impl<T, C> PartitionAwareCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            partitioning_config: PartitioningConfig::default(),
            is_partitioning_enabled: false,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_config(inner: C, config: PartitioningConfig) -> Self {
        Self {
            inner,
            partitioning_config: config,
            is_partitioning_enabled: false,
            _phantom: PhantomData,
        }
    }
    
    /// Get the underlying collection
    pub fn inner(&self) -> &C {
        &self.inner
    }
    
    /// Get mutable access to the underlying collection
    pub fn inner_mut(&mut self) -> &mut C {
        &mut self.inner
    }
    
    /// Consume the inner collection
    pub fn into_inner(self) -> C {
        self.inner
    }
    
    /// Calculate batch size for partitioning
    fn calculate_batch_size(node_count: usize, concurrency: usize, min_batch_size: usize) -> usize {
        let batch_size = node_count.div_ceil(concurrency); // Ceiling division
        std::cmp::max(batch_size, min_batch_size)
    }
}

impl<T, C> Collections<T> for PartitionAwareCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.inner.get(index)
    }
    
    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value);
    }
    
    fn len(&self) -> usize {
        self.inner.len()
    }
    
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    fn sum(&self) -> Option<T> where T: std::iter::Sum {
        self.inner.sum()
    }
    
    fn min(&self) -> Option<T> where T: Ord {
        self.inner.min()
    }
    
    fn max(&self) -> Option<T> where T: Ord {
        self.inner.max()
    }
    
    fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }
    
    fn std_dev(&self) -> Option<f64> {
        self.inner.std_dev()
    }
    
    fn variance(&self) -> Option<f64> {
        self.inner.variance()
    }
    
    fn median(&self) -> Option<T> where T: Ord {
        self.inner.median()
    }
    
    fn percentile(&self, p: f64) -> Option<T> where T: Ord {
        self.inner.percentile(p)
    }
    
    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord {
        self.inner.binary_search(key)
    }
    
    fn sort(&mut self) where T: Ord {
        self.inner.sort()
    }
    
    fn to_vec(self) -> Vec<T> {
        self.inner.to_vec()
    }
    
    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }
    
    fn is_null(&self, index: usize) -> bool {
        self.inner.is_null(index)
    }
    
    fn null_count(&self) -> usize {
        self.inner.null_count()
    }
    
    fn default_value(&self) -> T {
        self.inner.default_value()
    }
    
    fn backend(&self) -> crate::config::CollectionsBackend {
        self.inner.backend()
    }
    
    fn features(&self) -> &[crate::config::Extension] {
        &[Extension::Partitioning]
    }
    
    fn extensions(&self) -> &[crate::config::Extension] {
        &[Extension::Partitioning]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        todo!("Implement with_capacity for PartitionAwareCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        todo!("Implement with_defaults for PartitionAwareCollection")
    }
}

impl<T, C> PartitioningSupport<T> for PartitionAwareCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn create_partitions(&self, concurrency: usize) -> Result<Vec<Partition>, PartitioningError> {
        if !self.is_partitioning_enabled {
            return Err(PartitioningError::PartitioningNotEnabled);
        }
        
        let node_count = self.inner.len();
        let partitions = PartitionUtils::range_partition(
            concurrency,
            node_count,
            |p| p,
            self.partitioning_config.min_batch_size,
        );
        
        Ok(partitions)
    }
    
    fn create_degree_partitions(
        &self, 
        concurrency: usize, 
        degrees: Box<dyn DegreeFunction>
    ) -> Result<Vec<DegreePartition>, PartitioningError> {
        if !self.is_partitioning_enabled {
            return Err(PartitioningError::PartitioningNotEnabled);
        }
        
        let node_count = self.inner.len();
        // Estimate relationship count based on collection characteristics
        let relationship_count = node_count * 10; // Rough estimate
        
        let partitions = PartitionUtils::degree_partition(
            node_count,
            relationship_count,
            degrees,
            concurrency,
            |p| p,
            self.partitioning_config.min_batch_size,
        );
        
        Ok(partitions)
    }
    
    fn create_iterator_partitions<F>(
        &self, 
        concurrency: usize, 
        iterator_factory: F
    ) -> Result<Vec<IteratorPartition<impl Iterator<Item = usize>>>, PartitioningError>
    where
        F: Fn() -> Box<dyn Iterator<Item = usize>>,
    {
        if !self.is_partitioning_enabled {
            return Err(PartitioningError::PartitioningNotEnabled);
        }
        
        let node_count = self.inner.len();
        let batch_size = Self::calculate_batch_size(
            node_count,
            concurrency,
            self.partitioning_config.min_batch_size.unwrap_or(100),
        );
        
        let mut partitions = Vec::new();
        let mut start = 0;
        
        while start < node_count {
            let end = std::cmp::min(start + batch_size, node_count);
            let iterator = iterator_factory();
            let partition = IteratorPartition::new(iterator, end - start);
            partitions.push(partition);
            start = end;
        }
        
        Ok(partitions)
    }
    
    fn create_lazy_partitions(
        &self, 
        concurrency: usize, 
        degrees: Box<dyn DegreeFunction>
    ) -> Result<LazyDegreePartitionIterator, PartitioningError> {
        if !self.is_partitioning_enabled {
            return Err(PartitioningError::PartitioningNotEnabled);
        }
        
        let node_count = self.inner.len();
        let relationship_count = node_count * 10; // Rough estimate
        
        let iterator = PartitionUtils::degree_partition_stream(
            node_count,
            relationship_count,
            concurrency,
            degrees,
        );
        
        Ok(iterator)
    }
    
    fn process_partitions<F, R>(
        &self, 
        partitions: Vec<Partition>, 
        processor: F
    ) -> Result<Vec<R>, PartitioningError>
    where
        F: Fn(Partition) -> R + Send + Sync,
        R: Send,
    {
        if !self.is_partitioning_enabled {
            return Err(PartitioningError::PartitioningNotEnabled);
        }
        
        // Use rayon for parallel processing if available
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            Ok(partitions.into_par_iter().map(processor).collect())
        }
        
        #[cfg(not(feature = "rayon"))]
        {
            Ok(partitions.into_iter().map(processor).collect())
        }
    }
    
    fn optimal_partitioning_strategy(&self) -> MLPartitioningStrategy {
        let node_count = self.inner.len();
        
        // Simple heuristic for strategy selection
        if node_count < 1000 {
            MLPartitioningStrategy::Range
        } else if node_count < 100_000 {
            MLPartitioningStrategy::LoadBalanced
        } else {
            MLPartitioningStrategy::MemoryAware
        }
    }
    
    fn estimate_partition_load(&self, partitions: &[Partition]) -> PartitionLoadEstimate {
        let total_work = partitions.iter().map(|p| p.node_count()).sum();
        let partition_count = partitions.len();
        let average_load = total_work as f64 / partition_count as f64;
        
        let load_variance = if partition_count > 1 {
            let variance_sum: f64 = partitions.iter()
                .map(|p| {
                    let diff = p.node_count() as f64 - average_load;
                    diff * diff
                })
                .sum();
            variance_sum / (partition_count - 1) as f64
        } else {
            0.0
        };
        
        let is_balanced = load_variance < average_load * 0.1; // 10% tolerance
        
        let mut recommendations = Vec::new();
        if !is_balanced {
            recommendations.push("Consider using degree-based partitioning for better load balancing".to_string());
        }
        if partition_count > self.partitioning_config.concurrency * 2 {
            recommendations.push("Consider reducing partition count for better efficiency".to_string());
        }
        
        PartitionLoadEstimate {
            total_work,
            partition_count,
            average_load,
            load_variance,
            is_balanced,
            recommendations,
        }
    }
    
    fn enable_partitioning(&mut self, strategy: MLPartitioningStrategy) -> Result<(), PartitioningError> {
        self.partitioning_config.strategy = strategy;
        self.is_partitioning_enabled = true;
        Ok(())
    }
    
    fn disable_partitioning(&mut self) {
        self.is_partitioning_enabled = false;
    }
    
    fn is_partitioning_enabled(&self) -> bool {
        self.is_partitioning_enabled
    }
}

/// Partitioning error types
#[derive(Debug, thiserror::Error)]
pub enum PartitioningError {
    #[error("Partitioning not enabled")]
    PartitioningNotEnabled,
    #[error("Invalid partitioning configuration: {0}")]
    InvalidConfig(String),
    #[error("Partition creation failed: {0}")]
    PartitionCreationFailed(String),
    #[error("Parallel processing failed: {0}")]
    ParallelProcessingFailed(String),
    #[error("Load balancing failed: {0}")]
    LoadBalancingFailed(String),
    #[error("Memory optimization failed: {0}")]
    MemoryOptimizationFailed(String),
}

/// ML-specific partitioning utilities
pub struct MLPartitioningUtils;

impl MLPartitioningUtils {
    /// Create partitions optimized for ML training workloads
    pub fn create_ml_training_partitions<T, C>(
        collection: &PartitionAwareCollection<T, C>,
        batch_size: usize,
        num_epochs: usize,
    ) -> Result<Vec<Partition>, PartitioningError>
    where
        C: Collections<T>,
        T: Clone + Send + Sync,
    {
        let total_samples = collection.len();
        let samples_per_epoch = total_samples / num_epochs;
        let partitions_per_epoch = samples_per_epoch.div_ceil(batch_size);
        
        let partitions = PartitionUtils::range_partition(
            partitions_per_epoch,
            total_samples,
            |p| p,
            Some(batch_size),
        );
        
        Ok(partitions)
    }
    
    /// Create partitions optimized for ML inference workloads
    pub fn create_ml_inference_partitions<T, C>(
        collection: &PartitionAwareCollection<T, C>,
        concurrency: usize,
    ) -> Result<Vec<Partition>, PartitioningError>
    where
        C: Collections<T>,
        T: Clone + Send + Sync,
    {
        let total_samples = collection.len();
        let partitions = PartitionUtils::range_partition(
            concurrency,
            total_samples,
            |p| p,
            Some(100), // Smaller batches for inference
        );
        
        Ok(partitions)
    }
    
    /// Create partitions optimized for data preprocessing
    pub fn create_preprocessing_partitions<T, C>(
        collection: &PartitionAwareCollection<T, C>,
        concurrency: usize,
    ) -> Result<Vec<Partition>, PartitioningError>
    where
        C: Collections<T>,
        T: Clone + Send + Sync,
    {
        let total_samples = collection.len();
        let partitions = PartitionUtils::range_partition(
            concurrency,
            total_samples,
            |p| p,
            Some(1000), // Larger batches for preprocessing
        );
        
        Ok(partitions)
    }
    
    /// Analyze collection characteristics for optimal partitioning
    pub fn analyze_for_partitioning<T, C>(
        collection: &PartitionAwareCollection<T, C>,
    ) -> PartitioningAnalysis
    where
        C: Collections<T>,
        T: Clone + Send + Sync,
    {
        let size = collection.len();
        let backend = collection.backend();
        
        let recommended_strategy = if size < 1000 {
            MLPartitioningStrategy::Range
        } else if size < 100_000 {
            MLPartitioningStrategy::LoadBalanced
        } else {
            MLPartitioningStrategy::MemoryAware
        };
        
        let recommended_concurrency = std::cmp::min(
            num_cpus::get(),
            (size / 1000).max(1),
        );
        
        PartitioningAnalysis {
            collection_size: size,
            backend_type: format!("{:?}", backend),
            recommended_strategy,
            recommended_concurrency,
            estimated_memory_per_partition: size / recommended_concurrency,
            is_suitable_for_parallel_processing: size > 100,
        }
    }
}

/// Partitioning analysis result
#[derive(Debug, Clone)]
pub struct PartitioningAnalysis {
    pub collection_size: usize,
    pub backend_type: String,
    pub recommended_strategy: MLPartitioningStrategy,
    pub recommended_concurrency: usize,
    pub estimated_memory_per_partition: usize,
    pub is_suitable_for_parallel_processing: bool,
}

/// Partitioning utilities for Collections
pub struct PartitioningUtils;

impl PartitioningUtils {
    /// Convert GDS partitioning to ML partitioning
    pub fn from_gds_partitioning(partitioning: Partitioning) -> MLPartitioningStrategy {
        MLPartitioningStrategy::from_gds_partitioning(partitioning)
    }
    
    /// Convert ML partitioning to GDS partitioning
    pub fn to_gds_partitioning(strategy: MLPartitioningStrategy) -> Partitioning {
        strategy.to_gds_partitioning()
    }
    
    /// Create optimal partitioning configuration for ML workloads
    pub fn optimal_ml_config<T>(
        collection_size: usize,
        workload_type: MLWorkloadType,
    ) -> PartitioningConfig {
        let concurrency = match workload_type {
            MLWorkloadType::Training => num_cpus::get(),
            MLWorkloadType::Inference => num_cpus::get(),
            MLWorkloadType::Preprocessing => num_cpus::get() * 2,
            MLWorkloadType::Evaluation => num_cpus::get(),
        };
        
        let strategy = match workload_type {
            MLWorkloadType::Training => MLPartitioningStrategy::LoadBalanced,
            MLWorkloadType::Inference => MLPartitioningStrategy::Range,
            MLWorkloadType::Preprocessing => MLPartitioningStrategy::MemoryAware,
            MLWorkloadType::Evaluation => MLPartitioningStrategy::CacheOptimized,
        };
        
        let min_batch_size = match workload_type {
            MLWorkloadType::Training => Some(32),
            MLWorkloadType::Inference => Some(1),
            MLWorkloadType::Preprocessing => Some(1000),
            MLWorkloadType::Evaluation => Some(100),
        };
        
        PartitioningConfig {
            strategy,
            concurrency,
            min_batch_size,
            max_partition_size: None,
            enable_load_balancing: true,
            enable_memory_optimization: true,
            enable_cache_optimization: workload_type == MLWorkloadType::Inference,
        }
    }
}

/// ML workload types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MLWorkloadType {
    Training,
    Inference,
    Preprocessing,
    Evaluation,
}
