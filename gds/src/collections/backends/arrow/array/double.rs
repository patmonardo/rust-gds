use std::cmp::Ordering;
use std::sync::Arc;

use arrow2::array::{Array, MutablePrimitiveArray, PrimitiveArray};

use crate::collections::traits::{Collections, NullabilitySupport, PropertyValuesAdapter};
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;

use super::ArrowArrayBehavior;

const DEFAULT_DOUBLE_VALUE: f64 = 0.0;
const EMPTY_EXTENSIONS: [Extension; 0] = [];

#[derive(Clone, Debug)]
pub struct ArrowDoubleArray {
    array: Arc<PrimitiveArray<f64>>,
    default_value: f64,
}

impl ArrowDoubleArray {
    pub fn new() -> Self {
        Self::with_defaults(0, DEFAULT_DOUBLE_VALUE)
    }

    pub fn from_arc(array: Arc<PrimitiveArray<f64>>) -> Self {
        Self {
            array,
            default_value: DEFAULT_DOUBLE_VALUE,
        }
    }

    pub fn from_vec(values: Vec<f64>) -> Self {
        let array = PrimitiveArray::from_vec(values);
        Self {
            array: Arc::new(array),
            default_value: DEFAULT_DOUBLE_VALUE,
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn values(&self) -> &[f64] {
        self.array.values()
    }

    pub fn arrow(&self) -> &Arc<PrimitiveArray<f64>> {
        &self.array
    }

    pub fn into_arrow(self) -> Arc<PrimitiveArray<f64>> {
        self.array
    }

    fn rebuild_from_options(&mut self, values: Vec<Option<f64>>) {
        let mutable: MutablePrimitiveArray<f64> = values.into_iter().collect();
        let primitive: PrimitiveArray<f64> = mutable.into();
        self.array = Arc::new(primitive);
    }

    fn as_option_vec(&self) -> Vec<Option<f64>> {
        self.array.iter().map(|value| value.copied()).collect()
    }

    fn update_entry(&mut self, index: usize, value: Option<f64>) {
        let mut values = self.as_option_vec();
        if index >= values.len() {
            values.resize(index + 1, Some(self.default_value));
        }

        values[index] = value.or(Some(self.default_value));
        if value.is_none() {
            values[index] = None;
        }

        self.rebuild_from_options(values);
    }

    fn iter_non_null(&self) -> impl Iterator<Item = f64> + '_ {
        self.array.iter().flatten().copied()
    }

    fn non_null_len(&self) -> usize {
        self.array.len() - self.array.null_count()
    }

    fn compute_sum(&self) -> Option<f64> {
        let mut iter = self.iter_non_null();
        let first = iter.next()?;
        Some(iter.fold(first, |acc, value| acc + value))
    }

    fn compute_mean(&self) -> Option<f64> {
        let mut iter = self.iter_non_null();
        let first = iter.next()?;
        let (sum, count) = iter.fold((first, 1usize), |(acc, n), value| (acc + value, n + 1));
        Some(sum / count as f64)
    }

    fn compute_variance(&self) -> Option<f64> {
        let count = self.non_null_len();
        if count < 2 {
            return None;
        }
        let mean = self.compute_mean()?;
        let sum_sq: f64 = self
            .iter_non_null()
            .map(|value| {
                let diff = value - mean;
                diff * diff
            })
            .sum();
        Some(sum_sq / (count - 1) as f64)
    }

    fn dense_values(&self) -> Vec<f64> {
        self.array
            .iter()
            .map(|value| value.copied().unwrap_or(self.default_value))
            .collect()
    }

    fn sorted_non_null(&self) -> Vec<f64> {
        let mut values: Vec<f64> = self.iter_non_null().collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        values
    }

    fn partial_min(&self) -> Option<f64> {
        self.iter_non_null().fold(None, |acc, value| match acc {
            Some(current) => match value.partial_cmp(&current) {
                Some(Ordering::Less) => Some(value),
                Some(Ordering::Greater) | Some(Ordering::Equal) | None => Some(current),
            },
            None => Some(value),
        })
    }

    fn partial_max(&self) -> Option<f64> {
        self.iter_non_null().fold(None, |acc, value| match acc {
            Some(current) => match value.partial_cmp(&current) {
                Some(Ordering::Greater) => Some(value),
                Some(Ordering::Less) | Some(Ordering::Equal) | None => Some(current),
            },
            None => Some(value),
        })
    }

    pub fn percentile_value(&self, p: f64) -> Option<f64> {
        if !(0.0..=100.0).contains(&p) {
            return None;
        }
        let values = self.sorted_non_null();
        if values.is_empty() {
            return None;
        }
        if values.len() == 1 {
            return values.first().copied();
        }
        let rank = (p / 100.0) * (values.len() - 1) as f64;
        let lower_index = rank.floor() as usize;
        let upper_index = rank.ceil() as usize;
        let lower = values.get(lower_index).copied()?;
        let upper = values.get(upper_index).copied()?;
        if lower_index == upper_index {
            Some(lower)
        } else {
            let weight = rank - lower_index as f64;
            Some(lower + (upper - lower) * weight)
        }
    }
}

impl ArrowArrayBehavior for ArrowDoubleArray {
    fn len(&self) -> usize {
        self.array.len()
    }

    fn null_count(&self) -> usize {
        self.array.null_count()
    }

