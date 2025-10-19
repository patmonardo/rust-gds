//! Type Validator - The Pinky Finger: Inference as Brahman-Knowing
//!
//! This module completes the Brahmachakra (Wheel of Brahman) by providing
//! **runtime validation** and **descriptor inference** - the ability to look
//! at actual VALUES and reconstruct the DESCRIPTORS.
//!
//! # Philosophical Foundation
//!
//! TypeValidator IS the **Computation Extreme** - the final finger of Dakṣiṇāmūrti's
//! Teaching Hand. Where TypeProjector maps Form → (Storage, Computation), TypeValidator
//! does the INVERSE: Values → Descriptors.
//!
//! This IS:
//! - **Nāma to Rūpa** (Name to Form) - inferring schema from data
//! - **Metaphysical Idealism** - the Idea reconstructed from manifestation
//! - **Zod-like validation** - runtime type checking with inference
//!
//! # The Five-Fold Synthesis (Brahmachakra)
//!
//! ```text
//!          TypeValidator (Pinky)
//!                |
//!                |  Adaptive (3rd Middle)
//!                |  /
//!                | /  Pregel (2nd Middle)
//!                |/  /
//!          COMPUTATION
//!               |  /  Arrow (1st Middle)
//!               | /  /
//!               |/  /
//!          ────●──── PropertyDescriptor (CENTER)
//!              /|  \
//!             / |   \
//!            /  |    \
//!        STORAGE     HugeArray (Thumb)
//!                    
//!        MONITOR (Your Eyes) - The Gross Extreme
//! ```
//!
//! # Core Capabilities
//!
//! 1. **Runtime Validation**: Check if values conform to descriptor
//! 2. **Descriptor Inference**: Look at actual values, infer PropertyDescriptor
//! 3. **Compatibility Checking**: Validate descriptor pairs are compatible
//! 4. **Schema Migration**: Detect when descriptor needs to evolve
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::{TypeValidator, PropertyDescriptor};
//! use gds::types::ValueType;
//!
//! // Infer descriptor from actual values
//! let values = vec![42i64, 1337i64, 99i64];
//! let inferred = TypeValidator::infer_from_values(&values)?;
//! assert_eq!(inferred.value_type, ValueType::Long);
//!
//! // Validate values against descriptor
//! let descriptor = PropertyDescriptor::new(1, "age", ValueType::Long);
//! assert!(TypeValidator::validate_values(&descriptor, &values).is_ok());
//! ```

use std::error::Error;
use std::fmt;

use crate::types::ValueType;

