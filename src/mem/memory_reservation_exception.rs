//! Memory reservation exception
//!
//! Custom error type for memory reservation failures.

use std::error::Error;
use std::fmt;

/// Error indicating that a memory reservation attempt failed
///
/// Thrown when requested memory exceeds available memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryReservationExceededException {
    bytes_required: usize,
    bytes_available: usize,
    message: Option<String>,
}

impl MemoryReservationExceededException {
    /// Creates a new memory reservation exception
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::MemoryReservationExceededException;
    ///
    /// let err = MemoryReservationExceededException::new(1024 * 1024 * 100, 1024 * 1024 * 50, None);
    /// assert_eq!(err.bytes_required(), 1024 * 1024 * 100);
    /// assert_eq!(err.bytes_available(), 1024 * 1024 * 50);
    /// ```
    pub fn new(bytes_required: usize, bytes_available: usize, message: Option<String>) -> Self {
        Self {
            bytes_required,
            bytes_available,
            message,
        }
    }

    /// Returns the number of bytes that were required
    pub fn bytes_required(&self) -> usize {
        self.bytes_required
    }

    /// Returns the number of bytes that were available
    pub fn bytes_available(&self) -> usize {
        self.bytes_available
    }

    /// Returns the custom message, if any
    pub fn custom_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl fmt::Display for MemoryReservationExceededException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "{}", msg)
        } else {
            write!(
                f,
                "Memory reservation failed. Required: {} bytes, Available: {} bytes.",
                self.bytes_required, self.bytes_available
            )
        }
    }
}

impl Error for MemoryReservationExceededException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let err = MemoryReservationExceededException::new(1000, 500, None);
        
        assert_eq!(err.bytes_required(), 1000);
        assert_eq!(err.bytes_available(), 500);
        assert!(err.custom_message().is_none());
    }

    #[test]
    fn test_with_custom_message() {
        let err = MemoryReservationExceededException::new(
            1000,
            500,
            Some("Custom error message".to_string()),
        );
        
        assert_eq!(err.custom_message(), Some("Custom error message"));
    }

    #[test]
    fn test_display_default() {
        let err = MemoryReservationExceededException::new(1000, 500, None);
        let display = format!("{}", err);
        
        assert!(display.contains("1000"));
        assert!(display.contains("500"));
        assert!(display.contains("Required"));
        assert!(display.contains("Available"));
    }

    #[test]
    fn test_display_custom() {
        let err = MemoryReservationExceededException::new(
            1000,
            500,
            Some("Custom message".to_string()),
        );
        let display = format!("{}", err);
        
        assert_eq!(display, "Custom message");
    }
}
