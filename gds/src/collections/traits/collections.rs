//! Collections Traits: Core Collections Interface
//!
//! This module defines the core traits that all Collections implementations
//! must implement, providing a unified interface across all backends.

use crate::types::ValueType;
use crate::types::default_value::DefaultValue;
use crate::collections::config::{CollectionsBackend, Extension};
use std::iter::Sum;

/// Main Collections trait that all Collections implementations must implement
pub trait Collections<T>: Send + Sync {
    /// Get element at index
    fn get(&self, index: usize) -> Option<T>;
    
    /// Set element at index
    fn set(&mut self, index: usize, value: T);
    
    /// Get length of collection
    fn len(&self) -> usize;
    
    /// Check if collection is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Get sum of all elements (if supported)
    fn sum(&self) -> Option<T> where T: Sum;
    
    /// Get minimum element (if supported)
    fn min(&self) -> Option<T> where T: Ord;
    
    /// Get maximum element (if supported)
    fn max(&self) -> Option<T> where T: Ord;
    
    /// Get mean of all elements (if supported)
    fn mean(&self) -> Option<f64>;
    
    /// Get standard deviation (if supported)
    fn std_dev(&self) -> Option<f64>;
    
    /// Get variance (if supported)
    fn variance(&self) -> Option<f64>;
    
    /// Get median element (if supported)
    fn median(&self) -> Option<T> where T: Ord;
    
    /// Get percentile (if supported)
    fn percentile(&self, p: f64) -> Option<T> where T: Ord;
    
    /// Binary search for key (if supported)
    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord;
    
    /// Sort collection (if supported)
    fn sort(&mut self) where T: Ord;
    
    /// Convert to Vec
    fn to_vec(self) -> Vec<T>;
    
    /// Get as slice (if supported)
    fn as_slice(&self) -> &[T];
    
    /// Check if element at index is null (if supported)
    fn is_null(&self, index: usize) -> bool;
    
    /// Get count of null elements (if supported)
    fn null_count(&self) -> usize;
    
    /// Get default value for this collection
    fn default_value(&self) -> T;
    
    /// Get element count (same as len for most implementations)
    fn element_count(&self) -> usize {
        self.len()
    }
    
    /// Get backend type
    fn backend(&self) -> CollectionsBackend;
    
    /// Get enabled features
    fn features(&self) -> &[Extension];
    
    /// Get enabled extensions
    fn extensions(&self) -> &[Extension];
    
    /// Get value type
    fn value_type(&self) -> ValueType;
    
    /// Create with capacity
    fn with_capacity(capacity: usize) -> Self where Self: Sized;
    
    /// Create with default values
    fn with_defaults(count: usize, default_value: T) -> Self where Self: Sized;
}

/// PropertyValues adapter trait for Collections
pub trait PropertyValuesAdapter<T>: Collections<T> {
    // All methods are inherited from Collections<T>
}

/// Collections factory trait for creating Collections
pub trait CollectionsFactory<T> {
    /// Create new Collections instance
    fn new() -> Self;
    
    /// Create with capacity
    fn with_capacity(capacity: usize) -> Self;
    
    /// Create from Vec
    fn from_vec(values: Vec<T>) -> Self;
    
    /// Create from slice
    fn from_slice(slice: &[T]) -> Self;
    
    /// Create with default values
    fn with_defaults(count: usize, default_value: T) -> Self;
}

/// Aggregation support trait
pub trait AggregationSupport<T> {
    /// Get sum
    fn sum(&self) -> Option<T> where T: Sum;
    
    /// Get mean
    fn mean(&self) -> Option<f64> where T: Into<f64>;
    
    /// Get standard deviation
    fn std_dev(&self) -> Option<f64> where T: Into<f64>;
    
    /// Get variance
    fn variance(&self) -> Option<f64> where T: Into<f64>;
    
    /// Get median
    fn median(&self) -> Option<T> where T: Ord;
    
    /// Get percentile
    fn percentile(&self, p: f64) -> Option<T> where T: Ord;
}

/// Nullability support trait
pub trait NullabilitySupport<T> {
    /// Check if element is null
    fn is_null(&self, index: usize) -> bool;
    
    /// Get null count
    fn null_count(&self) -> usize;
    
    /// Set element to null
    fn set_null(&mut self, index: usize);
    
    /// Check if any element is null
    fn has_nulls(&self) -> bool;
}

/// Compression support trait
pub trait CompressionSupport<T> {
    /// Compress collection
    fn compress(&mut self) -> Result<(), CompressionError>;
    
    /// Decompress collection
    fn decompress(&mut self) -> Result<(), CompressionError>;
    
    /// Get compression ratio
    fn compression_ratio(&self) -> f64;
    
    /// Check if compressed
    fn is_compressed(&self) -> bool;
}

/// Caching support trait
pub trait CachingSupport<T> {
    /// Enable caching
    fn enable_caching(&mut self) -> Result<(), CachingError>;
    
    /// Disable caching
    fn disable_caching(&mut self);
    
    /// Check if caching enabled
    fn is_caching_enabled(&self) -> bool;
    
    /// Clear cache
    fn clear_cache(&mut self);
}

/// Parallel processing support trait
pub trait ParallelSupport<T> {
    /// Enable parallel processing
    fn enable_parallel(&mut self) -> Result<(), ParallelError>;
    
    /// Disable parallel processing
    fn disable_parallel(&mut self);
    
    /// Check if parallel processing enabled
    fn is_parallel_enabled(&self) -> bool;
    
    /// Set number of threads
    fn set_threads(&mut self, threads: usize);
}

/// Error types
#[derive(Debug, thiserror::Error)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Unsupported compression algorithm: {0}")]
    UnsupportedAlgorithm(String),
}

#[derive(Debug, thiserror::Error)]
pub enum CachingError {
    #[error("Caching failed: {0}")]
    CachingFailed(String),
    #[error("Cache initialization failed: {0}")]
    CacheInitFailed(String),
    #[error("Cache eviction failed: {0}")]
    CacheEvictionFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParallelError {
    #[error("Parallel processing failed: {0}")]
    ParallelFailed(String),
    #[error("Thread creation failed: {0}")]
    ThreadCreationFailed(String),
    #[error("Thread synchronization failed: {0}")]
    ThreadSyncFailed(String),
}
