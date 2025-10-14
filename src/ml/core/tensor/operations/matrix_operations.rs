//! Matrix operations - translated from DoubleMatrixOperations.java

use crate::ml::core::tensor::Matrix;

/// Modified version of Ejml implementation for multTransB.
///
/// Multiplies matrix `a` by the transpose of matrix `b` and stores result in `c`.
/// The mask predicate determines which elements of `c` to compute.
///
/// Reference: Ejml MatrixMatrixMult_DDRM::multTransB
pub fn mult_trans_b<F>(a: &Matrix, b: &Matrix, c: &mut Matrix, mask: F)
where
    F: Fn(usize) -> bool,
{
    let rows_a = a.rows();
    let cols_a = a.cols();
    let rows_b = b.rows();
    let cols_b = b.cols();

    // Validation
    assert_ne!(
        std::ptr::eq(a, c),
        true,
        "'a' cannot be the same matrix as 'c'"
    );
    assert_ne!(
        std::ptr::eq(b, c),
        true,
        "'b' cannot be the same matrix as 'c'"
    );
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
        let end = a_index_start + cols_b;
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
