use std::sync::atomic::{AtomicI64, Ordering};

/// Atomic minimum tracker using lock-free Compare-And-Swap operations.
///
/// Tracks the minimum value seen across multiple threads without locks.
/// Uses CAS loops to update the minimum atomically.
///
/// # Examples
///
/// ```
/// use gds::concurrency::atomics::AtomicMin;
/// use std::sync::atomic::Ordering;
///
/// let min = AtomicMin::new(100);
/// min.update(42, Ordering::SeqCst);
/// min.update(25, Ordering::SeqCst);
/// min.update(75, Ordering::SeqCst);  // Ignored - not smaller
/// assert_eq!(min.get(Ordering::SeqCst), 25);
/// ```
#[derive(Debug)]
pub struct AtomicMin {
    value: AtomicI64,
}

impl AtomicMin {
    /// Creates a new `AtomicMin` with the given initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMin;
    ///
    /// let min = AtomicMin::new(42);
    /// ```
    pub fn new(initial: i64) -> Self {
        Self {
            value: AtomicI64::new(initial),
        }
    }

    /// Creates a new `AtomicMin` initialized to i64::MAX.
    ///
    /// This is useful when you want to track the minimum of observed values
    /// without setting an initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMin;
    /// use std::sync::atomic::Ordering;
    ///
    /// let min = AtomicMin::max_value();
    /// min.update(42, Ordering::SeqCst);
    /// assert_eq!(min.get(Ordering::SeqCst), 42);
    /// ```
    pub fn max_value() -> Self {
        Self::new(i64::MAX)
    }

    /// Gets the current minimum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMin;
    /// use std::sync::atomic::Ordering;
    ///
    /// let min = AtomicMin::new(42);
    /// assert_eq!(min.get(Ordering::SeqCst), 42);
    /// ```
    pub fn get(&self, order: Ordering) -> i64 {
        self.value.load(order)
    }

    /// Updates the minimum if the given value is smaller.
    ///
    /// Uses a CAS loop to atomically update the minimum. Returns the
    /// minimum value after the update (which may be smaller than `value`
    /// if another thread updated it).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMin;
    /// use std::sync::atomic::Ordering;
    ///
    /// let min = AtomicMin::new(100);
    /// min.update(42, Ordering::SeqCst);
    /// assert_eq!(min.get(Ordering::SeqCst), 42);
    ///
    /// // Larger value doesn't change the min
    /// min.update(75, Ordering::SeqCst);
    /// assert_eq!(min.get(Ordering::SeqCst), 42);
    /// ```
    pub fn update(&self, value: i64, order: Ordering) -> i64 {
        let mut current = self.value.load(Ordering::Relaxed);
        loop {
            if value >= current {
                // Value is not smaller than current min
                return current;
            }

            match self
                .value
                .compare_exchange_weak(current, value, order, Ordering::Relaxed)
            {
                Ok(_) => return value,
                Err(actual) => current = actual,
            }
        }
    }

    /// Sets the value directly without comparison (unsafe for concurrent updates).
    ///
    /// This is useful for initialization or when you know there's no contention.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMin;
    /// use std::sync::atomic::Ordering;
    ///
    /// let min = AtomicMin::new(100);
    /// min.set(42, Ordering::SeqCst);
    /// assert_eq!(min.get(Ordering::SeqCst), 42);
    /// ```
    pub fn set(&self, value: i64, order: Ordering) {
        self.value.store(value, order);
    }
}

impl Default for AtomicMin {
    fn default() -> Self {
        Self::max_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new() {
        let min = AtomicMin::new(42);
        assert_eq!(min.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_max_value() {
        let min = AtomicMin::max_value();
        assert_eq!(min.get(Ordering::SeqCst), i64::MAX);
    }

    #[test]
    fn test_default() {
        let min = AtomicMin::default();
        assert_eq!(min.get(Ordering::SeqCst), i64::MAX);
    }

    #[test]
    fn test_update_decreases() {
        let min = AtomicMin::new(100);
        min.update(42, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_update_ignores_larger() {
        let min = AtomicMin::new(10);
        min.update(42, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_update_equal_ignored() {
        let min = AtomicMin::new(42);
        min.update(42, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_set() {
        let min = AtomicMin::new(100);
        min.set(42, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_concurrent_updates() {
        let min = Arc::new(AtomicMin::new(1000));
        let mut handles = vec![];

        // Spawn 10 threads, each updating with its thread number
        for i in 1..=10 {
            let min_clone = Arc::clone(&min);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    min_clone.update(i * 10, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Minimum should be 1 * 10 = 10
        assert_eq!(min.get(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_concurrent_mixed_values() {
        let min = Arc::new(AtomicMin::new(1000));
        let mut handles = vec![];

        // Thread 1: updates with decreasing values
        let min1 = Arc::clone(&min);
        handles.push(thread::spawn(move || {
            for i in (1..=100).rev() {
                min1.update(i, Ordering::SeqCst);
            }
        }));

        // Thread 2: updates with small value then larger values
        let min2 = Arc::clone(&min);
        handles.push(thread::spawn(move || {
            min2.update(5, Ordering::SeqCst);
            for i in 50..=100 {
                min2.update(i, Ordering::SeqCst);
            }
        }));

        for handle in handles {
            handle.join().unwrap();
        }

        // Minimum should be 1
        assert_eq!(min.get(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_negative_values() {
        let min = AtomicMin::new(-10);
        min.update(-50, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), -50);

        min.update(-25, Ordering::SeqCst);
        assert_eq!(min.get(Ordering::SeqCst), -50);
    }
}
