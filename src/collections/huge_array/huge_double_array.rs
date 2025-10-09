//! HugeDoubleArray - f64 array supporting billions of elements
//!
//! Floating-point optimized variant for storing double-precision values efficiently
//! while supporting massive datasets that exceed standard array limitations.

use crate::collections::PageUtil;

/// Maximum size for single-page arrays
const MAX_ARRAY_LENGTH: usize = 1 << 28;

/// A long-indexable f64 array that can contain more than 2 billion elements.
///
/// Designed for high-performance storage of continuous numeric data such as weights,
/// scores, distances, and other floating-point computations in graph analytics.
///
/// # Characteristics
///
/// - **Fixed size**: Cannot grow or shrink after creation
/// - **Dense storage**: Every position consumes memory (use sparse variants for sparse data)
/// - **Zero default**: Unset values return `0.0`
/// - **IEEE 754**: Full double-precision floating-point support
///
/// # Examples
///
/// ```
/// use rust_gds::collections::huge_array::HugeDoubleArray;
///
/// // Store PageRank scores
/// let mut scores = HugeDoubleArray::new(1_000_000);
/// scores.fill(1.0 / 1_000_000.0);
/// scores.set(0, 0.5);
/// assert_eq!(scores.get(0), 0.5);
/// ```
pub enum HugeDoubleArray {
    /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
    Single(SingleHugeDoubleArray),
    /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
    Paged(PagedHugeDoubleArray),
}

impl HugeDoubleArray {
    /// Creates a new array of the given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::huge_array::HugeDoubleArray;
    ///
    /// let scores = HugeDoubleArray::new(1_000_000);
    /// assert_eq!(scores.size(), 1_000_000);
    /// ```
    pub fn new(size: usize) -> Self {
        if size <= MAX_ARRAY_LENGTH {
            Self::Single(SingleHugeDoubleArray::new(size))
        } else {
            Self::Paged(PagedHugeDoubleArray::new(size))
        }
    }

