use crate::property_values_impl;
use crate::types::properties::graph::graph_property_values::{
    GraphPropertyValues, LongGraphPropertyValues,
};
use crate::types::properties::property_values::PropertyValues;
use crate::types::value_type::ValueType;
use crate::types::property_value::PropertyValue;
use std::any::Any;

/// Default implementation for long graph property values.
///
/// Storage: Vec<i64> - suitable for pure in-memory graph-level properties.
/// Future: Can be replaced with Arrow2 Int64Array for columnar storage.
///
/// Graph properties differ from node properties in that they use iterator-based
/// access rather than indexed access, making them ideal for streaming operations.
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

// Generate PropertyValues trait implementation
property_values_impl!(
    DefaultLongGraphPropertyValues,
    Long,
    i64,
    PropertyValue::Long,
    graph
);

// Manual GraphPropertyValues implementation (iterator-based)
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

// Specialized unchecked accessor for Long values
impl LongGraphPropertyValues for DefaultLongGraphPropertyValues {
    fn long_values_unchecked(&self) -> &[i64] {
        &self.values
    }
}

// Convenient FromIterator for building from iterators
impl FromIterator<i64> for DefaultLongGraphPropertyValues {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
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
    }

    #[test]
    fn test_long_to_double_conversion() {
        let values = DefaultLongGraphPropertyValues::new(vec![100, 200, 300]);
        let as_doubles: Vec<f64> = values.double_values().collect();
        assert_eq!(as_doubles, vec![100.0, 200.0, 300.0]);
    }

    #[test]
    fn test_singleton() {
        let values = DefaultLongGraphPropertyValues::singleton(42);
        assert_eq!(values.element_count(), 1);
        assert_eq!(values.long_values_unchecked(), &[42]);
    }

    #[test]
    fn test_empty_array_iterators() {
        let values = DefaultLongGraphPropertyValues::new(vec![1, 2, 3]);
        assert_eq!(values.double_array_values().count(), 0);
        assert_eq!(values.float_array_values().count(), 0);
        assert_eq!(values.long_array_values().count(), 0);
    }
}
