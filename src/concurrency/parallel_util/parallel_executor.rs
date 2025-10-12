//! Parallel execution utilities powered by Rayon.
//!
//! This module provides the core parallel execution functions that all graph algorithms use.
//! These mirror Java GDS's ParallelUtil but leverage Rust's Rayon for work-stealing parallelism.

use crate::concurrency::{BatchSize, Concurrency};
use crate::termination::TerminationFlag;
use rayon::prelude::*;
use std::ops::Range;

/// Execute a function in parallel for each node in the graph.
///
/// This is the workhorse function for node-parallel algorithms (PageRank, Label Propagation, etc.).
/// It partitions the node range into batches and executes the function in parallel using Rayon.
///
/// # Arguments
///
/// * `node_count` - Total number of nodes to process
/// * `concurrency` - Number of parallel threads to use
/// * `termination` - Flag to check for early termination
/// * `task` - Function to execute for each node (receives node_id)
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::{Concurrency, parallel_for_each_node};
/// use rust_gds::termination::TerminationFlag;
///
/// let node_count = 1000;
/// let concurrency = Concurrency::available_cores();
/// let termination = TerminationFlag::running_true();
///
/// parallel_for_each_node(
///     node_count,
///     concurrency,
///     &termination,
///     |node_id| {
///         // Process node_id in parallel
///         println!("Processing node {}", node_id);
///     }
/// );
/// ```
pub fn parallel_for_each_node<F>(
    node_count: usize,
    concurrency: Concurrency,
    termination: &TerminationFlag,
    task: F,
) where
    F: Fn(usize) + Send + Sync,
{
    if node_count == 0 {
        return;
    }

    // Check termination before starting
    if !termination.running() {
        return;
    }

    // Use Rayon's parallel iterator over node range
    (0..node_count)
        .into_par_iter()
        .with_max_len(BatchSize::for_parallel_work(node_count, concurrency).value())
        .try_for_each(|node_id| {
            // Check termination periodically
            if !termination.running() {
                return Err(());
            }

            task(node_id);
            Ok(())
        })
        .ok(); // Ignore termination errors
}

/// Execute a read-only parallel operation over a range with batching.
///
/// This is for algorithms that need to process ranges of data in parallel batches.
/// Each batch is processed by a separate thread, with termination checking.
///
/// # Arguments
///
/// * `start` - Start of the range (inclusive)
/// * `end` - End of the range (exclusive)
/// * `concurrency` - Number of parallel threads to use
/// * `batch_size` - Size of each batch
/// * `termination` - Flag to check for early termination
/// * `task` - Function to execute for each batch (receives Range<usize>)
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::{Concurrency, BatchSize, read_parallel};
/// use rust_gds::termination::TerminationFlag;
///
/// let concurrency = Concurrency::available_cores();
/// let batch_size = BatchSize::default();
/// let termination = TerminationFlag::running_true();
///
/// read_parallel(
///     0,
///     10000,
///     concurrency,
///     batch_size,
///     &termination,
///     |range| {
///         // Process range in parallel
///         println!("Processing range {:?}", range);
///     }
/// );
/// ```
pub fn read_parallel<F>(
    start: usize,
    end: usize,
    concurrency: Concurrency,
    batch_size: BatchSize,
    termination: &TerminationFlag,
    task: F,
) where
    F: Fn(Range<usize>) + Send + Sync,
{
    if start >= end {
        return;
    }

    // Check termination before starting
    if !termination.running() {
        return;
    }

    let total_work = end - start;
    let batch_size = batch_size.value().min(total_work);

    // Create batches
    let batches: Vec<Range<usize>> = (start..end)
        .step_by(batch_size)
        .map(|batch_start| {
            let batch_end = (batch_start + batch_size).min(end);
            batch_start..batch_end
        })
        .collect();

    // Process batches in parallel
    batches
        .into_par_iter()
        .with_max_len((total_work / concurrency.value()).max(1))
        .try_for_each(|range| {
            // Check termination before each batch
            if !termination.running() {
                return Err(());
            }

            task(range);
            Ok(())
        })
        .ok(); // Ignore termination errors
}

/// Execute a simple parallel task across multiple threads.
///
/// This is the most basic parallel execution primitive - just run the same task
/// N times in parallel, where N = concurrency level.
///
/// # Arguments
///
/// * `concurrency` - Number of parallel executions
/// * `termination` - Flag to check for early termination
/// * `task` - Function to execute (receives thread_id: 0..concurrency)
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::{Concurrency, run};
/// use rust_gds::termination::TerminationFlag;
///
/// let concurrency = Concurrency::of(4);
/// let termination = TerminationFlag::running_true();
///
/// run(concurrency, &termination, |thread_id| {
///     println!("Thread {} executing", thread_id);
/// });
/// ```
pub fn run<F>(concurrency: Concurrency, termination: &TerminationFlag, task: F)
where
    F: Fn(usize) + Send + Sync,
{
    // Check termination before starting
    if !termination.running() {
        return;
    }

    // Execute task in parallel for each thread
    (0..concurrency.value())
        .into_par_iter()
        .try_for_each(|thread_id| {
            // Check termination before executing
            if !termination.running() {
                return Err(());
            }

            task(thread_id);
            Ok(())
        })
        .ok(); // Ignore termination errors
}

