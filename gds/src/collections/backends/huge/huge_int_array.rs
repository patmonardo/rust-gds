//! HugeIntArray - i32 array supporting billions of elements
//!
//! Core numeric array implementation for graph data science, designed to handle
//! massive datasets that exceed standard array limitations.

use crate::collections::utils::cursor::{HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor};
use crate::collections::utils::{ArrayUtil, PageUtil};
use crate::concurrency::Concurrency;
use crate::core::utils::paged::ParallelIntPageCreator;

/// Maximum size for single-page arrays (from PageUtil)
const MAX_ARRAY_LENGTH: usize = 1 << 28; // ~268 million elements

/// A long-indexable i32 array that can contain more than 2 billion elements.
///
/// Implemented by paging smaller arrays to support approximately 32,000 billion elements.
/// For arrays small enough, uses optimized single-page implementation for maximum performance.
///
/// # Characteristics
///
/// - **Fixed size**: Cannot grow or shrink after creation
/// - **Dense storage**: Every position consumes memory (use sparse variants for sparse data)
/// - **Zero default**: Unset values return `0`
/// - **Thread safety**: Reads are safe, writes are not (external synchronization needed)
/// - **Cursor support**: Efficient zero-copy iteration over pages
///
/// # Examples
///
/// ```
/// use gds::collections::huge_array::HugeIntArray;
///
/// // Create array for 1 million node IDs
/// let mut node_ids = HugeIntArray::new(1_000_000);
/// node_ids.set(0, 42);
/// assert_eq!(node_ids.get(0), 42);
/// assert_eq!(node_ids.get(1), 0); // Default value
///
/// // Fill with values
/// node_ids.fill(100);
/// assert_eq!(node_ids.get(999_999), 100);
/// ```
///
/// # Cursor-Based Iteration
///
/// ```
/// use gds::collections::huge_array::HugeIntArray;
/// use gds::collections::cursor::{HugeCursor, init_cursor};
/// use gds::collections::cursor::HugeCursorSupport; // <<-- bring trait into scope so `new_cursor()` is available
///
/// let mut array = HugeIntArray::new(10000);
/// array.set_all(|i| i as i32);
///
/// let mut cursor = array.new_cursor();
/// init_cursor(&array, &mut cursor);
///
/// let mut sum = 0i32;
/// while cursor.next() {
///     let page = cursor.array().unwrap();
///     for i in cursor.offset()..cursor.limit() {
///         sum += page[i];
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub enum HugeIntArray {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeIntArray),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeIntArray),
}

