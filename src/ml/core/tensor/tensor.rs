//! Tensor trait for ML computations.
//!
//! Translated from Java GDS ml-core Tensor.java.
//! This provides a trait object-safe interface for ML functions.

use std::fmt;

/// Trait for downcasting to Any for type checking.
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Core tensor trait that is object-safe for use in ML functions.
/// 
/// This trait provides the interface needed by ML functions while being
/// compatible with trait objects (Box<dyn Tensor>).
pub trait Tensor: fmt::Debug + fmt::Display + Send + Sync + AsAny {
    /// Get the dimensions of this tensor.
    /// Java: `public int[] dimensions()`
    fn dimensions(&self) -> &[usize];
    
    /// Get the raw data array.
    /// Java: `protected double[] data`
    fn data(&self) -> &[f64];
    
    /// Get value at flat index.
    /// Java: `public double dataAt(int idx)`
    fn data_at(&self, idx: usize) -> f64;
    
    /// Get a specific dimension by index.
    /// Java: `public int dimension(int dimensionIndex)`
    fn dimension(&self, dimension_index: usize) -> usize;
    
    /// Calculate total size (product of all dimensions).
    /// Java: `public int totalSize()`
    fn total_size(&self) -> usize {
        self.dimensions().iter().product()
    }
    
    /// Sum all elements in the tensor.
    /// Java: `public double aggregateSum()`
    fn aggregate_sum(&self) -> f64 {
        self.data().iter().sum()
    }
    
    /// Check equality with tolerance.
    /// Java: `public boolean equals(Tensor<?> other, double tolerance)`
    fn equals(&self, other: &dyn Tensor, tolerance: f64) -> bool;
    
    /// Get short description for display.
    /// Java: `protected abstract String shortDescription()`
    fn short_description(&self) -> String;
    
    /// Clone this tensor as a boxed trait object.
    fn clone_box(&self) -> Box<dyn Tensor>;
    
    /// Create a new tensor with the same dimensions but zero data.
    fn create_with_same_dimensions(&self) -> Box<dyn Tensor>;
    
    /// Add another tensor element-wise.
    fn add(&self, other: &dyn Tensor) -> Box<dyn Tensor>;
    
    /// Apply function to each element, return new tensor.
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor>;
    
    /// Multiply by scalar (immutable).
    fn scalar_multiply(&self, scalar: f64) -> Box<dyn Tensor>;
    
    /// Element-wise product (immutable).
    fn elementwise_product(&self, other: &dyn Tensor) -> Box<dyn Tensor>;
    
    /// Create a tensor filled with ones, same dimensions as this tensor.
    fn ones_like(&self) -> Box<dyn Tensor>;
    
    /// Add another tensor to this one in-place (mutable operation).
    /// This is used for gradient accumulation.
    fn add_inplace(&mut self, other: &dyn Tensor);
}

/// Helper function to calculate size in bytes.
/// Java: `public static long sizeInBytes(int[] dimensions)`
pub fn size_in_bytes(dimensions: &[usize]) -> usize {
    let total_elements: usize = dimensions.iter().product();
    total_elements * std::mem::size_of::<f64>()
}
