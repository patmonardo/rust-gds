use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/// Number of elements in a single page (4096 elements = 4KB for most types)
const PAGE_SIZE: usize = 4096;
/// Number of bits to shift for page index calculation
const PAGE_SHIFT: usize = 12; // log2(4096)
/// Mask to extract offset within a page
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// A long-indexable mutable sparse list of f64 that can contain more than 2 billion elements.
///
/// Uses a HashMap to track pages, only allocating memory for pages that contain values.
/// Perfect for dynamic sparse double collections that grow and change over time.
///
/// **Memory Efficiency**: Only allocates 4KB pages where f64 values actually exist.
/// **Mutability**: Can be modified after creation using set(), add_to(), etc.
/// **NOT Thread-Safe**: Uses RefCell for interior mutability - not safe for concurrent access.
/// **Default Values**: Returns the default f64 for unset indices.
///
/// # Examples
///
/// ```
/// use gds::collections::HugeSparseDoubleList;
///
/// let list = HugeSparseDoubleList::of(0.0);
///
/// list.set(0, 3.14);
/// list.set(1_000_000, 2.71);
/// list.add_to(0, 0.01);  // 3.14 + 0.01 = 3.15
///
/// assert_eq!(list.get(0), 3.15);
/// assert_eq!(list.get(1_000_000), 2.71);
/// assert_eq!(list.get(50), 0.0); // default
/// assert!(list.contains(0));
/// assert!(!list.contains(50));
/// ```
#[derive(Debug)]
pub struct HugeSparseDoubleList {
    /// Maximum capacity (highest index + 1)
    capacity: RefCell<usize>,
    /// Default f64 returned for unset indices
    default_value: f64,
    /// HashMap storing only allocated pages
    pages: RefCell<HashMap<usize, Vec<f64>>>,
    /// HashSet tracking which indices have been explicitly set
    set_indices: RefCell<HashSet<usize>>,
}

impl HugeSparseDoubleList {
    /// Create a new sparse double list with the specified default value.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The f64 returned for unset indices
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::HugeSparseDoubleList;
    ///
    /// let list = HugeSparseDoubleList::of(0.0);
    /// list.set(42, 3.14);
    /// assert_eq!(list.get(42), 3.14);
    /// ```
    pub fn of(default_value: f64) -> Self {
        Self::with_capacity(default_value, 0)
    }

    /// Create a new sparse double list with an initial capacity hint.
    ///
    /// This can improve performance when the approximate maximum index is known,
    /// but the list will still grow dynamically if needed.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The f64 returned for unset indices
    /// * `capacity` - Initial capacity hint
    pub fn with_capacity(default_value: f64, capacity: usize) -> Self {
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

    /// Get the f64 value at the given index.
    ///
    /// Returns the default f64 if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve
    pub fn get(&self, index: usize) -> f64 {
        if !self.contains(index) {
            return self.default_value;
        }

        let page_index = index >> PAGE_SHIFT;
        let page_offset = index & PAGE_MASK;

        self.pages
            .borrow()
            .get(&page_index)
            .and_then(|page| page.get(page_offset).copied())
            .unwrap_or(self.default_value)
    }

    /// Set the f64 value at the given index.
    ///
    /// This will expand the capacity if needed and allocate a new page if necessary.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to set
    /// * `value` - The f64 to store
    pub fn set(&self, index: usize, value: f64) {
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
                .or_insert_with(|| vec![self.default_value; PAGE_SIZE]);
            page[page_offset] = value;
        }

        // Track that this index has been set
        self.set_indices.borrow_mut().insert(index);
    }

    /// Set the f64 value at the given index if and only if no value exists there.
    ///
    /// Returns true if the value was set, false if the index already had a value.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to conditionally set
    /// * `value` - The f64 to store if index is unset
    pub fn set_if_absent(&self, index: usize, value: f64) -> bool {
        if self.contains(index) {
            return false;
        }
        self.set(index, value);
        true
    }

    /// Add the given value to the existing f64 at the index.
    ///
    /// If no value exists, adds to the default value.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to add to
    /// * `value` - The f64 to add
    pub fn add_to(&self, index: usize, value: f64) {
        let current = self.get(index);
        self.set(index, current + value);
    }

