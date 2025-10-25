//! Collections Stack Extensions
//!
//! Repackages GDS Stack utilities as Collections Extensions for the Collections First approach.
//! This provides stack capabilities as extensions to any Collections implementation.

use crate::collections::traits::Collections;
use crate::config::Extension;
use crate::core::utils::paged::{
    HugeLongArrayStack, PagedLongStack
};
use std::marker::PhantomData;

/// Stack extension trait for Collections
pub trait StackSupport<T> {
    /// Enable stack functionality
    fn enable_stack(&mut self, capacity: usize) -> Result<(), StackError>;
    
    /// Disable stack functionality
    fn disable_stack(&mut self);
    
    /// Check if stack is enabled
    fn is_stack_enabled(&self) -> bool;
    
    /// Get stack capacity
    fn stack_capacity(&self) -> Option<usize>;
    
    /// Get stack size
    fn stack_size(&self) -> usize;
    
    /// Check if stack is empty
    fn is_stack_empty(&self) -> bool;
    
    /// Check if stack is full
    fn is_stack_full(&self) -> bool;
    
    /// Push element onto stack
    fn push(&mut self, value: T) -> Result<(), StackError>;
    
    /// Pop element from stack
    fn pop(&mut self) -> Option<T>;
    
    /// Peek at top element without removing
    fn peek(&self) -> Option<T>;
}

/// Stack configuration
#[derive(Debug, Clone)]
pub struct StackConfig {
    pub capacity: usize,
    pub enable_huge: bool,
    pub enable_paged: bool,
    pub enable_double: bool,
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            capacity: 1000,
            enable_huge: false,
            enable_paged: false,
            enable_double: false,
        }
    }
}

/// Stack-enabled collection wrapper
pub struct StackCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    stack_config: Option<StackConfig>,
    is_stack_enabled: bool,
    huge_stack: Option<HugeLongArrayStack>,
    paged_stack: Option<PagedLongStack>,
    _phantom: PhantomData<T>,
}

