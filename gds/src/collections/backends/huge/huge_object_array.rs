//! HugeObjectArray<T> - Generic array supporting billions of elements
//!
//! Provides storage for arbitrary types (reference types, structs, enums) with the same
//! massive-scale capabilities as HugeDoubleArray and HugeLongArray.

use crate::collections::cursor::{HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor};
use crate::collections::PageUtil;

/// Maximum size for single-page arrays
const MAX_ARRAY_LENGTH: usize = 1 << 28;

/// A long-indexable array of type T that can contain more than 2 billion elements.
///
/// Designed for storing complex objects, message queues, arrays, or any non-primitive
/// type that needs massive-scale storage.
///
/// # Characteristics
///
/// - **Fixed size**: Cannot grow or shrink after creation
/// - **Dense storage**: Every position consumes memory
/// - **Default initialized**: Uses `T::default()` for unset values
/// - **Generic**: Works with any `T: Default + Clone`
/// - **Cursor support**: Efficient zero-copy iteration over pages
///
/// # Examples
///
/// ```
/// use gds::collections::huge_array::HugeObjectArray;
///
/// // Store message queues
/// let mut queues: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(1_000_000);
/// queues.set(0, vec![1, 2, 3]);
/// assert_eq!(queues.get(0), &vec![1, 2, 3]);
/// ```
///
/// # Type Requirements
///
/// - `T: Default` - for initialization
/// - `T: Clone` - for setting values
pub enum HugeObjectArray<T: Default + Clone> {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeObjectArray<T>),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeObjectArray<T>),
}

impl<T: Default + Clone> HugeObjectArray<T> {
    /// Creates a new array of the given size, initialized with T::default().
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeObjectArray::new(size))
        } else {
            Self::Paged(PagedHugeObjectArray::new(size))
        }
    }

    /// Convenience so callers (and doctests) can call [new_cursor()](http://_vscodecontentref_/1) without
    /// importing the [HugeCursorSupport](http://_vscodecontentref_/2) trait.
    pub fn new_cursor(&self) -> HugeObjectArrayCursor<'_, T> {
        match self {
            Self::Single(arr) => HugeObjectArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeObjectArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }

    /// Creates a new array from the provided values.
    pub fn from_vec(values: Vec<T>) -> Self {
        let size = values.len();
        let mut array = Self::new(size);
        for (i, value) in values.into_iter().enumerate() {
            array.set(i, value);
        }
        array
    }

    /// Returns a reference to the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn get(&self, index: usize) -> &T {
        match self {
            Self::Single(arr) => arr.get(index),
            Self::Paged(arr) => arr.get(index),
        }
    }

    /// Returns a mutable reference to the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        match self {
            Self::Single(arr) => arr.get_mut(index),
            Self::Paged(arr) => arr.get_mut(index),
        }
    }

    /// Sets the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index >= size
    pub fn set(&mut self, index: usize, value: T) {
        match self {
            Self::Single(arr) => arr.set(index, value),
            Self::Paged(arr) => arr.set(index, value),
        }
    }

    /// Sets all elements using a generator function.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeObjectArray;
    ///
    /// let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(100);
    /// array.set_all(|i| vec![i as i64]);
    /// assert_eq!(array.get(5), &vec![5]);
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> T,
    {
        match self {
            Self::Single(arr) => arr.set_all(gen),
            Self::Paged(arr) => arr.set_all(gen),
        }
    }

    /// Fills all elements with clones of the specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeObjectArray;
    ///
    /// let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(100);
    /// array.fill(vec![1, 2, 3]);
    /// assert_eq!(array.get(50), &vec![1, 2, 3]);
    /// ```
    pub fn fill(&mut self, value: T) {
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
    ///
    /// Note: This is an approximation. For complex types, it returns
    /// `size * size_of::<T>()` which may not account for heap allocations
    /// within T (e.g., Vec's internal buffer).
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
    pub fn copy_to(&self, dest: &mut HugeObjectArray<T>, length: usize) {
        assert!(length <= self.size(), "length exceeds source array size");
        assert!(length <= dest.size(), "length exceeds dest array size");

        for i in 0..length {
            dest.set(i, self.get(i).clone());
        }
    }

    /// Creates a copy of this array with a new length.
    ///
    /// If new length is larger, new elements are initialized to T::default().
    pub fn copy_of(&self, new_length: usize) -> Self {
        let mut result = Self::new(new_length);
        let copy_length = usize::min(self.size(), new_length);
        self.copy_to(&mut result, copy_length);
        result
    }

    /// Converts to a standard Vec.
    pub fn to_vec(&self) -> Vec<T> {
        let size = self.size();
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.get(i).clone());
        }
        result
    }

    /// Creates an iterator over references to all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::huge_array::HugeObjectArray;
    ///
    /// let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(5);
    /// array.set_all(|i| vec![i as i64]);
    ///
    /// let lengths: Vec<usize> = array.iter().map(|v| v.len()).collect();
    /// assert_eq!(lengths, vec![1, 1, 1, 1, 1]);
    /// ```
    pub fn iter(&self) -> HugeObjectArrayIter<'_, T> {
        HugeObjectArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Iterator for HugeObjectArray
pub struct HugeObjectArrayIter<'a, T: Default + Clone> {
    array: &'a HugeObjectArray<T>,
    index: usize,
}

impl<'a, T: Default + Clone> Iterator for HugeObjectArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.array.size() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T: Default + Clone> ExactSizeIterator for HugeObjectArrayIter<'a, T> {}

