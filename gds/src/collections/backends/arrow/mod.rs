//! Arrow Collections: Arrow-compatible Collections with Compute Kernels
//!
//! This module provides Collections implementations that are compatible with
//! Apache Arrow and implement SIMD-accelerated compute kernels.

pub mod arrow_int_array;

pub use arrow_int_array::ArrowIntArray;
