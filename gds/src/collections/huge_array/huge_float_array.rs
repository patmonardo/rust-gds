//! HugeFloatArray - f32 array supporting billions of elements
//!
//! Floating-point optimized variant for storing single-precision values efficiently
//! while supporting massive datasets that exceed standard array limitations.

use crate::collections::cursor::{HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor};
use crate::collections::PageUtil;
use crate::concurrency::Concurrency;
use crate::core::utils::paged::ParallelFloatPageCreator;

/// Maximum size for single-page arrays
const MAX_ARRAY_LENGTH: usize = 1 << 28;

/// A long-indexable f32 array that can contain more than 2 billion elements.
///
/// Designed for high-performance storage of continuous numeric data such as weights,
/// scores, distances, and other floating-point computations in graph analytics.
///
/// # Characteristics
///
/// - **Fixed size**: Cannot grow or shrink after creation
/// - **Dense storage**: Every position consumes memory (use sparse variants for sparse data)
/// - **Zero default**: Unset values return `0.0`
/// - **IEEE 754**: Full single-precision floating-point support
/// - **Cursor support**: Efficient zero-copy iteration over pages
///
/// # Examples
///
/// ```text
/// use gds::collections::huge_array::HugeFloatArray;
///
/// // Store PageRank scores
/// let mut scores = HugeFloatArray::new(1_000_000);
/// scores.fill(1.0 / 1_000_000.0);
/// scores.set(0, 0.5);
/// assert_eq!(scores.get(0), 0.5);
/// ```text
///
/// # Cursor-Based Iteration
///
/// ```
/// use gds::collections::huge_array::HugeFloatArray;
/// use gds::collections::cursor::{HugeCursor, init_cursor};
///
/// let mut array = HugeFloatArray::new(10000);
/// array.set_all(|i| i as f32 * 0.5);
///
/// let mut cursor = array.new_cursor();
/// init_cursor(&array, &mut cursor);
///
/// let mut sum = 0.0;
/// while cursor.next() {
///     let page = cursor.array().unwrap();
///     for i in cursor.offset()..cursor.limit() {
///         sum += page[i] as f64;
///     }
/// }
/// ```
pub enum HugeFloatArray {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeFloatArray),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeFloatArray),
}

