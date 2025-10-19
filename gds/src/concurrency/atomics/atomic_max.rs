use std::sync::atomic::{AtomicI64, Ordering};

/// Atomic maximum tracker using lock-free Compare-And-Swap operations.
///
/// Tracks the maximum value seen across multiple threads without locks.
/// Uses CAS loops to update the maximum atomically.
///
/// # Examples
///
/// ```
/// use gds::concurrency::atomics::AtomicMax;
/// use std::sync::atomic::Ordering;
///
/// let max = AtomicMax::new(0);
/// max.update(42, Ordering::SeqCst);
/// max.update(100, Ordering::SeqCst);
/// max.update(25, Ordering::SeqCst);  // Ignored - not greater
/// assert_eq!(max.get(Ordering::SeqCst), 100);
/// ```
#[derive(Debug)]
pub struct AtomicMax {
    value: AtomicI64,
}

impl AtomicMax {
    /// Creates a new `AtomicMax` with the given initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMax;
    ///
    /// let max = AtomicMax::new(42);
    /// ```
    pub fn new(initial: i64) -> Self {
        Self {
            value: AtomicI64::new(initial),
        }
    }

    /// Creates a new `AtomicMax` initialized to i64::MIN.
    ///
    /// This is useful when you want to track the maximum of observed values
    /// without setting an initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMax;
    /// use std::sync::atomic::Ordering;
    ///
    /// let max = AtomicMax::min_value();
    /// max.update(42, Ordering::SeqCst);
    /// assert_eq!(max.get(Ordering::SeqCst), 42);
    /// ```
    pub fn min_value() -> Self {
        Self::new(i64::MIN)
    }

    /// Gets the current maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMax;
    /// use std::sync::atomic::Ordering;
    ///
    /// let max = AtomicMax::new(42);
    /// assert_eq!(max.get(Ordering::SeqCst), 42);
    /// ```
    pub fn get(&self, order: Ordering) -> i64 {
        self.value.load(order)
    }

    /// Updates the maximum if the given value is greater.
    ///
    /// Uses a CAS loop to atomically update the maximum. Returns the
    /// maximum value after the update (which may be larger than `value`
    /// if another thread updated it).
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::atomics::AtomicMax;
    /// use std::sync::atomic::Ordering;
    ///
    /// let max = AtomicMax::new(10);
    /// max.update(42, Ordering::SeqCst);
    /// assert_eq!(max.get(Ordering::SeqCst), 42);
    ///
    /// // Smaller value doesn't change the max
    /// max.update(20, Ordering::SeqCst);
    /// assert_eq!(max.get(Ordering::SeqCst), 42);
    /// ```
    pub fn update(&self, value: i64, order: Ordering) -> i64 {
        let mut current = self.value.load(Ordering::Relaxed);
        loop {
            if value <= current {
                // Value is not greater than current max
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
    /// use gds::concurrency::atomics::AtomicMax;
    /// use std::sync::atomic::Ordering;
    ///
    /// let max = AtomicMax::new(0);
    /// max.set(42, Ordering::SeqCst);
    /// assert_eq!(max.get(Ordering::SeqCst), 42);
    /// ```
    pub fn set(&self, value: i64, order: Ordering) {
        self.value.store(value, order);
    }
}

impl Default for AtomicMax {
    fn default() -> Self {
        Self::min_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new() {
        let max = AtomicMax::new(42);
        assert_eq!(max.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_min_value() {
        let max = AtomicMax::min_value();
        assert_eq!(max.get(Ordering::SeqCst), i64::MIN);
    }

    #[test]
    fn test_default() {
        let max = AtomicMax::default();
        assert_eq!(max.get(Ordering::SeqCst), i64::MIN);
    }

    #[test]
    fn test_update_increases() {
        let max = AtomicMax::new(10);
        max.update(42, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_update_ignores_smaller() {
        let max = AtomicMax::new(100);
        max.update(42, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_update_equal_ignored() {
        let max = AtomicMax::new(42);
        max.update(42, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_set() {
        let max = AtomicMax::new(10);
        max.set(42, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_concurrent_updates() {
        let max = Arc::new(AtomicMax::new(0));
        let mut handles = vec![];

        // Spawn 10 threads, each updating with its thread number
        for i in 1..=10 {
            let max_clone = Arc::clone(&max);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    max_clone.update(i * 10, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Maximum should be 10 * 10 = 100
        assert_eq!(max.get(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_concurrent_mixed_values() {
        let max = Arc::new(AtomicMax::new(0));
        let mut handles = vec![];

        // Thread 1: updates with increasing values
        let max1 = Arc::clone(&max);
        handles.push(thread::spawn(move || {
            for i in 1..=100 {
                max1.update(i, Ordering::SeqCst);
            }
        }));

        // Thread 2: updates with large value then smaller values
        let max2 = Arc::clone(&max);
        handles.push(thread::spawn(move || {
            max2.update(1000, Ordering::SeqCst);
            for i in 1..=50 {
                max2.update(i, Ordering::SeqCst);
            }
        }));

        for handle in handles {
            handle.join().unwrap();
        }

        // Maximum should be 1000
        assert_eq!(max.get(Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_negative_values() {
        let max = AtomicMax::new(-100);
        max.update(-50, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), -50);

        max.update(-200, Ordering::SeqCst);
        assert_eq!(max.get(Ordering::SeqCst), -50);
    }
}
