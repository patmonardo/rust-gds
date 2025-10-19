use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Number of elements in a single page (4096 elements = 4KB for most types)
const PAGE_SIZE: usize = 4096;
/// Number of bits to shift for page index calculation
const PAGE_SHIFT: usize = 12; // log2(4096)
/// Mask to extract offset within a page
const PAGE_MASK: usize = PAGE_SIZE - 1;

/// A long-indexable sparse array of Vec<f64> that can contain more than 2 billion elements.
///
/// Uses a HashMap to track pages, only allocating memory for pages that contain values.
/// Perfect for sparse collections of double arrays (e.g., high-precision feature vectors,
/// scientific data, ML embeddings).
///
/// **Memory Efficiency**: Only allocates 4KB pages where Vec<f64> values actually exist.
/// **Immutability**: Built once using a thread-safe builder, then read-only access.
/// **Default Values**: Returns a clone of the default Vec<f64> for unset indices.
///
/// # Examples
///
/// ```
/// use gds::collections::{HugeSparseDoubleArrayArray, HugeSparseDoubleArrayArrayBuilder};
///
/// let default_vec = vec![0.0];
/// let builder = HugeSparseDoubleArrayArray::builder(default_vec);
///
/// builder.set(0, vec![1.1, 2.2]);
/// builder.set(1_000_000, vec![3.3, 4.4]);
/// builder.set(100_000_000, vec![5.5, 6.6, 7.7]);
///
/// let array = builder.build();
/// assert_eq!(array.get(0), &vec![1.1, 2.2]);
/// assert_eq!(array.get(1_000_000), &vec![3.3, 4.4]);
/// assert_eq!(array.get(100_000_000), &vec![5.5, 6.6, 7.7]);
/// assert_eq!(array.get(50), &vec![0.0]); // default
/// ```
#[derive(Debug)]
pub struct HugeSparseDoubleArrayArray {
    /// Maximum capacity (highest index + 1)
    capacity: usize,
    /// Default Vec<f64> returned for unset indices
    default_value: Vec<f64>,
    /// HashMap storing only allocated pages
    pages: HashMap<usize, Vec<Vec<f64>>>,
    /// HashSet tracking which indices have been explicitly set
    set_indices: HashSet<usize>,
}

impl HugeSparseDoubleArrayArray {
    /// Create a new builder with the specified default Vec<f64>.
    ///
    /// The builder uses Arc<RwLock<_>> for thread-safe concurrent building.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The Vec<f64> returned for unset indices
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::HugeSparseDoubleArrayArray;
    ///
    /// let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
    /// builder.set(42, vec![3.14, 2.71]);
    /// let array = builder.build();
    /// ```
    pub fn builder(default_value: Vec<f64>) -> HugeSparseDoubleArrayArrayBuilder {
        HugeSparseDoubleArrayArrayBuilder::new(default_value, 0)
    }

    /// Create a new builder with an initial capacity hint.
    ///
    /// This can improve performance when the approximate maximum index is known,
    /// but the array will still grow dynamically if needed.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The Vec<f64> returned for unset indices
    /// * `capacity` - Initial capacity hint
    pub fn builder_with_capacity(
        default_value: Vec<f64>,
        capacity: usize,
    ) -> HugeSparseDoubleArrayArrayBuilder {
        HugeSparseDoubleArrayArrayBuilder::new(default_value, capacity)
    }

    /// Get the maximum number of values that can be stored.
    ///
    /// This is the highest index that has been set plus one.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the Vec<f64> at the given index.
    ///
    /// Returns a reference to the default Vec<f64> if the index was never set.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve
    ///
    /// # Panics
    ///
    /// Panics if index >= capacity()
    pub fn get(&self, index: usize) -> &Vec<f64> {
        if !self.contains(index) {
            return &self.default_value;
        }

        let page_index = index >> PAGE_SHIFT;
        let page_offset = index & PAGE_MASK;

        self.pages
            .get(&page_index)
            .and_then(|page| page.get(page_offset))
            .unwrap_or(&self.default_value)
    }

    /// Check if a value has been explicitly set at the given index.
    ///
    /// This is useful to distinguish between "never set" and "set to default value".
    ///
    /// # Arguments
    ///
    /// * `index` - The index to check
    pub fn contains(&self, index: usize) -> bool {
        self.set_indices.contains(&index)
    }

