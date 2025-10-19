//! Primitive long iterators for high-performance graph traversal
//!
//! In graph analytics, we're constantly iterating over node IDs and edge IDs (represented
//! as i64 values). This module provides specialized iterator types and utilities optimized
//! for this use case.
//!
//! ## Design Philosophy
//!
//! A graph is fundamentally just a **pair of iterators**:
//! - An iterator over vertex IDs (i64)
//! - An iterator over edge pairs (i64, i64)
//!
//! This module provides the primitive building blocks for graph traversal without
//! the overhead of boxed iterators or trait objects.

/// A specialized iterator for primitive i64 values
///
/// This trait extends Rust's standard Iterator with graph-specific conveniences.
/// All implementations automatically get standard iterator methods (map, filter, etc.)
/// plus specialized methods for graph analytics.
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::PrimitiveLongIterator;
///
/// // Range iterator for node IDs
/// let node_ids: Vec<i64> = (0..5).collect();
/// let sum: i64 = node_ids.iter().copied().sum();
/// assert_eq!(sum, 10);
/// ```
pub trait PrimitiveLongIterator: Iterator<Item = i64> + Sized {
    /// Collects all remaining values into a Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::primitive::{PrimitiveLongIterator, range};
    ///
    /// let values = range(0, 4).to_vec();
    /// assert_eq!(values, vec![0, 1, 2, 3, 4]);
    /// ```
    fn to_vec(self) -> Vec<i64> {
        self.collect()
    }

    /// Counts the number of elements in the iterator
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::primitive::{PrimitiveLongIterator, range};
    ///
    /// let count = range(0, 9).count_elements();
    /// assert_eq!(count, 10);
    /// ```
    fn count_elements(self) -> usize {
        self.count()
    }
}

// Blanket implementation: any Iterator<Item = i64> is a PrimitiveLongIterator
impl<I: Iterator<Item = i64>> PrimitiveLongIterator for I {}

/// Creates an iterator that produces values in the range [start, end], inclusive.
///
/// This is a convenience function for graph algorithms that need to iterate over
/// consecutive node IDs.
///
/// # Arguments
///
/// * `start` - The starting value (inclusive)
/// * `end` - The ending value (inclusive)
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::range;
///
/// let values: Vec<i64> = range(0, 4).collect();
/// assert_eq!(values, vec![0, 1, 2, 3, 4]);
/// ```
pub fn range(start: i64, end: i64) -> impl PrimitiveLongIterator {
    start..=end
}

/// Creates an empty iterator
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::empty;
///
/// let mut iter = empty();
/// assert_eq!(iter.next(), None);
/// ```
pub fn empty() -> impl PrimitiveLongIterator {
    std::iter::empty::<i64>()
}

/// Creates an iterator with a single value
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::single;
///
/// let values: Vec<i64> = single(42).collect();
/// assert_eq!(values, vec![42]);
/// ```
pub fn single(value: i64) -> impl PrimitiveLongIterator {
    std::iter::once(value)
}

/// Creates an iterator from an array of values
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::of;
///
/// let values: Vec<i64> = of(&[1, 2, 3, 4, 5]).collect();
/// assert_eq!(values, vec![1, 2, 3, 4, 5]);
/// ```
pub fn of(values: &[i64]) -> impl PrimitiveLongIterator + '_ {
    values.iter().copied()
}

/// Type alias for a predicate function on i64 values
///
/// Used for filtering iterators based on conditions
pub type LongPredicate = fn(i64) -> bool;

/// An iterable that produces primitive long iterators
///
/// This trait is useful for types that can be iterated multiple times,
/// producing a fresh iterator each time.
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::{PrimitiveLongIterable, PrimitiveLongIterator};
///
/// struct NodeIdRange {
///     start: i64,
///     end: i64,
/// }
///
/// impl PrimitiveLongIterable for NodeIdRange {
///     type Iter = std::ops::RangeInclusive<i64>;
///     
///     fn iterator(&self) -> Self::Iter {
///         self.start..=self.end
///     }
/// }
///
/// let range = NodeIdRange { start: 0, end: 4 };
/// let sum1: i64 = range.iterator().sum();
/// let sum2: i64 = range.iterator().sum();
/// assert_eq!(sum1, sum2);
/// ```
pub trait PrimitiveLongIterable {
    /// The iterator type produced by this iterable
    type Iter: PrimitiveLongIterator;

    /// Returns a fresh iterator over the elements
    fn iterator(&self) -> Self::Iter;
}

