//! Core memory page allocation system for huge data structures.
//!
//! Provides efficient memory management through page-based allocation:
//! - Consistent page sizes for predictable memory usage
//! - Type-safe page creation for different data types
//! - Memory estimation for capacity planning
//! - Factory pattern for allocator configuration
//!
//! Essential for billion-scale data structures:
//! - HugeLongArray: pages of i64 arrays
//! - HugeDoubleArray: pages of f64 arrays
//! - HugeObjectArray: pages of generic objects
//! - Matrix implementations: efficient 2D page layouts
//!
//! Performance characteristics:
//! - Page sizes optimized for cache performance (32KB default)
//! - Power-of-2 page sizes for efficient index calculations
//! - Minimal allocation overhead through consistent sizing
//! - Memory-mapped file compatibility

use crate::collections::PageUtil;
use crate::mem::{BitUtil, Estimate};
use std::marker::PhantomData;

/// Abstract trait for page allocation strategies.
///
/// Defines the contract for creating and managing memory pages for huge data structures.
///
/// # Type Parameter
///
/// * `T` - The type of page (e.g., `Vec<i64>`, `Vec<f64>`, or `Vec<T>`)
///
/// # Design Pattern
///
/// Uses the Abstract Factory pattern to allow different page creation strategies
/// while maintaining consistent interface across implementations.
pub trait PageAllocator<T> {
    /// Creates a new page of type T.
    ///
    /// Each page contains a fixed number of elements determined by the allocator configuration.
    ///
    /// # Returns
    ///
    /// New page instance with allocated capacity
    fn new_page(&self) -> T;

    /// Returns the number of elements per page.
    ///
    /// This is constant for a given allocator instance and is typically a power of 2
    /// for optimal bit-shift arithmetic.
    ///
    /// # Returns
    ///
    /// Elements per page
    fn page_size(&self) -> usize;

    /// Returns the memory usage per page in bytes.
    ///
    /// Includes array overhead and element storage for accurate memory estimation.
    ///
    /// # Returns
    ///
    /// Bytes per page
    fn bytes_per_page(&self) -> usize;

    /// Estimates total memory usage for given number of elements.
    ///
    /// Essential for capacity planning and resource allocation before creating huge arrays.
    ///
    /// # Arguments
    ///
    /// * `size` - Total number of elements needed
    ///
    /// # Returns
    ///
    /// Estimated memory usage in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::PageAllocatorFactory;
    ///
    /// let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
    /// let allocator = factory.new_allocator();
    ///
    /// let memory_needed = allocator.estimate_memory_usage(1_000_000_000);
    /// println!("1B elements needs ~{} GB", memory_needed / (1024 * 1024 * 1024));
    /// ```
    fn estimate_memory_usage(&self, size: usize) -> usize {
        let num_pages = PageUtil::num_pages_for(size, self.page_size());
        num_pages * self.bytes_per_page()
    }
}

/// Factory for creating page allocators with specific configurations.
///
/// Encapsulates all the parameters needed for consistent page allocation across
/// a huge data structure.
///
/// # Type Parameter
///
/// * `T` - The type of page (e.g., `Vec<i64>`, `Vec<f64>`)
///
/// # Design Benefits
///
/// - **Immutable configuration**: Factory parameters cannot be changed after creation
/// - **Consistent allocation**: All allocators from same factory use same page size
/// - **Type safety**: Compile-time guarantees about page types
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::PageAllocatorFactory;
///
/// // Create factory for i64 arrays with 32KB pages
/// let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
///
/// // Get page size for this configuration
/// assert_eq!(factory.page_size(), 4096); // 4096 i64s = 32KB
///
/// // Estimate memory for 1 billion elements
/// let memory = factory.estimate_memory_usage(1_000_000_000);
/// println!("Memory needed: {} MB", memory / (1024 * 1024));
/// ```
pub struct PageAllocatorFactory<T> {
    page_size: usize,
    bytes_per_page: usize,
    _phantom: PhantomData<T>,
}

impl<T> PageAllocatorFactory<T> {
    /// Creates a new factory with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of elements per page (must be power of 2)
    /// * `bytes_per_page` - Memory usage per page in bytes
    ///
    /// # Panics
    ///
    /// Panics if `page_size` is not a power of 2
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::PageAllocatorFactory;
    /// use gds::mem::Estimate;
    ///
    /// let page_size = 4096;
    /// let bytes_per_page = Estimate::size_of_long_array(page_size);
    ///
    /// let factory = PageAllocatorFactory::<Vec<i64>>::new(page_size, bytes_per_page);
    /// ```
    pub fn new(page_size: usize, bytes_per_page: usize) -> Self {
        assert!(
            BitUtil::is_power_of_two(page_size),
            "Page size must be power of 2, got {}",
            page_size
        );

        Self {
            page_size,
            bytes_per_page,
            _phantom: PhantomData,
        }
    }

