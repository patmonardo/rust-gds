//! High-performance parallel page creator for u8 array initialization.
//!
//! Essential for efficient bulk setup of billion-scale byte data:
//! - Parallel initialization of huge byte arrays with custom values
//! - Fast setup of flags, masks, and compact storage
//! - Bulk population of boolean arrays (8x more compact than bool)
//! - Efficient memory allocation and initialization patterns
//! - Multi-threaded page creation for massive datasets
//!
//! Performance characteristics:
//! - Parallel page creation using Rayon work-stealing
//! - Custom value generation functions
//! - Optimized memory allocation patterns (87.5% memory savings vs i64)
//! - Cache-friendly sequential fills within pages
//! - Minimal synchronization overhead
//!
//! Use Cases:
//! - Boolean flags (visited, marked, active)
//! - Bit masks and compact storage
//! - Color channels and image data
//! - Compact state machines
//! - Fast setup of byte lookup tables

use crate::concurrency::Concurrency;
use crate::core::utils::paged::{PageAllocator, PageAllocatorFactory};
use std::sync::Arc;

/// Parallel page creator for u8 arrays.
///
/// Creates and fills pages in parallel using configurable value generation functions.
///
/// # Generator Patterns
///
/// 1. **Identity**: `|i| (i & 0xFF) as u8` - Array[i] = i mod 256
/// 2. **Custom**: `|i| if i % 2 == 0 { 1 } else { 0 }` - Computed values
/// 3. **Pass-through**: None - Zero-initialized pages
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::ParallelBytePageCreator;
/// use gds::concurrency::Concurrency;
///
/// // Identity mapping for 1 billion elements
/// let creator = ParallelBytePageCreator::identity(Concurrency::of(8));
/// let pages = creator.create_pages(1_000_000_000);
///
/// // Result: pages[0][0] = 0, pages[0][1] = 1, ..., pages[0][255] = 255, pages[0][256] = 0
/// ```
pub struct ParallelBytePageCreator {
    #[allow(dead_code)]
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> u8 + Send + Sync>>,
    allocator_factory: PageAllocatorFactory<Vec<u8>>,
}

impl ParallelBytePageCreator {
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
    /// use gds::core::utils::paged::ParallelBytePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create pages with alternating 0/1 values
    /// let creator = ParallelBytePageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i % 2) as u8
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0, pages[0][1] = 1, pages[0][2] = 0, pages[0][3] = 1, ...
    /// ```
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> u8 + Send + Sync + 'static,
    {
        Self {
            concurrency,
            generator: Some(Arc::new(generator)),
            allocator_factory: PageAllocatorFactory::for_byte_array(),
        }
    }

    /// Creates an identity mapping page creator (index mod 256).
    ///
    /// Each element's value equals its index mod 256: array[i] = (i & 0xFF) as u8.
    /// Perfect for repeating patterns and compact storage.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelBytePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelBytePageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(10_000);
    ///
    /// // pages[0][0] = 0
    /// // pages[0][255] = 255
    /// // pages[0][256] = 0  (wraps)
    /// ```
    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| (i & 0xFF) as u8)
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
    /// use gds::core::utils::paged::ParallelBytePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelBytePageCreator::pass_through(Concurrency::of(4));
    /// let mut pages = creator.create_pages(10_000);
    ///
    /// // All pages initialized to 0
    /// ```
    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self {
            concurrency,
            generator: None,
            allocator_factory: PageAllocatorFactory::for_byte_array(),
        }
    }

    /// Creates pages for the given total size.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelBytePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelBytePageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(1_000_000);
    ///
    /// // Returns ~31 pages for 1 million elements (32768 elements per page)
    /// ```
    pub fn create_pages(&self, total_size: usize) -> Vec<Vec<u8>> {
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
        let mut pages: Vec<Vec<u8>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size);

        pages
    }

    /// Fills an array of pages with newly allocated and initialized u8 arrays.
    pub fn fill_pages(&self, pages: &mut [Vec<u8>], last_page_size: usize, page_size: usize) {
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
        page: &mut Vec<u8>,
        page_index: usize,
        page_size: usize,
        allocator: &impl PageAllocator<Vec<u8>>,
        generator: Option<&Arc<dyn Fn(usize) -> u8 + Send + Sync>>,
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
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(10_000);

        // Check first 256 values (0-255)
        for i in 0..256 {
            assert_eq!(pages[0][i], i as u8);
        }

        // Check wraparound
        let page_size = pages[0].len();
        if page_size > 256 {
            assert_eq!(pages[0][256], 0);
            assert_eq!(pages[0][257], 1);
            assert_eq!(pages[0][511], 255);
            assert_eq!(pages[0][512], 0);
        }
    }

    #[test]
    fn test_custom_generator() {
        // Generate alternating 0/1 values
        let creator = ParallelBytePageCreator::of(Concurrency::of(4), |i| (i % 2) as u8);
        let pages = creator.create_pages(1000);

        // Check alternating pattern
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][1], 1);
        assert_eq!(pages[0][2], 0);
        assert_eq!(pages[0][3], 1);
        assert_eq!(pages[0][100], 0);
        assert_eq!(pages[0][101], 1);
    }

    #[test]
    fn test_pass_through() {
        let creator = ParallelBytePageCreator::pass_through(Concurrency::of(4));
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
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(0);

        assert!(pages.is_empty());
    }

    #[test]
    fn test_single_page() {
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(100);

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].len(), 100);
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][99], 99);
    }

    #[test]
    fn test_exact_page_boundary() {
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();
        let page_size = allocator.page_size();

        // Create exactly 3 pages worth
        let pages = creator.create_pages(page_size * 3);

        assert_eq!(pages.len(), 3);
        for page in &pages {
            assert_eq!(page.len(), page_size);
        }

        // Check boundary values (with wraparound for u8)
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][255], 255);
    }

    #[test]
    fn test_large_array() {
        let creator = ParallelBytePageCreator::identity(Concurrency::of(8));
        let pages = creator.create_pages(1_000_000);

        // Check first and some known values
        assert_eq!(pages[0][0], 0);
        assert_eq!(pages[0][255], 255);
        assert_eq!(pages[0][256], 0);
    }

    #[test]
    fn test_parallel_consistency() {
        // Test that different concurrency levels produce identical results
        let size = 100_000;

        let pages1 = ParallelBytePageCreator::of(Concurrency::of(1), |i| ((i * 3) & 0xFF) as u8)
            .create_pages(size);
        let pages2 = ParallelBytePageCreator::of(Concurrency::of(2), |i| ((i * 3) & 0xFF) as u8)
            .create_pages(size);
        let pages4 = ParallelBytePageCreator::of(Concurrency::of(4), |i| ((i * 3) & 0xFF) as u8)
            .create_pages(size);
        let pages8 = ParallelBytePageCreator::of(Concurrency::of(8), |i| ((i * 3) & 0xFF) as u8)
            .create_pages(size);

        // Spot check several indices
        let indices = [0, 1000, 50000, 99999];
        for &idx in &indices {
            let allocator = ParallelBytePageCreator::identity(Concurrency::of(1))
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
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));

        let size = 1_000_000;
        let estimated = creator.estimate_memory_usage(size);

        // Should be approximately size * sizeof(u8)
        let expected = size * std::mem::size_of::<u8>();

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
        let creator = ParallelBytePageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();

        // Verify page size matches expected for u8 elements (32KB pages)
        let page_size = allocator.page_size();
        let expected_page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<u8>());

        assert_eq!(page_size, expected_page_size);
        assert_eq!(page_size, 32768); // 32KB / 1 byte = 32768 elements
    }
}
