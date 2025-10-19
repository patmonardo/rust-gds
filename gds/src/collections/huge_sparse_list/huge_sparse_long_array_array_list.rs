use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/// Number of elements in a single page (4096 elements = 4KB for most types)
const PAGE_SIZE: usize = 4096;
/// Number of bits to shift for page index calculation
const PAGE_SHIFT: usize = 12; // log2(4096)
/// Mask to extract offset within a page
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// A long-indexable mutable sparse list of Vec<Vec<i64>> (triple dimension!) that can contain
/// more than 2 billion elements.
///
/// Uses a HashMap to track pages, only allocating memory for pages that contain values.
/// Perfect for dynamic sparse collections of 2D long arrays (matrices, grids, etc.).
///
/// **Memory Efficiency**: Only allocates 4KB pages where Vec<Vec<i64>> values actually exist.
/// **Mutability**: Can be modified after creation using set().
/// **NOT Thread-Safe**: Uses RefCell for interior mutability - not safe for concurrent access.
/// **Default Values**: Returns a clone of the default Vec<Vec<i64>> for unset indices.
///
/// # Examples
///
/// ```
/// use rust_gds::collections::HugeSparseLongArrayArrayList;
///
/// let list = HugeSparseLongArrayArrayList::of(vec![]);
///
/// list.set(0, vec![vec![1, 2], vec![3, 4]]);
/// list.set(1_000_000, vec![vec![5]]);
///
/// assert_eq!(list.get(0), vec![vec![1, 2], vec![3, 4]]);
/// assert_eq!(list.get(1_000_000), vec![vec![5]]);
/// assert_eq!(list.get(50), Vec::<Vec<i64>>::new()); // default
/// ```
#[derive(Debug)]
pub struct HugeSparseLongArrayArrayList {
    /// Maximum capacity (highest index + 1)
    capacity: RefCell<usize>,
    /// Default Vec<Vec<i64>> returned for unset indices
    default_value: Vec<Vec<i64>>,
    /// HashMap storing only allocated pages
    pages: RefCell<HashMap<usize, Vec<Vec<Vec<i64>>>>>,
    /// HashSet tracking which indices have been explicitly set
    set_indices: RefCell<HashSet<usize>>,
}

impl HugeSparseLongArrayArrayList {
    /// Create a new sparse long array array list with the specified default value.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The Vec<Vec<i64>> returned for unset indices
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::HugeSparseLongArrayArrayList;
    ///
    /// let list = HugeSparseLongArrayArrayList::of(vec![]);
    /// list.set(42, vec![vec![1, 2], vec![3, 4]]);
    /// assert_eq!(list.get(42), vec![vec![1, 2], vec![3, 4]]);
    /// ```
    pub fn of(default_value: Vec<Vec<i64>>) -> Self {
        Self::with_capacity(default_value, 0)
    }

    /// Create a new sparse long array array list with an initial capacity hint.
    ///
    /// This can improve performance when the approximate maximum index is known,
    /// but the list will still grow dynamically if needed.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The Vec<Vec<i64>> returned for unset indices
    /// * `capacity` - Initial capacity hint
    pub fn with_capacity(default_value: Vec<Vec<i64>>, capacity: usize) -> Self {
        Self {
            capacity: RefCell::new(capacity),
            default_value,
            pages: RefCell::new(HashMap::new()),
            set_indices: RefCell::new(HashSet::new()),
        }
    }

    /// Get the current maximum number of values that can be stored.
    ///
    /// This is the highest index that has been set plus one.
    pub fn capacity(&self) -> usize {
        *self.capacity.borrow()
    }

    /// Check if a value has been explicitly set at the given index.
    ///
    /// This is useful to distinguish between "never set" and "set to default value".
    ///
    /// # Arguments
    ///
    /// * `index` - The index to check
    pub fn contains(&self, index: usize) -> bool {
        self.set_indices.borrow().contains(&index)
    }

    /// Get the Vec<Vec<i64>> at the given index.
    ///
    /// Returns a clone of the default Vec<Vec<i64>> if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve
    pub fn get(&self, index: usize) -> Vec<Vec<i64>> {
        if !self.contains(index) {
            return self.default_value.clone();
        }

        let page_index = index >> PAGE_SHIFT;
        let page_offset = index & PAGE_MASK;

        self.pages
            .borrow()
            .get(&page_index)
            .and_then(|page| page.get(page_offset).cloned())
            .unwrap_or_else(|| self.default_value.clone())
    }

