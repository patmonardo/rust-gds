use std::sync::atomic::{AtomicU64, Ordering};

/// Atomic f64 value using bit-casting to/from u64.
///
/// Provides atomic operations on floating-point values by storing them as u64
/// and converting via bit-casting. This matches the Java GDS `AtomicDouble` implementation
/// which uses `AtomicLong` + `Double.longBitsToDouble/doubleToLongBits`.
///
/// # Thread Safety
///
/// All operations are atomic and thread-safe. Multiple threads can safely read and write
/// to the same `AtomicDouble` concurrently.
///
/// # Memory Ordering
///
/// Most operations use `Ordering::SeqCst` for full sequential consistency, matching
/// Java's default volatile semantics. For performance-critical code, weaker orderings
/// can be used with the explicit ordering methods.
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::atomics::AtomicDouble;
/// use std::sync::atomic::Ordering;
///
/// let atomic = AtomicDouble::new(1.5);
/// assert_eq!(atomic.load(Ordering::SeqCst), 1.5);
///
/// atomic.store(2.5, Ordering::SeqCst);
/// assert_eq!(atomic.load(Ordering::SeqCst), 2.5);
///
/// // Atomic add via CAS loop
/// atomic.add(0.5, Ordering::SeqCst);
/// assert_eq!(atomic.load(Ordering::SeqCst), 3.0);
/// ```
#[derive(Debug)]
pub struct AtomicDouble {
    bits: AtomicU64,
}

impl AtomicDouble {
    /// Creates a new `AtomicDouble` with the given initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    ///
    /// let atomic = AtomicDouble::new(3.14);
    /// ```
    pub fn new(value: f64) -> Self {
        Self {
            bits: AtomicU64::new(value.to_bits()),
        }
    }

    /// Creates a new `AtomicDouble` with value 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::zero();
    /// assert_eq!(atomic.load(Ordering::SeqCst), 0.0);
    /// ```
    pub fn zero() -> Self {
        Self::new(0.0)
    }

    /// Loads a value from the atomic double.
    ///
    /// `load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Release`] or [`AcqRel`].
    ///
    /// [`Release`]: Ordering::Release
    /// [`AcqRel`]: Ordering::AcqRel
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    /// assert_eq!(atomic.load(Ordering::SeqCst), 5.0);
    /// ```
    pub fn load(&self, order: Ordering) -> f64 {
        f64::from_bits(self.bits.load(order))
    }

    /// Stores a value into the atomic double.
    ///
    /// `store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Acquire`] or [`AcqRel`].
    ///
    /// [`Acquire`]: Ordering::Acquire
    /// [`AcqRel`]: Ordering::AcqRel
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    /// atomic.store(10.0, Ordering::SeqCst);
    /// assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    /// ```
    pub fn store(&self, value: f64, order: Ordering) {
        self.bits.store(value.to_bits(), order);
    }

    /// Stores a value into the atomic double, returning the previous value.
    ///
    /// `swap` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    /// let old = atomic.swap(10.0, Ordering::SeqCst);
    /// assert_eq!(old, 5.0);
    /// assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    /// ```
    pub fn swap(&self, value: f64, order: Ordering) -> f64 {
        f64::from_bits(self.bits.swap(value.to_bits(), order))
    }