    /// Gets the page size for this factory.
    ///
    /// # Returns
    ///
    /// Number of elements per page
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Gets the bytes per page for this factory.
    ///
    /// # Returns
    ///
    /// Memory usage per page in bytes
    pub fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }

    /// Estimates memory usage for a given number of elements.
    ///
    /// # Arguments
    ///
    /// * `size` - Total number of elements
    ///
    /// # Returns
    ///
    /// Estimated memory usage in bytes
    pub fn estimate_memory_usage(&self, size: usize) -> usize {
        let num_pages = PageUtil::num_pages_for(size, self.page_size);
        num_pages * self.bytes_per_page
    }

    /// Creates a new allocator instance from this factory.
    ///
    /// Each allocator maintains its own state but shares the factory's configuration.
    ///
    /// # Returns
    ///
    /// New page allocator
    pub fn new_allocator(&self) -> DirectPageAllocator<T> {
        DirectPageAllocator::new(self.page_size, self.bytes_per_page)
    }
}

impl PageAllocatorFactory<Vec<i64>> {
    /// Creates a factory optimized for i64 arrays with 32KB pages.
    ///
    /// Configuration:
    /// - Page size: 4,096 elements (32KB for 8-byte i64s)
    /// - Optimized for: Graph node IDs, timestamps, counts
    ///
    /// # Returns
    ///
    /// Factory configured for i64 pages
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::PageAllocatorFactory;
    ///
    /// let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
    /// let allocator = factory.new_allocator();
    ///
    /// // Create a page
    /// let page = allocator.new_page();
    /// assert_eq!(page.capacity(), 4096);
    /// ```
    pub fn for_long_array() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<i64>());
        let bytes_per_page = Estimate::size_of_long_array(page_size);

        Self::new(page_size, bytes_per_page)
    }

    /// Creates a factory optimized for i64 arrays with 4KB pages.
    ///
    /// Configuration:
    /// - Page size: 512 elements (4KB for 8-byte i64s)
    /// - Optimized for: Cache-friendly access, smaller datasets
    ///
    /// # Returns
    ///
    /// Factory configured for small i64 pages
    pub fn for_long_array_4kb() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<i64>());
        let bytes_per_page = Estimate::size_of_long_array(page_size);

        Self::new(page_size, bytes_per_page)
    }
}

impl PageAllocatorFactory<Vec<f64>> {
    /// Creates a factory optimized for f64 arrays with 32KB pages.
    ///
    /// Configuration:
    /// - Page size: 4,096 elements (32KB for 8-byte f64s)
    /// - Optimized for: Edge weights, embeddings, scientific data
    ///
    /// # Returns
    ///
    /// Factory configured for f64 pages
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::PageAllocatorFactory;
    ///
    /// let factory = PageAllocatorFactory::<Vec<f64>>::for_double_array();
    /// let allocator = factory.new_allocator();
    ///
    /// let page = allocator.new_page();
    /// assert_eq!(page.capacity(), 4096);
    /// ```
    pub fn for_double_array() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<f64>());
        let bytes_per_page = Estimate::size_of_double_array(page_size);

        Self::new(page_size, bytes_per_page)
    }

    /// Creates a factory optimized for f64 arrays with 4KB pages.
    pub fn for_double_array_4kb() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<f64>());
        let bytes_per_page = Estimate::size_of_double_array(page_size);

        Self::new(page_size, bytes_per_page)
    }
}

impl PageAllocatorFactory<Vec<f32>> {
    /// Creates a factory optimized for f32 arrays with 32KB pages.
    ///
    /// Configuration:
    /// - Page size: 8,192 elements (32KB for 4-byte f32s)
    /// - Optimized for: Single-precision floating-point data
    ///
    /// # Returns
    ///
    /// Factory configured for f32 pages
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::PageAllocatorFactory;
    ///
    /// let factory = PageAllocatorFactory::<Vec<f32>>::for_float_array();
    /// let allocator = factory.new_allocator();
    ///
    /// let page = allocator.new_page();
    /// assert_eq!(page.capacity(), 8192);
    /// ```
    pub fn for_float_array() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<f32>());
        let bytes_per_page = Estimate::size_of_float_array(page_size);

        Self::new(page_size, bytes_per_page)
    }

    /// Creates a factory optimized for f32 arrays with 4KB pages.
    pub fn for_float_array_4kb() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<f32>());
        let bytes_per_page = Estimate::size_of_float_array(page_size);

        Self::new(page_size, bytes_per_page)
    }
}

impl PageAllocatorFactory<Vec<i32>> {
    /// Creates a factory optimized for i32 arrays with 32KB pages.
    ///
    /// Configuration:
    /// - Page size: 8,192 elements (32KB for 4-byte i32s)
    /// - Optimized for: Compact integers, indices, colors
    ///
    /// # Returns
    ///
    /// Factory configured for i32 pages
    pub fn for_int_array() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<i32>());
        let bytes_per_page = Estimate::size_of_int_array(page_size);

        Self::new(page_size, bytes_per_page)
    }
}

impl PageAllocatorFactory<Vec<u8>> {
    /// Creates a factory optimized for byte arrays with 32KB pages.
    ///
    /// Configuration:
    /// - Page size: 32,768 elements (32KB for 1-byte u8s)
    /// - Optimized for: Binary data, compressed storage, byte buffers
    ///
    /// # Returns
    ///
    /// Factory configured for byte pages
    pub fn for_byte_array() -> Self {
        let page_size =
            PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, std::mem::size_of::<u8>());
        let bytes_per_page = Estimate::size_of_byte_array(page_size);

