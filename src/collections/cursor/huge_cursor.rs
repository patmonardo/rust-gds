/// Cursor-based iteration over huge arrays with zero-copy page access.
///
/// This module provides efficient iteration over huge arrays that may span multiple pages.
/// Cursors present a unified view of the data while handling underlying paging structure
/// transparently.
///
/// # Design Philosophy
///
/// **1. Zero-Copy Access:**
/// Cursors provide direct access to underlying array pages without copying data,
/// ensuring maximum performance for large-scale data processing.
///
/// **2. Page-Aware Traversal:**
/// Automatically handles transitions between pages in multi-page arrays, presenting
/// a seamless iteration interface regardless of storage structure.
///
/// **3. Range-Based Processing:**
/// Supports efficient processing of arbitrary ranges within huge arrays, enabling
/// parallel processing and windowed operations.
///
/// # Example: Basic Iteration
///
/// ```
/// use rust_gds::collections::cursor::{HugeCursor, SinglePageCursor};
///
/// let data = vec![1, 2, 3, 4, 5];
/// let mut cursor = SinglePageCursor::new(&data);
/// cursor.set_range(0, 5);
///
/// while cursor.next() {
///     let page = cursor.array().unwrap();
///     for i in cursor.offset()..cursor.limit() {
///         let global_idx = cursor.base() + i;
///         let value = page[i];
///         println!("Index {}: {}", global_idx, value);
///     }
/// }
///

/// Number of elements in a single page
const PAGE_SIZE: usize = 4096;
/// Number of bits to shift for page index calculation
const PAGE_SHIFT: usize = 12; // log2(4096)
/// Mask to extract offset within a page
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// View of data underlying a huge array, accessible as slices of primitive arrays.
///
/// Cursors provide efficient iteration over arrays that may be split across multiple
/// pages, handling page transitions transparently while maintaining optimal memory
/// access patterns.
///
/// # State Management
///
/// A cursor maintains four key pieces of state:
/// - **base**: Global index of first element in current page
/// - **array**: Reference to current page data
/// - **offset**: Starting index within current page (inclusive)
/// - **limit**: Ending index within current page (exclusive)
///
/// # Thread Safety
///
/// Cursors are NOT thread-safe and should only be used from a single thread.
/// Multiple cursors can safely read the same array concurrently, but each cursor
/// maintains independent state.
pub trait HugeCursor<'a> {
    /// The type of array slice this cursor provides access to.
    type Array: ?Sized;

    /// Try to load the next page and return whether new data is available.
    ///
    /// This method advances the cursor to the next available page of data. Once it
    /// returns `false`, the cursor is exhausted and will never return `true` again
    /// until reset.
    ///
    /// # State Management
    /// - **First call**: Loads first page and sets up initial state
    /// - **Subsequent calls**: Advances to next pages until exhausted
    /// - **After exhaustion**: Always returns `false`
    ///
    /// # Returns
    /// - `true` if cursor contains new data
    /// - `false` if exhausted
    fn next(&mut self) -> bool;

    /// Get the base index for calculating global positions.
    ///
    /// This value represents the global index of the first element in the current page.
    /// To get the global index of an element at position `i` in the current page:
    /// `global_index = base() + i`
    fn base(&self) -> usize;

    /// Get a reference to the current array page, if available.
    ///
    /// Returns `None` if the cursor has not been positioned (before first `next()`)
    /// or after exhaustion.
    fn array(&self) -> Option<&'a Self::Array>;

    /// Get the offset into the current array page.
    ///
    /// This is the first valid index within the current page. Elements before this
    /// offset should not be accessed.
    fn offset(&self) -> usize;

    /// Get the limit of the current array page (exclusive).
    ///
    /// This is the first index that should NOT be accessed in the current page.
    /// The valid range is `[offset, limit)`.
    fn limit(&self) -> usize;

    /// Set the range for iteration.
    ///
    /// # Arguments
    /// - `start`: Starting index (inclusive)
    /// - `end`: Ending index (exclusive)
    ///
    /// # Range Semantics
    /// - Range is `[start, end)` (half-open interval)
    /// - Must satisfy: `0 <= start <= end <= capacity`
    fn set_range(&mut self, start: usize, end: usize);

    /// Reset the cursor to iterate over the full capacity.
    fn reset(&mut self);
}

