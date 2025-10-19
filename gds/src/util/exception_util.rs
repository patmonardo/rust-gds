//! Exception handling utilities.
//!
//! Provides root cause analysis, exception chaining, and resource cleanup helpers.
//! Mirrors Java's ExceptionUtil from core-utils.
//!
//! **Design Philosophy**: Rust uses "Error" and "Future" for its native concepts.
//! This leaves "Exception" available for our Java GDS compatibility framework.

use std::error::Error as StdError;
use std::fmt;

use crate::util::{CheckedConsumer, CheckedFunction, CheckedRunnable, CheckedSupplier};

/// Exception handling utilities matching Java patterns.
///
/// Direct translation of `org.neo4j.gds.utils.ExceptionUtil`.
pub struct ExceptionUtil;

impl ExceptionUtil {
    /// Returns the root cause of an exception chain.
    ///
    /// Navigates through the cause chain to find the original error.
    ///
    /// Copied from `org.neo4j.helpers.Exceptions.rootCause(Throwable)`.
    ///
    /// # Examples
    /// ```
    /// use gds::util::exception_util::ExceptionUtil;
    /// use std::io;
    ///
    /// let inner = io::Error::new(io::ErrorKind::NotFound, "file not found");
    /// let outer = io::Error::new(io::ErrorKind::Other, inner);
    /// let root = ExceptionUtil::root_cause(&outer);
    /// assert_eq!(root.to_string(), "file not found");
    /// ```
    pub fn root_cause(error: &dyn StdError) -> &dyn StdError {
        let mut current = error;
        while let Some(source) = current.source() {
            current = source;
        }
        current
    }

    /// Adds the current exception to the initial exception as suppressed.
    ///
    /// Copied from `org.neo4j.helpers.Exceptions.chain(Throwable, Throwable)`.
    ///
    /// Returns `current` if `initial` is None, otherwise returns `initial`
    /// with `current` conceptually added as suppressed (Rust doesn't have
    /// direct suppressed exception support, so caller should log both).
    pub fn chain<E: StdError + 'static>(
        initial: Option<Box<E>>,
        current: Option<Box<E>>,
    ) -> Option<Box<E>> {
        match (initial, current) {
            (None, current) => current,
            (initial, None) => initial,
            (Some(initial), Some(_current)) => {
                // TODO: Store suppressed exceptions in a wrapper type
                // For now, just return the initial error
                Some(initial)
            }
        }
    }

    /// Rethrows exception if it's unchecked.
    ///
    /// In Rust, all errors are "checked" in the sense that they must be
    /// handled via Result. This method provides Java compatibility by
    /// panicking (which is Rust's equivalent of throwing an unchecked exception).
    ///
    /// Typical usage:
    /// ```ignore
    /// catch (Throwable e) {
    ///     // common code
    ///     throwIfUnchecked(e);
    ///     throw new RuntimeException(e);
    /// }
    /// ```
    pub fn throw_if_unchecked<E: StdError>(error: E) -> ! {
        panic!("{}", error)
    }

    /// Close all AutoCloseable resources, collecting exceptions.
    ///
    /// Matches Java's `ExceptionUtil.closeAll(Iterator<AutoCloseable>)`.
    /// Attempts to close all items, chaining any exceptions that occur.
    pub fn close_all<I, E>(closeables: I) -> Result<(), E>
    where
        I: IntoIterator<Item = Box<dyn AutoCloseable>>,
        E: StdError + From<Box<dyn StdError>> + 'static,
    {
        let mut error: Option<Box<E>> = None;

        for closeable in closeables {
            if let Err(e) = closeable.close() {
                let current = Box::new(E::from(e));
                error = Self::chain(error, Some(current));
            }
        }

        match error {
            Some(e) => Err(*e),
            None => Ok(()),
        }
    }

    /// Close all resources with custom error handler.
    ///
    /// Matches Java's `ExceptionUtil.closeAll(CheckedConsumer, Iterator)`.
    pub fn close_all_with_handler<I, E, H>(handler: &H, closeables: I) -> Result<(), E>
    where
        I: IntoIterator<Item = Box<dyn AutoCloseable>>,
        H: CheckedConsumer<Box<dyn StdError>, E>,
        E: StdError + 'static,
    {
        let mut error: Option<Box<dyn StdError>> = None;

        for closeable in closeables {
            if let Err(e) = closeable.close() {
                error = match error {
                    None => Some(e),
                    Some(initial) => Some(initial), // Chain conceptually
                };
            }
        }

        if let Some(e) = error {
            handler.checked_accept(e)?;
        }

        Ok(())
    }

    /// Convert CheckedRunnable to plain closure (matches Java pattern).
    pub fn unchecked<E: StdError + 'static>(
        runnable: impl CheckedRunnable<E> + 'static,
    ) -> impl Fn() {
        move || runnable.run()
    }

    /// Run a CheckedRunnable directly.
    pub fn run<E: StdError + 'static>(runnable: impl CheckedRunnable<E>) {
        runnable.run()
    }

    /// Convert CheckedConsumer to plain closure.
    pub fn consumer<T, E: StdError + 'static>(
        consumer: impl CheckedConsumer<T, E> + 'static,
    ) -> impl Fn(T) {
        move |t| consumer.accept(t)
    }

    /// Convert CheckedFunction to plain closure.
    pub fn function<T, R, E: StdError + 'static>(
        function: impl CheckedFunction<T, R, E> + 'static,
    ) -> impl Fn(T) -> R {
        move |t| function.apply(t)
    }

    /// Convert CheckedSupplier to plain closure.
    pub fn supplier<T, E: StdError + 'static>(
        supplier: impl CheckedSupplier<T, E> + 'static,
    ) -> impl Fn() -> T {
        move || supplier.get()
    }

    /// Supply a value from a CheckedSupplier.
    pub fn supply<T, E: StdError + 'static>(supplier: impl CheckedSupplier<T, E>) -> T {
        supplier.get()
    }

    /// Apply a CheckedFunction to an input.
    pub fn apply<T, R, E: StdError + 'static>(
        function: impl CheckedFunction<T, R, E>,
        input: T,
    ) -> R {
        function.apply(input)
    }

    /// Safely run code with exception logging.
    ///
    /// Matches Java's `safeRunWithLogException`.
    pub fn safe_run_with_log_exception<F, M, L>(message: M, runnable: F, exception_consumer: L)
    where
        F: FnOnce(),
        M: FnOnce() -> String,
        L: FnOnce(String, Box<dyn StdError>),
    {
        // Use catch_unwind to catch panics (Rust's equivalent of exceptions)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(runnable));

        if let Err(panic_info) = result {
            let error_msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown panic".to_string()
            };

            let error = Box::new(PanicError::new(error_msg));
            exception_consumer(message(), error);
        }
    }
}

