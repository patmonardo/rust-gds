use std::fmt::Debug;

/// PropertyValue type for relationship properties
/// 
/// This is the default type for relationship properties, aligned with Java GDS
/// which uses Double for weights. For typed access, use TypedRelationshipCursor.
pub type PropertyValue = f64;

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
    fn has_next(&self) -> bool;

    /// Fetches the next property value from the underlying storage.
    ///
    /// Implementations should return `None` when no further values are
    /// available. The TypeScript equivalent throws in this case; returning an
    /// `Option` keeps the API safe on the Rust side while extension helpers can
    /// provide the "panic on exhaustion" behaviour when needed.
    fn next(&mut self) -> Option<f64>;

    /// Release any resources associated with this cursor.
    fn close(&mut self);

    /// Convenience helper mirroring the TypeScript semantics: return the next
    /// property value when one exists, otherwise fall back to the supplied
    /// value.
    fn next_or(&mut self, fallback: f64) -> f64 {
        self.next().unwrap_or(fallback)
    }

    /// Drain all remaining property values into a vector. This is primarily
    /// useful for tests and for slow-path algorithms that favour ergonomics
    /// over zero-allocation iteration.
    fn collect_remaining(&mut self) -> Vec<f64> {
        let mut out = Vec::new();
        while self.has_next() {
            if let Some(value) = self.next() {
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

    fn has_next(&self) -> bool {
        false
    }

    fn next(&mut self) -> Option<f64> {
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
        assert!(!cursor.has_next());
        assert_eq!(cursor.next(), None);
        cursor.close();
    }

    #[test]
    fn collect_remaining_on_empty_cursor_is_empty() {
        let mut cursor = EmptyPropertyCursor;
        assert!(cursor.collect_remaining().is_empty());
    }
}