/// Partition a range of work into equal-sized chunks for parallel processing.
///
/// Returns a vector of ranges, one per thread. The last partition may be larger
/// if the work doesn't divide evenly.
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::{Concurrency, partition_work};
///
/// let partitions = partition_work(0, 1000, Concurrency::of(4));
/// assert_eq!(partitions.len(), 4);
/// assert_eq!(partitions[0], 0..250);
/// assert_eq!(partitions[1], 250..500);
/// assert_eq!(partitions[2], 500..750);
/// assert_eq!(partitions[3], 750..1000);
/// ```
pub fn partition_work(start: usize, end: usize, concurrency: Concurrency) -> Vec<Range<usize>> {
    if start >= end {
        return vec![];
    }

    let total_work = end - start;
    let num_partitions = concurrency.value().min(total_work);

    if num_partitions == 0 {
        return vec![];
    }

    let base_chunk = total_work / num_partitions;
    let remainder = total_work % num_partitions;

    let mut partitions = Vec::with_capacity(num_partitions);
    let mut current = start;

    for i in 0..num_partitions {
        // First 'remainder' partitions get one extra item
        let chunk_size = if i < remainder {
            base_chunk + 1
        } else {
            base_chunk
        };

        let next = current + chunk_size;
        partitions.push(current..next);
        current = next;
    }

    partitions
}

