use crate::property_values_impl;
use crate::types::properties::graph::{GraphPropertyValues, LongArrayGraphPropertyValues};
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::any::Any;

/// Default implementation for long array graph property values.
///
/// Storage: Vec<Vec<i64>> - suitable for integer sequence graph properties.
/// Future: Can be replaced with Arrow2 ListArray<Int64Array> for columnar storage.
#[derive(Debug, Clone)]
pub struct DefaultLongArrayGraphPropertyValues {
    values: Vec<Vec<i64>>,
    dimension: Option<usize>,
}

impl DefaultLongArrayGraphPropertyValues {
    pub fn new(values: Vec<Vec<i64>>) -> Self {
        let dimension = values.first().map(|vec| vec.len());
        Self { values, dimension }
    }

    pub fn singleton(value: Vec<i64>) -> Self {
        Self::new(vec![value])
    }

    pub fn values(&self) -> &[Vec<i64>] {
        &self.values
    }
}

// Generate PropertyValues trait implementation for arrays
// Generate PropertyValues trait implementation for graph-level long arrays
property_values_impl!(DefaultLongArrayGraphPropertyValues, LongArray, graph_array);

// Manual GraphPropertyValues implementation (iterator-based)
impl GraphPropertyValues for DefaultLongArrayGraphPropertyValues {
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
        Box::new(
            self.values
                .iter()
                .map(|vec| vec.iter().map(|&v| v as f32).collect()),
        )
    }

    fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
        Box::new(self.values.iter().cloned())
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

// Specialized unchecked accessor for LongArray values
impl LongArrayGraphPropertyValues for DefaultLongArrayGraphPropertyValues {
    fn long_arrays_unchecked(&self) -> &[Vec<i64>] {
        &self.values
    }
}

// Convenient FromIterator for building from nested iterators
impl<T> FromIterator<T> for DefaultLongArrayGraphPropertyValues
where
    T: IntoIterator<Item = i64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let collected: Vec<Vec<i64>> = iter
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
    fn test_long_array_graph_property_values() {
        let values: DefaultLongArrayGraphPropertyValues =
            [[1, 2, 3], [4, 5, 6]].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::LongArray);
        assert_eq!(values.element_count(), 2);
        assert_eq!(values.dimension(), Some(3));
    }

    #[test]
    fn test_long_array_type_conversions() {
        let values = DefaultLongArrayGraphPropertyValues::new(vec![vec![1, 2], vec![3, 4]]);

        let double_arrays: Vec<Vec<f64>> = values.double_array_values().collect();
        assert_eq!(double_arrays, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let float_arrays: Vec<Vec<f32>> = values.float_array_values().collect();
        assert_eq!(float_arrays, vec![vec![1.0f32, 2.0], vec![3.0, 4.0]]);
    }

    #[test]
    fn test_singleton() {
        let values = DefaultLongArrayGraphPropertyValues::singleton(vec![10, 20, 30]);
        assert_eq!(values.element_count(), 1);
        assert_eq!(values.dimension(), Some(3));
    }
}
