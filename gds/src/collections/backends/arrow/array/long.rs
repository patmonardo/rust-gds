use std::sync::Arc;

use crate::collections::traits::{
    AggregationSupport, Collections, NullabilitySupport, PropertyValuesAdapter,
};
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use arrow2::array::{Array, MutablePrimitiveArray, PrimitiveArray};

use super::ArrowArrayBehavior;

const DEFAULT_LONG_VALUE: i64 = 0;
const EMPTY_EXTENSIONS: [Extension; 0] = [];

#[derive(Clone, Debug)]
pub struct ArrowLongArray {
    array: Arc<PrimitiveArray<i64>>,
    default_value: i64,
}

impl ArrowLongArray {
    pub fn new() -> Self {
        Self::with_defaults(0, DEFAULT_LONG_VALUE)
    }

    pub fn from_arc(array: Arc<PrimitiveArray<i64>>) -> Self {
        Self {
            array,
            default_value: DEFAULT_LONG_VALUE,
        }
    }

    pub fn from_vec(values: Vec<i64>) -> Self {
        let array = PrimitiveArray::from_vec(values);
        Self {
            array: Arc::new(array),
            default_value: DEFAULT_LONG_VALUE,
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn values(&self) -> &[i64] {
        self.array.values()
    }

    pub fn arrow(&self) -> &Arc<PrimitiveArray<i64>> {
        &self.array
    }

    pub fn into_arrow(self) -> Arc<PrimitiveArray<i64>> {
        self.array
    }

    fn rebuild_from_options(&mut self, values: Vec<Option<i64>>) {
        let mutable: MutablePrimitiveArray<i64> = values.into_iter().collect();
        let primitive: PrimitiveArray<i64> = mutable.into();
        self.array = Arc::new(primitive);
    }

    fn as_option_vec(&self) -> Vec<Option<i64>> {
        self.array.iter().map(|value| value.copied()).collect()
    }

    fn update_entry(&mut self, index: usize, value: Option<i64>) {
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

    fn iter_non_null(&self) -> impl Iterator<Item = i64> + '_ {
        self.array.iter().flatten().copied()
    }

    fn non_null_len(&self) -> usize {
        self.array.len() - self.array.null_count()
    }

    fn compute_sum(&self) -> Option<i64> {
        let mut iter = self.iter_non_null();
        let first = iter.next()?;
        Some(iter.fold(first, |acc, value| acc + value))
    }

    fn compute_mean(&self) -> Option<f64> {
        let mut iter = self.iter_non_null();
        let first = iter.next()? as f64;
        let (sum, count) = iter.fold((first, 1usize), |(acc, n), value| {
            (acc + value as f64, n + 1)
        });
        Some(sum / count as f64)
    }

    fn compute_min(&self) -> Option<i64> {
        self.iter_non_null().min()
    }

    fn compute_max(&self) -> Option<i64> {
        self.iter_non_null().max()
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
                let diff = value as f64 - mean;
                diff * diff
            })
            .sum();
        Some(sum_sq / (count - 1) as f64)
    }

    fn dense_values(&self) -> Vec<i64> {
        self.array
            .iter()
            .map(|value| value.copied().unwrap_or(self.default_value))
            .collect()
    }

    fn sorted_non_null(&self) -> Vec<i64> {
        let mut values: Vec<i64> = self.iter_non_null().collect();
        values.sort_unstable();
        values
    }
}

impl ArrowArrayBehavior for ArrowLongArray {
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

impl Collections<i64> for ArrowLongArray {
    fn get(&self, index: usize) -> Option<i64> {
        if index >= ArrowArrayBehavior::len(self) || ArrowArrayBehavior::is_null(self, index) {
            None
        } else {
            Some(self.array.value(index))
        }
    }

    fn set(&mut self, index: usize, value: i64) {
        self.update_entry(index, Some(value));
    }

    fn len(&self) -> usize {
        ArrowArrayBehavior::len(self)
    }

    fn sum(&self) -> Option<i64> {
        self.compute_sum()
    }

    fn min(&self) -> Option<i64> {
        self.compute_min()
    }

