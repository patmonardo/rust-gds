/// A sparse array of i64 vectors supporting >2 billion elements with memory-efficient storage.
///
/// This array stores `Vec<i64>` as elements, only allocating pages where vectors actually exist.
/// Perfect for sparse collections of variable-length data like adjacency lists or property arrays.
///
/// # Design
///
/// - **Sparse optimization**: Only allocates pages where Vec<i64> values exist
/// - **Default values**: Returns user-defined default Vec<i64> for unset indices
/// - **Immutability**: Built once using builder, then read-only
/// - **Thread-safe building**: Builder supports concurrent writes
///
/// # Memory Efficiency
///
/// For a sparse array with 1 billion capacity but only 1 million set vectors:
/// - Only allocates pages for the 1 million vectors that exist
/// - Plus the memory for the vectors themselves
///
/// # Examples
///
/// ```
/// use rust_gds::collections::HugeSparseLongArrayArray;
///
/// // Build sparse array of adjacency lists
/// let default_empty = vec![];
/// let mut builder = HugeSparseLongArrayArray::builder(default_empty.clone());
///
/// builder.set(0, vec![1, 2, 3]); // Node 0 connects to 1, 2, 3
/// builder.set(1_000_000, vec![42, 99]); // Sparse!
///
/// let array = builder.build();
///
/// // Access values
/// assert_eq!(array.get(0), &vec![1, 2, 3]);
/// assert_eq!(array.get(1_000_000), &vec![42, 99]);
/// assert_eq!(array.get(999), &Vec::<i64>::new()); // Returns default empty vec
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

/// Immutable sparse array of i64 vectors with memory-efficient paged storage.
pub struct HugeSparseLongArrayArray {
    capacity: usize,
    default_value: Vec<i64>,
    pages: HashMap<usize, Vec<Vec<i64>>>,
    set_indices: std::collections::HashSet<usize>, // Track which indices are explicitly set
}

impl HugeSparseLongArrayArray {
    /// Creates a new builder with the specified default value.
    ///
    /// # Arguments
    ///
    /// * `default_value` - Vec<i64> returned for unset indices
    ///
    /// # Returns
    ///
    /// A new builder for constructing a sparse array
    pub fn builder(default_value: Vec<i64>) -> HugeSparseLongArrayArrayBuilder {
        HugeSparseLongArrayArrayBuilder::new(default_value)
    }

    /// Creates a new builder with default value and initial capacity hint.
    ///
    /// # Arguments
    ///
    /// * `default_value` - Vec<i64> returned for unset indices
    /// * `initial_capacity` - Hint for expected maximum index (optimization only)
    ///
    /// # Returns
    ///
    /// A new builder for constructing a sparse array
    pub fn builder_with_capacity(
        default_value: Vec<i64>,
        initial_capacity: usize,
    ) -> HugeSparseLongArrayArrayBuilder {
        HugeSparseLongArrayArrayBuilder::with_capacity(default_value, initial_capacity)
    }

