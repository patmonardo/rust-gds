//! Reducers - Standard message reducer implementations
//!
//! Provides concrete implementations of the MessageReducer trait for common
//! message aggregation patterns. Reducers combine multiple messages sent to
//! the same target node into a single value, significantly reducing memory
//! consumption and improving performance.

use super::MessageReducer;
use std::fmt;

/// Sum reducer - adds all message values together
///
/// The identity element is 0.0, and messages are combined using addition.
///
/// # Example
///
/// ```ignore
/// let reducer = SumReducer;
/// assert_eq!(reducer.reduce(5.0, 3.0), 8.0);
/// assert_eq!(reducer.identity(), 0.0);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct SumReducer;

impl MessageReducer<f64> for SumReducer {
    fn reduce(&self, current: f64, message: f64) -> f64 {
        current + message
    }

    fn identity(&self) -> f64 {
        0.0
    }
}

/// Min reducer - keeps only the minimum value
///
/// The identity element is f64::MAX, and messages are combined by taking
/// the minimum value.
///
/// # Example
///
/// ```ignore
/// let reducer = MinReducer;
/// assert_eq!(reducer.reduce(5.0, 3.0), 3.0);
/// assert_eq!(reducer.identity(), f64::MAX);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct MinReducer;

impl MessageReducer<f64> for MinReducer {
    fn reduce(&self, current: f64, message: f64) -> f64 {
        current.min(message)
    }

    fn identity(&self) -> f64 {
        f64::MAX
    }
}

/// Max reducer - keeps only the maximum value
///
/// The identity element is f64::MIN (the most negative finite value),
/// and messages are combined by taking the maximum value.
///
/// # Example
///
/// ```ignore
/// let reducer = MaxReducer;
/// assert_eq!(reducer.reduce(5.0, 3.0), 5.0);
/// assert_eq!(reducer.identity(), f64::MIN);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct MaxReducer;

impl MessageReducer<f64> for MaxReducer {
    fn reduce(&self, current: f64, message: f64) -> f64 {
        current.max(message)
    }

    fn identity(&self) -> f64 {
        f64::MIN
    }
}

/// Count reducer - counts the number of messages
///
/// The identity element is 0.0, and each message increments the count by 1.
/// The actual message value is ignored.
///
/// # Example
///
/// ```ignore
/// let reducer = CountReducer;
/// assert_eq!(reducer.reduce(3.0, 42.0), 4.0);  // Count goes from 3 to 4
/// assert_eq!(reducer.identity(), 0.0);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct CountReducer;

impl MessageReducer<f64> for CountReducer {
    fn reduce(&self, current: f64, _message: f64) -> f64 {
        current + 1.0
    }

    fn identity(&self) -> f64 {
        0.0
    }
}

/// Enum representing all standard reducer types
///
/// Used for parsing reducer specifications from strings and for
/// type-safe reducer selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Reducer {
    /// Sum all messages
    #[default]
    Sum,
    /// Take minimum message value
    Min,
    /// Take maximum message value
    Max,
    /// Count number of messages
    Count,
}

impl Reducer {
    /// Parse a reducer from a string specification
    ///
    /// # Arguments
    ///
    /// * `s` - String specification (case-insensitive): "sum", "min", "max", or "count"
    ///
    /// # Returns
    ///
    /// The corresponding Reducer enum value
    ///
    /// # Errors
    ///
    /// Returns an error if the string doesn't match a known reducer type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let reducer = Reducer::parse("sum")?;
    /// assert_eq!(reducer, Reducer::Sum);
    ///
    /// let reducer = Reducer::parse("MAX")?;
    /// assert_eq!(reducer, Reducer::Max);
    /// ```
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "SUM" => Ok(Reducer::Sum),
            "MIN" => Ok(Reducer::Min),
            "MAX" => Ok(Reducer::Max),
            "COUNT" => Ok(Reducer::Count),
            _ => Err(format!("Unknown reducer: `{}`", s)),
        }
    }

    /// Convert a reducer to its canonical string representation
    ///
    /// # Returns
    ///
    /// The uppercase string name of the reducer
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(Reducer::Sum.to_string(), "SUM");
    /// assert_eq!(Reducer::Min.to_string(), "MIN");
    /// ```
    pub fn to_string_repr(&self) -> &'static str {
        match self {
            Reducer::Sum => "SUM",
            Reducer::Min => "MIN",
            Reducer::Max => "MAX",
            Reducer::Count => "COUNT",
        }
    }

    /// Create a boxed MessageReducer trait object for this reducer type
    ///
    /// # Returns
    ///
    /// A boxed trait object implementing MessageReducer<f64>
    ///
    /// # Example
    ///
    /// ```ignore
    /// let reducer = Reducer::Sum.as_trait_object();
    /// let result = reducer.reduce(5.0, 3.0);
    /// assert_eq!(result, 8.0);
    /// ```
    pub fn as_trait_object(self) -> Box<dyn MessageReducer<f64>> {
        match self {
            Reducer::Sum => Box::new(SumReducer),
            Reducer::Min => Box::new(MinReducer),
            Reducer::Max => Box::new(MaxReducer),
            Reducer::Count => Box::new(CountReducer),
        }
    }
}