/// Single-page cursor implementation for arrays that fit within one page.
///
/// This implementation is optimized for smaller arrays, providing minimal overhead
/// while maintaining the same interface as multi-page cursors.
///
/// # Optimization Benefits
/// - **Single iteration**: Only one call to `next()` returns true
/// - **Direct access**: No page management overhead
/// - **Minimal state**: Reduced memory footprint
pub struct SinglePageCursor<'a, T> {
    /// Reference to the array data
    array: Option<&'a [T]>,
    /// Base index (always 0 for single-page)
    base: usize,
    /// Offset within array
    offset: usize,
    /// Limit within array (exclusive)
    limit: usize,
    /// Whether cursor has been exhausted
    exhausted: bool,
}

impl<'a, T> SinglePageCursor<'a, T> {
    /// Create a new single-page cursor for the given array.
    pub fn new(array: &'a [T]) -> Self {
        Self {
            array: Some(array),
            base: 0,
            offset: 0,
            limit: array.len(),
            exhausted: false,
        }
    }

    /// Set a new array for this cursor to iterate over.
    pub fn set_array(&mut self, array: &'a [T]) {
        self.array = Some(array);
        self.base = 0;
        self.offset = 0;
        self.limit = array.len();
        self.exhausted = false;
    }
}

impl<'a, T> HugeCursor<'a> for SinglePageCursor<'a, T> {
    type Array = [T];

    fn next(&mut self) -> bool {
        if self.exhausted {
            return false;
        }

        // Check if there's actually any data in the range
        if self.offset >= self.limit {
            self.exhausted = true;
            return false;
        }

        self.exhausted = true;
        true
    }

    fn base(&self) -> usize {
        self.base
    }

    fn array(&self) -> Option<&'a [T]> {
        self.array
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn limit(&self) -> usize {
        self.limit
    }

    fn set_range(&mut self, start: usize, end: usize) {
        self.exhausted = false;
        self.offset = start;
        self.limit = end;
    }

    fn reset(&mut self) {
        self.exhausted = false;
        if let Some(arr) = self.array {
            self.offset = 0;
            self.limit = arr.len();
        }
    }
}

/// Multi-page cursor implementation for arrays split across multiple pages.
///
/// This implementation handles arrays that span multiple pages, automatically
/// managing page transitions and providing seamless iteration over very large
/// datasets.
///
/// # Page Management
/// - **Lazy loading**: Pages are accessed only when needed
/// - **Automatic transitions**: Seamlessly moves between pages
/// - **Range awareness**: Respects start/end boundaries across page boundaries
pub struct PagedCursor<'a, T> {
    /// All pages in the array
    pages: &'a [Vec<T>],
    /// Current page index
    page_index: isize,
    /// First page in range
    from_page: usize,
    /// Last page in range
    max_page: usize,
    /// Total capacity
    capacity: usize,
    /// End index (exclusive)
    end: usize,
    /// Base index for current page
    base: usize,
    /// Offset within current page
    offset: usize,
    /// Limit within current page (exclusive)
    limit: usize,
    /// Current array reference
    current_array: Option<&'a [T]>,
}

impl<'a, T> PagedCursor<'a, T> {
    /// Create a new paged cursor.
    ///
    /// # Arguments
    /// - `pages`: Slice of pages to iterate over
    /// - `capacity`: Total capacity across all pages
    pub fn new(pages: &'a [Vec<T>], capacity: usize) -> Self {
        Self {
            pages,
            page_index: -1,
            from_page: 0,
            max_page: 0,
            capacity,
            end: capacity,
            base: 0,
            offset: 0,
            limit: 0,
            current_array: None,
        }
    }

    /// Helper to get page index from global index
    fn page_index(index: usize) -> usize {
        index >> PAGE_SHIFT
    }

    /// Helper to get index within page from global index
    fn index_in_page(index: usize) -> usize {
        index & PAGE_MASK
    }

    /// Helper to get exclusive index within page
    fn exclusive_index_of_page(index: usize) -> usize {
        let result = Self::index_in_page(index);
        if result == 0 && index > 0 {
            PAGE_SIZE
        } else {
            result
        }
    }
}

