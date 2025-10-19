//! User logging and warning system for tracking per-user, per-task messages.
//!
//! This module provides infrastructure for logging warnings and messages
//! on a per-user, per-task basis. It's designed for multi-user environments
//! where each user's warnings need to be tracked separately.
//!
//! # Architecture
//!
//! - **UserLogStore**: Stores log messages per user/task
//! - **UserLogEntry**: Individual log entry with task, message, and timestamp
//! - **UserLogRegistry**: Adds warnings for a specific user
//! - **UserLogRegistryFactory**: Creates registry instances
//! - **EmptyUserLogStore/Factory**: No-op implementations for testing
//! - **LocalUserLogRegistryFactory**: Creates registries for specific users
//!
//! # Example
//!
//! ```text
//! use gds::core::utils::warnings::*;
//! use std::sync::Arc;
//!
//! // Create a store (empty for this example)
//! let store = Arc::new(EmptyUserLogStore::new());
//!
//! // Create a factory for a specific user
//! let factory = LocalUserLogRegistryFactory::new("alice".to_string(), store);
//!
//! // Create a registry
//! let registry = factory.new_instance();
//!
//! // Add warnings
//! let task = Task::new("Graph Loading", "load");
//! registry.add_warning_to_log(&task, "Node ID not found".to_string());
//! ```

mod empty_user_log_registry_factory;
mod empty_user_log_store;
mod local_user_log_registry_factory;
mod log_store;
mod per_database_user_log_store;
mod user_log_entry;
mod user_log_registry;
mod user_log_registry_factory;
mod user_log_store;

pub use empty_user_log_registry_factory::{
    EmptyUserLogRegistryFactory, EMPTY_USER_LOG_REGISTRY_FACTORY_INSTANCE,
};
pub use empty_user_log_store::{EmptyUserLogStore, EMPTY_USER_LOG_STORE_INSTANCE};
pub use local_user_log_registry_factory::LocalUserLogRegistryFactory;
pub use per_database_user_log_store::PerDatabaseUserLogStore;
pub use user_log_entry::UserLogEntry;
pub use user_log_registry::UserLogRegistry;
pub use user_log_registry_factory::UserLogRegistryFactory;
pub use user_log_store::UserLogStore;
