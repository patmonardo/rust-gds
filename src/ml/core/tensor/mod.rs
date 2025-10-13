//! Tensor module for ML in GDS.
//!
//! Translated from Java GDS ml-core tensor package.

pub mod tensor;
pub mod scalar;
pub mod vector;
pub mod matrix;
pub mod operations;

pub use tensor::Tensor;
pub use scalar::Scalar;
pub use vector::Vector;
pub use matrix::Matrix;
