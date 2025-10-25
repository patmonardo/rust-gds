//! Enhanced Vec implementation for Collections

use crate::collections::traits::Collections;
use std::iter::Sum;

/// Enhanced Vec implementation with Collections API
pub struct EnhancedVec<T> {
    data: Vec<T>,
}

impl<T> EnhancedVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

impl<T> Collections<T> for EnhancedVec<T> 
where
    T: Clone + Default + Ord + Sum + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.data.get(index).map(|x| x.clone())
    }

    fn set(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, T::default());
            self.data[index] = value;
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn sum(&self) -> Option<T> where T: Sum {
        Some(self.data.iter().cloned().sum())
    }

    fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            // For generic types, we can't cast to f64 directly
            // This is a limitation of the generic implementation
            None
        }
    }

    fn min(&self) -> Option<T> where T: Ord {
        self.data.iter().min().cloned()
    }

    fn max(&self) -> Option<T> where T: Ord {
        self.data.iter().max().cloned()
    }

    fn std_dev(&self) -> Option<f64> {
        // For generic types, we can't calculate std_dev
        // This is a limitation of the generic implementation
        None
    }

    fn variance(&self) -> Option<f64> {
        // For generic types, we can't calculate variance
        // This is a limitation of the generic implementation
        None
    }

    fn median(&self) -> Option<T> where T: Ord {
        if self.data.is_empty() {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort();
            let mid = sorted.len() / 2;
            Some(sorted[mid].clone())
        }
    }

    fn percentile(&self, p: f64) -> Option<T> where T: Ord {
        if self.data.is_empty() || p < 0.0 || p > 100.0 {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort();
            let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
            Some(sorted[index].clone())
        }
    }

    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord {
        self.data.binary_search(key)
    }

    fn sort(&mut self) where T: Ord {
        self.data.sort();
    }

    fn to_vec(self) -> Vec<T> {
        self.data
    }

    fn as_slice(&self) -> &[T] {
        &self.data
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> T { T::default() }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { 
        crate::types::ValueType::Long // Generic fallback
    }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: T) -> Self {
        Self { data: vec![default_value; count] }
    }
}
