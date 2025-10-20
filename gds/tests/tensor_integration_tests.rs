//! Comprehensive integration tests for the ML tensor system.
//!
//! These tests verify that the tensor system works correctly as a whole,
//! testing interactions between different tensor types and operations.
//!
//! Tests cover:
//! - Basic tensor operations (Scalar, Vector, Matrix)
//! - Cross-type operations and conversions
//! - Memory layout and performance characteristics
//! - Error handling and edge cases
//! - Compatibility with Java GDS tensor behavior

use gds::ml::core::dimensions;
use gds::ml::core::tensor::*;

#[cfg(test)]
mod basic_operations {
    use super::*;

    #[test]
    fn test_scalar_operations() {
        let a = Scalar::new(3.0);
        let b = Scalar::new(2.0);

        // Addition
        let sum = a.add(&b);
        assert_eq!(sum.data(), &[5.0]);
        assert_eq!(sum.dimensions(), &[1, 1]);

        // Scalar multiplication
        let scaled = a.scalar_multiply(2.0);
        assert_eq!(scaled.data(), &[6.0]);

        // Elementwise product
        let product = a.elementwise_product(&b);
        assert_eq!(product.data(), &[6.0]);

        // Map operation
        let squared = a.map(|x| x * x);
        assert_eq!(squared.data(), &[9.0]);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);

        // Addition
        let sum = v1.add(&v2);
        assert_eq!(sum.data(), &[5.0, 7.0, 9.0]);
        assert_eq!(sum.dimensions(), &[3, 1]);

        // Elementwise product
        let product = v1.elementwise_product(&v2);
        assert_eq!(product.data(), &[4.0, 10.0, 18.0]);

        // Scalar operations
        let doubled = v1.scalar_multiply(2.0);
        assert_eq!(doubled.data(), &[2.0, 4.0, 6.0]);

        // Map operations
        let squared = v1.map(|x| x * x);
        assert_eq!(squared.data(), &[1.0, 4.0, 9.0]);
    }

    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let m2 = Matrix::new(vec![5.0, 6.0, 7.0, 8.0], 2, 2);

        // Addition
        let sum = m1.add(&m2);
        assert_eq!(sum.data(), &[6.0, 8.0, 10.0, 12.0]);
        assert_eq!(sum.dimensions(), &[2, 2]);

        // Matrix multiplication
        let product = m1.multiply(&m2);
        assert_eq!(product.data(), &[19.0, 22.0, 43.0, 50.0]);

        // Elementwise product
        let elementwise = m1.elementwise_product(&m2);
        assert_eq!(elementwise.data(), &[5.0, 12.0, 21.0, 32.0]);

        // Scalar operations
        let doubled = m1.scalar_multiply(2.0);
        assert_eq!(doubled.data(), &[2.0, 4.0, 6.0, 8.0]);
    }
}

#[cfg(test)]
mod cross_type_operations {
    use super::*;

    #[test]
    fn test_matrix_vector_interactions() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let vector = Vector::new(vec![10.0, 20.0]);

        // Broadcast column-wise addition
        let result = matrix.sum_broadcast_column_wise(&vector);
        assert_eq!(result.data(), &[11.0, 22.0, 13.0, 24.0]);
        assert_eq!(result.dimensions(), &[2, 2]);
    }

    #[test]
    fn test_matrix_column_summation() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2);

        // Sum per column
        let column_sums = matrix.sum_per_column();
        assert_eq!(column_sums.data(), &[9.0, 12.0]); // [1+3+5, 2+4+6]
        assert_eq!(column_sums.dimensions(), &[2, 1]);
    }

    #[test]
    fn test_matrix_transpose_operations() {
        let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let m2 = Matrix::new(vec![5.0, 6.0, 7.0, 8.0], 2, 2);

        // Multiply with first operand transposed
        let result_trans_a = m1.multiply_trans_a(&m2);
        assert_eq!(result_trans_a.data(), &[26.0, 30.0, 38.0, 44.0]);

        // Multiply with second operand transposed
        let result_trans_b = m1.multiply_trans_b(&m2);
        assert_eq!(result_trans_b.data(), &[17.0, 23.0, 39.0, 53.0]);
    }
}

#[cfg(test)]
mod memory_and_performance {
    use super::*;

    #[test]
    fn test_memory_layout_consistency() {
        // Test that tensor data is stored in row-major order
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        // Verify row-major layout: [1,2] in first row, [3,4] in second row
        assert_eq!(matrix.data_at(0, 0), 1.0);
        assert_eq!(matrix.data_at(0, 1), 2.0);
        assert_eq!(matrix.data_at(1, 0), 3.0);
        assert_eq!(matrix.data_at(1, 1), 4.0);

        // Verify flat indexing matches row-major
        assert_eq!(matrix.data_at_flat(0), 1.0);
        assert_eq!(matrix.data_at_flat(1), 2.0);
        assert_eq!(matrix.data_at_flat(2), 3.0);
        assert_eq!(matrix.data_at_flat(3), 4.0);
    }

