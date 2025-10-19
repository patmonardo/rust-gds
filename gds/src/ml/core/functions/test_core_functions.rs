//! Standalone test for core ML functions to verify our updates work correctly.

#[cfg(test)]
mod core_function_tests {
    use crate::ml::core::computation_context::ComputationContext;
    use crate::ml::core::functions::{Constant, Weights};
    use crate::ml::core::tensor::{Matrix, Scalar, Vector, Tensor};
    use crate::ml::core::variable::Variable;

    #[test]
    fn test_constant_scalar_creation_and_apply() {
        let constant = Constant::scalar(42.0);
        assert_eq!(constant.dimensions(), &[1, 1]);
        assert!(!constant.require_gradient());
        
        let ctx = ComputationContext::new();
        let result = constant.apply(&ctx);
        let scalar = result
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");
        
        assert_eq!(scalar.value(), 42.0);
    }

    #[test]
    fn test_constant_vector_creation_and_apply() {
        let data = vec![1.0, 2.0, 3.0];
        let constant = Constant::vector(data.clone());
        assert_eq!(constant.dimensions(), &[3, 1]);
        assert!(!constant.require_gradient());
        
        let ctx = ComputationContext::new();
        let result = constant.apply(&ctx);
        let vector = result
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Expected Vector");
        
        assert_eq!(vector.data(), &data);
    }

    #[test]
    fn test_constant_matrix_creation_and_apply() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let constant = Constant::matrix(data.clone(), 2, 2);
        assert_eq!(constant.dimensions(), &[2, 2]);
        assert!(!constant.require_gradient());
        
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
    fn test_weights_matrix_creation_and_apply() {
        let weights = Weights::of_matrix(2, 3);
        assert_eq!(weights.dimensions(), &[2, 3]);
        assert!(weights.require_gradient());
        
        let ctx = ComputationContext::new();
        let result = weights.apply(&ctx);
        let matrix = result
            .as_any()
            .downcast_ref::<Matrix>()
            .expect("Expected Matrix");
        
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 3);
    }

    #[test]
    fn test_weights_vector_creation_and_apply() {
        let data = vec![1.0, 2.0, 3.0];
        let weights = Weights::of_vector(data.clone());
        assert_eq!(weights.dimensions(), &[3, 1]);
        assert!(weights.require_gradient());
        
        let ctx = ComputationContext::new();
        let result = weights.apply(&ctx);
        let vector = result
            .as_any()
            .downcast_ref::<Vector>()
            .expect("Expected Vector");
        
        assert_eq!(vector.data(), &data);
    }

    #[test]
    fn test_weights_scalar_creation_and_apply() {
        let weights = Weights::of_scalar(5.0);
        assert_eq!(weights.dimensions(), &[1, 1]);
        assert!(weights.require_gradient());
        
        let ctx = ComputationContext::new();
        let result = weights.apply(&ctx);
        let scalar = result
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");
        
        assert_eq!(scalar.value(), 5.0);
    }

    #[test]
    fn test_matrix_zeros_creation() {
        let matrix = Matrix::zeros(3, 4);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 4);
        assert_eq!(matrix.data(), &vec![0.0; 12]);
    }

    #[test]
    fn test_matrix_set_data_at_flat() {
        let mut matrix = Matrix::zeros(2, 2);
        matrix.set_data_at_flat(0, 1.0);
        matrix.set_data_at_flat(1, 2.0);
        matrix.set_data_at_flat(2, 3.0);
        matrix.set_data_at_flat(3, 4.0);
        
        assert_eq!(matrix.data(), &vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_tensor_add_operation() {
        let a = Scalar::new(1.5);
        let b = Scalar::new(2.5);
        let result = a.add(&b);
        let result_scalar = result.as_any().downcast_ref::<Scalar>().expect("Expected Scalar");
        assert_eq!(result_scalar.value(), 4.0);
    }

    #[test]
    fn test_tensor_scalar_multiply_operation() {
        let scalar = Scalar::new(3.0);
        let result = scalar.scalar_multiply(2.0);
        let result_scalar = result.as_any().downcast_ref::<Scalar>().expect("Expected Scalar");
        assert_eq!(result_scalar.value(), 6.0);
    }
}