impl<'a, T> HugeCursor<'a> for PagedCursor<'a, T> {
    type Array = [T];

    fn next(&mut self) -> bool {
        let current = self.page_index + 1;
        if current as usize > self.max_page {
            return false;
        }

        self.page_index = current;
        let current = current as usize;

        if current >= self.pages.len() {
            return false;
        }

        self.current_array = Some(&self.pages[current]);

        if current == self.from_page {
            // First page - offset and limit already set in set_range
            return true;
        }

        // Subsequent pages
        self.base = current * PAGE_SIZE;
        self.offset = 0;

        if current == self.max_page {
            // Last page - may have partial data
            self.limit = Self::exclusive_index_of_page(self.end);
        } else {
            // Middle page - use full page
            self.limit = self.pages[current].len();
        }

        true
    }

    fn base(&self) -> usize {
        self.base
    }

    fn array(&self) -> Option<&'a [T]> {
        self.current_array
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn limit(&self) -> usize {
        self.limit
    }

    fn set_range(&mut self, start: usize, end: usize) {
        self.from_page = Self::page_index(start);
        self.max_page = Self::page_index(end.saturating_sub(1));
        self.page_index = self.from_page as isize - 1;
        self.end = end;
        self.base = self.from_page * PAGE_SIZE;
        self.offset = Self::index_in_page(start);
        self.limit = if self.from_page == self.max_page {
            Self::exclusive_index_of_page(end)
        } else {
            PAGE_SIZE
        };
        self.current_array = None;
    }

    fn reset(&mut self) {
        self.set_range(0, self.capacity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_page_cursor_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let mut cursor = SinglePageCursor::new(&data);
        cursor.set_range(0, 5);

        assert!(cursor.next());
        assert_eq!(cursor.base(), 0);
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 5);
        assert_eq!(cursor.array().unwrap().len(), 5);

        // Second call should return false
        assert!(!cursor.next());
    }

    #[test]
    fn test_single_page_cursor_range() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut cursor = SinglePageCursor::new(&data);
        cursor.set_range(2, 7); // Elements 2-6

        assert!(cursor.next());
        assert_eq!(cursor.offset(), 2);
        assert_eq!(cursor.limit(), 7);

        let array = cursor.array().unwrap();
        assert_eq!(array[2], 3); // Index 2 is value 3
        assert_eq!(array[6], 7); // Index 6 is value 7
    }

    #[test]
    fn test_single_page_cursor_empty_range() {
        let data = vec![1, 2, 3];
        let mut cursor = SinglePageCursor::new(&data);
        cursor.set_range(2, 2); // Empty range

        assert!(!cursor.next()); // Should immediately return false
    }

    #[test]
    fn test_single_page_cursor_reset() {
        let data = vec![1, 2, 3, 4, 5];
        let mut cursor = SinglePageCursor::new(&data);
        cursor.set_range(1, 3);

        assert!(cursor.next());
        assert!(!cursor.next());

        // Reset and try again
        cursor.reset();
        assert!(cursor.next());
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 5);
    }

    #[test]
    fn test_paged_cursor_single_page() {
        let pages = vec![vec![1, 2, 3, 4, 5]];
        let mut cursor = PagedCursor::new(&pages, 5);
        cursor.set_range(0, 5);

        assert!(cursor.next());
        assert_eq!(cursor.base(), 0);
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 5);

        assert!(!cursor.next());
    }

    #[test]
    fn test_paged_cursor_multiple_pages() {
        // Create 3 pages of 4096 elements each
        let page1: Vec<i64> = (0..4096).collect();
        let page2: Vec<i64> = (4096..8192).collect();
        let page3: Vec<i64> = (8192..12288).collect();
        let pages = vec![page1, page2, page3];

        let mut cursor = PagedCursor::new(&pages, 12288);
        cursor.set_range(0, 12288);

        let mut page_count = 0;
        while cursor.next() {
            page_count += 1;
            assert!(cursor.array().is_some());
        }

        assert_eq!(page_count, 3);
    }

    #[test]
    fn test_paged_cursor_range_across_pages() {
        // Create 3 pages of 4096 elements each
        let page1: Vec<i64> = (0..4096).collect();
        let page2: Vec<i64> = (4096..8192).collect();
        let page3: Vec<i64> = (8192..12288).collect();
        let pages = vec![page1, page2, page3];

        let mut cursor = PagedCursor::new(&pages, 12288);
        cursor.set_range(100, 10000); // From first page across to third page

        // First page
        assert!(cursor.next());
        assert_eq!(cursor.base(), 0);
        assert_eq!(cursor.offset(), 100);
        assert_eq!(cursor.limit(), 4096);

        // Second page (full page)
        assert!(cursor.next());
        assert_eq!(cursor.base(), 4096);
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 4096);

        // Third page (partial)
        assert!(cursor.next());
        assert_eq!(cursor.base(), 8192);
        assert_eq!(cursor.offset(), 0);
        assert_eq!(cursor.limit(), 1808); // 10000 - 8192 = 1808

        assert!(!cursor.next());
    }
}
