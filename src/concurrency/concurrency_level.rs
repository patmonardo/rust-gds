use std::fmt;
use std::num::NonZeroUsize;

/// Represents a concurrency level (number of threads/workers) for parallel processing.
///
/// This type mirrors the Java GDS `Concurrency` class and TypeScript `Concurrency` class,
/// providing a type-safe wrapper around a positive integer representing the degree of parallelism.
///
/// # Guarantees
///
/// - Value is always at least 1 (enforced by `NonZeroUsize`)
/// - Copy-able and lightweight (single usize)
/// - Equality and hashing work as expected
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::Concurrency;
///
/// // Create with specific value
/// let c = Concurrency::new(4).unwrap();
/// assert_eq!(c.value(), 4);
///
/// // Use available CPU cores
/// let c = Concurrency::available_cores();
/// println!("Using {} threads", c.value());
///
/// // Single-threaded execution
/// let c = Concurrency::single_threaded();
/// assert_eq!(c.value(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Concurrency {
    value: NonZeroUsize,
}

impl Concurrency {
    /// Creates a new Concurrency with the specified value.
    ///
    /// Returns `None` if value is 0, following Rust idioms for fallible construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let valid = Concurrency::new(4);
    /// assert!(valid.is_some());
    ///
    /// let invalid = Concurrency::new(0);
    /// assert!(invalid.is_none());
    /// ```
    pub fn new(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(|v| Self { value: v })
    }

    /// Creates a new Concurrency with the specified value.
    ///
    /// # Panics
    ///
    /// Panics if value is less than 1, matching the Java GDS behavior:
    /// "Valid values for Concurrency are int[1..]"
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::of(4);
    /// assert_eq!(c.value(), 4);
    /// ```
    ///
    /// ```should_panic
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// // This will panic
    /// let c = Concurrency::of(0);
    /// ```
    pub fn of(value: usize) -> Self {
        Self::new(value).unwrap_or_else(|| {
            panic!(
                "Valid values for Concurrency are int[1..]. Value provided was '{}'.",
                value
            )
        })
    }

    /// Returns the concurrency level.
    ///
    /// This method mirrors `value()` in Java and `value()` in TypeScript.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::of(8);
    /// assert_eq!(c.value(), 8);
    /// ```
    #[inline]
    pub fn value(&self) -> usize {
        self.value.get()
    }

    /// Returns the square of the concurrency level.
    ///
    /// This method mirrors `squared()` in both Java and TypeScript.
    /// Used in algorithms that need quadratic work distribution.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::of(4);
    /// assert_eq!(c.squared(), 16);
    /// ```
    #[inline]
    pub fn squared(&self) -> usize {
        let v = self.value();
        v * v
    }

    /// Creates a concurrency level based on available CPU cores.
    ///
    /// This method mirrors `availableCores()` in TypeScript.
    /// Uses the `num_cpus` crate to detect logical processor count.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::available_cores();
    /// println!("Detected {} CPU cores", c.value());
    /// ```
    pub fn available_cores() -> Self {
        let cpus = num_cpus::get().max(1);
        Self {
            value: NonZeroUsize::new(cpus).unwrap(),
        }
    }

    /// Creates a single-threaded concurrency level.
    ///
    /// This method mirrors `singleThreaded()` in TypeScript.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::single_threaded();
    /// assert_eq!(c.value(), 1);
    /// ```
    pub const fn single_threaded() -> Self {
        // SAFETY: 1 is never zero
        Self {
            value: unsafe { NonZeroUsize::new_unchecked(1) },
        }
    }

    /// Converts from `usize`, clamping to minimum of 1.
    ///
    /// Unlike `of()`, this never panics - it clamps 0 to 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::from_usize(0);
    /// assert_eq!(c.value(), 1); // Clamped to 1
    ///
    /// let c = Concurrency::from_usize(8);
    /// assert_eq!(c.value(), 8);
    /// ```
    pub fn from_usize(value: usize) -> Self {
        Self::new(value.max(1)).unwrap()
    }
}

impl Default for Concurrency {
    /// Returns a single-threaded concurrency level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::default();
    /// assert_eq!(c.value(), 1);
    /// ```
    fn default() -> Self {
        Self::single_threaded()
    }
}

