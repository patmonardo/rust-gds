//! Adapter Macros: Code Generation for Adapters
//!
//! This module provides macros for generating adapter code,
//! including PropertyValues adapters and Collections adapters.

/// Macro for generating PropertyValues adapter implementations
#[macro_export]
macro_rules! collections_property_values_adapter {
    (
        $adapter_name:ident,        // e.g., PropertyValuesAdapter
        $element_type:ty,           // e.g., i32
        $value_type:expr,           // e.g., ValueType::Int
        $default_value:expr,        // e.g., 0i32
        $backend:expr,              // e.g., CollectionsBackend::Vec
        $features:expr,             // e.g., [Feature::Aggregation]
        $extensions:expr,           // e.g., [Extension::Ndarray]
        $doc_desc:expr              // Documentation
    ) => {
        use crate::collections::traits::{Collections, PropertyValuesAdapter};
        use crate::collections::config::{CollectionsBackend, Extension, Feature};
        use crate::types::ValueType;
        use crate::types::default_value::DefaultValue;
        use crate::types::properties::property_values::{PropertyValues, PropertyValuesResult};

        #[doc = $doc_desc]
        pub struct $adapter_name<T, C> 
        where 
            C: Collections<T> + PropertyValuesAdapter<T>,
        {
            collection: C,
            value_type: ValueType,
            default_value: T,
            element_count: usize,
        }

        impl<T, C> $adapter_name<T, C> 
        where 
            C: Collections<T> + PropertyValuesAdapter<T>,
        {
            pub fn new(collection: C, value_type: ValueType, default_value: T) -> Self {
                let element_count = collection.len();
                Self {
                    collection,
                    value_type,
                    default_value,
                    element_count,
                }
            }

            pub fn collection(&self) -> &C {
                &self.collection
            }

            pub fn collection_mut(&mut self) -> &mut C {
                &mut self.collection
            }

            pub fn value_type(&self) -> ValueType {
                self.value_type
            }

            pub fn default_value(&self) -> T {
                self.default_value.clone()
            }

            pub fn element_count(&self) -> usize {
                self.element_count
            }

            pub fn backend(&self) -> CollectionsBackend {
                self.collection.backend()
            }

            pub fn features(&self) -> &[Feature] {
                self.collection.features()
            }

            pub fn extensions(&self) -> &[Extension] {
                self.collection.extensions()
            }

            pub fn set_element_count(&mut self, count: usize) {
                self.element_count = count;
            }

            pub fn resize(&mut self, count: usize) {
                self.element_count = count;
            }

            pub fn clear(&mut self) {
                self.element_count = 0;
            }
        }

        impl<T, C> PropertyValues<T> for $adapter_name<T, C> 
        where 
            C: Collections<T> + PropertyValuesAdapter<T>,
            T: Clone,
        {
            fn get(&self, index: usize) -> PropertyValuesResult<T> {
                if index >= self.element_count {
                    return Err(crate::types::properties::property_values::PropertyValuesError::IndexOutOfBounds);
                }
                
                match self.collection.get(index) {
                    Some(value) => Ok(value),
                    None => Ok(self.default_value.clone()),
                }
            }

            fn set(&mut self, index: usize, value: T) -> PropertyValuesResult<()> {
                if index >= self.element_count {
                    return Err(crate::types::properties::property_values::PropertyValuesError::IndexOutOfBounds);
                }
                
                self.collection.set(index, value);
                Ok(())
            }

            fn len(&self) -> usize {
                self.element_count
            }

            fn is_empty(&self) -> bool {
                self.element_count == 0
            }

            fn default_value(&self) -> T {
                self.default_value.clone()
            }

            fn value_type(&self) -> ValueType {
                self.value_type
            }

            fn to_vec(self) -> Vec<T> {
                self.collection.to_vec()
            }

            fn as_slice(&self) -> &[T] {
                self.collection.as_slice()
            }
        }

        impl<T, C> Collections<T> for $adapter_name<T, C> 
        where 
            C: Collections<T> + PropertyValuesAdapter<T>,
            T: Clone,
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

            fn mean(&self) -> Option<f64> where T: Into<f64> {
                self.collection.mean()
            }

            fn std_dev(&self) -> Option<f64> where T: Into<f64> {
                self.collection.std_dev()
            }

            fn variance(&self) -> Option<f64> where T: Into<f64> {
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
        }

        impl<T, C> PropertyValuesAdapter<T> for $adapter_name<T, C> 
        where 
            C: Collections<T> + PropertyValuesAdapter<T>,
            T: Clone,
        {
            fn value_type(&self) -> ValueType {
                self.value_type
            }

            fn default_value(&self) -> T {
                self.default_value.clone()
            }

            fn backend(&self) -> CollectionsBackend {
                self.collection.backend()
            }

            fn features(&self) -> &[Feature] {
                self.collection.features()
            }

            fn extensions(&self) -> &[Extension] {
                self.collection.extensions()
            }
        }
    };
}
