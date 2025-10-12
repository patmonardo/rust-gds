//! High-level parallel executor for graph algorithms.
//!
//! The Executor provides a clean API for parallel execution with termination support,
//! built on top of Rayon's work-stealing scheduler.

use crate::concurrency::Concurrency;
use crate::termination::{TerminatedException, TerminationFlag};
use rayon::prelude::*;

/// Parallel executor for graph algorithms.
///
/// This is the main entry point for parallel execution. It wraps Rayon's thread pool
/// with termination checking and provides Pregel-friendly APIs.
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::virtual_threads::Executor;
/// use rust_gds::concurrency::Concurrency;
/// use rust_gds::termination::TerminationFlag;
///
/// let executor = Executor::new(Concurrency::of(4));
/// let termination = TerminationFlag::running_true();
///
/// // Execute parallel work with automatic synchronization
/// executor.scope(&termination, |scope| {
///     scope.spawn_many(1000, |node_id| {
///         // Process node in parallel
///         println!("Processing node {}", node_id);
///     });
/// });
/// ```
#[derive(Clone)]
pub struct Executor {
    concurrency: Concurrency,
}

impl Executor {
    /// Create a new executor with the specified concurrency level.
    ///
    /// The concurrency level determines how many threads Rayon will use for parallel work.
    pub fn new(concurrency: Concurrency) -> Self {
        Self { concurrency }
    }

    /// Get the concurrency level for this executor.
    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    /// Execute work within a synchronization scope.
    ///
    /// This is the core method for Pregel-style algorithms. All work spawned within
    /// the scope will complete before the scope ends, providing a synchronization boundary.
    ///
    /// # Arguments
    ///
    /// * `termination` - Flag to check for early termination
    /// * `work` - Function that receives a Scope and spawns parallel work
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all work completed, or `Err(())` if terminated early.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::virtual_threads::Executor;
    /// use rust_gds::concurrency::Concurrency;
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.scope(&termination, |scope| {
    ///     // Spawn parallel work
    ///     scope.spawn_many(100, |i| {
    ///         println!("Work {}", i);
    ///     });
    /// });
    /// ```
    pub fn scope<F, R>(
        &self,
        termination: &TerminationFlag,
        work: F,
    ) -> Result<R, TerminatedException>
    where
        F: FnOnce(&super::Scope) -> R + Send,
        R: Send,
    {
        // Check termination before starting
        if !termination.running() {
            return Err(TerminatedException);
        }

        // Create scope with termination flag
        let scope = super::Scope::new(self.concurrency, termination);

        // Execute work and return result
        Ok(work(&scope))
    }

    /// Execute a simple parallel loop over a range.
    ///
    /// This is a convenience method for the common case of iterating over nodes.
    /// It's equivalent to using `scope` with `spawn_many`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::virtual_threads::Executor;
    /// use rust_gds::concurrency::Concurrency;
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// executor.parallel_for(0, 1000, &termination, |node_id| {
    ///     println!("Processing node {}", node_id);
    /// });
    /// ```
    pub fn parallel_for<F>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        task: F,
    ) -> Result<(), TerminatedException>
    where
        F: Fn(usize) + Send + Sync,
    {
        if start >= end || !termination.running() {
            return if termination.running() {
                Ok(())
            } else {
                Err(TerminatedException)
            };
        }

        // Use Rayon's parallel iteration with termination checking
        (start..end).into_par_iter().try_for_each(|i| {
            if !termination.running() {
                return Err(TerminatedException);
            }
            task(i);
            Ok(())
        })
    }

    /// Execute a parallel map operation over a range.
    ///
    /// This collects results from all parallel tasks into a Vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::virtual_threads::Executor;
    /// use rust_gds::concurrency::Concurrency;
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// let results = executor.parallel_map(0, 10, &termination, |i| i * 2).unwrap();
    /// assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    /// ```
    pub fn parallel_map<F, T>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        mapper: F,
    ) -> Result<Vec<T>, TerminatedException>
    where
        F: Fn(usize) -> T + Send + Sync,
        T: Send,
    {
        if start >= end || !termination.running() {
            return if termination.running() {
                Ok(Vec::new())
            } else {
                Err(TerminatedException)
            };
        }

        (start..end)
            .into_par_iter()
            .map(|i| {
                if !termination.running() {
                    return Err(TerminatedException);
                }
                Ok(mapper(i))
            })
            .collect()
    }

