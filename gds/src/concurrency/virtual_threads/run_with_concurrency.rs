//! High-level task execution builder pattern.
//!
//! This provides a flexible builder API for running collections of tasks
//! with configurable concurrency, retry logic, and termination support.
//!
//! This mirrors Java GDS's RunWithConcurrency but with Rust's simplicity.

use crate::concurrency::{virtual_threads::Executor, Concurrency};
use crate::concurrency::TerminationFlag;

/// Builder for flexible task execution with concurrency control.
///
/// This provides a high-level API for running many tasks with:
/// - Configurable concurrency
/// - Termination support
/// - Sequential or parallel execution
/// - Clean error handling
///
/// # Examples
///
/// ```
/// use gds::concurrency::virtual_threads::RunWithConcurrency;
/// use gds::concurrency::Concurrency;
/// use gds::termination::TerminationFlag;
///
/// let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
///     Box::new(|| println!("Task 1")),
///     Box::new(|| println!("Task 2")),
///     Box::new(|| println!("Task 3")),
/// ];
///
/// RunWithConcurrency::builder()
///     .concurrency(Concurrency::of(4))
///     .tasks(tasks)
///     .run()
///     .unwrap();
/// ```
pub struct RunWithConcurrency {
    _private: (),
}

impl RunWithConcurrency {
    /// Create a new builder.
    pub fn builder() -> Builder {
        Builder::new()
    }
}

