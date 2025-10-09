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

//! Depth-aware visitor for task hierarchies.

use super::TaskVisitor;

/// Abstract visitor that tracks traversal depth in task hierarchy.
///
/// Useful for indented rendering, depth-limited operations, etc.
pub trait DepthAwareTaskVisitor: TaskVisitor {
    /// Set current depth in the hierarchy.
    fn set_depth(&mut self, depth: usize);

    /// Get current traversal depth.
    fn depth(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::tasks::Task;
    use std::cell::RefCell;

    struct TestDepthVisitor {
        depth: RefCell<usize>,
        max_depth_seen: RefCell<usize>,
    }

    impl TaskVisitor for TestDepthVisitor {
        fn visit(&self, _task: &Task) {
            let current_depth = *self.depth.borrow();
            let mut max_depth = self.max_depth_seen.borrow_mut();
            if current_depth > *max_depth {
                *max_depth = current_depth;
            }
        }
    }

    impl DepthAwareTaskVisitor for TestDepthVisitor {
        fn set_depth(&mut self, depth: usize) {
            *self.depth.borrow_mut() = depth;
        }

        fn depth(&self) -> usize {
            *self.depth.borrow()
        }
    }

    #[test]
    fn test_depth_tracking() {
        let mut visitor = TestDepthVisitor {
            depth: RefCell::new(0),
            max_depth_seen: RefCell::new(0),
        };

        let task = Task::new("test".to_string(), vec![]);

        visitor.set_depth(0);
        visitor.visit(&task);
        assert_eq!(*visitor.max_depth_seen.borrow(), 0);

        visitor.set_depth(2);
        visitor.visit(&task);
        assert_eq!(*visitor.max_depth_seen.borrow(), 2);
        assert_eq!(visitor.depth(), 2);
    }
}
