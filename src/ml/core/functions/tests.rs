//! Tests for ml/core/functions.
//!
//! Comprehensive tests for the most critical ML functions.

#[cfg(test)]
mod constant_tests {
    use crate::ml::core::computation_context::ComputationContext;
    use crate::ml::core::functions::Constant;
    use crate::ml::core::tensor::{Matrix, Scalar, Vector, Tensor};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_scalar_constant_creation() {
        let constant = Constant::scalar(42.0);
        assert_eq!(constant.dimensions(), &[1, 1]);
        assert!(!constant.require_gradient());
    }

    #[test]
    fn test_scalar_constant_apply() {
        let constant = Constant::scalar(42.0);
        let ctx = ComputationContext::new();

        let result = constant.apply(&ctx);
        let scalar = result
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");

        assert_eq!(scalar.value(), 42.0);
    }

    #[test]
    fn test_vector_constant_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let constant = Constant::vector(data.clone());

        assert_eq!(constant.dimensions(), &[5, 1]);
        assert!(!constant.require_gradient());
    }

    #[test]
    fn test_vector_constant_apply() {
        let data = vec![1.0, 2.0, 3.0];
        let constant = Constant::vector(data.clone());
        let ctx = ComputationContext::new();

        let result = constant.apply(&ctx);
        let vector = result
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Expected Vector");

        assert_eq!(vector.data(), &data);
    }

    #[test]
    fn test_matrix_constant_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let constant = Constant::matrix(data.clone(), 2, 3);

        assert_eq!(constant.dimensions(), &[2, 3]);
        assert!(!constant.require_gradient());
    }

    #[test]
    fn test_matrix_constant_apply() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let constant = Constant::matrix(data, 2, 2);
        let ctx = ComputationContext::new();

        let result = constant.apply(&ctx);
        let matrix = result
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix");

        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
    }

    #[test]
    fn test_constant_no_parents() {
        let constant = Constant::scalar(10.0);
        assert_eq!(constant.parents().len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_constant_gradient_panics() {
        let constant = Constant::scalar(10.0);
        let parent = Constant::scalar(5.0);
        let ctx = ComputationContext::new();

        // Should panic - constants don't have gradients
        let _ = constant.gradient(&parent, &ctx);
    }

    #[test]
    fn test_constant_size_estimation() {
        // Scalar: 1 element
        assert_eq!(Constant::size_in_bytes(&[1, 1]), std::mem::size_of::<f64>());

        // Vector: 100 elements
        assert_eq!(
            Constant::size_in_bytes(&[100, 1]),
            100 * std::mem::size_of::<f64>()
        );

        // Matrix: 50x20 = 1000 elements
        assert_eq!(
            Constant::size_in_bytes(&[50, 20]),
            1000 * std::mem::size_of::<f64>()
        );
    }
}

#[cfg(test)]
mod weights_tests {
    use crate::ml::core::computation_context::ComputationContext;
    use crate::ml::core::functions::Weights;
    use crate::ml::core::tensor::{Matrix, Scalar, Vector, Tensor};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_matrix_weights_creation() {
        let weights = Weights::of_matrix(3, 4);
        assert_eq!(weights.dimensions(), &[3, 4]);
        assert!(weights.require_gradient());
    }

    #[test]
    fn test_matrix_weights_apply() {
        let weights = Weights::of_matrix(2, 2);
        let ctx = ComputationContext::new();

        let result = weights.apply(&ctx);
        let matrix = result
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix");

        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
    }

    #[test]
    fn test_vector_weights_creation() {
        let data = vec![1.0, 2.0, 3.0];
        let weights = Weights::of_vector(data.clone());

        assert_eq!(weights.dimensions(), &[3, 1]);
        assert!(weights.require_gradient());
    }

    #[test]
    fn test_vector_weights_apply() {
        let data = vec![1.0, 2.0, 3.0];
        let weights = Weights::of_vector(data.clone());
        let ctx = ComputationContext::new();

        let result = weights.apply(&ctx);
        let vector = result
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Expected Vector");

        assert_eq!(vector.data(), &data);
    }

    #[test]
    fn test_scalar_weights_creation() {
        let weights = Weights::of_scalar(5.0);
        assert_eq!(weights.dimensions(), &[1, 1]);
        assert!(weights.require_gradient());
    }

    #[test]
    fn test_scalar_weights_apply() {
        let weights = Weights::of_scalar(42.0);
        let ctx = ComputationContext::new();

        let result = weights.apply(&ctx);
        let scalar = result
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");

        assert_eq!(scalar.value(), 42.0);
    }

    #[test]
    fn test_weights_require_gradient() {
        let weights = Weights::of_matrix(2, 2);
        assert!(weights.require_gradient());
    }

    #[test]
    fn test_weights_no_parents() {
        let weights = Weights::of_vector(vec![1.0, 2.0]);
        assert_eq!(weights.parents().len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_weights_gradient_panics() {
        let weights = Weights::of_scalar(1.0);
        let parent = Weights::of_scalar(2.0);
        let ctx = ComputationContext::new();

        // Should panic - weights are leaf variables
        let _ = weights.gradient(&parent, &ctx);
    }

    #[test]
    fn test_weights_size_estimation() {
        assert_eq!(
            Weights::size_in_bytes(10, 20),
            crate::ml::core::tensor::size_in_bytes(&[10, 20])
        );
    }
}

#[cfg(test)]
mod sigmoid_tests {
    use crate::ml::core::functions::{Constant, Sigmoid};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_sigmoid_function_zero() {
        assert_eq!(Sigmoid::sigmoid(0.0), 0.5);
    }

    #[test]
    fn test_sigmoid_function_positive() {
        let result = Sigmoid::sigmoid(2.0);
        assert!(result > 0.5 && result < 1.0);
        assert!((result - 0.8807970779778823).abs() < 1e-10);
    }

    #[test]
    fn test_sigmoid_function_negative() {
        let result = Sigmoid::sigmoid(-2.0);
        assert!(result > 0.0 && result < 0.5);
        assert!((result - 0.11920292202211755).abs() < 1e-10);
    }

    #[test]
    fn test_sigmoid_function_large_positive() {
        let result = Sigmoid::sigmoid(10.0);
        assert!(result > 0.99);
        assert!(result < 1.0);
    }

    #[test]
    fn test_sigmoid_function_large_negative() {
        let result = Sigmoid::sigmoid(-10.0);
        assert!(result > 0.0);
        assert!(result < 0.01);
    }

    #[test]
    fn test_sigmoid_creation() {
        let parent = Box::new(Constant::scalar(0.5));
        let sigmoid = Sigmoid::new(parent);

        assert_eq!(sigmoid.dimensions(), &[1, 1]);
        // Constant parent doesn't require gradient, so sigmoid won't either
        assert!(!sigmoid.require_gradient());
    }

    #[test]
    fn test_sigmoid_no_gradient_propagation() {
        let parent = Box::new(Constant::scalar(0.5));
        let sigmoid = Sigmoid::new(parent);

        // Constant doesn't require gradient, so sigmoid shouldn't either
        assert!(!sigmoid.require_gradient());
    }

    #[test]
    fn test_sigmoid_dimensions_preserved() {
        let parent = Box::new(Constant::matrix(vec![1.0; 12], 3, 4));
        let sigmoid = Sigmoid::new(parent);

        assert_eq!(sigmoid.dimensions(), &[3, 4]);
    }

    #[test]
    fn test_sigmoid_size_estimation() {
        assert_eq!(
            Sigmoid::size_in_bytes(10, 20),
            crate::ml::core::tensor::size_in_bytes(&[10, 20])
        );
    }
}

#[cfg(test)]
mod mean_square_error_tests {
    use crate::ml::core::functions::{Constant, MeanSquareError};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_mse_creation() {
        let predictions = Box::new(Constant::vector(vec![1.0, 2.0, 3.0]));
        let targets = Box::new(Constant::vector(vec![1.0, 2.0, 3.0]));

        let mse = MeanSquareError::new(predictions, targets);

        assert_eq!(mse.dimensions(), &[1, 1]);
        assert!(!mse.require_gradient());
    }

    #[test]
    fn test_mse_with_gradient_requirement() {
        let predictions = Box::new(Constant::vector(vec![1.0, 2.0]));
        let targets = Box::new(Constant::vector(vec![1.0, 2.0]));

        let mse = MeanSquareError::new(predictions, targets);

        // Both parents are constants, so no gradient required
        assert!(!mse.require_gradient());
    }

    #[test]
    fn test_mse_parents() {
        let predictions = Box::new(Constant::vector(vec![1.0, 2.0]));
        let targets = Box::new(Constant::vector(vec![1.0, 2.0]));

        let mse = MeanSquareError::new(predictions, targets);

        assert_eq!(mse.parents().len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_mse_dimension_mismatch_panics() {
        let predictions = Box::new(Constant::vector(vec![1.0, 2.0, 3.0]));
        let targets = Box::new(Constant::vector(vec![1.0, 2.0]));

        // Should panic - dimensions don't match
        let _ = MeanSquareError::new(predictions, targets);
    }

    #[test]
    fn test_mse_scalar_output() {
        let predictions = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));
        let targets = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));

        let mse = MeanSquareError::new(predictions, targets);

        // MSE always outputs a scalar
        assert_eq!(mse.dimensions(), &[1, 1]);
    }
}

#[cfg(test)]
mod matrix_sum_tests {
    use crate::ml::core::functions::{Constant, MatrixSum};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_matrix_sum_single_parent() {
        let parent = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));
        let sum = MatrixSum::new(vec![parent]);

        assert_eq!(sum.dimensions(), &[2, 3]);
        assert!(!sum.require_gradient());
    }

    #[test]
    fn test_matrix_sum_multiple_parents() {
        let parent1 = Box::new(Constant::matrix(vec![1.0; 4], 2, 2));
        let parent2 = Box::new(Constant::matrix(vec![2.0; 4], 2, 2));
        let parent3 = Box::new(Constant::matrix(vec![3.0; 4], 2, 2));

        let sum = MatrixSum::new(vec![parent1, parent2, parent3]);

        assert_eq!(sum.dimensions(), &[2, 2]);
        assert_eq!(sum.parents().len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_matrix_sum_empty_parents_panics() {
        // Should panic - no parents provided
        let _ = MatrixSum::new(vec![]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_sum_dimension_mismatch_panics() {
        let parent1 = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));
        let parent2 = Box::new(Constant::matrix(vec![1.0; 4], 2, 2));

        // Should panic - dimensions don't match
        let _ = MatrixSum::new(vec![parent1, parent2]);
    }

    #[test]
    fn test_matrix_sum_dimensions_preserved() {
        let parent1 = Box::new(Constant::matrix(vec![1.0; 12], 3, 4));
        let parent2 = Box::new(Constant::matrix(vec![2.0; 12], 3, 4));

        let sum = MatrixSum::new(vec![parent1, parent2]);

        assert_eq!(sum.dimensions(), &[3, 4]);
    }
}

#[cfg(test)]
mod element_sum_tests {
    use crate::ml::core::functions::{Constant, ElementSum};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_element_sum_single_parent() {
        let parent = Box::new(Constant::vector(vec![1.0, 2.0, 3.0]));
        let sum = ElementSum::new(vec![parent]);

        assert_eq!(sum.dimensions(), &[1, 1]);
        assert!(!sum.require_gradient());
    }

    #[test]
    fn test_element_sum_multiple_parents() {
        let parent1 = Box::new(Constant::scalar(1.0));
        let parent2 = Box::new(Constant::scalar(2.0));
        let parent3 = Box::new(Constant::scalar(3.0));

        let sum = ElementSum::new(vec![parent1, parent2, parent3]);

        assert_eq!(sum.dimensions(), &[1, 1]);
        assert_eq!(sum.parents().len(), 3);
    }

    #[test]
    fn test_element_sum_scalar_output() {
        let parent1 = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));
        let parent2 = Box::new(Constant::vector(vec![1.0; 5]));

        let sum = ElementSum::new(vec![parent1, parent2]);

        // ElementSum always outputs a scalar
        assert_eq!(sum.dimensions(), &[1, 1]);
    }
}

