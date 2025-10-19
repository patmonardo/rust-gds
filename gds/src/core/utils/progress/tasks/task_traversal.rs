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

//! Task hierarchy traversal utilities.

use super::{DepthAwareTaskVisitor, Task};

/// Utility for traversing task hierarchies with depth tracking.
///
/// Provides pre-order traversal (parent before children) with depth awareness.
pub struct TaskTraversal;

impl TaskTraversal {
    /// Traverse task hierarchy in pre-order with depth tracking.
    ///
    /// Starts traversal at depth 0 for the root task.
    ///
    /// # Arguments
    /// * `task` - The root task to start traversal from
    /// * `visitor` - The depth-aware visitor to apply at each node
    ///
    /// # Example
    /// ```
    /// use rust_gds::core::utils::progress::tasks::{Task, DepthAwareTaskVisitor, TaskTraversal};
    /// use std::sync::Arc;
    ///
    /// struct PrintVisitor;
    ///
    /// impl DepthAwareTaskVisitor for PrintVisitor {
    ///     fn set_depth(&mut self, _depth: usize) {}
    ///     fn depth(&self) -> usize { 0 }
    /// }
    ///
    /// let task = Task::new("root".to_string(), vec![]);
    /// let mut visitor = PrintVisitor;
    /// TaskTraversal::visit_pre_order_with_depth(&task, &mut visitor);
    /// ```
    pub fn visit_pre_order_with_depth(task: &Task, visitor: &mut dyn DepthAwareTaskVisitor) {
        Self::visit_pre_order_with_depth_internal(task, visitor, 0);
    }

    /// Internal recursive implementation of pre-order traversal with depth.
    fn visit_pre_order_with_depth_internal(
        task: &Task,
        visitor: &mut dyn DepthAwareTaskVisitor,
        depth: usize,
    ) {
        // Set depth on visitor before visiting
        visitor.set_depth(depth);

        // Visit current task (delegates to appropriate visit method based on task type)
        task.visit(visitor);

        // Recursively visit all subtasks at increased depth
        for sub_task in task.sub_tasks() {
            Self::visit_pre_order_with_depth_internal(sub_task, visitor, depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DepthTrackingVisitor {
        visited: RefCell<Vec<(String, usize)>>,
        depth: RefCell<usize>,
    }

    impl DepthTrackingVisitor {
        fn new() -> Self {
            Self {
                visited: RefCell::new(Vec::new()),
                depth: RefCell::new(0),
            }
        }

        fn visited(&self) -> Vec<(String, usize)> {
            self.visited.borrow().clone()
        }
    }

    impl super::super::TaskVisitor for DepthTrackingVisitor {
        fn visit(&self, task: &Task) {
            let depth = *self.depth.borrow();
            self.visited
                .borrow_mut()
                .push((task.description().to_string(), depth));
        }
    }

    impl DepthAwareTaskVisitor for DepthTrackingVisitor {
        fn set_depth(&mut self, depth: usize) {
            *self.depth.borrow_mut() = depth;
        }

        fn depth(&self) -> usize {
            *self.depth.borrow()
        }
    }

    #[test]
    fn test_single_task_traversal() {
        let task = Task::new("root".to_string(), vec![]);
        let mut visitor = DepthTrackingVisitor::new();

        TaskTraversal::visit_pre_order_with_depth(&task, &mut visitor);

        let visited = visitor.visited();
        assert_eq!(visited.len(), 1);
        assert_eq!(visited[0], ("root".to_string(), 0));
    }

    #[test]
    fn test_nested_task_traversal() {
        let child1 = Arc::new(Task::new("child1".to_string(), vec![]));
        let child2 = Arc::new(Task::new("child2".to_string(), vec![]));
        let parent = Task::new("parent".to_string(), vec![child1, child2]);

        let mut visitor = DepthTrackingVisitor::new();

        TaskTraversal::visit_pre_order_with_depth(&parent, &mut visitor);

        let visited = visitor.visited();
        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], ("parent".to_string(), 0));
        assert_eq!(visited[1], ("child1".to_string(), 1));
        assert_eq!(visited[2], ("child2".to_string(), 1));
    }

    #[test]
    fn test_deep_hierarchy_traversal() {
        let grandchild = Arc::new(Task::new("grandchild".to_string(), vec![]));
        let child = Arc::new(Task::new("child".to_string(), vec![grandchild]));
        let root = Task::new("root".to_string(), vec![child]);

        let mut visitor = DepthTrackingVisitor::new();

        TaskTraversal::visit_pre_order_with_depth(&root, &mut visitor);

        let visited = visitor.visited();
        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], ("root".to_string(), 0));
        assert_eq!(visited[1], ("child".to_string(), 1));
        assert_eq!(visited[2], ("grandchild".to_string(), 2));
    }

    #[test]
    fn test_pre_order_guarantees() {
        // Build tree:
        //       root
        //      /    \
        //    left   right
        //    /
        //  leaf

        let leaf = Arc::new(Task::new("leaf".to_string(), vec![]));
        let left = Arc::new(Task::new("left".to_string(), vec![leaf]));
        let right = Arc::new(Task::new("right".to_string(), vec![]));
        let root = Task::new("root".to_string(), vec![left, right]);

        let mut visitor = DepthTrackingVisitor::new();

        TaskTraversal::visit_pre_order_with_depth(&root, &mut visitor);

        let visited = visitor.visited();
        assert_eq!(visited.len(), 4);

        // Pre-order: parent before children
        assert_eq!(visited[0], ("root".to_string(), 0));
        assert_eq!(visited[1], ("left".to_string(), 1));
        assert_eq!(visited[2], ("leaf".to_string(), 2));
        assert_eq!(visited[3], ("right".to_string(), 1));

        // Verify left subtree fully visited before right
        let left_index = visited.iter().position(|v| v.0 == "left").unwrap();
        let leaf_index = visited.iter().position(|v| v.0 == "leaf").unwrap();
        let right_index = visited.iter().position(|v| v.0 == "right").unwrap();

        assert!(left_index < leaf_index);
        assert!(leaf_index < right_index);
    }
}