    /// Get the number of allocated pages.
    ///
    /// This is useful for understanding memory usage.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Estimate the memory usage in bytes.
    ///
    /// This includes the HashMap overhead and allocated pages.
    /// Note: Does not account for heap allocations within Vec<f64> elements.
    pub fn size_of(&self) -> usize {
        let page_size = PAGE_SIZE * std::mem::size_of::<Vec<f64>>();
        self.pages.len() * page_size
    }
}

/// Thread-safe builder for HugeSparseDoubleArrayArray.
///
/// Uses Arc<RwLock<_>> to allow concurrent building from multiple threads.
/// The builder can be reused after calling build().
#[derive(Debug, Clone)]
pub struct HugeSparseDoubleArrayArrayBuilder {
    default_value: Vec<f64>,
    capacity: Arc<RwLock<usize>>,
    pages: Arc<RwLock<HashMap<usize, Vec<Vec<f64>>>>>,
    set_indices: Arc<RwLock<HashSet<usize>>>,
}

impl HugeSparseDoubleArrayArrayBuilder {
    /// Create a new builder with the specified default value and capacity hint.
    fn new(default_value: Vec<f64>, capacity: usize) -> Self {
        Self {
            default_value,
            capacity: Arc::new(RwLock::new(capacity)),
            pages: Arc::new(RwLock::new(HashMap::new())),
            set_indices: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Set the Vec<f64> value at the given index.
    ///
    /// This will expand the capacity if needed and allocate a new page if necessary.
    /// Thread-safe: can be called concurrently from multiple threads.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to set
    /// * `value` - The Vec<f64> to store
    pub fn set(&self, index: usize, value: Vec<f64>) {
        // Update capacity if needed
        {
            let mut capacity = self.capacity.write().unwrap();
            if index >= *capacity {
                *capacity = index + 1;
            }
        }

        let page_index = index >> PAGE_SHIFT;
        let page_offset = index & PAGE_MASK;

        // Get or create the page and set the value
        {
            let mut pages = self.pages.write().unwrap();
            let page = pages
                .entry(page_index)
                .or_insert_with(|| vec![self.default_value.clone(); PAGE_SIZE]);
            page[page_offset] = value;
        }

        // Track that this index has been set
        {
            let mut set_indices = self.set_indices.write().unwrap();
            set_indices.insert(index);
        }
    }

    /// Build an immutable HugeSparseDoubleArrayArray from the current state.
    ///
    /// The builder can continue to be used after calling build().
    pub fn build(&self) -> HugeSparseDoubleArrayArray {
        let capacity = *self.capacity.read().unwrap();
        let pages = self.pages.read().unwrap().clone();
        let set_indices = self.set_indices.read().unwrap().clone();

        HugeSparseDoubleArrayArray {
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
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(0, vec![1.1, 2.2]);
        builder.set(1, vec![3.3]);
        builder.set(100, vec![4.4, 5.5, 6.6]);

        let array = builder.build();

        assert_eq!(array.get(0), &vec![1.1, 2.2]);
        assert_eq!(array.get(1), &vec![3.3]);
        assert_eq!(array.get(100), &vec![4.4, 5.5, 6.6]);
        assert_eq!(array.get(50), &vec![0.0]); // default
    }

    #[test]
    fn test_contains() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(0, vec![1.1]);
        builder.set(100, vec![2.2]);

        let array = builder.build();

        assert!(array.contains(0));
        assert!(array.contains(100));
        assert!(!array.contains(1));
        assert!(!array.contains(50));
        assert!(!array.contains(99));
    }

    #[test]
    fn test_sparse_distribution() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);

        // Set values across widely distributed indices
        builder.set(0, vec![1.1]);
        builder.set(10_000_000, vec![2.2]);
        builder.set(50_000_000, vec![3.3]);
        builder.set(100_000_000, vec![4.4]);

        let array = builder.build();

        // Should only allocate 4 pages despite 100M capacity
        assert_eq!(array.page_count(), 4);
        assert_eq!(array.capacity(), 100_000_001);

        // Verify values
        assert_eq!(array.get(0), &vec![1.1]);
        assert_eq!(array.get(10_000_000), &vec![2.2]);
        assert_eq!(array.get(50_000_000), &vec![3.3]);
        assert_eq!(array.get(100_000_000), &vec![4.4]);
    }

