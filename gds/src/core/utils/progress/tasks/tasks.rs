// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Factory for creating task hierarchies.

use super::{IterativeTask, IterativeTaskMode, LeafTask, Task};
use std::sync::Arc;

/// Utility factory for creating task hierarchies.
///
/// Provides static factory methods for all task types with convenient APIs.
pub struct Tasks;

impl Tasks {
    /// Create an empty task with no description or children.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::Tasks;
    ///
    /// let empty = Tasks::empty();
    /// assert_eq!(empty.description(), "");
    /// ```
    pub fn empty() -> Task {
        Task::new(String::new(), vec![])
    }

    /// Create an intermediate task with description and children.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, Task};
    /// use std::sync::Arc;
    ///
    /// let child = Arc::new(Task::new("child".to_string(), vec![]));
    /// let parent = Tasks::task("parent".to_string(), vec![child]);
    /// assert_eq!(parent.description(), "parent");
    /// ```
    pub fn task(description: String, children: Vec<Arc<Task>>) -> Task {
        Task::new(description, children)
    }

    /// Create an intermediate task with description and variadic children.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, Task};
    /// use std::sync::Arc;
    ///
    /// let child1 = Arc::new(Task::new("child1".to_string(), vec![]));
    /// let child2 = Arc::new(Task::new("child2".to_string(), vec![]));
    /// let parent = Tasks::task_with_children("parent".to_string(), child1, vec![child2]);
    /// assert_eq!(parent.sub_tasks().len(), 2);
    /// ```
    pub fn task_with_children(
        description: String,
        first_child: Arc<Task>,
        mut rest: Vec<Arc<Task>>,
    ) -> Task {
        let mut children = vec![first_child];
        children.append(&mut rest);
        Task::new(description, children)
    }

    /// Create a fixed iteration task that executes exactly N iterations.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, Task};
    /// use std::sync::Arc;
    ///
    /// let supplier = Arc::new(|| vec![
    ///     Arc::new(Task::new("step".to_string(), vec![]))
    /// ]);
    /// let task = Tasks::iterative_fixed("iterations".to_string(), supplier, 3);
    /// assert_eq!(task.max_iterations(), 3);
    /// ```
    pub fn iterative_fixed(
        description: String,
        sub_tasks_supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
        iterations: usize,
    ) -> IterativeTask {
        let sub_tasks = Self::unroll_tasks(&sub_tasks_supplier, iterations);
        IterativeTask::new(
            description,
            sub_tasks,
            sub_tasks_supplier,
            IterativeTaskMode::Fixed,
        )
    }

