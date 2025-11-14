use thiserror::Error;

/// Canonical result type for Arrow backend operations.
pub type ArrowResult<T> = Result<T, ArrowBackendError>;

/// Top-level error for Arrow-backed collections.
#[derive(Debug, Error)]
pub enum ArrowBackendError {
    #[error("Arrow kernel failed: {0}")]
    Kernel(#[from] ArrowKernelError),
    #[error("Arrow interop error: {0}")]
    Interop(String),
    #[error("Unsupported capability: {0}")]
    Unsupported(String),
}

/// Errors produced by compute kernels or buffers.
#[derive(Debug, Error)]
pub enum ArrowKernelError {
    #[error("Arrow compute failure: {0}")]
    Compute(String),
    #[error("Bitmap mismatch: {0}")]
    Bitmap(String),
}
