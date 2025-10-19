use std::fmt;
use std::num::NonZeroUsize;

/// Represents the size of a batch for parallel processing.
///
/// This is a simple value type that wraps a batch size, mirroring the Java GDS
/// `BatchSize` record. It provides type safety for batch size parameters.
///
/// In Java GDS:
/// ```java
/// public record BatchSize(int value) {}
/// ```
///
/// # Examples
///
/// ```
/// use gds::concurrency::BatchSize;
///
/// let batch = BatchSize::new(100);
/// assert_eq!(batch.value(), 100);
///
/// // Can't create a zero-sized batch
/// assert!(BatchSize::try_new(0).is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatchSize {
    value: NonZeroUsize,
}

impl BatchSize {
    /// Creates a new `BatchSize` with the specified value.
    ///
    /// Returns `None` if value is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::try_new(100);
    /// assert!(batch.is_some());
    ///
    /// let invalid = BatchSize::try_new(0);
    /// assert!(invalid.is_none());
    /// ```
    pub fn try_new(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(|v| Self { value: v })
    }

    /// Creates a new `BatchSize` with the specified value.
    ///
    /// # Panics
    ///
    /// Panics if value is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::new(100);
    /// assert_eq!(batch.value(), 100);
    /// ```
    pub fn new(value: usize) -> Self {
        Self::try_new(value).expect("BatchSize must be at least 1")
    }

    /// Returns the batch size value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::new(100);
    /// assert_eq!(batch.value(), 100);
    /// ```
    #[inline]
    pub fn value(&self) -> usize {
        self.value.get()
    }
}

impl Default for BatchSize {
    /// Returns a default batch size of 10,000.
    ///
    /// This is a common default in graph algorithms.
    fn default() -> Self {
        Self::new(10_000)
    }
}

impl fmt::Display for BatchSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BatchSize({})", self.value())
    }
}

impl From<NonZeroUsize> for BatchSize {
    fn from(value: NonZeroUsize) -> Self {
        Self { value }
    }
}

impl TryFrom<usize> for BatchSize {
    type Error = BatchSizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_new(value).ok_or(BatchSizeError::InvalidValue(value))
    }
}

/// Error type for BatchSize creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchSizeError {
    /// Value was 0
    InvalidValue(usize),
}

impl fmt::Display for BatchSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatchSizeError::InvalidValue(v) => {
                write!(f, "BatchSize must be at least 1, got {}", v)
            }
        }
    }
}

impl std::error::Error for BatchSizeError {}

// ============================================================================
// Rayon Integration Helpers
// ============================================================================

impl BatchSize {
    /// Calculates an appropriate batch size for parallel iteration.
    ///
    /// This uses Rayon's heuristics to determine a good batch size based on
    /// the total work and available parallelism.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::{BatchSize, Concurrency};
    ///
    /// let total_work = 1_000_000;
    /// let concurrency = Concurrency::available_cores();
    /// let batch = BatchSize::for_parallel_work(total_work, concurrency);
    /// println!("Using batch size: {}", batch.value());
    /// ```
    pub fn for_parallel_work(
        total_work: usize,
        concurrency: crate::concurrency::Concurrency,
    ) -> Self {
        if total_work == 0 {
            return Self::new(1);
        }

        // Heuristic: divide work by concurrency, but clamp to reasonable range
        let batch_size = (total_work / concurrency.value()).clamp(1, 100_000);
        Self::new(batch_size)
    }

    /// Creates a batch size for optimal cache locality.
    ///
    /// Returns a batch size that fits well in CPU cache (typically 4KB-16KB).
    /// Assuming 64-bit values, this is 512-2048 elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::cache_friendly();
    /// // Typically returns a few hundred to few thousand
    /// assert!(batch.value() >= 100);
    /// assert!(batch.value() <= 10_000);
    /// ```
    pub fn cache_friendly() -> Self {
        // 8KB / 8 bytes = 1024 elements
        Self::new(1024)
    }

