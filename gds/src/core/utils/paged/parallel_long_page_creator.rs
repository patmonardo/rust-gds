//! High-performance parallel page creator for huge array initialization.
//!
//! Essential for efficient bulk setup of billion-scale data structures:
//! - Parallel initialization of huge arrays with custom values
//! - Fast setup of identity mappings (node ID arrays)
//! - Bulk population of computed sequences (fibonacci, primes, etc.)
//! - Efficient memory allocation and initialization patterns
//! - Multi-threaded page creation for massive datasets
//!
//! Performance characteristics:
//! - Parallel page creation using worker pool
//! - Custom value generation functions
//! - Optimized memory allocation patterns
//! - Cache-friendly sequential fills within pages
//! - Minimal synchronization overhead
//!
//! Use Cases:
//! - Node ID arrays for graph structures
//! - Index mappings for data reorganization
//! - Precomputed sequences for mathematical operations
//! - Bulk initialization of sparse data structures
//! - Fast setup of lookup tables and mappings

use crate::concurrency::Concurrency;
use crate::core::utils::paged::{PageAllocator, PageAllocatorFactory};
use std::sync::Arc;

/// Parallel page creator for i64 arrays.
///
/// Creates and fills pages in parallel using configurable value generation functions.
///
/// # Generator Patterns
///
/// 1. **Identity**: `|i| i as i64` - Array[i] = i
/// 2. **Custom**: `|i| i * i` - Computed values
/// 3. **Pass-through**: None - Zero-initialized pages
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::ParallelLongPageCreator;
/// use gds::concurrency::Concurrency;
///
/// // Identity mapping for 1 billion elements
/// let creator = ParallelLongPageCreator::identity(Concurrency::of(8));
/// let pages = creator.create_pages(1_000_000_000);
///
/// // Result: pages[0][0] = 0, pages[0][1] = 1, ..., pages[n][m] = 999_999_999
/// ```
pub struct ParallelLongPageCreator {
    #[allow(dead_code)]
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> i64 + Send + Sync>>,
    allocator_factory: PageAllocatorFactory<Vec<i64>>,
}

impl ParallelLongPageCreator {
    /// Creates a parallel page creator with custom value generator.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    /// * `generator` - Function to generate values: (globalIndex) => value
    ///
    /// # Returns
    ///
    /// New parallel page creator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelLongPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create pages with squared values
    /// let creator = ParallelLongPageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i * i) as i64
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0, pages[0][1] = 1, pages[0][2] = 4, pages[0][3] = 9, ...
    /// ```
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> i64 + Send + Sync + 'static,
    {
        Self {
            concurrency,
            generator: Some(Arc::new(generator)),
            allocator_factory: PageAllocatorFactory::for_long_array(),
        }
    }