    /// Set the Vec<Vec<i64>> value at the given index.
    ///
    /// This will expand the capacity if needed and allocate a new page if necessary.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to set
    /// * `value` - The Vec<Vec<i64>> to store
    pub fn set(&self, index: usize, value: Vec<Vec<i64>>) {
        // Update capacity if needed
        {
            let mut capacity = self.capacity.borrow_mut();
            if index >= *capacity {
                *capacity = index + 1;
            }
        }

        let page_index = index >> PAGE_SHIFT;
        let page_offset = index & PAGE_MASK;

        // Get or create the page and set the value
        {
            let mut pages = self.pages.borrow_mut();
            let page = pages
                .entry(page_index)
                .or_insert_with(|| vec![self.default_value.clone(); PAGE_SIZE]);
            page[page_offset] = value;
        }

        // Track that this index has been set
        self.set_indices.borrow_mut().insert(index);
    }

    /// Apply the given consumer to all non-default values stored in the list.
    ///
    /// Only visits indices that have been explicitly set.
    ///
    /// # Arguments
    ///
    /// * `consumer` - Callback function receiving (index, &Vec<Vec<i64>>) pairs
    pub fn for_all<F>(&self, mut consumer: F)
    where
        F: FnMut(usize, &Vec<Vec<i64>>),
    {
        let set_indices = self.set_indices.borrow();
        let pages = self.pages.borrow();

        for &index in set_indices.iter() {
            let page_index = index >> PAGE_SHIFT;
            let page_offset = index & PAGE_MASK;

            if let Some(page) = pages.get(&page_index) {
                if let Some(value) = page.get(page_offset) {
                    consumer(index, value);
                }
            }
        }
    }

    /// Get the number of allocated pages.
    ///
    /// This is useful for understanding memory usage.
    pub fn page_count(&self) -> usize {
        self.pages.borrow().len()
    }

    /// Estimate the memory usage in bytes.
    ///
    /// This includes the HashMap overhead and allocated pages.
    /// Note: Does not account for heap allocations within Vec<Vec<i64>> elements.
    pub fn size_of(&self) -> usize {
        let page_size = PAGE_SIZE * std::mem::size_of::<Vec<Vec<i64>>>();
        self.pages.borrow().len() * page_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);
        list.set(0, vec![vec![1, 2], vec![3]]);
        list.set(1, vec![vec![4, 5, 6]]);
        list.set(100, vec![vec![7], vec![8], vec![9, 10]]);

