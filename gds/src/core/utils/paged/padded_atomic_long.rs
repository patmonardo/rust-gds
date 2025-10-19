//! Cache-line padded atomic long to prevent false sharing in multi-threaded scenarios.
//!
//! Essential for high-performance concurrent programming:
//! - Prevents false sharing between CPU cache lines
//! - Optimizes memory access patterns in multi-threaded code
//! - Reduces cache coherency traffic between CPU cores
//! - Maximizes throughput for heavily contended atomic operations
//! - Critical for scalable concurrent data structures
//!
//! # Performance Benefits
//!
//! - Eliminates cache line ping-ponging between cores
//! - Reduces memory bandwidth consumption
//! - Improves atomic operation throughput
//! - Scales better with increasing thread count
//! - Essential for NUMA-aware programming
//!
//! # Technical Details
//!
//! - Cache lines are typically 64 bytes on modern CPUs
//! - `AtomicI64` is 8 bytes, leaving 56 bytes for padding
//! - Padding fields prevent other variables sharing the cache line
//! - `sum()` method prevents compiler optimization removal
//!
//! # Use Cases
//!
//! - High-contention counters and accumulators
//! - Concurrent data structure coordination
//! - Performance-critical atomic operations
//! - Multi-threaded graph processing coordination
//! - Lock-free algorithm implementations
//!
//! # References
//!
//! - [False Sharing](http://mechanical-sympathy.blogspot.ch/2011/08/false-sharing-java-7.html)
//!
//! # Examples
//!
//! ```rust
//! use rust_gds::core::utils::paged::PaddedAtomicLong;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // High-contention counter scenario
//! let counter = Arc::new(PaddedAtomicLong::new(0));
//! let mut handles = vec![];
//!
//! for _ in 0..4 {
//!     let counter_clone = Arc::clone(&counter);
//!     let handle = thread::spawn(move || {
//!         for _ in 0..1_000_000 {
//!             counter_clone.fetch_add(1);
//!         }
//!     });
//!     handles.push(handle);
//! }
//!
//! for handle in handles {
//!     handle.join().unwrap();
//! }
//!
//! assert_eq!(counter.get(), 4_000_000);
//! ```

use std::sync::atomic::{AtomicI64, Ordering};

/// Cache-line padded atomic long for optimal multi-threaded performance.
///
/// Prevents false sharing by padding to cache line boundaries (64 bytes).
/// Critical for high-performance concurrent operations where multiple threads
/// frequently access different atomic variables.
///
/// # False Sharing Problem
///
/// When multiple threads access different variables that happen to be on the
/// same CPU cache line, they cause unnecessary cache coherency traffic.
/// Each write by one thread invalidates the entire cache line for all other
/// threads, even if they're working on different variables.
///
/// # Solution
///
/// By padding the atomic value to occupy an entire cache line (64 bytes),
/// we ensure that each `PaddedAtomicLong` is on its own cache line, eliminating
/// false sharing.
///
/// # Memory Layout
///
/// ```text
/// |<---- 64 bytes (one cache line) ---->|
/// | AtomicI64 (8B) | Padding (56B)     |
/// | value          | p1 p2 p3 p4 p5... |
/// ```
///
/// # Examples
///
/// ## Without Padding (False Sharing)
/// ```rust
/// use std::sync::atomic::{AtomicI64, Ordering};
/// use std::sync::Arc;
///
/// struct Counters {
///     counter1: AtomicI64,  // These may share a cache line!
///     counter2: AtomicI64,  // Causing false sharing
/// }
/// ```
///
/// ## With Padding (No False Sharing)
/// ```rust
/// use rust_gds::core::utils::paged::PaddedAtomicLong;
///
/// struct Counters {
///     counter1: PaddedAtomicLong,  // Each on own cache line
///     counter2: PaddedAtomicLong,  // No false sharing!
/// }
/// ```
#[repr(C)]
pub struct PaddedAtomicLong {
    /// The atomic value
    value: AtomicI64,

    /// Cache line padding to prevent false sharing
    /// Modern CPU cache lines are typically 64 bytes
    /// AtomicI64 (8 bytes) + padding (7 * 8 bytes) = 64 bytes total
    p1: i64,
    p2: i64,
    p3: i64,
    p4: i64,
    p5: i64,
    p6: i64,
    p7: i64,
}