/// Trait for AutoCloseable resources (matches Java's AutoCloseable).
///
/// This is our Exception system's resource management interface.
pub trait AutoCloseable {
    /// Close the resource, potentially returning an error.
    fn close(&self) -> Result<(), Box<dyn StdError>>;
}

/// Error wrapper for panics (used in safe_run_with_log_exception).
#[derive(Debug)]
struct PanicError {
    message: String,
}

impl PanicError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for PanicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Panic: {}", self.message)
    }
}

impl StdError for PanicError {}

/// Error type for chained exceptions.
#[derive(Debug)]
pub struct ChainedError {
    message: String,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}

impl ChainedError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cause: None,
        }
    }

    pub fn with_cause(message: impl Into<String>, cause: Box<dyn StdError + Send + Sync>) -> Self {
        Self {
            message: message.into(),
            cause: Some(cause),
        }
    }
}

impl fmt::Display for ChainedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for ChainedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_root_cause() {
        let inner = io::Error::new(io::ErrorKind::NotFound, "inner error");
        let middle = io::Error::new(io::ErrorKind::Other, inner);
        let outer = io::Error::new(io::ErrorKind::Other, middle);

        let root = ExceptionUtil::root_cause(&outer);
        assert_eq!(root.to_string(), "inner error");
    }

    #[test]
    fn test_root_cause_single() {
        let error = io::Error::new(io::ErrorKind::NotFound, "single error");
        let root = ExceptionUtil::root_cause(&error);
        assert_eq!(root.to_string(), "single error");
    }

    #[test]
    fn test_chain_none_none() {
        let result: Option<Box<io::Error>> = ExceptionUtil::chain(None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_chain_some_none() {
        let error = Box::new(io::Error::new(io::ErrorKind::NotFound, "error"));
        let result = ExceptionUtil::chain(Some(error), None);
        assert!(result.is_some());
    }

    #[test]
    fn test_chain_none_some() {
        let error = Box::new(io::Error::new(io::ErrorKind::NotFound, "error"));
        let result = ExceptionUtil::chain(None, Some(error));
        assert!(result.is_some());
    }

    #[test]
    fn test_chained_error() {
        let inner = Box::new(io::Error::new(io::ErrorKind::NotFound, "inner"));
        let error = ChainedError::with_cause("outer message", inner);

        assert_eq!(error.to_string(), "outer message");
        assert!(error.source().is_some());
    }

    #[test]
    fn test_unchecked_conversion() {
        use crate::util::checked_runnable;
        let runnable = checked_runnable(|| Ok::<(), io::Error>(()));
        let plain = ExceptionUtil::unchecked(runnable);
        plain(); // Should not panic
    }

    #[test]
    fn test_supply() {
        use crate::util::checked_supplier;
        let supplier = checked_supplier(|| Ok::<i32, io::Error>(42));
        let value = ExceptionUtil::supply(supplier);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_apply() {
        use crate::util::checked_function;
        let function = checked_function(|x: i32| Ok::<i32, io::Error>(x * 2));
        let result = ExceptionUtil::apply(function, 21);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_safe_run_with_log_exception_success() {
        let mut logged = false;
        ExceptionUtil::safe_run_with_log_exception(
            || "test message".to_string(),
            || { /* no panic */ },
            |_msg, _err| {
                logged = true;
            },
        );
        assert!(!logged);
    }

    #[test]
    fn test_safe_run_with_log_exception_panic() {
        let mut logged = false;
        let mut captured_msg = String::new();

        ExceptionUtil::safe_run_with_log_exception(
            || "test operation".to_string(),
            || panic!("test panic"),
            |msg, _err| {
                logged = true;
                captured_msg = msg;
            },
        );

        assert!(logged);
        assert_eq!(captured_msg, "test operation");
    }
}