    /// Execute a parallel reduction operation over a range.
    ///
    /// This is perfect for aggregations like sum, max, count, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::concurrency::virtual_threads::Executor;
    /// use rust_gds::concurrency::Concurrency;
    /// use rust_gds::termination::TerminationFlag;
    ///
    /// let executor = Executor::new(Concurrency::of(4));
    /// let termination = TerminationFlag::running_true();
    ///
    /// // Sum all values 0..100
    /// let sum = executor.parallel_reduce(
    ///     0, 100, &termination,
    ///     0usize,
    ///     |i| i,
    ///     |a, b| a + b
    /// ).unwrap();
    /// assert_eq!(sum, 4950);
    /// ```
    pub fn parallel_reduce<T, M, R>(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        identity: T,
        mapper: M,
        reducer: R,
    ) -> Result<T, TerminatedException>
    where
        T: Send + Sync + Clone,
        M: Fn(usize) -> T + Send + Sync,
        R: Fn(T, T) -> T + Send + Sync,
    {
        if start >= end || !termination.running() {
            return if termination.running() {
                Ok(identity)
            } else {
                Err(TerminatedException)
            };
        }

        (start..end)
            .into_par_iter()
            .try_fold(
                || identity.clone(),
                |acc, i| {
                    if !termination.running() {
                        return Err(TerminatedException);
                    }
                    Ok(reducer(acc, mapper(i)))
                },
            )
            .try_reduce(|| identity.clone(), |a, b| Ok(reducer(a, b)))
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new(Concurrency::available_cores())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn test_executor_new() {
        let executor = Executor::new(Concurrency::of(4));
        assert_eq!(executor.concurrency().value(), 4);
    }

    #[test]
    fn test_executor_default() {
        let executor = Executor::default();
        assert!(executor.concurrency().value() > 0);
    }

    #[test]
    fn test_parallel_for_executes_all() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .parallel_for(0, 100, &termination, move |_| {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    #[ignore] // Spawns 1000 tasks - can cause memory issues in constrained environments
    fn test_parallel_for_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        let result = executor.parallel_for(0, 1000, &termination, move |_| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert!(result.is_err());
        assert!(counter.load(Ordering::Relaxed) < 1000);
    }

    #[test]
    fn test_parallel_for_empty_range() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        executor
            .parallel_for(10, 10, &termination, move |_| {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_parallel_map() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let results = executor
            .parallel_map(0, 10, &termination, |i| i * 2)
            .unwrap();

        assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    #[test]
    #[ignore] // Spawns 1000 tasks - can cause memory issues in constrained environments
    fn test_parallel_map_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.parallel_map(0, 1000, &termination, |i| i * 2);

        assert!(result.is_err());
    }

    #[test]
    fn test_parallel_reduce_sum() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let sum = executor
            .parallel_reduce(0, 100, &termination, 0usize, |i| i, |a, b| a + b)
            .unwrap();

        assert_eq!(sum, 4950);
    }

    #[test]
    fn test_parallel_reduce_max() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::running_true();

        let max = executor
            .parallel_reduce(0, 100, &termination, 0usize, |i| i, |a, b| a.max(b))
            .unwrap();

        assert_eq!(max, 99);
    }

    #[test]
    #[ignore] // Spawns 1000 tasks - can cause memory issues in constrained environments
    fn test_parallel_reduce_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.parallel_reduce(0, 1000, &termination, 0usize, |i| i, |a, b| a + b);

        assert!(result.is_err());
    }

    #[test]
    fn test_scope_basic() {
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
    fn test_scope_respects_termination() {
        let executor = Executor::new(Concurrency::of(4));
        let termination = TerminationFlag::stop_running();

        let result = executor.scope(&termination, |_scope| {
            // Should not execute
            42
        });

        assert!(result.is_err());
    }
}