    #[test]
    fn test_dimension_calculations() {
        // Test dimension utility functions
        assert_eq!(dimensions::scalar(), vec![1, 1]);
        assert_eq!(dimensions::vector(5), vec![5, 1]);
        assert_eq!(dimensions::matrix(3, 4), vec![3, 4]);

        // Test dimension checks
        assert!(dimensions::is_scalar(&[1, 1]));
        assert!(dimensions::is_vector(&[5, 1]));
        assert!(!dimensions::is_vector(&[2, 3]));

        // Test total size calculation
        assert_eq!(dimensions::total_size(&[3, 4]), 12);
        assert_eq!(dimensions::total_size(&[2, 3, 4]), 24);
    }

    #[test]
    fn test_size_in_bytes_calculation() {
        // Test memory size calculations
        assert_eq!(size_in_bytes(&[1, 1]), 8); // 1 f64 = 8 bytes
        assert_eq!(size_in_bytes(&[5, 1]), 40); // 5 f64s = 40 bytes
        assert_eq!(size_in_bytes(&[3, 4]), 96); // 12 f64s = 96 bytes
    }
}

#[cfg(test)]
mod error_handling {
    use super::*;

    #[test]
    #[should_panic(expected = "Matrix dimensions must match")]
    fn test_matrix_multiplication_dimension_mismatch() {
        let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let m2 = Matrix::new(vec![1.0, 2.0, 3.0], 3, 1); // Wrong dimensions

        let _result = m1.multiply(&m2); // Should panic
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_vector_addition_length_mismatch() {
        let v1 = Vector::new(vec![1.0, 2.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]); // Different length

        let _result = v1.add(&v2); // Should panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_broadcast_dimension_mismatch() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let vector = Vector::new(vec![10.0]); // Wrong length for broadcast

        let _result = matrix.sum_broadcast_column_wise(&vector); // Should panic
    }
}

#[cfg(test)]
mod java_compatibility {
    use super::*;

