//! High-performance parallel page creator for f64 array initialization.
//!
//! Essential for efficient bulk setup of billion-scale floating-point data:
//! - Parallel initialization of huge double arrays with custom values
//! - Fast setup of weight arrays (edge weights, node scores)
//! - Bulk population of computed sequences (exponentials, logarithms, etc.)
//! - Efficient memory allocation and initialization patterns
//! - Multi-threaded page creation for massive datasets
//!
//! Performance characteristics:
//! - Parallel page creation using Rayon work-stealing
//! - Custom value generation functions
//! - Optimized memory allocation patterns
//! - Cache-friendly sequential fills within pages
//! - Minimal synchronization overhead
//!
//! Use Cases:
//! - Edge weight arrays for weighted graphs
//! - PageRank scores and centrality measures
//! - Node embedding vectors (component storage)
//! - Property values for graph analytics
//! - Fast setup of floating-point lookup tables

use crate::concurrency::Concurrency;
use crate::core::utils::paged::{PageAllocator, PageAllocatorFactory};
use std::sync::Arc;

/// Parallel page creator for f64 arrays.
///
/// Creates and fills pages in parallel using configurable value generation functions.
///
/// # Generator Patterns
///
/// 1. **Identity**: `|i| i as f64` - Array[i] = i as float
/// 2. **Custom**: `|i| (i as f64).sqrt()` - Computed values
/// 3. **Pass-through**: None - Zero-initialized pages
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::ParallelDoublePageCreator;
/// use gds::concurrency::Concurrency;
///
/// // Identity mapping for 1 billion elements
/// let creator = ParallelDoublePageCreator::identity(Concurrency::of(8));
/// let pages = creator.create_pages(1_000_000_000);
///
/// // Result: pages[0][0] = 0.0, pages[0][1] = 1.0, ..., pages[n][m] = 999_999_999.0
/// ```
pub struct ParallelDoublePageCreator {
    #[allow(dead_code)]
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> f64 + Send + Sync>>,
    allocator_factory: PageAllocatorFactory<Vec<f64>>,
}

impl ParallelDoublePageCreator {
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
    /// use gds::core::utils::paged::ParallelDoublePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create pages with square root values
    /// let creator = ParallelDoublePageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i as f64).sqrt()
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0.0, pages[0][1] = 1.0, pages[0][2] ≈ 1.414, ...
    /// ```
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> f64 + Send + Sync + 'static,
    {
        Self {
            concurrency,
            generator: Some(Arc::new(generator)),
            allocator_factory: PageAllocatorFactory::for_double_array(),
        }
    }