impl<'a, T: Default + Clone + 'a> HugeCursorSupport<'a> for HugeObjectArray<T> {
    type Cursor = HugeObjectArrayCursor<'a, T>;

    fn size(&self) -> usize {
        HugeObjectArray::size(self)
    }

    fn new_cursor(&'a self) -> Self::Cursor {
        match self {
            Self::Single(arr) => HugeObjectArrayCursor::Single(SinglePageCursor::new(&arr.data)),
            Self::Paged(arr) => {
                let capacity = arr.size;
                HugeObjectArrayCursor::Paged(PagedCursor::new(&arr.pages, capacity))
            }
        }
    }
}

pub enum HugeObjectArrayCursor<'a, T: Default + Clone> {
    Single(SinglePageCursor<'a, T>),
    Paged(PagedCursor<'a, T>),
}

impl<'a, T: Default + Clone> HugeCursor<'a> for HugeObjectArrayCursor<'a, T> {
    type Array = [T];

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
pub struct SingleHugeObjectArray<T: Default + Clone> {
    data: Vec<T>,
}

impl<T: Default + Clone> SingleHugeObjectArray<T> {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(T::default());
        }
        Self { data }
    }

    fn get(&self, index: usize) -> &T {
        &self.data[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }

    fn set(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> T,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: T) {
        for item in self.data.iter_mut() {
            *item = value.clone();
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<T>()
    }
}

// SingleHugeObjectArray does not need HugeCursorSupport directly;
// it's only used through the HugeObjectArray enum.

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
pub struct PagedHugeObjectArray<T: Default + Clone> {
    pages: Vec<Vec<T>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl<T: Default + Clone> PagedHugeObjectArray<T> {
    fn new(size: usize) -> Self {
        // For object arrays, use a fixed conservative page size of 4096 elements
        // (not 4KB memory) to avoid issues with non-power-of-2 sized types.
        // This is similar to Java GDS HugeObjectArray which uses a fixed page size.
        let page_size: usize = 4096; // elements per page
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

            let mut page = Vec::with_capacity(page_length);
            for _ in 0..page_length {
                page.push(T::default());
            }
            pages.push(page);
        }

        Self {
            pages,
            size,
            page_shift,
            page_mask,
        }
    }

    fn get(&self, index: usize) -> &T {
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        &self.pages[page_index][index_in_page]
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        &mut self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: T) {
        let page_index = index >> self.page_shift;
        let index_in_page = index & self.page_mask;
        self.pages[page_index][index_in_page] = value;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> T,
    {
        let mut global_index = 0;
        for page in &mut self.pages {
            for value in page.iter_mut() {
                *value = gen(global_index);
                global_index += 1;
            }
        }
    }

    fn fill(&mut self, value: T) {
        for page in &mut self.pages {
            for item in page.iter_mut() {
                *item = value.clone();
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn size_of(&self) -> usize {
        self.size * std::mem::size_of::<T>()
    }
}

// PagedHugeObjectArray does not need HugeCursorSupport directly;
// it's only used through the HugeObjectArray enum.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_page_array() {
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(100);
        assert_eq!(array.size(), 100);

        array.set(0, vec![1, 2, 3]);
        array.set(50, vec![4, 5, 6, 7]);

        assert_eq!(array.get(0), &vec![1, 2, 3]);
        assert_eq!(array.get(50), &vec![4, 5, 6, 7]);
        assert_eq!(array.get(99), &Vec::<i64>::new()); // Default
    }

    #[test]
    fn test_set_all() {
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(10);
        array.set_all(|i| vec![i as i64, (i * 2) as i64]);

        assert_eq!(array.get(0), &vec![0, 0]);
        assert_eq!(array.get(5), &vec![5, 10]);
        assert_eq!(array.get(9), &vec![9, 18]);
    }

    #[test]
    fn test_fill() {
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(10);
        array.fill(vec![42]);

        assert_eq!(array.get(0), &vec![42]);
        assert_eq!(array.get(5), &vec![42]);
        assert_eq!(array.get(9), &vec![42]);
    }

    #[test]
    fn test_get_mut() {
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(10);
        array.set(5, vec![1, 2, 3]);

        {
            let vec_ref = array.get_mut(5);
            vec_ref.push(4);
        }

        assert_eq!(array.get(5), &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_iter() {
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(5);
        array.set_all(|i| vec![i as i64]);

        let collected: Vec<Vec<i64>> = array.iter().cloned().collect();
        assert_eq!(collected, vec![vec![0], vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_copy_operations() {
        let mut source: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(10);
        source.set_all(|i| vec![i as i64]);

        let copy = source.copy_of(15);
        assert_eq!(copy.size(), 15);
        assert_eq!(copy.get(0), &vec![0]);
        assert_eq!(copy.get(9), &vec![9]);
        assert_eq!(copy.get(14), &Vec::<i64>::new()); // Default
    }

    #[test]
    fn test_to_vec() {
        let mut array: HugeObjectArray<String> = HugeObjectArray::new(3);
        array.set(0, "hello".to_string());
        array.set(1, "world".to_string());
        array.set(2, "!".to_string());

        let vec = array.to_vec();
        assert_eq!(vec, vec!["hello", "world", "!"]);
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to trigger paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array: HugeObjectArray<Vec<i64>> = HugeObjectArray::new(size);

        array.set(0, vec![1]);
        array.set(MAX_ARRAY_LENGTH, vec![2]);
        array.set(size - 1, vec![3]);

        assert_eq!(array.get(0), &vec![1]);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), &vec![2]);
        assert_eq!(array.get(size - 1), &vec![3]);
    }
}