    fn is_null(&self, index: usize) -> bool {
        self.array.is_null(index)
    }
}

impl Collections<f64> for ArrowDoubleArray {
    fn get(&self, index: usize) -> Option<f64> {
        if index >= ArrowArrayBehavior::len(self) || ArrowArrayBehavior::is_null(self, index) {
            None
        } else {
            Some(self.array.value(index))
        }
    }

    fn set(&mut self, index: usize, value: f64) {
        self.update_entry(index, Some(value));
    }

    fn len(&self) -> usize {
        ArrowArrayBehavior::len(self)
    }

    fn sum(&self) -> Option<f64> {
        self.compute_sum()
    }

    fn min(&self) -> Option<f64> {
        self.partial_min()
    }

    fn max(&self) -> Option<f64> {
        self.partial_max()
    }

    fn mean(&self) -> Option<f64> {
        self.compute_mean()
    }

    fn std_dev(&self) -> Option<f64> {
        self.compute_variance().map(|var| var.sqrt())
    }

    fn variance(&self) -> Option<f64> {
        self.compute_variance()
    }

    fn median(&self) -> Option<f64> {
        self.percentile_value(50.0)
    }

    fn percentile(&self, p: f64) -> Option<f64> {
        self.percentile_value(p)
    }

    fn binary_search(&self, key: &f64) -> Result<usize, usize> {
        let values = self.sorted_non_null();
        values.binary_search_by(|value| value.partial_cmp(key).unwrap_or(Ordering::Equal))
    }

    fn sort(&mut self) {
        let mut values = self.dense_values();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        self.rebuild_from_options(values.into_iter().map(Some).collect());
    }

    fn to_vec(self) -> Vec<f64> {
        self.dense_values()
    }

    fn as_slice(&self) -> &[f64] {
        self.array.values()
    }

    fn is_null(&self, index: usize) -> bool {
        ArrowArrayBehavior::is_null(self, index)
    }

    fn null_count(&self) -> usize {
        ArrowArrayBehavior::null_count(self)
    }

    fn default_value(&self) -> f64 {
        self.default_value
    }

    fn backend(&self) -> CollectionsBackend {
        CollectionsBackend::Arrow
    }

    fn features(&self) -> &[Extension] {
        &EMPTY_EXTENSIONS
    }

    fn extensions(&self) -> &[Extension] {
        &EMPTY_EXTENSIONS
    }

    fn value_type(&self) -> ValueType {
        ValueType::Double
    }

    fn with_capacity(capacity: usize) -> Self {
        let mutable = MutablePrimitiveArray::<f64>::with_capacity(capacity);
        let primitive: PrimitiveArray<f64> = mutable.into();
        Self {
            array: Arc::new(primitive),
            default_value: DEFAULT_DOUBLE_VALUE,
        }
    }

    fn with_defaults(count: usize, default_value: f64) -> Self {
        let array = PrimitiveArray::from_vec(vec![default_value; count]);
        Self {
            array: Arc::new(array),
            default_value,
        }
    }
}

impl NullabilitySupport<f64> for ArrowDoubleArray {
    fn is_null(&self, index: usize) -> bool {
        ArrowArrayBehavior::is_null(self, index)
    }

    fn null_count(&self) -> usize {
        ArrowArrayBehavior::null_count(self)
    }

    fn set_null(&mut self, index: usize) {
        self.update_entry(index, None);
    }

    fn has_nulls(&self) -> bool {
        ArrowArrayBehavior::null_count(self) > 0
    }
}

impl PropertyValuesAdapter<f64> for ArrowDoubleArray {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::traits::{Collections, NullabilitySupport};

    #[test]
    fn basic_operations() {
        let mut array = ArrowDoubleArray::from_vec(vec![1.0, 2.0, 3.0]);
        assert_eq!(Collections::len(&array), 3);
        assert_eq!(Collections::get(&array, 1), Some(2.0));

        Collections::set(&mut array, 1, 10.5);
        assert_eq!(Collections::get(&array, 1), Some(10.5));
        assert_eq!(Collections::sum(&array), Some(1.0 + 10.5 + 3.0));
        assert_eq!(Collections::mean(&array), Some((1.0 + 10.5 + 3.0) / 3.0));
        assert_eq!(array.partial_min(), Some(1.0));
        assert_eq!(array.partial_max(), Some(10.5));
    }

    #[test]
    fn null_handling() {
        let mut array = ArrowDoubleArray::from_vec(vec![5.0, 6.0, 7.0]);
        NullabilitySupport::set_null(&mut array, 1);
        assert!(NullabilitySupport::is_null(&array, 1));
        assert_eq!(NullabilitySupport::null_count(&array), 1);
        assert_eq!(Collections::get(&array, 1), None);

        Collections::set(&mut array, 1, 42.0);
        assert_eq!(Collections::get(&array, 1), Some(42.0));
        assert_eq!(NullabilitySupport::null_count(&array), 0);
    }

    #[test]
    fn percentile_and_sorting() {
        let mut array = ArrowDoubleArray::from_vec(vec![10.0, 30.0, 20.0, 40.0]);
        assert_eq!(array.percentile_value(50.0), Some(25.0));
        assert_eq!(array.percentile_value(0.0), Some(10.0));
        assert_eq!(array.percentile_value(100.0), Some(40.0));

        let sorted = array.sorted_non_null();
        assert_eq!(sorted, vec![10.0, 20.0, 30.0, 40.0]);
    }
}