        assert_eq!(list.get(0), vec![vec![1, 2], vec![3]]);
        assert_eq!(list.get(1), vec![vec![4, 5, 6]]);
        assert_eq!(list.get(100), vec![vec![7], vec![8], vec![9, 10]]);
        assert_eq!(list.get(50), Vec::<Vec<i64>>::new()); // default
    }

    #[test]
    fn test_contains() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);
        list.set(0, vec![vec![1]]);
        list.set(100, vec![vec![2]]);

        assert!(list.contains(0));
        assert!(list.contains(100));
        assert!(!list.contains(1));
        assert!(!list.contains(50));
        assert!(!list.contains(99));
    }

    #[test]
    fn test_sparse_distribution() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);

        // Set values across widely distributed indices
        list.set(0, vec![vec![1]]);
        list.set(10_000_000, vec![vec![2]]);
        list.set(50_000_000, vec![vec![3]]);
        list.set(100_000_000, vec![vec![4]]);

        // Should only allocate 4 pages despite 100M capacity
        assert_eq!(list.page_count(), 4);
        assert_eq!(list.capacity(), 100_000_001);

        // Verify values
        assert_eq!(list.get(0), vec![vec![1]]);
        assert_eq!(list.get(10_000_000), vec![vec![2]]);
        assert_eq!(list.get(50_000_000), vec![vec![3]]);
        assert_eq!(list.get(100_000_000), vec![vec![4]]);
    }

    #[test]
    fn test_empty_matrices() {
        let list = HugeSparseLongArrayArrayList::of(vec![vec![0]]);
        list.set(0, vec![]); // explicitly set empty
        list.set(1, vec![vec![1, 2]]);

        assert_eq!(list.get(0), Vec::<Vec<i64>>::new()); // explicitly set to empty
        assert_eq!(list.get(1), vec![vec![1, 2]]);
        assert_eq!(list.get(2), vec![vec![0]]); // default
        assert!(list.contains(0)); // explicitly set
        assert!(!list.contains(2)); // never set
    }

    #[test]
    fn test_default_value() {
        let default = vec![vec![9, 8], vec![7]];
        let list = HugeSparseLongArrayArrayList::of(default.clone());
        list.set(42, vec![vec![1]]);

        assert_eq!(list.get(42), vec![vec![1]]);
        assert_eq!(list.get(0), default);
        assert_eq!(list.get(100), default);
    }

    #[test]
    fn test_capacity_growth() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);
        list.set(10, vec![vec![1]]);
        assert_eq!(list.capacity(), 11);

        list.set(1000, vec![vec![2]]);
        assert_eq!(list.capacity(), 1001);

        list.set(5, vec![vec![3]]); // shouldn't shrink
        assert_eq!(list.capacity(), 1001);
    }

    #[test]
    fn test_page_boundaries() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);

        // Set values around page boundaries
        list.set(PAGE_SIZE - 1, vec![vec![1]]); // last element of first page
        list.set(PAGE_SIZE, vec![vec![2]]); // first element of second page
        list.set(PAGE_SIZE + 1, vec![vec![3]]); // second element of second page

        assert_eq!(list.get(PAGE_SIZE - 1), vec![vec![1]]);
        assert_eq!(list.get(PAGE_SIZE), vec![vec![2]]);
        assert_eq!(list.get(PAGE_SIZE + 1), vec![vec![3]]);
        assert_eq!(list.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);
        list.set(42, vec![vec![1, 2]]);
        list.set(42, vec![vec![3], vec![4, 5]]); // overwrite

        assert_eq!(list.get(42), vec![vec![3], vec![4, 5]]);
    }

    #[test]
    fn test_for_all() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);
        list.set(0, vec![vec![1]]);
        list.set(100, vec![vec![2]]);
        list.set(1000, vec![vec![3]]);

        let mut collected = Vec::new();
        list.for_all(|index, value| {
            collected.push((index, value.clone()));
        });

        // Sort for consistent ordering
        collected.sort_by_key(|&(i, _)| i);

        assert_eq!(
            collected,
            vec![
                (0, vec![vec![1]]),
                (100, vec![vec![2]]),
                (1000, vec![vec![3]])
            ]
        );
    }

    #[test]
    fn test_large_matrices() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);

        // Store large 2D matrix
        let large_matrix: Vec<Vec<i64>> = (0..100)
            .map(|row| (0..100).map(|col| (row * 100 + col) as i64).collect())
            .collect();

        list.set(42, large_matrix.clone());

        assert_eq!(list.get(42), large_matrix);
        assert_eq!(list.get(42).len(), 100);
        assert_eq!(list.get(42)[0].len(), 100);
    }

    #[test]
    fn test_jagged_arrays() {
        let list = HugeSparseLongArrayArrayList::of(vec![]);

        // Jagged arrays (different row lengths)
        let jagged = vec![vec![1], vec![2, 3], vec![4, 5, 6], vec![7, 8, 9, 10]];

        list.set(42, jagged.clone());

        assert_eq!(list.get(42), jagged);
        assert_eq!(list.get(42)[0].len(), 1);
        assert_eq!(list.get(42)[1].len(), 2);
        assert_eq!(list.get(42)[2].len(), 3);
        assert_eq!(list.get(42)[3].len(), 4);
    }

    #[test]
    fn test_with_capacity() {
        let list = HugeSparseLongArrayArrayList::with_capacity(vec![], 1000);
        assert_eq!(list.capacity(), 1000);

        list.set(500, vec![vec![1, 2, 3]]);
        assert_eq!(list.capacity(), 1000);
        assert_eq!(list.get(500), vec![vec![1, 2, 3]]);
    }
}
