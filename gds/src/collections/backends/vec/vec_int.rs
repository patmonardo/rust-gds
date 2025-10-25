//! VecInt: Vec-based i32 Collections implementation

use crate::collections::traits::Collections;
use std::iter::Sum;

/// Vec-based i32 Collections implementation
pub struct VecInt {
    pub data: Vec<i32>,
}

impl VecInt {
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

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
}

impl Collections<i32> for VecInt {
    fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).map(|x| *x)
    }

    fn set(&mut self, index: usize, value: i32) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, 0i32);
            self.data[index] = value;
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn sum(&self) -> Option<i32> where i32: Sum {
        Some(self.data.iter().cloned().sum())
    }

    fn mean(&self) -> Option<f64> where i32: Into<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(
                self.data.iter()
                    .map(|&x| x as f64)
                    .sum::<f64>() / self.data.len() as f64
            )
        }
    }

    fn min(&self) -> Option<i32> where i32: Ord {
        self.data.iter().min().cloned()
    }

    fn max(&self) -> Option<i32> where i32: Ord {
        self.data.iter().max().cloned()
    }

    fn std_dev(&self) -> Option<f64> where i32: Into<f64> {
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

    fn variance(&self) -> Option<f64> where i32: Into<f64> {
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
            if sorted.len() % 2 == 0 {
                Some((sorted[mid - 1] + sorted[mid]) / 2)
            } else {
                Some(sorted[mid])
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<i32> where i32: Ord {
        if self.data.is_empty() || p < 0.0 || p > 100.0 {
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

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> i32 { 0 }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Int }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: i32) -> Self {
        Self { data: vec![default_value; count] }
    }
}
