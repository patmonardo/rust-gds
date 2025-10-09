use super::{Progress, Status, TaskVisitor, UNKNOWN_VOLUME};
use crate::core::utils::clock_service::ClockService;
use std::sync::{Arc, Mutex};

/// Base class for all tasks in the progress tracking system.
/// Handles task hierarchy, state management, timing, and memory estimation.
#[derive(Clone)]
pub struct Task {
    description: String,
    sub_tasks: Vec<Arc<Task>>,
    status: Arc<Mutex<Status>>,
    start_time: Arc<Mutex<u64>>,
    finish_time: Arc<Mutex<u64>>,
    estimated_memory_range_in_bytes: Arc<Mutex<(usize, usize)>>,
    max_concurrency: Arc<Mutex<usize>>,
}

impl Task {
    pub const UNKNOWN_CONCURRENCY: usize = usize::MAX;
    pub const NOT_STARTED: u64 = 0;
    pub const NOT_FINISHED: u64 = 0;

    /// Create a new task with description and subtasks.
    pub fn new(description: String, sub_tasks: Vec<Arc<Task>>) -> Self {
        Self {
            description,
            sub_tasks,
            status: Arc::new(Mutex::new(Status::Pending)),
            start_time: Arc::new(Mutex::new(Self::NOT_STARTED)),
            finish_time: Arc::new(Mutex::new(Self::NOT_FINISHED)),
            estimated_memory_range_in_bytes: Arc::new(Mutex::new((0, 0))),
            max_concurrency: Arc::new(Mutex::new(Self::UNKNOWN_CONCURRENCY)),
        }
    }

    /// Get task description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get subtasks.
    pub fn sub_tasks(&self) -> &[Arc<Task>] {
        &self.sub_tasks
    }

    /// Get current status.
    pub fn status(&self) -> Status {
        *self.status.lock().unwrap()
    }

    /// Get next subtask to execute.
    pub fn next_subtask(&self) -> Option<Arc<Task>> {
        self.next_subtask_after_validation()
    }

    /// Start task execution.
    pub fn start(&self) {
        let current_status = self.status();
        if current_status != Status::Pending {
            panic!(
                "Cannot start task '{}' with status {:?}. Task must have status Pending.",
                self.description, current_status
            );
        }

        *self.status.lock().unwrap() = Status::Running;
        *self.start_time.lock().unwrap() = ClockService::clock().millis();
    }

    /// Finish task successfully.
    pub fn finish(&self) {
        *self.status.lock().unwrap() = Status::Finished;
        *self.finish_time.lock().unwrap() = ClockService::clock().millis();
    }

    /// Cancel task execution.
    pub fn cancel(&self) {
        let current_status = self.status();
        if current_status == Status::Running {
            *self.status.lock().unwrap() = Status::Canceled;
            *self.finish_time.lock().unwrap() = ClockService::clock().millis();
        }
    }

    /// Mark task as failed.
    pub fn fail(&self) {
        *self.status.lock().unwrap() = Status::Failed;
    }

    /// Get current progress.
    pub fn get_progress(&self) -> Progress {
        // If no subtasks, volume is unknown
        if self.sub_tasks.is_empty() {
            return Progress::of(UNKNOWN_VOLUME, UNKNOWN_VOLUME);
        }

        let mut progress = 0usize;
        let mut volume = 0usize;
        let mut has_unknown_volume = false;

        for sub_task in &self.sub_tasks {
            let sub_progress = sub_task.get_progress();

            if sub_progress.volume() == UNKNOWN_VOLUME {
                has_unknown_volume = true;
                break;
            }

            progress += sub_progress.progress();
            volume += sub_progress.volume();
        }

        if has_unknown_volume {
            Progress::of(UNKNOWN_VOLUME, UNKNOWN_VOLUME)
        } else {
            Progress::of(progress, volume)
        }
    }

    /// Set task volume (no-op for base Task, overridden in LeafTask).
    pub fn set_volume(&self, _volume: usize) {
        // Base implementation does nothing
    }

    /// Log progress (no-op for base Task, overridden in LeafTask).
    pub fn log_progress(&self, _value: usize) {
        // Base implementation does nothing
    }

    /// Accept a visitor (Visitor pattern).
    pub fn visit(&self, task_visitor: &dyn TaskVisitor) {
        task_visitor.visit_intermediate_task(self);
    }

