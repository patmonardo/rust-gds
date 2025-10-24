//! Selector Validation: Type-Safe Property Selectors
//!
//! This module provides macros for validating property selectors to ensure
//! they have the correct ValueType for algorithm requirements.
//!
//! **Pattern**: Algorithm specs validate selector types at parse/validate time
//! to ensure weights are numeric, keys are hashable, etc.

use crate::types::ValueType;

/// Macro to generate selector validation for algorithm specs
/// 
/// This macro generates validation logic for property selectors to ensure
/// they have the correct ValueType for the algorithm's requirements.
/// 
/// # Example
/// 
/// ```rust
/// selector_validation! {
///     DijkstraConfig,
///     weight_selector: NumericSelector => {
///         validate_numeric();
///         validate_double();
///     }
/// }
/// ```
#[macro_export]
macro_rules! selector_validation {
    ($config_type:ty, $selector_field:ident: $selector_type:ty => {
        $(
            $validation_method:ident();
        )*
    }) => {
        impl $config_type {
            $(
                /// Validates the selector has the correct type
                pub fn $validation_method(&self) -> Result<(), String> {
                    // This would be implemented based on the specific selector type
                    // For now, we'll use a placeholder implementation
                    Ok(())
                }
            )*
        }
    };
}

/// Macro to generate type validation for specific ValueType requirements
/// 
/// This macro generates validation methods for ensuring selectors have
/// the correct ValueType for algorithm requirements.
#[macro_export]
macro_rules! validate_selector_type {
    ($config_type:ty, $selector_field:ident, $expected_type:ident) => {
        impl $config_type {
            /// Validates the selector has the expected ValueType
            pub fn validate_selector_type(&self) -> Result<(), String> {
                let actual_type = self.$selector_field.get_value_type()?;
                if actual_type != ValueType::$expected_type {
                    return Err(format!("Type mismatch: expected {:?}, got {:?}", ValueType::$expected_type, actual_type));
                }
                Ok(())
            }
            
            /// Validates the selector is compatible with the expected type
            pub fn validate_selector_compatibility(&self) -> Result<(), String> {
                let actual_type = self.$selector_field.get_value_type()?;
                if !ValueType::$expected_type.is_compatible_with(&actual_type) {
                    return Err(format!("Type incompatibility: {:?} is not compatible with {:?}", actual_type, ValueType::$expected_type));
                }
                Ok(())
            }
            
            /// Validates the selector with a custom error message
            pub fn validate_selector_with_message(&self) -> Result<(), String> {
                let actual_type = self.$selector_field.get_value_type()?;
                if actual_type != ValueType::$expected_type {
                    return Err(format!("Selector validation failed: expected {:?}, got {:?}", ValueType::$expected_type, actual_type));
                }
                Ok(())
            }
        }
    };
}

/// Macro to generate validation for multiple selectors
/// 
/// This macro generates validation methods for multiple selectors in a config.
#[macro_export]
macro_rules! validate_all_selectors {
    ($config_type:ty, $($selector_field:ident: $expected_type:ident),*) => {
        impl $config_type {
            /// Validates all selectors have the correct types
            pub fn validate_all_selectors(&self) -> Result<(), String> {
                $(
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::$expected_type {
                        return Err(format!("Type mismatch for {}: expected {:?}, got {:?}", stringify!($selector_field), ValueType::$expected_type, actual_type));
                    }
                )*
                Ok(())
            }
            
            /// Validates all selectors are compatible with expected types
            pub fn validate_all_selectors_compatibility(&self) -> Result<(), String> {
                $(
                    let actual_type = self.$selector_field.get_value_type()?;
                    if !ValueType::$expected_type.is_compatible_with(&actual_type) {
                        return Err(format!("Type incompatibility for {}: {:?} is not compatible with {:?}", stringify!($selector_field), actual_type, ValueType::$expected_type));
                    }
                )*
                Ok(())
            }
            
            /// Validates all selectors with custom error messages
            pub fn validate_all_selectors_with_messages(&self) -> Result<(), String> {
                $(
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::$expected_type {
                        return Err(format!("Validation failed for {}: expected {:?}, got {:?}", stringify!($selector_field), ValueType::$expected_type, actual_type));
                    }
                )*
                Ok(())
            }
        }
    };
}

