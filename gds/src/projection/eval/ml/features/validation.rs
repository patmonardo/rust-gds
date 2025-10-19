/// Feature validation utilities.
///
/// Mirrors Java GDS `FeatureStepUtil` class.
use crate::projection::codegen::ComputeError;
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

/// Get property dimension (1 for scalar, N for array).
pub fn property_dimension(property: &Arc<dyn PropertyValues>) -> usize {
    match property.value_type() {
        ValueType::Long | ValueType::Double => 1,
        ValueType::FloatArray | ValueType::DoubleArray => {
            // For embeddings/arrays, try to get dimension from PropertyValues trait
            // TODO Phase 2.5: Use NodePropertyValues to inspect actual array length
            // For now,  just return 1 as placeholder
            1
        }
        _ => 1,
    }
}

/// Validate features (node-centric).
pub fn validate_features(features: &Arc<dyn PropertyValues>) -> Result<(), ComputeError> {
    if features.element_count() == 0 {
        return Err(ComputeError::StepFailed("Feature values are empty".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::{MockDoublePropertyValues, MockLongPropertyValues};

    #[test]
    fn test_property_dimension_scalar() {
        let property: Arc<dyn PropertyValues> = Arc::new(MockLongPropertyValues::new(10));
        assert_eq!(property_dimension(&property), 1);
    }

    #[test]
    fn test_validate_features_valid() {
        let features: Arc<dyn PropertyValues> =
            Arc::new(MockDoublePropertyValues::from_vec(vec![0.1, 0.5]));
        assert!(validate_features(&features).is_ok());
    }
}
