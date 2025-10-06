use crate::property_values_impl;
use crate::types::properties::graph::graph_property_values::{
    FloatArrayGraphPropertyValues, GraphPropertyValues,
};
use crate::types::properties::property_values::PropertyValues;
use crate::types::value_type::ValueType;
use std::any::Any;

/// Default implementation for float array graph property values.
///
/// Storage: Vec<Vec<f32>> - memory-efficient for graph-level array properties.
/// Future: Can be replaced with Arrow2 ListArray<Float32Array> for columnar storage.
#[derive(Debug, Clone)]
pub struct DefaultFloatArrayGraphPropertyValues {
    values: Vec<Vec<f32>>,
    dimension: Option<usize>,
}

impl DefaultFloatArrayGraphPropertyValues {
    pub fn new(values: Vec<Vec<f32>>) -> Self {
        let dimension = values.first().map(|vec| vec.len());
        Self { values, dimension }
    }

    pub fn singleton(value: Vec<f32>) -> Self {
        Self::new(vec![value])
    }

    pub fn values(&self) -> &[Vec<f32>] {
        &self.values
    }
}

// Generate PropertyValues trait implementation for arrays
property_values_impl!(DefaultFloatArrayGraphPropertyValues, FloatArray, graph_array);

// Manual GraphPropertyValues implementation (iterator-based)
impl GraphPropertyValues for DefaultFloatArrayGraphPropertyValues {
    fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        Box::new(std::iter::empty())
    }

    fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
        Box::new(std::iter::empty())
    }

    fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
        Box::new(
            self.values
                .iter()
                .map(|vec| vec.iter().map(|&v| v as f64).collect()),
        )
    }

    fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
        Box::new(self.values.iter().cloned())
    }

    fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
        Box::new(std::iter::empty())
    }

    fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn Any>> + '_> {
        Box::new(
            self.values
                .iter()
                .map(|vec| Box::new(vec.clone()) as Box<dyn Any>),
        )
    }

    fn dimension(&self) -> Option<usize> {
        self.dimension
    }
}

// Specialized unchecked accessor for FloatArray values
impl FloatArrayGraphPropertyValues for DefaultFloatArrayGraphPropertyValues {
    fn float_arrays_unchecked(&self) -> &[Vec<f32>] {
        &self.values
    }
}

// Convenient FromIterator for building from nested iterators
impl<T> FromIterator<T> for DefaultFloatArrayGraphPropertyValues
where
    T: IntoIterator<Item = f32>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let collected: Vec<Vec<f32>> = iter
            .into_iter()
            .map(|inner| inner.into_iter().collect())
            .collect();
        Self::new(collected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_array_graph_property_values() {
        let values: DefaultFloatArrayGraphPropertyValues =
            [[1.0f32, 2.0, 3.0], [4.0, 5.0, 6.0]].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::FloatArray);
        assert_eq!(values.element_count(), 2);
        assert_eq!(values.dimension(), Some(3));
    }

    #[test]
    fn test_float_array_to_double_array_conversion() {
        let values = DefaultFloatArrayGraphPropertyValues::new(vec![vec![1.5f32, 2.5]]);
        let double_arrays: Vec<Vec<f64>> = values.double_array_values().collect();
        assert_eq!(double_arrays, vec![vec![1.5f64, 2.5f64]]);
    }

    #[test]
    fn test_singleton() {
        let values = DefaultFloatArrayGraphPropertyValues::singleton(vec![1.0, 2.0]);
        assert_eq!(values.element_count(), 1);
        assert_eq!(values.float_arrays_unchecked(), &[vec![1.0f32, 2.0f32]]);
    }
}
