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

//! Visitor pattern for traversing task hierarchies.

use super::{IterativeTask, LeafTask, Task};

/// Visitor for traversing task hierarchies.
///
/// Provides specialized visit methods for different task types.
/// Default implementations delegate to the generic `visit()` method.
pub trait TaskVisitor {
    /// Visit a leaf task (terminal node).
    fn visit_leaf_task(&self, task: &LeafTask) {
        self.visit(task.base());
    }

    /// Visit an intermediate task (has children).
    fn visit_intermediate_task(&self, task: &Task) {
        self.visit(task);
    }

    /// Visit an iterative task (repeating operation).
    fn visit_iterative_task(&self, task: &IterativeTask) {
        self.visit(task.base());
    }

    /// Generic visit method - fallback for all task types.
    fn visit(&self, _task: &Task) {
        // Default implementation does nothing
    }
}

/// Marker trait for types that can be visited (kept for backwards compatibility).
pub trait TaskLike {
    fn description(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct CountingVisitor {
        visit_count: RefCell<usize>,
        leaf_count: RefCell<usize>,
    }

    impl TaskVisitor for CountingVisitor {
        fn visit_leaf_task(&self, _task: &LeafTask) {
            *self.leaf_count.borrow_mut() += 1;
        }

        fn visit(&self, _task: &Task) {
            *self.visit_count.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_visitor_delegation() {
        let visitor = CountingVisitor {
            visit_count: RefCell::new(0),
            leaf_count: RefCell::new(0),
        };

        let task = LeafTask::new("test".to_string(), 100);

        visitor.visit_leaf_task(&task);
        assert_eq!(*visitor.leaf_count.borrow(), 1);
    }
}
