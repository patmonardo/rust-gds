use super::{TerminatedException, TerminationMonitor};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Flag for checking if an algorithm should terminate.
///
/// This provides throttled termination checking with a configurable interval.
/// Checking is throttled to avoid excessive overhead - by default, the
/// underlying monitor is only queried once every 10 seconds.
///
/// # Examples
///
/// ```
/// use rust_gds::termination::{TerminationFlag, EmptyTerminationMonitor};
///
/// let flag = TerminationFlag::new(EmptyTerminationMonitor);
///
/// // Check if running
/// assert!(flag.running());
///
/// // Assert running (panics if terminated)
/// flag.assert_running();
/// ```
pub struct TerminationFlag {
    monitor: Arc<dyn TerminationMonitor + Send + Sync>,
    last_check: parking_lot::Mutex<Instant>,
    cached_running: AtomicBool,
    interval: Duration,
}

impl TerminationFlag {
    /// Default check interval: 10 seconds.
    pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(10);

    /// Creates a new termination flag with the default interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::termination::{TerminationFlag, EmptyTerminationMonitor};
    ///
    /// let flag = TerminationFlag::new(EmptyTerminationMonitor);
    /// assert!(flag.running());
    /// ```
    pub fn new<M>(monitor: M) -> Self
    where
        M: TerminationMonitor + Send + Sync + 'static,
    {
        Self::with_interval(monitor, Self::DEFAULT_INTERVAL)
    }

    /// Creates a new termination flag with a custom check interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::termination::{TerminationFlag, EmptyTerminationMonitor};
    /// use std::time::Duration;
    ///
    /// let flag = TerminationFlag::with_interval(
    ///     EmptyTerminationMonitor,
    ///     Duration::from_secs(5)
    /// );
    /// assert!(flag.running());
    /// ```
    pub fn with_interval<M>(monitor: M, interval: Duration) -> Self
    where
        M: TerminationMonitor + Send + Sync + 'static,
    {
        Self {
            monitor: Arc::new(monitor),
            last_check: parking_lot::Mutex::new(Instant::now()),
            cached_running: AtomicBool::new(true),
            interval,
        }
    }

    /// Checks if the computation should continue running.
    ///
    /// This method is throttled - it only queries the underlying monitor
    /// once per interval period. Between checks, it returns a cached value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::termination::{TerminationFlag, EmptyTerminationMonitor};
    ///
    /// let flag = TerminationFlag::new(EmptyTerminationMonitor);
    /// assert!(flag.running());
    /// ```
    pub fn running(&self) -> bool {
        let now = Instant::now();
        let mut last_check = self.last_check.lock();

        if now.duration_since(*last_check) > self.interval {
            // Time to check the monitor
            if self.monitor.is_terminated() {
                self.cached_running.store(false, Ordering::Release);
            }
            *last_check = now;
        }

        self.cached_running.load(Ordering::Acquire)
    }

    /// Asserts that the computation is still running.
    ///
    /// # Panics
    ///
    /// Panics with a `TerminatedException` if the computation should terminate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::termination::{TerminationFlag, EmptyTerminationMonitor};
    ///
    /// let flag = TerminationFlag::new(EmptyTerminationMonitor);
    /// flag.assert_running(); // Does not panic
    /// ```
    pub fn assert_running(&self) {
        if !self.running() {
            panic!("{}", TerminatedException);
        }
    }

    /// Forcefully terminates the computation.
    ///
    /// This immediately panics with a `TerminatedException`.
    ///
    /// # Panics
    ///
    /// Always panics with a `TerminatedException`.
    pub fn terminate(&self) -> ! {
        panic!("{}", TerminatedException);
    }
}

// Convenience constructors
impl TerminationFlag {
    /// Creates a flag that always returns `true` (always running).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let flag = TerminationFlag::running_true();
    /// assert!(flag.running());
    /// ```
    pub fn running_true() -> Self {
        use crate::termination::termination_monitor::EmptyTerminationMonitor;
        Self::new(EmptyTerminationMonitor)
    }