    /// Creates an identity mapping page creator.
    ///
    /// Each element's value equals its index: array[i] = i.
    /// Perfect for node ID arrays and identity transformations.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// Identity page creator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelLongPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create node ID mapping for graph with 1M nodes
    /// let creator = ParallelLongPageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(1_000_000);
    ///
    /// // Result: pages[0] = [0, 1, 2, ..., 4095]
    /// //         pages[1] = [4096, 4097, ..., 8191]
    /// //         etc.
    /// ```
    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| i as i64)
    }

    /// Creates a pass-through page creator that doesn't initialize values.
    ///
    /// Pages are allocated but left with default values (zeros).
    /// Useful when you need page structure but will fill values later.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// Pass-through page creator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelLongPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Allocate pages without initialization
    /// let creator = ParallelLongPageCreator::pass_through(Concurrency::of(4));
    /// let pages = creator.create_pages(100_000);
    ///
    /// // Result: pages allocated but filled with zeros
    /// // Fill with custom logic later...
    /// ```
    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self {
            concurrency,
            generator: None,
            allocator_factory: PageAllocatorFactory::for_long_array(),
        }
    }

    /// Creates and fills pages for the specified total size.
    ///
    /// This is the main entry point that orchestrates parallel page creation and filling.
    ///
    /// # Arguments
    ///
    /// * `total_size` - Total number of elements to store
    ///
    /// # Returns
    ///
    /// Vector of pages containing the initialized data
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelLongPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelLongPageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(10_000);
    ///
    /// assert_eq!(pages.len(), 3); // ceil(10_000 / 4_096) = 3 pages
    /// assert_eq!(pages[0].len(), 4096);
    /// assert_eq!(pages[1].len(), 4096);
    /// assert_eq!(pages[2].len(), 1808); // 10_000 - 8_192
    /// ```
    pub fn create_pages(&self, total_size: usize) -> Vec<Vec<i64>> {
        if total_size == 0 {
            return Vec::new();
        }

        let page_size = self.allocator_factory.page_size();
        let num_pages = total_size.div_ceil(page_size);
        let last_page_size = if total_size.is_multiple_of(page_size) {
            page_size
        } else {
            total_size % page_size
        };

        // Pre-allocate pages vector
        let mut pages: Vec<Vec<i64>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size);

        pages
    }

    /// Fills an array of pages with newly allocated and initialized i64 arrays.
    ///
    /// All pages except the last one will have size equal to page_size.
    /// The last page will have the specified last_page_size.
    ///
    /// # Arguments
    ///
    /// * `pages` - Mutable slice of pages to fill
    /// * `last_page_size` - Size of the last page
    /// * `page_size` - Size of regular pages
    ///
    /// # Performance
    ///
    /// O(numPages / concurrency) with parallel speedup
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelLongPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelLongPageCreator::identity(Concurrency::of(4));
    /// let mut pages = vec![Vec::new(); 100];
    ///
    /// creator.fill_pages(&mut pages, 4096, 4096);
    ///
    /// // All pages now filled with identity mapping
    /// assert_eq!(pages[0][0], 0);
    /// assert_eq!(pages[0][4095], 4095);
    /// assert_eq!(pages[1][0], 4096);
    /// ```
    pub fn fill_pages(&self, pages: &mut [Vec<i64>], last_page_size: usize, page_size: usize) {
        if pages.is_empty() {
            return;
        }

        let last_page_index = pages.len() - 1;

        // Phase 1: Allocate and fill all pages except last in parallel
        if last_page_index > 0 {
            use rayon::prelude::*;

            let generator = self.generator.clone();
            let allocator = self.allocator_factory.new_allocator();

            // Use rayon's par_iter_mut to give each thread exclusive access to its page
            pages[0..last_page_index]
                .par_iter_mut()
                .enumerate()
                .for_each(|(page_index, page)| {
                    self.create_and_fill_page(
                        page,
                        page_index,
                        page_size,
                        &allocator,
                        generator.as_ref(),
                    );
                });
        }

        // Phase 2: Handle last page separately (might have different size)
        let allocator = self.allocator_factory.new_allocator();
        self.create_and_fill_page(
            &mut pages[last_page_index],
            last_page_index,
            last_page_size,
            &allocator,
            self.generator.as_ref(),
        );
    }

    /// Fills a single page with data.
    ///
    /// If a generator is configured, fills with generated values.
    /// Otherwise, leaves page with default values (zeros).
    ///
    /// # Arguments
    ///
    /// * `page` - Page to fill
    /// * `base` - Base index for this page (starting global index)
    ///
    /// # Generator Use Cases
    ///
    /// - Identity mapping: `|i| i as i64`
    /// - Random values: `|i| (i * 1103515245 + 12345) as i64` (LCG)
    /// - Mathematical functions: `|i| (i * i) as i64`
    /// - Sparse mappings: `|i| (i * 100 + 42) as i64`
    pub fn fill_page(&self, page: &mut Vec<i64>, base: usize) {
        if let Some(ref gen) = self.generator {
            for i in 0..page.capacity() {
                page.push(gen(base + i));
            }
        } else {
            // Pass-through: fill with zeros
            page.resize(page.capacity(), 0);
        }
    }

    /// Creates and fills a single page with computed values.
    ///
    /// Allocates page using the allocator and populates it using the generator function.
    ///
    /// # Arguments
    ///
    /// * `page` - Mutable reference to page to fill
    /// * `page_index` - Index of this page
    /// * `page_size` - Size of the page to create
    /// * `allocator` - PageAllocator for creating pages
    /// * `generator` - Optional generator function
    fn create_and_fill_page(
        &self,
        page: &mut Vec<i64>,
        page_index: usize,
        page_size: usize,
        allocator: &impl PageAllocator<Vec<i64>>,
        generator: Option<&Arc<dyn Fn(usize) -> i64 + Send + Sync>>,
    ) {
        // Create page with correct capacity
        *page = allocator.new_page();

        // Calculate base index for this page
        let base = page_index * allocator.page_size();

        // Fill with generated values or zeros
        if let Some(gen) = generator {
            for i in 0..page_size {
                page.push(gen(base + i));
            }
        } else {
            page.resize(page_size, 0);
        }
    }

    /// Gets the page size used by this creator.
    pub fn page_size(&self) -> usize {
        self.allocator_factory.page_size()
    }

    /// Estimates memory usage for the given total size.
    pub fn estimate_memory_usage(&self, total_size: usize) -> usize {
        self.allocator_factory.estimate_memory_usage(total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_mapping() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(2));
        let pages = creator.create_pages(10_000);

        // Check first page
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][100], 100);
        assert_eq!(pages[0][4095], 4095);

        // Check second page
        assert_eq!(pages[1][0], 4096);
        assert_eq!(pages[1][100], 4196);

        // Check last page (partial)
        let last_page = &pages[pages.len() - 1];
        let last_index = 9999;
        let last_page_offset = last_index % 4096;
        assert_eq!(last_page[last_page_offset], last_index as i64);
    }

    #[test]
    fn test_custom_generator() {
        let creator = ParallelLongPageCreator::of(Concurrency::of(4), |i| (i * i) as i64);
        let pages = creator.create_pages(1000);

        assert_eq!(pages[0][0], 0); // 0^2
        assert_eq!(pages[0][1], 1); // 1^2
        assert_eq!(pages[0][2], 4); // 2^2
        assert_eq!(pages[0][10], 100); // 10^2
    }

    #[test]
    fn test_pass_through() {
        let creator = ParallelLongPageCreator::pass_through(Concurrency::of(2));
        let pages = creator.create_pages(1000);

        // All values should be zero
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][500], 0);
    }

    #[test]
    fn test_empty_array() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(0);

        assert_eq!(pages.len(), 0);
    }

    #[test]
    fn test_single_page() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(100);

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].len(), 100);
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][99], 99);
    }

    #[test]
    fn test_exact_page_boundary() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(4));
        let page_size = creator.page_size();
        let pages = creator.create_pages(page_size * 2);

        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].len(), page_size);
        assert_eq!(pages[1].len(), page_size);

        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[1][0], page_size as i64);
    }

    #[test]
    fn test_large_array() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(8));
        let pages = creator.create_pages(1_000_000);

        let page_size = creator.page_size();
        let num_pages = (1_000_000 + page_size - 1) / page_size;

        assert_eq!(pages.len(), num_pages);

        // Verify first and last elements
        assert_eq!(pages[0][0], 0);

        let last_page = &pages[pages.len() - 1];
        let last_element_index = 999_999;
        let last_page_offset = last_element_index % page_size;
        assert_eq!(last_page[last_page_offset], 999_999);
    }

    #[test]
    fn test_parallel_consistency() {
        // Run with different concurrency levels and verify consistency
        let sizes = vec![1_000, 10_000, 100_000];
        let concurrencies = vec![1, 2, 4, 8];

        for size in sizes {
            let reference =
                ParallelLongPageCreator::identity(Concurrency::of(1)).create_pages(size);

            for concurrency in &concurrencies {
                let pages = ParallelLongPageCreator::identity(Concurrency::of(*concurrency))
                    .create_pages(size);

                assert_eq!(pages.len(), reference.len());

                // Check all values match
                for (page_idx, page) in pages.iter().enumerate() {
                    for (elem_idx, &value) in page.iter().enumerate() {
                        assert_eq!(
                            value, reference[page_idx][elem_idx],
                            "Mismatch at page {} element {} with concurrency {}",
                            page_idx, elem_idx, concurrency
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_memory_estimation() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(4));

        let mem_1k = creator.estimate_memory_usage(1_000);
        let mem_1m = creator.estimate_memory_usage(1_000_000);
        let mem_1b = creator.estimate_memory_usage(1_000_000_000);

        assert!(mem_1m > mem_1k);
        assert!(mem_1b > mem_1m);

        // Should be roughly proportional (allow wider range due to page overhead)
        let ratio = mem_1m as f64 / mem_1k as f64;
        assert!(ratio > 100.0 && ratio < 2000.0); // Rough check with page granularity
    }

    #[test]
    fn test_page_size_configuration() {
        let creator = ParallelLongPageCreator::identity(Concurrency::of(4));

        // Page size should be 4096 for 32KB pages with 8-byte i64s
        assert_eq!(creator.page_size(), 4096);
    }
}
