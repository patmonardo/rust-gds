//! Eval Macro DSL — The Master Projector
//!
//! This module contains the `value_type_table!` macro — the single source of truth
//! that projects compile-time schema into both Gross and Subtle worlds:
//!
//! - Gross: PropertyValues adapters, ArrayBackend bindings, storage hints
//! - Subtle: GdsValue impls, PrimitiveValues factory arms, runtime accessors
//!
//! The macro emits:
//! - PropertyDescriptor constants
//! - Registration functions
//! - Functor implementations (Gross ↔ Subtle)
//! - Typed accessor methods
//! - Storage adapter wiring
//!
//! Philosophy: The Eval macro IS the Form (Noumenal schema) that projects into
//! both Phenomenal appearances (Gross storage + Subtle runtime). The Form Processor
//! (Transcendental/Pure Nama) enforces policy at the boundary.

/// The Master Projector Macro: value_type_table!
///
/// Syntax:
/// ```ignore
/// value_type_table! {
///     TypeName {
///         id: <PropertyId>,
///         value_type: <ValueType expr>,
///         storage_hint: <StorageHint>,
///         rust_type: <Rust type>,
///         gross_adapter: <PropertyValues impl>,
///         subtle_impl: <GdsValue impl>,
///     },
///     // ... more entries
/// }
/// ```
///
/// Each entry generates:
/// - A module `mod <TypeName>` containing:
///   - `DESCRIPTOR: PropertyDescriptor`
///   - `register() -> bool`
///   - `Functor` struct implementing GrossSubtleFunctor
///   - Typed accessor helpers
#[macro_export]
macro_rules! value_type_table {
    (
        $(
            $type_name:ident {
                id: $id:expr,
                value_type: $value_type:expr,
                storage_hint: $storage_hint:expr,
                rust_type: $rust_type:ty,
                $(gross_adapter: $gross_adapter:ty,)?
                $(subtle_impl: $subtle_impl:ty,)?
            }
        ),* $(,)?
    ) => {
        $(
            /// Generated module for the property type
            #[allow(non_snake_case)]
            pub mod $type_name {
                use $crate::projection::codegen::descriptors::{PropertyDescriptor, StorageHint};
                use $crate::projection::eval::form_processor;
                use $crate::projection::codegen::functors::{GrossToSubtle, SubtleToGross};
                use $crate::types::ValueType;
                use $crate::values::traits::GdsValue;
                use std::sync::Arc;

                lazy_static::lazy_static! {
                    /// Compile-time property descriptor
                    pub static ref DESCRIPTOR: PropertyDescriptor = PropertyDescriptor {
                        id: $id,
                        name: stringify!($type_name).to_string(),
                        value_type: $value_type,
                        nullable: true,
                        storage_hint: $storage_hint,
                    };
                }

                /// Register this property descriptor with the Form Processor.
                /// Returns true if newly registered, false if already present.
                /// Call this during store initialization or macro-generated setup.
                pub fn register() -> bool {
                    form_processor::register_property_descriptor((*DESCRIPTOR).clone())
                }

                /// Functor for this property type (Gross ↔ Subtle conversions)
                pub struct Functor;

                impl SubtleToGross for Functor {
                    fn project_to_storage(
                        &self,
                        value: Option<Arc<dyn GdsValue>>,
                    ) -> Result<Option<Arc<dyn GdsValue>>, form_processor::FormProcessorError> {
                        // Default: identity conversion (macro can override)
                        // Real implementations will check value_type and perform
                        // canonical conversions via form_processor helpers
                        Ok(value)
                    }
                }

                impl GrossToSubtle for Functor {
                    fn project_to_runtime(
                        &self,
                        value: Option<Arc<dyn GdsValue>>,
                    ) -> Result<Option<Arc<dyn GdsValue>>, form_processor::FormProcessorError> {
                        // Default: identity conversion (macro can override)
                        Ok(value)
                    }
                }

                #[cfg(test)]
                mod tests {
                    use super::*;

                    #[test]
                    fn test_descriptor() {
                        assert_eq!(DESCRIPTOR.id, $id);
                        assert_eq!(DESCRIPTOR.value_type, $value_type);
                    }

                    #[test]
                    fn test_registration() {
                        form_processor::clear_property_registry();
                        assert!(register());
                        assert!(!register()); // already registered

                        let retrieved = form_processor::get_property_descriptor($id).unwrap();
                        assert_eq!(retrieved.id, $id);
                    }

                    #[test]
                    fn test_functor() {
                        let functor = Functor;
                        let result = functor.project_to_storage(None);
                        assert!(result.is_ok());

                        let result = functor.project_to_runtime(None);
                        assert!(result.is_ok());
                    }
                }
            }
        )*

        /// Registry of all generated property types
        pub mod registry {
            use super::*;

            /// Register all property descriptors defined in this table.
            /// Call once during initialization.
            pub fn register_all() -> usize {
                let mut count = 0;
                $(
                    if $type_name::register() {
                        count += 1;
                    }
                )*
                count
            }

            /// Get all property IDs defined in this table
            pub fn all_property_ids() -> Vec<u32> {
                vec![$($id),*]
            }
        }
    };
}

// Example usage (commented out, for documentation):
//
// value_type_table! {
//     Long {
//         id: 1,
//         value_type: ValueType::Long,
//         storage_hint: StorageHint::FixedWidth,
//         rust_type: i64,
//     },
//     Double {
//         id: 2,
//         value_type: ValueType::Double,
//         storage_hint: StorageHint::FixedWidth,
//         rust_type: f64,
//     },
//     StringProp {
//         id: 3,
//         value_type: ValueType::String,
//         storage_hint: StorageHint::VariableLength,
//         rust_type: String,
//     },
// }