    /// Creates a small batch size for fine-grained parallelism.
    ///
    /// Useful when work per item is expensive and you want good load balancing.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::small();
    /// assert_eq!(batch.value(), 64);
    /// ```
    pub const fn small() -> Self {
        // SAFETY: 64 is never zero
        Self {
            value: unsafe { NonZeroUsize::new_unchecked(64) },
        }
    }

    /// Creates a medium batch size for balanced parallelism.
    ///
    /// Good default for most algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::medium();
    /// assert_eq!(batch.value(), 1024);
    /// ```
    pub const fn medium() -> Self {
        // SAFETY: 1024 is never zero
        Self {
            value: unsafe { NonZeroUsize::new_unchecked(1024) },
        }
    }

    /// Creates a large batch size for coarse-grained parallelism.
    ///
    /// Useful when work per item is cheap and you want to minimize overhead.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::BatchSize;
    ///
    /// let batch = BatchSize::large();
    /// assert_eq!(batch.value(), 10_000);
    /// ```
    pub const fn large() -> Self {
        // SAFETY: 10_000 is never zero
        Self {
            value: unsafe { NonZeroUsize::new_unchecked(10_000) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let batch = BatchSize::new(100);
        assert_eq!(batch.value(), 100);
    }

    #[test]
    #[should_panic(expected = "BatchSize must be at least 1")]
    fn test_new_zero_panics() {
        let _ = BatchSize::new(0);
    }

    #[test]
    fn test_try_new() {
        let batch = BatchSize::try_new(100);
        assert!(batch.is_some());
        assert_eq!(batch.unwrap().value(), 100);

        let invalid = BatchSize::try_new(0);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_value() {
        let batch = BatchSize::new(42);
        assert_eq!(batch.value(), 42);
    }

    #[test]
    fn test_default() {
        let batch = BatchSize::default();
        assert_eq!(batch.value(), 10_000);
    }

    #[test]
    fn test_display() {
        let batch = BatchSize::new(100);
        assert_eq!(batch.to_string(), "BatchSize(100)");
    }

    #[test]
    fn test_equality() {
        let b1 = BatchSize::new(100);
        let b2 = BatchSize::new(100);
        let b3 = BatchSize::new(200);

        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }

    #[test]
    fn test_copy() {
        let b1 = BatchSize::new(100);
        let b2 = b1; // Copy
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_from_nonzero() {
        let nz = NonZeroUsize::new(100).unwrap();
        let batch = BatchSize::from(nz);
        assert_eq!(batch.value(), 100);
    }

    #[test]
    fn test_try_from() {
        let batch: Result<BatchSize, _> = 100.try_into();
        assert!(batch.is_ok());
        assert_eq!(batch.unwrap().value(), 100);

        let invalid: Result<BatchSize, _> = 0.try_into();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_error_display() {
        let err = BatchSizeError::InvalidValue(0);
        let msg = err.to_string();
        assert!(msg.contains("must be at least 1"));
        assert!(msg.contains("0"));
    }

    #[test]
    fn test_for_parallel_work() {
        use crate::concurrency::Concurrency;

        let concurrency = Concurrency::of(4);
        let batch = BatchSize::for_parallel_work(1_000_000, concurrency);

        // Should divide work across cores
        assert!(batch.value() > 0);
        assert!(batch.value() <= 1_000_000);
    }

    #[test]
    fn test_for_parallel_work_zero() {
        use crate::concurrency::Concurrency;

        let concurrency = Concurrency::of(4);
        let batch = BatchSize::for_parallel_work(0, concurrency);
        assert_eq!(batch.value(), 1);
    }

    #[test]
    fn test_cache_friendly() {
        let batch = BatchSize::cache_friendly();
        assert_eq!(batch.value(), 1024);
    }

    #[test]
    fn test_small() {
        let batch = BatchSize::small();
        assert_eq!(batch.value(), 64);
    }

    #[test]
    fn test_medium() {
        let batch = BatchSize::medium();
        assert_eq!(batch.value(), 1024);
    }

    #[test]
    fn test_large() {
        let batch = BatchSize::large();
        assert_eq!(batch.value(), 10_000);
    }
}