/// Base iterator implementation for custom primitive long iterators
///
/// This struct provides a foundation for implementing stateful iterators
/// that lazily compute their next values. It follows the pattern from the
/// Java GDS implementation but adapted to Rust idioms.
///
/// # Examples
///
/// ```
/// use gds::collections::primitive::PrimitiveLongBaseIterator;
///
/// struct FibonacciIterator {
///     base: PrimitiveLongBaseIterator,
///     prev: i64,
///     current: i64,
///     limit: i64,
/// }
///
/// impl FibonacciIterator {
///     fn new(limit: i64) -> Self {
///         Self {
///             base: PrimitiveLongBaseIterator::new(),
///             prev: 0,
///             current: 1,
///             limit,
///         }
///     }
/// }
///
/// impl Iterator for FibonacciIterator {
///     type Item = i64;
///     
///     fn next(&mut self) -> Option<i64> {
///         self.base.next_with(|| {
///             if self.current > self.limit {
///                 return None;
///             }
///             let value = self.current;
///             let next = self.prev + self.current;
///             self.prev = self.current;
///             self.current = next;
///             Some(value)
///         })
///     }
/// }
/// ```
pub struct PrimitiveLongBaseIterator {
    next_value: Option<i64>,
    has_next_decided: bool,
}

impl PrimitiveLongBaseIterator {
    /// Creates a new base iterator
    pub fn new() -> Self {
        Self {
            next_value: None,
            has_next_decided: false,
        }
    }

    /// Advances the iterator using the provided fetch function
    ///
    /// This is the core method for implementing custom iterators. The fetch
    /// function should return `Some(value)` if there's a next element, or
    /// `None` if the iteration is complete.
    pub fn next_with<F>(&mut self, fetch: F) -> Option<i64>
    where
        F: FnOnce() -> Option<i64>,
    {
        if !self.has_next_decided {
            self.next_value = fetch();
            self.has_next_decided = true;
        }

        if let Some(value) = self.next_value {
            self.has_next_decided = false;
            Some(value)
        } else {
            None
        }
    }
}

impl Default for PrimitiveLongBaseIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let values: Vec<i64> = range(0, 4).collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_single_value() {
        let values: Vec<i64> = range(5, 5).collect();
        assert_eq!(values, vec![5]);
    }

    #[test]
    fn test_empty() {
        let values: Vec<i64> = empty().collect();
        assert_eq!(values, Vec::<i64>::new());
    }

    #[test]
    fn test_single() {
        let values: Vec<i64> = single(42).collect();
        assert_eq!(values, vec![42]);
    }

    #[test]
    fn test_of() {
        let values: Vec<i64> = of(&[1, 2, 3, 4, 5]).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_to_vec() {
        let values = range(0, 4).to_vec();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_count_elements() {
        let count = range(0, 9).count_elements();
        assert_eq!(count, 10);
    }

    #[test]
    fn test_filter() {
        let even: Vec<i64> = range(0, 9).filter(|&x| x % 2 == 0).collect();
        assert_eq!(even, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_map() {
        let doubled: Vec<i64> = range(0, 4).map(|x| x * 2).collect();
        assert_eq!(doubled, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_sum() {
        let sum: i64 = range(1, 10).sum();
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_iterable() {
        struct SimpleRange {
            start: i64,
            end: i64,
        }

        impl PrimitiveLongIterable for SimpleRange {
            type Iter = std::ops::RangeInclusive<i64>;

            fn iterator(&self) -> Self::Iter {
                self.start..=self.end
            }
        }

        let range = SimpleRange { start: 0, end: 4 };
        let sum1: i64 = range.iterator().sum();
        let sum2: i64 = range.iterator().sum();
        assert_eq!(sum1, 10);
        assert_eq!(sum2, 10);
    }

    #[test]
    fn test_base_iterator() {
        struct CountUpTo {
            base: PrimitiveLongBaseIterator,
            current: i64,
            limit: i64,
        }

        impl CountUpTo {
            fn new(limit: i64) -> Self {
                Self {
                    base: PrimitiveLongBaseIterator::new(),
                    current: 0,
                    limit,
                }
            }
        }

        impl Iterator for CountUpTo {
            type Item = i64;

            fn next(&mut self) -> Option<i64> {
                self.base.next_with(|| {
                    if self.current <= self.limit {
                        let value = self.current;
                        self.current += 1;
                        Some(value)
                    } else {
                        None
                    }
                })
            }
        }

        let values: Vec<i64> = CountUpTo::new(4).collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }
}
