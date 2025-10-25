//! Monadic PropertyValues Macro: Auto-generate typed property values
//!
//! This macro generates MonadicPropertyValues implementations for any type T
//! that can be stored in a Collections<T> backend.
//!
//! # Example
//!
//! ```rust
//! monadic_property_values!(MonadicLongPropertyValues => i64, ValueType::Long);
//! ```
//!
//! Generates:
//! - `MonadicLongPropertyValues<C: Collections<i64>>` struct
//! - `new()` and `values()` methods
//! - `PropertyValues` trait implementation

/// Generate a MonadicPropertyValues type for a given element type
#[macro_export]
macro_rules! monadic_property_values {
    ($name:ident => $element:ty, $value_type:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<C>
        where
            C: Collections<$element> + Debug + Clone,
        {
            values: C,
            value_type: ValueType,
            default_value: $element,
        }

        impl<C> $name<C>
        where
            C: Collections<$element> + Debug + Clone,
        {
            pub fn new(values: C, default_value: $element) -> Self {
                Self {
                    values,
                    value_type: $value_type,
                    default_value,
                }
            }

            pub fn values(&self) -> &C {
                &self.values
            }
        }

        impl<C> PropertyValues for $name<C>
        where
            C: Collections<$element> + Debug + Clone,
        {
            fn value_type(&self) -> ValueType {
                self.value_type
            }

            fn element_count(&self) -> usize {
                self.values.len()
            }
        }
    };
}

// Re-export for internal use
pub use monadic_property_values;

