//! Huge array utilities for managing page-based large arrays
//!
//! Provides utilities for splitting huge arrays into pages to work around
//! maximum array size limitations.

/// Utility for managing huge arrays via page-based indexing
///
/// Large arrays are split into pages of a fixed size (default 2^14 = 16,384 elements).
/// This allows working with arrays larger than typical size limits.
pub struct HugeArrays;

impl HugeArrays {
    /// Maximum array length (2^28 = 268,435,456)
    pub const MAX_ARRAY_LENGTH: usize = 1 << 28;

    /// Page shift (14 bits = 16K elements per page)
    pub const PAGE_SHIFT: u32 = 14;

    /// Page size (2^14 = 16,384 elements)
    pub const PAGE_SIZE: usize = 1 << Self::PAGE_SHIFT;

    /// Page mask for extracting in-page index
    const PAGE_MASK: usize = Self::PAGE_SIZE - 1;

    /// Calculates the page index for a given global array index
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::HugeArrays;
    ///
    /// let index = 100_000;
    /// let page = HugeArrays::page_index(index);
    /// assert_eq!(page, 6); // 100,000 / 16,384 = 6
    /// ```
    pub fn page_index(index: usize) -> usize {
        index >> Self::PAGE_SHIFT
    }

    /// Calculates the index within a page for a given global array index
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::HugeArrays;
    ///
    /// let index = 100_000;
    /// let in_page = HugeArrays::index_in_page(index);
    /// assert_eq!(in_page, 100_000 % HugeArrays::PAGE_SIZE);
    /// ```
    pub fn index_in_page(index: usize) -> usize {
        index & Self::PAGE_MASK
    }

    /// Calculates the page index with a custom page shift value
    pub fn page_index_with_shift(index: usize, page_shift: u32) -> usize {
        index >> page_shift
    }

    /// Calculates the index within a page with a custom page mask
    pub fn index_in_page_with_mask(index: usize, page_mask: usize) -> usize {
        index & page_mask
    }

    /// Calculates the exclusive index of a page
    ///
    /// Returns how many elements are used in the page for the given index
    pub fn exclusive_index_of_page(index: usize) -> usize {
        1 + ((index - 1) & Self::PAGE_MASK)
    }

    /// Reconstructs a global index from a page index and an index within that page
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::HugeArrays;
    ///
    /// let page = 6;
    /// let in_page = 1696;
    /// let reconstructed = HugeArrays::index_from_page_index_and_index_in_page(page, in_page);
    /// assert_eq!(reconstructed, 100_000);
    /// ```
    pub fn index_from_page_index_and_index_in_page(
        page_index: usize,
        index_in_page: usize,
    ) -> usize {
        (page_index << Self::PAGE_SHIFT) | index_in_page
    }

    /// Calculates the number of pages needed for the given capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::HugeArrays;
    ///
    /// let pages = HugeArrays::number_of_pages(100_000);
    /// assert_eq!(pages, 7); // ceil(100,000 / 16,384) = 7
    /// ```
    pub fn number_of_pages(capacity: usize) -> usize {
        let num_pages = (capacity + Self::PAGE_MASK) >> Self::PAGE_SHIFT;

        if num_pages > usize::MAX / 2 {
            panic!(
                "pageSize={} is too small for capacity: {}",
                Self::PAGE_SIZE,
                capacity
            );
        }

        num_pages
    }

    /// Calculates the number of pages with custom page shift and mask values
    pub fn number_of_pages_custom(capacity: usize, page_shift: u32, page_mask: usize) -> usize {
        let num_pages = (capacity + page_mask) >> page_shift;

        if num_pages > usize::MAX / 2 {
            panic!(
                "pageSize={} is too small for capacity: {}",
                1 << page_shift,
                capacity
            );
        }

        num_pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_constants() {
        assert_eq!(HugeArrays::PAGE_SIZE, 16384);
        assert_eq!(HugeArrays::PAGE_SHIFT, 14);
    }

    #[test]
    fn test_page_index() {
        assert_eq!(HugeArrays::page_index(0), 0);
        assert_eq!(HugeArrays::page_index(16383), 0);
        assert_eq!(HugeArrays::page_index(16384), 1);
        assert_eq!(HugeArrays::page_index(100_000), 6);
    }

    #[test]
    fn test_index_in_page() {
        assert_eq!(HugeArrays::index_in_page(0), 0);
        assert_eq!(HugeArrays::index_in_page(100), 100);
        assert_eq!(HugeArrays::index_in_page(16384), 0);
        assert_eq!(HugeArrays::index_in_page(16385), 1);
        assert_eq!(HugeArrays::index_in_page(100_000), 1696);
    }

    #[test]
    fn test_roundtrip() {
        let original_index = 100_000;
        let page = HugeArrays::page_index(original_index);
        let in_page = HugeArrays::index_in_page(original_index);
        let reconstructed = HugeArrays::index_from_page_index_and_index_in_page(page, in_page);

        assert_eq!(reconstructed, original_index);
    }

    #[test]
    fn test_number_of_pages() {
        assert_eq!(HugeArrays::number_of_pages(0), 0);
        assert_eq!(HugeArrays::number_of_pages(1), 1);
        assert_eq!(HugeArrays::number_of_pages(16384), 1);
        assert_eq!(HugeArrays::number_of_pages(16385), 2);
        assert_eq!(HugeArrays::number_of_pages(100_000), 7);
    }

    #[test]
    fn test_exclusive_index_of_page() {
        assert_eq!(HugeArrays::exclusive_index_of_page(1), 1);
        assert_eq!(HugeArrays::exclusive_index_of_page(16384), 16384);
        assert_eq!(HugeArrays::exclusive_index_of_page(16385), 1);
    }

    #[test]
    fn test_custom_page_operations() {
        let custom_shift = 10;
        let custom_mask = (1 << custom_shift) - 1;

        let index = 5000;
        let page = HugeArrays::page_index_with_shift(index, custom_shift);
        let in_page = HugeArrays::index_in_page_with_mask(index, custom_mask);

        assert_eq!(page * (1 << custom_shift) + in_page, index);
    }
}