impl HugeFloatArray {
    /// Creates a new array of the given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeFloatArray;
    ///
    /// let scores = HugeFloatArray::new(1_000_000);
    /// assert_eq!(scores.size(), 1_000_000);
    /// ```
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeFloatArray::new(size))
        } else {
            Self::Paged(PagedHugeFloatArray::new(size))
        }
    }

    /// Creates a new array from the provided values.
    pub fn from_vec(values: Vec<f32>) -> Self {
        let size = values.len();
        let mut array = Self::new(size);
        for (i, &value) in values.iter().enumerate() {
            array.set(i, value);
        }
        array
    }

    /// Creates a new array with values generated in parallel using the provided function.
    ///
    /// This method leverages parallel page creation for efficient initialization of large arrays,
    /// making it significantly faster than sequential initialization for arrays with millions or
    /// billions of elements.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of elements in the array
    /// * `concurrency` - Parallelism configuration (Sequential, Single, or specific worker count)
    /// * `generator` - Function that maps index to value: `Fn(usize) -> f32`
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeFloatArray;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Identity mapping
    /// let array = HugeFloatArray::with_generator(
    ///     1_000_000,
    ///     Concurrency::Single,
    ///     |i| i as f32
    /// );
    /// assert_eq!(array.get(42), 42.0);
    ///
    /// // Custom computation
    /// let weights = HugeFloatArray::with_generator(
    ///     10_000,
    ///     Concurrency::Single,
    ///     |i| (i as f32).sqrt()
    /// );
    /// assert_eq!(weights.get(100), 10.0);
    /// ```
    pub fn with_generator<F>(size: usize, concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> f32 + Send + Sync + 'static,
    {
        if size <= MAX_ARRAY_LENGTH {
            // Small array: use single-page variant
            let mut array = Self::Single(SingleHugeFloatArray::new(size));
            array.set_all(generator);
            array
        } else {
            // Large array: use parallel page creation
            let page_creator = ParallelFloatPageCreator::of(concurrency, generator);
            let pages = page_creator.create_pages(size);
            Self::Paged(PagedHugeFloatArray::from_pages(size, pages))
        }
    }

    /// Returns the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn get(&self, index: usize) -> f32 {
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
    pub fn set(&mut self, index: usize, value: f32) {
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
    /// use gds::collections::huge_array::HugeFloatArray;
    ///
    /// let mut array = HugeFloatArray::new(10);
    /// array.set(0, 1.5);
    /// array.add_to(0, 2.5);
    /// assert_eq!(array.get(0), 4.0);
    /// ```
    pub fn add_to(&mut self, index: usize, delta: f32) {
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
    /// use gds::collections::huge_array::HugeFloatArray;
    ///
    /// let mut array = HugeFloatArray::new(5);
    /// array.set_all(|i| i as f32 * 0.5);
    /// assert_eq!(array.get(0), 0.0);
    /// assert_eq!(array.get(2), 1.0);
    /// assert_eq!(array.get(4), 2.0);
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f32,
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
    /// use gds::collections::huge_array::HugeFloatArray;
    ///
    /// let mut array = HugeFloatArray::new(100);
    /// array.fill(3.14159);
    /// assert_eq!(array.get(50), 3.14159);
    /// ```
    pub fn fill(&mut self, value: f32) {
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
    pub fn copy_to(&self, dest: &mut HugeFloatArray, length: usize) {
        assert!(length <= self.size(), "length exceeds source array size");
        assert!(length <= dest.size(), "length exceeds dest array size");

        for i in 0..length {
            dest.set(i, self.get(i));
        }
    }

    /// Creates a copy of this array with a new length.
    ///
    /// If new length is larger, new elements are initialized to 0.0.
    pub fn copy_of(&self, new_length: usize) -> Self {
        let mut result = Self::new(new_length);
        let copy_length = usize::min(self.size(), new_length);
        self.copy_to(&mut result, copy_length);
        result
    }

    /// Converts to a standard Vec.
    pub fn to_vec(&self) -> Vec<f32> {
        let size = self.size();
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.get(i));
        }
        result
    }

    /// Creates an iterator over all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeFloatArray;
    ///
    /// let mut array = HugeFloatArray::new(5);
    /// array.set_all(|i| i as f32);
    ///
    /// let sum: f32 = array.iter().sum();
    /// assert_eq!(sum, 10.0); // 0 + 1 + 2 + 3 + 4
    /// ```
    pub fn iter(&self) -> HugeFloatArrayIter<'_> {
        HugeFloatArrayIter {
            array: self,
            index: 0,
        }
    }

    /// Inherent helper so callers (and doctests) can call `new_cursor()` without
    /// importing the `HugeCursorSupport` trait.
    pub fn new_cursor(&self) -> HugeFloatArrayCursor<'_> {
        match self {
            Self::Single(arr) => HugeFloatArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeFloatArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

/// Iterator for HugeFloatArray
pub struct HugeFloatArrayIter<'a> {
    array: &'a HugeFloatArray,
    index: usize,
}

impl<'a> Iterator for HugeFloatArrayIter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

// Cursor support for HugeFloatArray
impl<'a> HugeCursorSupport<'a> for HugeFloatArray {
    type Cursor = HugeFloatArrayCursor<'a>;

    fn size(&self) -> usize {
        HugeFloatArray::size(self)
    }

    fn new_cursor(&'a self) -> Self::Cursor {
        match self {
            HugeFloatArray::Single(arr) => {
                HugeFloatArrayCursor::Single(SinglePageCursor::new(&arr.data))
            }
            HugeFloatArray::Paged(arr) => {
                let capacity = arr.size;
                HugeFloatArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

/// Cursor implementation for HugeFloatArray
pub enum HugeFloatArrayCursor<'a> {
    Single(SinglePageCursor<'a, f32>),
    Paged(PagedCursor<'a, f32>),
}

impl<'a> HugeCursor<'a> for HugeFloatArrayCursor<'a> {
    type Array = [f32];

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
pub struct SingleHugeFloatArray {
    data: Vec<f32>,
}

impl SingleHugeFloatArray {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    fn get(&self, index: usize) -> f32 {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: f32) {
        self.data[index] = value;
    }

    fn add_to(&mut self, index: usize, delta: f32) {
        self.data[index] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f32,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: f32) {
        self.data.fill(value);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<f32>()
    }
}

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
pub struct PagedHugeFloatArray {
    pages: Vec<Vec<f32>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeFloatArray {
    fn new(size: usize) -> Self {
        // Calculate page size for f32 elements with 4KB pages
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<f32>());
        let page_shift = page_size.trailing_zeros();
        let page_mask = page_size - 1;
        let num_pages = PageUtil::num_pages_for(size, page_size);

        let mut pages = Vec::with_capacity(num_pages);
        for page_index in 0..num_pages {
            let page_length = if page_index == num_pages - 1 {
                PageUtil::exclusive_index_of_page(size, page_mask)
            } else {
                page_size
            };
            pages.push(vec![0.0; page_length]);
        }

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    /// Creates a new paged array from pre-populated pages.
    ///
    /// Used internally by `with_generator` after parallel page creation.
    /// Pages must be properly sized according to PageUtil calculations.
    /// Uses 32KB pages to match ParallelFloatPageCreator.
    fn from_pages(size: usize, pages: Vec<Vec<f32>>) -> Self {
        // Determine page size from the actual filled page length (first page).
        // Fall back to the default page size if pages are empty or the first page has zero length.
        let page_size = if !pages.is_empty() && !pages[0].is_empty() {
            pages[0].len()
        } else {
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<f32>())
        };
        let page_shift = page_size.trailing_zeros();
        let page_mask = page_size - 1;

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> f32 {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: f32) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] = value;
    }

    fn add_to(&mut self, index: usize, delta: f32) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f32,
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

    fn fill(&mut self, value: f32) {
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
            total += page.len() * std::mem::size_of::<f32>();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_small() {
        let array = HugeFloatArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0.0);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeFloatArray::new(10);
        let test_val = 3.15159; // Not PI, just a test value
        array.set(5, test_val);
        assert_eq!(array.get(5), test_val);
    }

    #[test]
    fn test_add_to() {
        let mut array = HugeFloatArray::new(10);
        array.set(0, 1.5);
        array.add_to(0, 2.5);
        assert_eq!(array.get(0), 4.0);
    }

    #[test]
    fn test_fill() {
        let mut array = HugeFloatArray::new(100);
        let test_val = 2.72828; // Not E, just a test value
        array.fill(test_val);
        assert_eq!(array.get(0), test_val);
        assert_eq!(array.get(99), test_val);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeFloatArray::new(5);
        array.set_all(|i| i as f32 * 0.5);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(2), 1.0);
        assert_eq!(array.get(4), 2.0);
    }

    #[test]
    fn test_from_vec() {
        let array = HugeFloatArray::from_vec(vec![1.0, 2.0, 3.0]);
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), 1.0);
        assert_eq!(array.get(1), 2.0);
        assert_eq!(array.get(2), 3.0);
    }

    #[test]
    fn test_to_vec() {
        let mut array = HugeFloatArray::new(3);
        array.set(0, 1.5);
        array.set(1, 2.5);
        array.set(2, 3.5);
        let vec = array.to_vec();
        assert_eq!(vec, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_copy_of() {
        let mut original = HugeFloatArray::new(3);
        original.set(0, 1.1);
        original.set(1, 2.2);
        original.set(2, 3.3);

        let copy = original.copy_of(5);
        assert_eq!(copy.size(), 5);
        assert_eq!(copy.get(0), 1.1);
        assert_eq!(copy.get(1), 2.2);
        assert_eq!(copy.get(2), 3.3);
        assert_eq!(copy.get(3), 0.0);
        assert_eq!(copy.get(4), 0.0);
    }

    #[test]
    fn test_iter() {
        let mut array = HugeFloatArray::new(5);
        array.set_all(|i| i as f32);

        let sum: f32 = array.iter().sum();
        assert_eq!(sum, 10.0);
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to use paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array = HugeFloatArray::new(size);

        array.set(0, 100.5);
        array.set(MAX_ARRAY_LENGTH, 200.5);
        array.set(size - 1, 300.5);

        assert_eq!(array.get(0), 100.5);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 200.5);
        assert_eq!(array.get(size - 1), 300.5);
    }

    // Cursor tests

    #[test]
    fn test_cursor_basic_iteration() {
        use crate::collections::cursor::init_cursor;

        let mut array = HugeFloatArray::new(100);
        array.set_all(|i| i as f32);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        let mut sum = 0.0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i] as f64;
            }
        }

        assert_eq!(sum, 4950.0); // Sum of 0..99
    }

    #[test]
    fn test_cursor_range_iteration() {
        use crate::collections::cursor::init_cursor_range;

        let mut array = HugeFloatArray::new(100);
        array.set_all(|i| i as f32);

        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 10, 20);

        let mut sum = 0.0;
        let mut count = 0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i] as f64;
                count += 1;
            }
        }

        assert_eq!(count, 10); // Elements 10-19
        assert_eq!(sum, 145.0); // Sum of 10..19
    }

    #[test]
    fn test_cursor_empty_range() {
        use crate::collections::cursor::init_cursor_range;

        let array = HugeFloatArray::new(100);
        let mut cursor = array.new_cursor();
        init_cursor_range(&array, &mut cursor, 50, 50);

        assert!(!cursor.next()); // Empty range
    }

    #[test]
    fn test_cursor_reset() {
        use crate::collections::cursor::init_cursor;

        let mut array = HugeFloatArray::new(10);
        array.set_all(|i| i as f32);

        let mut cursor = array.new_cursor();
        init_cursor(&array, &mut cursor);

        // First iteration
        assert!(cursor.next());

        // Reset and iterate again
        cursor.reset();
        let mut sum = 0.0;
        while cursor.next() {
            let page = cursor.array().unwrap();
            for i in cursor.offset()..cursor.limit() {
                sum += page[i] as f64;
            }
        }

        assert_eq!(sum, 45.0); // Sum of 0..9
    }

    #[test]
    fn test_with_generator_small_array() {
        use crate::concurrency::Concurrency;

        // Small array should use single-page implementation
        let array = HugeFloatArray::with_generator(1000, Concurrency::of(4), |i| i as f32);

        assert_eq!(array.size(), 1000);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(500), 500.0);
        assert_eq!(array.get(999), 999.0);

        // Verify it's actually single-page variant
        assert!(matches!(array, HugeFloatArray::Single(_)));
    }

    #[test]
    fn test_with_generator_large_array() {
        use crate::concurrency::Concurrency;

        // Large array should use paged implementation
        let size = MAX_ARRAY_LENGTH + 10000;
        let array = HugeFloatArray::with_generator(size, Concurrency::of(4), |i| i as f32);

        assert_eq!(array.size(), size);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), MAX_ARRAY_LENGTH as f32);
        assert_eq!(array.get(size - 1), (size - 1) as f32);

        // Verify it's actually paged variant
        assert!(matches!(array, HugeFloatArray::Paged(_)));
    }

    #[test]
    fn test_with_generator_custom_computation() {
        use crate::concurrency::Concurrency;

        // Test with sqrt computation
        let size = 10_000;
        let array =
            HugeFloatArray::with_generator(size, Concurrency::of(4), |i| (i as f32).sqrt());

        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(100), 10.0);
        assert_eq!(array.get(10000 - 1), (9999.0_f32).sqrt());
    }

    #[test]
    fn test_with_generator_parallel_consistency() {
        use crate::concurrency::Concurrency;

        // Test that different concurrency levels produce same results
        let size = 100_000;

        let array1 =
            HugeFloatArray::with_generator(size, Concurrency::of(1), |i| (i as f32) * 0.5);
        let array2 =
            HugeFloatArray::with_generator(size, Concurrency::of(4), |i| (i as f32) * 0.5);
        let array8 =
            HugeFloatArray::with_generator(size, Concurrency::of(8), |i| (i as f32) * 0.5);

        // Spot check several indices
        for idx in [0, 1000, 50000, 99999] {
            let expected = (idx as f32) * 0.5;
            assert_eq!(array1.get(idx), expected);
            assert_eq!(array2.get(idx), expected);
            assert_eq!(array8.get(idx), expected);
        }
    }

    #[test]
    fn test_with_generator_million_elements() {
        use crate::concurrency::Concurrency;

        // Test with 10 million elements
        let size = 10_000_000;
        let array = HugeFloatArray::with_generator(size, Concurrency::of(8), |i| {
            if i % 1_000_000 == 0 {
                i as f32
            } else {
                0.0
            }
        });

        assert_eq!(array.size(), size);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(5_000_000), 5_000_000.0);
        assert_eq!(array.get(9_999_999), 0.0);
    }

    #[test]
    fn test_with_generator_identity_mapping() {
        use crate::concurrency::Concurrency;

        let array = HugeFloatArray::with_generator(1000, Concurrency::of(4), |i| i as f32);

        for i in 0..1000 {
            assert_eq!(array.get(i), i as f32);
        }
    }

    #[test]
    fn test_with_generator_constant_values() {
        use crate::concurrency::Concurrency;

        let array = HugeFloatArray::with_generator(1000, Concurrency::of(4), |_| 3.14159);

        for i in 0..1000 {
            assert_eq!(array.get(i), 3.14159);
        }
    }

    #[test]
    fn test_with_generator_zero_values() {
        use crate::concurrency::Concurrency;

        let array = HugeFloatArray::with_generator(1000, Concurrency::of(4), |_| 0.0);

        for i in 0..1000 {
            assert_eq!(array.get(i), 0.0);
        }
    }

    #[test]
    fn test_with_generator_alternating_pattern() {
        use crate::concurrency::Concurrency;

        let array = HugeFloatArray::with_generator(100, Concurrency::of(4), |i| {
            if i % 2 == 0 {
                1.0
            } else {
                -1.0
            }
        });

        assert_eq!(array.get(0), 1.0);
        assert_eq!(array.get(1), -1.0);
        assert_eq!(array.get(50), 1.0);
        assert_eq!(array.get(51), -1.0);
    }
}
