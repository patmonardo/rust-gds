//! Memory estimation services.
//!
//! Higher-level services for estimating memory usage of graphs and algorithms.
//!
//! This module provides service-layer abstractions for memory estimation that sit
//! on top of the core memory system. These services are designed to work with
//! graph projections, algorithm configurations, and provide user-friendly memory
//! estimation workflows.
//!
//! # Modules
//!
//! - `graph_memory_estimation` - Container for graph dimensions + memory tree
//! - `memory_estimation_result` - Builder and formatter for estimation results
//! - `memory_budget_validator` - Validate estimations against memory budgets
//!
//! # Example
//!
//! ```rust,ignore
//! use gds::mem::memest::*;
//!
//! // Create estimation result
//! let result = MemoryEstimationResultBuilder::new()
//!     .with_dimensions(dimensions)
//!     .with_memory_tree(tree)
//!     .build();
//!
//! // Validate against budget
//! let validator = MemoryBudgetValidator::new(8 * 1024 * 1024 * 1024); // 8 GiB
//! if validator.validate(&result) {
//!     println!("Memory OK: {}", result.format_memory_usage());
//! }
//! ```

mod fictitious_graph_estimation;
mod graph_memory_estimation;
mod memory_budget_validator;
mod memory_estimation_result;

pub use fictitious_graph_estimation::FictitiousGraphEstimationService;
pub use graph_memory_estimation::GraphMemoryEstimation;
pub use memory_budget_validator::MemoryBudgetValidator;
pub use memory_estimation_result::{MemoryEstimationResult, MemoryEstimationResultBuilder};
