use std::fmt::Debug;

/// Relationship property values are modelled as 64-bit signed integers (Java GDS alignment).
/// 
/// This replaces the SUSPECT f64 with i64 to match Java GDS Long type.
/// All property values, weights, and counts are now consistently i64.
pub type PropertyValue = i64;

/// Cursor iterating over the values of relationship properties.
///
/// This trait mirrors the TypeScript `PropertyCursor` API that powers the
/// relationship property facilities in the JavaScript implementation. The
/// contract is intentionally low-level so that specialised backends (for
/// example Arrow IPC readers or memory-mapped buffers) can expose highly
/// optimised iteration strategies without forcing allocations.
pub trait PropertyCursor: Debug {
    /// Reinitialise this cursor to point at a new logical index within the
    /// backing storage.
    fn init(&mut self, index: usize, degree: usize);

    /// Returns `true` if there is at least one more property value available.
    fn has_next_long(&self) -> bool;

    /// Fetches the next property value from the underlying storage.
    ///
    /// Implementations should return `None` when no further values are
    /// available. The TypeScript equivalent throws in this case; returning an
    /// `Option` keeps the API safe on the Rust side while extension helpers can
    /// provide the "panic on exhaustion" behaviour when needed.
    fn next_long(&mut self) -> Option<PropertyValue>;

    /// Release any resources associated with this cursor.
    fn close(&mut self);

    /// Convenience helper mirroring the TypeScript semantics: return the next
    /// property value when one exists, otherwise fall back to the supplied
    /// value.
    fn next_long_or(&mut self, fallback: PropertyValue) -> PropertyValue {
        self.next_long().unwrap_or(fallback)
    }

    /// Drain all remaining property values into a vector. This is primarily
    /// useful for tests and for slow-path algorithms that favour ergonomics
    /// over zero-allocation iteration.
    fn collect_remaining(&mut self) -> Vec<PropertyValue> {
        let mut out = Vec::new();
        while self.has_next_long() {
            if let Some(value) = self.next_long() {
                out.push(value);
            } else {
                break;
            }
        }
        out
    }
}

/// Empty implementation of a property cursor. Used when a relationship has no
/// associated property values.
#[derive(Debug, Default, Clone, Copy)]
pub struct EmptyPropertyCursor;

impl PropertyCursor for EmptyPropertyCursor {
    fn init(&mut self, _index: usize, _degree: usize) {}

    fn has_next_long(&self) -> bool {
        false
    }

    fn next_long(&mut self) -> Option<PropertyValue> {
        None
    }

    fn close(&mut self) {}
}

impl EmptyPropertyCursor {
    /// Returns a shared reference to the singleton empty cursor instance.
    pub fn shared() -> &'static Self {
        static INSTANCE: EmptyPropertyCursor = EmptyPropertyCursor;
        &INSTANCE
    }

    /// Returns a boxed trait object for the empty cursor. Useful in contexts
    /// where boxed cursors are exchanged.
    pub fn boxed() -> Box<dyn PropertyCursor> {
        Box::new(EmptyPropertyCursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cursor_reports_no_values() {
        let mut cursor = EmptyPropertyCursor;
        cursor.init(0, 0);
        assert!(!cursor.has_next_long());
        assert_eq!(cursor.next_long(), None);
        cursor.close();
    }

    #[test]
    fn collect_remaining_on_empty_cursor_is_empty() {
        let mut cursor = EmptyPropertyCursor;
        assert!(cursor.collect_remaining().is_empty());
    }
}
