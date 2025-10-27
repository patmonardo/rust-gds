//! Arrow Collections: Arrow-compatible Collections with Compute Kernels
//!
//! This module provides Collections implementations that are compatible with
//! Apache Arrow and implement SIMD-accelerated compute kernels.

use crate::collections::traits::Collections;
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use std::iter::Sum;

/// Arrow-compatible i32 collection
pub struct ArrowIntArray {
    data: Vec<i32>,
    null_bitmap: Option<Vec<u8>>,
    null_count: usize,
}

impl Default for ArrowIntArray {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowIntArray {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            null_bitmap: None,
            null_count: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            null_bitmap: None,
            null_count: 0,
        }
    }
    
    pub fn from_vec(data: Vec<i32>) -> Self {
        Self {
            data,
            null_bitmap: None,
            null_count: 0,
        }
    }
}

impl Collections<i32> for ArrowIntArray {
    fn get(&self, index: usize) -> Option<i32> {
        if index < self.data.len() {
            Some(self.data[index])
        } else {
            None
        }
    }
    
    fn set(&mut self, index: usize, value: i32) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, 0);
            self.data[index] = value;
        }
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn sum(&self) -> Option<i32> where i32: Sum {
        Some(self.data.iter().sum())
    }
    
    fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().map(|&x| x as f64).sum::<f64>() / self.data.len() as f64)
        }
    }
    
    fn min(&self) -> Option<i32> where i32: Ord {
        self.data.iter().min().cloned()
    }
    
    fn max(&self) -> Option<i32> where i32: Ord {
        self.data.iter().max().cloned()
    }
    
    fn std_dev(&self) -> Option<f64> {
        if self.data.len() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.data.iter()
                .map(|&x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.data.len() - 1) as f64;
            Some(variance.sqrt())
        }
    }
    
    fn variance(&self) -> Option<f64> {
        if self.data.len() < 2 {
            None
        } else {
            let mean = self.mean()?;
            Some(self.data.iter()
                .map(|&x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.data.len() - 1) as f64)
        }
    }
    
    fn median(&self) -> Option<i32> where i32: Ord {
        if self.data.is_empty() {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort();
            let mid = sorted.len() / 2;
            if sorted.len().is_multiple_of(2) {
                Some((sorted[mid - 1] + sorted[mid]) / 2)
            } else {
                Some(sorted[mid])
            }
        }
    }
    
    fn percentile(&self, p: f64) -> Option<i32> where i32: Ord {
        if self.data.is_empty() || !(0.0..=100.0).contains(&p) {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort();
            let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
            Some(sorted[index])
        }
    }
    
    fn binary_search(&self, key: &i32) -> Result<usize, usize> where i32: Ord {
        self.data.binary_search(key)
    }
    
    fn sort(&mut self) where i32: Ord {
        self.data.sort();
    }
    
    fn to_vec(self) -> Vec<i32> {
        self.data
    }
    
    fn as_slice(&self) -> &[i32] {
        &self.data
    }
    
    fn is_null(&self, index: usize) -> bool {
        if let Some(ref bitmap) = self.null_bitmap {
            if index < self.data.len() {
                let byte_index = index / 8;
                let bit_index = index % 8;
                if byte_index < bitmap.len() {
                    return (bitmap[byte_index] & (1 << bit_index)) == 0;
                }
            }
        }
        false
    }
    
    fn null_count(&self) -> usize {
        self.null_count
    }
    
    fn default_value(&self) -> i32 {
        0
    }
    
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    fn element_count(&self) -> usize {
        self.data.len()
    }
    
    fn backend(&self) -> CollectionsBackend {
        CollectionsBackend::Arrow
    }
    
    fn features(&self) -> &[Extension] {
        &[]
    }
    
    fn extensions(&self) -> &[Extension] {
        &[]
    }
    
    fn value_type(&self) -> ValueType {
        ValueType::Int
    }
    
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    
    fn with_defaults(count: usize, default_value: i32) -> Self {
        Self {
            data: vec![default_value; count],
            null_bitmap: None,
            null_count: 0,
        }
    }
}