impl PaddedAtomicLong {
    /// Creates a new `PaddedAtomicLong` with the given initial value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(0);
    /// assert_eq!(counter.get(), 0);
    ///
    /// let initialized = PaddedAtomicLong::new(42);
    /// assert_eq!(initialized.get(), 42);
    /// ```
    pub fn new(value: i64) -> Self {
        Self {
            value: AtomicI64::new(value),
            // Initialize padding fields to non-zero values
            // This prevents compiler optimizations from removing them
            p1: 1,
            p2: 2,
            p3: 3,
            p4: 4,
            p5: 5,
            p6: 6,
            p7: 7,
        }
    }

    /// Gets the current value.
    ///
    /// Uses `Ordering::SeqCst` for strongest consistency guarantees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(42);
    /// assert_eq!(counter.get(), 42);
    /// ```
    #[inline]
    pub fn get(&self) -> i64 {
        self.value.load(Ordering::SeqCst)
    }

    /// Sets the value.
    ///
    /// Uses `Ordering::SeqCst` for strongest consistency guarantees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(0);
    /// counter.set(42);
    /// assert_eq!(counter.get(), 42);
    /// ```
    #[inline]
    pub fn set(&self, value: i64) {
        self.value.store(value, Ordering::SeqCst);
    }

    /// Atomically adds the given value and returns the new value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.add_and_get(5), 15);
    /// assert_eq!(counter.get(), 15);
    /// ```
    #[inline]
    pub fn add_and_get(&self, delta: i64) -> i64 {
        self.value.fetch_add(delta, Ordering::SeqCst) + delta
    }

    /// Atomically increments by one and returns the new value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(0);
    /// assert_eq!(counter.increment_and_get(), 1);
    /// assert_eq!(counter.increment_and_get(), 2);
    /// assert_eq!(counter.get(), 2);
    /// ```
    #[inline]
    pub fn increment_and_get(&self) -> i64 {
        self.add_and_get(1)
    }

    /// Atomically decrements by one and returns the new value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.decrement_and_get(), 9);
    /// assert_eq!(counter.decrement_and_get(), 8);
    /// assert_eq!(counter.get(), 8);
    /// ```
    #[inline]
    pub fn decrement_and_get(&self) -> i64 {
        self.add_and_get(-1)
    }

    /// Atomically sets to the given value and returns the old value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.get_and_set(42), 10);
    /// assert_eq!(counter.get(), 42);
    /// ```
    #[inline]
    pub fn get_and_set(&self, new_value: i64) -> i64 {
        self.value.swap(new_value, Ordering::SeqCst)
    }

