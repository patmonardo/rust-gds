//! Page-based memory management and address calculation
//!
//! This module provides **foundational algorithms** for implementing paged data structures
//! that can handle massive datasets exceeding standard vector limitations.
//! It's designed specifically for graph data science applications that need to manage
//! billions of elements efficiently through memory paging strategies.
//!
//! ## Paging Architecture Philosophy
//!
//! 1. **Memory Fragmentation Prevention**: Smaller fixed-size pages reduce fragmentation
//! 2. **Cache-Friendly Access Patterns**: Page sizes align with CPU cache hierarchies
//! 3. **Platform Optimization**: Bit manipulation for zero-cost address translation
//!
//! ## Memory Layout
//!
//! ```text
//! Global Index: |  Page Index  |  Index in Page  |
//! Bits:         | High bits    |  Low bits       |
//! Example:      | 20 bits      |  12 bits (4KB)  |
//! ```

use crate::mem::BitUtil;

/// Utility for page-based memory management
///
/// Provides static methods for address translation and page calculation
/// in paged data structures.
pub struct PageUtil;

impl PageUtil {
    /// Maximum safe array length to prevent garbage collection pressure.
    ///
    /// Arrays larger than this threshold have a higher risk of triggering full GC cycles.
    /// This limit (256MB worth of references) prevents full GC events by avoiding
    /// large consecutive memory allocations that can fragment the heap.
    ///
    /// ## GC Optimization Strategy
    ///
    /// By keeping individual arrays smaller than this limit, memory management
    /// can be more efficient using incremental collection strategies
    /// rather than falling back to expensive full heap collections.
    ///
    /// ## Memory Calculation
    ///
    /// - 2^28 = 268,435,456 elements
    /// - At 8 bytes per reference = ~2GB of reference space
    /// - Actual object memory is allocated separately and managed incrementally
    pub const MAX_ARRAY_LENGTH: usize = 1 << 28;

    /// Standard 4KB page size for cache-aligned data structures.
    ///
    /// This page size aligns with:
    /// - **CPU L1 cache**: Most modern CPUs have 32KB L1 data cache
    /// - **Memory pages**: Operating system virtual memory page size
    /// - **Cache lines**: Optimal for sequential access patterns
    ///
    /// ## Element Capacity Examples
    ///
    /// - Numbers (8 bytes): 512 elements per page
    /// - Integers (4 bytes): 1,024 elements per page
    /// - References (8 bytes): 512 references per page
    pub const PAGE_SIZE_4KB: usize = 1 << 12;

    /// Standard 32KB page size for balanced performance and memory overhead.
    ///
    /// This page size provides:
    /// - **Lower overhead**: Fewer page references to manage
    /// - **Good locality**: Large enough for meaningful sequential access
    /// - **Cache friendly**: Fits comfortably in L2 cache (256KB-1MB typical)
    ///
    /// ## Element Capacity Examples
    ///
    /// - Numbers (8 bytes): 4,096 elements per page
    /// - Integers (4 bytes): 8,192 elements per page
    /// - References (8 bytes): 4,096 references per page
    pub const PAGE_SIZE_32KB: usize = 1 << 15;

    /// Calculates the number of elements that fit in a page of the given byte size.
    ///
    /// This method computes **element density** for pages by determining how many
    /// elements of a given size can fit within a specified page size in bytes.
    /// It's essential for optimizing memory layout and access patterns.
    ///
    /// ## Element Size Requirements
    ///
    /// The element size must be a power of 2 (1, 2, 4, 8, 16, etc.) to enable
    /// efficient bit-shift arithmetic for the calculation.
    ///
    /// ## Calculation Strategy
    ///
    /// Uses bit shifting instead of division for optimal performance:
    /// `elements_per_page = page_size_in_bytes >> log2(element_size)`
    ///
    /// # Arguments
    ///
    /// * `page_size_in_bytes` - The page size in bytes (typically 4KB or 32KB)
    /// * `size_of_element` - The size of each element in bytes (must be power of 2)
    ///
    /// # Returns
    ///
    /// Number of elements that fit in a page of the given size
    ///
    /// # Panics
    ///
    /// Panics if `size_of_element` is not a power of 2
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// // Calculate elements per page for different data types
    /// let doubles_per_page = PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 8);  // 4,096
    /// let ints_per_page = PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 4);     // 8,192
    /// let bytes_per_page = PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 1);    // 32,768
    ///
    /// assert_eq!(doubles_per_page, 4096);
    /// assert_eq!(ints_per_page, 8192);
    /// assert_eq!(bytes_per_page, 32768);
    /// ```
    pub fn page_size_for(page_size_in_bytes: usize, size_of_element: usize) -> usize {
        assert!(
            BitUtil::is_power_of_two(size_of_element),
            "Element size {} must be a power of 2",
            size_of_element
        );
        page_size_in_bytes >> BitUtil::number_of_trailing_zeros(size_of_element)
    }

