// Arrow2 Task System
//
// Provides parallel task execution for graph import operations.
//
// Design principles:
// - Task abstraction: ImportTask trait for nodes/edges
// - Parallel execution: TaskRunner orchestrates N tasks across threads
// - Progress tracking: Import statistics per task
// - Resource management: Thread pool, memory limits
// - Error handling: Graceful failure, task-level errors
//
// Architecture:
// - ImportTask: Abstract job interface (translate RecordScannerTask)
// - ImportResult: Task execution results (records, properties, duration)
// - TaskRunner: Parallel executor using Rayon
// - TaskFactory: Creates tasks for parallel execution

use super::scanner::{BatchScanner, ScanCursor};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;

// ================================================================================================
// Error Types
// ================================================================================================

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Task terminated by user")]
    Terminated,

    #[error("Task index out of bounds: {index} (max: {max})")]
    InvalidTaskIndex { index: usize, max: usize },

    #[error("Thread pool error: {message}")]
    ThreadPoolError { message: String },

    #[error("Scanner error: {message}")]
    ScannerError { message: String },
}

// ================================================================================================
// ImportResult - Task execution results
// ================================================================================================

/// Results from executing an import task.
///
/// Tracks records imported, properties imported, and execution time.
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Number of records (nodes or edges) imported
    pub records_imported: u64,

    /// Number of properties imported
    pub properties_imported: u64,

    /// Duration of the import in nanoseconds
    pub duration_nanos: u64,

    /// Task index
    pub task_index: usize,
}

impl ImportResult {
    /// Creates a new import result
    pub fn new(
        records_imported: u64,
        properties_imported: u64,
        duration_nanos: u64,
        task_index: usize,
    ) -> Self {
        Self {
            records_imported,
            properties_imported,
            duration_nanos,
            task_index,
        }
    }

    /// Returns the duration in seconds
    pub fn duration_secs(&self) -> f64 {
        self.duration_nanos as f64 / 1_000_000_000.0
    }

    /// Returns the import rate (records per second)
    pub fn records_per_second(&self) -> f64 {
        if self.duration_nanos == 0 {
            0.0
        } else {
            (self.records_imported as f64 * 1_000_000_000.0) / self.duration_nanos as f64
        }
    }
}

// ================================================================================================
// ImportTask Trait - Abstract task interface
// ================================================================================================

/// Abstract interface for import tasks (nodes or edges).
///
/// Implements the job pattern: receives a scanner cursor and imports data.
/// Each task runs on a separate thread and reports results.
///
/// This is the Rust translation of Java's RecordScannerTask interface.
pub trait ImportTask: Send {
    /// Executes the import task using the provided scanner cursor.
    ///
    /// Returns the number of records and properties imported as (records, properties).
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError>;

    /// Returns a human-readable name for this task (for logging/debugging).
    fn task_name(&self) -> String;

    /// Returns the task index.
    fn task_index(&self) -> usize;
}

// ================================================================================================
// TaskFactory Trait - Creates tasks for parallel execution
// ================================================================================================

/// Factory for creating import tasks.
///
/// Translates Java's RecordScannerTaskFactory pattern.
pub trait TaskFactory: Send + Sync {
    /// Creates a new task for the given task index.
    ///
    /// Called once per thread to create independent task instances.
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError>;
}

// ================================================================================================
// TaskRunner - Parallel task executor
// ================================================================================================

/// Orchestrates parallel execution of import tasks.
///
/// Creates N tasks (one per thread), executes them in parallel using Rayon,
/// and aggregates results.
///
/// This is the Rust translation of Java's RecordScannerTaskRunner.
pub struct TaskRunner {
    thread_count: usize,
    termination_flag: Arc<AtomicBool>,
}

