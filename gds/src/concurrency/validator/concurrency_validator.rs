use std::fmt;

/// Concurrency validator trait.
///
/// Validates concurrency settings against limitations, primarily for
/// license enforcement. The actual validation is simple - just check
/// if requested > limit - but we package it in a trait for extensibility.
///
/// In Java GDS:
/// ```java
/// public interface ConcurrencyValidator {
///     void validate(int requested, String key, int limit) throws IllegalArgumentException;
/// }
/// ```
///
/// # Examples
///
/// ```
/// use gds::concurrency::validator::{ConcurrencyValidator, OpenGdsConcurrencyValidator};
///
/// let validator = OpenGdsConcurrencyValidator;
/// assert!(validator.validate(4, "concurrency", 4).is_ok());
/// assert!(validator.validate(8, "concurrency", 4).is_err());
/// ```
pub trait ConcurrencyValidator {
    /// Validates a requested concurrency level against a limit.
    ///
    /// # Arguments
    ///
    /// * `requested_concurrency` - The requested concurrency level
    /// * `config_key` - Configuration key for error messages
    /// * `concurrency_limitation` - Maximum allowed concurrency
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(ConcurrencyValidatorError)` if invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::validator::{ConcurrencyValidator, OpenGdsConcurrencyValidator};
    ///
    /// let validator = OpenGdsConcurrencyValidator;
    ///
    /// // Valid: requested <= limit
    /// assert!(validator.validate(4, "concurrency", 4).is_ok());
    ///
    /// // Invalid: requested > limit
    /// let result = validator.validate(8, "concurrency", 4);
    /// assert!(result.is_err());
    /// ```
    fn validate(
        &self,
        requested_concurrency: usize,
        config_key: &str,
        concurrency_limitation: usize,
    ) -> Result<(), ConcurrencyValidatorError>;
}

/// Error returned when concurrency validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcurrencyValidatorError {
    pub requested: usize,
    pub limit: usize,
    pub config_key: String,
    pub message: String,
}

impl ConcurrencyValidatorError {
    /// Creates a new validation error.
    pub fn new(requested: usize, limit: usize, config_key: impl Into<String>) -> Self {
        let config_key = config_key.into();
        let message = format!(
            "Community users cannot exceed {}={} (you configured {}={}), see https://neo4j.com/docs/graph-data-science/",
            config_key, limit, config_key, requested
        );
        Self {
            requested,
            limit,
            config_key,
            message,
        }
    }
}

impl fmt::Display for ConcurrencyValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConcurrencyValidatorError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = ConcurrencyValidatorError::new(8, 4, "concurrency");
        assert_eq!(err.requested, 8);
        assert_eq!(err.limit, 4);
        assert_eq!(err.config_key, "concurrency");
        assert!(err.message.contains("concurrency=4"));
        assert!(err.message.contains("concurrency=8"));
    }

    #[test]
    fn test_error_display() {
        let err = ConcurrencyValidatorError::new(8, 4, "concurrency");
        let msg = err.to_string();
        assert!(msg.contains("Community users"));
        assert!(msg.contains("concurrency=4"));
        assert!(msg.contains("concurrency=8"));
        assert!(msg.contains("neo4j.com/docs"));
    }

    #[test]
    fn test_error_clone() {
        let err1 = ConcurrencyValidatorError::new(8, 4, "concurrency");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
