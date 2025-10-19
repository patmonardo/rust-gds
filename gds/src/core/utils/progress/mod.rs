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

//! Progress tracking infrastructure for long-running graph algorithms.
//!
//! This module provides comprehensive progress tracking for data science pipelines:
//! - Task management and registration
//! - Progress logging with batched updates
//! - Task stores and registries
//! - User and job tracking
//!
//! # Architecture
//!
//! The progress tracking system consists of several layers:
//!
//! 1. **Tasks**: Define what work needs to be done (`Task`)
//! 2. **Storage**: Persist and query running tasks (`TaskStore`)
//! 3. **Registry**: Manage tasks for user sessions (`TaskRegistry`)
//! 4. **Logging**: Report progress efficiently (`ProgressLogger`)
//!
//! # Examples
//!
//! ```rust,ignore
//! use gds::core::utils::progress::*;
//!
//! // Create a task store
//! let store = PerDatabaseTaskStore::new();
//!
//! // Register a task
//! let job_id = JobId::new();
//! let task = Task::new("Graph Algorithm".to_string(), 1000);
//! store.store("user".to_string(), job_id.clone(), task);
//!
//! // Query tasks
//! let user_tasks = store.query_by_username("user");
//! ```

pub mod batching_progress_logger;
pub mod empty_task_store;
pub mod job_id;
pub mod observable_task_store;
pub mod per_database_task_store;
pub mod progress_logger;
pub mod task;
pub mod task_registry;
pub mod task_registry_factory;
pub mod task_store;
pub mod task_store_holder;
pub mod task_store_listener;
pub mod task_store_provider;
pub mod task_store_service;
pub mod tasks;
pub mod user_task;

pub use batching_progress_logger::{BatchingProgressLogger, MAXIMUM_LOG_INTERVAL};
pub use empty_task_store::EmptyTaskStore;
pub use job_id::JobId;
pub use observable_task_store::ObservableTaskStore;
pub use per_database_task_store::PerDatabaseTaskStore;
pub use progress_logger::{MessageFactory, ProgressLogger, NO_MESSAGE};
pub use task::{Task, UNKNOWN_VOLUME};
pub use task_registry::TaskRegistry;
pub use task_registry_factory::{
    EmptyTaskRegistryFactory, LocalTaskRegistryFactory, TaskRegistryFactories, TaskRegistryFactory,
};
pub use task_store::TaskStore;
#[allow(deprecated)]
pub use task_store_holder::TaskStoreHolder;
pub use task_store_listener::TaskStoreListener;
pub use task_store_provider::{SimpleTaskStoreProvider, TaskStoreProvider, TaskStoreProviders};
pub use task_store_service::TaskStoreService;
pub use user_task::UserTask;