#[cfg(test)]
mod l2_norm_squared_tests {
    use crate::ml::core::functions::{Constant, L2NormSquared};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_l2_norm_squared_creation() {
        let parent = Box::new(Constant::vector(vec![3.0, 4.0]));
        let l2_norm = L2NormSquared::new(parent);

        assert_eq!(l2_norm.dimensions(), &[1, 1]);
        assert!(!l2_norm.require_gradient());
    }

    #[test]
    fn test_l2_norm_squared_scalar_output() {
        let parent = Box::new(Constant::matrix(vec![1.0; 12], 3, 4));
        let l2_norm = L2NormSquared::new(parent);

        // L2NormSquared always outputs a scalar
        assert_eq!(l2_norm.dimensions(), &[1, 1]);
    }

    #[test]
    fn test_l2_norm_squared_memory_estimation() {
        assert_eq!(
            L2NormSquared::size_in_bytes_of_apply(),
            crate::ml::core::tensor::size_in_bytes(&[1])
        );
    }
}

#[cfg(test)]
mod relu_tests {
    use crate::ml::core::functions::{Constant, Relu};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_relu_creation_with_alpha() {
        let parent = Box::new(Constant::scalar(1.0));
        let relu = Relu::new(parent, 0.01);

        assert_eq!(relu.dimensions(), &[1, 1]);
        assert!(!relu.require_gradient());
    }

