/// Monitor for checking termination status.
///
/// This is a trait (Rust's equivalent of Java's functional interface) that
/// algorithms can use to check if termination has been requested.
///
/// # Examples
///
/// ```
/// use gds::termination::TerminationMonitor;
///
/// struct MyMonitor {
///     should_stop: bool,
/// }
///
/// impl TerminationMonitor for MyMonitor {
///     fn is_terminated(&self) -> bool {
///         self.should_stop
///     }
/// }
/// ```
pub trait TerminationMonitor {
    /// Checks if termination has been requested.
    ///
    /// Returns `true` if the computation should terminate, `false` to continue.
    fn is_terminated(&self) -> bool;
}

/// Blanket implementation for Arc-wrapped monitors.
///
/// This allows using `Arc<T>` wherever `T: TerminationMonitor` is expected,
/// which is essential for sharing monitors across threads.
impl<T: TerminationMonitor + ?Sized> TerminationMonitor for std::sync::Arc<T> {
    fn is_terminated(&self) -> bool {
        (**self).is_terminated()
    }
}

/// Empty monitor that never terminates.
///
/// This is useful for algorithms that don't support cancellation or
/// for testing purposes.
///
/// # Examples
///
/// ```
/// use gds::termination::{TerminationMonitor, EmptyTerminationMonitor};
///
/// let monitor = EmptyTerminationMonitor;
/// assert!(!monitor.is_terminated());
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyTerminationMonitor;

impl TerminationMonitor for EmptyTerminationMonitor {
    fn is_terminated(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_monitor_never_terminates() {
        let monitor = EmptyTerminationMonitor;
        assert!(!monitor.is_terminated());
        assert!(!monitor.is_terminated()); // Multiple calls
    }

    #[test]
    fn test_empty_monitor_default() {
        let monitor = EmptyTerminationMonitor::default();
        assert!(!monitor.is_terminated());
    }

    #[test]
    fn test_custom_monitor() {
        struct CustomMonitor {
            terminated: bool,
        }

        impl TerminationMonitor for CustomMonitor {
            fn is_terminated(&self) -> bool {
                self.terminated
            }
        }

        let running = CustomMonitor { terminated: false };
        assert!(!running.is_terminated());

        let stopped = CustomMonitor { terminated: true };
        assert!(stopped.is_terminated());
    }

    #[test]
    fn test_trait_object() {
        let monitor: Box<dyn TerminationMonitor> = Box::new(EmptyTerminationMonitor);
        assert!(!monitor.is_terminated());
    }
}
