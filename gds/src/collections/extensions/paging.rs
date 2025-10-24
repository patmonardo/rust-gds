//! Collections Paging Extensions
//!
//! Repackages GDS Paged Utils as Collections Extensions for the Collections First approach.
//! This provides paging capabilities as extensions to any Collections implementation.

use crate::collections::traits::Collections;
use crate::collections::config::Extension;
use std::marker::PhantomData;

/// Paging extension trait for Collections
pub trait PagingSupport<T> {
    /// Enable paging for this collection
    fn enable_paging(&mut self, page_size: usize) -> Result<(), PagingError>;
    
    /// Disable paging
    fn disable_paging(&mut self);
    
    /// Check if paging is enabled
    fn is_paging_enabled(&self) -> bool;
    
    /// Get current page size
    fn page_size(&self) -> Option<usize>;
    
    /// Get memory usage estimate
    fn memory_estimate(&self) -> usize;
    
    /// Get page count
    fn page_count(&self) -> usize;
}

/// Paging configuration
#[derive(Debug, Clone)]
pub struct PagingConfig {
    pub page_size: usize,
    pub max_pages: Option<usize>,
    pub enable_atomic_operations: bool,
    pub enable_parallel_creation: bool,
}

impl Default for PagingConfig {
    fn default() -> Self {
        Self {
            page_size: 4096, // 4KB pages
            max_pages: None,
            enable_atomic_operations: true,
            enable_parallel_creation: true,
        }
    }
}

/// Paging-enabled collection wrapper
pub struct PagedCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    paging_config: Option<PagingConfig>,
    is_paging_enabled: bool,
    _phantom: PhantomData<T>,
}

impl<T, C> PagedCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            paging_config: None,
            is_paging_enabled: false,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_paging_config(inner: C, config: PagingConfig) -> Self {
        Self {
            inner,
            paging_config: Some(config),
            is_paging_enabled: false,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for PagedCollection<T, C>
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
    
    fn percentile(&self, _p: f64) -> Option<T> where T: Ord {
        self.inner.percentile(_p)
    }
    
    fn binary_search(&self, _key: &T) -> Result<usize, usize> where T: Ord {
        self.inner.binary_search(_key)
    }
    
    fn sort(&mut self) where T: Ord {
        self.inner.sort();
    }
    
    fn to_vec(self) -> Vec<T> {
        self.inner.to_vec()
    }
    
    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }
    
    fn is_null(&self, _index: usize) -> bool {
        self.inner.is_null(_index)
    }
    
    fn null_count(&self) -> usize {
        self.inner.null_count()
    }
    
    fn default_value(&self) -> T {
        self.inner.default_value()
    }
    
    fn backend(&self) -> crate::collections::config::CollectionsBackend {
        crate::collections::config::CollectionsBackend::Huge
    }
    
    fn features(&self) -> &[crate::collections::config::Extension] {
        &[Extension::Paging]
    }
    
    fn extensions(&self) -> &[crate::collections::config::Extension] {
        &[Extension::Paging]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for paged collections
        todo!("Implement with_capacity for PagedCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for paged collections
        todo!("Implement with_defaults for PagedCollection")
    }
}

impl<T, C> PagingSupport<T> for PagedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_paging(&mut self, page_size: usize) -> Result<(), PagingError> {
        let config = PagingConfig {
            page_size,
            max_pages: None,
            enable_atomic_operations: true,
            enable_parallel_creation: true,
        };
        
        self.paging_config = Some(config);
        self.is_paging_enabled = true;
        
        Ok(())
    }
    
    fn disable_paging(&mut self) {
        self.paging_config = None;
        self.is_paging_enabled = false;
    }
    
    fn is_paging_enabled(&self) -> bool {
        self.is_paging_enabled
    }
    
    fn page_size(&self) -> Option<usize> {
        self.paging_config.as_ref().map(|c| c.page_size)
    }
    
    fn memory_estimate(&self) -> usize {
        self.inner.len() * std::mem::size_of::<T>()
    }
    
    fn page_count(&self) -> usize {
        if let Some(ref config) = self.paging_config {
            (self.inner.len() + config.page_size - 1) / config.page_size
        } else {
            1
        }
    }
}

/// Paging error types
#[derive(Debug, thiserror::Error)]
pub enum PagingError {
    #[error("Paging initialization failed: {0}")]
    InitFailed(String),
    #[error("Page allocation failed: {0}")]
    AllocationFailed(String),
    #[error("Page access failed: {0}")]
    AccessFailed(String),
    #[error("Invalid page size: {0}")]
    InvalidPageSize(String),
}

/// Paging utilities
pub struct PagingUtils;

impl PagingUtils {
    /// Calculate optimal page size for given data size
    pub fn optimal_page_size(data_size: usize) -> usize {
        // Use power of 2 page sizes for optimal bit manipulation
        let mut page_size = 1024; // Start with 1KB
        while page_size < data_size && page_size < 65536 {
            page_size *= 2;
        }
        page_size
    }
    
    /// Estimate memory usage for paged collection
    pub fn estimate_memory<T>(element_count: usize, page_size: usize) -> usize {
        let pages_needed = (element_count + page_size - 1) / page_size;
        let page_overhead = 24; // Array header overhead
        let total_overhead = pages_needed * page_overhead;
        let data_size = element_count * std::mem::size_of::<T>();
        total_overhead + data_size
    }
    
    /// Check if paging is beneficial for given data size
    pub fn should_enable_paging<T>(data_size: usize) -> bool {
        // Enable paging for large datasets (>1MB)
        data_size * std::mem::size_of::<T>() > 1024 * 1024
    }
}