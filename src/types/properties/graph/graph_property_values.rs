use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::any::Any;
use std::sync::Arc;

/// Graph-level property value interface mirroring the GDS API.
/// Provides typed iterator accessors and basic metadata. Concrete
/// implementations live under the `impls` module to mirror the
/// node and relationship layouts.
pub trait GraphPropertyValues: PropertyValues + std::fmt::Debug + Send + Sync {
    /// Returns an iterator over the values interpreted as doubles (if supported).
    fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_>;

    /// Returns an iterator over the values interpreted as longs (if supported).
    fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_>;

    /// Returns an iterator over double array values (if supported).
    fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_>;

    /// Returns an iterator over float array values (if supported).
    fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_>;

    /// Returns an iterator over long array values (if supported).
    fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_>;

    /// Returns an iterator over boxed objects for dynamic access.
    fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn Any>> + '_>;

    /// Number of logical values contained in this property container.
    fn value_count(&self) -> usize {
        self.element_count()
    }

    /// Returns true when at least one logical value is present.
    fn has_values(&self) -> bool {
        self.value_count() > 0
    }

    /// Optional dimension metadata for array-based values.
    fn dimension(&self) -> Option<usize> {
        None
    }
}

/// Graph property values comprised of scalar longs.
pub trait LongGraphPropertyValues: GraphPropertyValues {
    /// Returns the underlying long values without additional validation.
    fn long_values_unchecked(&self) -> &[i64];
}

/// Graph property values comprised of scalar doubles.
pub trait DoubleGraphPropertyValues: GraphPropertyValues {
    /// Returns the underlying double values without additional validation.
    fn double_values_unchecked(&self) -> &[f64];
}

/// Graph property values comprised of arrays of doubles.
pub trait DoubleArrayGraphPropertyValues: GraphPropertyValues {
    /// Returns the underlying double array values without additional validation.
    fn double_arrays_unchecked(&self) -> &[Vec<f64>];
}

/// Graph property values comprised of arrays of floats.
pub trait FloatArrayGraphPropertyValues: GraphPropertyValues {
    /// Returns the underlying float array values without additional validation.
    fn float_arrays_unchecked(&self) -> &[Vec<f32>];
}

/// Graph property values comprised of arrays of longs.
pub trait LongArrayGraphPropertyValues: GraphPropertyValues {
    /// Returns the underlying long array values without additional validation.
    fn long_arrays_unchecked(&self) -> &[Vec<i64>];
}

// Implement PropertyValues for trait objects so they can be stored generically.
impl PropertyValues for Box<dyn GraphPropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}

impl PropertyValues for Arc<dyn GraphPropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}
