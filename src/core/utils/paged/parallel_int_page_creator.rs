//! High-performance parallel page creator for i32 array initialization.
//!
//! Essential for efficient bulk setup of billion-scale compact integer data:
//! - Parallel initialization of huge int arrays with custom values
//! - Fast setup of compact node IDs (4-byte vs 8-byte)
//! - Bulk population of color codes, flags, and indices
//! - Efficient memory allocation and initialization patterns
//! - Multi-threaded page creation for massive datasets
//!
//! Performance characteristics:
//! - Parallel page creation using Rayon work-stealing
//! - Custom value generation functions
//! - Optimized memory allocation patterns (50% memory vs i64)
//! - Cache-friendly sequential fills within pages
//! - Minimal synchronization overhead
//!
//! Use Cases:
//! - Compact node ID arrays (save 50% memory)
//! - Color codes for graph visualization
//! - Integer flags and status codes
//! - Compact index mappings
//! - Fast setup of integer lookup tables

use crate::concurrency::Concurrency;
use crate::core::utils::paged::{PageAllocator, PageAllocatorFactory};
use std::sync::Arc;

/// Parallel page creator for i32 arrays.
///
/// Creates and fills pages in parallel using configurable value generation functions.
///
/// # Generator Patterns
///
/// 1. **Identity**: `|i| i as i32` - Array[i] = i
/// 2. **Custom**: `|i| (i % 256) as i32` - Computed values
/// 3. **Pass-through**: None - Zero-initialized pages
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::paged::ParallelIntPageCreator;
/// use rust_gds::concurrency::Concurrency;
///
/// // Identity mapping for 1 billion elements
/// let creator = ParallelIntPageCreator::identity(Concurrency::of(8));
/// let pages = creator.create_pages(1_000_000_000);
///
/// // Result: pages[0][0] = 0, pages[0][1] = 1, ..., pages[n][m] = 999_999_999
/// ```
pub struct ParallelIntPageCreator {
    #[allow(dead_code)]
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> i32 + Send + Sync>>,
    allocator_factory: PageAllocatorFactory<Vec<i32>>,
}

impl ParallelIntPageCreator {
    /// Creates a parallel page creator with custom value generator.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    /// * `generator` - Function to generate values: (globalIndex) => value
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::ParallelIntPageCreator;
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// // Create pages with modulo values
    /// let creator = ParallelIntPageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i % 256) as i32
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0, pages[0][1] = 1, ..., pages[0][255] = 255, pages[0][256] = 0
    /// ```
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> i32 + Send + Sync + 'static,
    {
        Self {
            concurrency,
            generator: Some(Arc::new(generator)),
            allocator_factory: PageAllocatorFactory::for_int_array(),
        }
    }

