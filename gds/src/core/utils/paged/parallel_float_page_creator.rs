//! High-performance parallel page creator for f32 array initialization.
//!
//! Essential for efficient bulk setup of billion-scale floating-point data:
//! - Parallel initialization of huge float arrays with custom values
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

/// Parallel page creator for f32 arrays.
///
/// Creates and fills pages in parallel using configurable value generation functions.
///
/// # Generator Patterns
///
/// 1. **Identity**: `|i| i as f32` - Array[i] = i as float
/// 2. **Custom**: `|i| (i as f32).sqrt()` - Computed values
/// 3. **Pass-through**: None - Zero-initialized pages
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::ParallelFloatPageCreator;
/// use gds::concurrency::Concurrency;
///
/// // Identity mapping for 1 billion elements
/// let creator = ParallelFloatPageCreator::identity(Concurrency::of(8));
/// let pages = creator.create_pages(1_000_000_000);
///
/// // Result: pages[0][0] = 0.0, pages[0][1] = 1.0, ..., pages[n][m] = 999_999_999.0
/// ```
pub struct ParallelFloatPageCreator {
    #[allow(dead_code)]
    concurrency: Concurrency,
    generator: Option<Arc<dyn Fn(usize) -> f32 + Send + Sync>>,
    allocator_factory: PageAllocatorFactory<Vec<f32>>,
}

impl ParallelFloatPageCreator {
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
    /// use gds::core::utils::paged::ParallelFloatPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create pages with square root values
    /// let creator = ParallelFloatPageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i as f32).sqrt()
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0.0, pages[0][1] = 1.0, pages[0][2] ≈ 1.414, ...
    /// ```
    pub fn of<F>(concurrency: Concurrency, generator: F) -> Self
    where
        F: Fn(usize) -> f32 + Send + Sync + 'static,
    {
        Self {
            concurrency,
            generator: Some(Arc::new(generator)),
            allocator_factory: PageAllocatorFactory::for_float_array(),
        }
    }

    /// Creates an identity mapping page creator.
    ///
    /// Each element's value equals its index as f32: array[i] = i as f32.
    /// Perfect for continuous node/edge indexing and identity transformations.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// New identity page creator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelFloatPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create identity mapping for 1 million elements
    /// let creator = ParallelFloatPageCreator::identity(Concurrency::of(8));
    /// let pages = creator.create_pages(1_000_000);
    ///
    /// // Result: pages[0][0] = 0.0, pages[0][1] = 1.0, ..., pages[n][m] = 999_999.0
    /// ```
    pub fn identity(concurrency: Concurrency) -> Self {
        Self::of(concurrency, |i| i as f32)
    }

    /// Creates a pass-through page creator (zero-initialized).
    ///
    /// All elements are initialized to 0.0f32. Useful for sparse arrays
    /// where most elements remain zero, or as a base for subsequent updates.
    ///
    /// # Arguments
    ///
    /// * `concurrency` - Number of parallel workers
    ///
    /// # Returns
    ///
    /// New pass-through page creator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelFloatPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create zero-initialized pages
    /// let creator = ParallelFloatPageCreator::pass_through(Concurrency::of(4));
    /// let pages = creator.create_pages(1000);
    ///
    /// // Result: All elements are 0.0f32
    /// ```
    pub fn pass_through(concurrency: Concurrency) -> Self {
        Self {
            concurrency,
            generator: None,
            allocator_factory: PageAllocatorFactory::for_float_array(),
        }
    }

    /// Creates pages for the specified total size.
    ///
    /// Pages are created and filled in parallel using the configured generator.
    /// If no generator is configured (pass-through mode), pages are zero-initialized.
    ///
    /// # Arguments
    ///
    /// * `total_size` - Total number of elements across all pages
    ///
    /// # Returns
    ///
    /// Vector of filled pages ready for use in HugeFloatArray
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ParallelFloatPageCreator;
    /// use gds::concurrency::Concurrency;
    ///
    /// // Create pages with custom values
    /// let creator = ParallelFloatPageCreator::of(
    ///     Concurrency::of(4),
    ///     |i| (i as f32) * 0.5
    /// );
    ///
    /// let pages = creator.create_pages(1000);
    /// // pages[0][0] = 0.0, pages[0][1] = 0.5, pages[0][2] = 1.0, ...
    /// ```
    pub fn create_pages(&self, total_size: usize) -> Vec<Vec<f32>> {
        if let Some(ref generator) = self.generator {
            self.create_pages_with_generator(total_size, generator)
        } else {
            self.create_zero_pages(total_size)
        }
    }

    /// Creates pages using the provided generator function.
    fn create_pages_with_generator(
        &self,
        total_size: usize,
        generator: &Arc<dyn Fn(usize) -> f32 + Send + Sync>,
    ) -> Vec<Vec<f32>> {
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
        let mut pages: Vec<Vec<f32>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size, &Some(generator.clone()));

