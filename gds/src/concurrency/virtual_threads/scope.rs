//! Synchronization scope for Pregel-style supersteps.
//!
//! A Scope represents a synchronization boundary where all spawned work must
//! complete before the scope ends. This is perfect for iterative algorithms
//! like Pregel where each iteration (superstep) must finish before the next begins.

use crate::concurrency::{BatchSize, Concurrency};
use crate::concurrency::TerminationFlag;
use rayon::prelude::*;

/// Synchronization scope for parallel work.
///
/// This provides a boundary where all spawned work must complete before
/// proceeding. Perfect for Pregel supersteps or any iterative algorithm.
///
/// # Synchronization Guarantee
///
/// When a Scope method returns, ALL spawned work has completed. This is
/// enforced by Rayon's scoped threads - the scope cannot end while any
/// spawned work is still running.
///
/// # Examples
///
/// ```
/// use gds::concurrency::virtual_threads::{Executor, Scope};
/// use gds::concurrency::Concurrency;
/// use gds::termination::TerminationFlag;
///
/// let executor = Executor::new(Concurrency::of(4));
/// let termination = TerminationFlag::running_true();
///
/// executor.scope(&termination, |scope| {
///     // All this work completes before scope ends
///     scope.spawn_many(1000, |node_id| {
///         // Process node
///     });
///     // Implicit synchronization barrier here
/// });
/// ```
pub struct Scope<'a> {
    concurrency: Concurrency,
    termination: &'a TerminationFlag,
}

impl<'a> Scope<'a> {
    /// Create a new scope (internal use only).
    pub(super) fn new(concurrency: Concurrency, termination: &'a TerminationFlag) -> Self {
        Self {
            concurrency,
            termination,
        }
    }

    /// Spawn multiple parallel tasks over a range.
    ///
    /// This is the primary method for spawning work in a scope. All tasks
    /// will complete before this method returns.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of tasks to spawn (0..count)
    /// * `task` - Function to execute for each index
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.scope(&termination, |scope| {
    ///     scope.spawn_many(100, |i| {
    ///         println!("Task {}", i);
    ///     });
    ///     // All 100 tasks complete here
    /// });
    /// ```
    pub fn spawn_many<F>(&self, count: usize, task: F)
    where
        F: Fn(usize) + Send + Sync,
    {
        if count == 0 || !self.termination.running() {
            return;
        }

        let batch_size = BatchSize::for_parallel_work(count, self.concurrency);

        (0..count)
            .into_par_iter()
            .with_max_len(batch_size.value())
            .try_for_each(|i| {
                if !self.termination.running() {
                    return Err(());
                }
                task(i);
                Ok(())
            })
            .ok(); // Ignore termination errors
    }

    /// Spawn tasks over a range with custom start and end.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::Executor;
    /// use gds::concurrency::Concurrency;
    /// use gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.scope(&termination, |scope| {
    ///     scope.spawn_range(10, 20, |i| {
    ///         println!("Task {}", i);
    ///     });
    /// });
    /// ```
    pub fn spawn_range<F>(&self, start: usize, end: usize, task: F)
    where
        F: Fn(usize) + Send + Sync,
    {
        if start >= end || !self.termination.running() {
            return;
        }

        self.spawn_many(end - start, |i| task(start + i));
    }

    /// Spawn a single task (rarely needed, but available).
    ///
    /// Note: For spawning multiple tasks, use `spawn_many` which is more efficient.
    pub fn spawn<F>(&self, task: F)
    where
        F: FnOnce() + Send,
    {
        if !self.termination.running() {
            return;
        }

        task();
    }

    /// Get the concurrency level for this scope.
    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    /// Check if execution should continue.
    pub fn should_continue(&self) -> bool {
        self.termination.running()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::virtual_threads::Executor;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_scope_spawn_many() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .scope(&termination, |scope| {
                scope.spawn_many(100, |_| {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_scope_spawn_many_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .scope(&termination, |scope| {
                scope.spawn_many(1000, |_| {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .ok();

        // Should terminate early
        assert!(counter.load(Ordering::Relaxed) < 1000);
    }

    #[test]
    fn test_scope_spawn_range() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let sum = Arc::new(AtomicUsize::new(0));

        let sum_clone = Arc::clone(&sum);
        executor
            .scope(&termination, |scope| {
                scope.spawn_range(10, 20, |i| {
                    sum_clone.fetch_add(i, Ordering::Relaxed);
                });
            })
            .unwrap();

        // Sum of 10..20 = 145
        assert_eq!(sum.load(Ordering::Relaxed), 145);
    }

    #[test]
    fn test_scope_spawn_range_empty() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .scope(&termination, |scope| {
                scope.spawn_range(10, 10, |_| {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_scope_spawn_single() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let executed = Arc::new(AtomicUsize::new(0));

        let executed_clone = Arc::clone(&executed);
        executor
            .scope(&termination, |scope| {
                scope.spawn(move || {
                    executed_clone.store(42, Ordering::Relaxed);
                });
            })
            .unwrap();

        assert_eq!(executed.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_scope_should_continue() {
        let executor = Executor::new(Concurrency::of(4));
        let termination_running = TerminationFlag::running_true();
        let termination_stopped = TerminationFlag::stop_running();

        executor
            .scope(&termination_running, |scope| {
                assert!(scope.should_continue());
            })
            .unwrap();

        executor
            .scope(&termination_stopped, |scope| {
                assert!(!scope.should_continue());
            })
            .ok();
    }

    #[test]
    fn test_scope_synchronization_barrier() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let phase1_complete = Arc::new(AtomicUsize::new(0));
        let phase2_counter = Arc::new(AtomicUsize::new(0));

        // Phase 1
        let phase1_clone = Arc::clone(&phase1_complete);
        executor
            .scope(&termination, |scope| {
                scope.spawn_many(10, |_| {
                    phase1_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .unwrap();

        // Synchronization point - all phase 1 work is complete
        assert_eq!(phase1_complete.load(Ordering::Relaxed), 10);

        // Phase 2 - uses result from phase 1
        let phase2_clone = Arc::clone(&phase2_counter);
        let phase1_result = phase1_complete.load(Ordering::Relaxed);
        executor
            .scope(&termination, |scope| {
                scope.spawn_many(phase1_result, |_| {
                    phase2_clone.fetch_add(1, Ordering::Relaxed);
                });
            })
            .unwrap();

        assert_eq!(phase2_counter.load(Ordering::Relaxed), 10);
    }
}
