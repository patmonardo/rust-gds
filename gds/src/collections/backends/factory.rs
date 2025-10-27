//! Backend Factory: Creates Collections backends from configuration
//!
//! Provides helper functions to instantiate the appropriate Collections backend
//! based on CollectionsConfig settings.

use crate::config::CollectionsConfig;
use crate::collections::backends::vec::*;

/// Creates a Collections backend for i64 values from config and data
/// 
/// Currently only supports Vec backend. Huge and Arrow backends will be added in future phases.
pub fn create_long_backend_from_config(
    _config: &CollectionsConfig<i64>,
    values: Vec<i64>,
) -> VecLong {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecLong::from(values)
}

/// Creates a Collections backend for f64 values from config and data
pub fn create_double_backend_from_config(
    _config: &CollectionsConfig<f64>,
    values: Vec<f64>,
) -> VecDouble {
    // For now, always use Vec backend regardless of config
    // TODO: Implement adaptive backend selection when Huge and Arrow are ready
    VecDouble::from(values)
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
    use crate::collections::traits::Collections;

    #[test]
    fn test_create_long_backend_vec() {
        let config = CollectionsConfig::<i64>::default();
        
        let backend = create_long_backend_from_config(&config, vec![1, 2, 3]);
        assert_eq!(backend.len(), 3);
        assert_eq!(backend.get(0), Some(1));
    }

    #[test]
    fn test_create_double_backend_vec() {
        let config = CollectionsConfig::<f64>::default();
        
        let backend = create_double_backend_from_config(&config, vec![1.0, 2.0, 3.0]);
        assert_eq!(backend.len(), 3);
        assert_eq!(backend.get(0), Some(1.0));
    }

    #[test]
    fn test_fallback_to_vec_for_huge() {
        let config = CollectionsConfig::<i64>::default();
        
        // Should fall back to Vec until Huge is implemented
        let backend = create_long_backend_from_config(&config, vec![1, 2, 3]);
        assert_eq!(backend.len(), 3);
    }
}

