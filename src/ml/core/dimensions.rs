//! Dimensions utilities for ML tensors in GDS.
//!
//! Translated from Java GDS ml-core Dimensions.java.
//! This is a literal 1:1 translation following repository translation policy.

/// Index constants for dimension arrays
pub const ROWS_INDEX: usize = 0;
pub const COLUMNS_INDEX: usize = 1;

/// Create scalar dimensions [1]
pub fn scalar() -> Vec<usize> {
    vec![1]
}

/// Create vector dimensions [size]
pub fn vector(size: usize) -> Vec<usize> {
    vec![size]
}

/// Create matrix dimensions [rows, cols]
pub fn matrix(rows: usize, cols: usize) -> Vec<usize> {
    vec![rows, cols]
}

/// Check if dimensions represent a vector (at most one dimension larger than 1)
pub fn is_vector(dimensions: &[usize]) -> bool {
    let mut dim_larger_one = 0;
    for &dim in dimensions {
        if dim > 1 {
            dim_larger_one += 1;
        }
    }
    dim_larger_one <= 1
}

/// Check if dimensions represent a scalar (total size is 1)
pub fn is_scalar(dimensions: &[usize]) -> bool {
    total_size(dimensions) == 1
}

/// Calculate total size from dimensions
pub fn total_size(dimensions: &[usize]) -> usize {
    if dimensions.is_empty() {
        return 0;
    }
    dimensions.iter().product()
}

/// Render dimensions as human-readable string
pub fn render(dimensions: &[usize]) -> String {
    match dimensions.len() {
        0 => "Scalar".to_string(),
        1 => format!("Vector({})", dimensions[0]),
        2 => format!("Matrix({}, {})", dimensions[0], dimensions[1]),
        n => format!("{}-Dim Tensor: {:?}", n, dimensions),
    }
}
