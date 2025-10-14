//! Checked functional interfaces - Error-safe operations
//!
//! Direct translation of Java's Checked* interfaces from core-utils.
//! These traits extend standard functional patterns with error handling.
//!
//! **Pattern**: Each trait provides:
//! - A `checked_*` method that returns Result<T, E>
//! - A default `unchecked` method that panics on error (matches Java's throw behavior)
//! - Static factory method for convenience
//!
//! **Usage**:
//! ```ignore
//! let supplier = ||| -> Result<i32, MyError> { Ok(42) };
//! let value = supplier.get(); // Panics on error, like Java
//! let result = supplier.checked_get(); // Returns Result
//! ```

use std::error::Error as StdError;

/// A Supplier that can return errors during value production.
///
/// Matches Java's `CheckedSupplier<T, E extends Exception>`.
pub trait CheckedSupplier<T, E: StdError> {
    /// Produce a value, potentially returning an error.
    fn checked_get(&self) -> Result<T, E>;

    /// Get the value, panicking on error (matches Java's throw behavior).
    fn get(&self) -> T
    where
        T: Sized,
        E: 'static,
    {
        match self.checked_get() {
            Ok(value) => value,
            Err(e) => {
                crate::util::ExceptionUtil::throw_if_unchecked(e);
            }
        }
    }
}

/// Factory method for creating CheckedSupplier from a closure.
pub fn checked_supplier<T, E, F>(f: F) -> impl CheckedSupplier<T, E>
where
    F: Fn() -> Result<T, E>,
    E: StdError,
{
    struct FnSupplier<F>(F);
    impl<T, E, F> CheckedSupplier<T, E> for FnSupplier<F>
    where
        F: Fn() -> Result<T, E>,
        E: StdError,
    {
        fn checked_get(&self) -> Result<T, E> {
            (self.0)()
        }
    }
    FnSupplier(f)
}

/// A Function that can return errors during transformation.
///
/// Matches Java's `CheckedFunction<T, R, E extends Exception>`.
pub trait CheckedFunction<T, R, E: StdError> {
    /// Transform the input value, potentially returning an error.
    fn checked_apply(&self, input: T) -> Result<R, E>;

    /// Apply the function, panicking on error (matches Java's throw behavior).
    fn apply(&self, input: T) -> R
    where
        R: Sized,
        E: 'static,
    {
        match self.checked_apply(input) {
            Ok(result) => result,
            Err(e) => {
                crate::util::ExceptionUtil::throw_if_unchecked(e);
            }
        }
    }
}

/// Factory method for creating CheckedFunction from a closure.
pub fn checked_function<T, R, E, F>(f: F) -> impl CheckedFunction<T, R, E>
where
    F: Fn(T) -> Result<R, E>,
    E: StdError,
{
    struct FnFunction<F>(F);
    impl<T, R, E, F> CheckedFunction<T, R, E> for FnFunction<F>
    where
        F: Fn(T) -> Result<R, E>,
        E: StdError,
    {
        fn checked_apply(&self, input: T) -> Result<R, E> {
            (self.0)(input)
        }
    }
    FnFunction(f)
}

/// A Consumer that can return errors during processing.
///
/// Matches Java's `CheckedConsumer<T, E extends Exception>`.
pub trait CheckedConsumer<T, E: StdError> {
    /// Process the input value, potentially returning an error.
    fn checked_accept(&self, value: T) -> Result<(), E>;

    /// Accept the value, panicking on error (matches Java's throw behavior).
    fn accept(&self, value: T)
    where
        E: 'static,
    {
        match self.checked_accept(value) {
            Ok(()) => (),
            Err(e) => {
                crate::util::ExceptionUtil::throw_if_unchecked(e);
            }
        }
    }
}

/// Factory method for creating CheckedConsumer from a closure.
pub fn checked_consumer<T, E, F>(f: F) -> impl CheckedConsumer<T, E>
where
    F: Fn(T) -> Result<(), E>,
    E: StdError,
{
    struct FnConsumer<F>(F);
    impl<T, E, F> CheckedConsumer<T, E> for FnConsumer<F>
    where
        F: Fn(T) -> Result<(), E>,
        E: StdError,
    {
        fn checked_accept(&self, value: T) -> Result<(), E> {
            (self.0)(value)
        }
    }
    FnConsumer(f)
}

/// A Runnable that can return errors during execution.
///
/// Matches Java's `CheckedRunnable<E extends Exception>`.
pub trait CheckedRunnable<E: StdError> {
    /// Execute the operation, potentially returning an error.
    fn checked_run(&self) -> Result<(), E>;

    /// Run the operation, panicking on error (matches Java's throw behavior).
    fn run(&self)
    where
        E: 'static,
    {
        match self.checked_run() {
            Ok(()) => (),
            Err(e) => {
                crate::util::ExceptionUtil::throw_if_unchecked(e);
            }
        }
    }
}

/// Factory method for creating CheckedRunnable from a closure.
pub fn checked_runnable<E, F>(f: F) -> impl CheckedRunnable<E>
where
    F: Fn() -> Result<(), E>,
    E: StdError,
{
    struct FnRunnable<F>(F);
    impl<E, F> CheckedRunnable<E> for FnRunnable<F>
    where
        F: Fn() -> Result<(), E>,
        E: StdError,
    {
        fn checked_run(&self) -> Result<(), E> {
            (self.0)()
        }
    }
    FnRunnable(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_checked_supplier_ok() {
        let supplier = checked_supplier(|| Ok::<i32, IoError>(42));
        assert_eq!(supplier.checked_get().unwrap(), 42);
        assert_eq!(supplier.get(), 42);
    }

    #[test]
    fn test_checked_supplier_error() {
        let supplier =
            checked_supplier(|| Err::<i32, IoError>(IoError::new(ErrorKind::Other, "test error")));
        assert!(supplier.checked_get().is_err());
    }

    #[test]
    fn test_checked_function_ok() {
        let function = checked_function(|x: i32| Ok::<i32, IoError>(x * 2));
        assert_eq!(function.checked_apply(21).unwrap(), 42);
        assert_eq!(function.apply(21), 42);
    }

    #[test]
    fn test_checked_function_error() {
        let function = checked_function(|_x: i32| {
            Err::<i32, IoError>(IoError::new(ErrorKind::Other, "test error"))
        });
        assert!(function.checked_apply(21).is_err());
    }

    #[test]
    fn test_checked_consumer_ok() {
        let consumer = checked_consumer(|_x: i32| Ok::<(), IoError>(()));
        consumer.checked_accept(42).unwrap();
        consumer.accept(42);
    }

    #[test]
    fn test_checked_consumer_error() {
        let consumer = checked_consumer(|_x: i32| {
            Err::<(), IoError>(IoError::new(ErrorKind::Other, "test error"))
        });
        assert!(consumer.checked_accept(42).is_err());
    }

    #[test]
    fn test_checked_runnable_ok() {
        let runnable = checked_runnable(|| Ok::<(), IoError>(()));
        runnable.checked_run().unwrap();
        runnable.run();
    }

    #[test]
    fn test_checked_runnable_error() {
        let runnable =
            checked_runnable(|| Err::<(), IoError>(IoError::new(ErrorKind::Other, "test error")));
        assert!(runnable.checked_run().is_err());
    }
}
