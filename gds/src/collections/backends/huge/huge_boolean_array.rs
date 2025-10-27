//! HugeBooleanArray - bool array supporting billions of elements
//!
//! Core boolean array implementation for graph data science, designed to handle
//! massive datasets that exceed standard array limitations.

use crate::collections::cursor::{HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor};
use crate::collections::{ArrayUtil, PageUtil};

/// Maximum size for single-page arrays (from PageUtil)
const MAX_ARRAY_LENGTH: usize = 1 << 28; // ~268 million elements

/// A long-indexable bool array that can contain more than 2 billion elements.
///
/// Implemented by paging smaller arrays to support approximately 32,000 billion elements.
/// For arrays small enough, uses optimized single-page implementation for maximum performance.
///
/// # Characteristics
///
/// - **Fixed size**: Cannot grow or shrink after creation
/// - **Dense storage**: Every position consumes memory (use sparse variants for sparse data)
/// - **False default**: Unset values return `false`
/// - **Thread safety**: Reads are safe, writes are not (external synchronization needed)
/// - **Cursor support**: Efficient zero-copy iteration over pages
///
/// # Examples
///
/// ```
/// use gds::collections::huge_array::HugeBooleanArray;
///
/// // Create array for 1 million boolean flags
/// let mut flags = HugeBooleanArray::new(1_000_000);
/// flags.set(0, true);
/// assert_eq!(flags.get(0), true);
/// assert_eq!(flags.get(1), false); // Default value
///
/// // Fill with values
/// flags.fill(true);
/// assert_eq!(flags.get(999_999), true);
/// ```
pub enum HugeBooleanArray {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeBooleanArray),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeBooleanArray),
}

