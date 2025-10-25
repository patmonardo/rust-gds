//! Collections Factory: Factory for Creating Collections
//!
//! This module provides factory functions for creating Collections instances,
//! including Huge, Vec, Arrow, and other backend implementations.

use crate::collections::backends::vec::{VecLong, VecDouble, VecInt, VecFloat, VecShort, VecByte, VecBoolean, VecChar};
use crate::collections::backends::huge::{HugeLongArray, HugeDoubleArray, HugeIntArray, HugeFloatArray, HugeShortArray, HugeByteArray, HugeBooleanArray, HugeCharArray};
use crate::collections::traits::{Collections, CollectionsFactory as CollectionsFactoryTrait};
use crate::config::{CollectionsBackend, CollectionsConfig};

/// Factory for creating Collections instances from config
pub struct CollectionFactory;

impl CollectionFactory {
    /// Creates a long (i64) collection based on config
    pub fn create_long_collection(config: &CollectionsConfig<i64>) -> Box<dyn Collections<i64>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(VecLong::with_capacity(capacity))
                } else {
                    Box::new(VecLong::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeLongArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                // TODO: Implement Arrow backend
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(VecLong::with_capacity(capacity))
            }
            _ => {
                // Default to Vec
                Box::new(VecLong::with_capacity(capacity))
            }
        }
    }

    /// Creates a double (f64) collection based on config
    pub fn create_double_collection(config: &CollectionsConfig<f64>) -> Box<dyn Collections<f64>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(VecDouble::with_capacity(capacity))
                } else {
                    Box::new(VecDouble::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeDoubleArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                // TODO: Implement Arrow backend
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(VecDouble::with_capacity(capacity))
            }
            _ => {
                Box::new(VecDouble::with_capacity(capacity))
            }
        }
    }

    /// Creates an int (i32) collection based on config
    pub fn create_int_collection(config: &CollectionsConfig<i32>) -> Box<dyn Collections<i32>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(VecInt::with_capacity(capacity))
                } else {
                    Box::new(VecInt::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeIntArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(VecInt::with_capacity(capacity))
            }
            _ => {
                Box::new(VecInt::with_capacity(capacity))
            }
        }
    }

    /// Creates a float (f32) collection based on config
    pub fn create_float_collection(config: &CollectionsConfig<f32>) -> Box<dyn Collections<f32>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(<VecFloat as CollectionsFactoryTrait<f32>>::with_capacity(capacity))
                } else {
                    Box::new(VecFloat::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeFloatArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(<VecFloat as CollectionsFactoryTrait<f32>>::with_capacity(capacity))
            }
            _ => {
                Box::new(<VecFloat as CollectionsFactoryTrait<f32>>::with_capacity(capacity))
            }
        }
    }

    /// Creates a short (i16) collection based on config
    pub fn create_short_collection(config: &CollectionsConfig<i16>) -> Box<dyn Collections<i16>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(<VecShort as CollectionsFactoryTrait<i16>>::with_capacity(capacity))
                } else {
                    Box::new(VecShort::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeShortArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(<VecShort as CollectionsFactoryTrait<i16>>::with_capacity(capacity))
            }
            _ => {
                Box::new(<VecShort as CollectionsFactoryTrait<i16>>::with_capacity(capacity))
            }
        }
    }

    /// Creates a byte (i8) collection based on config
    pub fn create_byte_collection(config: &CollectionsConfig<i8>) -> Box<dyn Collections<i8>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(<VecByte as CollectionsFactoryTrait<i8>>::with_capacity(capacity))
                } else {
                    Box::new(VecByte::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeByteArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(<VecByte as CollectionsFactoryTrait<i8>>::with_capacity(capacity))
            }
            _ => {
                Box::new(<VecByte as CollectionsFactoryTrait<i8>>::with_capacity(capacity))
            }
        }
    }

    /// Creates a boolean collection based on config
    pub fn create_boolean_collection(config: &CollectionsConfig<bool>) -> Box<dyn Collections<bool>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(<VecBoolean as CollectionsFactoryTrait<bool>>::with_capacity(capacity))
                } else {
                    Box::new(VecBoolean::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeBooleanArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(<VecBoolean as CollectionsFactoryTrait<bool>>::with_capacity(capacity))
            }
            _ => {
                Box::new(<VecBoolean as CollectionsFactoryTrait<bool>>::with_capacity(capacity))
            }
        }
    }

    /// Creates a char collection based on config
    pub fn create_char_collection(config: &CollectionsConfig<char>) -> Box<dyn Collections<char>> {
        let capacity = config.element_type.element_count;
        match config.backend.primary {
            CollectionsBackend::Vec => {
                if capacity > 0 {
                    Box::new(<VecChar as CollectionsFactoryTrait<char>>::with_capacity(capacity))
                } else {
                    Box::new(VecChar::new())
                }
            }
            CollectionsBackend::Huge => {
                Box::new(HugeCharArray::new(capacity))
            }
            CollectionsBackend::Arrow => {
                eprintln!("Warning: Arrow backend not yet implemented, falling back to Vec");
                Box::new(<VecChar as CollectionsFactoryTrait<char>>::with_capacity(capacity))
            }
            _ => {
                Box::new(<VecChar as CollectionsFactoryTrait<char>>::with_capacity(capacity))
            }
        }
    }
}
