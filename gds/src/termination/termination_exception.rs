use std::error::Error;
use std::fmt;

/// Exception thrown when a computation is terminated.
///
/// This mirrors Java's TerminatedException, providing a clear signal
/// that an algorithm was gracefully cancelled.
///
/// # Examples
///
/// ```
/// use gds::termination::TerminatedException;
///
/// fn compute() -> Result<(), TerminatedException> {
///     // Check termination flag
///     return Err(TerminatedException);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminatedException;

impl fmt::Display for TerminatedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The execution has been terminated.")
    }
}

impl Error for TerminatedException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let err = TerminatedException;
        assert_eq!(err.to_string(), "The execution has been terminated.");
    }

    #[test]
    fn test_equality() {
        let err1 = TerminatedException;
        let err2 = TerminatedException;
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_clone() {
        let err1 = TerminatedException;
        let err2 = err1;
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_as_error() {
        let err: Box<dyn Error> = Box::new(TerminatedException);
        assert!(err.to_string().contains("terminated"));
    }
}
