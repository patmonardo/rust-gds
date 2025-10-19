use crate::collections::HugeLongArray;

use super::huge_matrices::{normalize_triangular_coordinates, triangular_index, triangular_size};
use super::HugeLongSquareMatrix;

/// Upper-triangular matrix backed by a `HugeLongArray`.
pub struct HugeLongTriangularMatrix {
    array: HugeLongArray,
    order: usize,
}

impl HugeLongTriangularMatrix {
    /// Creates a new triangular matrix with the given order.
    ///
    /// # Panics
    ///
    /// Panics if the computed storage size overflows.
    pub fn new(order: usize) -> Self {
        let size = triangular_size(order);
        Self {
            array: HugeLongArray::new(size),
            order,
        }
    }

    /// Sets the value at `(x, y)` (symmetrically for `(y, x)`).
    pub fn set(&mut self, x: usize, y: usize, value: i64) {
        let (row, col) = normalize_triangular_coordinates(self.order, x, y);
        let index = triangular_index(self.order, row, col);
        self.array.set(index, value);
    }

    /// Returns the value at `(x, y)` (order-independent).
    pub fn get(&self, x: usize, y: usize) -> i64 {
        let (row, col) = normalize_triangular_coordinates(self.order, x, y);
        let index = triangular_index(self.order, row, col);
        self.array.get(index)
    }

    /// Adds `delta` to the value stored at `(x, y)`.
    pub fn add_to(&mut self, x: usize, y: usize, delta: i64) {
        let (row, col) = normalize_triangular_coordinates(self.order, x, y);
        let index = triangular_index(self.order, row, col);
        self.array.add_to(index, delta);
    }

    /// Fills the matrix with `value`.
    pub fn fill(&mut self, value: i64) {
        self.array.fill(value);
    }

    /// Fills the diagonal with `value`.
    pub fn fill_diagonal(&mut self, value: i64) {
        for i in 0..self.order {
            let index = triangular_index(self.order, i, i);
            self.array.set(index, value);
        }
    }

    /// Returns the diagonal as a vector.
    pub fn diagonal(&self) -> Vec<i64> {
        (0..self.order).map(|i| self.get(i, i)).collect()
    }

    /// Overwrites the diagonal with the provided values.
    pub fn set_diagonal(&mut self, values: &[i64]) {
        assert_eq!(values.len(), self.order, "diagonal length mismatch");
        for (i, &value) in values.iter().enumerate() {
            let index = triangular_index(self.order, i, i);
            self.array.set(index, value);
        }
    }

    /// Sum of the diagonal elements.
    pub fn trace(&self) -> i64 {
        (0..self.order).map(|i| self.get(i, i)).sum()
    }

    /// Number of stored elements (N(N+1)/2).
    pub fn len(&self) -> usize {
        self.array.size()
    }

    /// Matrix order.
    pub fn order(&self) -> usize {
        self.order
    }

    /// Returns `true` when no elements are stored.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Counts the number of non-zero stored elements.
    pub fn count_non_zero(&self) -> usize {
        self.array.iter().filter(|value| *value != 0).count()
    }

    /// Sparsity ratio between 0 (dense) and 1 (empty).
    pub fn sparsity(&self) -> f64 {
        let total = self.len();
        if total == 0 {
            return 0.0;
        }
        let non_zero = self.count_non_zero();
        (total - non_zero) as f64 / total as f64
    }

    /// Expands the triangular storage into a full square matrix.
    pub fn to_square_matrix(&self) -> HugeLongSquareMatrix {
        let mut square = HugeLongSquareMatrix::new(self.order);
        for i in 0..self.order {
            for j in i..self.order {
                let value = self.get(i, j);
                square.set(i, j, value);
                if i != j {
                    square.set(j, i, value);
                }
            }
        }
        square
    }

    /// Iterator over all stored entries where `x <= y`.
    pub fn entries(&self) -> TriangularEntries<'_> {
        TriangularEntries {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    /// Iterator over diagonal entries `(i, value)`.
    pub fn diagonal_entries(&self) -> DiagonalEntries<'_> {
        DiagonalEntries {
            matrix: self,
            index: 0,
        }
    }

    /// Iterator over off-diagonal entries where `x < y`.
    pub fn off_diagonal_entries(&self) -> OffDiagonalEntries<'_> {
        OffDiagonalEntries {
            matrix: self,
            row: 0,
            col: 1,
        }
    }
}

