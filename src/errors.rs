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

// Note: Do not implement a blanket From<ApiError> to Box<dyn Error> because
// the standard library already provides a conflicting implementation for all
// StdError types. Leave conversions explicit where needed.