    #[test]
    fn test_relu_creation_default_alpha() {
        let parent = Box::new(Constant::scalar(1.0));
        let relu = Relu::with_default_alpha(parent);

        assert_eq!(relu.dimensions(), &[1, 1]);
    }

    #[test]
    fn test_relu_dimensions_preserved() {
        let parent = Box::new(Constant::matrix(vec![1.0; 12], 3, 4));
        let relu = Relu::new(parent, 0.01);

        assert_eq!(relu.dimensions(), &[3, 4]);
    }
}

#[cfg(test)]
mod constant_scale_tests {
    use crate::ml::core::functions::{Constant, ConstantScale};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_constant_scale_creation() {
        let parent = Box::new(Constant::scalar(5.0));
        let scaled = ConstantScale::new(parent, 2.0);

        assert_eq!(scaled.dimensions(), &[1, 1]);
        assert!(!scaled.require_gradient());
    }

    #[test]
    fn test_constant_scale_dimensions_preserved() {
        let parent = Box::new(Constant::matrix(vec![1.0; 6], 2, 3));
        let scaled = ConstantScale::new(parent, 3.0);

        assert_eq!(scaled.dimensions(), &[2, 3]);
    }

    #[test]
    fn test_constant_scale_parents() {
        let parent = Box::new(Constant::vector(vec![1.0, 2.0]));
        let scaled = ConstantScale::new(parent, 2.0);

        assert_eq!(scaled.parents().len(), 1);
    }
}
