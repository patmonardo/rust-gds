use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/// Number of elements in a single page (4096 elements = 4KB for most types)
const PAGE_SIZE: usize = 4096;
/// Number of bits to shift for page index calculation
const PAGE_SHIFT: usize = 12; // log2(4096)
/// Mask to extract offset within a page
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// A long-indexable mutable sparse list of i64 that can contain more than 2 billion elements.
///
/// Uses a HashMap to track pages, only allocating memory for pages that contain values.
/// Perfect for dynamic sparse long collections that grow and change over time.
///
/// **Memory Efficiency**: Only allocates 4KB pages where i64 values actually exist.
/// **Mutability**: Can be modified after creation using set(), add_to(), etc.
/// **NOT Thread-Safe**: Uses RefCell for interior mutability - not safe for concurrent access.
/// **Default Values**: Returns the default i64 for unset indices.
///
/// # Examples
///
/// ```
/// use rust_gds::collections::HugeSparseLongList;
///
/// let list = HugeSparseLongList::of(0);
///
/// list.set(0, 42);
/// list.set(1_000_000, 99);
/// list.add_to(0, 8);  // 42 + 8 = 50
///
/// assert_eq!(list.get(0), 50);
/// assert_eq!(list.get(1_000_000), 99);
/// assert_eq!(list.get(50), 0); // default
/// assert!(list.contains(0));
/// assert!(!list.contains(50));
/// ```
#[derive(Debug)]
pub struct HugeSparseLongList {
    /// Maximum capacity (highest index + 1)
    capacity: RefCell<usize>,
    /// Default i64 returned for unset indices
    default_value: i64,
    /// HashMap storing only allocated pages
    pages: RefCell<HashMap<usize, Vec<i64>>>,
    /// HashSet tracking which indices have been explicitly set
    set_indices: RefCell<HashSet<usize>>,
}

impl HugeSparseLongList {
    /// Create a new sparse long list with the specified default value.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The i64 returned for unset indices
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::collections::HugeSparseLongList;
    ///
    /// let list = HugeSparseLongList::of(0);
    /// list.set(42, 100);
    /// assert_eq!(list.get(42), 100);
    /// ```
    pub fn of(default_value: i64) -> Self {
        Self::with_capacity(default_value, 0)
    }

