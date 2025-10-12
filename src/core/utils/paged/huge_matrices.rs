/// Computes the index of `(x, y)` within an upper-triangular storage layout.
///
/// # Panics
///
/// Panics when coordinates are out of bounds or when `x > y`.
pub fn triangular_index(order: usize, x: usize, y: usize) -> usize {
    assert!(x <= y, "triangular index requires x <= y ({} > {})", x, y);
    assert!(
        x < order,
        "triangular index x {} out of bounds {}",
        x,
        order
    );
    assert!(
        y < order,
        "triangular index y {} out of bounds {}",
        y,
        order
    );

    let order_mul = x
        .checked_mul(order)
        .expect("triangular index overflow computing row offset");
    let diagonal_term = x
        .checked_mul(
            x.checked_add(1)
                .expect("triangular index overflow adding 1"),
        )
        .expect("triangular index overflow computing diagonal contribution")
        / 2;

    order_mul + y - diagonal_term
}

/// Returns the total number of elements stored for an `order` sized triangular matrix.
///
/// # Panics
///
/// Panics if the computed size overflows `usize`.
pub fn triangular_size(order: usize) -> usize {
    let order_plus_one = order
        .checked_add(1)
        .expect("triangular size overflow computing order + 1");
    let product = order
        .checked_mul(order_plus_one)
        .expect("triangular size overflow computing product");
    product / 2
}

/// Normalizes coordinates so that the first element is always the smaller index.
///
/// # Panics
///
/// Panics if coordinates are outside the matrix bounds.
pub fn normalize_triangular_coordinates(order: usize, x: usize, y: usize) -> (usize, usize) {
    assert!(x < order, "coordinate x {} out of bounds {}", x, order);
    assert!(y < order, "coordinate y {} out of bounds {}", y, order);

    if x <= y {
        (x, y)
    } else {
        (y, x)
    }
}

#[cfg(test)]
mod tests {
    use super::{triangular_index, triangular_size};

    #[test]
    fn triangular_index_matches_manual_values() {
        let order = 4;
        let expected = [[0, 1, 2, 3], [0, 4, 5, 6], [0, 0, 7, 8], [0, 0, 0, 9]];

        for i in 0..order {
            for j in i..order {
                assert_eq!(triangular_index(order, i, j), expected[i][j]);
            }
        }
    }

    #[test]
    fn triangular_size_computes_expected_value() {
        assert_eq!(triangular_size(0), 0);
        assert_eq!(triangular_size(1), 1);
        assert_eq!(triangular_size(2), 3);
        assert_eq!(triangular_size(3), 6);
        assert_eq!(triangular_size(10), 55);
    }
}
