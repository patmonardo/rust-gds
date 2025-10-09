use super::{Progress, Task, TaskVisitor, UNKNOWN_VOLUME};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Leaf task implementation - terminal node in task hierarchy.
/// Tracks progress with atomic operations and handles volume updates.
pub struct LeafTask {
    base: Task,
    volume: Arc<Mutex<usize>>,
    current_progress: AtomicUsize,
}

impl LeafTask {
    /// Create a new leaf task with description and volume.
    pub fn new(description: String, volume: usize) -> Self {
        Self {
            base: Task::new(description, vec![]),
            volume: Arc::new(Mutex::new(volume)),
            current_progress: AtomicUsize::new(0),
        }
    }

    /// Get the base task.
    pub fn base(&self) -> &Task {
        &self.base
    }

    /// Finish task and set progress to 100%.
    pub fn finish(&self) {
        self.base.finish();

        // This task should now be considered to have 100% progress.
        let mut volume = self.volume.lock().unwrap();
        if *volume == super::UNKNOWN_VOLUME {
            *volume = self.current_progress.load(Ordering::Relaxed);
        }

        let current = self.current_progress.load(Ordering::Relaxed);
        let remaining = volume.saturating_sub(current);
        self.current_progress
            .fetch_add(remaining, Ordering::Relaxed);
    }

    /// Set task volume.
    pub fn set_volume(&self, volume: usize) {
        *self.volume.lock().unwrap() = volume;
    }

    /// Log progress increment.
    pub fn log_progress(&self, value: usize) {
        self.current_progress.fetch_add(value, Ordering::Relaxed);
    }

    /// Get current progress.
    pub fn get_progress(&self) -> Progress {
        let current = self.current_progress.load(Ordering::Relaxed);
        let volume = *self.volume.lock().unwrap();
        Progress::of(current, volume)
    }

    /// Accept a visitor (Visitor pattern).
    pub fn visit(&self, task_visitor: &dyn TaskVisitor) {
        task_visitor.visit_leaf_task(self);
    }

    /// Get current progress value.
    pub fn current_progress_value(&self) -> usize {
        self.current_progress.load(Ordering::Relaxed)
    }

    /// Get task volume.
    pub fn volume(&self) -> usize {
        *self.volume.lock().unwrap()
    }

    /// Reset progress to zero.
    pub fn reset_progress(&self) {
        self.current_progress.store(0, Ordering::Relaxed);
    }

    /// Check if task has unknown volume.
    pub fn has_unknown_volume(&self) -> bool {
        *self.volume.lock().unwrap() == super::UNKNOWN_VOLUME
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::tasks::Status;

    #[test]
    fn test_leaf_task_creation() {
        let task = LeafTask::new("Leaf Task".to_string(), 100);
        assert_eq!(task.base().description(), "Leaf Task");
        assert_eq!(task.volume(), 100);
        assert_eq!(task.current_progress_value(), 0);
    }

    #[test]
    fn test_leaf_task_progress() {
        let task = LeafTask::new("Progress Task".to_string(), 100);

        task.log_progress(25);
        assert_eq!(task.current_progress_value(), 25);

        task.log_progress(35);
        assert_eq!(task.current_progress_value(), 60);

        let progress = task.get_progress();
        assert_eq!(progress.progress(), 60);
        assert_eq!(progress.volume(), 100);
    }

    #[test]
    fn test_leaf_task_finish_with_known_volume() {
        let task = LeafTask::new("Finish Task".to_string(), 100);

        task.base().start();
        task.log_progress(50);
        task.finish();

        assert_eq!(task.base().status(), Status::Finished);
        assert_eq!(task.current_progress_value(), 100);

        let progress = task.get_progress();
        assert_eq!(progress.progress(), 100);
        assert_eq!(progress.volume(), 100);
    }

    #[test]
    fn test_leaf_task_finish_with_unknown_volume() {
        let task = LeafTask::new("Unknown Volume".to_string(), UNKNOWN_VOLUME);

        task.log_progress(75);
        task.finish();

        // Volume should be set to current progress
        assert_eq!(task.volume(), 75);
        assert_eq!(task.current_progress_value(), 75);
    }

    #[test]
    fn test_leaf_task_set_volume() {
        let task = LeafTask::new("Set Volume".to_string(), 100);
        assert_eq!(task.volume(), 100);

        task.set_volume(200);
        assert_eq!(task.volume(), 200);
    }

    #[test]
    fn test_leaf_task_reset_progress() {
        let task = LeafTask::new("Reset Task".to_string(), 100);

        task.log_progress(50);
        assert_eq!(task.current_progress_value(), 50);

        task.reset_progress();
        assert_eq!(task.current_progress_value(), 0);
    }

    #[test]
    fn test_leaf_task_unknown_volume() {
        let task = LeafTask::new("Unknown".to_string(), UNKNOWN_VOLUME);
        assert!(task.has_unknown_volume());

        task.set_volume(100);
        assert!(!task.has_unknown_volume());
    }

    #[test]
    fn test_leaf_task_over_completion() {
        let task = LeafTask::new("Over Complete".to_string(), 100);

        task.log_progress(120);
        let progress = task.get_progress();

        // Progress can exceed volume (e.g., for estimated volumes)
        assert_eq!(progress.progress(), 120);
        assert_eq!(progress.volume(), 100);
        assert!(progress.relative_progress() >= 1.0);
    }

    #[test]
    fn test_leaf_task_concurrent_progress() {
        use std::sync::Arc;
        use std::thread;

        let task = Arc::new(LeafTask::new("Concurrent".to_string(), 1000));
        let mut handles = vec![];

        for _ in 0..10 {
            let task_clone = Arc::clone(&task);
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    task_clone.log_progress(1);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(task.current_progress_value(), 100);
    }
}
