//! Collections Random Extensions
//!
//! Repackages GDS Random utilities as Collections Extensions for the Collections First approach.
//! This provides random generation and shuffling capabilities as extensions to any Collections implementation.

use crate::collections::traits::Collections;
use crate::collections::config::Extension;
use crate::core::utils::shuffle::{ShuffleUtil, SplittableRandom, Random};
use std::marker::PhantomData;

/// Random extension trait for Collections
pub trait RandomSupport<T> {
    /// Enable random functionality
    fn enable_random(&mut self, seed: Option<u64>) -> Result<(), RandomError>;
    
    /// Disable random functionality
    fn disable_random(&mut self);
    
    /// Check if random is enabled
    fn is_random_enabled(&self) -> bool;
    
    /// Shuffle the collection
    fn shuffle(&mut self) -> Result<(), RandomError>;
    
    /// Generate random data
    fn generate_random(&mut self, count: usize) -> Result<(), RandomError>;
    
    /// Get random element
    fn random_element(&mut self) -> Option<T>;
    
    /// Get random sample
    fn random_sample(&mut self, size: usize) -> Vec<T>;
    
    /// Split random generator
    fn split_random(&mut self) -> Result<SplittableRandom, RandomError>;
}

/// Random configuration
#[derive(Debug, Clone)]
pub struct RandomConfig {
    pub seed: Option<u64>,
    pub enable_shuffling: bool,
    pub enable_generation: bool,
    pub enable_sampling: bool,
    pub enable_splitting: bool,
}

impl Default for RandomConfig {
    fn default() -> Self {
        Self {
            seed: None,
            enable_shuffling: true,
            enable_generation: true,
            enable_sampling: true,
            enable_splitting: true,
        }
    }
}

/// Random-enabled collection wrapper
pub struct RandomCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    random_config: Option<RandomConfig>,
    is_random_enabled: bool,
    random_generator: Option<SplittableRandom>,
    _phantom: PhantomData<T>,
}

impl<T, C> RandomCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            random_config: None,
            is_random_enabled: false,
            random_generator: None,
            _phantom: PhantomData,
        }
    }
    
    pub fn with_random_config(inner: C, config: RandomConfig) -> Self {
        Self {
            inner,
            random_config: Some(config),
            is_random_enabled: false,
            random_generator: None,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for RandomCollection<T, C>
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
        &[Extension::Random]
    }
    
    fn extensions(&self) -> &[crate::collections::config::Extension] {
        &[Extension::Random]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for random collections
        todo!("Implement with_capacity for RandomCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for random collections
        todo!("Implement with_defaults for RandomCollection")
    }
}

impl<T, C> RandomSupport<T> for RandomCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_random(&mut self, seed: Option<u64>) -> Result<(), RandomError> {
        let config = RandomConfig {
            seed,
            enable_shuffling: true,
            enable_generation: true,
            enable_sampling: true,
            enable_splitting: true,
        };
        
        self.random_generator = Some(SplittableRandom::with_seed(seed));
        self.random_config = Some(config);
        self.is_random_enabled = true;
        
        Ok(())
    }
    
    fn disable_random(&mut self) {
        self.random_config = None;
        self.is_random_enabled = false;
        self.random_generator = None;
    }
    
    fn is_random_enabled(&self) -> bool {
        self.is_random_enabled
    }
    
    fn shuffle(&mut self) -> Result<(), RandomError> {
        if !self.is_random_enabled {
            return Err(RandomError::RandomNotEnabled);
        }
        
        if let Some(ref mut rng) = self.random_generator {
            // Use Fisher-Yates shuffle algorithm directly on the collection
            let len = self.inner.len();
            for i in (1..len).rev() {
                let j = rng.next_long(0, i + 1);
                // Swap elements at positions i and j
                if let (Some(val_i), Some(val_j)) = (self.inner.get(i), self.inner.get(j)) {
                    self.inner.set(i, val_j.clone());
                    self.inner.set(j, val_i.clone());
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_random(&mut self, count: usize) -> Result<(), RandomError> {
        if !self.is_random_enabled {
            return Err(RandomError::RandomNotEnabled);
        }
        
        if let Some(ref mut rng) = self.random_generator {
            // Generate random data based on the value type
            for i in 0..count {
                let random_value = Self::generate_random_value_static(rng)?;
                self.inner.set(i, random_value);
            }
        }
        
        Ok(())
    }
    
    fn random_element(&mut self) -> Option<T> {
        if !self.is_random_enabled || self.inner.is_empty() {
            return None;
        }
        
        if let Some(ref mut rng) = self.random_generator {
            let index = rng.next_long(0, self.inner.len());
            self.inner.get(index)
        } else {
            None
        }
    }
    
    fn random_sample(&mut self, size: usize) -> Vec<T> {
        if !self.is_random_enabled || self.inner.is_empty() {
            return Vec::new();
        }
        
        if let Some(ref mut rng) = self.random_generator {
            let mut sample = Vec::new();
            let collection_size = self.inner.len();
            
            for _ in 0..size {
                let index = rng.next_long(0, collection_size);
                if let Some(value) = self.inner.get(index) {
                    sample.push(value);
                }
            }
            
            sample
        } else {
            Vec::new()
        }
    }
    
    fn split_random(&mut self) -> Result<SplittableRandom, RandomError> {
        if !self.is_random_enabled {
            return Err(RandomError::RandomNotEnabled);
        }
        
        if let Some(ref mut rng) = self.random_generator {
            Ok(rng.split())
        } else {
            Err(RandomError::RandomNotEnabled)
        }
    }
}

impl<T, C> RandomCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    /// Generate a random value based on the collection's value type
    fn generate_random_value_static(rng: &mut SplittableRandom) -> Result<T, RandomError> {
        // This is a simplified implementation - in practice, you'd need proper type conversion
        // For now, we'll use a placeholder implementation
        Err(RandomError::GenerationFailed("Type-specific random generation not implemented".to_string()))
    }
}

/// Random error types
#[derive(Debug, thiserror::Error)]
pub enum RandomError {
    #[error("Random initialization failed: {0}")]
    InitFailed(String),
    #[error("Random operation failed: {0}")]
    OperationFailed(String),
    #[error("Random generation failed: {0}")]
    GenerationFailed(String),
    #[error("Random not enabled")]
    RandomNotEnabled,
    #[error("Shuffling failed: {0}")]
    ShufflingFailed(String),
    #[error("Sampling failed: {0}")]
    SamplingFailed(String),
}

/// Random utilities
pub struct RandomUtils;

impl RandomUtils {
    /// Create default random configuration
    pub fn default_config() -> RandomConfig {
        RandomConfig::default()
    }
    
    /// Create random configuration with seed
    pub fn with_seed(seed: u64) -> RandomConfig {
        RandomConfig {
            seed: Some(seed),
            enable_shuffling: true,
            enable_generation: true,
            enable_sampling: true,
            enable_splitting: true,
        }
    }
    
    /// Create random configuration for testing
    pub fn testing_config() -> RandomConfig {
        RandomConfig {
            seed: Some(42), // Fixed seed for reproducible tests
            enable_shuffling: true,
            enable_generation: true,
            enable_sampling: true,
            enable_splitting: true,
        }
    }
    
    /// Estimate memory usage for random operations
    pub fn estimate_random_memory(collection_size: usize) -> usize {
        // Random operations typically don't require additional memory
        // beyond the collection itself
        0
    }
    
    /// Calculate optimal sample size for given collection size
    pub fn optimal_sample_size(collection_size: usize) -> usize {
        // Use square root of collection size as a reasonable sample size
        (collection_size as f64).sqrt() as usize
    }
}
