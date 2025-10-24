//! VecDouble: Vec-based f64 Collections implementation

use crate::collections::traits::Collections;
use std::iter::Sum;

/// Vec-based f64 Collections implementation
pub struct VecDouble {
    pub data: Vec<f64>,
}

impl VecDouble {
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

    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.data.pop()
    }
}

impl Collections<f64> for VecDouble {
    fn get(&self, index: usize) -> Option<f64> {
        self.data.get(index).map(|x| *x)
    }

    fn set(&mut self, index: usize, value: f64) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, 0.0f64);
            self.data[index] = value;
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn sum(&self) -> Option<f64> where f64: Sum {
        Some(self.data.iter().cloned().sum())
    }

    fn mean(&self) -> Option<f64> where f64: Into<f64> {
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

    fn min(&self) -> Option<f64> {
        self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
    }

    fn max(&self) -> Option<f64> {
        self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).cloned()
    }

    fn std_dev(&self) -> Option<f64> where f64: Into<f64> {
        if self.data.len() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.data.iter()
                .map(|&x| {
                    let diff = x - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.data.len() - 1) as f64;
            Some(variance.sqrt())
        }
    }

    fn variance(&self) -> Option<f64> where f64: Into<f64> {
        if self.data.len() < 2 {
            None
        } else {
            let mean = self.mean()?;
            Some(self.data.iter()
                .map(|&x| {
                    let diff = x - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.data.len() - 1) as f64)
        }
    }

    fn median(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                Some((sorted[mid - 1] + sorted[mid]) / 2.0)
            } else {
                Some(sorted[mid])
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<f64> {
        if self.data.is_empty() || p < 0.0 || p > 100.0 {
            None
        } else {
            let mut sorted = self.data.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
            Some(sorted[index])
        }
    }

    fn binary_search(&self, key: &f64) -> Result<usize, usize> {
        self.data.binary_search_by(|x| x.partial_cmp(key).unwrap())
    }

    fn sort(&mut self) {
        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn to_vec(self) -> Vec<f64> {
        self.data
    }

    fn as_slice(&self) -> &[f64] {
        &self.data
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> f64 { 0.0 }
    fn backend(&self) -> crate::collections::config::CollectionsBackend { 
        crate::collections::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::collections::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::collections::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Double }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: f64) -> Self {
        Self { data: vec![default_value; count] }
    }
}
