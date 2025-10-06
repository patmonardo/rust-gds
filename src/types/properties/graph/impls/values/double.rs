use crate::property_values_impl;
use crate::types::properties::graph::graph_property_values::{
    DoubleGraphPropertyValues, GraphPropertyValues,
};
use crate::types::properties::property_values::PropertyValues;
use crate::types::property::ValueType;
use crate::types::property_value::PropertyValue;
use std::any::Any;

/// Default implementation for double graph property values.
/// 
/// Storage: Vec<f64> - suitable for pure in-memory graph-level properties.
/// Future: Can be replaced with Arrow2 Float64Array for columnar storage.
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

// Generate PropertyValues trait implementation
property_values_impl!(
    DefaultDoubleGraphPropertyValues,
    Double,
    f64,
    PropertyValue::Double,
    graph
);

// Manual GraphPropertyValues implementation (iterator-based)
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

// Specialized unchecked accessor for Double values
impl DoubleGraphPropertyValues for DefaultDoubleGraphPropertyValues {
    fn double_values_unchecked(&self) -> &[f64] {
        &self.values
    }
}

// Convenient FromIterator for building from iterators
impl FromIterator<f64> for DefaultDoubleGraphPropertyValues {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_graph_property_values() {
        let values: DefaultDoubleGraphPropertyValues = [1.5, 2.5, 3.5].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.element_count(), 3);
        let collected: Vec<f64> = values.double_values().collect();
        assert_eq!(collected, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_double_to_long_conversion() {
        let values = DefaultDoubleGraphPropertyValues::new(vec![1.9, 2.1, 3.7]);
        let as_longs: Vec<i64> = values.long_values().collect();
        assert_eq!(as_longs, vec![1, 2, 3]);
    }

    #[test]
    fn test_singleton() {
        let values = DefaultDoubleGraphPropertyValues::singleton(3.14);
        assert_eq!(values.element_count(), 1);
        assert_eq!(values.double_values_unchecked(), &[3.14]);
    }
}
