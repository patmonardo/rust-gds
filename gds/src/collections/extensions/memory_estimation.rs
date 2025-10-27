//! Collections Memory Estimation Extensions
//!
//! Repackages GDS Memory Estimation utilities as Collections Extensions for the Collections First approach.
//! This provides memory estimation capabilities as extensions to any Collections implementation.

use crate::collections::traits::Collections;
use crate::config::Extension;
use crate::mem::Estimate;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Memory estimation result
#[derive(Debug, Clone)]
pub struct MemoryEstimationResult {
    pub estimated_bytes: u64,
    pub breakdown: HashMap<String, u64>,
    pub is_conservative: bool,
    pub notes: Vec<String>,
}

/// Memory estimation extension trait for Collections
pub trait MemoryEstimationSupport<T> {
    /// Estimate memory usage for this collection
    fn estimate_memory(&self) -> MemoryEstimationResult;
    
    /// Estimate memory usage with custom dimensions
    fn estimate_memory_with_dimensions(&self, dimensions: &CollectionDimensions) -> MemoryEstimationResult;
    
    /// Get memory breakdown by component
    fn memory_breakdown(&self) -> HashMap<String, usize>;
    
    /// Check if collection fits in memory budget
    fn fits_in_budget(&self, budget_bytes: usize) -> bool;
    
    /// Get memory efficiency ratio
    fn memory_efficiency(&self) -> f64;
    
    /// Enable memory tracking
    fn enable_memory_tracking(&mut self) -> Result<(), MemoryError>;
    
    /// Disable memory tracking
    fn disable_memory_tracking(&mut self);
    
    /// Get current memory usage
    fn current_memory_usage(&self) -> usize;
}

/// Collection dimensions for memory estimation
#[derive(Debug, Clone)]
pub struct CollectionDimensions {
    pub element_count: usize,
    pub element_size: usize,
    pub null_count: usize,
    pub compression_ratio: Option<f64>,
    pub overhead_factor: f64,
}

impl Default for CollectionDimensions {
    fn default() -> Self {
        Self {
            element_count: 0,
            element_size: 8, // Default to 8 bytes
            null_count: 0,
            compression_ratio: None,
            overhead_factor: 1.2, // 20% overhead
        }
    }
}

/// Memory estimation configuration
#[derive(Debug, Clone)]
pub struct MemoryEstimationConfig {
    pub include_overhead: bool,
    pub include_compression: bool,
    pub include_null_bitmap: bool,
    pub conservative_estimate: bool,
    pub track_detailed_breakdown: bool,
}

impl Default for MemoryEstimationConfig {
    fn default() -> Self {
        Self {
            include_overhead: true,
            include_compression: true,
            include_null_bitmap: true,
            conservative_estimate: false,
            track_detailed_breakdown: true,
        }
    }
}

/// Simple memory tracker
pub struct SimpleMemoryTracker {
    total_memory: usize,
}

impl Default for SimpleMemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleMemoryTracker {
    pub fn new() -> Self {
        Self { total_memory: 0 }
    }
    
    pub fn total_memory_usage(&self) -> usize {
        self.total_memory
    }
}

/// Memory-aware collection wrapper
pub struct MemoryAwareCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    memory_config: MemoryEstimationConfig,
    memory_tracker: Option<SimpleMemoryTracker>,
    dimensions: CollectionDimensions,
    _phantom: PhantomData<T>,
}

impl<T, C> MemoryAwareCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        let dimensions = CollectionDimensions {
            element_count: inner.len(),
            element_size: std::mem::size_of::<T>(),
            null_count: inner.null_count(),
            compression_ratio: None,
            overhead_factor: 1.2,
        };
        
        Self {
            inner,
            memory_config: MemoryEstimationConfig::default(),
            memory_tracker: None,
            dimensions,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_config(inner: C, config: MemoryEstimationConfig) -> Self {
        let dimensions = CollectionDimensions {
            element_count: inner.len(),
            element_size: std::mem::size_of::<T>(),
            null_count: inner.null_count(),
            compression_ratio: None,
            overhead_factor: 1.2,
        };
        
        Self {
            inner,
            memory_config: config,
            memory_tracker: None,
            dimensions,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for MemoryAwareCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.inner.get(index)
    }
    
    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value);
        // Update dimensions if tracking is enabled
        if self.memory_tracker.is_some() {
            self.dimensions.element_count = self.inner.len();
        }
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
        &[Extension::MemoryEstimation]
    }
    
    fn extensions(&self) -> &[crate::config::Extension] {
        &[Extension::MemoryEstimation]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for memory-aware collections
        todo!("Implement with_capacity for MemoryAwareCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for memory-aware collections
        todo!("Implement with_defaults for MemoryAwareCollection")
    }
}

impl<T, C> MemoryEstimationSupport<T> for MemoryAwareCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn estimate_memory(&self) -> MemoryEstimationResult {
        let base_size = self.dimensions.element_count * self.dimensions.element_size;
        let mut total_size = base_size;
        
        let mut breakdown = HashMap::new();
        breakdown.insert("data".to_string(), base_size);
        
        if self.memory_config.include_overhead {
            let overhead = (base_size as f64 * (self.dimensions.overhead_factor - 1.0)) as usize;
            total_size += overhead;
            breakdown.insert("overhead".to_string(), overhead);
        }
        
        if self.memory_config.include_null_bitmap && self.dimensions.null_count > 0 {
            let null_bitmap_size = self.dimensions.element_count.div_ceil(8); // 1 bit per element
            total_size += null_bitmap_size;
            breakdown.insert("null_bitmap".to_string(), null_bitmap_size);
        }
        