impl HugeIntArray {
    /// Creates a new array of the given size.
    ///
    /// Automatically chooses optimal implementation based on size.
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeIntArray::new(size))
        } else {
            Self::Paged(PagedHugeIntArray::new(size))
        }
    }

    /// Inherent helper so callers (and doctests) can call `new_cursor()` without
    /// importing the `HugeCursorSupport` trait.
    pub fn new_cursor(&self) -> HugeIntArrayCursor<'_> {
        match self {
            Self::Single(arr) => HugeIntArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeIntArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }

    /// Creates a new array from pre-allocated pages.
    ///
    /// This method is used by `HugeIntArrayBuilder` to construct arrays
    /// from pages that have been filled concurrently.
    ///
    /// # Arguments
    ///
    /// * `pages` - Pre-allocated and filled page vector
    /// * `size` - Logical size of the array (number of valid elements)
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// // Create pages manually
    /// let page1 = vec![1, 2, 3, 4, 5];
    /// let page2 = vec![6, 7, 8, 9, 10];
    /// let pages = vec![page1, page2];
    ///
    /// let array = HugeIntArray::of(pages, 10);
    /// assert_eq!(array.get(0), 1);
    /// assert_eq!(array.get(9), 10);
    /// ```
    pub fn of(pages: Vec<Vec<i32>>, size: usize) -> Self {
        if pages.is_empty() {
            // Empty array
            Self::Single(SingleHugeIntArray::new(0))
        } else if pages.len() == 1 && size <= MAX_ARRAY_LENGTH {
            // Single page - truncate to actual size
            let mut page = pages.into_iter().next().unwrap();
            page.truncate(size);
            Self::Single(SingleHugeIntArray { data: page })
        } else {
            // Multiple pages
            Self::Paged(PagedHugeIntArray::from_pages(pages, size))
        }
    }

    /// Creates a new array from the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// let array = HugeIntArray::from_vec(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(array.get(2), 3);
    /// ```
    pub fn from_vec(values: Vec<i32>) -> Self {
        let size = values.len();
        let mut array = Self::new(size);
        for (i, &value) in values.iter().enumerate() {
            array.set(i, value);
        }
        array
    }

    /// Creates a new array with values generated in parallel using the provided function.
    ///
    /// This method uses parallel page creation for optimal performance on large arrays.
    /// For small arrays (≤ MAX_ARRAY_LENGTH), uses single-page sequential generation.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of elements in the array
    /// * `concurrency` - Parallelism level for page creation
    /// * `generator` - Function that generates value for each index
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create 1 billion node IDs in parallel
    /// let node_ids = HugeIntArray::with_generator(
    ///     1_000_000_000,
    ///     Concurrency::of(8),
    ///     |i| i as i32
    /// );
    /// assert_eq!(node_ids.get(0), 0);
    /// assert_eq!(node_ids.get(999_999_999), 999_999_999);
    ///
    /// // Custom sequence generation
    /// let squares = HugeIntArray::with_generator(
    ///     10_000,
    ///     Concurrency::of(4),
    ///     |i| (i * i) as i32
    /// );
    /// assert_eq!(squares.get(100), 10_000);
    /// ```
    pub fn with_generator<F>(size: usize, concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> i32 + Send + Sync + 'static,
    {
        if size <= MAX_ARRAY_LENGTH {
            // Small arrays: use single-page sequential generation
            let mut array = Self::Single(SingleHugeIntArray::new(size));
            array.set_all(generator);
            array
        } else {
            // Large arrays: use parallel page creation
            let creator = ParallelIntPageCreator::of(concurrency, generator);
            let pages = creator.create_pages(size);
            Self::Paged(PagedHugeIntArray::from_pages(pages, size))
        }
    }

    /// Returns the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn get(&self, index: usize) -> i32 {
        match self {
            Self::Single(arr) => arr.get(index),
            Self::Paged(arr) => arr.get(index),
        }
    }

    /// Sets the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn set(&mut self, index: usize, value: i32) {
        match self {
            Self::Single(arr) => arr.set(index, value),
            Self::Paged(arr) => arr.set(index, value),
        }
    }

    /// Adds delta to the value at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// let mut array = HugeIntArray::new(10);
    /// array.set(0, 10);
    /// array.add_to(0, 5);
    /// assert_eq!(array.get(0), 15);
    /// ```
    pub fn add_to(&mut self, index: usize, delta: i32) {
        match self {
            Self::Single(arr) => arr.add_to(index, delta),
            Self::Paged(arr) => arr.add_to(index, delta),
        }
    }

    /// Sets all elements using the provided generator function.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// let mut array = HugeIntArray::new(5);
    /// array.set_all(|i| (i * 2) as i32);
    /// assert_eq!(array.get(0), 0);
    /// assert_eq!(array.get(1), 2);
    /// assert_eq!(array.get(2), 4);
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i32,
    {
        match self {
            Self::Single(arr) => arr.set_all(gen),
            Self::Paged(arr) => arr.set_all(gen),
        }
    }

    /// Fills all elements with the specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// let mut array = HugeIntArray::new(100);
    /// array.fill(42);
    /// assert_eq!(array.get(50), 42);
    /// ```
    pub fn fill(&mut self, value: i32) {
        match self {
            Self::Single(arr) => arr.fill(value),
            Self::Paged(arr) => arr.fill(value),
        }
    }

    /// Returns the number of elements in the array.
    pub fn size(&self) -> usize {
        match self {
            Self::Single(arr) => arr.size(),
            Self::Paged(arr) => arr.size(),
        }
    }

    /// Returns the memory used by this array in bytes.
    pub fn size_of(&self) -> usize {
        match self {
            Self::Single(arr) => arr.size_of(),
            Self::Paged(arr) => arr.size_of(),
        }
    }

    /// Copies elements from this array to the destination array.
    ///
    /// # Panics
    ///
    /// Panics if length exceeds either array's size
    pub fn copy_to(&self, dest: &mut HugeIntArray, length: usize) {
        assert!(length <= self.size(), "length exceeds source array size");
        assert!(length <= dest.size(), "length exceeds dest array size");

        for i in 0..length {
            dest.set(i, self.get(i));
        }
    }

    /// Creates a copy of this array with a new length.
    ///
    /// If new length is larger, new elements are initialized to 0.
    pub fn copy_of(&self, new_length: usize) -> Self {
        let mut result = Self::new(new_length);
        let copy_length = usize::min(self.size(), new_length);
        self.copy_to(&mut result, copy_length);
        result
    }

    /// Converts to a standard Vec.
    ///
    /// # Panics
    ///
    /// Panics if size exceeds usize::MAX
    pub fn to_vec(&self) -> Vec<i32> {
        let size = self.size();
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.get(i));
        }
        result
    }

    /// Performs binary search for the given value (array must be sorted).
    ///
    /// Returns the index if found, or -(insertion_point + 1) if not found.
    pub fn binary_search(&self, search_value: i32) -> isize {
        match self {
            Self::Single(arr) => arr.binary_search(search_value),
            Self::Paged(arr) => arr.binary_search(search_value),
        }
    }

    /// Creates an iterator over all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeIntArray;
    ///
    /// let mut array = HugeIntArray::new(5);
    /// array.set_all(|i| i as i32);
    ///
    /// let sum: i32 = array.iter().sum();
    /// assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
    /// ```
    pub fn iter(&self) -> HugeIntArrayIter<'_> {
        HugeIntArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Iterator for HugeIntArray