        Self::new(page_size, bytes_per_page)
    }
}

/// Direct implementation of PageAllocator.
///
/// Uses simple delegation to pre-configured page factory settings.
/// This is the concrete implementation returned by `PageAllocatorFactory::new_allocator()`.
///
/// # Type Parameter
///
/// * `T` - The type of page to allocate (e.g., `Vec<i64>`)
pub struct DirectPageAllocator<T> {
    page_size: usize,
    bytes_per_page: usize,
    _phantom: PhantomData<T>,
}

impl<T> DirectPageAllocator<T> {
    /// Creates a new direct allocator with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of elements per page
    /// * `bytes_per_page` - Memory usage per page
    ///
    /// # Panics
    ///
    /// Panics if page_size is not a power of 2
    fn new(page_size: usize, bytes_per_page: usize) -> Self {
        assert!(
            BitUtil::is_power_of_two(page_size),
            "Page size must be power of 2"
        );

        Self {
            page_size,
            bytes_per_page,
            _phantom: PhantomData,
        }
    }
}

impl PageAllocator<Vec<i64>> for DirectPageAllocator<Vec<i64>> {
    fn new_page(&self) -> Vec<i64> {
        // allocate capacity but keep len == 0 (empty but reserved)
        Vec::with_capacity(self.page_size)
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }
}

impl PageAllocator<Vec<f64>> for DirectPageAllocator<Vec<f64>> {
    fn new_page(&self) -> Vec<f64> {
        Vec::with_capacity(self.page_size)
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }
}

impl PageAllocator<Vec<i32>> for DirectPageAllocator<Vec<i32>> {
    fn new_page(&self) -> Vec<i32> {
        Vec::with_capacity(self.page_size)
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }
}

impl PageAllocator<Vec<u8>> for DirectPageAllocator<Vec<u8>> {
    fn new_page(&self) -> Vec<u8> {
        Vec::with_capacity(self.page_size)
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }
}

impl PageAllocator<Vec<f32>> for DirectPageAllocator<Vec<f32>> {
    fn new_page(&self) -> Vec<f32> {
        Vec::with_capacity(self.page_size)
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn bytes_per_page(&self) -> usize {
        self.bytes_per_page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_array_factory() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();

        assert_eq!(factory.page_size(), 4096); // 32KB / 8 bytes
        assert!(factory.bytes_per_page() > 0);

        let allocator = factory.new_allocator();
        let page = allocator.new_page();

        assert_eq!(page.capacity(), 4096);
        assert_eq!(page.len(), 0); // Empty but allocated
    }

    #[test]
    fn test_double_array_factory() {
        let factory = PageAllocatorFactory::<Vec<f64>>::for_double_array();

        assert_eq!(factory.page_size(), 4096); // 32KB / 8 bytes

        let allocator = factory.new_allocator();
        let page = allocator.new_page();

        assert_eq!(page.capacity(), 4096);
    }

    #[test]
    fn test_int_array_factory() {
        let factory = PageAllocatorFactory::<Vec<i32>>::for_int_array();

        assert_eq!(factory.page_size(), 8192); // 32KB / 4 bytes

        let allocator = factory.new_allocator();
        let page = allocator.new_page();

        assert_eq!(page.capacity(), 8192);
    }

    #[test]
    fn test_byte_array_factory() {
        let factory = PageAllocatorFactory::<Vec<u8>>::for_byte_array();

        assert_eq!(factory.page_size(), 32768); // 32KB / 1 byte

        let allocator = factory.new_allocator();
        let page = allocator.new_page();

        assert_eq!(page.capacity(), 32768);
    }

    #[test]
    fn test_memory_estimation() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();

        // 1 million elements
        let memory_1m = factory.estimate_memory_usage(1_000_000);
        assert!(memory_1m > 0);

        // 1 billion elements
        let memory_1b = factory.estimate_memory_usage(1_000_000_000);
        assert!(memory_1b > memory_1m);

        // Should be roughly proportional
        let ratio = memory_1b as f64 / memory_1m as f64;
        assert!((ratio - 1000.0).abs() < 100.0); // Within 10% of 1000x
    }

    #[test]
    fn test_allocator_memory_estimation() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array();
        let allocator = factory.new_allocator();

        let memory = allocator.estimate_memory_usage(1_000_000);
        assert_eq!(memory, factory.estimate_memory_usage(1_000_000));
    }

    #[test]
    fn test_4kb_pages() {
        let factory = PageAllocatorFactory::<Vec<i64>>::for_long_array_4kb();

        assert_eq!(factory.page_size(), 512); // 4KB / 8 bytes

        let allocator = factory.new_allocator();
        let page = allocator.new_page();

        assert_eq!(page.capacity(), 512);
    }

    #[test]
    #[should_panic(expected = "Page size must be power of 2")]
    fn test_invalid_page_size() {
        PageAllocatorFactory::<Vec<i64>>::new(4095, 32768); // Not power of 2
    }
}