impl fmt::Display for Reducer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_reducer() {
        let reducer = SumReducer;
        assert_eq!(reducer.identity(), 0.0);
        assert_eq!(reducer.reduce(0.0, 5.0), 5.0);
        assert_eq!(reducer.reduce(5.0, 3.0), 8.0);
        assert_eq!(reducer.reduce(reducer.identity(), 42.0), 42.0);
    }

    #[test]
    fn test_min_reducer() {
        let reducer = MinReducer;
        assert_eq!(reducer.identity(), f64::MAX);
        assert_eq!(reducer.reduce(5.0, 3.0), 3.0);
        assert_eq!(reducer.reduce(3.0, 5.0), 3.0);
        assert_eq!(reducer.reduce(reducer.identity(), 42.0), 42.0);
    }

    #[test]
    fn test_max_reducer() {
        let reducer = MaxReducer;
        assert_eq!(reducer.identity(), f64::MIN);
        assert_eq!(reducer.reduce(5.0, 3.0), 5.0);
        assert_eq!(reducer.reduce(3.0, 5.0), 5.0);
        assert_eq!(reducer.reduce(reducer.identity(), 42.0), 42.0);
    }

    #[test]
    fn test_count_reducer() {
        let reducer = CountReducer;
        assert_eq!(reducer.identity(), 0.0);
        assert_eq!(reducer.reduce(0.0, 5.0), 1.0);
        assert_eq!(reducer.reduce(3.0, 42.0), 4.0);
        assert_eq!(reducer.reduce(reducer.identity(), 999.0), 1.0);
    }

    #[test]
    fn test_reducer_parse() {
        assert_eq!(Reducer::parse("sum").unwrap(), Reducer::Sum);
        assert_eq!(Reducer::parse("SUM").unwrap(), Reducer::Sum);
        assert_eq!(Reducer::parse("Sum").unwrap(), Reducer::Sum);

        assert_eq!(Reducer::parse("min").unwrap(), Reducer::Min);
        assert_eq!(Reducer::parse("MIN").unwrap(), Reducer::Min);

        assert_eq!(Reducer::parse("max").unwrap(), Reducer::Max);
        assert_eq!(Reducer::parse("MAX").unwrap(), Reducer::Max);

        assert_eq!(Reducer::parse("count").unwrap(), Reducer::Count);
        assert_eq!(Reducer::parse("COUNT").unwrap(), Reducer::Count);
    }

    #[test]
    fn test_reducer_parse_invalid() {
        assert!(Reducer::parse("unknown").is_err());
        assert!(Reducer::parse("average").is_err());
        assert!(Reducer::parse("").is_err());
    }

    #[test]
    fn test_reducer_to_string() {
        assert_eq!(Reducer::Sum.to_string_repr(), "SUM");
        assert_eq!(Reducer::Min.to_string_repr(), "MIN");
        assert_eq!(Reducer::Max.to_string_repr(), "MAX");
        assert_eq!(Reducer::Count.to_string_repr(), "COUNT");
    }

    #[test]
    fn test_reducer_display() {
        assert_eq!(format!("{}", Reducer::Sum), "SUM");
        assert_eq!(format!("{}", Reducer::Min), "MIN");
        assert_eq!(format!("{}", Reducer::Max), "MAX");
        assert_eq!(format!("{}", Reducer::Count), "COUNT");
    }

    #[test]
    fn test_reducer_as_trait_object() {
        let sum_reducer = Reducer::Sum.as_trait_object();
        assert_eq!(sum_reducer.identity(), 0.0);
        assert_eq!(sum_reducer.reduce(5.0, 3.0), 8.0);

        let min_reducer = Reducer::Min.as_trait_object();
        assert_eq!(min_reducer.reduce(5.0, 3.0), 3.0);

        let max_reducer = Reducer::Max.as_trait_object();
        assert_eq!(max_reducer.reduce(5.0, 3.0), 5.0);

        let count_reducer = Reducer::Count.as_trait_object();
        assert_eq!(count_reducer.reduce(3.0, 42.0), 4.0);
    }

    #[test]
    fn test_reducer_default() {
        assert_eq!(Reducer::default(), Reducer::Sum);
    }

    #[test]
    fn test_sum_reducer_default() {
        let reducer: SumReducer = Default::default();
        assert_eq!(reducer.identity(), 0.0);
    }

    #[test]
    fn test_min_reducer_default() {
        let reducer: MinReducer = Default::default();
        assert_eq!(reducer.identity(), f64::MAX);
    }

    #[test]
    fn test_max_reducer_default() {
        let reducer: MaxReducer = Default::default();
        assert_eq!(reducer.identity(), f64::MIN);
    }

    #[test]
    fn test_count_reducer_default() {
        let reducer: CountReducer = Default::default();
        assert_eq!(reducer.identity(), 0.0);
    }
}