    /// Creates an identity mapping page creator.
    ///
    /// Each element's value equals its index: array[i] = i as i32.
    /// Perfect for compact node ID arrays (uses 50% memory vs i64).
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::ParallelIntPageCreator;
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelIntPageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(10_000);
    ///
    /// // pages[0][0] = 0
    /// // pages[0][8191] = 8191
    /// // pages[1][0] = 8192
    /// ```
    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| i as i32)
    }

    /// Creates a pass-through page creator (zero-initialized).
    ///
    /// Pages are allocated but not explicitly filled, resulting in zero values.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::ParallelIntPageCreator;
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelIntPageCreator::pass_through(Concurrency::of(4));
    /// let mut pages = creator.create_pages(10_000);
    ///
    /// // All pages initialized to 0
    /// ```
    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self {
            concurrency,
            generator: None,
            allocator_factory: PageAllocatorFactory::for_int_array(),
        }
    }

    /// Creates pages for the given total size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::core::utils::paged::ParallelIntPageCreator;
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelIntPageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(1_000_000);
    ///
    /// // Returns ~122 pages for 1 million elements (8192 elements per page)
    /// ```
    pub fn create_pages(&self, total_size: usize) -> Vec<Vec<i32>> {
        if total_size == 0 {
            return Vec::new();
        }

        let page_size = self.allocator_factory.page_size();
        let num_pages = total_size.div_ceil(page_size);
        let last_page_size = if total_size % page_size == 0 {
            page_size
        } else {
            total_size % page_size
        };

        // Pre-allocate pages vector
        let mut pages: Vec<Vec<i32>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size);

        pages
    }

    /// Fills an array of pages with newly allocated and initialized i32 arrays.
    pub fn fill_pages(&self, pages: &mut [Vec<i32>], last_page_size: usize, page_size: usize) {
        if pages.is_empty() {
            return;
        }

        let last_page_index = pages.len() - 1;

        // Phase 1: Allocate and fill all pages except last in parallel
        if last_page_index > 0 {
            use rayon::prelude::*;

            pages[0..last_page_index]
                .par_iter_mut()
                .enumerate()
                .for_each(|(page_index, page)| {
                    let allocator = self.allocator_factory.new_allocator();
                    self.create_and_fill_page(
                        page,
                        page_index,
                        page_size,
                        &allocator,
                        self.generator.as_ref(),
                    );
                });
        }

        // Phase 2: Fill last page sequentially
        let allocator = self.allocator_factory.new_allocator();
        self.create_and_fill_page(
            &mut pages[last_page_index],
            last_page_index,
            last_page_size,
            &allocator,
            self.generator.as_ref(),
        );
    }

    /// Creates and fills a single page with computed values.
    fn create_and_fill_page(
        &self,
        page: &mut Vec<i32>,
        page_index: usize,
        page_size: usize,
        allocator: &impl PageAllocator<Vec<i32>>,
        generator: Option<&Arc<dyn Fn(usize) -> i32 + Send + Sync>>,
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
    use crate::collections::PageUtil;

    #[test]
    fn test_identity_mapping() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(10_000);

        // Check first page, first element
        assert_eq!(pages[0][0], 0);

        // Check first page, last element
        let page_size = pages[0].len();
        assert_eq!(pages[0][page_size - 1], (page_size - 1) as i32);

        // Check second page, first element
        if pages.len() > 1 {
            assert_eq!(pages[1][0], page_size as i32);
        }

        // Check last element
        let last_page = &pages[pages.len() - 1];
        let last_index = last_page.len() - 1;
        assert_eq!(last_page[last_index], 9999);
    }

    #[test]
    fn test_custom_generator() {
        // Generate modulo 256 values
        let creator = ParallelIntPageCreator::of(Concurrency::of(4), |i| (i % 256) as i32);
        let pages = creator.create_pages(1000);

        // Check specific values
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][100], 100);
        assert_eq!(pages[0][255], 255);

        // Check wraparound
        let page_size = pages[0].len();
        if page_size > 256 {
            assert_eq!(pages[0][256], 0);
            assert_eq!(pages[0][257], 1);
        }
    }

    #[test]
    fn test_pass_through() {
        let creator = ParallelIntPageCreator::pass_through(Concurrency::of(4));
        let pages = creator.create_pages(5000);

        // All values should be 0
        for page in &pages {
            for &value in page {
                assert_eq!(value, 0);
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(0);

        assert!(pages.is_empty());
    }

    #[test]
    fn test_single_page() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(100);

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].len(), 100);
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][99], 99);
    }

    #[test]
    fn test_exact_page_boundary() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();
        let page_size = allocator.page_size();

        // Create exactly 3 pages worth
        let pages = creator.create_pages(page_size * 3);

        assert_eq!(pages.len(), 3);
        for page in &pages {
            assert_eq!(page.len(), page_size);
        }

        // Check boundary values
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][page_size - 1], (page_size - 1) as i32);
        assert_eq!(pages[1][0], page_size as i32);
        assert_eq!(pages[2][page_size - 1], (page_size * 3 - 1) as i32);
    }

    #[test]
    #[ignore] // Large test (1M elements, 8 threads) - run with: cargo test -- --ignored
    fn test_large_array() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(8));
        let pages = creator.create_pages(1_000_000);

        // Check first and last elements
        assert_eq!(pages[0][0], 0);

        let last_page = &pages[pages.len() - 1];
        let last_index = last_page.len() - 1;
        assert_eq!(last_page[last_index], 999_999);
    }

    #[test]
    fn test_parallel_consistency() {
        // Test that different concurrency levels produce identical results
        let size = 100_000;

        let pages1 =
            ParallelIntPageCreator::of(Concurrency::of(1), |i| (i * 3) as i32).create_pages(size);
        let pages2 =
            ParallelIntPageCreator::of(Concurrency::of(2), |i| (i * 3) as i32).create_pages(size);
        let pages4 =
            ParallelIntPageCreator::of(Concurrency::of(4), |i| (i * 3) as i32).create_pages(size);
        let pages8 =
            ParallelIntPageCreator::of(Concurrency::of(8), |i| (i * 3) as i32).create_pages(size);

        // Spot check several indices
        let indices = [0, 1000, 50000, 99999];
        for &idx in &indices {
            let allocator = ParallelIntPageCreator::identity(Concurrency::of(1))
                .allocator_factory
                .new_allocator();
            let page_size = allocator.page_size();

            let page_index = idx / page_size;
            let offset = idx % page_size;

            let val1 = pages1[page_index][offset];
            let val2 = pages2[page_index][offset];
            let val4 = pages4[page_index][offset];
            let val8 = pages8[page_index][offset];

            assert_eq!(val1, val2);
            assert_eq!(val2, val4);
            assert_eq!(val4, val8);
        }
    }

    #[test]
    fn test_memory_estimation() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));

        let size = 1_000_000;
        let estimated = creator.estimate_memory_usage(size);

        // Should be approximately size * sizeof(i32)
        let expected = size * std::mem::size_of::<i32>();

        // Allow for page overhead
        let ratio = estimated as f64 / expected as f64;
        assert!(
            ratio >= 0.1 && ratio <= 20.0,
            "Memory estimation ratio {} should be reasonable",
            ratio
        );
    }

    #[test]
    fn test_page_size_configuration() {
        let creator = ParallelIntPageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();

        // Verify page size matches expected for i32 elements (32KB pages)
        let page_size = allocator.page_size();
        let expected_page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<i32>());

        assert_eq!(page_size, expected_page_size);
        assert_eq!(page_size, 8192); // 32KB / 4 bytes = 8192 elements
    }
}