    /// Get start time in milliseconds.
    pub fn start_time(&self) -> u64 {
        *self.start_time.lock().unwrap()
    }

    /// Get finish time in milliseconds.
    pub fn finish_time(&self) -> u64 {
        *self.finish_time.lock().unwrap()
    }

    /// Check if task has not started yet.
    pub fn has_not_started(&self) -> bool {
        self.start_time() == Self::NOT_STARTED
    }

    /// Get estimated memory range in bytes.
    pub fn estimated_memory_range_in_bytes(&self) -> (usize, usize) {
        *self.estimated_memory_range_in_bytes.lock().unwrap()
    }

    /// Get maximum concurrency.
    pub fn max_concurrency(&self) -> usize {
        *self.max_concurrency.lock().unwrap()
    }

    /// Set estimated memory range.
    pub fn set_estimated_memory_range_in_bytes(&self, min: usize, max: usize) {
        *self.estimated_memory_range_in_bytes.lock().unwrap() = (min, max);
    }

    /// Set maximum concurrency.
    pub fn set_max_concurrency(&self, concurrency: usize) {
        *self.max_concurrency.lock().unwrap() = concurrency;
    }

    /// Get next subtask after validation (can be overridden).
    fn next_subtask_after_validation(&self) -> Option<Arc<Task>> {
        for sub_task in &self.sub_tasks {
            if sub_task.status() == Status::Pending {
                return Some(Arc::clone(sub_task));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task".to_string(), vec![]);
        assert_eq!(task.description(), "Test Task");
        assert_eq!(task.status(), Status::Pending);
        assert_eq!(task.sub_tasks().len(), 0);
    }

    #[test]
    fn test_task_lifecycle() {
        let task = Task::new("Lifecycle Task".to_string(), vec![]);

        assert_eq!(task.status(), Status::Pending);
        assert!(task.has_not_started());

        task.start();
        assert_eq!(task.status(), Status::Running);
        assert!(!task.has_not_started());
        assert!(task.start_time() > 0);

        task.finish();
        assert_eq!(task.status(), Status::Finished);
        assert!(task.finish_time() > 0);
    }

    #[test]
    #[should_panic(expected = "Cannot start task")]
    fn test_task_cannot_start_twice() {
        let task = Task::new("Double Start".to_string(), vec![]);
        task.start();
        task.start(); // Should panic
    }

    #[test]
    fn test_task_cancellation() {
        let task = Task::new("Cancel Task".to_string(), vec![]);
        task.start();
        task.cancel();
        assert_eq!(task.status(), Status::Canceled);
        assert!(task.finish_time() > 0);
    }

    #[test]
    fn test_task_failure() {
        let task = Task::new("Fail Task".to_string(), vec![]);
        task.fail();
        assert_eq!(task.status(), Status::Failed);
    }

    #[test]
    fn test_task_with_subtasks() {
        let sub1 = Arc::new(Task::new("Sub 1".to_string(), vec![]));
        let sub2 = Arc::new(Task::new("Sub 2".to_string(), vec![]));
        let parent = Task::new("Parent".to_string(), vec![sub1.clone(), sub2.clone()]);

        assert_eq!(parent.sub_tasks().len(), 2);

        let next = parent.next_subtask();
        assert!(next.is_some());
        assert_eq!(next.unwrap().description(), "Sub 1");
    }

    #[test]
    fn test_task_progress_aggregation() {
        let sub1 = Arc::new(Task::new("Sub 1".to_string(), vec![]));
        let sub2 = Arc::new(Task::new("Sub 2".to_string(), vec![]));
        let parent = Task::new("Parent".to_string(), vec![sub1, sub2]);

        let progress = parent.get_progress();
        // Base tasks have unknown volume
        assert_eq!(progress.volume(), UNKNOWN_VOLUME);
    }

    #[test]
    fn test_task_memory_estimation() {
        let task = Task::new("Memory Task".to_string(), vec![]);
        task.set_estimated_memory_range_in_bytes(1024, 2048);

        let (min, max) = task.estimated_memory_range_in_bytes();
        assert_eq!(min, 1024);
        assert_eq!(max, 2048);
    }

    #[test]
    fn test_task_concurrency() {
        let task = Task::new("Concurrent Task".to_string(), vec![]);
        assert_eq!(task.max_concurrency(), Task::UNKNOWN_CONCURRENCY);

        task.set_max_concurrency(8);
        assert_eq!(task.max_concurrency(), 8);
    }
}