    /// Stores a value into the atomic double if the current value is the same as the `current` value.
    ///
    /// The return value is a result indicating whether the new value was written and containing
    /// the previous value. On success this value is guaranteed to be equal to `current`.
    ///
    /// `compare_exchange` takes two [`Ordering`] arguments to describe the memory ordering of this
    /// operation. `success` describes the required ordering for the read-modify-write operation that takes
    /// place if the comparison with `current` succeeds. `failure` describes the required ordering for the
    /// load operation that takes place when the comparison fails. Using [`Acquire`] as success ordering
    /// makes the store part of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// [`Relaxed`]: Ordering::Relaxed
    /// [`Release`]: Ordering::Release
    /// [`Acquire`]: Ordering::Acquire
    /// [`SeqCst`]: Ordering::SeqCst
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    ///
    /// // Successful CAS
    /// assert_eq!(
    ///     atomic.compare_exchange(5.0, 10.0, Ordering::SeqCst, Ordering::SeqCst),
    ///     Ok(5.0)
    /// );
    /// assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    ///
    /// // Failed CAS
    /// assert_eq!(
    ///     atomic.compare_exchange(5.0, 15.0, Ordering::SeqCst, Ordering::SeqCst),
    ///     Err(10.0)
    /// );
    /// assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    /// ```
    pub fn compare_exchange(
        &self,
        current: f64,
        new: f64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f64, f64> {
        self.bits
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }

    /// Stores a value into the atomic double if the current value is the same as the `current` value.
    ///
    /// Unlike [`AtomicDouble::compare_exchange`], this function is allowed to spuriously fail even when the
    /// comparison succeeds, which can result in more efficient code on some platforms. The return value
    /// is a result indicating whether the new value was written and containing the previous value.
    ///
    /// `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory ordering of this
    /// operation. `success` describes the required ordering for the read-modify-write operation that takes
    /// place if the comparison with `current` succeeds. `failure` describes the required ordering for the
    /// load operation that takes place when the comparison fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    ///
    /// let mut current = atomic.load(Ordering::Relaxed);
    /// loop {
    ///     match atomic.compare_exchange_weak(current, 10.0, Ordering::SeqCst, Ordering::Relaxed) {
    ///         Ok(_) => break,
    ///         Err(actual) => current = actual,
    ///     }
    /// }
    /// assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    /// ```
    pub fn compare_exchange_weak(
        &self,
        current: f64,
        new: f64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f64, f64> {
        self.bits
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }

    /// Atomically adds a value to the current value using a CAS loop.
    ///
    /// This operation performs a compare-and-swap loop to add the value atomically.
    /// Returns the previous value before the addition.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    /// let old = atomic.add(3.0, Ordering::SeqCst);
    /// assert_eq!(old, 5.0);
    /// assert_eq!(atomic.load(Ordering::SeqCst), 8.0);
    /// ```
    pub fn add(&self, value: f64, order: Ordering) -> f64 {
        let mut current = self.bits.load(Ordering::Relaxed);
        loop {
            let current_f64 = f64::from_bits(current);
            let new_f64 = current_f64 + value;
            let new_bits = new_f64.to_bits();

            match self
                .bits
                .compare_exchange_weak(current, new_bits, order, Ordering::Relaxed)
            {
                Ok(_) => return current_f64,
                Err(actual) => current = actual,
            }
        }
    }

    /// Atomically subtracts a value from the current value using a CAS loop.
    ///
    /// This operation performs a compare-and-swap loop to subtract the value atomically.
    /// Returns the previous value before the subtraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::atomics::AtomicDouble;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicDouble::new(5.0);
    /// let old = atomic.sub(3.0, Ordering::SeqCst);
    /// assert_eq!(old, 5.0);
    /// assert_eq!(atomic.load(Ordering::SeqCst), 2.0);
    /// ```
    pub fn sub(&self, value: f64, order: Ordering) -> f64 {
        self.add(-value, order)
    }
}

impl Default for AtomicDouble {
    /// Creates an `AtomicDouble` with value 0.0.
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new() {
        let atomic = AtomicDouble::new(3.14);
        assert_eq!(atomic.load(Ordering::SeqCst), 3.14);
    }

    #[test]
    fn test_zero() {
        let atomic = AtomicDouble::zero();
        assert_eq!(atomic.load(Ordering::SeqCst), 0.0);
    }

    #[test]
    fn test_load_store() {
        let atomic = AtomicDouble::new(1.5);
        assert_eq!(atomic.load(Ordering::SeqCst), 1.5);

        atomic.store(2.5, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), 2.5);
    }

    #[test]
    fn test_swap() {
        let atomic = AtomicDouble::new(5.0);
        let old = atomic.swap(10.0, Ordering::SeqCst);
        assert_eq!(old, 5.0);
        assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    }

    #[test]
    fn test_compare_exchange_success() {
        let atomic = AtomicDouble::new(5.0);
        let result = atomic.compare_exchange(5.0, 10.0, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(result, Ok(5.0));
        assert_eq!(atomic.load(Ordering::SeqCst), 10.0);
    }

    #[test]
    fn test_compare_exchange_failure() {
        let atomic = AtomicDouble::new(5.0);
        let result = atomic.compare_exchange(3.0, 10.0, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(result, Err(5.0));
        assert_eq!(atomic.load(Ordering::SeqCst), 5.0);
    }

    #[test]
    fn test_add() {
        let atomic = AtomicDouble::new(5.0);
        let old = atomic.add(3.0, Ordering::SeqCst);
        assert_eq!(old, 5.0);
        assert_eq!(atomic.load(Ordering::SeqCst), 8.0);
    }

    #[test]
    fn test_sub() {
        let atomic = AtomicDouble::new(5.0);
        let old = atomic.sub(3.0, Ordering::SeqCst);
        assert_eq!(old, 5.0);
        assert_eq!(atomic.load(Ordering::SeqCst), 2.0);
    }

    #[test]
    fn test_default() {
        let atomic = AtomicDouble::default();
        assert_eq!(atomic.load(Ordering::SeqCst), 0.0);
    }

    #[test]
    fn test_concurrent_adds() {
        let atomic = Arc::new(AtomicDouble::new(0.0));
        let num_threads = 10;
        let adds_per_thread = 1000;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let atomic_clone = Arc::clone(&atomic);
                thread::spawn(move || {
                    for _ in 0..adds_per_thread {
                        atomic_clone.add(0.1, Ordering::SeqCst);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let expected = (num_threads * adds_per_thread) as f64 * 0.1;
        let result = atomic.load(Ordering::SeqCst);
        assert!(
            (result - expected).abs() < 0.001,
            "Expected {}, got {}",
            expected,
            result
        );
    }

    #[test]
    fn test_concurrent_cas() {
        let atomic = Arc::new(AtomicDouble::new(0.0));
        let num_threads = 10;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let atomic_clone = Arc::clone(&atomic);
                thread::spawn(move || loop {
                    let current = atomic_clone.load(Ordering::Relaxed);
                    if atomic_clone
                        .compare_exchange_weak(
                            current,
                            current + 1.0,
                            Ordering::SeqCst,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                    {
                        break;
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(atomic.load(Ordering::SeqCst), num_threads as f64);
    }
}
