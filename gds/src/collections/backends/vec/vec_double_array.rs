//! VecDoubleArray: Vec-based Option<Vec<f64>> Collections implementation

use crate::collections::traits::{Collections, PropertyValuesAdapter};
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;

/// Vec-based Option<Vec<f64>> Collections implementation for arrays
#[derive(Debug, Clone)]
pub struct VecDoubleArray {
    pub data: Vec<Option<Vec<f64>>>,
}

impl Default for VecDoubleArray {
    fn default() -> Self {
        Self::new()
    }
}

impl VecDoubleArray {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

impl From<Vec<Option<Vec<f64>>>> for VecDoubleArray {
    fn from(data: Vec<Option<Vec<f64>>>) -> Self {
        Self { data }
    }
}

impl Collections<Option<Vec<f64>>> for VecDoubleArray {
    fn get(&self, index: usize) -> Option<Option<Vec<f64>>> {
        self.data.get(index).cloned()
    }

    fn set(&mut self, index: usize, value: Option<Vec<f64>>) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, None);
            self.data[index] = value;
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    // Arrays don't support statistical operations - return None
    fn sum(&self) -> Option<Option<Vec<f64>>> { None }
    fn mean(&self) -> Option<f64> { None }
    fn std_dev(&self) -> Option<f64> { None }
    fn variance(&self) -> Option<f64> { None }
    fn min(&self) -> Option<Option<Vec<f64>>> { None }
    fn max(&self) -> Option<Option<Vec<f64>>> { None }
    fn median(&self) -> Option<Option<Vec<f64>>> { None }
    fn percentile(&self, _p: f64) -> Option<Option<Vec<f64>>> { None }
    
    fn binary_search(&self, _value: &Option<Vec<f64>>) -> Result<usize, usize> {
        Err(0) // Arrays don't support search
    }

    fn sort(&mut self) {
        // Arrays don't support sorting
    }

    fn to_vec(self) -> Vec<Option<Vec<f64>>> {
        self.data
    }

    fn as_slice(&self) -> &[Option<Vec<f64>>] {
        &self.data
    }

    fn null_count(&self) -> usize {
        self.data.iter().filter(|x| x.is_none()).count()
    }

    fn default_value(&self) -> Option<Vec<f64>> {
        None
    }

    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }

    fn is_null(&self, index: usize) -> bool {
        index >= self.data.len() || self.data[index].is_none()
    }

    fn backend(&self) -> CollectionsBackend {
        CollectionsBackend::Vec
    }

    fn features(&self) -> &[Extension] {
        &[]
    }

    fn extensions(&self) -> &[Extension] {
        &[]
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn with_defaults(count: usize, _default_value: Option<Vec<f64>>) -> Self {
        Self {
            data: vec![None; count],
        }
    }
}

impl PropertyValuesAdapter<Option<Vec<f64>>> for VecDoubleArray {}

