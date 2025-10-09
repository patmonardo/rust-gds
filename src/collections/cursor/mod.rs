/// Cursor-based iteration for huge arrays.
///
/// This module provides efficient, zero-copy iteration over huge arrays that may be
/// split across multiple pages. Cursors handle page transitions transparently while
/// maintaining optimal memory access patterns.
///
/// **Key Concepts:**
/// - **Zero-copy access**: Direct access to underlying pages
/// - **Page-aware traversal**: Automatic page boundary handling
/// - **Range-based processing**: Support for parallel chunk processing
pub mod huge_cursor;
pub mod huge_cursor_support;

pub use huge_cursor::{HugeCursor, PagedCursor, SinglePageCursor};
pub use huge_cursor_support::{HugeCursorSupport, init_cursor, init_cursor_range};