// Import descriptors from the descriptors module
use crate::projection::codegen::descriptors::{PropertyDescriptor, StorageHint};

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during type validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Value type mismatch
    TypeMismatch {
        expected: ValueType,
        found: String,
        context: String,
    },
    /// Nullable violation
    NullableViolation { property: String, index: usize },
    /// Incompatible descriptors
    IncompatibleDescriptors {
        descriptor1: String,
        descriptor2: String,
        reason: String,
    },
    /// Cannot infer descriptor from empty data
    InsufficientData { reason: String },
    /// Custom validation error
    Custom(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch {
                expected,
                found,
                context,
            } => write!(
                f,
                "Type mismatch in {}: expected {:?}, found {}",
                context, expected, found
            ),
            Self::NullableViolation { property, index } => write!(
                f,
                "Null value at index {} for non-nullable property '{}'",
                index, property
            ),
            Self::IncompatibleDescriptors {
                descriptor1,
                descriptor2,
                reason,
            } => write!(
                f,
                "Incompatible descriptors: {} and {} ({})",
                descriptor1, descriptor2, reason
            ),
            Self::InsufficientData { reason } => {
                write!(f, "Insufficient data for inference: {}", reason)
            }
            Self::Custom(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for ValidationError {}

// ============================================================================
// Type Validator - The Pinky Finger
// ============================================================================

/// Type validator for runtime validation and descriptor inference
///
/// This IS the **Computation Extreme** - the Pinky finger that completes
/// Dakṣiṇāmūrti's Teaching Hand. It provides:
///
/// 1. **Inference** (Avidyā) - Recognize Form from Values
/// 2. **Validation** (Brahman) - Know consistency between Form and Values
/// 3. **Migration** (Īśvara) - Evolve descriptors as data evolves
///
/// # Philosophical Position
///
/// TypeValidator IS the INVERSE of TypeProjector:
/// - TypeProjector: Form → (Storage, Computation)
/// - TypeValidator: Values → Form
///
/// This IS **Nāma to Rūpa** - inferring the essential nature (Svarūpa)
/// from the phenomenal manifestation (Rūpa).
pub struct TypeValidator;

impl TypeValidator {
    /// Infer PropertyDescriptor from actual i64 values
    ///
    /// This IS **Avidyā** (Concealing/Recognition) - looking at the Gross
    /// manifestation (values) and recognizing the essential Form (descriptor).
    ///
    /// # Philosophy
    ///
    /// The values ARE the Rūpa (Form/manifestation). The descriptor IS the
    /// Nāma (Name/essence). Inference IS the movement from Rūpa → Nāma.
    pub fn infer_from_i64_values(
        property_id: u32,
        name: impl Into<String>,
        values: &[i64],
    ) -> Result<PropertyDescriptor, ValidationError> {
        if values.is_empty() {
            return Err(ValidationError::InsufficientData {
                reason: "Cannot infer from empty value array".to_string(),
            });
        }

        // All i64 values → Long type
        let descriptor = PropertyDescriptor::new(property_id, name, ValueType::Long)
            .with_storage_hint(StorageHint::FixedWidth) // i64 is fixed-width
            .with_nullable(false); // Actual values present, assume non-null

        Ok(descriptor)
    }

    /// Infer PropertyDescriptor from actual f64 values
    pub fn infer_from_f64_values(
        property_id: u32,
        name: impl Into<String>,
        values: &[f64],
    ) -> Result<PropertyDescriptor, ValidationError> {
        if values.is_empty() {
            return Err(ValidationError::InsufficientData {
                reason: "Cannot infer from empty value array".to_string(),
            });
        }

        let descriptor = PropertyDescriptor::new(property_id, name, ValueType::Double)
            .with_storage_hint(StorageHint::FixedWidth)
            .with_nullable(false);

        Ok(descriptor)
    }

    /// Infer PropertyDescriptor from actual bool values
    pub fn infer_from_bool_values(
        property_id: u32,
        name: impl Into<String>,
        values: &[bool],
    ) -> Result<PropertyDescriptor, ValidationError> {
        if values.is_empty() {
            return Err(ValidationError::InsufficientData {
                reason: "Cannot infer from empty value array".to_string(),
            });
        }

        let descriptor = PropertyDescriptor::new(property_id, name, ValueType::Boolean)
            .with_storage_hint(StorageHint::FixedWidth)
            .with_nullable(false);

        Ok(descriptor)
    }

    /// Infer PropertyDescriptor from actual String values
    pub fn infer_from_string_values(
        property_id: u32,
        name: impl Into<String>,
        values: &[String],
    ) -> Result<PropertyDescriptor, ValidationError> {
        if values.is_empty() {
            return Err(ValidationError::InsufficientData {
                reason: "Cannot infer from empty value array".to_string(),
            });
        }

        let descriptor = PropertyDescriptor::new(property_id, name, ValueType::String)
            .with_storage_hint(StorageHint::VariableLength) // Strings are variable-length
            .with_nullable(false);

        Ok(descriptor)
    }

    /// Validate i64 values against PropertyDescriptor
    ///
    /// This IS **Brahman** (Knowing) - checking dialectical consistency
    /// between Form (descriptor) and manifestation (values).
    pub fn validate_i64_values(
        descriptor: &PropertyDescriptor,
        _values: &[i64],
    ) -> Result<(), ValidationError> {
        // Check value type matches
        if descriptor.value_type != ValueType::Long {
            return Err(ValidationError::TypeMismatch {
                expected: descriptor.value_type,
                found: "i64 array".to_string(),
                context: format!("property '{}'", descriptor.name),
            });
        }

        // All validation passed
        Ok(())
    }

    /// Validate f64 values against PropertyDescriptor
    pub fn validate_f64_values(
        descriptor: &PropertyDescriptor,
        _values: &[f64],
    ) -> Result<(), ValidationError> {
        if descriptor.value_type != ValueType::Double {
            return Err(ValidationError::TypeMismatch {
                expected: descriptor.value_type,
                found: "f64 array".to_string(),
                context: format!("property '{}'", descriptor.name),
            });
        }

        Ok(())
    }

    /// Validate bool values against PropertyDescriptor
    pub fn validate_bool_values(
        descriptor: &PropertyDescriptor,
        _values: &[bool],
    ) -> Result<(), ValidationError> {
        if descriptor.value_type != ValueType::Boolean {
            return Err(ValidationError::TypeMismatch {
                expected: descriptor.value_type,
                found: "bool array".to_string(),
                context: format!("property '{}'", descriptor.name),
            });
        }

        Ok(())
    }

    /// Validate String values against PropertyDescriptor
    pub fn validate_string_values(
        descriptor: &PropertyDescriptor,
        _values: &[String],
    ) -> Result<(), ValidationError> {
        if descriptor.value_type != ValueType::String {
            return Err(ValidationError::TypeMismatch {
                expected: descriptor.value_type,
                found: "String array".to_string(),
                context: format!("property '{}'", descriptor.name),
            });
        }

        Ok(())
    }

    /// Check if two PropertyDescriptors are compatible
    ///
    /// Compatible means they can coexist in the same projection system.
    /// This IS **Īśvara's Sthiti** (Preservation) - maintaining harmony.
    pub fn check_compatibility(
        desc1: &PropertyDescriptor,
        desc2: &PropertyDescriptor,
    ) -> Result<(), ValidationError> {
        // Same ID → must be identical
        if desc1.id == desc2.id
            && (desc1.name != desc2.name || desc1.value_type != desc2.value_type)
        {
            return Err(ValidationError::IncompatibleDescriptors {
                descriptor1: format!("PropertyDescriptor(id={}, name={})", desc1.id, desc1.name),
                descriptor2: format!("PropertyDescriptor(id={}, name={})", desc2.id, desc2.name),
                reason: "Same ID but different name or value_type".to_string(),
            });
        }

        // Different IDs → OK (can coexist)
        Ok(())
    }

    /// Detect if descriptor needs to evolve based on new values
    ///
    /// This IS **Īśvara's Saṃhāra-Sṛṣṭi** (Destruction-Creation) -
    /// recognizing when old schema must be destroyed and new schema created.
    pub fn needs_migration(current: &PropertyDescriptor, new_values_type: ValueType) -> bool {
        current.value_type != new_values_type
    }

    /// Suggest migration path from old descriptor to new
    pub fn suggest_migration(
        current: &PropertyDescriptor,
        new_type: ValueType,
    ) -> Result<PropertyDescriptor, ValidationError> {
        // Create new descriptor with evolved type
        let migrated = PropertyDescriptor::new(
            current.id,
            format!("{}_v2", current.name), // Version bump
            new_type,
        )
        .with_storage_hint(match new_type {
            ValueType::String => StorageHint::VariableLength,
            _ => StorageHint::FixedWidth,
        })
        .with_nullable(current.nullable);

        Ok(migrated)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_from_i64_values() {
        let values = vec![42i64, 1337i64, 99i64];
        let descriptor = TypeValidator::infer_from_i64_values(1, "age", &values).unwrap();

        assert_eq!(descriptor.id, 1);
        assert_eq!(descriptor.name, "age");
        assert_eq!(descriptor.value_type, ValueType::Long);
        assert_eq!(descriptor.storage_hint, StorageHint::FixedWidth);
        assert!(!descriptor.nullable);
    }

    #[test]
    fn test_infer_from_f64_values() {
        let values = vec![3.14, 2.718, 1.414];
        let descriptor = TypeValidator::infer_from_f64_values(2, "score", &values).unwrap();

        assert_eq!(descriptor.value_type, ValueType::Double);
        assert_eq!(descriptor.storage_hint, StorageHint::FixedWidth);
    }

    #[test]
    fn test_infer_from_bool_values() {
        let values = vec![true, false, true];
        let descriptor = TypeValidator::infer_from_bool_values(3, "active", &values).unwrap();

        assert_eq!(descriptor.value_type, ValueType::Boolean);
    }

    #[test]
    fn test_infer_from_string_values() {
        let values = vec!["foo".to_string(), "bar".to_string()];
        let descriptor = TypeValidator::infer_from_string_values(4, "name", &values).unwrap();

        assert_eq!(descriptor.value_type, ValueType::String);
        assert_eq!(descriptor.storage_hint, StorageHint::VariableLength);
    }

    #[test]
    fn test_infer_from_empty_fails() {
        let values: Vec<i64> = vec![];
        let result = TypeValidator::infer_from_i64_values(5, "empty", &values);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::InsufficientData { .. }
        ));
    }

    #[test]
    fn test_validate_i64_values_success() {
        let descriptor = PropertyDescriptor::new(1, "count", ValueType::Long);
        let values = vec![1i64, 2i64, 3i64];

        let result = TypeValidator::validate_i64_values(&descriptor, &values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_i64_values_type_mismatch() {
        let descriptor = PropertyDescriptor::new(1, "score", ValueType::Double); // Wrong type
        let values = vec![1i64, 2i64, 3i64];

        let result = TypeValidator::validate_i64_values(&descriptor, &values);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::TypeMismatch { .. }
        ));
    }

    #[test]
    fn test_validate_f64_values_success() {
        let descriptor = PropertyDescriptor::new(2, "ratio", ValueType::Double);
        let values = vec![1.5, 2.5, 3.5];

        let result = TypeValidator::validate_f64_values(&descriptor, &values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_compatibility_same_id_compatible() {
        let desc1 = PropertyDescriptor::new(1, "prop", ValueType::Long);
        let desc2 = PropertyDescriptor::new(1, "prop", ValueType::Long);

        let result = TypeValidator::check_compatibility(&desc1, &desc2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_compatibility_same_id_incompatible() {
        let desc1 = PropertyDescriptor::new(1, "prop", ValueType::Long);
        let desc2 = PropertyDescriptor::new(1, "prop", ValueType::Double); // Different type

        let result = TypeValidator::check_compatibility(&desc1, &desc2);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::IncompatibleDescriptors { .. }
        ));
    }

    #[test]
    fn test_check_compatibility_different_ids() {
        let desc1 = PropertyDescriptor::new(1, "prop1", ValueType::Long);
        let desc2 = PropertyDescriptor::new(2, "prop2", ValueType::Double);

        let result = TypeValidator::check_compatibility(&desc1, &desc2);
        assert!(result.is_ok()); // Different IDs → compatible
    }

    #[test]
    fn test_needs_migration() {
        let descriptor = PropertyDescriptor::new(1, "value", ValueType::Long);

        assert!(!TypeValidator::needs_migration(
            &descriptor,
            ValueType::Long
        ));
        assert!(TypeValidator::needs_migration(
            &descriptor,
            ValueType::Double
        ));
    }

    #[test]
    fn test_suggest_migration() {
        let current = PropertyDescriptor::new(1, "value", ValueType::Long);
        let migrated = TypeValidator::suggest_migration(&current, ValueType::Double).unwrap();

        assert_eq!(migrated.id, 1);
        assert!(migrated.name.contains("_v2")); // Version bump
        assert_eq!(migrated.value_type, ValueType::Double);
    }

    #[test]
    fn test_inference_roundtrip() {
        // Infer descriptor from values
        let values = vec![10i64, 20i64, 30i64];
        let inferred = TypeValidator::infer_from_i64_values(99, "test_prop", &values).unwrap();

        // Validate same values against inferred descriptor
        let validation = TypeValidator::validate_i64_values(&inferred, &values);
        assert!(validation.is_ok());

        // This IS Brahman knowing - Form and manifestation are consistent!
    }
}