impl fmt::Display for Concurrency {
    /// Formats as `Concurrency(value)`, matching TypeScript's `toString()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c = Concurrency::of(4);
    /// assert_eq!(c.to_string(), "Concurrency(4)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Concurrency({})", self.value())
    }
}

impl From<NonZeroUsize> for Concurrency {
    fn from(value: NonZeroUsize) -> Self {
        Self { value }
    }
}

impl TryFrom<usize> for Concurrency {
    type Error = ConcurrencyError;

    /// Tries to create a Concurrency from a usize.
    ///
    /// Returns an error if value is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::Concurrency;
    ///
    /// let c: Result<Concurrency, _> = 4.try_into();
    /// assert!(c.is_ok());
    ///
    /// let c: Result<Concurrency, _> = 0.try_into();
    /// assert!(c.is_err());
    /// ```
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(ConcurrencyError::InvalidValue(value))
    }
}

/// Error type for Concurrency creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcurrencyError {
    /// Value was less than 1
    InvalidValue(usize),
}

impl fmt::Display for ConcurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConcurrencyError::InvalidValue(v) => {
                write!(
                    f,
                    "Valid values for Concurrency are int[1..]. Value provided was '{}'.",
                    v
                )
            }
        }
    }
}

impl std::error::Error for ConcurrencyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let c = Concurrency::new(4).unwrap();
        assert_eq!(c.value(), 4);
    }

    #[test]
    fn test_new_invalid() {
        let c = Concurrency::new(0);
        assert!(c.is_none());
    }

    #[test]
    fn test_of_valid() {
        let c = Concurrency::of(8);
        assert_eq!(c.value(), 8);
    }

    #[test]
    #[should_panic(expected = "Valid values for Concurrency are int[1..]")]
    fn test_of_invalid() {
        let _ = Concurrency::of(0);
    }

    #[test]
    fn test_value() {
        let c = Concurrency::of(16);
        assert_eq!(c.value(), 16);
    }

    #[test]
    fn test_squared() {
        let c = Concurrency::of(4);
        assert_eq!(c.squared(), 16);

        let c = Concurrency::of(8);
        assert_eq!(c.squared(), 64);

        let c = Concurrency::of(1);
        assert_eq!(c.squared(), 1);
    }

    #[test]
    fn test_available_cores() {
        let c = Concurrency::available_cores();
        assert!(c.value() >= 1);
        println!("Detected {} CPU cores", c.value());
    }

    #[test]
    fn test_single_threaded() {
        let c = Concurrency::single_threaded();
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn test_from_usize() {
        let c = Concurrency::from_usize(0);
        assert_eq!(c.value(), 1); // Clamped

        let c = Concurrency::from_usize(4);
        assert_eq!(c.value(), 4);
    }

    #[test]
    fn test_default() {
        let c = Concurrency::default();
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn test_display() {
        let c = Concurrency::of(4);
        assert_eq!(c.to_string(), "Concurrency(4)");
    }

    #[test]
    fn test_equality() {
        let c1 = Concurrency::of(4);
        let c2 = Concurrency::of(4);
        let c3 = Concurrency::of(8);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_copy() {
        let c1 = Concurrency::of(4);
        let c2 = c1; // Copy
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Concurrency::of(4));
        set.insert(Concurrency::of(4)); // Duplicate
        set.insert(Concurrency::of(8));

        assert_eq!(set.len(), 2); // Only 2 unique values
    }

    #[test]
    fn test_try_from() {
        let c: Result<Concurrency, _> = 4.try_into();
        assert!(c.is_ok());
        assert_eq!(c.unwrap().value(), 4);

        let c: Result<Concurrency, _> = 0.try_into();
        assert!(c.is_err());
    }

    #[test]
    fn test_error_message() {
        let err = ConcurrencyError::InvalidValue(0);
        let msg = err.to_string();
        assert!(msg.contains("Valid values for Concurrency are int[1..]"));
        assert!(msg.contains("0"));
    }

    #[test]
    fn test_from_nonzero() {
        let nz = NonZeroUsize::new(4).unwrap();
        let c = Concurrency::from(nz);
        assert_eq!(c.value(), 4);
    }
}