    /// Calculates the number of pages needed to store the given capacity of elements.
    ///
    /// This method determines how many pages are required to store a specified
    /// number of elements. It handles both power-of-2 and non-power-of-2 page sizes
    /// efficiently.
    ///
    /// ## Optimization for Power-of-2
    ///
    /// When page size is a power of 2, uses fast bit operations:
    /// `num_pages = (capacity + page_mask) >> page_shift`
    ///
    /// ## General Case
    ///
    /// For non-power-of-2 page sizes, uses standard division with ceiling:
    /// `num_pages = ceil(capacity / page_size)`
    ///
    /// # Arguments
    ///
    /// * `capacity` - Total number of elements to store
    /// * `page_size` - Number of elements per page
    ///
    /// # Returns
    ///
    /// Number of pages needed to store the capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// // 10,000 elements with 4,096 elements per page = 3 pages
    /// let pages = PageUtil::num_pages_for(10_000, 4_096);
    /// assert_eq!(pages, 3);
    ///
    /// // Exactly 8,192 elements with 4,096 per page = 2 pages
    /// let exact_pages = PageUtil::num_pages_for(8_192, 4_096);
    /// assert_eq!(exact_pages, 2);
    /// ```
    pub fn num_pages_for(capacity: usize, page_size: usize) -> usize {
        if capacity == 0 {
            return 0;
        }

        // Check if pageSize is power of 2
        if BitUtil::is_power_of_two(page_size) {
            // Use fast bit operations for power-of-2 page sizes
            let page_shift = BitUtil::number_of_trailing_zeros(page_size);
            let page_mask = page_size - 1;
            Self::num_pages_for_shift(capacity, page_shift, page_mask)
        } else {
            // Use standard division for non-power-of-2 page sizes
            capacity.div_ceil(page_size)
        }
    }

    /// Calculates the number of pages using precomputed bit manipulation values.
    ///
    /// This is an optimized version for when page shift and mask are already computed.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Total number of elements to store
    /// * `page_shift` - log2(page_size) for bit shift operations
    /// * `page_mask` - (page_size - 1) for bit masking operations
    ///
    /// # Returns
    ///
    /// Number of pages needed
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// let page_shift = 12; // 4KB = 2^12
    /// let page_mask = (1 << page_shift) - 1;
    /// let pages = PageUtil::num_pages_for_shift(10_000, page_shift, page_mask);
    /// assert!(pages > 0);
    /// ```
    pub fn num_pages_for_shift(capacity: usize, page_shift: u32, page_mask: usize) -> usize {
        // Note: In the TS version, this checks against Number.MAX_SAFE_INTEGER.
        // In Rust, overflow is caught by debug assertions during the bit shift.
        // The calculation itself cannot exceed usize::MAX by construction.
        (capacity + page_mask) >> page_shift
    }

    /// Calculates the total capacity for a given number of pages.
    ///
    /// This method computes the **maximum number of elements** that can be stored
    /// in a specified number of pages. It's the inverse operation of `num_pages_for`.
    ///
    /// ## Use Cases
    ///
    /// - **Memory estimation**: Calculate total memory requirements
    /// - **Capacity planning**: Determine maximum elements for allocated pages
    /// - **Index validation**: Ensure indices don't exceed page boundaries
    ///
    /// # Arguments
    ///
    /// * `num_pages` - Number of pages allocated
    /// * `page_shift` - log2(page_size) for bit shift operations
    ///
    /// # Returns
    ///
    /// Total capacity (maximum number of elements)
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// let page_shift = 15; // 32KB pages
    /// let capacity = PageUtil::capacity_for(10, page_shift);
    /// assert_eq!(capacity, 10 * (1 << page_shift)); // 10 * 32768
    /// ```
    pub fn capacity_for(num_pages: usize, page_shift: u32) -> usize {
        num_pages << page_shift
    }

    /// Extracts the page index from a global element index.
    ///
    /// This method performs **address translation** to determine which page
    /// contains the element at the specified global index. It uses bit shifting
    /// for optimal performance.
    ///
    /// ## Address Decomposition
    ///
    /// The global index is split into two parts:
    /// - **High bits**: Page index (which page?)
    /// - **Low bits**: Index within page (where in the page?)
    ///
    /// ## Bit Operation
    ///
    /// `page_index = global_index >> page_shift`
    ///
    /// # Arguments
    ///
    /// * `index` - Global element index
    /// * `page_shift` - log2(page_size) for bit shift operations
    ///
    /// # Returns
    ///
    /// Index of the page containing the element
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// // With 32KB pages (2^15 elements)
    /// let page_shift = 15;
    /// let global_index = 100_000;
    /// let page_index = PageUtil::page_index(global_index, page_shift);
    /// assert_eq!(page_index, 3); // 100_000 / 32_768 = 3
    /// ```
    pub fn page_index(index: usize, page_shift: u32) -> usize {
        index >> page_shift
    }