impl<T, C> StackCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            stack_config: None,
            is_stack_enabled: false,
            huge_stack: None,
            paged_stack: None,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_stack_config(inner: C, config: StackConfig) -> Self {
        Self {
            inner,
            stack_config: Some(config),
            is_stack_enabled: false,
            huge_stack: None,
            paged_stack: None,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for StackCollection<T, C>
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
    
    fn backend(&self) -> crate::config::CollectionsBackend {
        self.inner.backend()
    }
    
    fn features(&self) -> &[crate::config::Extension] {
        &[Extension::Stack]
    }
    
    fn extensions(&self) -> &[crate::config::Extension] {
        &[Extension::Stack]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for stack collections
        todo!("Implement with_capacity for StackCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for stack collections
        todo!("Implement with_defaults for StackCollection")
    }
}

impl<T, C> StackSupport<T> for StackCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_stack(&mut self, capacity: usize) -> Result<(), StackError> {
        let config = StackConfig {
            capacity,
            enable_huge: capacity > 100_000, // Use huge stack for large capacities
            enable_paged: capacity > 1_000_000, // Use paged stack for massive capacities
            enable_double: false, // Default to long stack
        };
        
        if config.enable_paged {
            // Use paged stack for massive capacities
            let paged_stack = PagedLongStack::new(capacity);
            self.paged_stack = Some(paged_stack);
        } else if config.enable_huge {
            // Use huge stack for large capacities
            let huge_stack = HugeLongArrayStack::new(capacity);
            self.huge_stack = Some(huge_stack);
        }
        
        self.stack_config = Some(config);
        self.is_stack_enabled = true;
        
        Ok(())
    }
    
    fn disable_stack(&mut self) {
        self.stack_config = None;
        self.is_stack_enabled = false;
        self.huge_stack = None;
        self.paged_stack = None;
    }
    
    fn is_stack_enabled(&self) -> bool {
        self.is_stack_enabled
    }
    
    fn stack_capacity(&self) -> Option<usize> {
        self.stack_config.as_ref().map(|c| c.capacity)
    }
    
    fn stack_size(&self) -> usize {
        if let Some(ref huge) = self.huge_stack {
            huge.size()
        } else if let Some(ref paged) = self.paged_stack {
            paged.size()
        } else {
            0
        }
    }
    
    fn is_stack_empty(&self) -> bool {
        if let Some(ref huge) = self.huge_stack {
            huge.is_empty()
        } else if let Some(ref paged) = self.paged_stack {
            paged.is_empty()
        } else {
            true
        }
    }
    
    fn is_stack_full(&self) -> bool {
        if let Some(ref huge) = self.huge_stack {
            huge.size() >= self.stack_capacity().unwrap_or(0)
        } else if let Some(ref paged) = self.paged_stack {
            paged.size() >= self.stack_capacity().unwrap_or(0)
        } else {
            false
        }
    }
    
    fn push(&mut self, value: T) -> Result<(), StackError> {
        if !self.is_stack_enabled {
            return Err(StackError::StackNotEnabled);
        }
        
        if self.is_stack_full() {
            return Err(StackError::StackFull);
        }
        
        // Convert T to i64 for stack operations
        let stack_value = self.convert_to_stack_value(value)?;
        
        if let Some(ref mut huge) = self.huge_stack {
            huge.push(stack_value);
        } else if let Some(ref mut paged) = self.paged_stack {
            paged.push(stack_value);
        }
        
        Ok(())
    }
    
    fn pop(&mut self) -> Option<T> {
        if !self.is_stack_enabled || self.is_stack_empty() {
            return None;
        }
        
        let stack_value = if let Some(ref mut huge) = self.huge_stack {
            huge.pop()
        } else if let Some(ref mut paged) = self.paged_stack {
            paged.pop()
        } else {
            return None;
        };
        
        self.convert_from_stack_value(stack_value)
    }
    
    fn peek(&self) -> Option<T> {
        if !self.is_stack_enabled || self.is_stack_empty() {
            return None;
        }
        
        let stack_value = if let Some(ref huge) = self.huge_stack {
            huge.peek()
        } else if let Some(ref paged) = self.paged_stack {
            paged.peek()
        } else {
            return None;
        };
        
        self.convert_from_stack_value(stack_value)
    }
}

impl<T, C> StackCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    /// Convert T to i64 for stack operations
    fn convert_to_stack_value(&self, value: T) -> Result<i64, StackError> {
        // This is a simplified conversion - in practice, you'd need proper type conversion
        // For now, we'll use a placeholder implementation
        Err(StackError::ConversionFailed("Type conversion not implemented".to_string()))
    }
    
    /// Convert i64 from stack operations back to T
    fn convert_from_stack_value(&self, value: i64) -> Option<T> {
        // This is a simplified conversion - in practice, you'd need proper type conversion
        // For now, we'll use a placeholder implementation
        None
    }
}

/// Stack error types
#[derive(Debug, thiserror::Error)]
pub enum StackError {
    #[error("Stack initialization failed: {0}")]
    InitFailed(String),
    #[error("Stack operation failed: {0}")]
    OperationFailed(String),
    #[error("Stack is full")]
    StackFull,
    #[error("Stack is empty")]
    StackEmpty,
    #[error("Stack not enabled")]
    StackNotEnabled,
    #[error("Type conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Invalid stack configuration: {0}")]
    InvalidConfig(String),
}

/// Stack utilities
pub struct StackUtils;

impl StackUtils {
    /// Estimate memory usage for stack
    pub fn estimate_stack_memory(capacity: usize, use_huge: bool, use_paged: bool) -> usize {
        if use_paged {
            // Paged stack memory estimation
            let page_size = 4096;
            let pages_needed = (capacity + page_size - 1) / page_size;
            pages_needed * page_size * std::mem::size_of::<i64>()
        } else if use_huge {
            // Huge stack memory estimation
            capacity * std::mem::size_of::<i64>()
        } else {
            // Regular stack memory estimation
            capacity * std::mem::size_of::<i64>()
        }
    }
    
    /// Choose optimal stack type based on capacity
    pub fn choose_stack_type(capacity: usize) -> StackConfig {
        StackConfig {
            capacity,
            enable_huge: capacity > 100_000,
            enable_paged: capacity > 1_000_000,
            enable_double: false,
        }
    }
    
    /// Calculate optimal capacity for given data size
    pub fn optimal_capacity(data_size: usize) -> usize {
        // Use power of 2 capacities for optimal performance
        let mut capacity = 1024;
        while capacity < data_size && capacity < 10_000_000 {
            capacity *= 2;
        }
        capacity.min(data_size)
    }
}
