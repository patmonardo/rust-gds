//! Matrix operations - translated from DoubleMatrixOperations.java

use crate::ml::core::tensor::{Matrix, Tensor};

/// Modified version of Ejml implementation for multTransB.
///
/// Multiplies matrix `a` by the transpose of matrix `b` and stores result in `c`.
/// The mask predicate determines which elements of `c` to compute.
///
/// Reference: Ejml MatrixMatrixMult_DDRM::multTransB
pub fn mult_trans_b<F>(a: &Matrix, b: &Matrix, c: &mut Matrix, mut mask: F)
where
    F: FnMut(usize) -> bool,
{
    let rows_a = a.rows();
    let cols_a = a.cols();
    let rows_b = b.rows();
    let cols_b = b.cols();

    // Validation
    assert!(!std::ptr::eq(a, c), "'a' cannot be the same matrix as 'c'");
    assert!(!std::ptr::eq(b, c), "'b' cannot be the same matrix as 'c'");
    assert_eq!(
        cols_a, cols_b,
        "Matrices 'a' and 'b' must have compatible dimensions"
    );
    assert_eq!(
        c.rows(),
        rows_a,
        "Matrix 'c' does not have compatible dimensions"
    );
    assert_eq!(
        c.cols(),
        rows_b,
        "Matrix 'c' does not have compatible dimensions"
    );

    let mut a_index_start = 0;
    let mut c_index = 0;

    for _x_a in 0..rows_a {
        let end = a_index_start + cols_a;
        let mut index_b = 0;

        for _x_b in 0..rows_b {
            if mask(c_index) {
                let mut index_a = a_index_start;
                let mut total = 0.0;

                while index_a < end {
                    total += a.data()[index_a] * b.data()[index_b];
                    index_a += 1;
                    index_b += 1;
                }

                c.set_data_at_flat(c_index, total);
            } else {
                index_b += cols_b;
            }
            c_index += 1;
        }
        a_index_start += cols_a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult_trans_b_matches_full_product() {
        let a = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let b = Matrix::new(vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0], 2, 3);
        let mut c = Matrix::with_dimensions(2, 2);

        mult_trans_b(&a, &b, &mut c, |_| true);

        assert_eq!(c.data(), &[50.0, 68.0, 122.0, 167.0]);
    }

    #[test]
    fn mult_trans_b_respects_mask() {
        let a = Matrix::new(vec![1.0, 0.0, 0.0, 1.0], 2, 2);
        let b = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let mut c = Matrix::with_dimensions(2, 2);

        mult_trans_b(&a, &b, &mut c, |index| index % 2 == 0);

        assert_eq!(c.data(), &[1.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "compatible dimensions")]
    fn mult_trans_b_panics_on_dimension_mismatch() {
        let a = Matrix::with_dimensions(2, 3);
        let b = Matrix::with_dimensions(3, 4);
        let mut c = Matrix::with_dimensions(2, 3);

        mult_trans_b(&a, &b, &mut c, |_| true);
    }
}