        pages
    }

    /// Creates zero-initialized pages.
    fn create_zero_pages(&self, total_size: usize) -> Vec<Vec<f32>> {
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
        let mut pages: Vec<Vec<f32>> = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(Vec::new());
        }

        self.fill_pages(&mut pages, last_page_size, page_size, &None);

        pages
    }

    /// Fills an array of pages with newly allocated and initialized f32 arrays.
    ///
    /// All pages except the last one will have size equal to page_size.
    /// The last page will have the specified last_page_size.
    fn fill_pages(
        &self,
        pages: &mut [Vec<f32>],
        last_page_size: usize,
        page_size: usize,
        generator: &Option<Arc<dyn Fn(usize) -> f32 + Send + Sync>>,
    ) {
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
                        generator,
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
            generator,
        );
    }

    /// Creates and fills a single page with the specified size.
    fn create_and_fill_page(
        &self,
        page: &mut Vec<f32>,
        page_index: usize,
        page_size: usize,
        allocator: &impl PageAllocator<Vec<f32>>,
        generator: &Option<Arc<dyn Fn(usize) -> f32 + Send + Sync>>,
    ) {
        // Allocate the page
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_creator() {
        let creator = ParallelFloatPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(1000);

        assert!(!pages.is_empty());
        assert_eq!(pages[0][0], 0.0);
        assert_eq!(pages[0][1], 1.0);
        assert_eq!(pages[0][2], 2.0);
    }

    #[test]
    fn test_custom_generator() {
        let creator = ParallelFloatPageCreator::of(Concurrency::of(4), |i| (i as f32).sqrt());
        let pages = creator.create_pages(1000);

        assert!(!pages.is_empty());
        assert_eq!(pages[0][0], 0.0);
        assert_eq!(pages[0][1], 1.0);
        assert_eq!(pages[0][4], 2.0); // sqrt(4) = 2.0
    }

    #[test]
    fn test_pass_through_creator() {
        let creator = ParallelFloatPageCreator::pass_through(Concurrency::of(4));
        let pages = creator.create_pages(1000);

        assert!(!pages.is_empty());
        // All values should be 0.0
        for page in &pages {
            for &value in page {
                assert_eq!(value, 0.0);
            }
        }
    }

    #[test]
    fn test_large_array_creation() {
        let creator = ParallelFloatPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(1_000_000);

        assert!(!pages.is_empty());
        // Check first and last elements
        assert_eq!(pages[0][0], 0.0);
        
        // Find the last page and check its last element
        let last_page_index = pages.len() - 1;
        let last_page = &pages[last_page_index];
        let last_element_index = last_page.len() - 1;
        let expected_last_value = (1_000_000 - 1) as f32;
        assert_eq!(last_page[last_element_index], expected_last_value);
    }

    #[test]
    fn test_concurrent_consistency() {
        // Test that different concurrency levels produce same results
        let size = 100_000;

        let pages1 = ParallelFloatPageCreator::of(Concurrency::of(1), |i| (i as f32) * 0.5)
            .create_pages(size);
        let pages2 = ParallelFloatPageCreator::of(Concurrency::of(2), |i| (i as f32) * 0.5)
            .create_pages(size);
        let pages4 = ParallelFloatPageCreator::of(Concurrency::of(4), |i| (i as f32) * 0.5)
            .create_pages(size);
        let pages8 = ParallelFloatPageCreator::of(Concurrency::of(8), |i| (i as f32) * 0.5)
            .create_pages(size);

        // Spot check several indices across all concurrency levels
        for idx in [0, 1000, 50000, 99999] {
            let expected = (idx as f32) * 0.5;
            
            // Find the page and index within that page
            let page_size = 32 * 1024 / std::mem::size_of::<f32>(); // 32KB pages
            let page_index = idx / page_size;
            let index_in_page = idx % page_size;
            
            assert_eq!(pages1[page_index][index_in_page], expected);
            assert_eq!(pages2[page_index][index_in_page], expected);
            assert_eq!(pages4[page_index][index_in_page], expected);
            assert_eq!(pages8[page_index][index_in_page], expected);
        }
    }

    #[test]
    fn test_million_elements() {
        let creator = ParallelFloatPageCreator::of(Concurrency::of(8), |i| {
            if i % 1_000_000 == 0 {
                i as f32
            } else {
                0.0
            }
        });

        let pages = creator.create_pages(10_000_000);
        assert!(!pages.is_empty());

        // Check specific values
        let page_size = 32 * 1024 / std::mem::size_of::<f32>(); // 32KB pages
        let page_5m = 5_000_000 / page_size;
        let index_5m = 5_000_000 % page_size;
        assert_eq!(pages[page_5m][index_5m], 5_000_000.0);
    }

    #[test]
    fn test_identity_mapping() {
        let creator = ParallelFloatPageCreator::identity(Concurrency::of(4));
        let pages = creator.create_pages(1000);

        // Verify identity mapping
        for (page_index, page) in pages.iter().enumerate() {
            let page_size = 32 * 1024 / std::mem::size_of::<f32>(); // 32KB pages
            let start_index = page_index * page_size;
            
            for (i, &value) in page.iter().enumerate() {
                let global_index = start_index + i;
                if global_index < 1000 {
                    assert_eq!(value, global_index as f32);
                }
            }
        }
    }

    #[test]
    fn test_constant_values() {
        let creator = ParallelFloatPageCreator::of(Concurrency::of(4), |_| 3.14159);
        let pages = creator.create_pages(1000);

        for page in &pages {
            for &value in page {
                assert_eq!(value, 3.14159);
            }
        }
    }

    #[test]
    fn test_zero_values() {
        let creator = ParallelFloatPageCreator::of(Concurrency::of(4), |_| 0.0);
        let pages = creator.create_pages(1000);

        for page in &pages {
            for &value in page {
                assert_eq!(value, 0.0);
            }
        }
    }

    #[test]
    fn test_alternating_pattern() {
        let creator = ParallelFloatPageCreator::of(Concurrency::of(4), |i| {
            if i % 2 == 0 {
                1.0
            } else {
                -1.0
            }
        });

        let pages = creator.create_pages(100);

        assert_eq!(pages[0][0], 1.0);
        assert_eq!(pages[0][1], -1.0);
        assert_eq!(pages[0][50], 1.0);
        assert_eq!(pages[0][51], -1.0);
    }
}