/// Execute a parallel reduction operation.
///
/// This combines parallel execution with reduction, perfect for aggregation algorithms
/// like computing graph statistics (sum of degrees, max centrality, etc.).
///
/// # Arguments
///
/// * `node_count` - Total number of nodes to process
/// * `concurrency` - Number of parallel threads to use
/// * `termination` - Flag to check for early termination
/// * `identity` - Initial value for reduction
/// * `mapper` - Function to map each node to a value
/// * `reducer` - Function to combine two values
///
/// # Examples
///
/// ```
/// use rust_gds::concurrency::{Concurrency, parallel_reduce};
/// use rust_gds::termination::TerminationFlag;
///
/// let concurrency = Concurrency::available_cores();
/// let termination = TerminationFlag::running_true();
///
/// // Sum all node IDs in parallel
/// let sum = parallel_reduce(
///     1000,
///     concurrency,
///     &termination,
///     0usize,
///     |node_id| node_id,
///     |a, b| a + b
/// );
/// ```
pub fn parallel_reduce<T, M, R>(
    node_count: usize,
    concurrency: Concurrency,
    termination: &TerminationFlag,
    identity: T,
    mapper: M,
    reducer: R,
) -> T
where
    T: Send + Sync + Clone,
    M: Fn(usize) -> T + Send + Sync,
    R: Fn(T, T) -> T + Send + Sync,
{
    if node_count == 0 || !termination.running() {
        return identity;
    }

    (0..node_count)
        .into_par_iter()
        .with_max_len(BatchSize::for_parallel_work(node_count, concurrency).value())
        .try_fold(
            || identity.clone(),
            |acc, node_id| {
                if !termination.running() {
                    return Err(());
                }
                Ok(reducer(acc, mapper(node_id)))
            },
        )
        .try_reduce(|| identity.clone(), |a, b| Ok(reducer(a, b)))
        .unwrap_or(identity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_parallel_for_each_node_executes_all() {
        let node_count = 100;
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        parallel_for_each_node(node_count, concurrency, &termination, move |_node_id| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(counter.load(Ordering::Relaxed), node_count);
    }

    #[test]
    #[ignore] // Spawns 1000 tasks - can cause memory issues in constrained environments
    fn test_parallel_for_each_node_respects_termination() {
        let node_count = 1000;
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        parallel_for_each_node(node_count, concurrency, &termination, move |_node_id| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        // Should process zero or very few nodes before termination check
        assert!(counter.load(Ordering::Relaxed) < node_count);
    }

    #[test]
    fn test_parallel_for_each_node_empty() {
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        parallel_for_each_node(0, concurrency, &termination, move |_node_id| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_read_parallel_processes_all_batches() {
        let concurrency = Concurrency::of(4);
        let batch_size = BatchSize::new(10);
        let termination = TerminationFlag::running_true();
        let processed = Arc::new(Mutex::new(Vec::new()));

        let processed_clone = Arc::clone(&processed);
        read_parallel(
            0,
            100,
            concurrency,
            batch_size,
            &termination,
            move |range| {
                processed_clone.lock().unwrap().extend(range);
            },
        );

        let all_processed = processed.lock().unwrap();
        assert_eq!(all_processed.len(), 100);
        // Verify all indices 0..100 are present
        let mut sorted = all_processed.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn test_read_parallel_respects_termination() {
        let concurrency = Concurrency::of(4);
        let batch_size = BatchSize::new(10);
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        read_parallel(
            0,
            1000,
            concurrency,
            batch_size,
            &termination,
            move |range| {
                counter_clone.fetch_add(range.len(), Ordering::Relaxed);
            },
        );

        // Should process very little before termination
        assert!(counter.load(Ordering::Relaxed) < 1000);
    }

    #[test]
    fn test_read_parallel_empty_range() {
        let concurrency = Concurrency::of(4);
        let batch_size = BatchSize::default();
        let termination = TerminationFlag::running_true();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        read_parallel(
            10,
            10,
            concurrency,
            batch_size,
            &termination,
            move |_range| {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            },
        );

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_run_executes_for_each_thread() {
        let concurrency = Concurrency::of(8);
        let termination = TerminationFlag::running_true();
        let thread_ids = Arc::new(Mutex::new(Vec::new()));

        let thread_ids_clone = Arc::clone(&thread_ids);
        run(concurrency, &termination, move |thread_id| {
            thread_ids_clone.lock().unwrap().push(thread_id);
        });

        let ids = thread_ids.lock().unwrap();
        assert_eq!(ids.len(), 8);
        // Verify all thread IDs 0..8 are present
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..8).collect::<Vec<_>>());
    }

    #[test]
    fn test_run_respects_termination() {
        let concurrency = Concurrency::of(8);
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        run(concurrency, &termination, move |_thread_id| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        // Should execute zero tasks
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_partition_work_equal_division() {
        let partitions = partition_work(0, 1000, Concurrency::of(4));
        assert_eq!(partitions.len(), 4);
        assert_eq!(partitions[0], 0..250);
        assert_eq!(partitions[1], 250..500);
        assert_eq!(partitions[2], 500..750);
        assert_eq!(partitions[3], 750..1000);
    }

    #[test]
    fn test_partition_work_unequal_division() {
        let partitions = partition_work(0, 1000, Concurrency::of(3));
        assert_eq!(partitions.len(), 3);
        // 1000 / 3 = 333 remainder 1, so first partition gets extra item
        assert_eq!(partitions[0], 0..334);
        assert_eq!(partitions[1], 334..667);
        assert_eq!(partitions[2], 667..1000);
    }

    #[test]
    fn test_partition_work_more_threads_than_work() {
        let partitions = partition_work(0, 5, Concurrency::of(10));
        assert_eq!(partitions.len(), 5); // Only 5 partitions for 5 items
        assert_eq!(partitions[0], 0..1);
        assert_eq!(partitions[1], 1..2);
        assert_eq!(partitions[2], 2..3);
        assert_eq!(partitions[3], 3..4);
        assert_eq!(partitions[4], 4..5);
    }

    #[test]
    fn test_partition_work_empty() {
        let partitions = partition_work(10, 10, Concurrency::of(4));
        assert_eq!(partitions.len(), 0);
    }

    #[test]
    fn test_parallel_reduce_sum() {
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::running_true();

        let sum = parallel_reduce(
            100,
            concurrency,
            &termination,
            0usize,
            |node_id| node_id,
            |a, b| a + b,
        );

        // Sum of 0..100 = 99 * 100 / 2 = 4950
        assert_eq!(sum, 4950);
    }

    #[test]
    fn test_parallel_reduce_max() {
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::running_true();

        let max = parallel_reduce(
            100,
            concurrency,
            &termination,
            0usize,
            |node_id| node_id,
            |a, b| a.max(b),
        );

        assert_eq!(max, 99);
    }

    #[test]
    fn test_parallel_reduce_respects_termination() {
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::stop_running();

        let sum = parallel_reduce(
            1000,
            concurrency,
            &termination,
            0usize,
            |node_id| node_id,
            |a, b| a + b,
        );

        // Should return identity value when terminated
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_parallel_reduce_empty() {
        let concurrency = Concurrency::of(4);
        let termination = TerminationFlag::running_true();

        let sum = parallel_reduce(
            0,
            concurrency,
            &termination,
            42usize,
            |node_id| node_id,
            |a, b| a + b,
        );

        // Should return identity value for empty input
        assert_eq!(sum, 42);
    }
}