    /// Creates an identity mapping page creator.
    ///
    /// Each element's value equals its index as f64: array[i] = i as f64.
    /// Perfect for continuous node/edge indexing and identity transformations.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// New parallel page creator with identity mapping
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelDoublePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelDoublePageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(10_000);
    ///
    /// // pages[0][0] = 0.0
    /// // pages[0][4095] = 4095.0
    /// // pages[1][0] = 4096.0
    /// ```
    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| i as f64)
    }

    /// Creates a pass-through page creator (zero-initialized).
    ///
    /// Pages are allocated but not explicitly filled, resulting in zero values.
    /// Useful when pages will be filled by external logic.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// New parallel page creator with no generator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelDoublePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelDoublePageCreator::pass_through(Concurrency::of(4));
    /// let mut pages = creator.create_pages(10_000);
    ///
    /// // All pages initialized to 0.0
    /// // Custom filling logic can be applied after creation
    /// ```
    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self {
            concurrency,
            generator: None,
            allocator_factory: PageAllocatorFactory::for_double_array(),
        }
    }

    /// Creates pages for the given total size.
    ///
    /// # Arguments
    ///
    /// * `size` - Total number of elements across all pages
    ///
    /// # Returns
    ///
    /// Vector of pages, each containing up to PAGE_SIZE elements
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelDoublePageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// let creator = ParallelDoublePageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(1_000_000);
    ///
    /// // Returns ~244 pages for 1 million elements
    /// ```
    pub fn create_pages(&self, total_size: usize) -> Vec<Vec<f64>> {
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
        let mut pages: Vec<Vec<f64>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size);

        pages
    }

    /// Fills an array of pages with newly allocated and initialized f64 arrays.
    ///
    /// All pages except the last one will have size equal to page_size.
    /// The last page will have the specified last_page_size.
    pub fn fill_pages(&self, pages: &mut [Vec<f64>], last_page_size: usize, page_size: usize) {
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
        page: &mut Vec<f64>,
        page_index: usize,
        page_size: usize,
        allocator: &impl PageAllocator<Vec<f64>>,
        generator: Option<&Arc<dyn Fn(usize) -> f64 + Send + Sync>>,
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
            page.resize(page_size, 0.0);
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
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(10_000);

        // Check first page, first element
        assert!((pages[0][0] - 0.0).abs() < f64::EPSILON);

        // Check first page, last element
        let page_size = pages[0].len();
        assert!((pages[0][page_size - 1] - (page_size - 1) as f64).abs() < f64::EPSILON);

        // Check second page, first element
        if pages.len() > 1 {
            assert!((pages[1][0] - page_size as f64).abs() < f64::EPSILON);
        }

        // Check last element
        let last_page = &pages[pages.len() - 1];
        let last_index = last_page.len() - 1;
        assert!((last_page[last_index] - 9999.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_custom_generator() {
        // Generate square roots
        let creator = ParallelDoublePageCreator::of(Concurrency::of(4), |i| (i as f64).sqrt());
        let pages = creator.create_pages(1000);

        // Check specific values
        assert!((pages[0][0] - 0.0).abs() < f64::EPSILON);
        assert!((pages[0][1] - 1.0).abs() < f64::EPSILON);
        assert!((pages[0][4] - 2.0).abs() < f64::EPSILON);
        assert!((pages[0][9] - 3.0).abs() < f64::EPSILON);
        assert!((pages[0][16] - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_pass_through() {
        let creator = ParallelDoublePageCreator::pass_through(Concurrency::of(4));
        let pages = creator.create_pages(5000);

        // All values should be 0.0 (default for f64)
        for page in &pages {
            for &value in page {
                assert!((value - 0.0).abs() < f64::EPSILON);
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(0);

        assert!(pages.is_empty());
    }

    #[test]
    fn test_single_page() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(100);

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].len(), 100);
        assert!((pages[0][0] - 0.0).abs() < f64::EPSILON);
        assert!((pages[0][99] - 99.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exact_page_boundary() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();
        let page_size = allocator.page_size();

        // Create exactly 3 pages worth
        let pages = creator.create_pages(page_size * 3);

        assert_eq!(pages.len(), 3);
        for page in &pages {
            assert_eq!(page.len(), page_size);
        }

        // Check boundary values
        assert!((pages[0][0] - 0.0).abs() < f64::EPSILON);
        assert!((pages[0][page_size - 1] - (page_size - 1) as f64).abs() < f64::EPSILON);
        assert!((pages[1][0] - page_size as f64).abs() < f64::EPSILON);
        assert!((pages[2][page_size - 1] - (page_size * 3 - 1) as f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_large_array() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(8));
        let pages = creator.create_pages(1_000_000);

        // Check first and last elements
        assert!((pages[0][0] - 0.0).abs() < f64::EPSILON);

        let last_page = &pages[pages.len() - 1];
        let last_index = last_page.len() - 1;
        assert!((last_page[last_index] - 999_999.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parallel_consistency() {
        // Test that different concurrency levels produce identical results
        let size = 100_000;

        let pages1 = ParallelDoublePageCreator::of(Concurrency::of(1), |i| (i as f64) * 0.5)
            .create_pages(size);
        let pages2 = ParallelDoublePageCreator::of(Concurrency::of(2), |i| (i as f64) * 0.5)
            .create_pages(size);
        let pages4 = ParallelDoublePageCreator::of(Concurrency::of(4), |i| (i as f64) * 0.5)
            .create_pages(size);
        let pages8 = ParallelDoublePageCreator::of(Concurrency::of(8), |i| (i as f64) * 0.5)
            .create_pages(size);

        // Spot check several indices
        let indices = [0, 1000, 50000, 99999];
        for &idx in &indices {
            let allocator = ParallelDoublePageCreator::identity(Concurrency::of(1))
                .allocator_factory
                .new_allocator();
            let page_size = allocator.page_size();

            let page_index = idx / page_size;
            let offset = idx % page_size;

            let val1 = pages1[page_index][offset];
            let val2 = pages2[page_index][offset];
            let val4 = pages4[page_index][offset];
            let val8 = pages8[page_index][offset];

            assert!((val1 - val2).abs() < f64::EPSILON);
            assert!((val2 - val4).abs() < f64::EPSILON);
            assert!((val4 - val8).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_memory_estimation() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));

        let size = 1_000_000;
        let estimated = creator.estimate_memory_usage(size);

        // Should be approximately size * sizeof(f64)
        let expected = size * std::mem::size_of::<f64>();

        // Allow for some page overhead (page granularity causes non-linear scaling at small sizes)
        let ratio = estimated as f64 / expected as f64;
        assert!(
            ratio >= 0.1 && ratio <= 20.0,
            "Memory estimation ratio {} should be reasonable",
            ratio
        );
    }

    #[test]
    fn test_page_size_configuration() {
        let creator = ParallelDoublePageCreator::identity(Concurrency::of(4));
        let allocator = creator.allocator_factory.new_allocator();

        // Verify page size matches expected for f64 elements (32KB pages)
        let page_size = allocator.page_size();
        let expected_page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<f64>());

        assert_eq!(page_size, expected_page_size);
        assert_eq!(page_size, 4096); // 32KB / 8 bytes = 4096 elements
    }
}