pub struct HugeIntArrayIter<'a> {
    array: &'a HugeIntArray,
    index: usize,
}

impl<'a> Iterator for HugeIntArrayIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

// Cursor support for HugeIntArray
impl<'a> HugeCursorSupport<'a> for HugeIntArray {
    type Cursor = HugeIntArrayCursor<'a>;

    fn size(&self) -> usize {
        HugeIntArray::size(self)
    }

    fn new_cursor(&'a self) -> Self::Cursor {
        match self {
            HugeIntArray::Single(arr) => {
                HugeIntArrayCursor::Single(SinglePageCursor::new(&arr.data))
            }
            HugeIntArray::Paged(arr) => {
                let capacity = arr.size;
                HugeIntArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

/// Cursor implementation for HugeIntArray
pub enum HugeIntArrayCursor<'a> {
    Single(SinglePageCursor<'a, i32>),
    Paged(PagedCursor<'a, i32>),
}

impl<'a> HugeCursor<'a> for HugeIntArrayCursor<'a> {
    type Array = [i32];

    fn next(&mut self) -> bool {
        match self {
            Self::Single(cursor) => cursor.next(),
            Self::Paged(cursor) => cursor.next(),
        }
    }

    fn base(&self) -> usize {
        match self {
            Self::Single(cursor) => cursor.base(),
            Self::Paged(cursor) => cursor.base(),
        }
    }

    fn array(&self) -> Option<&'a Self::Array> {
        match self {
            Self::Single(cursor) => cursor.array(),
            Self::Paged(cursor) => cursor.array(),
        }
    }

    fn offset(&self) -> usize {
        match self {
            Self::Single(cursor) => cursor.offset(),
            Self::Paged(cursor) => cursor.offset(),
        }
    }

    fn limit(&self) -> usize {
        match self {
            Self::Single(cursor) => cursor.limit(),
            Self::Paged(cursor) => cursor.limit(),
        }
    }

    fn set_range(&mut self, from: usize, to: usize) {
        match self {
            Self::Single(cursor) => cursor.set_range(from, to),
            Self::Paged(cursor) => cursor.set_range(from, to),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::Single(cursor) => cursor.reset(),
            Self::Paged(cursor) => cursor.reset(),
        }
    }
}

/// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
#[derive(Debug, Clone)]
pub struct SingleHugeIntArray {
    data: Vec<i32>,
}

impl SingleHugeIntArray {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    fn get(&self, index: usize) -> i32 {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: i32) {
        self.data[index] = value;
    }

    fn add_to(&mut self, index: usize, delta: i32) {
        self.data[index] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i32,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: i32) {
        self.data.fill(value);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<i32>()
    }

    fn binary_search(&self, search_value: i32) -> isize {
        // Convert &Vec<i32> into &[i64] for ArrayUtil::binary_search_index
        let data_i64: Vec<i64> = self.data.iter().map(|&v| v as i64).collect();
        ArrayUtil::binary_search_index(&data_i64, data_i64.len(), search_value as i64)
    }
}

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
#[derive(Debug, Clone)]
pub struct PagedHugeIntArray {
    pages: Vec<Vec<i32>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeIntArray {
    fn new(size: usize) -> Self {
        // Calculate page size for i32 elements with 4KB pages
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<i32>());
        let page_shift = page_size.trailing_zeros(); // log2 of page_size
        let page_mask = page_size - 1;
        let num_pages = PageUtil::num_pages_for(size, page_size);

        let mut pages = Vec::with_capacity(num_pages);
        for page_index in 0..num_pages {
            let page_length = if page_index == num_pages - 1 {
                PageUtil::exclusive_index_of_page(size, page_mask)
            } else {
                page_size
            };
            pages.push(vec![0; page_length]);
        }

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    /// Creates a PagedHugeIntArray from pre-allocated pages.
    ///
    /// This is an internal constructor used by `with_generator` for parallel page creation.
    /// Pages must already be allocated and filled with appropriate values.
    fn from_pages(pages: Vec<Vec<i32>>, size: usize) -> Self {
        // Determine page size from the actual filled page length (first page).
        // Fall back to the default page size if pages are empty or the first page has zero length.
        let page_size = if !pages.is_empty() && !pages[0].is_empty() {
            pages[0].len()
        } else {
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<i32>())
        };
        let page_shift = page_size.trailing_zeros(); // log2 of page_size
        let page_mask = page_size - 1;

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> i32 {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: i32) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] = value;
    }

    fn add_to(&mut self, index: usize, delta: i32) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i32,
    {
        let mut global_index = 0;
        for page in &mut self.pages {
            for value in page.iter_mut() {
                if global_index < self.size {
                    *value = gen(global_index);
                    global_index += 1;
                }
            }
        }
    }

    fn fill(&mut self, value: i32) {
        for page in &mut self.pages {
            page.fill(value);
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        let mut total = 0;
        for page in &self.pages {
            total += page.len() * std::mem::size_of::<i32>();
        }
        total
    }

    fn binary_search(&self, search_value: i32) -> isize {
        // For paged arrays, we need to search across all pages
        // This is a simplified implementation - could be optimized
        let mut low = 0isize;
        let mut high = (self.size - 1) as isize;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_val = self.get(mid as usize);

            if mid_val < search_value {
                low = mid + 1;
            } else if mid_val > search_value {
                high = mid - 1;
            } else {
                return mid; // Found
            }
        }

        -(low + 1) // Not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::cursor::{init_cursor, init_cursor_range};

    // Basic array tests

    #[test]
    fn test_new() {
        let array = HugeIntArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeIntArray::new(10);
        array.set(5, 42);
        assert_eq!(array.get(5), 42);
    }

    #[test]
    fn test_add_to() {
        let mut array = HugeIntArray::new(10);
        array.set(0, 10);
        array.add_to(0, 5);
        assert_eq!(array.get(0), 15);
    }

    #[test]
    fn test_fill() {
        let mut array = HugeIntArray::new(100);
        array.fill(99);
        assert_eq!(array.get(0), 99);
        assert_eq!(array.get(99), 99);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeIntArray::new(5);
        array.set_all(|i| (i * 2) as i32);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(1), 2);
        assert_eq!(array.get(4), 8);
    }

    #[test]
    fn test_from_vec() {
        let array = HugeIntArray::from_vec(vec![10, 20, 30]);
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), 10);
        assert_eq!(array.get(1), 20);
        assert_eq!(array.get(2), 30);
    }

