use crate::collections::HugeLongArray;

/// Row-major matrix backed by a `HugeLongArray` for massive datasets.
pub struct HugeLongMatrix {
    array: HugeLongArray,
    rows: usize,
    cols: usize,
}

impl HugeLongMatrix {
    /// Creates a matrix with the provided dimensions.
    ///
    /// # Panics
    ///
    /// Panics if `rows * cols` overflows.
    pub fn new(rows: usize, cols: usize) -> Self {
        let total = match rows.checked_mul(cols) {
            Some(value) => value,
            None => panic!("matrix dimensions overflow: rows {} cols {}", rows, cols),
        };

        Self {
            array: HugeLongArray::new(total),
            rows,
            cols,
        }
    }

    /// Sets the value located at `row`, `col`.
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds.
    pub fn set(&mut self, row: usize, col: usize, value: i64) {
        assert!(row < self.rows, "row {} out of bounds {}", row, self.rows);
        assert!(col < self.cols, "col {} out of bounds {}", col, self.cols);
        let index = self.index_of(row, col);
        self.array.set(index, value);
    }

    /// Returns the value located at `row`, `col`.
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds.
    pub fn get(&self, row: usize, col: usize) -> i64 {
        assert!(row < self.rows, "row {} out of bounds {}", row, self.rows);
        assert!(col < self.cols, "col {} out of bounds {}", col, self.cols);
        let index = self.index_of(row, col);
        self.array.get(index)
    }

    /// Adds `delta` to the value located at `row`, `col`.
    pub fn add_to(&mut self, row: usize, col: usize, delta: i64) {
        assert!(row < self.rows, "row {} out of bounds {}", row, self.rows);
        assert!(col < self.cols, "col {} out of bounds {}", col, self.cols);
        let index = self.index_of(row, col);
        let current = self.array.get(index);
        self.array.set(index, current + delta);
    }

    /// Fills the entire matrix with `value`.
    pub fn fill(&mut self, value: i64) {
        self.array.fill(value);
    }

    /// Fills the given row with `value`.
    pub fn fill_row(&mut self, row: usize, value: i64) {
        assert!(row < self.rows, "row {} out of bounds {}", row, self.rows);
        for col in 0..self.cols {
            let index = self.index_of(row, col);
            self.array.set(index, value);
        }
    }

    /// Fills the given column with `value`.
    pub fn fill_col(&mut self, col: usize, value: i64) {
        assert!(col < self.cols, "col {} out of bounds {}", col, self.cols);
        for row in 0..self.rows {
            let index = self.index_of(row, col);
            self.array.set(index, value);
        }
    }

    /// Copies the specified row into a vector.
    pub fn row(&self, row: usize) -> Vec<i64> {
        assert!(row < self.rows, "row {} out of bounds {}", row, self.rows);
        let mut result = Vec::with_capacity(self.cols);
        for col in 0..self.cols {
            result.push(self.get(row, col));
        }
        result
    }

    /// Copies the specified column into a vector.
    pub fn col(&self, col: usize) -> Vec<i64> {
        assert!(col < self.cols, "col {} out of bounds {}", col, self.cols);
        let mut result = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            result.push(self.get(row, col));
        }
        result
    }

    /// Number of rows in the matrix.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns in the matrix.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Total element count.
    pub fn len(&self) -> usize {
        self.array.size()
    }

    /// Checks whether the matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

#[cfg(test)]
mod tests {
    use super::HugeLongMatrix;

    #[test]
    fn set_get_round_trip() {
        let mut matrix = HugeLongMatrix::new(3, 4);
        matrix.set(0, 0, 1);
        matrix.set(1, 2, 42);
        matrix.set(2, 3, -7);

        assert_eq!(matrix.get(0, 0), 1);
        assert_eq!(matrix.get(1, 2), 42);
        assert_eq!(matrix.get(2, 3), -7);
    }

    #[test]
    fn supports_rectangular_shapes() {
        let mut matrix = HugeLongMatrix::new(2, 5);
        for row in 0..2 {
            for col in 0..5 {
                matrix.set(row, col, (row * 10 + col) as i64);
            }
        }

        for row in 0..2 {
            for col in 0..5 {
                assert_eq!(matrix.get(row, col), (row * 10 + col) as i64);
            }
        }
    }

    #[test]
    fn add_to_accumulates() {
        let mut matrix = HugeLongMatrix::new(2, 2);
        matrix.add_to(0, 0, 5);
        matrix.add_to(0, 0, 3);
        assert_eq!(matrix.get(0, 0), 8);
    }

    #[test]
    fn fill_row_and_col() {
        let mut matrix = HugeLongMatrix::new(3, 3);
        matrix.fill(1);
        matrix.fill_row(1, 7);
        matrix.fill_col(2, 9);

        assert!(matrix.row(1).iter().all(|&value| value == 7));
        for row in 0..3 {
            assert_eq!(matrix.get(row, 2), 9);
        }
    }

    #[test]
    #[should_panic(expected = "matrix dimensions overflow")]
    fn overflow_panics() {
        let _ = HugeLongMatrix::new(usize::MAX, 2);
    }
}
