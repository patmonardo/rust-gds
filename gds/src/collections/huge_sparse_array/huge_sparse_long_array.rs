/// A sparse long array supporting >2 billion elements with memory-efficient paged storage.
///
/// This array only allocates pages where values actually exist, using a HashMap to track
/// which pages contain data. Perfect for sparse distributions common in graph algorithms.
///
/// # Design
///
/// - **Sparse optimization**: Only allocates 4KB pages where values exist
/// - **Default values**: Returns user-defined default for unset indices
/// - **Immutability**: Built once using builder, then read-only
/// - **Thread-safe building**: Builder supports concurrent writes
///
/// # Memory Efficiency
///
/// For a sparse array with 1 billion capacity but only 1 million set values:
/// - Dense array: ~8GB (1 billion × 8 bytes)
/// - Sparse array: ~8MB (1 million × 8 bytes + page overhead)
///
/// # Examples
///
/// ```
/// use gds::collections::HugeSparseLongArray;
///
/// // Build sparse array
/// let mut builder = HugeSparseLongArray::builder(-1); // default value = -1
/// builder.set(0, 100);
/// builder.set(1_000_000, 200);
/// builder.set(1_000_000_000, 300); // Sparse!
///
/// let array = builder.build();
///
/// // Access values
/// assert_eq!(array.get(0), 100);
/// assert_eq!(array.get(1_000_000), 200);
/// assert_eq!(array.get(1_000_000_000), 300);
/// assert_eq!(array.get(999), -1); // Returns default
///
/// // Check if explicitly set
/// assert!(array.contains(0));
/// assert!(!array.contains(999));
/// ```
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

const PAGE_SHIFT: u32 = 12; // 4096 elements per page (2^12)
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// Immutable sparse long array with memory-efficient paged storage.
pub struct HugeSparseLongArray {
    capacity: usize,
    default_value: i64,
    pages: HashMap<usize, Vec<i64>>,
    set_indices: std::collections::HashSet<usize>, // Track which indices are explicitly set
}

impl HugeSparseLongArray {
    /// Creates a new builder with the specified default value.
    ///
    /// # Arguments
    ///
    /// * `default_value` - Value returned for unset indices
    ///
    /// # Returns
    ///
    /// A new builder for constructing a sparse array
    pub fn builder(default_value: i64) -> HugeSparseLongArrayBuilder {
        HugeSparseLongArrayBuilder::new(default_value)
    }

    /// Creates a new builder with default value and initial capacity hint.
    ///
    /// # Arguments
    ///
    /// * `default_value` - Value returned for unset indices
    /// * `initial_capacity` - Hint for expected maximum index (optimization only)
    ///
    /// # Returns
    ///
    /// A new builder for constructing a sparse array
    pub fn builder_with_capacity(
        default_value: i64,
        initial_capacity: usize,
    ) -> HugeSparseLongArrayBuilder {
        HugeSparseLongArrayBuilder::with_capacity(default_value, initial_capacity)
    }

    /// Returns the maximum number of values that can be stored.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gets the value at the specified index.
    ///
    /// Returns the default value if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to retrieve
    ///
    /// # Returns
    ///
    /// The value at the index, or default value if unset
    pub fn get(&self, index: usize) -> i64 {
        if index >= self.capacity {
            return self.default_value;
        }

        let page_index = index >> PAGE_SHIFT;
        if let Some(page) = self.pages.get(&page_index) {
            let index_in_page = index & PAGE_MASK;
            page[index_in_page]
        } else {
            self.default_value
        }
    }

    /// Checks if a value has been explicitly set at the given index.
    ///
    /// Distinguishes between "default value" and "explicitly set to default".
    ///
    /// # Arguments
    ///
    /// * `index` - Index to check
    ///
    /// # Returns
    ///
    /// `true` if a value was explicitly set at this index
    pub fn contains(&self, index: usize) -> bool {
        if index >= self.capacity {
            return false;
        }
        self.set_indices.contains(&index)
    }

    /// Returns the number of pages currently allocated.
    ///
    /// Useful for monitoring memory usage.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Returns the estimated memory usage in bytes.
    ///
    /// Includes page storage and HashMap overhead.
    pub fn size_of(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        total += self.pages.len() * (std::mem::size_of::<usize>() + PAGE_SIZE * 8);
        total += self.pages.capacity() * std::mem::size_of::<(usize, Vec<i64>)>();
        total
    }
}

