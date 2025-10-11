//! Form Processor (Transcendental / Pure Nama)
//!
//! The Form Processor is the canonical policy surface between compile-time schema
//! (PropertyDescriptor) and runtime conversion/validation. It centralizes:
//! - Checked u64→usize conversions (public boundary)
//! - Safe widening/coercion rules
//! - Functor mappings between Gross (PropertyValues) and Subtle (PrimitiveValues)
//! - PropertyDescriptor registry for runtime validation
//!
//! This module is intentionally small and self-contained so macros and generated
//! code can call a stable policy surface.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::RwLock;

use crate::types::ValueType;

/// Errors produced by the form processor helpers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormProcessorError {
    /// u64 value is too large to fit into a `usize` on this target
    IndexOverflow(u64),
    /// Type mismatch during conversion
    TypeMismatch { expected: ValueType, got: ValueType },
    /// Null value where non-null expected
    Null,
    /// Unsupported conversion
    Unsupported(String),
}

impl fmt::Display for FormProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormProcessorError::IndexOverflow(id) => {
                write!(
                    f,
                    "index {} cannot be represented as usize on this target",
                    id
                )
            }
            FormProcessorError::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {:?}, got {:?}", expected, got)
            }
            FormProcessorError::Null => write!(f, "null value"),
            FormProcessorError::Unsupported(msg) => write!(f, "unsupported conversion: {}", msg),
        }
    }
}

impl Error for FormProcessorError {}

/// Convert a public `u64` id into an internal `usize` index in a fallible,
/// target-portable way. Returns `Err(FormProcessorError::IndexOverflow)` when
/// the conversion is not possible (e.g., on 32-bit targets where the u64 is
/// larger than usize::MAX).
pub fn checked_u64_to_usize(id: u64) -> Result<usize, FormProcessorError> {
    usize::try_from(id).map_err(|_| FormProcessorError::IndexOverflow(id))
}

/// Safe widening cast from i32 -> i64. Kept as a helper to centralize policy
/// in case we later want to instrument or validate conversions.
#[inline]
pub fn widen_i32_to_i64(v: i32) -> i64 {
    v as i64
}

/// Safe widening cast from f32 -> f64.
#[inline]
pub fn widen_f32_to_f64(v: f32) -> f64 {
    v as f64
}

// ============================================================================
// PropertyDescriptor Registry (runtime storage)
// ============================================================================

lazy_static::lazy_static! {
    static ref PROPERTY_REGISTRY: RwLock<HashMap<u32, super::property_descriptor::PropertyDescriptor>> =
        RwLock::new(HashMap::new());
}

// Test-only global lock to serialize registry operations and avoid races
// when the test harness runs tests in parallel. This is only compiled in
// test builds and does not affect production code.
#[cfg(test)]
lazy_static::lazy_static! {
    static ref PROPERTY_REGISTRY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

/// Register a PropertyDescriptor at runtime (called by macro-generated init code).
/// Returns true if newly registered, false if already present.
pub fn register_property_descriptor(desc: super::property_descriptor::PropertyDescriptor) -> bool {
    #[cfg(test)]
    let _guard = PROPERTY_REGISTRY_TEST_LOCK.lock().unwrap();
    use std::collections::hash_map::Entry;
    let mut registry = PROPERTY_REGISTRY.write().unwrap();
    match registry.entry(desc.id) {
        Entry::Vacant(e) => {
            e.insert(desc);
            true
        }
        Entry::Occupied(_) => false,
    }
}

/// Lookup a PropertyDescriptor by id.
pub fn get_property_descriptor(id: u32) -> Option<super::property_descriptor::PropertyDescriptor> {
    #[cfg(test)]
    let _guard = PROPERTY_REGISTRY_TEST_LOCK.lock().unwrap();
    let registry = PROPERTY_REGISTRY.read().unwrap();
    registry.get(&id).cloned()
}

/// Clear the registry (useful for tests).
#[cfg(test)]
pub fn clear_property_registry() {
    #[cfg(test)]
    let _guard = PROPERTY_REGISTRY_TEST_LOCK.lock().unwrap();
    let mut registry = PROPERTY_REGISTRY.write().unwrap();
    registry.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_u64_to_usize_ok() {
        // small value should convert
        let v: u64 = 42;
        let r = checked_u64_to_usize(v).unwrap();
        assert_eq!(r, 42usize);
    }

    #[test]
    fn widen_ints_and_floats() {
        assert_eq!(widen_i32_to_i64(123), 123i64);
        assert_eq!(widen_f32_to_f64(3.5f32), 3.5f64);
    }

    #[test]
    fn test_registry() {
        use crate::projection::property_descriptor::PropertyDescriptor;
        use crate::types::ValueType;

        clear_property_registry();

        let desc = PropertyDescriptor::new(42, "test", ValueType::Long);
        assert!(register_property_descriptor(desc.clone()));
        assert!(!register_property_descriptor(desc.clone())); // already present

        let retrieved = get_property_descriptor(42).unwrap();
        assert_eq!(retrieved.id, 42);
        assert_eq!(retrieved.name, "test");
    }
}
