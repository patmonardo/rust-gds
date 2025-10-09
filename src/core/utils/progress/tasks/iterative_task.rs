use std::sync::Arc;
use super::{Task, Progress, TaskVisitor};
#[cfg(test)]
use super::UNKNOWN_VOLUME;

/// Execution modes for iterative tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterativeTaskMode {
    /// Upper bound but can terminate early.
    Dynamic,
    /// Unbounded - can keep adding iterations.
    Open,
    /// Upper bound and will execute exactly n times.
    Fixed,
}

/// Task that executes iterations of subtasks.
/// Supports dynamic, open, and fixed iteration modes.
pub struct IterativeTask {
    base: Task,
    sub_tasks_supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
    mode: IterativeTaskMode,
    max_iterations: usize,
}

impl IterativeTask {
    /// Create a new iterative task.
    ///
    /// # Arguments
    /// * `description` - Task description
    /// * `sub_tasks` - Unrolled list of subtasks for DYNAMIC and FIXED modes
    /// * `sub_tasks_supplier` - Supplier for creating new iteration subtasks
    /// * `mode` - Execution mode
    pub fn new(
        description: String,
        sub_tasks: Vec<Arc<Task>>,
        sub_tasks_supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
        mode: IterativeTaskMode,
    ) -> Self {
        let max_iterations = match mode {
            IterativeTaskMode::Open => 0,
            _ => {
                let tasks_per_iteration = sub_tasks_supplier().len();
                if tasks_per_iteration == 0 {
                    0
                } else {
                    sub_tasks.len() / tasks_per_iteration
                }
            }
        };

        Self {
            base: Task::new(description, sub_tasks),
            sub_tasks_supplier,
            mode,
            max_iterations,
        }
    }

    /// Get the base task.
    pub fn base(&self) -> &Task {
        &self.base
    }

    /// Get current progress.
    pub fn get_progress(&self) -> Progress {
        let base_progress = self.base.get_progress();
        
        if self.mode == IterativeTaskMode::Open {
            // For open mode, report progress as unknown
            Progress::of(base_progress.progress(), super::UNKNOWN_VOLUME)
        } else {
            base_progress
        }
    }

    /// Get next subtask after validation.
    #[allow(dead_code)] // Reserved for iteration control
    fn next_subtask_after_validation(&self) -> Option<Arc<Task>> {
        // First check if there's a pending subtask
        if let Some(next) = self.base.next_subtask() {
            return Some(next);
        }

        // For open mode or if we haven't reached max iterations, add more
        if self.mode == IterativeTaskMode::Open || self.can_add_more_iterations() {
            self.add_iteration_internal();
            self.base.next_subtask()
        } else {
            None
        }
    }

    /// Finish the task.
    pub fn finish(&self) {
        if self.mode == IterativeTaskMode::Fixed {
            let current_iteration = self.current_iteration();
            if current_iteration < self.max_iterations {
                panic!(
                    "Cannot finish iterative task '{}'. Expected {} iterations but only {} were executed.",
                    self.base.description(),
                    self.max_iterations,
                    current_iteration
                );
            }
        }
        self.base.finish();
    }

    /// Get current iteration number (0-based).
    pub fn current_iteration(&self) -> usize {
        let tasks_per_iteration = (self.sub_tasks_supplier)().len();
        if tasks_per_iteration == 0 {
            return 0;
        }

        let mut completed = 0;
        for sub_task in self.base.sub_tasks() {
            let status = sub_task.status();
            if status.is_terminal() {
                completed += 1;
            }
        }

        completed / tasks_per_iteration
    }

    /// Get execution mode.
    pub fn mode(&self) -> IterativeTaskMode {
        self.mode
    }

    /// Get maximum iterations.
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Get tasks per iteration.
    pub fn tasks_per_iteration(&self) -> usize {
        (self.sub_tasks_supplier)().len()
    }

    /// Check if more iterations can be added.
    pub fn can_add_more_iterations(&self) -> bool {
        match self.mode {
            IterativeTaskMode::Open => true,
            IterativeTaskMode::Dynamic => self.current_iteration() < self.max_iterations,
            IterativeTaskMode::Fixed => false,
        }
    }

    /// Add a new iteration (internal, modifies base task's subtasks).
    /// Note: This is a simplified version since Rust doesn't allow easy mutation of Arc<Task>.
    #[allow(dead_code)] // Reserved for dynamic iteration addition
    fn add_iteration_internal(&self) {
        // In the Java version, this modifies the subtasks list.
        // In Rust, we would need to use interior mutability (Mutex) on the subtasks.
        // For now, this is a placeholder that matches the interface.
        // A full implementation would require refactoring Task to have mutable subtasks.
    }

