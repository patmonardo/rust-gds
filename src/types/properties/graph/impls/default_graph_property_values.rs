use crate::property_values_impl;
use crate::types::properties::graph::{
    DoubleArrayGraphPropertyValues, DoubleGraphPropertyValues, FloatArrayGraphPropertyValues,
    GraphPropertyValues, LongArrayGraphPropertyValues, LongGraphPropertyValues,
};
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct DefaultLongGraphPropertyValues {
    values: Vec<i64>,
}

impl DefaultLongGraphPropertyValues {
    pub fn new(values: Vec<i64>) -> Self {
        Self { values }
    }

    pub fn singleton(value: i64) -> Self {
        Self::new(vec![value])
    }

    pub fn values(&self) -> &[i64] {
        &self.values
    }
}

property_values_impl!(DefaultLongGraphPropertyValues, Long, graph);

impl GraphPropertyValues for DefaultLongGraphPropertyValues {
    fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        Box::new(self.values.iter().map(|&value| value as f64))
    }

    fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
        Box::new(self.values.iter().copied())
    }

    fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
        Box::new(std::iter::empty())
    }

    fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
        Box::new(std::iter::empty())
    }

    fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
        Box::new(std::iter::empty())
    }

    fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn Any>> + '_> {
        Box::new(
            self.values
                .iter()
                .map(|&value| Box::new(value) as Box<dyn Any>),
        )
    }
}

impl LongGraphPropertyValues for DefaultLongGraphPropertyValues {
    fn long_values_unchecked(&self) -> &[i64] {
        &self.values
    }
}

impl FromIterator<i64> for DefaultLongGraphPropertyValues {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone)]
pub struct DefaultDoubleGraphPropertyValues {
    values: Vec<f64>,
}

impl DefaultDoubleGraphPropertyValues {
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    pub fn singleton(value: f64) -> Self {
        Self::new(vec![value])
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

property_values_impl!(DefaultDoubleGraphPropertyValues, Double, graph);

impl GraphPropertyValues for DefaultDoubleGraphPropertyValues {
    fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        Box::new(self.values.iter().copied())
    }

    fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
        Box::new(self.values.iter().map(|&value| value as i64))
    }

    fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
        Box::new(std::iter::empty())
    }

    fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
        Box::new(std::iter::empty())
    }

    fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
        Box::new(std::iter::empty())
    }

    fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn Any>> + '_> {
        Box::new(
            self.values
                .iter()
                .map(|&value| Box::new(value) as Box<dyn Any>),
        )
    }
}

impl DoubleGraphPropertyValues for DefaultDoubleGraphPropertyValues {
    fn double_values_unchecked(&self) -> &[f64] {
        &self.values
    }
}

impl FromIterator<f64> for DefaultDoubleGraphPropertyValues {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

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

property_values_impl!(
    DefaultDoubleArrayGraphPropertyValues,
    DoubleArray,
    graph_array
);

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

impl DoubleArrayGraphPropertyValues for DefaultDoubleArrayGraphPropertyValues {
    fn double_arrays_unchecked(&self) -> &[Vec<f64>] {
        &self.values
    }
}

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

property_values_impl!(
    DefaultFloatArrayGraphPropertyValues,
    FloatArray,
    graph_array
);

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

impl FloatArrayGraphPropertyValues for DefaultFloatArrayGraphPropertyValues {
    fn float_arrays_unchecked(&self) -> &[Vec<f32>] {
        &self.values
    }
}

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

property_values_impl!(DefaultLongArrayGraphPropertyValues, LongArray, graph_array);

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

impl LongArrayGraphPropertyValues for DefaultLongArrayGraphPropertyValues {
    fn long_arrays_unchecked(&self) -> &[Vec<i64>] {
        &self.values
    }
}

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
    fn test_long_graph_property_values() {
        let values: DefaultLongGraphPropertyValues = [1, 2, 3].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.element_count(), 3);
        let collected: Vec<i64> = values.long_values().collect();
        assert_eq!(collected, vec![1, 2, 3]);
        let as_doubles: Vec<f64> = values.double_values().collect();
        assert_eq!(as_doubles, vec![1.0, 2.0, 3.0]);
    }

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
}
