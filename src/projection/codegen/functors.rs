//! Functor traits for Gross ↔ Subtle conversions
//!
//! These traits define the canonical 1:1 mappings between:
//! - Gross: PropertyValues (storage-oriented, column-based, u64-indexed)
//! - Subtle: PrimitiveValues (runtime-oriented, GdsValue trait objects)
//!
//! The Form Processor implements these functors and the Eval macro generates
//! specific adapter implementations per ValueType.

use crate::projection::eval::FormProcessorError;
use crate::values::traits::GdsValue;
use std::sync::Arc;

/// Functor: Subtle → Gross (PrimitiveValue → PropertyValues backend write)
///
/// Converts a runtime GdsValue into a form suitable for storage in PropertyValues.
/// This is the "projection into storage" direction.
pub trait SubtleToGross {
    /// Convert a GdsValue into a storage representation.
    /// Returns an Arc<dyn GdsValue> that can be stored in PropertyValues,
    /// or an error if conversion is unsupported/invalid.
    fn project_to_storage(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

/// Functor: Gross → Subtle (PropertyValues backend read → PrimitiveValue)
///
/// Converts a storage representation into a runtime GdsValue.
/// This is the "projection into runtime" direction.
pub trait GrossToSubtle {
    /// Convert a storage value into a runtime GdsValue.
    /// Returns an Arc<dyn GdsValue> suitable for algorithm consumption,
    /// or an error if conversion is unsupported/invalid.
    fn project_to_runtime(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError>;
}

/// Bidirectional functor: Gross ↔ Subtle
///
/// A type that implements both directions of conversion.
/// Most generated adapters will implement this.
pub trait GrossSubtleFunctor: SubtleToGross + GrossToSubtle {}

// Blanket impl: any type implementing both directions is a functor
impl<T: SubtleToGross + GrossToSubtle> GrossSubtleFunctor for T {}

/// Identity functor (for types where Gross = Subtle, no conversion needed)
pub struct IdentityFunctor;

impl SubtleToGross for IdentityFunctor {
    fn project_to_storage(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError> {
        Ok(value)
    }
}

impl GrossToSubtle for IdentityFunctor {
    fn project_to_runtime(
        &self,
        value: Option<Arc<dyn GdsValue>>,
    ) -> Result<Option<Arc<dyn GdsValue>>, FormProcessorError> {
        Ok(value)
    }
}

/// Widening functor (e.g., i32 → i64, f32 → f64)
pub struct WideningFunctor<F, T> {
    _phantom: std::marker::PhantomData<(F, T)>,
}

impl<F, T> Default for WideningFunctor<F, T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

// Specific widening implementations will be macro-generated

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_functor() {
        let functor = IdentityFunctor;
        let value = None;

        let stored = functor.project_to_storage(value.clone()).unwrap();
        assert!(stored.is_none());

        let runtime = functor.project_to_runtime(value).unwrap();
        assert!(runtime.is_none());
    }
}
