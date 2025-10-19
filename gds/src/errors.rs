use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
}

/// Memory estimation not implemented for this algorithm
///
/// **Translation**: `MemoryEstimationNotImplementedException.java`
///
/// This error is raised when an algorithm does not provide memory estimation
/// capabilities. Memory estimation is used for capacity planning and resource
/// allocation before running expensive graph algorithms.
///
/// ## Design Note
///
/// In Java, this extends `IllegalArgumentException`. In Rust, we model it as
/// a dedicated error type that can be converted to `ApiError::InvalidInput`
/// when needed, but preserves semantic meaning for better error handling.
///
/// ## Usage
///
/// ```rust,ignore
/// fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError> {
///     Err(MemoryEstimationError::NotImplemented)
/// }
/// ```
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}

// Note: Do not implement a blanket From<ApiError> to Box<dyn Error> because
// the standard library already provides a conflicting implementation for all
// StdError types. Leave conversions explicit where needed.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_estimation_error_message() {
        let err = MemoryEstimationError::NotImplemented;
        assert_eq!(
            err.to_string(),
            "Memory estimation is not implemented for this algorithm."
        );
    }

    #[test]
    fn test_memory_estimation_error_is_cloneable() {
        let err1 = MemoryEstimationError::NotImplemented;
        let err2 = err1; // Copy
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_api_error_messages() {
        let not_found = ApiError::NotFound("graph".to_string());
        assert_eq!(not_found.to_string(), "Not Found: graph");

        let invalid = ApiError::InvalidInput("bad value".to_string());
        assert_eq!(invalid.to_string(), "Invalid Input: bad value");

        let internal = ApiError::InternalError("oops".to_string());
        assert_eq!(internal.to_string(), "Internal Error: oops");
    }
}