    /// Creates a new array from the provided values.
    pub fn from_vec(values: Vec<f64>) -> Self {
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
    pub fn get(&self, index: usize) -> f64 {
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
    pub fn set(&mut self, index: usize, value: f64) {
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
    /// use rust_gds::collections::huge_array::HugeDoubleArray;
    ///
    /// let mut array = HugeDoubleArray::new(10);
    /// array.set(0, 1.5);
    /// array.add_to(0, 2.5);
    /// assert_eq!(array.get(0), 4.0);
    /// ```
    pub fn add_to(&mut self, index: usize, delta: f64) {
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
    /// use rust_gds::collections::huge_array::HugeDoubleArray;
    ///
    /// let mut array = HugeDoubleArray::new(5);
    /// array.set_all(|i| i as f64 * 0.5);
    /// assert_eq!(array.get(0), 0.0);
    /// assert_eq!(array.get(2), 1.0);
    /// assert_eq!(array.get(4), 2.0);
    /// ```
    pub fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f64,
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
    /// use rust_gds::collections::huge_array::HugeDoubleArray;
    ///
    /// let mut array = HugeDoubleArray::new(100);
    /// array.fill(3.14159);
    /// assert_eq!(array.get(50), 3.14159);
    /// ```
    pub fn fill(&mut self, value: f64) {
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
    pub fn copy_to(&self, dest: &mut HugeDoubleArray, length: usize) {
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
    pub fn to_vec(&self) -> Vec<f64> {
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
    /// use rust_gds::collections::huge_array::HugeDoubleArray;
    ///
    /// let mut array = HugeDoubleArray::new(5);
    /// array.set_all(|i| i as f64);
    ///
    /// let sum: f64 = array.iter().sum();
    /// assert_eq!(sum, 10.0); // 0 + 1 + 2 + 3 + 4
    /// ```
    pub fn iter(&self) -> HugeDoubleArrayIter<'_> {
        HugeDoubleArrayIter {
            array: self,
            index: 0,
        }
    }
}

/// Iterator for HugeDoubleArray
pub struct HugeDoubleArrayIter<'a> {
    array: &'a HugeDoubleArray,
    index: usize,
}

impl<'a> Iterator for HugeDoubleArrayIter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.index < self.array.size() {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

/// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
pub struct SingleHugeDoubleArray {
    data: Vec<f64>,
}

impl SingleHugeDoubleArray {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    fn get(&self, index: usize) -> f64 {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }

    fn add_to(&mut self, index: usize, delta: f64) {
        self.data[index] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f64,
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            *value = gen(i);
        }
    }

    fn fill(&mut self, value: f64) {
        self.data.fill(value);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn size_of(&self) -> usize {
        self.data.len() * std::mem::size_of::<f64>()
    }
}

/// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
pub struct PagedHugeDoubleArray {
    pages: Vec<Vec<f64>>,
    size: usize,
    page_shift: u32,
    page_mask: usize,
}

impl PagedHugeDoubleArray {
    fn new(size: usize) -> Self {
        // Calculate page size for f64 elements with 4KB pages
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<f64>());
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

    fn get(&self, index: usize) -> f64 {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page]
    }

    fn set(&mut self, index: usize, value: f64) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] = value;
    }

    fn add_to(&mut self, index: usize, delta: f64) {
        let page_index = PageUtil::page_index(index, self.page_shift);
        let index_in_page = PageUtil::index_in_page(index, self.page_mask);
        self.pages[page_index][index_in_page] += delta;
    }

    fn set_all<F>(&mut self, gen: F)
    where
        F: Fn(usize) -> f64,
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

    fn fill(&mut self, value: f64) {
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
            total += page.len() * std::mem::size_of::<f64>();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_small() {
        let array = HugeDoubleArray::new(100);
        assert_eq!(array.size(), 100);
        assert_eq!(array.get(0), 0.0);
    }

    #[test]
    fn test_get_set() {
        let mut array = HugeDoubleArray::new(10);
        array.set(5, 3.14159);
        assert_eq!(array.get(5), 3.14159);
    }

    #[test]
    fn test_add_to() {
        let mut array = HugeDoubleArray::new(10);
        array.set(0, 1.5);
        array.add_to(0, 2.5);
        assert_eq!(array.get(0), 4.0);
    }

    #[test]
    fn test_fill() {
        let mut array = HugeDoubleArray::new(100);
        array.fill(2.71828);
        assert_eq!(array.get(0), 2.71828);
        assert_eq!(array.get(99), 2.71828);
    }

    #[test]
    fn test_set_all() {
        let mut array = HugeDoubleArray::new(5);
        array.set_all(|i| i as f64 * 0.5);
        assert_eq!(array.get(0), 0.0);
        assert_eq!(array.get(2), 1.0);
        assert_eq!(array.get(4), 2.0);
    }

    #[test]
    fn test_from_vec() {
        let array = HugeDoubleArray::from_vec(vec![1.0, 2.0, 3.0]);
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), 1.0);
        assert_eq!(array.get(1), 2.0);
        assert_eq!(array.get(2), 3.0);
    }

    #[test]
    fn test_to_vec() {
        let mut array = HugeDoubleArray::new(3);
        array.set(0, 1.5);
        array.set(1, 2.5);
        array.set(2, 3.5);
        let vec = array.to_vec();
        assert_eq!(vec, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_copy_of() {
        let mut original = HugeDoubleArray::new(3);
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
        let mut array = HugeDoubleArray::new(5);
        array.set_all(|i| i as f64);

        let sum: f64 = array.iter().sum();
        assert_eq!(sum, 10.0);
    }

    #[test]
    fn test_paged_array() {
        // Create array large enough to use paging
        let size = MAX_ARRAY_LENGTH + 1000;
        let mut array = HugeDoubleArray::new(size);

        array.set(0, 100.5);
        array.set(MAX_ARRAY_LENGTH, 200.5);
        array.set(size - 1, 300.5);

        assert_eq!(array.get(0), 100.5);
        assert_eq!(array.get(MAX_ARRAY_LENGTH), 200.5);
        assert_eq!(array.get(size - 1), 300.5);
    }
}
