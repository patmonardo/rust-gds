//! Vec Collections Macro: Enhanced Standard Library Vectors
//!
//! This macro generates Vec-based Collections with enhanced features,
//! including aggregation methods, nullability support, and extensions.

use crate::collections::traits::{
    Collections, PropertyValuesAdapter, CollectionsFactory,
    AggregationSupport, NullabilitySupport, CompressionSupport,
};
use crate::collections::config::{CollectionsBackend, Extension, Feature};
use crate::types::ValueType;
use crate::types::default_value::DefaultValue;

/// Vec Collections macro that generates Vec-based Collections implementations
#[macro_export]
macro_rules! vec_collections {
    (
        $type_name:ident,           // e.g., VecInt
        $element_type:ty,           // e.g., i32
        $value_type:expr,           // e.g., ValueType::Int
        $default_value:expr,        // e.g., 0i32
        $features:expr,             // e.g., [Feature::Aggregation]
        $extensions:expr,           // e.g., [Extension::Ndarray]
        $doc_desc:expr              // Documentation
    ) => {
        collections! {
            $type_name,
            $element_type,
            $value_type,
            $default_value,
            CollectionsBackend::Vec,
            $features,
            $extensions,
            $doc_desc
        }

        // Vec-specific implementation
        impl $type_name {
            pub fn new() -> Self {
                Self {
                    data: Vec::new(),
                    metadata: CollectionsMetadata {
                        backend: CollectionsBackend::Vec,
                        features: $features,
                        extensions: $extensions,
                        performance_profile: PerformanceProfile::default(),
                    },
                    extensions: ExtensionsRegistry::new($extensions),
                }
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    data: Vec::with_capacity(capacity),
                    metadata: CollectionsMetadata {
                        backend: CollectionsBackend::Vec,
                        features: $features,
                        extensions: $extensions,
                        performance_profile: PerformanceProfile::default(),
                    },
                    extensions: ExtensionsRegistry::new($extensions),
                }
            }

            pub fn push(&mut self, value: $element_type) {
                self.data.push(value);
            }

            pub fn pop(&mut self) -> Option<$element_type> {
                self.data.pop()
            }

            pub fn reserve(&mut self, additional: usize) {
                self.data.reserve(additional);
            }

            pub fn shrink_to_fit(&mut self) {
                self.data.shrink_to_fit();
            }

            pub fn clear(&mut self) {
                self.data.clear();
            }

            pub fn extend_from_slice(&mut self, slice: &[$element_type]) {
                self.data.extend_from_slice(slice);
            }

            pub fn insert(&mut self, index: usize, value: $element_type) {
                self.data.insert(index, value);
            }

            pub fn remove(&mut self, index: usize) -> $element_type {
                self.data.remove(index)
            }

            pub fn swap_remove(&mut self, index: usize) -> $element_type {
                self.data.swap_remove(index)
            }

            pub fn retain<F>(&mut self, f: F) where F: FnMut(&$element_type) -> bool {
                self.data.retain(f);
            }

            pub fn dedup(&mut self) where $element_type: PartialEq {
                self.data.dedup();
            }

            pub fn dedup_by<F>(&mut self, same_bucket: F) where F: FnMut(&mut $element_type, &mut $element_type) -> bool {
                self.data.dedup_by(same_bucket);
            }

            pub fn dedup_by_key<F, K>(&mut self, key: F) where F: FnMut(&mut $element_type) -> K, K: PartialEq {
                self.data.dedup_by_key(key);
            }
        }

        impl Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> {
                self.data.get(index).cloned()
            }

            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.data.len() {
                    self.data[index] = value;
                } else {
                    self.data.resize(index + 1, $default_value);
                    self.data[index] = value;
                }
            }

            fn len(&self) -> usize {
                self.data.len()
            }

            fn as_slice(&self) -> &[$element_type] {
                &self.data
            }

            // Vec-specific optimizations
            $(
                if $features.contains(&Feature::Aggregation) {
                    fn sum(&self) -> Option<$element_type> where $element_type: Sum {
                        Some(self.data.iter().cloned().sum())
                    }

                    fn mean(&self) -> Option<f64> where $element_type: Into<f64> {
                        if self.data.is_empty() {
                            None
                        } else {
                            Some(
                                self.data.iter()
                                    .map(|&x| x.into())
                                    .sum::<f64>() / self.data.len() as f64
                            )
                        }
                    }

                    fn std_dev(&self) -> Option<f64> where $element_type: Into<f64> {
                        if self.data.len() < 2 {
                            None
                        } else {
                            let mean = self.mean()?;
                            let variance = self.data.iter()
                                .map(|&x| {
                                    let diff = x.into() - mean;
                                    diff * diff
                                })
                                .sum::<f64>() / (self.data.len() - 1) as f64;
                            Some(variance.sqrt())
                        }
                    }

                    fn variance(&self) -> Option<f64> where $element_type: Into<f64> {
                        if self.data.len() < 2 {
                            None
                        } else {
                            let mean = self.mean()?;
                            Some(
                                self.data.iter()
                                    .map(|&x| {
                                        let diff = x.into() - mean;
                                        diff * diff
                                    })
                                    .sum::<f64>() / (self.data.len() - 1) as f64
                            )
                        }
                    }

                    fn median(&self) -> Option<$element_type> where $element_type: Ord {
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

                    fn percentile(&self, p: f64) -> Option<$element_type> where $element_type: Ord {
                        if self.data.is_empty() || p < 0.0 || p > 100.0 {
                            None
                        } else {
                            let mut sorted = self.data.clone();
                            sorted.sort();
                            let index = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
                            Some(sorted[index])
                        }
                    }
                }
            )*

            $(
                if $features.contains(&Feature::Nullability) {
                    fn is_null(&self, _index: usize) -> bool { false }
                    fn null_count(&self) -> usize { 0 }
                }
            )*

            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> where $element_type: Ord {
                self.data.binary_search(key)
            }

            fn sort(&mut self) where $element_type: Ord {
                self.data.sort();
            }

            fn to_vec(self) -> Vec<$element_type> {
                self.data
            }
        }

        impl CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self {
                Self::new()
            }

            fn with_capacity(capacity: usize) -> Self {
                Self::with_capacity(capacity)
            }

            fn from_vec(values: Vec<$element_type>) -> Self {
                Self {
                    data: values,
                    metadata: CollectionsMetadata {
                        backend: CollectionsBackend::Vec,
                        features: $features,
                        extensions: $extensions,
                        performance_profile: PerformanceProfile::default(),
                    },
                    extensions: ExtensionsRegistry::new($extensions),
                }
            }

            fn from_slice(slice: &[$element_type]) -> Self {
                Self {
                    data: slice.to_vec(),
                    metadata: CollectionsMetadata {
                        backend: CollectionsBackend::Vec,
                        features: $features,
                        extensions: $extensions,
                        performance_profile: PerformanceProfile::default(),
                    },
                    extensions: ExtensionsRegistry::new($extensions),
                }
            }

            fn with_defaults(count: usize, default_value: $element_type) -> Self {
                Self {
                    data: vec![default_value; count],
                    metadata: CollectionsMetadata {
                        backend: CollectionsBackend::Vec,
                        features: $features,
                        extensions: $extensions,
                        performance_profile: PerformanceProfile::default(),
                    },
                    extensions: ExtensionsRegistry::new($extensions),
                }
            }
        }
    };
}