/// Macro to generate specific type validations
/// 
/// This macro generates validation methods for specific ValueType requirements.
#[macro_export]
macro_rules! validate_numeric_selectors {
    ($config_type:ty, $($selector_field:ident),*) => {
        impl $config_type {
            $(
                /// Validates the selector is numeric
                pub fn validate_numeric_selector(&self) -> Result<(), String> {
                    let actual_type = self.$selector_field.get_value_type()?;
                    if !matches!(actual_type, ValueType::Long | ValueType::Double | ValueType::Float | ValueType::Int | ValueType::Short | ValueType::Byte) {
                        return Err(format!("Selector {} must be numeric, got {:?}", stringify!($selector_field), actual_type));
                    }
                    Ok(())
                }
                
                /// Validates the selector is a double
                pub fn validate_double_selector(&self) -> Result<(), String> {
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::Double {
                        return Err(format!("Selector {} must be Double, got {:?}", stringify!($selector_field), actual_type));
                    }
                    Ok(())
                }
                
                /// Validates the selector is a long
                pub fn validate_long_selector(&self) -> Result<(), String> {
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::Long {
                        return Err(format!("Selector {} must be Long, got {:?}", stringify!($selector_field), actual_type));
                    }
                    Ok(())
                }
            )*
        }
    };
}

/// Macro to generate string selector validations
/// 
/// This macro generates validation methods for string selectors.
#[macro_export]
macro_rules! validate_string_selectors {
    ($config_type:ty, $($selector_field:ident),*) => {
        impl $config_type {
            $(
                /// Validates the selector is a string
                pub fn validate_string_selector(&self) -> Result<(), String> {
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::String {
                        return Err(format!("Selector {} must be String, got {:?}", stringify!($selector_field), actual_type));
                    }
                    Ok(())
                }
            )*
        }
    };
}

/// Macro to generate boolean selector validations
/// 
/// This macro generates validation methods for boolean selectors.
#[macro_export]
macro_rules! validate_boolean_selectors {
    ($config_type:ty, $($selector_field:ident),*) => {
        impl $config_type {
            $(
                /// Validates the selector is a boolean
                pub fn validate_boolean_selector(&self) -> Result<(), String> {
                    let actual_type = self.$selector_field.get_value_type()?;
                    if actual_type != ValueType::Boolean {
                        return Err(format!("Selector {} must be Boolean, got {:?}", stringify!($selector_field), actual_type));
                    }
                    Ok(())
                }
            )*
        }
    };
}

/// Trait for selectors that can provide their ValueType
/// 
/// This trait enables selectors to report their ValueType for validation.
pub trait SelectorTypeProvider {
    /// Gets the ValueType for this selector
    fn get_value_type(&self) -> Result<ValueType, String>;
}

/// Mock implementation for testing
#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockSelector {
        value_type: ValueType,
    }
    
    impl MockSelector {
        fn new(value_type: ValueType) -> Self {
            Self { value_type }
        }
    }
    
    impl SelectorTypeProvider for MockSelector {
        fn get_value_type(&self) -> Result<ValueType, String> {
            Ok(self.value_type)
        }
    }
    
    struct MockConfig {
        weight_selector: MockSelector,
    }
    
    impl MockConfig {
        fn new(weight_type: ValueType) -> Self {
            Self {
                weight_selector: MockSelector::new(weight_type),
            }
        }
    }
    
    #[test]
    fn test_selector_validation() {
        let config = MockConfig::new(ValueType::Double);
        
        // This would test the validation logic
        // For now, just verify the mock works
        assert_eq!(config.weight_selector.get_value_type().unwrap(), ValueType::Double);
    }
}