    fn max(&self) -> Option<i64> {
        self.compute_max()
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

    fn median(&self) -> Option<i64> {
        let values = self.sorted_non_null();
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        if values.len() % 2 == 0 {
            Some((values[mid - 1] + values[mid]) / 2)
        } else {
            Some(values[mid])
        }
    }

    fn percentile(&self, p: f64) -> Option<i64> {
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
        let lower_value = values.get(lower_index).copied()?;
        let upper_value = values.get(upper_index).copied()?;
        if lower_index == upper_index {
            Some(lower_value)
        } else {
            let lower = lower_value as f64;
            let upper = upper_value as f64;
            let weight = rank - lower_index as f64;
            let interpolated = lower + (upper - lower) * weight;
            Some(interpolated.round() as i64)
        }
    }

    fn binary_search(&self, key: &i64) -> Result<usize, usize> {
        let values = self.sorted_non_null();
        values.binary_search(key)
    }

    fn sort(&mut self)
    where
        i64: Ord,
    {
        let mut values = self.dense_values();
        values.sort_unstable();
        self.rebuild_from_options(values.into_iter().map(Some).collect());
    }

    fn to_vec(self) -> Vec<i64> {
        self.dense_values()
    }

    fn as_slice(&self) -> &[i64] {
        self.array.values()
    }

    fn is_null(&self, index: usize) -> bool {
        ArrowArrayBehavior::is_null(self, index)
    }

    fn null_count(&self) -> usize {
        ArrowArrayBehavior::null_count(self)
    }

    fn default_value(&self) -> i64 {
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
        ValueType::Long
    }

    fn with_capacity(capacity: usize) -> Self {
        let mutable = MutablePrimitiveArray::<i64>::with_capacity(capacity);
        let primitive: PrimitiveArray<i64> = mutable.into();
        Self {
            array: Arc::new(primitive),
            default_value: DEFAULT_LONG_VALUE,
        }
    }

    fn with_defaults(count: usize, default_value: i64) -> Self {
        let array = PrimitiveArray::from_vec(vec![default_value; count]);
        Self {
            array: Arc::new(array),
            default_value,
        }
    }
}

impl NullabilitySupport<i64> for ArrowLongArray {
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

impl AggregationSupport<i64> for ArrowLongArray {
    fn sum(&self) -> Option<i64> {
        Collections::sum(self)
    }

    fn mean(&self) -> Option<f64> {
        Collections::mean(self)
    }

    fn std_dev(&self) -> Option<f64> {
        Collections::std_dev(self)
    }

    fn variance(&self) -> Option<f64> {
        Collections::variance(self)
    }

    fn median(&self) -> Option<i64> {
        Collections::median(self)
    }

    fn percentile(&self, p: f64) -> Option<i64> {
        Collections::percentile(self, p)
    }
}

impl PropertyValuesAdapter<i64> for ArrowLongArray {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::traits::{AggregationSupport, Collections, NullabilitySupport};

    #[test]
    fn basic_operations() {
        let mut array = ArrowLongArray::from_vec(vec![1, 2, 3]);
        assert_eq!(Collections::len(&array), 3);
        assert_eq!(Collections::get(&array, 1), Some(2));

        array.set(1, 10);
        assert_eq!(Collections::get(&array, 1), Some(10));
        assert_eq!(Collections::sum(&array), Some(1 + 10 + 3));
        assert_eq!(Collections::min(&array), Some(1));
        assert_eq!(Collections::max(&array), Some(10));
        assert_eq!(Collections::mean(&array), Some((1.0 + 10.0 + 3.0) / 3.0));
    }

    #[test]
    fn null_handling() {
        let mut array = ArrowLongArray::from_vec(vec![5, 6, 7]);
        array.set_null(1);
        assert!(NullabilitySupport::is_null(&array, 1));
        assert_eq!(NullabilitySupport::null_count(&array), 1);
        assert_eq!(Collections::get(&array, 1), None);

        array.set(1, 42);
        assert_eq!(Collections::get(&array, 1), Some(42));
        assert_eq!(NullabilitySupport::null_count(&array), 0);
    }

    #[test]
    fn percentile_and_median() {
        let array = ArrowLongArray::from_vec(vec![10, 30, 20, 40]);
        assert_eq!(Collections::median(&array), Some(25));
        assert_eq!(AggregationSupport::percentile(&array, 50.0), Some(25));
        assert_eq!(AggregationSupport::percentile(&array, 0.0), Some(10));
        assert_eq!(AggregationSupport::percentile(&array, 100.0), Some(40));
    }
}
