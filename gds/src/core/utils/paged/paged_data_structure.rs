//! Universal base structure for all paged data structures.
//!
//! Provides the essential infrastructure for billion-scale data storage:
//! - Thread-safe dynamic growth with lock-based coordination
//! - Efficient page-based indexing with bit manipulation
//! - Memory capacity management and estimation
//! - Atomic size and capacity tracking
//! - Generic page allocation through PageAllocator
//!
//! Foundation for all huge data structures:
//! - HugeLongArray: billion-element integer arrays
//! - HugeDoubleArray: massive floating-point datasets
//! - HugeObjectArray: large-scale object collections
//! - PagedLongStack: billion-element stack for DFS
//! - Matrix implementations: 2D paged storage
//! - Map/Set implementations: hash table backing
//!
//! # Performance Characteristics
//!
//! - O(1) index calculation using bit shifts and masks
//! - Minimal locking (only during growth operations)
//! - Power-of-2 page sizes for optimal bit manipulation
//! - Atomic operations for thread-safe size tracking
//! - Efficient memory estimation and capacity planning
//!
//! # Concurrency Features
//!
//! - Thread-safe growth with mutex semantics
//! - Atomic size and capacity counters
//! - Lock-free reads during normal operations
//! - Safe concurrent access during structure growth
//!
//! # Example
//!
//! ```rust
//! use gds::core::utils::paged::{PagedDataStructure, PageAllocatorFactory};
//! use std::sync::atomic::AtomicUsize;
//!
//! // Custom paged structure for demonstration
//! struct MyPagedArray {
//!     base: PagedDataStructure<Vec<i64>>,
//! }
//!
//! impl MyPagedArray {
//!     fn new(size: usize) -> Self {
//!         let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
//!         let allocator = factory.new_allocator();
//!         Self {
//!             base: PagedDataStructure::new(size, allocator),
//!         }
//!     }
//!
//!     fn get(&self, index: usize) -> Option<i64> {
//!         if index >= self.base.size() {
//!             return None;
//!         }
//!         let page_idx = self.base.page_index(index);
//!         let idx_in_page = self.base.index_in_page(index);
//!         self.base.pages()[page_idx].get(idx_in_page).copied()
//!     }
//! }
//! ```

use crate::collections::PageUtil;
use crate::core::utils::paged::PageAllocator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// Thread-safe atomic counter with padding to avoid false sharing.
///
/// Simulates Java's `@Contended` annotation by adding padding to prevent
/// false cache line sharing between atomic counters.
#[repr(align(64))] // Cache line alignment
struct PaddedAtomicUsize {
    value: AtomicUsize,
    #[allow(dead_code)]
    padding: [u8; 56], // 64 - 8 = 56 bytes padding
}