    /// Atomically sets the value to the given updated value if the current value equals the expected value.
    ///
    /// Returns `true` if the update succeeded, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    ///
    /// // Success case
    /// assert!(counter.compare_and_set(10, 20));
    /// assert_eq!(counter.get(), 20);
    ///
    /// // Failure case
    /// assert!(!counter.compare_and_set(10, 30)); // Expected 10, but value is 20
    /// assert_eq!(counter.get(), 20); // Value unchanged
    /// ```
    #[inline]
    pub fn compare_and_set(&self, expect: i64, update: i64) -> bool {
        self.value
            .compare_exchange(expect, update, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    /// Returns the current value and atomically increments by one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.get_and_increment(), 10);
    /// assert_eq!(counter.get(), 11);
    /// ```
    #[inline]
    pub fn get_and_increment(&self) -> i64 {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    /// Returns the current value and atomically decrements by one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.get_and_decrement(), 10);
    /// assert_eq!(counter.get(), 9);
    /// ```
    #[inline]
    pub fn get_and_decrement(&self) -> i64 {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    /// Returns the current value and atomically adds the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// assert_eq!(counter.get_and_add(5), 10);
    /// assert_eq!(counter.get(), 15);
    /// ```
    #[inline]
    pub fn get_and_add(&self, delta: i64) -> i64 {
        self.value.fetch_add(delta, Ordering::SeqCst)
    }

    /// Atomically adds the given value using fetch_add.
    ///
    /// This is the most common operation and directly maps to the atomic fetch_add.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(10);
    /// counter.fetch_add(5);
    /// assert_eq!(counter.get(), 15);
    /// ```
    #[inline]
    pub fn fetch_add(&self, delta: i64) -> i64 {
        self.value.fetch_add(delta, Ordering::SeqCst)
    }

    /// Prevents compiler optimization from removing padding fields.
    ///
    /// The sum operation ensures fields are not optimized away.
    /// This method should never be called in production code - it exists
    /// solely to keep the compiler from removing the padding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_gds::core::utils::paged::PaddedAtomicLong;
    ///
    /// let counter = PaddedAtomicLong::new(0);
    /// let padding_sum = counter.sum();
    /// assert_eq!(padding_sum, 1 + 2 + 3 + 4 + 5 + 6 + 7); // 28
    /// ```
    #[inline]
    pub fn sum(&self) -> i64 {
        self.p1 + self.p2 + self.p3 + self.p4 + self.p5 + self.p6 + self.p7
    }
}

// Thread safety markers
unsafe impl Send for PaddedAtomicLong {}
unsafe impl Sync for PaddedAtomicLong {}

// Default implementation
impl Default for PaddedAtomicLong {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new() {
        let counter = PaddedAtomicLong::new(42);
        assert_eq!(counter.get(), 42);
    }

    #[test]
    fn test_get_set() {
        let counter = PaddedAtomicLong::new(0);
        assert_eq!(counter.get(), 0);

        counter.set(42);
        assert_eq!(counter.get(), 42);
    }

    #[test]
    fn test_increment() {
        let counter = PaddedAtomicLong::new(0);
        assert_eq!(counter.increment_and_get(), 1);
        assert_eq!(counter.increment_and_get(), 2);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_decrement() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.decrement_and_get(), 9);
        assert_eq!(counter.decrement_and_get(), 8);
        assert_eq!(counter.get(), 8);
    }

    #[test]
    fn test_add_and_get() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.add_and_get(5), 15);
        assert_eq!(counter.add_and_get(-3), 12);
        assert_eq!(counter.get(), 12);
    }

    #[test]
    fn test_get_and_set() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.get_and_set(42), 10);
        assert_eq!(counter.get(), 42);
    }

    #[test]
    fn test_compare_and_set() {
        let counter = PaddedAtomicLong::new(10);

        // Success case
        assert!(counter.compare_and_set(10, 20));
        assert_eq!(counter.get(), 20);

        // Failure case
        assert!(!counter.compare_and_set(10, 30));
        assert_eq!(counter.get(), 20);
    }

    #[test]
    fn test_get_and_increment() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.get_and_increment(), 10);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_get_and_decrement() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.get_and_decrement(), 10);
        assert_eq!(counter.get(), 9);
    }

    #[test]
    fn test_get_and_add() {
        let counter = PaddedAtomicLong::new(10);
        assert_eq!(counter.get_and_add(5), 10);
        assert_eq!(counter.get(), 15);
    }

    #[test]
    fn test_fetch_add() {
        let counter = PaddedAtomicLong::new(10);
        counter.fetch_add(5);
        assert_eq!(counter.get(), 15);
    }

    #[test]
    fn test_concurrent_increments() {
        let counter = Arc::new(PaddedAtomicLong::new(0));
        let mut handles = vec![];

        for _ in 0..4 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..10_000 {
                    counter_clone.fetch_add(1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 40_000);
    }

    #[test]
    fn test_sum_prevents_optimization() {
        let counter = PaddedAtomicLong::new(0);
        let sum = counter.sum();
        assert_eq!(sum, 1 + 2 + 3 + 4 + 5 + 6 + 7); // 28
    }

    #[test]
    fn test_default() {
        let counter = PaddedAtomicLong::default();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_memory_layout() {
        use std::mem;

        // Verify struct size includes padding
        let size = mem::size_of::<PaddedAtomicLong>();
        println!("PaddedAtomicLong size: {} bytes", size);

        // Should be at least 64 bytes (one cache line)
        assert!(size >= 64, "PaddedAtomicLong should be at least 64 bytes");
    }
}