    /// Creates a flag that always returns `false` (never running).
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let flag = TerminationFlag::stop_running();
    /// flag.assert_running(); // Panics!
    /// ```
    pub fn stop_running() -> Self {
        struct AlwaysTerminated;
        impl TerminationMonitor for AlwaysTerminated {
            fn is_terminated(&self) -> bool {
                true
            }
        }
        let flag = Self::new(AlwaysTerminated);
        // Force immediate check to set cached state correctly
        flag.cached_running.store(false, Ordering::Release);
        flag
    }
}

impl Default for TerminationFlag {
    /// Returns a flag that always runs (never terminates).
    fn default() -> Self {
        Self::running_true()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_running_true() {
        let flag = TerminationFlag::running_true();
        assert!(flag.running());
        assert!(flag.running()); // Multiple calls
    }

    #[test]
    fn test_default() {
        let flag = TerminationFlag::default();
        assert!(flag.running());
    }

    #[test]
    fn test_stop_running() {
        let flag = TerminationFlag::stop_running();
        thread::sleep(Duration::from_millis(50)); // Wait for first check
        assert!(!flag.running());
    }

    #[test]
    fn test_assert_running_success() {
        let flag = TerminationFlag::running_true();
        flag.assert_running(); // Should not panic
    }

    #[test]
    #[should_panic(expected = "terminated")]
    fn test_assert_running_panics() {
        let flag = TerminationFlag::stop_running();
        thread::sleep(Duration::from_millis(50));
        flag.assert_running(); // Should panic
    }

    #[test]
    #[should_panic(expected = "terminated")]
    fn test_terminate() {
        let flag = TerminationFlag::running_true();
        flag.terminate(); // Always panics
    }

    #[test]
    fn test_custom_monitor() {
        struct TestMonitor {
            terminated: AtomicBool,
        }
        impl TerminationMonitor for TestMonitor {
            fn is_terminated(&self) -> bool {
                self.terminated.load(Ordering::Acquire)
            }
        }

        let monitor = Arc::new(TestMonitor {
            terminated: AtomicBool::new(false),
        });
        let monitor_clone = Arc::clone(&monitor);

        // Use a very short interval for testing
        let flag = TerminationFlag::with_interval(monitor_clone, Duration::from_millis(10));

        assert!(flag.running());

        // Terminate the monitor
        monitor.terminated.store(true, Ordering::Release);

        // Wait for next check
        thread::sleep(Duration::from_millis(50));

        assert!(!flag.running());
    }

    #[test]
    fn test_throttling() {
        struct CountingMonitor {
            count: AtomicBool,
            check_count: parking_lot::Mutex<usize>,
        }
        impl TerminationMonitor for CountingMonitor {
            fn is_terminated(&self) -> bool {
                *self.check_count.lock() += 1;
                self.count.load(Ordering::Acquire)
            }
        }

        let monitor = Arc::new(CountingMonitor {
            count: AtomicBool::new(false),
            check_count: parking_lot::Mutex::new(0),
        });
        let monitor_clone = Arc::clone(&monitor);

        let flag = TerminationFlag::with_interval(monitor_clone, Duration::from_millis(100));

        // Multiple rapid calls should only check once initially
        for _ in 0..10 {
            flag.running();
        }

        let initial_count = *monitor.check_count.lock();
        assert!(initial_count <= 2); // May check once or twice due to timing

        // Wait for interval
        thread::sleep(Duration::from_millis(150));

        // Another check should query the monitor
        flag.running();
        let final_count = *monitor.check_count.lock();
        assert!(final_count > initial_count);
    }

    #[test]
    fn test_thread_safety() {
        let flag = Arc::new(TerminationFlag::running_true());
        let mut handles = vec![];

        for _ in 0..10 {
            let flag_clone = Arc::clone(&flag);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    assert!(flag_clone.running());
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