impl PaddedAtomicUsize {
    fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
            padding: [0; 56],
        }
    }

    fn get(&self) -> usize {
        self.value.load(Ordering::Acquire)
    }

    fn set(&self, value: usize) {
        self.value.store(value, Ordering::Release);
    }

    fn get_and_set(&self, value: usize) -> usize {
        self.value.swap(value, Ordering::AcqRel)
    }

    fn compare_and_set(&self, expected: usize, update: usize) -> bool {
        self.value
            .compare_exchange(expected, update, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

/// Universal base structure for paged data structures.
///
/// Provides thread-safe growth, efficient indexing, and memory management
/// for billion-scale data storage.
///
/// # Type Parameter
///
/// * `T` - The type of page (e.g., `Vec<i64>`, `Vec<f64>`)
pub struct PagedDataStructure<T> {
    /// Number of elements per page
    #[allow(dead_code)]
    page_size: usize,
    /// Bit shift for page index calculation (log2 of page_size)
    page_shift: u32,
    /// Bit mask for index-in-page calculation (page_size - 1)
    page_mask: usize,
    /// Maximum supported size based on bit manipulation limits
    max_supported_size: usize,

    /// Page storage - can grow dynamically
    pages: Mutex<Vec<T>>,

    /// Current size of the data structure (filled elements)
    size_counter: PaddedAtomicUsize,
    /// Current capacity (allocated elements)
    capacity_counter: PaddedAtomicUsize,

    /// Page allocator for creating new pages
    allocator: Box<dyn PageAllocator<T> + Send + Sync>,
}

impl<T> PagedDataStructure<T> {
    /// Creates a new paged data structure with specified size and allocator.
    ///
    /// # Arguments
    ///
    /// * `size` - Initial number of elements
    /// * `allocator` - Page allocator for creating pages
    ///
    /// # Panics
    ///
    /// Panics if size exceeds maximum supported size (2^31 * page_size)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gds::core::utils::paged::{PagedDataStructure, PageAllocatorFactory};
    ///
    /// let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
    /// let allocator = factory.new_allocator();
    /// let paged = PagedDataStructure::new(1_000_000, allocator);
    ///
    /// assert_eq!(paged.size(), 1_000_000);
    /// assert!(paged.capacity() >= 1_000_000);
    /// ```
    pub fn new<A>(size: usize, allocator: A) -> Self
    where
        A: PageAllocator<T> + Send + Sync + 'static,
    {
        let page_size = allocator.page_size();
        let page_shift = page_size.trailing_zeros();
        let page_mask = page_size - 1;

        // Calculate maximum supported size (2^31 * page_size)
        let max_index_shift = 31 + page_shift;
        let max_supported_size = if max_index_shift < 63 {
            1usize << max_index_shift
        } else {
            usize::MAX
        };

        assert!(
            size <= max_supported_size,
            "Size {} exceeds maximum supported size {}",
            size,
            max_supported_size
        );

        let num_pages = PageUtil::num_pages_for(size, page_size);
        let mut pages = Vec::with_capacity(num_pages);

        // Allocate initial pages
        for _ in 0..num_pages {
            pages.push(allocator.new_page());
        }

        let capacity = num_pages << page_shift;

        Self {
            page_size,
            page_shift,
            page_mask,
            max_supported_size,
            pages: Mutex::new(pages),
            size_counter: PaddedAtomicUsize::new(size),
            capacity_counter: PaddedAtomicUsize::new(capacity),
            allocator: Box::new(allocator),
        }
    }

    /// Returns the current size of the data structure.
    ///
    /// Indices up to this size have been filled with data.
    ///
    /// # Returns
    ///
    /// Current size (number of filled elements)
    pub fn size(&self) -> usize {
        self.size_counter.get()
    }

    /// Returns the current capacity of the data structure.
    ///
    /// The structure can safely be written up to this index (exclusive).
    ///
    /// # Returns
    ///
    /// Current capacity (number of allocated elements)
    pub fn capacity(&self) -> usize {
        self.capacity_counter.get()
    }

    /// Releases all resources and returns freed memory estimate.
    ///
    /// # Warning
    ///
    /// Invalidates the data structure - do not use after calling this!
    ///
    /// # Returns
    ///
    /// Estimated bytes freed
    pub fn release(&self) -> usize {
        self.size_counter.set(0);
        let freed = self
            .allocator
            .estimate_memory_usage(self.capacity_counter.get_and_set(0));
        let mut pages = self.pages.lock().unwrap();
        pages.clear();
        freed
    }

    /// Returns memory usage estimation in bytes.
    ///
    /// # Returns
    ///
    /// Estimated memory usage
    pub fn size_of(&self) -> usize {
        self.allocator.estimate_memory_usage(self.capacity())
    }

    /// Accesses the pages directly (for subclass implementation).
    ///
    /// # Returns
    ///
    /// Reference to page storage
    ///
    /// # Note
    ///
    /// This is protected in the design - only subclasses should access pages.
    /// In Rust, we make it public but document that it's for internal use.
    pub fn pages(&self) -> std::sync::MutexGuard<'_, Vec<T>> {
        self.pages.lock().unwrap()
    }

    /// Calculates the number of pages needed for given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Number of elements
    ///
    /// # Returns
    ///
    /// Number of pages required
    pub fn num_pages(&self, capacity: usize) -> usize {
        PageUtil::num_pages_for_shift(capacity, self.page_shift, self.page_mask)
    }

    /// Calculates capacity for a given number of pages.
    ///
    /// # Arguments
    ///
    /// * `num_pages` - Number of pages
    ///
    /// # Returns
    ///
    /// Total capacity in elements
    pub fn capacity_for(&self, num_pages: usize) -> usize {
        num_pages << self.page_shift
    }

    /// Efficiently calculates page index for a given element index.
    ///
    /// Uses bit shifting for O(1) performance.
    ///
    /// # Arguments
    ///
    /// * `index` - Element index
    ///
    /// # Returns
    ///
    /// Page index containing the element
    pub fn page_index(&self, index: usize) -> usize {
        index >> self.page_shift
    }

    /// Efficiently calculates index within page for a given element index.
    ///
    /// Uses bit masking for O(1) performance.
    ///
    /// # Arguments
    ///
    /// * `index` - Element index
    ///
    /// # Returns
    ///
    /// Index within the page
    pub fn index_in_page(&self, index: usize) -> usize {
        index & self.page_mask
    }

    /// Thread-safe growth to accommodate new size.
    ///
    /// Preserves existing content and is no-op if already large enough.
    ///
    /// # Arguments
    ///
    /// * `new_size` - Target size for the structure
    ///
    /// # Panics
    ///
    /// Panics if new_size exceeds maximum supported size
    pub fn grow(&self, new_size: usize) {
        self.grow_with_skip(new_size, None);
    }

    /// Thread-safe growth with optional page skip.
    ///
    /// # Arguments
    ///
    /// * `new_size` - Target size for the structure
    /// * `skip_page` - Optional page index to skip during allocation
    pub fn grow_with_skip(&self, new_size: usize, skip_page: Option<usize>) {
        assert!(
            new_size <= self.max_supported_size,
            "New size {} exceeds maximum {}",
            new_size,
            self.max_supported_size
        );

        // Fast path - no growth needed
        if self.capacity_counter.get() >= new_size {
            self.grow_size(new_size);
            return;
        }

        // Slow path - need to grow
        let mut pages = self.pages.lock().unwrap();

        // Double-check after acquiring lock
        if self.capacity_counter.get() >= new_size {
            self.grow_size(new_size);
            return;
        }

        // Allocate new pages
        let current_num_pages = pages.len();
        let target_num_pages = self.num_pages(new_size);

        for page_idx in current_num_pages..target_num_pages {
            if Some(page_idx) != skip_page {
                pages.push(self.allocator.new_page());
            }
        }

        self.capacity_counter.set(self.capacity_for(pages.len()));
        drop(pages);

        self.grow_size(new_size);
    }

    /// Atomically updates the size counter.
    ///
    /// Uses compare-and-swap for thread safety.
    fn grow_size(&self, new_size: usize) {
        loop {
            let current_size = self.size_counter.get();
            if current_size >= new_size {
                break;
            }
            if self.size_counter.compare_and_set(current_size, new_size) {
                break;
            }
            // CAS failed, retry
        }
    }
}

// Safety: PagedDataStructure is thread-safe through Mutex and atomic operations
unsafe impl<T: Send> Send for PagedDataStructure<T> {}
unsafe impl<T: Send> Sync for PagedDataStructure<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::paged::PageAllocatorFactory;

    #[test]
    fn test_new_paged_structure() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = PagedDataStructure::new(10_000, allocator);

        assert_eq!(paged.size(), 10_000);
        assert!(paged.capacity() >= 10_000);
    }

    #[test]
    fn test_page_calculations() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = PagedDataStructure::new(10_000, allocator);

        let page_size = paged.page_size;

        // Test page_index
        assert_eq!(paged.page_index(0), 0);
        assert_eq!(paged.page_index(page_size - 1), 0);
        assert_eq!(paged.page_index(page_size), 1);
        assert_eq!(paged.page_index(2 * page_size + 100), 2);

        // Test index_in_page
        assert_eq!(paged.index_in_page(0), 0);
        assert_eq!(paged.index_in_page(page_size - 1), page_size - 1);
        assert_eq!(paged.index_in_page(page_size), 0);
        assert_eq!(paged.index_in_page(2 * page_size + 100), 100);
    }

    #[test]
    fn test_grow() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = PagedDataStructure::new(1000, allocator);

        let initial_capacity = paged.capacity();
        assert_eq!(paged.size(), 1000);

        paged.grow(100_000);

        assert_eq!(paged.size(), 100_000);
        assert!(paged.capacity() >= 100_000);
        assert!(paged.capacity() > initial_capacity);
    }

    #[test]
    fn test_grow_no_op() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = PagedDataStructure::new(10_000, allocator);

        let initial_capacity = paged.capacity();

        // Growing to smaller size is no-op
        paged.grow(5_000);

        assert_eq!(paged.size(), 10_000); // Size doesn't shrink
        assert_eq!(paged.capacity(), initial_capacity); // Capacity unchanged
    }

    #[test]
    fn test_concurrent_growth() {
        use std::sync::Arc;

        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = Arc::new(PagedDataStructure::new(1000, allocator));

        std::thread::scope(|s| {
            for _ in 0..4 {
                let paged = Arc::clone(&paged);
                s.spawn(move || {
                    paged.grow(100_000);
                });
            }
        });

        assert_eq!(paged.size(), 100_000);
        assert!(paged.capacity() >= 100_000);
    }

    #[test]
    fn test_release() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();
        let paged = PagedDataStructure::new(10_000, allocator);

        let memory_before = paged.size_of();
        assert!(memory_before > 0);

        let freed = paged.release();
        assert_eq!(freed, memory_before);

        assert_eq!(paged.size(), 0);
        assert_eq!(paged.capacity(), 0);
    }
}