/// Builder for RunWithConcurrency task execution.
pub struct Builder {
    concurrency: Option<Concurrency>,
    tasks: Vec<Box<dyn FnOnce() + Send>>,
    termination_flag: TerminationFlag,
    executor: Option<Executor>,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            concurrency: None,
            tasks: Vec::new(),
            termination_flag: TerminationFlag::running_true(),
            executor: None,
        }
    }

    /// Set the concurrency level.
    ///
    /// If concurrency is 1, tasks run sequentially on the calling thread.
    /// If concurrency > 1, tasks run in parallel using the executor.
    pub fn concurrency(mut self, concurrency: Concurrency) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    /// Add tasks from a Vec of closures.
    ///
    /// This is the most flexible way to add tasks - just provide closures!
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::concurrency::virtual_threads::RunWithConcurrency;
    /// use gds::concurrency::Concurrency;
    ///
    /// let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
    ///     Box::new(|| println!("Task 1")),
    ///     Box::new(|| println!("Task 2")),
    /// ];
    ///
    /// RunWithConcurrency::builder()
    ///     .concurrency(Concurrency::of(2))
    ///     .tasks(tasks)
    ///     .run();
    /// ```
    pub fn tasks(mut self, tasks: Vec<Box<dyn FnOnce() + Send>>) -> Self {
        self.tasks = tasks;
        self
    }

    /// Add tasks from an iterator of closures.
    pub fn tasks_from_iter<I, F>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() + Send + 'static,
    {
        self.tasks = iter
            .into_iter()
            .map(|f| Box::new(f) as Box<dyn FnOnce() + Send>)
            .collect();
        self
    }

    /// Set the termination flag.
    pub fn termination_flag(mut self, flag: TerminationFlag) -> Self {
        self.termination_flag = flag;
        self
    }

    /// Set a custom executor.
    ///
    /// If not provided, a default executor will be created with the specified concurrency.
    pub fn executor(mut self, executor: Executor) -> Self {
        self.executor = Some(executor);
        self
    }

    /// Build and return the configured parameters.
    pub fn build(self) -> Result<RunWithConcurrencyParams, String> {
        let concurrency = self.concurrency.ok_or("concurrency must be set")?;

        if self.tasks.is_empty() {
            return Err("at least one task must be provided".to_string());
        }

        let executor = self.executor.unwrap_or_else(|| Executor::new(concurrency));

        Ok(RunWithConcurrencyParams {
            concurrency,
            tasks: self.tasks,
            termination_flag: self.termination_flag,
            executor,
        })
    }

    /// Build and immediately run all tasks.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all tasks completed successfully, or `Err(())` if terminated early.
    pub fn run(self) -> Result<(), String> {
        self.build()?.run()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

/// Configured parameters for task execution.
pub struct RunWithConcurrencyParams {
    concurrency: Concurrency,
    tasks: Vec<Box<dyn FnOnce() + Send>>,
    termination_flag: TerminationFlag,
    executor: Executor,
}

impl RunWithConcurrencyParams {
    /// Get the concurrency level.
    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    /// Get the termination flag.
    pub fn termination_flag(&self) -> &TerminationFlag {
        &self.termination_flag
    }

    /// Execute all tasks according to the configuration.
    ///
    /// If concurrency is 1, tasks run sequentially on the calling thread.
    /// If concurrency > 1, tasks run in parallel using the executor.
    pub fn run(self) -> Result<(), String> {
        // Check termination before starting
        if !self.termination_flag.running() {
            return Err("terminated before execution".to_string());
        }

        if self.concurrency.value() == 1 {
            // Sequential execution on calling thread
            self.run_sequential()
        } else {
            // Parallel execution using executor
            self.run_parallel()
        }
    }

    fn run_sequential(self) -> Result<(), String> {
        for task in self.tasks {
            if !self.termination_flag.running() {
                return Err("terminated during sequential execution".to_string());
            }
            task();
        }
        Ok(())
    }

    fn run_parallel(self) -> Result<(), String> {
        // Use a scope to ensure all tasks complete
        // We need to store tasks in a way that allows safe parallel access
        let tasks = std::sync::Arc::new(std::sync::Mutex::new(self.tasks));
        let task_count = tasks.lock().unwrap().len();

        self.executor
            .scope(&self.termination_flag, |scope| {
                scope.spawn_many(task_count, |_| {
                    // Pop one task at a time from the shared Vec
                    let task = {
                        let mut tasks_guard = tasks.lock().unwrap();
                        tasks_guard.pop()
                    };

                    if let Some(task) = task {
                        task();
                    }
                });
            })
            .map_err(|_| "terminated during parallel execution".to_string())
    }
}

/// Helper function to create a runnable closure from a function.
pub fn runnable<F>(f: F) -> Box<dyn FnOnce() + Send>
where
    F: FnOnce() + Send + 'static,
{
    Box::new(f)
}

/// Helper function to create multiple runnables from functions.
pub fn runnables<F>(functions: Vec<F>) -> Vec<Box<dyn FnOnce() + Send>>
where
    F: FnOnce() + Send + 'static,
{
    functions
        .into_iter()
        .map(|f| Box::new(f) as Box<dyn FnOnce() + Send>)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = Arc::clone(&counter);
        let c2 = Arc::clone(&counter);

        let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
            Box::new(move || {
                c1.fetch_add(1, Ordering::Relaxed);
            }),
            Box::new(move || {
                c2.fetch_add(1, Ordering::Relaxed);
            }),
        ];

        RunWithConcurrency::builder()
            .concurrency(Concurrency::of(2))
            .tasks(tasks)
            .run()
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_sequential_execution() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut tasks: Vec<Box<dyn FnOnce() + Send>> = Vec::new();

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            tasks.push(Box::new(move || {
                c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        RunWithConcurrency::builder()
            .concurrency(Concurrency::of(1))
            .tasks(tasks)
            .run()
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 10);
    }

    #[test]
    fn test_parallel_execution() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut tasks: Vec<Box<dyn FnOnce() + Send>> = Vec::new();

        for _ in 0..100 {
            let c = Arc::clone(&counter);
            tasks.push(Box::new(move || {
                c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        RunWithConcurrency::builder()
            .concurrency(Concurrency::of(4))
            .tasks(tasks)
            .run()
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_termination_sequential() {
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));
        let mut tasks: Vec<Box<dyn FnOnce() + Send>> = Vec::new();

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            tasks.push(Box::new(move || {
                c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        let result = RunWithConcurrency::builder()
            .concurrency(Concurrency::of(1))
            .tasks(tasks)
            .termination_flag(termination)
            .run();

        assert!(result.is_err());
        assert!(counter.load(Ordering::Relaxed) < 10);
    }

    #[test]
    fn test_termination_parallel() {
        let termination = TerminationFlag::stop_running();
        let counter = Arc::new(AtomicUsize::new(0));
        let mut tasks: Vec<Box<dyn FnOnce() + Send>> = Vec::new();

        for _ in 0..1000 {
            let c = Arc::clone(&counter);
            tasks.push(Box::new(move || {
                c.fetch_add(1, Ordering::Relaxed);
            }));
        }

        let result = RunWithConcurrency::builder()
            .concurrency(Concurrency::of(4))
            .tasks(tasks)
            .termination_flag(termination)
            .run();

        assert!(result.is_err());
        assert!(counter.load(Ordering::Relaxed) < 1000);
    }

    #[test]
    fn test_runnable_helper() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let task = runnable(move || {
            c.fetch_add(42, Ordering::Relaxed);
        });

        task();
        assert_eq!(counter.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_runnables_helper() {
        let counter = Arc::new(AtomicUsize::new(0));
        let tasks: Vec<_> = (0..5)
            .map(|_| {
                let c = Arc::clone(&counter);
                move || {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
            .collect();

        let boxed_tasks = runnables(tasks);

        RunWithConcurrency::builder()
            .concurrency(Concurrency::of(2))
            .tasks(boxed_tasks)
            .run()
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_tasks_from_iter() {
        let counter = Arc::new(AtomicUsize::new(0));
        let functions: Vec<_> = (0..10)
            .map(|_| {
                let c = Arc::clone(&counter);
                move || {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
            .collect();

        RunWithConcurrency::builder()
            .concurrency(Concurrency::of(4))
            .tasks_from_iter(functions)
            .run()
            .unwrap();

        assert_eq!(counter.load(Ordering::Relaxed), 10);
    }

    #[test]
    fn test_builder_validation() {
        // Missing concurrency
        let result = RunWithConcurrency::builder().build();
        assert!(result.is_err());

        // Empty tasks
        let result = RunWithConcurrency::builder()
            .concurrency(Concurrency::of(4))
            .build();
        assert!(result.is_err());
    }
}
