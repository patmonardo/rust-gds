//! HugeByteArray - i8 array supporting billions of elements
//!
//! Core numeric array implementation for graph data science, designed to handle
//! massive datasets that exceed standard array limitations.

use crate::collections::cursor::{HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor};
use crate::collections::{ArrayUtil, PageUtil};

/// Maximum size for single-page arrays (from PageUtil)
const MAX_ARRAY_LENGTH: usize = 1 << 28; // ~268 million elements

/// A long-indexable i8 array that can contain more than 2 billion elements.
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
/// use gds::collections::huge_array::HugeByteArray;
///
/// // Create array for 1 million node IDs
/// let mut node_ids = HugeByteArray::new(1_000_000);
/// node_ids.set(0, 42i8);
/// assert_eq!(node_ids.get(0), 42);
/// assert_eq!(node_ids.get(1), 0); // Default value
///
/// // Fill with values
/// node_ids.fill(100i8);
/// assert_eq!(node_ids.get(999_999), 100);
/// ```
pub enum HugeByteArray {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeByteArray),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeByteArray),
}

impl HugeByteArray {
    /// Creates a new array of the given size.
    ///
    /// Automatically chooses optimal implementation based on size.
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeByteArray::new(size))
        } else {
            Self::Paged(PagedHugeByteArray::new(size))
        }
    }

    /// Inherent helper so callers (and doctests) can call `new_cursor()` without
    /// importing the `HugeCursorSupport` trait.
    pub fn new_cursor(&self) -> HugeByteArrayCursor<'_> {
        match self {
            Self::Single(arr) => HugeByteArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeByteArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }

    /// Creates a new array from the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeByteArray;
    ///
    /// let array = HugeByteArray::from_vec(vec![1i8, 2, 3, 4, 5]);
    /// assert_eq!(array.get(2), 3);
    /// ```
    pub fn from_vec(values: Vec<i8>) -> Self {
        let size = values.len();
        let mut array = Self::new(size);
        for (i, &value) in values.iter().enumerate() {
            array.set(i, value);
        }
        array
    }

    /// Returns the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn get(&self, index: usize) -> i8 {
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
    pub fn set(&mut self, index: usize, value: i8) {
        match self {
            Self::Single(arr) => arr.set(index, value),
            Self::Paged(arr) => arr.set(index, value),
        }
    }

    /// Sets all elements using the provided generator function.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeByteArray;
    ///
    /// let mut array = HugeByteArray::new(5);
    /// array.set_all(|i| (i * 2) as i8);
    /// assert_eq!(array.get(0), 0);
    /// assert_eq!(array.get(1), 2);
    /// assert_eq!(array.get(2), 4);
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i8,
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
    /// use gds::collections::huge_array::HugeByteArray;
    ///
    /// let mut array = HugeByteArray::new(100);
    /// array.fill(42i8);
    /// assert_eq!(array.get(50), 42);
    /// ```
    pub fn fill(&mut self, value: i8) {
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
    pub fn copy_to(&self, dest: &mut HugeByteArray, length: usize) {
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
    pub fn to_vec(&self) -> Vec<i8> {
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
    pub fn binary_search(&self, search_value: i8) -> isize {
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
    /// use gds::collections::huge_array::HugeByteArray;
    ///
    /// let mut array = HugeByteArray::new(5);
    /// array.set_all(|i| i as i8);
    ///
    /// let sum: i8 = array.iter().sum();
    /// assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
    /// ```
    pub fn iter(&self) -> HugeByteArrayIter<'_> {
        HugeByteArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Iterator for HugeByteArray
pub struct HugeByteArrayIter<'a> {
    array: &'a HugeByteArray,
    index: usize,
}

impl<'a> Iterator for HugeByteArrayIter<'a> {
    type Item = i8;

    fn next(&mut self) -> Option<i8> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

// Cursor support for HugeByteArray
impl<'a> HugeCursorSupport<'a> for HugeByteArray {
    type Cursor = HugeByteArrayCursor<'a>;

    fn size(&self) -> usize {
        HugeByteArray::size(self)
    }

    fn new_cursor(&'a self) -> Self::Cursor {
        match self {
            HugeByteArray::Single(arr) => {
                HugeByteArrayCursor::Single(SinglePageCursor::new(&arr.data))
            }
            HugeByteArray::Paged(arr) => {
                let capacity = arr.size;
                HugeByteArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

/// Cursor implementation for HugeByteArray
pub enum HugeByteArrayCursor<'a> {
    Single(SinglePageCursor<'a, i8>),
    Paged(PagedCursor<'a, i8>),
}

impl<'a> HugeCursor<'a> for HugeByteArrayCursor<'a> {
    type Array = [i8];

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
pub struct SingleHugeByteArray {
    data: Vec<i8>,
}

impl SingleHugeByteArray {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    fn get(&self, index: usize) -> i8 {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: i8) {
        self.data[index] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i8,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: i8) {
        self.data.fill(value);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<i8>()
    }

    fn binary_search(&self, search_value: i8) -> isize {
        // Convert &Vec<i8> into &[i64] for ArrayUtil::binary_search_index
        let data_i64: Vec<i64> = self.data.iter().map(|&v| v as i64).collect();
        ArrayUtil::binary_search_index(&data_i64, data_i64.len(), search_value as i64)
    }
}

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
pub struct PagedHugeByteArray {
    pages: Vec<Vec<i8>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeByteArray {
    fn new(size: usize) -> Self {
        // Calculate page size for i8 elements with 4KB pages
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<i8>());
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

    fn get(&self, index: usize) -> i8 {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: i8) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> i8,
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

    fn fill(&mut self, value: i8) {
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
            total += page.len() * std::mem::size_of::<i8>();
        }
        total
    }

    fn binary_search(&self, search_value: i8) -> isize {
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

// Collections impl via macro
use crate::huge_collections;
use crate::types::ValueType;
huge_collections!(
    HugeByteArray,
    i8,
    ValueType::Byte,
    0i8,
    to_f64 = |x: i8| x as f64,
    kind: Ord,
    [],
    [],
    "Collections impl for HugeByteArray"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::cursor::{init_cursor, init_cursor_range};

    // Basic array tests

    #[test]
    fn test_new() {
        let array = HugeByteArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeByteArray::new(10);
        array.set(5, 42i8);
        assert_eq!(array.get(5), 42);
    }

    #[test]
    fn test_fill() {
        let mut array = HugeByteArray::new(100);
        array.fill(99i8);
        assert_eq!(array.get(0), 99);
        assert_eq!(array.get(99), 99);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeByteArray::new(5);
        array.set_all(|i| (i * 2) as i8);
        assert_eq!(array.get(0), 0);
        assert_eq!(array.get(1), 2);
        assert_eq!(array.get(4), 8);
    }

    #[test]
    fn test_from_vec() {
        let array = HugeByteArray::from_vec(vec![10i8, 20, 30]);
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), 10);
        assert_eq!(array.get(1), 20);
        assert_eq!(array.get(2), 30);
    }

    #[test]
    fn test_to_vec() {
        let mut array = HugeByteArray::new(3);
        array.set(0, 100i8);
        array.set(1, 120i8);
        array.set(2, 127i8);
        let vec = array.to_vec();
        assert_eq!(vec, vec![100, 120, 127]);
    }

    #[test]
    fn test_copy_of() {
        let mut original = HugeByteArray::new(3);
        original.set(0, 1i8);
        original.set(1, 2i8);
        original.set(2, 3i8);

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
        let array = HugeByteArray::from_vec(vec![1i8, 3, 5, 7, 9]);
        assert_eq!(array.binary_search(5), 2);
        assert_eq!(array.binary_search(1), 0);
        assert_eq!(array.binary_search(9), 4);
        assert!(array.binary_search(4) < 0); // Not found
    }

    #[test]
    fn test_iter() {
        let mut array = HugeByteArray::new(5);
        array.set_all(|i| i as i8);

        let sum: i8 = array.iter().sum();
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to use paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array = HugeByteArray::new(size);

        array.set(0, 100i8);
        array.set(MAX_ARRAY_LENGTH, 120i8);
        array.set(size - 1, 127i8);

        assert_eq!(array.get(0), 100);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 120);
        assert_eq!(array.get(size - 1), 127);
    }

    // Cursor tests

    #[test]
    fn test_cursor_basic_iteration() {
        let mut array = HugeByteArray::new(10); // Smaller array
        array.set_all(|i| (i % 10) as i8); // Values 0-9

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        let mut sum = 0i8;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
            }
        }

        assert_eq!(sum, 45); // Sum of 0..9
    }

    #[test]
    fn test_cursor_range_iteration() {
        let mut array = HugeByteArray::new(20); // Smaller array
        array.set_all(|i| (i % 10) as i8); // Values 0-9 repeating

        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 10, 20);

        let mut sum = 0i8;
        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
                count += 1;
            }
        }

        assert_eq!(count, 10); // Elements 10-19
        assert_eq!(sum, 45); // Sum of 0..9 (repeating pattern)
    }

    #[test]
    fn test_cursor_empty_range() {
        let array = HugeByteArray::new(100);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 50, 50);

        assert!(!cursor.next()); // Empty range
    }

    #[test]
    fn test_cursor_reset() {
        let mut array = HugeByteArray::new(10);
        array.set_all(|i| i as i8);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        // First iteration
        assert!(cursor.next());

        // Reset and iterate again
        cursor.reset();
        let mut sum = 0i8;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i];
            }
        }

        assert_eq!(sum, 45); // Sum of 0..9
    }
}