/// Thread-safe builder for constructing sparse long arrays.
///
/// Supports concurrent writes from multiple threads, automatically growing
/// capacity as needed.
pub struct HugeSparseLongArrayBuilder {
    default_value: i64,
    capacity: Arc<RwLock<usize>>,
    pages: Arc<RwLock<HashMap<usize, Vec<i64>>>>,
    set_indices: Arc<RwLock<std::collections::HashSet<usize>>>,
}

impl HugeSparseLongArrayBuilder {
    /// Creates a new builder with the specified default value.
    fn new(default_value: i64) -> Self {
        Self {
            default_value,
            capacity: Arc::new(RwLock::new(0)),
            pages: Arc::new(RwLock::new(HashMap::new())),
            set_indices: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }

    /// Creates a new builder with default value and initial capacity hint.
    fn with_capacity(default_value: i64, initial_capacity: usize) -> Self {
        Self {
            default_value,
            capacity: Arc::new(RwLock::new(initial_capacity)),
            pages: Arc::new(RwLock::new(HashMap::new())),
            set_indices: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }

    /// Sets the value at the specified index.
    ///
    /// Overwrites any existing value. Thread-safe and automatically grows capacity.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to set
    /// * `value` - Value to store
    pub fn set(&mut self, index: usize, value: i64) {
        // Ensure capacity
        {
            let mut cap = self.capacity.write().unwrap();
            if index >= *cap {
                *cap = index + 1;
            }
        }

        let page_index = index >> PAGE_SHIFT;
        let index_in_page = index & PAGE_MASK;

        let mut pages = self.pages.write().unwrap();
        let page = pages
            .entry(page_index)
            .or_insert_with(|| vec![self.default_value; PAGE_SIZE]);
        page[index_in_page] = value;

        // Track that this index was explicitly set
        self.set_indices.write().unwrap().insert(index);
    }

    /// Sets the value at the index only if no value exists there.
    ///
    /// Thread-safe check-and-set operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to conditionally set
    /// * `value` - Value to store if index is unset
    ///
    /// # Returns
    ///
    /// `true` if the value was set, `false` if index already had a value
    pub fn set_if_absent(&mut self, index: usize, value: i64) -> bool {
        // Ensure capacity
        {
            let mut cap = self.capacity.write().unwrap();
            if index >= *cap {
                *cap = index + 1;
            }
        }

        // Check if already set
        if self.set_indices.read().unwrap().contains(&index) {
            return false;
        }

        // Not set yet - set it
        self.set(index, value);
        true
    }

    /// Adds the specified value to the existing value at the index.
    ///
    /// If no value exists, adds to the default value. Thread-safe operation.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to add to
    /// * `value` - Value to add
    pub fn add_to(&mut self, index: usize, value: i64) {
        // Ensure capacity
        {
            let mut cap = self.capacity.write().unwrap();
            if index >= *cap {
                *cap = index + 1;
            }
        }

        let page_index = index >> PAGE_SHIFT;
        let index_in_page = index & PAGE_MASK;

        let mut pages = self.pages.write().unwrap();
        let page = pages
            .entry(page_index)
            .or_insert_with(|| vec![self.default_value; PAGE_SIZE]);
        page[index_in_page] += value;

        // Track that this index was explicitly set
        self.set_indices.write().unwrap().insert(index);
    }

    /// Builds an immutable sparse array from the current builder state.
    ///
    /// The builder can continue to be used after building.
    ///
    /// # Returns
    ///
    /// An immutable `HugeSparseLongArray`
    pub fn build(&self) -> HugeSparseLongArray {
        let capacity = *self.capacity.read().unwrap();
        let pages = self.pages.read().unwrap().clone();
        let set_indices = self.set_indices.read().unwrap().clone();

        HugeSparseLongArray {
            capacity,
            default_value: self.default_value,
            pages,
            set_indices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let mut builder = HugeSparseLongArray::builder(0);
        builder.set(0, 42);
        builder.set(100, 100);
        builder.set(1000, 1000);

        let array = builder.build();

        assert_eq!(array.get(0), 42);
        assert_eq!(array.get(100), 100);
        assert_eq!(array.get(1000), 1000);
        assert_eq!(array.get(50), 0); // Default value
    }

    #[test]
    fn test_contains() {
        let mut builder = HugeSparseLongArray::builder(-1);
        builder.set(0, 100);
        builder.set(1000, 200);

        let array = builder.build();

        assert!(array.contains(0));
        assert!(!array.contains(50));
        assert!(array.contains(1000));
        assert!(!array.contains(2000));
    }

    #[test]
    fn test_sparse_distribution() {
        let mut builder = HugeSparseLongArray::builder(0);

        // Set widely distributed values
        builder.set(0, 1);
        builder.set(10_000, 2);
        builder.set(1_000_000, 3);
        builder.set(100_000_000, 4);

        let array = builder.build();

        assert_eq!(array.get(0), 1);
        assert_eq!(array.get(10_000), 2);
        assert_eq!(array.get(1_000_000), 3);
        assert_eq!(array.get(100_000_000), 4);

        // Should only have 4 pages allocated
        assert_eq!(array.page_count(), 4);

        // All other values return default
        assert_eq!(array.get(500_000), 0);
    }

    #[test]
    fn test_add_to() {
        let mut builder = HugeSparseLongArray::builder(0);

        builder.add_to(0, 10);
        builder.add_to(0, 20);
        builder.add_to(0, 30);

        let array = builder.build();
        assert_eq!(array.get(0), 60);
    }

    #[test]
    fn test_set_if_absent() {
        let mut builder = HugeSparseLongArray::builder(0);

        // First set should succeed
        assert!(builder.set_if_absent(0, 100));

        // Second set should fail (value already exists)
        assert!(!builder.set_if_absent(0, 200));

        let array = builder.build();
        assert_eq!(array.get(0), 100); // Should be first value
    }

    #[test]
    fn test_default_value() {
        let mut builder = HugeSparseLongArray::builder(-999);
        builder.set(100, 42);

        let array = builder.build();

        assert_eq!(array.get(100), 42);
        assert_eq!(array.get(0), -999); // Default
        assert_eq!(array.get(1000), -999); // Default
    }

    #[test]
    fn test_capacity_growth() {
        let mut builder = HugeSparseLongArray::builder(0);

        builder.set(10, 1);
        let array1 = builder.build();
        assert!(array1.capacity() >= 11);

        builder.set(1000, 2);
        let array2 = builder.build();
        assert!(array2.capacity() >= 1001);

        builder.set(1_000_000, 3);
        let array3 = builder.build();
        assert!(array3.capacity() >= 1_000_001);
    }

    #[test]
    fn test_page_boundaries() {
        let mut builder = HugeSparseLongArray::builder(0);

        // Test around page boundary (4096 elements per page)
        builder.set(4095, 100); // Last element of page 0
        builder.set(4096, 200); // First element of page 1
        builder.set(4097, 300); // Second element of page 1

        let array = builder.build();

        assert_eq!(array.get(4095), 100);
        assert_eq!(array.get(4096), 200);
        assert_eq!(array.get(4097), 300);
        assert_eq!(array.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let mut builder = HugeSparseLongArray::builder(0);

        builder.set(100, 42);
        builder.set(100, 100); // Overwrite
        builder.set(100, 200); // Overwrite again

        let array = builder.build();
        assert_eq!(array.get(100), 200);
    }

    #[test]
    fn test_builder_reuse() {
        let mut builder = HugeSparseLongArray::builder(0);

        builder.set(0, 100);
        let array1 = builder.build();
        assert_eq!(array1.get(0), 100);

        // Continue using builder
        builder.set(1, 200);
        let array2 = builder.build();
        assert_eq!(array2.get(0), 100);
        assert_eq!(array2.get(1), 200);
    }

    #[test]
    fn test_with_capacity() {
        let mut builder = HugeSparseLongArray::builder_with_capacity(0, 10_000);

        builder.set(5000, 42);
        let array = builder.build();

        assert_eq!(array.get(5000), 42);
        assert!(array.capacity() >= 10_000);
    }
}
