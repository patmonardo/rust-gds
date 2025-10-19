/// Support trait for arrays that can provide cursor-based iteration.
///
/// This trait establishes the contract that all HugeArray variants must implement
/// to provide cursor-based iteration. It serves as the bridge between array storage
/// and cursor iteration.
use super::huge_cursor::HugeCursor;

/// Interface for arrays that support cursor-based iteration.
///
/// This trait defines how arrays create, initialize, and configure cursors for
/// different iteration scenarios.
///
/// # Cursor Lifecycle
///
/// ```text
/// Create → Initialize → Position → Iterate → Drop
///    ↓         ↓          ↓         ↓        ↓
/// newCursor  initCursor  next()   process  (automatic)
/// (invalid)  (invalid)  (valid)   (valid)  (cleaned)
/// ```
///
/// # Example
///
/// ```ignore
/// // Full array iteration
/// let mut cursor = array.new_cursor();
/// init_cursor(&array, &mut cursor);
///
/// while cursor.next() {
///     let page = cursor.array().unwrap();
///     for i in cursor.offset()..cursor.limit() {
///         let value = page[i];
///         // Process value...
///     }
/// }
/// ```
pub trait HugeCursorSupport<'a> {
    /// The type of cursor this array provides.
    type Cursor: HugeCursor<'a>;

    /// Returns the logical length of this array in elements.
    ///
    /// This represents the total number of elements across all pages.
    /// The size determines the valid index range: `[0, size())`.
    ///
    /// # Performance
    /// This should be an O(1) operation.
    fn size(&self) -> usize;

    /// Creates a new cursor for iterating over this array.
    ///
    /// # Critical State Warning
    /// The returned cursor is in an **invalid, unpositioned state**. You MUST call
    /// `init_cursor()` or `init_cursor_range()` to configure it, then `next()` to
    /// position it before accessing any data fields.
    ///
    /// # Empty Array Behavior
    /// Creating a cursor for an empty array has undefined behavior and may result
    /// in errors when used.
    fn new_cursor(&'a self) -> Self::Cursor;

    /// Get the capacity of the array (may be larger than size).
    fn capacity(&self) -> usize {
        self.size()
    }
}

/// Initialize a cursor to iterate over the complete array.
///
/// This configures the cursor to visit every element from index 0 to `size() - 1`.
/// After calling this function, you must still call `cursor.next()` to position
/// the cursor to the first valid page.
///
/// # Arguments
/// - `support`: The array providing cursor support
/// - `cursor`: The cursor to initialize (will be modified in place)
///
/// # Example
///
/// ```ignore
/// let mut cursor = array.new_cursor();
/// init_cursor(&array, &mut cursor);
///
/// while cursor.next() {
///     // Process data...
/// }
/// ```
pub fn init_cursor<'a, S>(support: &S, cursor: &mut S::Cursor)
where
    S: HugeCursorSupport<'a>,
{
    let size = support.size();
    cursor.reset();
    cursor.set_range(0, size);
}

/// Initialize a cursor to iterate over a specific range within the array.
///
/// This enables range-based iteration for parallel processing or chunked algorithms.
///
/// # Range Semantics
/// Uses half-open interval notation `[start, end)`:
/// - `start`: First index to include (inclusive)
/// - `end`: First index to exclude (exclusive)
/// - Element count: `end - start`
/// - Empty range when `start == end`
///
/// # Parameter Validation
/// - `start` must be in range `[0, size]`
/// - `end` must be in range `[start, size]`
/// - Both parameters must be non-negative
///
/// # Panics
/// Panics if validation fails with descriptive error message.
///
/// # Example: Parallel Processing
///
/// ```ignore
/// let total_size = array.size();
/// let chunk_size = total_size / 4;
///
/// for i in 0..4 {
///     let start = i * chunk_size;
///     let end = if i == 3 { total_size } else { (i + 1) * chunk_size };
///
///     let mut cursor = array.new_cursor();
///     init_cursor_range(&array, &mut cursor, start, end);
///
///     while cursor.next() {
///         // Process chunk...
///     }
/// }
/// ```
pub fn init_cursor_range<'a, S>(support: &S, cursor: &mut S::Cursor, start: usize, end: usize)
where
    S: HugeCursorSupport<'a>,
{
    let size = support.size();

    if start > size {
        panic!("start expected to be in [0 : {}] but got {}", size, start);
    }

    if end < start || end > size {
        panic!(
            "end expected to be in [{} : {}] but got {}",
            start, size, end
        );
    }

    cursor.set_range(start, end);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::cursor::SinglePageCursor;

    // Simple test implementation
    struct TestArray {
        data: Vec<i64>,
    }

    impl TestArray {
        fn new(data: Vec<i64>) -> Self {
            Self { data }
        }
    }

    impl<'a> HugeCursorSupport<'a> for TestArray {
        type Cursor = SinglePageCursor<'a, i64>;

        fn size(&self) -> usize {
            self.data.len()
        }

        fn new_cursor(&'a self) -> Self::Cursor {
            SinglePageCursor::new(&self.data)
        }
    }

    #[test]
    fn test_init_cursor_full() {
        let array = TestArray::new(vec![1, 2, 3, 4, 5]);
        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        assert!(cursor.next());
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 5);
    }

    #[test]
    fn test_init_cursor_range() {
        let array = TestArray::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 2, 7);

        assert!(cursor.next());
        assert_eq!(cursor.offset(), 2);
        assert_eq!(cursor.limit(), 7);
    }

    #[test]
    #[should_panic(expected = "start expected to be in")]
    fn test_init_cursor_range_invalid_start() {
        let array = TestArray::new(vec![1, 2, 3]);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 10, 11);
    }

    #[test]
    #[should_panic(expected = "end expected to be in")]
    fn test_init_cursor_range_invalid_end() {
        let array = TestArray::new(vec![1, 2, 3]);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 0, 10);
    }

    #[test]
    #[should_panic(expected = "end expected to be in")]
    fn test_init_cursor_range_end_before_start() {
        let array = TestArray::new(vec![1, 2, 3, 4, 5]);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 3, 1);
    }

    #[test]
    fn test_init_cursor_empty_range() {
        let array = TestArray::new(vec![1, 2, 3, 4, 5]);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 2, 2);

        // Empty range should not produce any data
        assert!(!cursor.next());
    }
}