impl TaskRunner {
    /// Creates a new task runner with the specified thread count.
    pub fn new(thread_count: usize) -> Result<Self, TaskError> {
        if thread_count == 0 {
            return Err(TaskError::ThreadPoolError {
                message: "Thread count must be > 0".to_string(),
            });
        }

        Ok(Self {
            thread_count,
            termination_flag: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Executes import tasks in parallel using the provided factory and scanner.
    ///
    /// Creates `thread_count` tasks, each with its own cursor from the scanner.
    /// Tasks run concurrently and independently reserve batches.
    ///
    /// Returns aggregated results from all tasks.
    pub fn run_import<S>(
        &self,
        scanner: Arc<S>,
        factory: Arc<dyn TaskFactory>,
    ) -> Result<AggregatedImportResult, TaskError>
    where
        S: BatchScanner + 'static,
    {
        let start = Instant::now();

        // Create tasks
        let mut tasks: Vec<Box<dyn ImportTask>> = Vec::with_capacity(self.thread_count);
        for i in 0..self.thread_count {
            tasks.push(factory.create_task(i)?);
        }

        // Execute tasks in parallel using Rayon's parallel iterator
        use rayon::prelude::*;

        let results: Result<Vec<ImportResult>, TaskError> = tasks
            .into_par_iter()
            .map(|mut task| {
                // Check termination before starting
                if self.termination_flag.load(Ordering::Relaxed) {
                    return Err(TaskError::Terminated);
                }

                let task_start = Instant::now();
                let task_index = task.task_index();

                // Create cursor for this task
                let mut cursor = scanner.create_cursor();

                // Execute task
                let (records, properties) = task.execute(cursor.as_mut())?;

                let duration_nanos = task_start.elapsed().as_nanos() as u64;

                Ok(ImportResult::new(
                    records,
                    properties,
                    duration_nanos,
                    task_index,
                ))
            })
            .collect();

        let results = results?;
        let total_duration = start.elapsed().as_nanos() as u64;

        // Aggregate results
        Ok(AggregatedImportResult::from_results(
            results,
            total_duration,
        ))
    }

    /// Signals termination to all running tasks.
    pub fn terminate(&self) {
        self.termination_flag.store(true, Ordering::Relaxed);
    }

    /// Returns the configured thread count.
    pub fn thread_count(&self) -> usize {
        self.thread_count
    }
}

// ================================================================================================
// AggregatedImportResult - Combined results from all tasks
// ================================================================================================

/// Aggregated results from all import tasks.
#[derive(Debug, Clone)]
pub struct AggregatedImportResult {
    /// Total records imported across all tasks
    pub total_records_imported: u64,

    /// Total properties imported across all tasks
    pub total_properties_imported: u64,

    /// Total duration in nanoseconds
    pub total_duration_nanos: u64,

    /// Number of tasks that completed
    pub tasks_completed: usize,

    /// Individual task results
    pub task_results: Vec<ImportResult>,
}

impl AggregatedImportResult {
    /// Creates aggregated results from individual task results.
    pub fn from_results(task_results: Vec<ImportResult>, total_duration_nanos: u64) -> Self {
        let total_records_imported: u64 = task_results.iter().map(|r| r.records_imported).sum();
        let total_properties_imported: u64 =
            task_results.iter().map(|r| r.properties_imported).sum();
        let tasks_completed = task_results.len();

        Self {
            total_records_imported,
            total_properties_imported,
            total_duration_nanos,
            tasks_completed,
            task_results,
        }
    }

    /// Returns the total duration in seconds.
    pub fn duration_secs(&self) -> f64 {
        self.total_duration_nanos as f64 / 1_000_000_000.0
    }

    /// Returns the overall import rate (records per second).
    pub fn records_per_second(&self) -> f64 {
        if self.total_duration_nanos == 0 {
            0.0
        } else {
            (self.total_records_imported as f64 * 1_000_000_000.0)
                / self.total_duration_nanos as f64
        }
    }

    /// Returns the average records per task.
    pub fn avg_records_per_task(&self) -> f64 {
        if self.tasks_completed == 0 {
            0.0
        } else {
            self.total_records_imported as f64 / self.tasks_completed as f64
        }
    }

    /// Returns the throughput efficiency (actual vs theoretical).
    ///
    /// Perfect parallelism would be 1.0, lower values indicate overhead.
    pub fn parallelism_efficiency(&self) -> f64 {
        if self.task_results.is_empty() {
            return 0.0;
        }

        let total_task_time: u64 = self.task_results.iter().map(|r| r.duration_nanos).sum();
        let wall_clock_time = self.total_duration_nanos;

        if wall_clock_time == 0 {
            return 0.0;
        }

        // Ideal parallelism: total_task_time / (wall_clock_time * num_tasks)
        // Efficiency: (total_task_time / num_tasks) / wall_clock_time
        let avg_task_time = total_task_time as f64 / self.tasks_completed as f64;
        avg_task_time / wall_clock_time as f64
    }
}

// ================================================================================================
// Progress Tracking
// ================================================================================================

/// Progress tracker for long-running imports.
///
/// Thread-safe atomic counters for tracking progress across tasks.
#[derive(Debug)]
pub struct ProgressTracker {
    records_processed: Arc<AtomicU64>,
    properties_processed: Arc<AtomicU64>,
    batches_processed: Arc<AtomicU64>,
}

impl ProgressTracker {
    /// Creates a new progress tracker.
    pub fn new() -> Self {
        Self {
            records_processed: Arc::new(AtomicU64::new(0)),
            properties_processed: Arc::new(AtomicU64::new(0)),
            batches_processed: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Records progress for a batch.
    pub fn record_batch(&self, records: u64, properties: u64) {
        self.records_processed.fetch_add(records, Ordering::Relaxed);
        self.properties_processed
            .fetch_add(properties, Ordering::Relaxed);
        self.batches_processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the current number of records processed.
    pub fn records_processed(&self) -> u64 {
        self.records_processed.load(Ordering::Relaxed)
    }

    /// Returns the current number of properties processed.
    pub fn properties_processed(&self) -> u64 {
        self.properties_processed.load(Ordering::Relaxed)
    }

    /// Returns the current number of batches processed.
    pub fn batches_processed(&self) -> u64 {
        self.batches_processed.load(Ordering::Relaxed)
    }

    /// Resets all progress counters.
    pub fn reset(&self) {
        self.records_processed.store(0, Ordering::Relaxed);
        self.properties_processed.store(0, Ordering::Relaxed);
        self.batches_processed.store(0, Ordering::Relaxed);
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ProgressTracker {
    fn clone(&self) -> Self {
        Self {
            records_processed: Arc::clone(&self.records_processed),
            properties_processed: Arc::clone(&self.properties_processed),
            batches_processed: Arc::clone(&self.batches_processed),
        }
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::factory::arrow::{
        ArrowBatchReference, NodeBatchScanner, NodeTableReference,
    };
    use arrow2::array::{Array, Int64Array, Utf8Array};
    use arrow2::chunk::Chunk;
    use arrow2::datatypes::{DataType, Field, Schema};

    // Mock task that counts batches
    struct MockImportTask {
        task_index: usize,
        batches_to_process: usize,
    }

    impl ImportTask for MockImportTask {
        fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError> {
            let mut records = 0u64;
            let mut batches = 0usize;

            while cursor.reserve_batch() && batches < self.batches_to_process {
                cursor.consume_batch(&mut |batch: &ArrowBatchReference| {
                    records += batch.len() as u64;
                    true
                });
                batches += 1;
            }

            Ok((records, 0)) // No properties for mock
        }

        fn task_name(&self) -> String {
            format!("mock-task-{}", self.task_index)
        }

        fn task_index(&self) -> usize {
            self.task_index
        }
    }

    struct MockTaskFactory {
        batches_per_task: usize,
    }

    impl TaskFactory for MockTaskFactory {
        fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError> {
            Ok(Box::new(MockImportTask {
                task_index,
                batches_to_process: self.batches_per_task,
            }))
        }
    }

    fn create_test_scanner(row_count: usize, batch_size: usize) -> Arc<NodeBatchScanner> {
        let ids: Vec<i64> = (1..=row_count as i64).collect();
        let labels: Vec<Option<&str>> = (0..row_count).map(|_| Some("Person")).collect();

        let id_array: Box<dyn Array> = Box::new(Int64Array::from_vec(ids));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from(labels));

        let chunk = Chunk::new(vec![id_array, label_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
        ]));

        let node_table = Arc::new(NodeTableReference::new("nodes", chunk, schema).unwrap());
        Arc::new(NodeBatchScanner::new(node_table, batch_size).unwrap())
    }

    #[test]
    fn test_import_result_creation() {
        let result = ImportResult::new(1000, 500, 1_000_000_000, 0);

        assert_eq!(result.records_imported, 1000);
        assert_eq!(result.properties_imported, 500);
        assert_eq!(result.duration_nanos, 1_000_000_000);
        assert_eq!(result.duration_secs(), 1.0);
        assert_eq!(result.records_per_second(), 1000.0);
    }

    #[test]
    fn test_task_runner_creation() {
        let runner = TaskRunner::new(4).unwrap();
        assert_eq!(runner.thread_count(), 4);
    }

    #[test]
    fn test_task_runner_invalid_thread_count() {
        let result = TaskRunner::new(0);
        assert!(matches!(result, Err(TaskError::ThreadPoolError { .. })));
    }

    #[test]
    fn test_task_runner_single_task() {
        let scanner = create_test_scanner(100, 25); // 100 rows, 25 per batch = 4 batches
        let factory = Arc::new(MockTaskFactory {
            batches_per_task: 10, // Process up to 10 batches
        });

        let runner = TaskRunner::new(1).unwrap();
        let result = runner.run_import(scanner, factory).unwrap();

        assert_eq!(result.tasks_completed, 1);
        assert_eq!(result.total_records_imported, 100); // All 100 rows processed
    }

    #[test]
    fn test_task_runner_parallel_tasks() {
        let scanner = create_test_scanner(1000, 100); // 1000 rows, 100 per batch = 10 batches
        let factory = Arc::new(MockTaskFactory {
            batches_per_task: 10,
        });

        let runner = TaskRunner::new(4).unwrap();
        let result = runner.run_import(scanner, factory).unwrap();

        assert_eq!(result.tasks_completed, 4);
        // Each task should process some batches (total = 1000 rows)
        assert_eq!(result.total_records_imported, 1000);
    }

    #[test]
    fn test_aggregated_result_calculations() {
        let task_results = vec![
            ImportResult::new(100, 50, 1_000_000_000, 0),
            ImportResult::new(150, 75, 1_500_000_000, 1),
            ImportResult::new(200, 100, 2_000_000_000, 2),
        ];

        let aggregated = AggregatedImportResult::from_results(task_results, 2_000_000_000);

        assert_eq!(aggregated.total_records_imported, 450);
        assert_eq!(aggregated.total_properties_imported, 225);
        assert_eq!(aggregated.tasks_completed, 3);
        assert_eq!(aggregated.avg_records_per_task(), 150.0);
    }

    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new();

        assert_eq!(tracker.records_processed(), 0);
        assert_eq!(tracker.properties_processed(), 0);
        assert_eq!(tracker.batches_processed(), 0);

        tracker.record_batch(100, 50);
        tracker.record_batch(150, 75);

        assert_eq!(tracker.records_processed(), 250);
        assert_eq!(tracker.properties_processed(), 125);
        assert_eq!(tracker.batches_processed(), 2);

        tracker.reset();
        assert_eq!(tracker.records_processed(), 0);
    }

    #[test]
    fn test_progress_tracker_clone() {
        let tracker = ProgressTracker::new();
        tracker.record_batch(100, 50);

        let cloned = tracker.clone();
        assert_eq!(cloned.records_processed(), 100);

        // Both should see the same underlying counters
        tracker.record_batch(50, 25);
        assert_eq!(cloned.records_processed(), 150);
    }
}
