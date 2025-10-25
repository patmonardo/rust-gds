//! Huge Collections: Paged Arrays for Billions of Elements
//!
//! Provides paged array implementations that can handle massive datasets
//! by splitting data across multiple pages with automatic single-page
//! vs multi-page selection.

// Declare modules
pub mod huge_int_array;
pub mod huge_long_array;
pub mod huge_double_array;
pub mod huge_float_array;
pub mod huge_byte_array;
pub mod huge_short_array;
pub mod huge_boolean_array;
pub mod huge_char_array;
pub mod huge_object_array;
pub mod huge_atomic_array;

// Re-export existing HugeArray types (backward compatibility)
pub use huge_int_array::HugeIntArray;
pub use huge_long_array::HugeLongArray;
pub use huge_double_array::HugeDoubleArray;
pub use huge_float_array::HugeFloatArray;
pub use huge_byte_array::HugeByteArray;
pub use huge_short_array::HugeShortArray;
pub use huge_boolean_array::HugeBooleanArray;
pub use huge_char_array::HugeCharArray;
pub use huge_object_array::HugeObjectArray;

// Re-export atomic array types
pub use huge_atomic_array::*;

// Re-export Collections trait implementations
pub use crate::collections::traits::Collections;
use std::iter::Sum;

// Implement Collections for existing HugeArray types
impl Collections<i32> for HugeIntArray { 
    fn get(&self, index: usize) -> Option<i32> {
        if index < self.size() {
            Some(self.get(index))
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: i32) {
        if index < self.size() {
            self.set(index, value);
        }
    }

    fn len(&self) -> usize {
        self.size()
    }

    fn sum(&self) -> Option<i32> where i32: Sum {
        Some(self.iter().sum())
    }

    fn mean(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            Some(
                self.iter()
                    .map(|x| x as f64)
                    .sum::<f64>() / self.size() as f64
            )
        }
    }

    fn min(&self) -> Option<i32> where i32: Ord {
        if self.size() == 0 {
            None
        } else {
            self.iter().min()
        }
    }

    fn max(&self) -> Option<i32> where i32: Ord {
        if self.size() == 0 {
            None
        } else {
            self.iter().max()
        }
    }