        if self.memory_config.include_compression {
            if let Some(ratio) = self.dimensions.compression_ratio {
                let compressed_size = (base_size as f64 * ratio) as usize;
                breakdown.insert("compressed_size".to_string(), compressed_size);
            }
        }
        
        MemoryEstimationResult {
            estimated_bytes: total_size as u64,
            breakdown: breakdown.into_iter().map(|(k, v)| (k, v as u64)).collect(),
            is_conservative: self.memory_config.conservative_estimate,
            notes: vec!["Memory estimation for Collections".to_string()],
        }
    }
    
    fn estimate_memory_with_dimensions(&self, dimensions: &CollectionDimensions) -> MemoryEstimationResult {
        let base_size = dimensions.element_count * dimensions.element_size;
        let mut total_size = base_size;
        
        let mut breakdown = HashMap::new();
        breakdown.insert("data".to_string(), base_size);
        
        if self.memory_config.include_overhead {
            let overhead = (base_size as f64 * (dimensions.overhead_factor - 1.0)) as usize;
            total_size += overhead;
            breakdown.insert("overhead".to_string(), overhead);
        }
        
        if self.memory_config.include_null_bitmap && dimensions.null_count > 0 {
            let null_bitmap_size = dimensions.element_count.div_ceil(8);
            total_size += null_bitmap_size;
            breakdown.insert("null_bitmap".to_string(), null_bitmap_size);
        }
        
        MemoryEstimationResult {
            estimated_bytes: total_size as u64,
            breakdown: breakdown.into_iter().map(|(k, v)| (k, v as u64)).collect(),
            is_conservative: self.memory_config.conservative_estimate,
            notes: vec!["Memory estimation with custom dimensions".to_string()],
        }
    }
    
    fn memory_breakdown(&self) -> HashMap<String, usize> {
        let base_size = self.dimensions.element_count * self.dimensions.element_size;
        let mut breakdown = HashMap::new();
        
        breakdown.insert("data".to_string(), base_size);
        
        if self.memory_config.include_overhead {
            let overhead = (base_size as f64 * (self.dimensions.overhead_factor - 1.0)) as usize;
            breakdown.insert("overhead".to_string(), overhead);
        }
        
        if self.memory_config.include_null_bitmap && self.dimensions.null_count > 0 {
            let null_bitmap_size = self.dimensions.element_count.div_ceil(8);
            breakdown.insert("null_bitmap".to_string(), null_bitmap_size);
        }
        
        breakdown
    }
    
    fn fits_in_budget(&self, budget_bytes: usize) -> bool {
        let estimation = self.estimate_memory();
        estimation.estimated_bytes <= budget_bytes as u64
    }
    
    fn memory_efficiency(&self) -> f64 {
        let base_size = self.dimensions.element_count * self.dimensions.element_size;
        let total_size = self.estimate_memory().estimated_bytes as usize;
        
        if total_size == 0 {
            return 1.0;
        }
        
        base_size as f64 / total_size as f64
    }
    
    fn enable_memory_tracking(&mut self) -> Result<(), MemoryError> {
        self.memory_tracker = Some(SimpleMemoryTracker::new());
        Ok(())
    }
    
    fn disable_memory_tracking(&mut self) {
        self.memory_tracker = None;
    }
    
    fn current_memory_usage(&self) -> usize {
        if let Some(ref tracker) = self.memory_tracker {
            // Get current memory usage from tracker
            tracker.total_memory_usage()
        } else {
            // Fallback to estimation
            self.estimate_memory().estimated_bytes as usize
        }
    }
}

/// Memory error types
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Memory estimation failed: {0}")]
    EstimationFailed(String),
    #[error("Memory tracking failed: {0}")]
    TrackingFailed(String),
    #[error("Memory budget exceeded: {0}")]
    BudgetExceeded(String),
    #[error("Invalid memory configuration: {0}")]
    InvalidConfig(String),
}

/// Memory estimation utilities
pub struct MemoryEstimationUtils;

impl MemoryEstimationUtils {
    /// Estimate memory for different collection types
    pub fn estimate_collection_memory<T>(
        element_count: usize,
        collection_type: &str,
    ) -> usize {
        let element_size = std::mem::size_of::<T>();
        let base_size = element_count * element_size;
        
        match collection_type {
            "vec" => base_size + Estimate::BYTES_ARRAY_HEADER,
            "huge" => {
                let page_size = 4096;
                let pages_needed = element_count.div_ceil(page_size);
                base_size + pages_needed * Estimate::BYTES_ARRAY_HEADER
            },
            "arrow" => {
                let null_bitmap_size = element_count.div_ceil(8);
                base_size + null_bitmap_size + Estimate::BYTES_ARRAY_HEADER
            },
            _ => base_size,
        }
    }
    
    /// Calculate optimal memory configuration
    pub fn optimal_memory_config<T>(
        element_count: usize,
        available_memory: usize,
    ) -> MemoryEstimationConfig {
        let estimated_size = Self::estimate_collection_memory::<T>(element_count, "vec");
        let memory_pressure = estimated_size as f64 / available_memory as f64;
        
        MemoryEstimationConfig {
            include_overhead: true,
            include_compression: memory_pressure > 0.8,
            include_null_bitmap: true,
            conservative_estimate: memory_pressure > 0.9,
            track_detailed_breakdown: memory_pressure > 0.7,
        }
    }
    
    /// Format memory size in human-readable format
    pub fn format_memory_size(bytes: usize) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