    /// Extracts the index within a page from a global element index.
    ///
    /// This method performs **address translation** to determine the position
    /// of an element within its containing page. It uses bit masking for
    /// optimal performance.
    ///
    /// ## Bit Operation
    ///
    /// `index_in_page = global_index & page_mask`
    ///
    /// # Arguments
    ///
    /// * `index` - Global element index
    /// * `page_mask` - (page_size - 1) for bit masking operations
    ///
    /// # Returns
    ///
    /// Index of the element within its page
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// // With 32KB pages (2^15 elements)
    /// let page_mask = (1 << 15) - 1;
    /// let global_index = 100_000;
    /// let index_in_page = PageUtil::index_in_page(global_index, page_mask);
    /// assert_eq!(index_in_page, 100_000 % 32_768); // 1696
    /// ```
    pub fn index_in_page(index: usize, page_mask: usize) -> usize {
        index & page_mask
    }

    /// Calculates the exclusive (one-past-the-end) index within the page.
    ///
    /// This method determines the **boundary position** for partial pages, which is
    /// essential for handling the last page in a paged array that may not be completely
    /// filled.
    ///
    /// ## Boundary Handling
    ///
    /// Most pages are completely filled, but the last page typically contains fewer
    /// elements than the page capacity. This method calculates the correct boundary
    /// for safe iteration and memory allocation.
    ///
    /// ## Calculation Strategy
    ///
    /// - For power-of-2 page sizes: Uses bit masking for optimal performance
    /// - For non-power-of-2 page sizes: Uses modulo arithmetic for correctness
    ///
    /// # Arguments
    ///
    /// * `index` - Global element index (typically the last element index)
    /// * `page_mask` - (page_size - 1) for bit masking operations
    ///
    /// # Returns
    ///
    /// Exclusive index (length) of elements in the page containing the index
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::PageUtil;
    ///
    /// // Last element at index 10,000 with 4,096 element pages
    /// let page_mask = 4095; // 4096 - 1
    /// let exclusive = PageUtil::exclusive_index_of_page(10_000, page_mask);
    /// assert_eq!(exclusive, (10_000 % 4096) + 1); // Position after last element
    /// ```
    pub fn exclusive_index_of_page(index: usize, page_mask: usize) -> usize {
        let page_size = page_mask + 1;

        if BitUtil::is_power_of_two(page_size) {
            // For power-of-2: use bit masking
            (index & page_mask) + 1
        } else {
            // For non-power-of-2: use modulo
            (index % page_size) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size_for() {
        // 32KB page with 8-byte elements = 4096 elements
        assert_eq!(PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 8), 4096);

        // 4KB page with 4-byte elements = 1024 elements
        assert_eq!(PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, 4), 1024);

        // 32KB page with 1-byte elements = 32768 elements
        assert_eq!(PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 1), 32768);
    }

    #[test]
    #[should_panic(expected = "must be a power of 2")]
    fn test_page_size_for_invalid() {
        PageUtil::page_size_for(PageUtil::PAGE_SIZE_32KB, 7); // Not a power of 2
    }

    #[test]
    fn test_num_pages_for() {
        // 10,000 elements with 4,096 per page = 3 pages
        assert_eq!(PageUtil::num_pages_for(10_000, 4_096), 3);

        // Exactly 8,192 elements with 4,096 per page = 2 pages
        assert_eq!(PageUtil::num_pages_for(8_192, 4_096), 2);

        // Zero capacity = 0 pages
        assert_eq!(PageUtil::num_pages_for(0, 4_096), 0);
    }

    #[test]
    fn test_page_index_and_index_in_page() {
        let page_shift = 15; // 32KB pages (2^15 elements)
        let page_mask = (1 << page_shift) - 1;

        let global_index = 100_000;

        // Calculate page index
        let page_idx = PageUtil::page_index(global_index, page_shift);
        assert_eq!(page_idx, 3); // 100_000 / 32_768 = 3

        // Calculate index within page
        let idx_in_page = PageUtil::index_in_page(global_index, page_mask);
        assert_eq!(idx_in_page, 1696); // 100_000 % 32_768 = 1696

        // Verify reconstruction
        let reconstructed = (page_idx << page_shift) + idx_in_page;
        assert_eq!(reconstructed, global_index);
    }

    #[test]
    fn test_capacity_for() {
        let page_shift = 15; // 32KB pages
        let capacity = PageUtil::capacity_for(10, page_shift);
        assert_eq!(capacity, 10 * 32768); // 327,680
    }

    #[test]
    fn test_exclusive_index_of_page() {
        let page_mask = 4095; // 4096 - 1

        // Last element at index 10,000
        let exclusive = PageUtil::exclusive_index_of_page(10_000, page_mask);
        assert_eq!(exclusive, (10_000 % 4096) + 1);
    }
}
