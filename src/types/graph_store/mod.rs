//! GraphStore - Central interface for managing and accessing graph data.
#![allow(clippy::module_inception)]
//!
//! The GraphStore module provides the main orchestration layer for graph data management,
//! including schema, properties, topology, and filtered views.

mod capabilities;
mod database_id;
mod database_info;
mod default_graph_store;
mod deletion_result;
mod graph_name;
mod graph_store;

pub use capabilities::Capabilities;
pub use database_id::DatabaseId;
pub use database_info::{DatabaseInfo, DatabaseLocation};
pub use default_graph_store::DefaultGraphStore;
pub use deletion_result::DeletionResult;
pub use graph_name::GraphName;
pub use graph_store::{GraphStore, GraphStoreAdapter, GraphStoreError, GraphStoreResult};
