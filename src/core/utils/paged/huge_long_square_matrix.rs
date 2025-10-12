use super::HugeLongMatrix;

/// Square variant of `HugeLongMatrix` with additional symmetric helpers.
pub struct HugeLongSquareMatrix {
    matrix: HugeLongMatrix,
    order: usize,
}

impl HugeLongSquareMatrix {
    /// Creates an `order` × `order` matrix.
    pub fn new(order: usize) -> Self {
        Self {
            matrix: HugeLongMatrix::new(order, order),
            order,
        }
    }

    /// Set the value at the given coordinates.
    pub fn set(&mut self, row: usize, col: usize, value: i64) {
        self.matrix.set(row, col, value);
    }

    /// Get the value at the given coordinates.
    pub fn get(&self, row: usize, col: usize) -> i64 {
        self.matrix.get(row, col)
    }

    /// Adds `delta` to the value at both `(i, j)` and `(j, i)`.
    pub fn add_to_symmetric(&mut self, i: usize, j: usize, delta: i64) {
        self.matrix.add_to(i, j, delta);
        if i != j {
            self.matrix.add_to(j, i, delta);
        }
    }

    /// Sets both `(i, j)` and `(j, i)` to `value`.
    pub fn set_symmetric(&mut self, i: usize, j: usize, value: i64) {
        self.matrix.set(i, j, value);
        if i != j {
            self.matrix.set(j, i, value);
        }
    }

    /// Fill the diagonal with `value`.
    pub fn fill_diagonal(&mut self, value: i64) {
        for i in 0..self.order {
            self.matrix.set(i, i, value);
        }
    }

    /// Returns the diagonal values.
    pub fn diagonal(&self) -> Vec<i64> {
        (0..self.order).map(|i| self.matrix.get(i, i)).collect()
    }

    /// Overwrites the diagonal with `values`.
    pub fn set_diagonal(&mut self, values: &[i64]) {
        assert_eq!(values.len(), self.order, "diagonal length mismatch");
        for (i, &value) in values.iter().enumerate() {
            self.matrix.set(i, i, value);
        }
    }

    /// Sum of the diagonal elements.
    pub fn trace(&self) -> i64 {
        (0..self.order).map(|i| self.matrix.get(i, i)).sum()
    }

    /// Checks whether the matrix is symmetric within `tolerance`.
    pub fn is_symmetric(&self, tolerance: i64) -> bool {
        for i in 0..self.order {
            for j in (i + 1)..self.order {
                let diff = (self.matrix.get(i, j) - self.matrix.get(j, i)).abs();
                if diff > tolerance {
                    return false;
                }
            }
        }
        true
    }

    /// Order (dimension) of the matrix.
    pub fn order(&self) -> usize {
        self.order
    }

    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.order
    }

    /// Number of columns.
    pub fn cols(&self) -> usize {
        self.order
    }

    /// Returns a reference to the underlying matrix for advanced operations.
    pub fn as_matrix(&self) -> &HugeLongMatrix {
        &self.matrix
    }

    /// Returns a mutable reference to the underlying matrix.
    pub fn as_matrix_mut(&mut self) -> &mut HugeLongMatrix {
        &mut self.matrix
    }

    /// Total elements stored.
    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    /// Returns `true` when the matrix has zero size.
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }

    /// Iterator over the upper triangular portion (inclusive of diagonal).
    pub fn upper_triangular_entries(&self) -> UpperTriangularEntries<'_> {
        UpperTriangularEntries {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    /// Iterator over the lower triangular portion (inclusive of diagonal).
    pub fn lower_triangular_entries(&self) -> LowerTriangularEntries<'_> {
        LowerTriangularEntries {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    /// Iterator over off-diagonal entries.
    pub fn off_diagonal_entries(&self) -> OffDiagonalEntries<'_> {
        OffDiagonalEntries {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

pub struct UpperTriangularEntries<'a> {
    matrix: &'a HugeLongSquareMatrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for UpperTriangularEntries<'a> {
    type Item = (usize, usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let order = self.matrix.order;
        if self.row >= order {
            return None;
        }
        let value = (self.row, self.col, self.matrix.get(self.row, self.col));
        self.col += 1;
        if self.col >= order {
            self.row += 1;
            self.col = self.row;
        }
        Some(value)
    }
}

pub struct LowerTriangularEntries<'a> {
    matrix: &'a HugeLongSquareMatrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for LowerTriangularEntries<'a> {
    type Item = (usize, usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let order = self.matrix.order;
        if self.row >= order {
            return None;
        }

        let value = (self.row, self.col, self.matrix.get(self.row, self.col));
        if self.col >= self.row {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(value)
    }
}

pub struct OffDiagonalEntries<'a> {
    matrix: &'a HugeLongSquareMatrix,
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
                self.col = 0;
                continue;
            }

            if self.row != self.col {
                let result = (self.row, self.col, self.matrix.get(self.row, self.col));
                self.col += 1;
                return Some(result);
            }

            self.col += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HugeLongSquareMatrix;

    #[test]
    fn set_get_round_trip() {
        let mut matrix = HugeLongSquareMatrix::new(3);
        matrix.set(0, 0, 1);
        matrix.set(1, 2, 42);
        matrix.set(2, 1, -7);

        assert_eq!(matrix.get(0, 0), 1);
        assert_eq!(matrix.get(1, 2), 42);
        assert_eq!(matrix.get(2, 1), -7);
    }

    #[test]
    fn symmetric_helpers_work() {
        let mut matrix = HugeLongSquareMatrix::new(3);
        matrix.set_symmetric(0, 2, 5);
        assert_eq!(matrix.get(0, 2), 5);
        assert_eq!(matrix.get(2, 0), 5);

        matrix.add_to_symmetric(1, 2, 3);
        assert_eq!(matrix.get(1, 2), 3);
        assert_eq!(matrix.get(2, 1), 3);
    }

    #[test]
    fn diagonal_operations() {
        let mut matrix = HugeLongSquareMatrix::new(4);
        matrix.fill_diagonal(2);
        assert_eq!(matrix.diagonal(), vec![2, 2, 2, 2]);

        matrix.set_diagonal(&[1, 2, 3, 4]);
        assert_eq!(matrix.trace(), 10);
    }

    #[test]
    fn symmetry_check() {
        let mut matrix = HugeLongSquareMatrix::new(2);
        matrix.set(0, 1, 7);
        matrix.set(1, 0, 7);
        assert!(matrix.is_symmetric(0));

        matrix.set(1, 0, 10);
        assert!(!matrix.is_symmetric(0));
        assert!(matrix.is_symmetric(3));
    }

    #[test]
    fn iterators_cover_expected_entries() {
        let mut matrix = HugeLongSquareMatrix::new(3);
        let mut counter = 0;
        for i in 0..3 {
            for j in 0..3 {
                matrix.set(i, j, counter);
                counter += 1;
            }
        }

        let upper: Vec<_> = matrix.upper_triangular_entries().collect();
        assert_eq!(upper.len(), 6);
        assert!(upper.iter().all(|&(i, j, _)| i <= j));

        let lower: Vec<_> = matrix.lower_triangular_entries().collect();
        assert_eq!(lower.len(), 6);
        assert!(lower.iter().all(|&(i, j, _)| i >= j));

        let off: Vec<_> = matrix.off_diagonal_entries().collect();
        assert_eq!(off.len(), 6);
        assert!(off.iter().all(|&(i, j, _)| i != j));
    }
}