    fn std_dev(&self) -> Option<f64> where i32: Into<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.iter()
                .map(|x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64;
            Some(variance.sqrt())
        }
    }

    fn variance(&self) -> Option<f64> where i32: Into<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            Some(self.iter()
                .map(|x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64)
        }
    }

    fn median(&self) -> Option<i32> where i32: Ord {
        if self.size() == 0 {
            None
        } else {
            let mut values: Vec<i32> = self.iter().collect();
            values.sort();
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                Some((values[mid - 1] + values[mid]) / 2)
            } else {
                Some(values[mid])
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<i32> where i32: Ord {
        if self.size() == 0 || p < 0.0 || p > 100.0 {
            None
        } else {
            let mut values: Vec<i32> = self.iter().collect();
            values.sort();
            let index = (p / 100.0 * (values.len() - 1) as f64).round() as usize;
            Some(values[index])
        }
    }

    fn binary_search(&self, key: &i32) -> Result<usize, usize> where i32: Ord {
        // For HugeArray, we need to collect into a Vec for binary search
        let values: Vec<i32> = self.iter().collect();
        values.binary_search(key)
    }

    fn sort(&mut self) where i32: Ord {
        // HugeArray doesn't support in-place sorting
        // This would require a complete reimplementation
        // For now, we'll leave this as a no-op
    }

    fn to_vec(self) -> Vec<i32> {
        self.iter().collect()
    }

    fn as_slice(&self) -> &[i32] {
        // HugeArray doesn't support slices, return empty
        &[]
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> i32 { 0 }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Huge 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Int }
    fn with_capacity(capacity: usize) -> Self {
        Self::new(capacity)
    }
    fn with_defaults(count: usize, default_value: i32) -> Self {
        let mut array = Self::new(count);
        for i in 0..count {
            array.set(i, default_value);
        }
        array
    }
}

impl Collections<i64> for HugeLongArray { 
    fn get(&self, index: usize) -> Option<i64> {
        if index < self.size() {
            Some(self.get(index))
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: i64) {
        if index < self.size() {
            self.set(index, value);
        }
    }

    fn len(&self) -> usize {
        self.size()
    }

    fn sum(&self) -> Option<i64> where i64: Sum {
        Some(self.iter().sum())
    }

    fn mean(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            Some(
                self.iter()
                    .map(|x| x as f64)
                    .sum::<f64>() / self.size() as f64
            )
        }
    }

    fn min(&self) -> Option<i64> where i64: Ord {
        if self.size() == 0 {
            None
        } else {
            self.iter().min()
        }
    }

    fn max(&self) -> Option<i64> where i64: Ord {
        if self.size() == 0 {
            None
        } else {
            self.iter().max()
        }
    }

    fn std_dev(&self) -> Option<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.iter()
                .map(|x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64;
            Some(variance.sqrt())
        }
    }

    fn variance(&self) -> Option<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            Some(self.iter()
                .map(|x| {
                    let diff = x as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64)
        }
    }

    fn median(&self) -> Option<i64> where i64: Ord {
        if self.size() == 0 {
            None
        } else {
            let mut values: Vec<i64> = self.iter().collect();
            values.sort();
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                Some((values[mid - 1] + values[mid]) / 2)
            } else {
                Some(values[mid])
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<i64> where i64: Ord {
        if self.size() == 0 || p < 0.0 || p > 100.0 {
            None
        } else {
            let mut values: Vec<i64> = self.iter().collect();
            values.sort();
            let index = (p / 100.0 * (values.len() - 1) as f64).round() as usize;
            Some(values[index])
        }
    }

    fn binary_search(&self, key: &i64) -> Result<usize, usize> where i64: Ord {
        let values: Vec<i64> = self.iter().collect();
        values.binary_search(key)
    }

    fn sort(&mut self) where i64: Ord {
        // HugeArray doesn't support in-place sorting
    }

    fn to_vec(self) -> Vec<i64> {
        self.iter().collect()
    }

    fn as_slice(&self) -> &[i64] {
        // HugeArray doesn't support slices, return empty
        &[]
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> i64 { 0 }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Huge 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Long }
    fn with_capacity(capacity: usize) -> Self {
        Self::new(capacity)
    }
    fn with_defaults(count: usize, default_value: i64) -> Self {
        let mut array = Self::new(count);
        for i in 0..count {
            array.set(i, default_value);
        }
        array
    }
}

impl Collections<f64> for HugeDoubleArray { 
    fn get(&self, index: usize) -> Option<f64> {
        if index < self.size() {
            Some(self.get(index))
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: f64) {
        if index < self.size() {
            self.set(index, value);
        }
    }

    fn len(&self) -> usize {
        self.size()
    }

    fn sum(&self) -> Option<f64> where f64: Sum {
        Some(self.iter().sum())
    }

    fn mean(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            Some(
                self.iter()
                    .map(|x| x as f64)
                    .sum::<f64>() / self.size() as f64
            )
        }
    }

    fn min(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            self.iter().min_by(|a, b| a.partial_cmp(b).unwrap())
        }
    }

    fn max(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            self.iter().max_by(|a, b| a.partial_cmp(b).unwrap())
        }
    }

    fn std_dev(&self) -> Option<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            let variance = self.iter()
                .map(|x| {
                    let diff = x - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64;
            Some(variance.sqrt())
        }
    }

    fn variance(&self) -> Option<f64> {
        if self.size() < 2 {
            None
        } else {
            let mean = self.mean()?;
            Some(self.iter()
                .map(|x| {
                    let diff = x - mean;
                    diff * diff
                })
                .sum::<f64>() / (self.size() - 1) as f64)
        }
    }

    fn median(&self) -> Option<f64> {
        if self.size() == 0 {
            None
        } else {
            let mut values: Vec<f64> = self.iter().collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                Some((values[mid - 1] + values[mid]) / 2.0)
            } else {
                Some(values[mid])
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<f64> {
        if self.size() == 0 || p < 0.0 || p > 100.0 {
            None
        } else {
            let mut values: Vec<f64> = self.iter().collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = (p / 100.0 * (values.len() - 1) as f64).round() as usize;
            Some(values[index])
        }
    }

    fn binary_search(&self, key: &f64) -> Result<usize, usize> {
        let values: Vec<f64> = self.iter().collect();
        values.binary_search_by(|x| x.partial_cmp(key).unwrap())
    }

    fn sort(&mut self) {
        // HugeArray doesn't support in-place sorting
    }

    fn to_vec(self) -> Vec<f64> {
        self.iter().collect()
    }

    fn as_slice(&self) -> &[f64] {
        // HugeArray doesn't support slices, return empty
        &[]
    }

    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> f64 { 0.0 }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Huge 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Double }
    fn with_capacity(capacity: usize) -> Self {
        Self::new(capacity)
    }
    fn with_defaults(count: usize, default_value: f64) -> Self {
        let mut array = Self::new(count);
        for i in 0..count {
            array.set(i, default_value);
        }
        array
    }
}

// Huge-specific utilities
pub mod utils {
    pub use crate::collections::utils::PageUtil;
    pub use crate::collections::utils::ArrayUtil;
}

// Huge-specific macros
pub use crate::huge_collections;