impl HugeBooleanArray {
    /// Creates a new array of the given size.
    ///
    /// Automatically chooses optimal implementation based on size.
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeBooleanArray::new(size))
        } else {
            Self::Paged(PagedHugeBooleanArray::new(size))
        }
    }

    /// Inherent helper so callers (and doctests) can call `new_cursor()` without
    /// importing the `HugeCursorSupport` trait.
    pub fn new_cursor(&self) -> HugeBooleanArrayCursor<'_> {
        match self {
            Self::Single(arr) => HugeBooleanArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeBooleanArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }

    /// Creates a new array from the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeBooleanArray;
    ///
    /// let array = HugeBooleanArray::from_vec(vec![true, false, true]);
    /// assert_eq!(array.get(1), false);
    /// ```
    pub fn from_vec(values: Vec<bool>) -> Self {
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
    pub fn get(&self, index: usize) -> bool {
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
    pub fn set(&mut self, index: usize, value: bool) {
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
    /// use gds::collections::huge_array::HugeBooleanArray;
    ///
    /// let mut array = HugeBooleanArray::new(5);
    /// array.set_all(|i| i % 2 == 0);
    /// assert_eq!(array.get(0), true);  // 0 % 2 == 0
    /// assert_eq!(array.get(1), false); // 1 % 2 != 0
    /// assert_eq!(array.get(2), true);  // 2 % 2 == 0
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> bool,
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
    /// use gds::collections::huge_array::HugeBooleanArray;
    ///
    /// let mut array = HugeBooleanArray::new(100);
    /// array.fill(true);
    /// assert_eq!(array.get(50), true);
    /// ```
    pub fn fill(&mut self, value: bool) {
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
    pub fn copy_to(&self, dest: &mut HugeBooleanArray, length: usize) {
        assert!(length <= self.size(), "length exceeds source array size");
        assert!(length <= dest.size(), "length exceeds dest array size");

        for i in 0..length {
            dest.set(i, self.get(i));
        }
    }

    /// Creates a copy of this array with a new length.
    ///
    /// If new length is larger, new elements are initialized to false.
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
    pub fn to_vec(&self) -> Vec<bool> {
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
    pub fn binary_search(&self, search_value: bool) -> isize {
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
    /// use gds::collections::huge_array::HugeBooleanArray;
    ///
    /// let mut array = HugeBooleanArray::new(5);
    /// array.set_all(|i| i % 2 == 0);
    ///
    /// let count: usize = array.iter().filter(|&&b| b).count();
    /// assert_eq!(count, 3); // true at indices 0, 2, 4
    /// ```
    pub fn iter(&self) -> HugeBooleanArrayIter<'_> {
        HugeBooleanArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Iterator for HugeBooleanArray
pub struct HugeBooleanArrayIter<'a> {
    array: &'a HugeBooleanArray,
    index: usize,
}

impl<'a> Iterator for HugeBooleanArrayIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

// Cursor support for HugeBooleanArray
impl<'a> HugeCursorSupport<'a> for HugeBooleanArray {
    type Cursor = HugeBooleanArrayCursor<'a>;

    fn size(&self) -> usize {
        HugeBooleanArray::size(self)
    }

    fn new_cursor(&'a self) -> Self::Cursor {
        match self {
            HugeBooleanArray::Single(arr) => {
                HugeBooleanArrayCursor::Single(SinglePageCursor::new(&arr.data))
            }
            HugeBooleanArray::Paged(arr) => {
                let capacity = arr.size;
                HugeBooleanArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

/// Cursor implementation for HugeBooleanArray
pub enum HugeBooleanArrayCursor<'a> {
    Single(SinglePageCursor<'a, bool>),
    Paged(PagedCursor<'a, bool>),
}

impl<'a> HugeCursor<'a> for HugeBooleanArrayCursor<'a> {
    type Array = [bool];

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
pub struct SingleHugeBooleanArray {
    data: Vec<bool>,
}

impl SingleHugeBooleanArray {
    fn new(size: usize) -> Self {
        Self {
            data: vec![false; size],
        }
    }

    fn get(&self, index: usize) -> bool {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: bool) {
        self.data[index] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> bool,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: bool) {
        self.data.fill(value);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<bool>()
    }

    fn binary_search(&self, search_value: bool) -> isize {
        // Convert &Vec<bool> into &[i64] for ArrayUtil::binary_search_index
        let data_i64: Vec<i64> = self.data.iter().map(|&v| v as i64).collect();
        ArrayUtil::binary_search_index(&data_i64, data_i64.len(), search_value as i64)
    }
}

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
pub struct PagedHugeBooleanArray {
    pages: Vec<Vec<bool>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeBooleanArray {
    fn new(size: usize) -> Self {
        // Calculate page size for bool elements with 4KB pages
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<bool>());
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
            pages.push(vec![false; page_length]);
        }

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> bool {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: bool) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> bool,
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

    fn fill(&mut self, value: bool) {
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
            total += page.len() * std::mem::size_of::<bool>();
        }
        total
    }

    fn binary_search(&self, search_value: bool) -> isize {
        // For paged arrays, we need to search across all pages
        // This is a simplified implementation - could be optimized
        let mut low = 0isize;
        let mut high = (self.size - 1) as isize;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_val = self.get(mid as usize);

            if !mid_val & search_value {
                low = mid + 1;
            } else if mid_val & !search_value {
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
        let array = HugeBooleanArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), false);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeBooleanArray::new(10);
        array.set(5, true);
        assert_eq!(array.get(5), true);
        assert_eq!(array.get(4), false);
    }

    #[test]
    fn test_fill() {
        let mut array = HugeBooleanArray::new(100);
        array.fill(true);
        assert_eq!(array.get(0), true);
        assert_eq!(array.get(99), true);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeBooleanArray::new(5);
        array.set_all(|i| i % 2 == 0);
        assert_eq!(array.get(0), true);  // 0 % 2 == 0
        assert_eq!(array.get(1), false); // 1 % 2 != 0
        assert_eq!(array.get(2), true);  // 2 % 2 == 0
        assert_eq!(array.get(3), false); // 3 % 2 != 0
        assert_eq!(array.get(4), true);  // 4 % 2 == 0
    }

    #[test]
    fn test_from_vec() {
        let array = HugeBooleanArray::from_vec(vec![true, false, true]);
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), true);
        assert_eq!(array.get(1), false);
        assert_eq!(array.get(2), true);
    }

    #[test]
    fn test_to_vec() {
        let mut array = HugeBooleanArray::new(3);
        array.set(0, true);
        array.set(1, false);
        array.set(2, true);
        let vec = array.to_vec();
        assert_eq!(vec, vec![true, false, true]);
    }

    #[test]
    fn test_copy_of() {
        let mut original = HugeBooleanArray::new(3);
        original.set(0, true);
        original.set(1, false);
        original.set(2, true);

        let copy = original.copy_of(5);
        assert_eq!(copy.size(), 5);
        assert_eq!(copy.get(0), true);
        assert_eq!(copy.get(1), false);
        assert_eq!(copy.get(2), true);
        assert_eq!(copy.get(3), false); // Default value
        assert_eq!(copy.get(4), false); // Default value
    }

    #[test]
    fn test_binary_search() {
        let array = HugeBooleanArray::from_vec(vec![false, false, true, true, true]);
        assert_eq!(array.binary_search(true), 2);
        assert_eq!(array.binary_search(false), 0);
        assert!(array.binary_search(true) >= 0); // Found
    }

    #[test]
    fn test_iter() {
        let mut array = HugeBooleanArray::new(5);
        array.set_all(|i| i % 2 == 0);

        let count: usize = array.iter().filter(|&b| b).count();
        assert_eq!(count, 3); // true at indices 0, 2, 4
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to use paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array = HugeBooleanArray::new(size);

        array.set(0, true);
        array.set(MAX_ARRAY_LENGTH, false);
        array.set(size - 1, true);

        assert_eq!(array.get(0), true);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), false);
        assert_eq!(array.get(size - 1), true);
    }

    // Cursor tests

    #[test]
    fn test_cursor_basic_iteration() {
        let mut array = HugeBooleanArray::new(100);
        array.set_all(|i| i % 2 == 0);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                if page[i] {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 50); // Every other element is true
    }

    #[test]
    fn test_cursor_range_iteration() {
        let mut array = HugeBooleanArray::new(100);
        array.set_all(|i| i % 2 == 0);

        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 10, 20);

        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                if page[i] {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 5); // Elements 10, 12, 14, 16, 18 are true
    }

    #[test]
    fn test_cursor_empty_range() {
        let array = HugeBooleanArray::new(100);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 50, 50);

        assert!(!cursor.next()); // Empty range
    }

    #[test]
    fn test_cursor_reset() {
        let mut array = HugeBooleanArray::new(10);
        array.set_all(|i| i % 2 == 0);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        // First iteration
        assert!(cursor.next());

        // Reset and iterate again
        cursor.reset();
        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                if page[i] {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 5); // Elements 0, 2, 4, 6, 8 are true
    }
}

// Collections impl via macro
use crate::huge_collections;
use crate::types::ValueType;
huge_collections!(
    HugeBooleanArray,
    bool,
    ValueType::Boolean,
    false,
    kind: OrdNoAgg,
    [],
    [],
    "Collections impl for HugeBooleanArray"
);