    #[test]
    fn test_empty_vectors() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(0, vec![]); // explicitly set empty vector
        builder.set(1, vec![1.1, 2.2]);

        let array = builder.build();

        assert_eq!(array.get(0), &Vec::<f64>::new()); // explicitly set to empty
        assert_eq!(array.get(1), &vec![1.1, 2.2]);
        assert_eq!(array.get(2), &vec![0.0]); // default
        assert!(array.contains(0)); // explicitly set
        assert!(!array.contains(2)); // never set
    }

    #[test]
    fn test_default_value() {
        let default = vec![9.9, 8.8];
        let builder = HugeSparseDoubleArrayArray::builder(default.clone());
        builder.set(42, vec![1.1]);

        let array = builder.build();

        assert_eq!(array.get(42), &vec![1.1]);
        assert_eq!(array.get(0), &default);
        assert_eq!(array.get(100), &default);
    }

    #[test]
    fn test_capacity_growth() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(10, vec![1.1]);
        assert_eq!(*builder.capacity.read().unwrap(), 11);

        builder.set(1000, vec![2.2]);
        assert_eq!(*builder.capacity.read().unwrap(), 1001);

        builder.set(5, vec![3.3]); // shouldn't shrink
        assert_eq!(*builder.capacity.read().unwrap(), 1001);
    }

    #[test]
    fn test_page_boundaries() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);

        // Set values around page boundaries
        builder.set(PAGE_SIZE - 1, vec![1.1]); // last element of first page
        builder.set(PAGE_SIZE, vec![2.2]); // first element of second page
        builder.set(PAGE_SIZE + 1, vec![3.3]); // second element of second page

        let array = builder.build();

        assert_eq!(array.get(PAGE_SIZE - 1), &vec![1.1]);
        assert_eq!(array.get(PAGE_SIZE), &vec![2.2]);
        assert_eq!(array.get(PAGE_SIZE + 1), &vec![3.3]);
        assert_eq!(array.page_count(), 2);
    }

    #[test]
    fn test_overwrite() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(42, vec![1.1]);
        builder.set(42, vec![2.2, 3.3]); // overwrite

        let array = builder.build();

        assert_eq!(array.get(42), &vec![2.2, 3.3]);
    }

    #[test]
    fn test_builder_reuse() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);
        builder.set(0, vec![1.1]);

        let array1 = builder.build();
        assert_eq!(array1.get(0), &vec![1.1]);

        // Builder can be reused
        builder.set(1, vec![2.2]);
        let array2 = builder.build();
        assert_eq!(array2.get(0), &vec![1.1]);
        assert_eq!(array2.get(1), &vec![2.2]);
    }

    #[test]
    fn test_large_vectors() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);

        // Store large vectors
        let large_vec: Vec<f64> = (0..10_000).map(|i| i as f64 * 0.1).collect();
        builder.set(42, large_vec.clone());

        let array = builder.build();

        assert_eq!(array.get(42), &large_vec);
        assert_eq!(array.get(42).len(), 10_000);
    }

    #[test]
    fn test_floating_point_precision() {
        let builder = HugeSparseDoubleArrayArray::builder(vec![0.0]);

        use std::f64::consts::{E, PI, SQRT_2};
        builder.set(0, vec![PI]);
        builder.set(1, vec![E, SQRT_2]);
        builder.set(2, vec![1.0 / 3.0]);

        let array = builder.build();

        assert_eq!(array.get(0), &vec![PI]);
        assert_eq!(array.get(1), &vec![E, SQRT_2]);
        assert!((array.get(2)[0] - 0.333333333333333).abs() < 1e-10);
    }

    #[test]
    fn test_with_capacity() {
        let builder = HugeSparseDoubleArrayArray::builder_with_capacity(vec![0.0], 1000);
        assert_eq!(*builder.capacity.read().unwrap(), 1000);

        builder.set(500, vec![1.1]);
        let array = builder.build();

        assert_eq!(array.capacity(), 1000);
        assert_eq!(array.get(500), &vec![1.1]);
    }
}