    /// Apply the given consumer to all non-default values stored in the list.
    ///
    /// Only visits indices that have been explicitly set.
    ///
    /// # Arguments
    ///
    /// * `consumer` - Callback function receiving (index, value) pairs
    pub fn for_all<F>(&self, mut consumer: F)
    where
        F: FnMut(usize, f64),
    {
        let set_indices = self.set_indices.borrow();
        for &index in set_indices.iter() {
            let value = self.get(index);
            consumer(index, value);
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
    pub fn size_of(&self) -> usize {
        let page_size = PAGE_SIZE * std::mem::size_of::<f64>();
        self.pages.borrow().len() * page_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(0, 3.14);
        list.set(1, 2.71);
        list.set(100, 1.41);

        assert_eq!(list.get(0), 3.14);
        assert_eq!(list.get(1), 2.71);
        assert_eq!(list.get(100), 1.41);
        assert_eq!(list.get(50), 0.0); // default
    }

    #[test]
    fn test_contains() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(0, 3.14);
        list.set(100, 2.71);

        assert!(list.contains(0));
        assert!(list.contains(100));
        assert!(!list.contains(1));
        assert!(!list.contains(50));
        assert!(!list.contains(99));
    }

    #[test]
    fn test_sparse_distribution() {
        let list = HugeSparseDoubleList::of(0.0);

        // Set values across widely distributed indices
        list.set(0, 1.1);
        list.set(10_000_000, 2.2);
        list.set(50_000_000, 3.3);
        list.set(100_000_000, 4.4);

        // Should only allocate 4 pages despite 100M capacity
        assert_eq!(list.page_count(), 4);
        assert_eq!(list.capacity(), 100_000_001);

        // Verify values
        assert_eq!(list.get(0), 1.1);
        assert_eq!(list.get(10_000_000), 2.2);
        assert_eq!(list.get(50_000_000), 3.3);
        assert_eq!(list.get(100_000_000), 4.4);
    }

    #[test]
    fn test_add_to() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(0, 3.14);
        list.add_to(0, 0.01);
        assert!((list.get(0) - 3.15).abs() < 1e-10);

        // Add to unset index (should add to default)
        list.add_to(100, 2.71);
        assert!((list.get(100) - 2.71).abs() < 1e-10);
    }

    #[test]
    fn test_set_if_absent() {
        let list = HugeSparseDoubleList::of(0.0);

        // First set should succeed
        assert!(list.set_if_absent(0, 3.14));
        assert_eq!(list.get(0), 3.14);

        // Second set should fail
        assert!(!list.set_if_absent(0, 2.71));
        assert_eq!(list.get(0), 3.14); // unchanged
    }

    #[test]
    fn test_default_value() {
        let list = HugeSparseDoubleList::of(9.99);
        list.set(42, 3.14);

        assert_eq!(list.get(42), 3.14);
        assert_eq!(list.get(0), 9.99); // default
        assert_eq!(list.get(100), 9.99); // default
    }

    #[test]
    fn test_capacity_growth() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(10, 1.1);
        assert_eq!(list.capacity(), 11);

        list.set(1000, 2.2);
        assert_eq!(list.capacity(), 1001);

        list.set(5, 3.3); // shouldn't shrink
        assert_eq!(list.capacity(), 1001);
    }

    #[test]
    fn test_page_boundaries() {
        let list = HugeSparseDoubleList::of(0.0);

        // Set values around page boundaries
        list.set(PAGE_SIZE - 1, 1.1); // last element of first page
        list.set(PAGE_SIZE, 2.2); // first element of second page
        list.set(PAGE_SIZE + 1, 3.3); // second element of second page

        assert_eq!(list.get(PAGE_SIZE - 1), 1.1);
        assert_eq!(list.get(PAGE_SIZE), 2.2);
        assert_eq!(list.get(PAGE_SIZE + 1), 3.3);
        assert_eq!(list.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(42, 3.14);
        list.set(42, 2.71); // overwrite

        assert_eq!(list.get(42), 2.71);
    }

    #[test]
    fn test_for_all() {
        let list = HugeSparseDoubleList::of(0.0);
        list.set(0, 1.1);
        list.set(100, 2.2);
        list.set(1000, 3.3);

        let mut collected = Vec::new();
        list.for_all(|index, value| {
            collected.push((index, value));
        });

        // Sort for consistent ordering
        collected.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0].0, 0);
        assert!((collected[0].1 - 1.1).abs() < 1e-10);
        assert_eq!(collected[1].0, 100);
        assert!((collected[1].1 - 2.2).abs() < 1e-10);
        assert_eq!(collected[2].0, 1000);
        assert!((collected[2].1 - 3.3).abs() < 1e-10);
    }

    #[test]
    fn test_floating_point_precision() {
        let list = HugeSparseDoubleList::of(0.0);

        use std::f64::consts::{E, PI, SQRT_2};
        list.set(0, PI);
        list.set(1, E);
        list.set(2, SQRT_2);

        assert_eq!(list.get(0), PI);
        assert_eq!(list.get(1), E);
        assert_eq!(list.get(2), SQRT_2);
    }

    #[test]
    fn test_with_capacity() {
        let list = HugeSparseDoubleList::with_capacity(0.0, 1000);
        assert_eq!(list.capacity(), 1000);

        list.set(500, 3.14);
        assert_eq!(list.capacity(), 1000);
        assert_eq!(list.get(500), 3.14);
    }
}