    /// Create a new sparse long list with an initial capacity hint.
    ///
    /// This can improve performance when the approximate maximum index is known,
    /// but the list will still grow dynamically if needed.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The i64 returned for unset indices
    /// * `capacity` - Initial capacity hint
    pub fn with_capacity(default_value: i64, capacity: usize) -> Self {
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

    /// Get the i64 value at the given index.
    ///
    /// Returns the default i64 if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve
    pub fn get(&self, index: usize) -> i64 {
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

    /// Set the i64 value at the given index.
    ///
    /// This will expand the capacity if needed and allocate a new page if necessary.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to set
    /// * `value` - The i64 to store
    pub fn set(&self, index: usize, value: i64) {
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

    /// Set the i64 value at the given index if and only if no value exists there.
    ///
    /// Returns true if the value was set, false if the index already had a value.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to conditionally set
    /// * `value` - The i64 to store if index is unset
    pub fn set_if_absent(&self, index: usize, value: i64) -> bool {
        if self.contains(index) {
            return false;
        }
        self.set(index, value);
        true
    }

    /// Add the given value to the existing i64 at the index.
    ///
    /// If no value exists, adds to the default value.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to add to
    /// * `value` - The i64 to add
    pub fn add_to(&self, index: usize, value: i64) {
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
        F: FnMut(usize, i64),
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
        let page_size = PAGE_SIZE * std::mem::size_of::<i64>();
        self.pages.borrow().len() * page_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let list = HugeSparseLongList::of(0);
        list.set(0, 42);
        list.set(1, 99);
        list.set(100, 123);

        assert_eq!(list.get(0), 42);
        assert_eq!(list.get(1), 99);
        assert_eq!(list.get(100), 123);
        assert_eq!(list.get(50), 0); // default
    }

    #[test]
    fn test_contains() {
        let list = HugeSparseLongList::of(0);
        list.set(0, 42);
        list.set(100, 99);

        assert!(list.contains(0));
        assert!(list.contains(100));
        assert!(!list.contains(1));
        assert!(!list.contains(50));
        assert!(!list.contains(99));
    }

    #[test]
    fn test_sparse_distribution() {
        let list = HugeSparseLongList::of(0);

        // Set values across widely distributed indices
        list.set(0, 10);
        list.set(10_000_000, 20);
        list.set(50_000_000, 30);
        list.set(100_000_000, 40);

        // Should only allocate 4 pages despite 100M capacity
        assert_eq!(list.page_count(), 4);
        assert_eq!(list.capacity(), 100_000_001);

        // Verify values
        assert_eq!(list.get(0), 10);
        assert_eq!(list.get(10_000_000), 20);
        assert_eq!(list.get(50_000_000), 30);
        assert_eq!(list.get(100_000_000), 40);
    }

    #[test]
    fn test_add_to() {
        let list = HugeSparseLongList::of(0);
        list.set(0, 10);
        list.add_to(0, 5);
        assert_eq!(list.get(0), 15);

        // Add to unset index (should add to default)
        list.add_to(100, 42);
        assert_eq!(list.get(100), 42);
    }

    #[test]
    fn test_set_if_absent() {
        let list = HugeSparseLongList::of(0);

        // First set should succeed
        assert!(list.set_if_absent(0, 42));
        assert_eq!(list.get(0), 42);

        // Second set should fail
        assert!(!list.set_if_absent(0, 99));
        assert_eq!(list.get(0), 42); // unchanged
    }

    #[test]
    fn test_default_value() {
        let list = HugeSparseLongList::of(999);
        list.set(42, 100);

        assert_eq!(list.get(42), 100);
        assert_eq!(list.get(0), 999); // default
        assert_eq!(list.get(100), 999); // default
    }

    #[test]
    fn test_capacity_growth() {
        let list = HugeSparseLongList::of(0);
        list.set(10, 42);
        assert_eq!(list.capacity(), 11);

        list.set(1000, 99);
        assert_eq!(list.capacity(), 1001);

        list.set(5, 123); // shouldn't shrink
        assert_eq!(list.capacity(), 1001);
    }

    #[test]
    fn test_page_boundaries() {
        let list = HugeSparseLongList::of(0);

        // Set values around page boundaries
        list.set(PAGE_SIZE - 1, 10); // last element of first page
        list.set(PAGE_SIZE, 20); // first element of second page
        list.set(PAGE_SIZE + 1, 30); // second element of second page

        assert_eq!(list.get(PAGE_SIZE - 1), 10);
        assert_eq!(list.get(PAGE_SIZE), 20);
        assert_eq!(list.get(PAGE_SIZE + 1), 30);
        assert_eq!(list.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let list = HugeSparseLongList::of(0);
        list.set(42, 100);
        list.set(42, 200); // overwrite

        assert_eq!(list.get(42), 200);
    }

    #[test]
    fn test_for_all() {
        let list = HugeSparseLongList::of(0);
        list.set(0, 10);
        list.set(100, 20);
        list.set(1000, 30);

        let mut collected = Vec::new();
        list.for_all(|index, value| {
            collected.push((index, value));
        });

        // Sort for consistent ordering
        collected.sort();

        assert_eq!(collected, vec![(0, 10), (100, 20), (1000, 30)]);
    }

    #[test]
    fn test_with_capacity() {
        let list = HugeSparseLongList::with_capacity(0, 1000);
        assert_eq!(list.capacity(), 1000);

        list.set(500, 42);
        assert_eq!(list.capacity(), 1000);
        assert_eq!(list.get(500), 42);
    }
}