    /// Create a dynamic iteration task that can terminate early (up to N iterations).
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, Task};
    /// use std::sync::Arc;
    ///
    /// let supplier = Arc::new(|| vec![
    ///     Arc::new(Task::new("step".to_string(), vec![]))
    /// ]);
    /// let task = Tasks::iterative_dynamic("iterations".to_string(), supplier, 5);
    /// assert_eq!(task.max_iterations(), 5);
    /// ```
    pub fn iterative_dynamic(
        description: String,
        sub_tasks_supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
        iterations: usize,
    ) -> IterativeTask {
        let sub_tasks = Self::unroll_tasks(&sub_tasks_supplier, iterations);
        IterativeTask::new(
            description,
            sub_tasks,
            sub_tasks_supplier,
            IterativeTaskMode::Dynamic,
        )
    }

    /// Create an open iteration task with unbounded iterations.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, Task, IterativeTaskMode};
    /// use std::sync::Arc;
    ///
    /// let supplier = Arc::new(|| vec![
    ///     Arc::new(Task::new("step".to_string(), vec![]))
    /// ]);
    /// let task = Tasks::iterative_open("iterations".to_string(), supplier);
    /// assert_eq!(task.mode(), IterativeTaskMode::Open);
    /// ```
    pub fn iterative_open(
        description: String,
        sub_tasks_supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
    ) -> IterativeTask {
        IterativeTask::new(
            description,
            vec![],
            sub_tasks_supplier,
            IterativeTaskMode::Open,
        )
    }

    /// Create a leaf task with unknown volume.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Tasks, UNKNOWN_VOLUME};
    ///
    /// let task = Tasks::leaf("process".to_string());
    /// assert_eq!(task.volume(), UNKNOWN_VOLUME);
    /// ```
    pub fn leaf(description: String) -> LeafTask {
        LeafTask::new(description, super::UNKNOWN_VOLUME)
    }

    /// Create a leaf task with specified volume.
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::Tasks;
    ///
    /// let task = Tasks::leaf_with_volume("process".to_string(), 1000);
    /// assert_eq!(task.volume(), 1000);
    /// ```
    pub fn leaf_with_volume(description: String, volume: usize) -> LeafTask {
        LeafTask::new(description, volume)
    }

    /// Unroll tasks for fixed/dynamic iterations.
    ///
    /// Creates a flat list of all iteration subtasks by calling the supplier
    /// multiple times and concatenating the results.
    fn unroll_tasks(
        sub_tasks_supplier: &Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync>,
        iterations: usize,
    ) -> Vec<Arc<Task>> {
        (0..iterations).flat_map(|_| sub_tasks_supplier()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_task() {
        let task = Tasks::empty();
        assert_eq!(task.description(), "");
        assert_eq!(task.sub_tasks().len(), 0);
    }

    #[test]
    fn test_task_with_children() {
        let child1 = Arc::new(Task::new("child1".to_string(), vec![]));
        let child2 = Arc::new(Task::new("child2".to_string(), vec![]));

        let parent = Tasks::task("parent".to_string(), vec![child1, child2]);

        assert_eq!(parent.description(), "parent");
        assert_eq!(parent.sub_tasks().len(), 2);
    }

    #[test]
    fn test_task_with_variadic_children() {
        let child1 = Arc::new(Task::new("child1".to_string(), vec![]));
        let child2 = Arc::new(Task::new("child2".to_string(), vec![]));
        let child3 = Arc::new(Task::new("child3".to_string(), vec![]));

        let parent = Tasks::task_with_children("parent".to_string(), child1, vec![child2, child3]);

        assert_eq!(parent.description(), "parent");
        assert_eq!(parent.sub_tasks().len(), 3);
    }

    #[test]
    fn test_leaf_unknown_volume() {
        let task = Tasks::leaf("leaf".to_string());

        assert_eq!(task.base().description(), "leaf");
        assert_eq!(task.volume(), super::super::UNKNOWN_VOLUME);
        assert!(task.has_unknown_volume());
    }

    #[test]
    fn test_leaf_with_volume() {
        let task = Tasks::leaf_with_volume("leaf".to_string(), 1000);

        assert_eq!(task.base().description(), "leaf");
        assert_eq!(task.volume(), 1000);
        assert!(!task.has_unknown_volume());
    }

    #[test]
    fn test_iterative_fixed() {
        let supplier = Arc::new(|| {
            vec![
                Arc::new(Task::new("step1".to_string(), vec![])),
                Arc::new(Task::new("step2".to_string(), vec![])),
            ]
        });

        let task = Tasks::iterative_fixed("fixed".to_string(), supplier, 3);

        assert_eq!(task.base().description(), "fixed");
        assert_eq!(task.mode(), IterativeTaskMode::Fixed);
        assert_eq!(task.max_iterations(), 3);
        assert_eq!(task.tasks_per_iteration(), 2);
        assert_eq!(task.base().sub_tasks().len(), 6); // 3 iterations * 2 tasks
    }

    #[test]
    fn test_iterative_dynamic() {
        let supplier = Arc::new(|| vec![Arc::new(Task::new("step".to_string(), vec![]))]);

        let task = Tasks::iterative_dynamic("dynamic".to_string(), supplier, 5);

        assert_eq!(task.base().description(), "dynamic");
        assert_eq!(task.mode(), IterativeTaskMode::Dynamic);
        assert_eq!(task.max_iterations(), 5);
        assert_eq!(task.tasks_per_iteration(), 1);
        assert_eq!(task.base().sub_tasks().len(), 5);
    }

    #[test]
    fn test_iterative_open() {
        let supplier = Arc::new(|| vec![Arc::new(Task::new("step".to_string(), vec![]))]);

        let task = Tasks::iterative_open("open".to_string(), supplier);

        assert_eq!(task.base().description(), "open");
        assert_eq!(task.mode(), IterativeTaskMode::Open);
        assert_eq!(task.max_iterations(), 0);
        assert_eq!(task.base().sub_tasks().len(), 0); // No initial unrolling
    }

    #[test]
    fn test_unroll_tasks() {
        let supplier: Arc<dyn Fn() -> Vec<Arc<Task>> + Send + Sync> = Arc::new(|| {
            vec![
                Arc::new(Task::new("a".to_string(), vec![])),
                Arc::new(Task::new("b".to_string(), vec![])),
            ]
        });

        let unrolled = Tasks::unroll_tasks(&supplier, 3);

        assert_eq!(unrolled.len(), 6);
        assert_eq!(unrolled[0].description(), "a");
        assert_eq!(unrolled[1].description(), "b");
        assert_eq!(unrolled[2].description(), "a");
        assert_eq!(unrolled[3].description(), "b");
        assert_eq!(unrolled[4].description(), "a");
        assert_eq!(unrolled[5].description(), "b");
    }

    #[test]
    fn test_complex_hierarchy() {
        // Build: root -> [phase1, phase2] where phase1 -> [step1, step2]
        let step1 = Arc::new(Tasks::leaf_with_volume("step1".to_string(), 100));
        let step2 = Arc::new(Tasks::leaf_with_volume("step2".to_string(), 200));
        let phase1 = Arc::new(Tasks::task(
            "phase1".to_string(),
            vec![
                Arc::new(Task::new(step1.base().description().to_string(), vec![])),
                Arc::new(Task::new(step2.base().description().to_string(), vec![])),
            ],
        ));

        let phase2 = Arc::new(Tasks::leaf_with_volume("phase2".to_string(), 300));

        let root = Tasks::task(
            "root".to_string(),
            vec![
                phase1,
                Arc::new(Task::new(phase2.base().description().to_string(), vec![])),
            ],
        );

        assert_eq!(root.description(), "root");
        assert_eq!(root.sub_tasks().len(), 2);
    }
}