    /// Accept a visitor (Visitor pattern).
    pub fn visit(&self, task_visitor: &dyn TaskVisitor) {
        task_visitor.visit_iterative_task(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::tasks::Status;

    fn create_sub_tasks_supplier() -> Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync> {
        Arc::new(|| {
            vec![
                Arc::new(Task::new("Sub 1".to_string(), vec![])),
                Arc::new(Task::new("Sub 2".to_string(), vec![])),
            ]
        })
    }

    fn unroll_tasks(supplier: &Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>, iterations: usize) -> Vec<Arc<Task>> {
        (0..iterations)
            .flat_map(|_| supplier())
            .collect()
    }

    #[test]
    fn test_iterative_task_fixed_mode() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 3);
        
        let task = IterativeTask::new(
            "Fixed Iteration".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Fixed,
        );
        
        assert_eq!(task.mode(), IterativeTaskMode::Fixed);
        assert_eq!(task.max_iterations(), 3);
        assert_eq!(task.tasks_per_iteration(), 2);
        assert_eq!(task.base().sub_tasks().len(), 6);
    }

    #[test]
    fn test_iterative_task_dynamic_mode() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 5);
        
        let task = IterativeTask::new(
            "Dynamic Iteration".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Dynamic,
        );
        
        assert_eq!(task.mode(), IterativeTaskMode::Dynamic);
        assert_eq!(task.max_iterations(), 5);
        assert!(task.can_add_more_iterations());
    }

    #[test]
    fn test_iterative_task_open_mode() {
        let supplier = create_sub_tasks_supplier();
        
        let task = IterativeTask::new(
            "Open Iteration".to_string(),
            vec![],
            supplier,
            IterativeTaskMode::Open,
        );
        
        assert_eq!(task.mode(), IterativeTaskMode::Open);
        assert_eq!(task.max_iterations(), 0);
        assert!(task.can_add_more_iterations());
    }

    #[test]
    fn test_iterative_task_current_iteration() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 3);
        
        let task = IterativeTask::new(
            "Iteration Count".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Fixed,
        );
        
        assert_eq!(task.current_iteration(), 0);
        
        // Complete first iteration (2 tasks)
        task.base().sub_tasks()[0].start();
        task.base().sub_tasks()[0].finish();
        task.base().sub_tasks()[1].start();
        task.base().sub_tasks()[1].finish();
        
        assert_eq!(task.current_iteration(), 1);
    }

    #[test]
    fn test_iterative_task_progress_fixed() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 2);
        
        let task = IterativeTask::new(
            "Progress Fixed".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Fixed,
        );
        
        let progress = task.get_progress();
        // Progress delegates to base task
        assert_eq!(progress.volume(), UNKNOWN_VOLUME);
    }

    #[test]
    fn test_iterative_task_progress_open() {
        let supplier = create_sub_tasks_supplier();
        
        let task = IterativeTask::new(
            "Progress Open".to_string(),
            vec![],
            supplier,
            IterativeTaskMode::Open,
        );
        
        let progress = task.get_progress();
        // Open mode always reports unknown volume
        assert_eq!(progress.volume(), UNKNOWN_VOLUME);
    }

    #[test]
    #[should_panic(expected = "Cannot finish iterative task")]
    fn test_iterative_task_finish_fixed_incomplete() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 3);
        
        let task = IterativeTask::new(
            "Incomplete Fixed".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Fixed,
        );
        
        // Finish only 1 iteration out of 3
        task.base().sub_tasks()[0].start();
        task.base().sub_tasks()[0].finish();
        task.base().sub_tasks()[1].start();
        task.base().sub_tasks()[1].finish();
        
        task.finish(); // Should panic
    }

    #[test]
    fn test_iterative_task_finish_fixed_complete() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 2);
        
        let task = IterativeTask::new(
            "Complete Fixed".to_string(),
            sub_tasks,
            supplier,
            IterativeTaskMode::Fixed,
        );
        
        // Complete all iterations
        for sub_task in task.base().sub_tasks() {
            sub_task.start();
            sub_task.finish();
        }
        
        task.finish();
        assert_eq!(task.base().status(), Status::Finished);
    }

    #[test]
    fn test_iterative_task_can_add_more_dynamic() {
        let supplier = create_sub_tasks_supplier();
        let sub_tasks = unroll_tasks(&supplier, 3);
        
        let task = IterativeTask::new(
            "Add More Dynamic".to_string(),
            sub_tasks,
            supplier.clone(),
            IterativeTaskMode::Dynamic,
        );
        
        assert!(task.can_add_more_iterations());
        
        // Complete all 3 iterations
        for sub_task in task.base().sub_tasks() {
            sub_task.start();
            sub_task.finish();
        }
        
        // Should not be able to add more after reaching max
        assert!(!task.can_add_more_iterations());
    }

    #[test]
    fn test_iterative_task_can_add_more_open() {
        let supplier = create_sub_tasks_supplier();
        
        let task = IterativeTask::new(
            "Add More Open".to_string(),
            vec![],
            supplier,
            IterativeTaskMode::Open,
        );
        
        // Open mode can always add more
        assert!(task.can_add_more_iterations());
    }
}
