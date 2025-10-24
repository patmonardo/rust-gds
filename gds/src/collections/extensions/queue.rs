//! Collections Queue Extensions
//!
//! Repackages GDS Queue utilities as Collections Extensions for the Collections First approach.
//! This provides queue capabilities as extensions to any Collections implementation.

use crate::collections::traits::Collections;
use crate::collections::config::Extension;
use crate::core::utils::queue::{
    BoundedLongLongPriorityQueue, BoundedLongPriorityQueue, HugeLongPriorityQueue
};
use std::marker::PhantomData;

/// Queue extension trait for Collections
pub trait QueueSupport<T> {
    /// Enable priority queue functionality
    fn enable_priority_queue(&mut self, capacity: usize, order: QueueOrder) -> Result<(), QueueError>;
    
    /// Disable queue functionality
    fn disable_queue(&mut self);
    
    /// Check if queue is enabled
    fn is_queue_enabled(&self) -> bool;
    
    /// Get queue capacity
    fn queue_capacity(&self) -> Option<usize>;
    
    /// Get queue size
    fn queue_size(&self) -> usize;
    
    /// Check if queue is empty
    fn is_queue_empty(&self) -> bool;
    
    /// Check if queue is full
    fn is_queue_full(&self) -> bool;
}

/// Queue order for priority queues
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueueOrder {
    /// Min priority queue (lower values have higher priority)
    Min,
    /// Max priority queue (higher values have higher priority)
    Max,
}

/// Queue configuration
#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub capacity: usize,
    pub order: QueueOrder,
    pub enable_bounded: bool,
    pub enable_huge: bool,
    pub enable_priority: bool,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            capacity: 1000,
            order: QueueOrder::Min,
            enable_bounded: true,
            enable_huge: false,
            enable_priority: true,
        }
    }
}

/// Queue-enabled collection wrapper
pub struct QueueCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    queue_config: Option<QueueConfig>,
    is_queue_enabled: bool,
    bounded_queue: Option<BoundedLongPriorityQueue>,
    huge_queue: Option<HugeLongPriorityQueue>,
    _phantom: PhantomData<T>,
}

impl<T, C> QueueCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            queue_config: None,
            is_queue_enabled: false,
            bounded_queue: None,
            huge_queue: None,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_queue_config(inner: C, config: QueueConfig) -> Self {
        Self {
            inner,
            queue_config: Some(config),
            is_queue_enabled: false,
            bounded_queue: None,
            huge_queue: None,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for QueueCollection<T, C>
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
        self.inner.sort();
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
    
    fn backend(&self) -> crate::collections::config::CollectionsBackend {
        self.inner.backend()
    }
    
    fn features(&self) -> &[crate::collections::config::Extension] {
        &[Extension::Queue]
    }
    
    fn extensions(&self) -> &[crate::collections::config::Extension] {
        &[Extension::Queue]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for queue collections
        todo!("Implement with_capacity for QueueCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for queue collections
        todo!("Implement with_defaults for QueueCollection")
    }
}

impl<T, C> QueueSupport<T> for QueueCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_priority_queue(&mut self, capacity: usize, order: QueueOrder) -> Result<(), QueueError> {
        let config = QueueConfig {
            capacity,
            order,
            enable_bounded: true,
            enable_huge: capacity > 100_000, // Use huge queue for large capacities
            enable_priority: true,
        };
        
        if config.enable_huge {
            let huge_queue = match order {
                QueueOrder::Min => HugeLongPriorityQueue::min(capacity),
                QueueOrder::Max => HugeLongPriorityQueue::max(capacity),
            };
            self.huge_queue = Some(huge_queue);
        } else {
            let bounded_queue = match order {
                QueueOrder::Min => BoundedLongPriorityQueue::min(capacity),
                QueueOrder::Max => BoundedLongPriorityQueue::max(capacity),
            };
            self.bounded_queue = Some(bounded_queue);
        }
        
        self.queue_config = Some(config);
        self.is_queue_enabled = true;
        
        Ok(())
    }
    
    fn disable_queue(&mut self) {
        self.queue_config = None;
        self.is_queue_enabled = false;
        self.bounded_queue = None;
        self.huge_queue = None;
    }
    
    fn is_queue_enabled(&self) -> bool {
        self.is_queue_enabled
    }
    
    fn queue_capacity(&self) -> Option<usize> {
        self.queue_config.as_ref().map(|c| c.capacity)
    }
    
    fn queue_size(&self) -> usize {
        if let Some(ref bounded) = self.bounded_queue {
            bounded.size()
        } else if let Some(ref huge) = self.huge_queue {
            huge.size()
        } else {
            0
        }
    }
    
    fn is_queue_empty(&self) -> bool {
        if let Some(ref bounded) = self.bounded_queue {
            bounded.size() == 0
        } else if let Some(ref huge) = self.huge_queue {
            huge.is_empty()
        } else {
            true
        }
    }
    
    fn is_queue_full(&self) -> bool {
        if let Some(ref bounded) = self.bounded_queue {
            bounded.size() >= self.queue_capacity().unwrap_or(0)
        } else if let Some(ref huge) = self.huge_queue {
            huge.size() >= self.queue_capacity().unwrap_or(0)
        } else {
            false
        }
    }
}

/// Queue error types
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("Queue initialization failed: {0}")]
    InitFailed(String),
    #[error("Queue operation failed: {0}")]
    OperationFailed(String),
    #[error("Queue is full: {0}")]
    QueueFull(String),
    #[error("Queue is empty: {0}")]
    QueueEmpty(String),
    #[error("Invalid queue configuration: {0}")]
    InvalidConfig(String),
}

/// Queue utilities
pub struct QueueUtils;

impl QueueUtils {
    /// Estimate memory usage for queue
    pub fn estimate_queue_memory(capacity: usize, use_huge: bool) -> usize {
        if use_huge {
            HugeLongPriorityQueue::memory_estimation(capacity)
        } else {
            // Bounded queue memory estimation
            capacity * (std::mem::size_of::<i64>() + std::mem::size_of::<f64>())
        }
    }
    
    /// Choose optimal queue type based on capacity
    pub fn choose_queue_type(capacity: usize) -> QueueConfig {
        QueueConfig {
            capacity,
            order: QueueOrder::Min,
            enable_bounded: capacity <= 100_000,
            enable_huge: capacity > 100_000,
            enable_priority: true,
        }
    }
    
    /// Calculate optimal capacity for given data size
    pub fn optimal_capacity(data_size: usize) -> usize {
        // Use power of 2 capacities for optimal performance
        let mut capacity = 1024;
        while capacity < data_size && capacity < 1_000_000 {
            capacity *= 2;
        }
        capacity.min(data_size)
    }
}
