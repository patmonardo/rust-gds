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

//! Task hierarchy and progress tracking for graph algorithms.
//!
//! This module provides a comprehensive task system for tracking
//! long-running graph computations:
//!
//! - **Task Types**: Leaf, intermediate, and iterative tasks
//! - **Progress Tracking**: Volume-based progress with batched updates
//! - **Visitor Pattern**: Flexible traversal of task hierarchies
//! - **Status Management**: Lifecycle tracking (pending → running → finished)
//!
//! # Architecture
//!
//! The task system follows a composite pattern with visitor support:
//!
//! ```text
//! Task (base)
//! ├── LeafTask (terminal nodes with progress)
//! ├── Task (intermediate nodes with subtasks)
//! └── IterativeTask (repeating subtasks)
//! ```

pub mod depth_aware_task_visitor;
pub mod iterative_task;
pub mod leaf_task;
pub mod log_level;
pub mod progress;
pub mod status;
pub mod task;
pub mod task_traversal;
pub mod task_visitor;
#[allow(clippy::module_inception)]
pub mod tasks;

pub use depth_aware_task_visitor::DepthAwareTaskVisitor;
pub use iterative_task::{IterativeTask, IterativeTaskMode};
pub use leaf_task::LeafTask;
pub use log_level::LogLevel;
pub use progress::{Progress, UNKNOWN_VOLUME};
pub use status::Status;
pub use task::Task;
pub use task_traversal::TaskTraversal;
pub use task_visitor::{TaskLike, TaskVisitor};
pub use tasks::Tasks;
