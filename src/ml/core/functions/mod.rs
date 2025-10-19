//! Functions module for ML in GDS.
//!
//! This module contains ML computation functions (neural network layers, loss functions, etc.)
//! Core abstractions like AbstractVariable are in the parent ml/core module.

// Base classes for functions
pub mod single_parent_variable;

// Constants and weights
pub mod constant;
pub mod lazy_constant;
pub mod weights;

// Basic operations
pub mod constant_scale;
pub mod element_sum;
pub mod element_wise_max;
pub mod ewise_add_matrix_scalar;

// Matrix operations
pub mod matrix_multiply_with_transposed_second_operand;
pub mod matrix_sum;
pub mod matrix_vector_sum;

// Activations
pub mod reduced_softmax;
pub mod relu;
pub mod sigmoid;
pub mod softmax;

// Normalization
pub mod normalize_rows;

// Loss functions
pub mod cross_entropy_loss;
pub mod focal_loss;
pub mod l2_norm_squared;
pub mod logistic_loss;
pub mod mean_square_error;
pub mod reduced_cross_entropy_loss;
pub mod reduced_focal_loss;
pub mod root_mean_square_error;

// Graph operations
pub mod multi_mean;
pub mod slice;

// Re-exports
// Note: AbstractVariable is exported from parent ml/core module, not here
pub use constant::*;
pub use constant_scale::*;
pub use cross_entropy_loss::*;
pub use element_sum::*;
pub use element_wise_max::*;
pub use ewise_add_matrix_scalar::*;
pub use focal_loss::*;
pub use l2_norm_squared::*;
pub use lazy_constant::*;
pub use logistic_loss::*;
pub use matrix_multiply_with_transposed_second_operand::*;
pub use matrix_sum::*;
pub use matrix_vector_sum::*;
pub use mean_square_error::*;
pub use multi_mean::*;
pub use normalize_rows::*;
pub use reduced_cross_entropy_loss::*;
pub use reduced_focal_loss::*;
pub use reduced_softmax::*;
pub use relu::*;
pub use root_mean_square_error::*;
pub use sigmoid::*;
pub use single_parent_variable::*;
pub use slice::*;
pub use softmax::*;
pub use weights::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_core_functions;