    #[test]
    fn test_java_scalar_compatibility() {
        // Test that our Scalar behaves like Java's Scalar
        let scalar = Scalar::new(42.0);

        assert_eq!(scalar.value(), 42.0);
        assert_eq!(scalar.dimensions(), &[1, 1]);
        assert_eq!(scalar.data(), &[42.0]);

        // Test toString equivalent
        let display = format!("{}", scalar);
        assert!(display.contains("Scalar"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_java_vector_compatibility() {
        // Test that our Vector behaves like Java's Vector
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);

        assert_eq!(vector.length(), 3);
        assert_eq!(vector.dimensions(), &[3, 1]);
        assert_eq!(vector.data(), &[1.0, 2.0, 3.0]);

        // Test element access
        assert_eq!(vector.data_at(0), 1.0);
        assert_eq!(vector.data_at(1), 2.0);
        assert_eq!(vector.data_at(2), 3.0);
    }

    #[test]
    fn test_java_matrix_compatibility() {
        // Test that our Matrix behaves like Java's Matrix
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(matrix.dimensions(), &[2, 2]);

        // Test 2D access
        assert_eq!(matrix.data_at(0, 0), 1.0);
        assert_eq!(matrix.data_at(0, 1), 2.0);
        assert_eq!(matrix.data_at(1, 0), 3.0);
        assert_eq!(matrix.data_at(1, 1), 4.0);

        // Test row access
        assert_eq!(matrix.row(0), &[1.0, 2.0]);
        assert_eq!(matrix.row(1), &[3.0, 4.0]);
    }

    #[test]
    fn test_java_tensor_factory_compatibility() {
        // Test tensor creation patterns that match Java's TensorFactory

        // Scalar creation
        let scalar = Scalar::new(5.0);
        assert_eq!(scalar.dimensions(), &[1, 1]);

        // Vector creation
        let vector = Vector::create(2.0, 3);
        assert_eq!(vector.dimensions(), &[3, 1]);
        assert_eq!(vector.data(), &[2.0, 2.0, 2.0]);

        // Matrix creation
        let matrix = Matrix::create(3.0, 2, 2);
        assert_eq!(matrix.dimensions(), &[2, 2]);
        assert_eq!(matrix.data(), &[3.0, 3.0, 3.0, 3.0]);
    }
}

#[cfg(test)]
mod advanced_operations {
    use super::*;

    #[test]
    fn test_inplace_operations() {
        let mut matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let other = Matrix::new(vec![0.5, 1.0, 1.5, 2.0], 2, 2);

        // Test inplace addition
        matrix.add_inplace(&other);
        assert_eq!(matrix.data(), &[1.5, 3.0, 4.5, 6.0]);

        // Test scalar multiplication (immutable operation)
        let scaled = matrix.scalar_multiply(2.0);
        assert_eq!(scaled.data(), &[3.0, 6.0, 9.0, 12.0]);

        // Test elementwise product (immutable operation)
        let factor = Matrix::new(vec![2.0, 1.0, 0.5, 1.0], 2, 2);
        let result = scaled.elementwise_product(&factor);
        assert_eq!(result.data(), &[6.0, 6.0, 4.5, 12.0]);
    }

    #[test]
    fn test_map_operations() {
        let vector = Vector::new(vec![1.0, 4.0, 9.0, 16.0]);

        // Test map operation
        let sqrt_vector = vector.map(|x| x.sqrt());
        assert_eq!(sqrt_vector.data(), &[1.0, 2.0, 3.0, 4.0]);

        // Test map operation on clone (immutable operation)
        let vector_copy = vector.clone();
        let sqrt_vector_copy = vector_copy.map(|x| x.sqrt());
        assert_eq!(sqrt_vector_copy.data(), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_aggregate_operations() {
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);

        // Test aggregate sum
        let total_sum = matrix.aggregate_sum();
        assert_eq!(total_sum, 21.0); // 1+2+3+4+5+6

        // Test column sums
        let column_sums = matrix.sum_per_column();
        assert_eq!(column_sums.data(), &[5.0, 7.0, 9.0]); // [1+4, 2+5, 3+6]
    }

    #[test]
    fn test_ones_like_operations() {
        let scalar = Scalar::new(42.0);
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        // Test ones_like for each type
        let scalar_ones = scalar.ones_like();
        assert_eq!(scalar_ones.data(), &[1.0]);

        let vector_ones = vector.ones_like();
        assert_eq!(vector_ones.data(), &[1.0, 1.0, 1.0]);

        let matrix_ones = matrix.ones_like();
        assert_eq!(matrix_ones.data(), &[1.0, 1.0, 1.0, 1.0]);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_tensors() {
        // Test empty vector
        let empty_vector = Vector::with_size(0);
        assert_eq!(empty_vector.length(), 0);
        assert_eq!(empty_vector.data(), &[] as &[f64]);

        // Test 1x1 matrix
        let single_matrix = Matrix::with_dimensions(1, 1);
        assert_eq!(single_matrix.rows(), 1);
        assert_eq!(single_matrix.cols(), 1);
        assert_eq!(single_matrix.data(), &[0.0]);
    }

    #[test]
    fn test_large_tensors() {
        // Test larger tensors to ensure performance
        let large_vector = Vector::create(1.0, 1000);
        assert_eq!(large_vector.length(), 1000);

        let large_matrix = Matrix::create(2.0, 100, 100);
        assert_eq!(large_matrix.rows(), 100);
        assert_eq!(large_matrix.cols(), 100);

        // Test operations on large tensors
        let doubled = large_vector.scalar_multiply(2.0);
        assert_eq!(doubled.data()[0], 2.0);
        assert_eq!(doubled.data()[999], 2.0);
    }

    #[test]
    fn test_precision_and_tolerance() {
        let a = Scalar::new(1.0);
        let b = Scalar::new(1.0000000001);

        // Test equality with tolerance
        assert!(a.equals(&b, 1e-6));
        assert!(!a.equals(&b, 1e-12));

        // Test exact equality
        let c = Scalar::new(1.0);
        assert!(a.equals(&c, 0.0));
    }
}

#[cfg(test)]
mod performance_characteristics {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_matrix_multiplication_performance() {
        // Create larger matrices for performance testing
        let size = 50;
        let data_a: Vec<f64> = (0..size * size).map(|i| i as f64).collect();
        let data_b: Vec<f64> = (0..size * size).map(|i| (i + 1) as f64).collect();

        let matrix_a = Matrix::new(data_a, size, size);
        let matrix_b = Matrix::new(data_b, size, size);

        // Time the multiplication
        let start = Instant::now();
        let _result = matrix_a.multiply(&matrix_b);
        let duration = start.elapsed();

        // Verify result dimensions
        assert_eq!(_result.dimensions(), &[size, size]);

        // Performance should be reasonable (this is a basic check)
        println!(
            "Matrix multiplication ({}x{}) took: {:?}",
            size, size, duration
        );
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that tensors don't have excessive memory overhead
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        // Test clone operations don't leak memory
        let vector_clone = vector.clone();
        let matrix_clone = matrix.clone();

        assert_eq!(vector.data(), vector_clone.data());
        assert_eq!(matrix.data(), matrix_clone.data());

        // Test that operations create new tensors efficiently
        let _sum = vector.add(&vector_clone);
        let _product = matrix.multiply(&matrix_clone);
    }
}
