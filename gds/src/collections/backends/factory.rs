//! Backend Factory: Creates Collections backends from configuration
//!
//! Provides helper functions to instantiate the appropriate Collections backend
//! based on CollectionsConfig settings.

use crate::collections::backends::arrow::{ArrowDoubleArray, ArrowLongArray};
use crate::collections::backends::huge::{HugeDoubleArray, HugeLongArray};
use crate::collections::backends::vec::*;
use crate::collections::traits::Collections;
use crate::config::{CollectionsBackend, CollectionsConfig};

/// Long collection backend variants produced by this factory
#[derive(Debug)]
pub enum LongCollection {
    Vec(VecLong),
    Huge(HugeLongArray),
    Arrow(ArrowLongArray),
}

impl LongCollection {
    pub fn len(&self) -> usize {
        match self {
            LongCollection::Vec(collection) => collection.len(),
            LongCollection::Huge(collection) => collection.len(),
            LongCollection::Arrow(collection) => collection.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<i64> {
        match self {
            LongCollection::Vec(collection) => collection.get(index),
            LongCollection::Huge(collection) => Some(collection.get(index)),
            LongCollection::Arrow(collection) => collection.get(index),
        }
    }
}

/// Double collection backend variants produced by this factory
#[derive(Debug)]
pub enum DoubleCollection {
    Vec(VecDouble),
    Huge(HugeDoubleArray),
    Arrow(ArrowDoubleArray),
}

impl DoubleCollection {
    pub fn len(&self) -> usize {
        match self {
            DoubleCollection::Vec(collection) => collection.len(),
            DoubleCollection::Huge(collection) => collection.len(),
            DoubleCollection::Arrow(collection) => collection.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        match self {
            DoubleCollection::Vec(collection) => collection.get(index),
            DoubleCollection::Huge(collection) => Some(collection.get(index)),
            DoubleCollection::Arrow(collection) => collection.get(index),
        }
    }
}

fn resolve_backend<T>(config: &CollectionsConfig<T>) -> CollectionsBackend {
    let mut candidates =
        std::iter::once(config.backend.primary).chain(config.backend.fallbacks.iter().copied());
    candidates
        .find(|backend| {
            matches!(
                backend,
                CollectionsBackend::Vec | CollectionsBackend::Huge | CollectionsBackend::Arrow
            )
        })
        .unwrap_or(CollectionsBackend::Vec)
}

/// Creates a Collections backend for i64 values from config and data.
///
/// Supports Vec, Huge, and Arrow collections depending on the selected backend.
pub fn create_long_backend_from_config(
    config: &CollectionsConfig<i64>,
    values: Vec<i64>,
) -> LongCollection {
    match resolve_backend(config) {
        CollectionsBackend::Arrow => LongCollection::Arrow(ArrowLongArray::from_vec(values)),
        CollectionsBackend::Huge => LongCollection::Huge(HugeLongArray::from_vec(values)),
        _ => LongCollection::Vec(VecLong::from(values)),
    }
}

/// Creates a Collections backend for f64 values from config and data.
pub fn create_double_backend_from_config(
    config: &CollectionsConfig<f64>,
    values: Vec<f64>,
) -> DoubleCollection {
    match resolve_backend(config) {
        CollectionsBackend::Arrow => DoubleCollection::Arrow(ArrowDoubleArray::from_vec(values)),
        CollectionsBackend::Huge => DoubleCollection::Huge(HugeDoubleArray::from_vec(values)),
        _ => DoubleCollection::Vec(VecDouble::from(values)),
    }
}

/// Creates a Collections backend for f32 values from config and data
pub fn create_float_backend_from_config(
    _config: &CollectionsConfig<f32>,
    values: Vec<f32>,
) -> VecFloat {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecFloat::from(values)
}

/// Creates a Collections backend for i32 values from config and data
pub fn create_int_backend_from_config(
    _config: &CollectionsConfig<i32>,
    values: Vec<i32>,
) -> VecInt {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecInt::from(values)
}

/// Creates a Collections backend for i16 values from config and data
pub fn create_short_backend_from_config(
    _config: &CollectionsConfig<i16>,
    values: Vec<i16>,
) -> VecShort {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecShort { data: values }
}

/// Creates a Collections backend for i8 values from config and data
pub fn create_byte_backend_from_config(
    _config: &CollectionsConfig<i8>,
    values: Vec<i8>,
) -> VecByte {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecByte { data: values }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CollectionsConfigBuilder;

    #[test]
    fn test_create_long_backend_vec() {
        let config = CollectionsConfig::<i64>::default();

        match create_long_backend_from_config(&config, vec![1, 2, 3]) {
            LongCollection::Vec(vec_backend) => {
                assert_eq!(vec_backend.len(), 3);
                assert_eq!(vec_backend.get(0), Some(1));
            }
            other => panic!("expected Vec backend, got {:?}", other),
        }
    }

    #[test]
    fn test_create_double_backend_vec() {
        let config = CollectionsConfig::<f64>::default();

        match create_double_backend_from_config(&config, vec![1.0, 2.0, 3.0]) {
            DoubleCollection::Vec(vec_backend) => {
                assert_eq!(vec_backend.len(), 3);
                assert_eq!(vec_backend.get(0), Some(1.0));
            }
            other => panic!("expected Vec backend, got {:?}", other),
        }
    }

    #[test]
    fn test_arrow_backend_selection() {
        let config = CollectionsConfigBuilder::<i64>::new()
            .with_backend(CollectionsBackend::Arrow)
            .build();

        match create_long_backend_from_config(&config, vec![42, 7]) {
            LongCollection::Arrow(arrow_backend) => {
                assert_eq!(arrow_backend.len(), 2);
                assert_eq!(arrow_backend.get(1), Some(7));
            }
            other => panic!("expected Arrow backend, got {:?}", other),
        }
    }

    #[test]
    fn test_huge_backend_selection() {
        let config = CollectionsConfigBuilder::<f64>::new()
            .with_backend(CollectionsBackend::Huge)
            .build();

        match create_double_backend_from_config(&config, vec![10.0, 11.0, 12.0]) {
            DoubleCollection::Huge(huge_backend) => {
                assert_eq!(huge_backend.len(), 3);
                assert_eq!(huge_backend.get(2), 12.0);
            }
            other => panic!("expected Huge backend, got {:?}", other),
        }
    }
}