    /// Returns the maximum number of values that can be stored.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gets the Vec<i64> at the specified index.
    ///
    /// Returns a reference to the default value if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to retrieve
    ///
    /// # Returns
    ///
    /// Reference to the Vec<i64> at the index, or default if unset
    pub fn get(&self, index: usize) -> &Vec<i64> {
        if index >= self.capacity {
            return &self.default_value;
        }

        let page_index = index >> PAGE_SHIFT;
        if let Some(page) = self.pages.get(&page_index) {
            let index_in_page = index & PAGE_MASK;
            &page[index_in_page]
        } else {
            &self.default_value
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
    /// `true` if a Vec<i64> was explicitly set at this index
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

    /// Returns the estimated memory usage in bytes (pages only, not Vec contents).
    pub fn size_of(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();
        total += self.pages.len()
            * (std::mem::size_of::<usize>() + PAGE_SIZE * std::mem::size_of::<Vec<i64>>());
        total += self.pages.capacity() * std::mem::size_of::<(usize, Vec<Vec<i64>>)>();
        total
    }
}

/// Thread-safe builder for constructing sparse arrays of i64 vectors.
///
/// Supports concurrent writes from multiple threads, automatically growing
/// capacity as needed.
pub struct HugeSparseLongArrayArrayBuilder {
    default_value: Vec<i64>,
    capacity: Arc<RwLock<usize>>,
    pages: Arc<RwLock<HashMap<usize, Vec<Vec<i64>>>>>,
    set_indices: Arc<RwLock<std::collections::HashSet<usize>>>,
}

impl HugeSparseLongArrayArrayBuilder {
    /// Creates a new builder with the specified default value.
    fn new(default_value: Vec<i64>) -> Self {
        Self {
            default_value,
            capacity: Arc::new(RwLock::new(0)),
            pages: Arc::new(RwLock::new(HashMap::new())),
            set_indices: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }

    /// Creates a new builder with default value and initial capacity hint.
    fn with_capacity(default_value: Vec<i64>, initial_capacity: usize) -> Self {
        Self {
            default_value,
            capacity: Arc::new(RwLock::new(initial_capacity)),
            pages: Arc::new(RwLock::new(HashMap::new())),
            set_indices: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }

    /// Sets the Vec<i64> at the specified index.
    ///
    /// Overwrites any existing value. Thread-safe and automatically grows capacity.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to set
    /// * `value` - Vec<i64> to store
    pub fn set(&mut self, index: usize, value: Vec<i64>) {
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
            .or_insert_with(|| vec![self.default_value.clone(); PAGE_SIZE]);
        page[index_in_page] = value;

        // Track that this index was explicitly set
        self.set_indices.write().unwrap().insert(index);
    }

    /// Builds an immutable sparse array from the current builder state.
    ///
    /// The builder can continue to be used after building.
    ///
    /// # Returns
    ///
    /// An immutable `HugeSparseLongArrayArray`
    pub fn build(&self) -> HugeSparseLongArrayArray {
        let capacity = *self.capacity.read().unwrap();
        let pages = self.pages.read().unwrap().clone();
        let set_indices = self.set_indices.read().unwrap().clone();

        HugeSparseLongArrayArray {
            capacity,
            default_value: self.default_value.clone(),
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
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty.clone());

        builder.set(0, vec![1, 2, 3]);
        builder.set(100, vec![10, 20]);
        builder.set(1000, vec![100]);

        let array = builder.build();

        assert_eq!(array.get(0), &vec![1, 2, 3]);
        assert_eq!(array.get(100), &vec![10, 20]);
        assert_eq!(array.get(1000), &vec![100]);
        assert_eq!(array.get(50), &Vec::<i64>::new()); // Default value
    }

    #[test]
    fn test_contains() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        builder.set(0, vec![1, 2]);
        builder.set(1000, vec![3, 4]);

        let array = builder.build();

        assert!(array.contains(0));
        assert!(!array.contains(50));
        assert!(array.contains(1000));
        assert!(!array.contains(2000));
    }

    #[test]
    fn test_sparse_distribution() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        // Set widely distributed adjacency lists
        builder.set(0, vec![1, 2, 3]);
        builder.set(10_000, vec![42]);
        builder.set(1_000_000, vec![99, 100, 101]);
        builder.set(100_000_000, vec![999]);

        let array = builder.build();

        assert_eq!(array.get(0), &vec![1, 2, 3]);
        assert_eq!(array.get(10_000), &vec![42]);
        assert_eq!(array.get(1_000_000), &vec![99, 100, 101]);
        assert_eq!(array.get(100_000_000), &vec![999]);

        // Should only have 4 pages allocated
        assert_eq!(array.page_count(), 4);

        // All other values return default empty vec
        assert_eq!(array.get(500_000), &Vec::<i64>::new());
    }

    #[test]
    fn test_empty_vectors() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty.clone());

        builder.set(0, vec![]);
        builder.set(1, vec![1, 2, 3]);

        let array = builder.build();

        // Explicitly set to empty
        assert!(array.contains(0));
        assert_eq!(array.get(0), &Vec::<i64>::new());

        // Has values
        assert!(array.contains(1));
        assert_eq!(array.get(1), &vec![1, 2, 3]);

        // Never set
        assert!(!array.contains(2));
        assert_eq!(array.get(2), &default_empty);
    }

    #[test]
    fn test_default_value() {
        let default = vec![-1, -2, -3];
        let mut builder = HugeSparseLongArrayArray::builder(default.clone());

        builder.set(100, vec![42]);

        let array = builder.build();

        assert_eq!(array.get(100), &vec![42]);
        assert_eq!(array.get(0), &default); // Default
        assert_eq!(array.get(1000), &default); // Default
    }

    #[test]
    fn test_capacity_growth() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        builder.set(10, vec![1]);
        let array1 = builder.build();
        assert!(array1.capacity() >= 11);

        builder.set(1000, vec![2]);
        let array2 = builder.build();
        assert!(array2.capacity() >= 1001);

        builder.set(1_000_000, vec![3]);
        let array3 = builder.build();
        assert!(array3.capacity() >= 1_000_001);
    }

    #[test]
    fn test_page_boundaries() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        // Test around page boundary (4096 elements per page)
        builder.set(4095, vec![100]); // Last element of page 0
        builder.set(4096, vec![200]); // First element of page 1
        builder.set(4097, vec![300]); // Second element of page 1

        let array = builder.build();

        assert_eq!(array.get(4095), &vec![100]);
        assert_eq!(array.get(4096), &vec![200]);
        assert_eq!(array.get(4097), &vec![300]);
        assert_eq!(array.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        builder.set(100, vec![1]);
        builder.set(100, vec![2, 3]); // Overwrite
        builder.set(100, vec![4, 5, 6]); // Overwrite again

        let array = builder.build();
        assert_eq!(array.get(100), &vec![4, 5, 6]);
    }

    #[test]
    fn test_builder_reuse() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        builder.set(0, vec![100]);
        let array1 = builder.build();
        assert_eq!(array1.get(0), &vec![100]);

        // Continue using builder
        builder.set(1, vec![200]);
        let array2 = builder.build();
        assert_eq!(array2.get(0), &vec![100]);
        assert_eq!(array2.get(1), &vec![200]);
    }

    #[test]
    fn test_large_vectors() {
        let default_empty = vec![];
        let mut builder = HugeSparseLongArrayArray::builder(default_empty);

        // Store large adjacency lists
        let large_vec: Vec<i64> = (0..10000).collect();
        builder.set(0, large_vec.clone());

        let array = builder.build();
        assert_eq!(array.get(0), &large_vec);
    }
}
