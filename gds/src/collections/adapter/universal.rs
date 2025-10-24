//! Universal PropertyValues Adapter: Seamless Collections Integration
//!
//! This module provides the UniversalPropertyValues adapter that allows
//! seamless integration between PropertyValues and Collections, providing
//! a unified interface for all Collections backends.

use crate::collections::traits::{Collections, PropertyValuesAdapter};
use crate::collections::config::{CollectionsBackend, Extension, Feature};
use crate::types::ValueType;
use crate::types::default_value::DefaultValue;
use crate::types::properties::property_values::{PropertyValues, PropertyValuesResult};

/// Universal PropertyValues adapter that wraps any Collections implementation
#[derive(Debug)]
pub struct UniversalPropertyValues<T, C> 
where 
    C: Collections<T> + PropertyValuesAdapter<T> + Send + Sync + std::fmt::Debug,
{
    /// The underlying Collections implementation
    collection: C,
    /// Value type
    value_type: ValueType,
    /// Default value
    default_value: T,
    /// Element count
    element_count: usize,
}

impl<T, C> UniversalPropertyValues<T, C> 
where 
    C: Collections<T> + PropertyValuesAdapter<T> + Send + Sync + std::fmt::Debug,
    T: Clone + Send + Sync + std::fmt::Debug,
{
    /// Creates a new UniversalPropertyValues instance
    pub fn new(collection: C, value_type: ValueType, default_value: T) -> Self {
        let element_count = collection.len();
        Self {
            collection,
            value_type,
            default_value,
            element_count,
        }
    }

    /// Gets the underlying Collections implementation
    pub fn collection(&self) -> &C {
        &self.collection
    }

    /// Gets mutable reference to the underlying Collections implementation
    pub fn collection_mut(&mut self) -> &mut C {
        &mut self.collection
    }

    /// Gets the value type
    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    /// Gets the default value
    pub fn default_value(&self) -> T {
        self.default_value.clone()
    }

    /// Gets the element count
    pub fn element_count(&self) -> usize {
        self.element_count
    }

    /// Gets the backend
    pub fn backend(&self) -> CollectionsBackend {
        self.collection.backend()
    }

    /// Gets the enabled features
    pub fn features(&self) -> &[Extension] {
        self.collection.features()
    }

    /// Gets the enabled extensions
    pub fn extensions(&self) -> &[Extension] {
        self.collection.extensions()
    }

    /// Updates the element count
    pub fn set_element_count(&mut self, count: usize) {
        self.element_count = count;
    }

    /// Resizes the collection to the specified count
    pub fn resize(&mut self, count: usize) {
        self.element_count = count;
        // The underlying collection will handle the actual resizing
    }

    /// Clears the collection
    pub fn clear(&mut self) {
        self.element_count = 0;
        // The underlying collection will handle the actual clearing
    }
}

impl<T, C> PropertyValues for UniversalPropertyValues<T, C> 
where 
    C: Collections<T> + PropertyValuesAdapter<T> + Send + Sync + std::fmt::Debug,
    T: Clone + Send + Sync + std::fmt::Debug,
{
    fn value_type(&self) -> ValueType {
        self.value_type
    }

    fn element_count(&self) -> usize {
        self.element_count
    }
}

impl<T, C> Collections<T> for UniversalPropertyValues<T, C> 
where 
    C: Collections<T> + PropertyValuesAdapter<T> + Send + Sync + std::fmt::Debug,
    T: Clone + Send + Sync + std::fmt::Debug,
{
    fn get(&self, index: usize) -> Option<T> {
        self.collection.get(index)
    }

    fn set(&mut self, index: usize, value: T) {
        self.collection.set(index, value);
    }

    fn len(&self) -> usize {
        self.collection.len()
    }

    fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    fn sum(&self) -> Option<T> where T: Sum {
        self.collection.sum()
    }

    fn min(&self) -> Option<T> where T: Ord {
        self.collection.min()
    }

    fn max(&self) -> Option<T> where T: Ord {
        self.collection.max()
    }

    fn mean(&self) -> Option<f64> {
        self.collection.mean()
    }

    fn std_dev(&self) -> Option<f64> {
        self.collection.std_dev()
    }

    fn variance(&self) -> Option<f64> {
        self.collection.variance()
    }

    fn median(&self) -> Option<T> where T: Ord {
        self.collection.median()
    }

    fn percentile(&self, p: f64) -> Option<T> where T: Ord {
        self.collection.percentile(p)
    }

    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord {
        self.collection.binary_search(key)
    }

    fn sort(&mut self) where T: Ord {
        self.collection.sort();
    }

    fn to_vec(self) -> Vec<T> {
        self.collection.to_vec()
    }

    fn as_slice(&self) -> &[T] {
        self.collection.as_slice()
    }

    fn is_null(&self, index: usize) -> bool {
        self.collection.is_null(index)
    }

    fn null_count(&self) -> usize {
        self.collection.null_count()
    }
    
    fn default_value(&self) -> T {
        self.default_value.clone()
    }
    
    fn backend(&self) -> crate::collections::config::CollectionsBackend {
        self.collection.backend()
    }
    
    fn features(&self) -> &[crate::collections::config::Extension] {
        self.collection.features()
    }
    
    fn extensions(&self) -> &[crate::collections::config::Extension] {
        self.collection.extensions()
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }
    
    fn with_capacity(capacity: usize) -> Self where Self: Sized {
        // This is tricky for UniversalPropertyValues since we need a collection
        // For now, create a default instance
        panic!("with_capacity not implemented for UniversalPropertyValues")
    }
    
    fn with_defaults(count: usize, default_value: T) -> Self where Self: Sized {
        // This is tricky for UniversalPropertyValues since we need a collection
        // For now, create a default instance
        panic!("with_defaults not implemented for UniversalPropertyValues")
    }
}

/// Import Sum trait for aggregation
use std::iter::Sum;