pub struct TriangularEntries<'a> {
    matrix: &'a HugeLongTriangularMatrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for TriangularEntries<'a> {
    type Item = (usize, usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let order = self.matrix.order;
        if self.row >= order {
            return None;
        }

        if self.col >= order {
            self.row += 1;
            self.col = self.row;
            return self.next();
        }

        if self.col < self.row {
            self.col = self.row;
            return self.next();
        }

        let value = self.matrix.get(self.row, self.col);
        let result = (self.row, self.col, value);

        self.col += 1;
        if self.col >= order {
            self.row += 1;
            self.col = self.row;
        }

        Some(result)
    }
}

pub struct DiagonalEntries<'a> {
    matrix: &'a HugeLongTriangularMatrix,
    index: usize,
}

impl<'a> Iterator for DiagonalEntries<'a> {
    type Item = (usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.matrix.order {
            return None;
        }

        let i = self.index;
        self.index += 1;
        Some((i, self.matrix.get(i, i)))
    }
}

pub struct OffDiagonalEntries<'a> {
    matrix: &'a HugeLongTriangularMatrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for OffDiagonalEntries<'a> {
    type Item = (usize, usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let order = self.matrix.order;
        while self.row < order {
            if self.col >= order {
                self.row += 1;
                self.col = self.row + 1;
                continue;
            }

            if self.col <= self.row {
                self.col = self.row + 1;
                continue;
            }

            let value = self.matrix.get(self.row, self.col);
            let result = (self.row, self.col, value);
            self.col += 1;
            return Some(result);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HugeLongTriangularMatrix;

    #[test]
    fn set_get_is_symmetric() {
        let mut matrix = HugeLongTriangularMatrix::new(4);
        matrix.set(2, 0, 5);
        matrix.set(3, 1, -7);

        assert_eq!(matrix.get(0, 2), 5);
        assert_eq!(matrix.get(2, 0), 5);
        assert_eq!(matrix.get(3, 1), -7);
        assert_eq!(matrix.get(1, 3), -7);
    }

    #[test]
    fn add_to_accumulates() {
        let mut matrix = HugeLongTriangularMatrix::new(3);
        matrix.add_to(0, 1, 2);
        matrix.add_to(1, 0, 3);
        assert_eq!(matrix.get(0, 1), 5);
        assert_eq!(matrix.get(1, 0), 5);
    }

    #[test]
    fn diagonal_operations_work() {
        let mut matrix = HugeLongTriangularMatrix::new(5);
        matrix.fill_diagonal(4);
        assert_eq!(matrix.diagonal(), vec![4; 5]);

        matrix.set_diagonal(&[1, 2, 3, 4, 5]);
        assert_eq!(matrix.trace(), 15);
    }

    #[test]
    fn iterators_cover_expected_entries() {
        let mut matrix = HugeLongTriangularMatrix::new(4);
        let mut counter = 1i64;
        for i in 0..4 {
            for j in i..4 {
                matrix.set(i, j, counter);
                counter += 1;
            }
        }

        let entries: Vec<_> = matrix.entries().collect();
        assert_eq!(entries.len(), 10);
        assert!(entries.iter().all(|&(i, j, _)| i <= j));

        let diagonal: Vec<_> = matrix.diagonal_entries().collect();
        assert_eq!(diagonal.len(), 4);
        assert!(diagonal
            .iter()
            .enumerate()
            .all(|(i, &(idx, value))| idx == i && value == matrix.get(i, i)));

        let off: Vec<_> = matrix.off_diagonal_entries().collect();
        assert_eq!(off.len(), 6);
        assert!(off.iter().all(|&(i, j, _)| i < j));
    }

    #[test]
    fn conversion_to_square_matrix_preserves_values() {
        let mut tri = HugeLongTriangularMatrix::new(3);
        tri.set(0, 0, 1);
        tri.set(0, 2, 5);
        tri.set(1, 2, 7);

        let square = tri.to_square_matrix();
        for i in 0..3 {
            for j in 0..3 {
                let expected = tri.get(i, j);
                assert_eq!(square.get(i, j), expected);
            }
        }
    }

    #[test]
    fn sparsity_and_counts() {
        let mut matrix = HugeLongTriangularMatrix::new(3);
        assert_eq!(matrix.count_non_zero(), 0);
        assert_eq!(matrix.sparsity(), 1.0);

        matrix.set(0, 1, 9);
        matrix.set(1, 1, 4);
        assert_eq!(matrix.count_non_zero(), 2);
        assert!((matrix.sparsity() - (1.0 - 2.0 / 6.0)).abs() < 1e-9);
    }
}
