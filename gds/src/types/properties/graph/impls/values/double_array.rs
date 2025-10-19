use crate::property_values_impl;
use crate::types::properties::graph::{DoubleArrayGraphPropertyValues, GraphPropertyValues};
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::any::Any;

/// Default implementation for double array graph property values.
///
/// Storage: Vec<Vec<f64>> - suitable for pure in-memory graph-level array properties.
/// Future: Can be replaced with Arrow2 ListArray<Float64Array> for columnar storage.
///
/// Note: Unlike node properties, graph array properties don't use Option wrapping
/// since graph properties are expected to always have values.
#[derive(Debug, Clone)]
pub struct DefaultDoubleArrayGraphPropertyValues {
    values: Vec<Vec<f64>>,
    dimension: Option<usize>,
}

impl DefaultDoubleArrayGraphPropertyValues {
    pub fn new(values: Vec<Vec<f64>>) -> Self {
        let dimension = values.first().map(|vec| vec.len());
        Self { values, dimension }
    }

    pub fn singleton(value: Vec<f64>) -> Self {
        Self::new(vec![value])
    }

    pub fn values(&self) -> &[Vec<f64>] {
        &self.values
    }
}

// Generate PropertyValues trait implementation for arrays
property_values_impl!(
    DefaultDoubleArrayGraphPropertyValues,
    DoubleArray,
    graph_array
);

// Manual GraphPropertyValues implementation (iterator-based)
impl GraphPropertyValues for DefaultDoubleArrayGraphPropertyValues {
    fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        Box::new(std::iter::empty())
    }

    fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
        Box::new(std::iter::empty())
    }

    fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
        Box::new(self.values.iter().cloned())
    }

    fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
        Box::new(
            self.values
                .iter()
                .map(|vec| vec.iter().map(|&v| v as f32).collect()),
        )
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

// Specialized unchecked accessor for DoubleArray values
impl DoubleArrayGraphPropertyValues for DefaultDoubleArrayGraphPropertyValues {
    fn double_arrays_unchecked(&self) -> &[Vec<f64>] {
        &self.values
    }
}

// Convenient FromIterator for building from nested iterators
impl<T> FromIterator<T> for DefaultDoubleArrayGraphPropertyValues
where
    T: IntoIterator<Item = f64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let collected: Vec<Vec<f64>> = iter
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
    fn test_double_array_graph_property_values() {
        let values: DefaultDoubleArrayGraphPropertyValues =
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::DoubleArray);
        assert_eq!(values.element_count(), 2);
        assert_eq!(values.dimension(), Some(3));
        let arrays: Vec<Vec<f64>> = values.double_array_values().collect();
        assert_eq!(arrays.len(), 2);
        assert_eq!(arrays[0], vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_double_array_to_float_array_conversion() {
        let values = DefaultDoubleArrayGraphPropertyValues::new(vec![vec![1.0, 2.0]]);
        let float_arrays: Vec<Vec<f32>> = values.float_array_values().collect();
        assert_eq!(float_arrays, vec![vec![1.0f32, 2.0f32]]);
    }

    #[test]
    fn test_singleton() {
        let values = DefaultDoubleArrayGraphPropertyValues::singleton(vec![1.0, 2.0, 3.0]);
        assert_eq!(values.element_count(), 1);
        assert_eq!(values.dimension(), Some(3));
    }

    #[test]
    fn test_empty_scalar_iterators() {
        let values = DefaultDoubleArrayGraphPropertyValues::new(vec![vec![1.0, 2.0]]);
        assert_eq!(values.double_values().count(), 0);
        assert_eq!(values.long_values().count(), 0);
    }
}