    #[test]
    fn test_to_vec() {
        let mut array = HugeIntArray::new(3);
        array.set(0, 100);
        array.set(1, 200);
        array.set(2, 300);
        let vec = array.to_vec();
        assert_eq!(vec, vec![100, 200, 300]);
    }

    #[test]
    fn test_copy_of() {
        let mut original = HugeIntArray::new(3);
        original.set(0, 1);
        original.set(1, 2);
        original.set(2, 3);

        let copy = original.copy_of(5);
        assert_eq!(copy.size(), 5);
        assert_eq!(copy.get(0), 1);
        assert_eq!(copy.get(1), 2);
        assert_eq!(copy.get(2), 3);
        assert_eq!(copy.get(3), 0);
        assert_eq!(copy.get(4), 0);
    }

    #[test]
    fn test_binary_search() {
        let array = HugeIntArray::from_vec(vec![1, 3, 5, 7, 9]);
        assert_eq!(array.binary_search(5), 2);
        assert_eq!(array.binary_search(1), 0);
        assert_eq!(array.binary_search(9), 4);
        assert!(array.binary_search(4) < 0); // Not found
    }

    #[test]
    fn test_iter() {
        let mut array = HugeIntArray::new(5);
        array.set_all(|i| i as i32);

        let sum: i32 = array.iter().sum();
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to use paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array = HugeIntArray::new(size);

        array.set(0, 100);
        array.set(MAX_ARRAY_LENGTH, 200);
        array.set(size - 1, 300);

        assert_eq!(array.get(0), 100);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 200);
        assert_eq!(array.get(size - 1), 300);
    }

    // Cursor tests

    #[test]
    fn test_cursor_basic_iteration() {
        let mut array = HugeIntArray::new(100);
        array.set_all(|i| i as i32);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        let mut sum = 0i32;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
            }
        }

        assert_eq!(sum, 4950); // Sum of 0..99
    }

    #[test]
    fn test_cursor_range_iteration() {
        let mut array = HugeIntArray::new(100);
        array.set_all(|i| i as i32);

        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 10, 20);

        let mut sum = 0i32;
        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
                count += 1;
            }
        }

        assert_eq!(count, 10); // Elements 10-19
        assert_eq!(sum, 145); // Sum of 10..19
    }

    #[test]
    fn test_cursor_empty_range() {
        let array = HugeIntArray::new(100);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 50, 50);

        assert!(!cursor.next()); // Empty range
    }

    #[test]
    fn test_cursor_reset() {
        let mut array = HugeIntArray::new(10);
        array.set_all(|i| i as i32);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        // First iteration
        assert!(cursor.next());

        // Reset and iterate again
        cursor.reset();
        let mut sum = 0i32;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
            }
        }

        assert_eq!(sum, 45); // Sum of 0..9
    }

    // with_generator tests

    #[test]
    fn test_with_generator_small_array() {
        use crate::concurrency::Concurrency;

        // Small array should use single-page implementation
        let array = HugeIntArray::with_generator(1000, Concurrency::of(4), |i| i as i32);

        assert_eq!(array.size(), 1000);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(500), 500);
        assert_eq!(array.get(999), 999);

        // Verify it's actually single-page variant
        assert!(matches!(array, HugeIntArray::Single(_)));
    }

    #[test]
    fn test_with_generator_large_array() {
        use crate::concurrency::Concurrency;

        // Large array should use paged implementation
        let size = MAX_ARRAY_LENGTH + 10000;
        let array = HugeIntArray::with_generator(size, Concurrency::of(4), |i| i as i32);

        assert_eq!(array.size(), size);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), MAX_ARRAY_LENGTH as i32);
        assert_eq!(array.get(size - 1), (size - 1) as i32);

        // Verify it's paged variant
        assert!(matches!(array, HugeIntArray::Paged(_)));
    }

    #[test]
    fn test_with_generator_identity_mapping() {
        use crate::concurrency::Concurrency;

        // Test identity mapping for 1 million elements
        let array = HugeIntArray::with_generator(1_000_000, Concurrency::of(8), |i| i as i32);

        // Spot checks
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(12345), 12345);
        assert_eq!(array.get(999_999), 999_999);
    }

    #[test]
    fn test_with_generator_custom_function() {
        use crate::concurrency::Concurrency;

        // Test custom generator: squares
        let array = HugeIntArray::with_generator(1000, Concurrency::of(4), |i| (i * i) as i32);

        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(1), 1);
        assert_eq!(array.get(10), 100);
        assert_eq!(array.get(100), 10_000);
    }

    #[test]
    fn test_with_generator_parallel_consistency() {
        use crate::concurrency::Concurrency;

        // Test that different concurrency levels produce same results
        let size = 100_000;

        let array1 = HugeIntArray::with_generator(size, Concurrency::of(1), |i| (i * 3) as i32);
        let array2 = HugeIntArray::with_generator(size, Concurrency::of(4), |i| (i * 3) as i32);
        let array8 = HugeIntArray::with_generator(size, Concurrency::of(8), |i| (i * 3) as i32);

        // Spot check several indices
        for idx in [0, 1000, 50000, 99999] {
            let expected = (idx * 3) as i32;
            assert_eq!(array1.get(idx), expected);
            assert_eq!(array2.get(idx), expected);
            assert_eq!(array8.get(idx), expected);
        }
    }

    #[test]
    #[ignore = "DANGEROUS: Allocates 1 billion elements - can crash system"]
    fn test_with_generator_billion_elements() {
        use crate::concurrency::Concurrency;

        // Test with 1 billion elements (this is fast with parallel creation!)
        let size = 1_000_000_000;
        let array = HugeIntArray::with_generator(size, Concurrency::of(8), |i| {
            if i % 1_000_000 == 0 {
                i as i32
            } else {
                0
            }
        });

        // Check size
        assert_eq!(array.size(), size);

        // Check milestone values
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(1_000_000), 1_000_000);
        assert_eq!(array.get(500_000_000), 500_000_000);
        assert_eq!(array.get(999_000_000), 999_000_000);

        // Check non-milestone values
        assert_eq!(array.get(1), 0);
        assert_eq!(array.get(999_999), 0);
    }

    #[test]
    fn test_with_generator_boundary_conditions() {
        use crate::concurrency::Concurrency;

        // Test exact page boundary
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<i32>());
        let array = HugeIntArray::with_generator(page_size * 10, Concurrency::of(4), |i| i as i32);

        // Check boundaries between pages
        assert_eq!(array.get(page_size - 1), (page_size - 1) as i32);
        assert_eq!(array.get(page_size), page_size as i32);
        assert_eq!(array.get(page_size + 1), (page_size + 1) as i32);
    }

    #[test]
    fn test_with_generator_compatibility_with_operations() {
        use crate::concurrency::Concurrency;

        // Verify that arrays created with with_generator work with all operations
        let mut array = HugeIntArray::with_generator(10000, Concurrency::of(4), |i| i as i32);

        // Test set/get
        array.set(5000, 999);
        assert_eq!(array.get(5000), 999);

        // Test add_to
        array.add_to(5000, 1);
        assert_eq!(array.get(5000), 1000);

        // Test fill
        array.fill(42);
        assert_eq!(array.get(0), 42);
        assert_eq!(array.get(9999), 42);

        // Test set_all
        array.set_all(|i| (i * 2) as i32);
        assert_eq!(array.get(100), 200);

        // Test iteration
        let sum: i32 = array.iter().take(10).sum();
        assert_eq!(sum, 90); // 0 + 2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18
    }
}

// Collections impl via macro
use crate::huge_collections;
use crate::types::ValueType;
huge_collections!(
    HugeIntArray,
    i32,
    ValueType::Int,
    0i32,
    to_f64 = |x: i32| x as f64,
    kind: Ord,
    [],
    [],
    "Collections impl for HugeIntArray"
);
