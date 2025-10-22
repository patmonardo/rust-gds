//! Optional value handling matching Java/TypeScript Optional patterns.
//!
//! Provides a type-safe way to represent values that may or may not be present.

use std::fmt;

/// Optional value container matching Java Optional API.
///
/// # Examples
/// ```
/// use create::core::utils::miscoptional::Optional;
///
/// let some = Optional::of(42);
/// assert!(some.is_present());
/// assert_eq!(some.get(), 42);
///
/// let none: Optional<i32> = Optional::empty();
/// assert!(!none.is_present());
/// assert_eq!(none.or_else(10), 10);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Optional<T> {
    value: Option<T>,
}

impl<T> Optional<T> {
    /// Create an Optional with a value.
    ///
    /// # Panics
    /// Panics if the value is None (use `of_nullable` for Option types).
    pub fn of(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Create an Optional from an Option (allows None).
    pub fn of_nullable(value: Option<T>) -> Self {
        Self { value }
    }

    /// Create an empty Optional.
    pub fn empty() -> Self {
        Self { value: None }
    }

    /// Check if a value is present.
    pub fn is_present(&self) -> bool {
        self.value.is_some()
    }

    /// Get the value if present.
    ///
    /// # Panics
    /// Panics if no value is present.
    pub fn get(&self) -> &T {
        self.value.as_ref().expect("No value present")
    }

    /// Get the value or a default.
    pub fn or_else(&self, other: T) -> T
    where
        T: Clone,
    {
        self.value.clone().unwrap_or(other)
    }

    /// Transform the value if present.
    pub fn map<U, F>(&self, f: F) -> Optional<U>
    where
        F: FnOnce(&T) -> U,
    {
        match &self.value {
            Some(v) => Optional::of(f(v)),
            None => Optional::empty(),
        }
    }

    /// Consume and return the inner Option.
    pub fn into_option(self) -> Option<T> {
        self.value
    }

    /// Get a reference to the inner Option.
    pub fn as_option(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        Self::of_nullable(value)
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(value: Optional<T>) -> Self {
        value.value
    }
}

impl<T: fmt::Display> fmt::Display for Optional<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "Optional[{}]", v),
            None => write!(f, "Optional.empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of() {
        let opt = Optional::of(42);
        assert!(opt.is_present());
        assert_eq!(*opt.get(), 42);
    }

    #[test]
    fn test_empty() {
        let opt: Optional<i32> = Optional::empty();
        assert!(!opt.is_present());
    }

    #[test]
    fn test_of_nullable() {
        let opt1 = Optional::of_nullable(Some(42));
        assert!(opt1.is_present());
        assert_eq!(*opt1.get(), 42);

        let opt2: Optional<i32> = Optional::of_nullable(None);
        assert!(!opt2.is_present());
    }

    #[test]
    fn test_or_else() {
        let some = Optional::of(42);
        assert_eq!(some.or_else(10), 42);

        let none: Optional<i32> = Optional::empty();
        assert_eq!(none.or_else(10), 10);
    }

    #[test]
    fn test_map() {
        let opt = Optional::of(42);
        let mapped = opt.map(|x| x * 2);
        assert_eq!(*mapped.get(), 84);

        let empty: Optional<i32> = Optional::empty();
        let mapped_empty = empty.map(|x| x * 2);
        assert!(!mapped_empty.is_present());
    }

    #[test]
    #[should_panic(expected = "No value present")]
    fn test_get_panics_when_empty() {
        let opt: Optional<i32> = Optional::empty();
        opt.get();
    }

    #[test]
    fn test_from_option() {
        let opt: Optional<i32> = Some(42).into();
        assert!(opt.is_present());
        assert_eq!(*opt.get(), 42);

        let none: Optional<i32> = None.into();
        assert!(!none.is_present());
    }

    #[test]
    fn test_into_option() {
        let opt = Optional::of(42);
        let option: Option<i32> = opt.into_option();
        assert_eq!(option, Some(42));

        let empty: Optional<i32> = Optional::empty();
        let none: Option<i32> = empty.into_option();
        assert_eq!(none, None);
    }
}
