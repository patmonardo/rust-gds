//! HugeArray Implementation Macros
//!
//! Generates typed columns supporting billions of elements with automatic
//! single-page vs multi-page selection based on size.
//!
//! **Pattern**: Every HugeArray is a typed column that can back PropertyValues
//! implementations. The macro generates both single-page (optimized) and
//! multi-page (scalable) variants automatically.

// Note: These imports are used in the macro implementations but not in the module itself
// They will be available when the macros are expanded

/// Generates a complete HugeArray implementation for a primitive type.
///
/// # Generated Types
///
/// For each invocation `huge_primitive_array!(HugeFooArray, foo_type, "Foo")`:
/// - `HugeFooArray` - Main enum dispatching between Single/Paged
/// - `SingleHugeFooArray` - Optimized single-page implementation
/// - `PagedHugeFooArray` - Multi-page implementation for large arrays
/// - Cursor support via `HugeCursorSupport` trait
///
/// # Example
///
/// ```ignore
/// huge_primitive_array! {
///     HugeIntArray, i32, "Int",
///     "A long-indexable i32 array supporting billions of elements."
/// }
/// ```
#[macro_export]
macro_rules! huge_primitive_array {
    (
        $huge_name:ident,           // e.g., HugeIntArray
        $single_name:ident,         // e.g., SingleHugeIntArray
        $paged_name:ident,          // e.g., PagedHugeIntArray
        $cursor_name:ident,         // e.g., HugeIntArrayCursor
        $element_type:ty,           // e.g., i32
        $type_display:expr,         // e.g., "Int"
        $doc_desc:expr              // Documentation description
    ) => {
        use $crate::collections::cursor::{
            HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor,
        };
        use $crate::collections::{ArrayUtil, PageUtil};

        /// Maximum size for single-page arrays (from PageUtil)
        const MAX_ARRAY_LENGTH: usize = 1 << 28; // ~268 million elements

        #[doc = $doc_desc]
        ///
        /// Implemented by paging smaller arrays to support approximately 32,000 billion elements.
        /// For arrays small enough, uses optimized single-page implementation for maximum performance.
        ///
        /// # Characteristics
        ///
        /// - **Fixed size**: Cannot grow or shrink after creation
        /// - **Dense storage**: Every position consumes memory (use sparse variants for sparse data)
        /// - **Zero default**: Unset values return default value (0 for numeric types)
        /// - **Thread safety**: Reads are safe, writes are not (external synchronization needed)
        /// - **Cursor support**: Efficient zero-copy iteration over pages
        pub enum $huge_name {
            /// Single-page implementation for arrays ≤ MAX_ARRAY_LENGTH
            Single($single_name),
            /// Multi-page implementation for arrays > MAX_ARRAY_LENGTH
            Paged($paged_name),
        }

        impl $huge_name {
            /// Creates a new array of the given size.
            ///
            /// Automatically chooses optimal implementation based on size.
            pub fn new(size: usize) -> Self {
                if size <= MAX_ARRAY_LENGTH {
                    Self::Single($single_name::new(size))
                } else {
                    Self::Paged($paged_name::new(size))
                }
            }

            /// Creates a new array from the provided values.
            pub fn from_vec(values: Vec<$element_type>) -> Self {
                let size = values.len();
                let mut array = Self::new(size);
                for (i, value) in values.into_iter().enumerate() {
                    array.set(i, value);
                }
                array
            }

            /// Gets the value at the specified index.
            ///
            /// Returns the default value if index is out of bounds.
            pub fn get(&self, index: usize) -> $element_type {
                match self {
                    Self::Single(arr) => arr.get(index),
                    Self::Paged(arr) => arr.get(index),
                }
            }

            /// Sets the value at the specified index.
            ///
            /// # Panics
            ///
            /// Panics if index is out of bounds.
            pub fn set(&mut self, index: usize, value: $element_type) {
                match self {
                    Self::Single(arr) => arr.set(index, value),
                    Self::Paged(arr) => arr.set(index, value),
                }
            }

            /// Sets all elements using the provided generator function.
            pub fn set_all<F>(&mut self, gen: F)
            where
                F: Fn(usize) -> $element_type,
            {
                match self {
                    Self::Single(arr) => arr.set_all(gen),
                    Self::Paged(arr) => arr.set_all(gen),
                }
            }

            /// Fills all elements with the specified value.
            pub fn fill(&mut self, value: $element_type) {
                match self {
                    Self::Single(arr) => arr.fill(value),
                    Self::Paged(arr) => arr.fill(value),
                }
            }

            /// Returns the number of elements in the array.
            pub fn size(&self) -> usize {
                match self {
                    Self::Single(arr) => arr.size(),
                    Self::Paged(arr) => arr.size(),
                }
            }

            /// Returns the memory used by this array in bytes.
            pub fn size_of(&self) -> usize {
                match self {
                    Self::Single(arr) => arr.size_of(),
                    Self::Paged(arr) => arr.size_of(),
                }
            }

            /// Copies elements from this array to the destination array.
            ///
            /// # Panics
            ///
            /// Panics if length exceeds either array's size
            pub fn copy_to(&self, dest: &mut $huge_name, length: usize) {
                assert!(length <= self.size(), "length exceeds source array size");
                assert!(length <= dest.size(), "length exceeds dest array size");

                for i in 0..length {
                    dest.set(i, self.get(i));
                }
            }

            /// Creates a copy of this array with a new length.
            ///
            /// If new length is larger, new elements are initialized to default value.
            pub fn copy_of(&self, new_length: usize) -> Self {
                let mut result = Self::new(new_length);
                let copy_length = usize::min(self.size(), new_length);
                self.copy_to(&mut result, copy_length);
                result
            }

            /// Converts to a standard Vec.
            pub fn to_vec(&self) -> Vec<$element_type> {
                let size = self.size();
                let mut result = Vec::with_capacity(size);
                for i in 0..size {
                    result.push(self.get(i));
                }
                result
            }

            /// Performs binary search for the given value (array must be sorted).
            ///
            /// Returns the index if found, or -(insertion_point + 1) if not found.
            pub fn binary_search(&self, search_value: $element_type) -> isize
            where
                $element_type: PartialOrd,
            {
                match self {
                    Self::Single(arr) => arr.binary_search(search_value),
                    Self::Paged(arr) => arr.binary_search(search_value),
                }
            }

            /// Performs binary search using a comparator function.
            pub fn binary_search_by<F>(&self, mut cmp: F) -> isize
            where
                F: FnMut($element_type) -> std::cmp::Ordering,
            {
                let mut low = 0isize;
                let mut high = (self.size() - 1) as isize;

                while low <= high {
                    let mid = ((low as u64 + high as u64) >> 1) as isize;
                    let mid_val = self.get(mid as usize);

                    match cmp(mid_val) {
                        std::cmp::Ordering::Less => low = mid + 1,
                        std::cmp::Ordering::Greater => high = mid - 1,
                        std::cmp::Ordering::Equal => return mid,
                    }
                }
                -(low + 1)
            }
        }

        // =============================================================================
        // SingleHugeArray Implementation
        // =============================================================================

        /// Single-page array implementation (≤ 268M elements).
        pub struct $single_name {
            data: Vec<$element_type>,
        }

        impl $single_name {
            pub fn new(size: usize) -> Self {
                assert!(size <= MAX_ARRAY_LENGTH, "size exceeds MAX_ARRAY_LENGTH");
                Self {
                    data: vec![<$element_type>::default(); size],
                }
            }

            pub fn get(&self, index: usize) -> $element_type {
                self.data.get(index).copied().unwrap_or_default()
            }

            pub fn set(&mut self, index: usize, value: $element_type) {
                if index < self.data.len() {
                    self.data[index] = value;
                }
            }

            pub fn set_all<F>(&mut self, gen: F)
            where
                F: Fn(usize) -> $element_type,
            {
                for i in 0..self.data.len() {
                    self.data[i] = gen(i);
                }
            }

            pub fn fill(&mut self, value: $element_type) {
                self.data.fill(value);
            }

            pub fn size(&self) -> usize {
                self.data.len()
            }

            pub fn size_of(&self) -> usize {
                self.data.len() * std::mem::size_of::<$element_type>()
            }

            pub fn binary_search(&self, search_value: $element_type) -> isize
            where
                $element_type: PartialOrd,
            {
                let mut low = 0isize;
                let mut high = (self.data.len() - 1) as isize;

                while low <= high {
                    let mid = ((low as u64 + high as u64) >> 1) as isize;
                    let mid_val = self.data[mid as usize];

                    if mid_val < search_value {
                        low = mid + 1;
                    } else if mid_val > search_value {
                        high = mid - 1;
                    } else {
                        return mid;
                    }
                }
                -(low + 1)
            }
        }

        impl<'a> HugeCursorSupport<'a> for $single_name {
            type Cursor = SinglePageCursor<'a, $element_type>;

            fn size(&self) -> usize {
                self.data.len()
            }

            fn new_cursor(&'a self) -> Self::Cursor {
                SinglePageCursor::new(&self.data)
            }
        }

        // =============================================================================
        // PagedHugeArray Implementation
        // =============================================================================

        /// Multi-page array implementation (> 268M elements).
        pub struct $paged_name {
            pages: Vec<Vec<$element_type>>,
            size: usize,
        }

        impl $paged_name {
            pub fn new(size: usize) -> Self {
                // Calculate page size for element type with 4KB pages
                let page_size = PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<$element_type>());
                let page_shift = page_size.trailing_zeros(); // log2 of page_size
                let page_mask = page_size - 1;
                let num_pages = PageUtil::num_pages_for(size, page_size);

                let mut pages = Vec::with_capacity(num_pages);
                for page_index in 0..num_pages {
                    let page_length = if page_index == num_pages - 1 {
                        PageUtil::exclusive_index_of_page(size, page_mask)
                    } else {
                        page_size
                    };
                    pages.push(vec![<$element_type>::default(); page_length]);
                }

                Self { pages, size }
            }

            pub fn get(&self, index: usize) -> $element_type {
                if index >= self.size {
                    return <$element_type>::default();
                }
                let page_size = PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<$element_type>());
                let page_shift = page_size.trailing_zeros();
                let page_mask = page_size - 1;
                let page_index = PageUtil::page_index(index, page_shift);
                let index_in_page = PageUtil::index_in_page(index, page_mask);
                self.pages[page_index][index_in_page]
            }

            pub fn set(&mut self, index: usize, value: $element_type) {
                assert!(index < self.size, "index out of bounds");
                let page_size = PageUtil::page_size_for(PageUtil::PAGE_SIZE_4KB, std::mem::size_of::<$element_type>());
                let page_shift = page_size.trailing_zeros();
                let page_mask = page_size - 1;
                let page_index = PageUtil::page_index(index, page_shift);
                let index_in_page = PageUtil::index_in_page(index, page_mask);
                self.pages[page_index][index_in_page] = value;
            }

            pub fn set_all<F>(&mut self, gen: F)
            where
                F: Fn(usize) -> $element_type,
            {
                let mut global_index = 0usize;
                for page in &mut self.pages {
                    for elem in page.iter_mut() {
                        if global_index < self.size {
                            *elem = gen(global_index);
                            global_index += 1;
                        }
                    }
                }
            }

            pub fn fill(&mut self, value: $element_type) {
                for page in &mut self.pages {
                    page.fill(value);
                }
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn size_of(&self) -> usize {
                self.pages.iter().map(|p| p.len()).sum::<usize>()
                    * std::mem::size_of::<$element_type>()
            }

            pub fn binary_search(&self, search_value: $element_type) -> isize
            where
                $element_type: PartialOrd,
            {
                let mut low = 0isize;
                let mut high = (self.size - 1) as isize;

                while low <= high {
                    let mid = ((low as u64 + high as u64) >> 1) as isize;
                    let mid_val = self.get(mid as usize);

                    if mid_val < search_value {
                        low = mid + 1;
                    } else if mid_val > search_value {
                        high = mid - 1;
                    } else {
                        return mid;
                    }
                }
                -(low + 1)
            }
        }

        impl<'a> HugeCursorSupport<'a> for $paged_name {
            type Cursor = PagedCursor<'a, $element_type>;

            fn size(&self) -> usize {
                self.size
            }

            fn new_cursor(&'a self) -> Self::Cursor {
                PagedCursor::new(&self.pages, self.size)
            }
        }

        /// Cursor for iterating over HugeArray elements
        pub enum $cursor_name<'a> {
            Single(SinglePageCursor<'a, $element_type>),
            Paged(PagedCursor<'a, $element_type>),
        }

        impl<'a> HugeCursor<'a> for $cursor_name<'a> {
            type Array = [$element_type];

            fn next(&mut self) -> bool {
                match self {
                    Self::Single(cursor) => cursor.next(),
                    Self::Paged(cursor) => cursor.next(),
                }
            }

            fn base(&self) -> usize {
                match self {
                    Self::Single(cursor) => cursor.base(),
                    Self::Paged(cursor) => cursor.base(),
                }
            }

            fn offset(&self) -> usize {
                match self {
                    Self::Single(cursor) => cursor.offset(),
                    Self::Paged(cursor) => cursor.offset(),
                }
            }

            fn limit(&self) -> usize {
                match self {
                    Self::Single(cursor) => cursor.limit(),
                    Self::Paged(cursor) => cursor.limit(),
                }
            }

            fn array(&self) -> Option<&'a Self::Array> {
                match self {
                    Self::Single(cursor) => cursor.array(),
                    Self::Paged(cursor) => cursor.array(),
                }
            }

            fn reset(&mut self) {
                match self {
                    Self::Single(cursor) => cursor.reset(),
                    Self::Paged(cursor) => cursor.reset(),
                }
            }

            fn set_range(&mut self, start: usize, end: usize) {
                match self {
                    Self::Single(cursor) => cursor.set_range(start, end),
                    Self::Paged(cursor) => cursor.set_range(start, end),
                }
            }
        }

        impl<'a> HugeCursorSupport<'a> for $huge_name {
            type Cursor = $cursor_name<'a>;

            fn size(&self) -> usize {
                match self {
                    Self::Single(arr) => arr.size(),
                    Self::Paged(arr) => arr.size(),
                }
            }

            fn new_cursor(&'a self) -> Self::Cursor {
                match self {
                    Self::Single(arr) => $cursor_name::Single(arr.new_cursor()),
                    Self::Paged(arr) => $cursor_name::Paged(arr.new_cursor()),
                }
            }
        }
    };
}

