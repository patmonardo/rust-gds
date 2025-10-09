//! Virtual threads abstraction for graph algorithms.
//!
//! This module provides a high-level abstraction over Rayon's work-stealing parallelism
//! that's specifically designed for iterative graph algorithms like Pregel.
//!
//! ## Design Philosophy
//!
//! Java GDS has complex thread management with ExecutorService, WorkerPools, ThreadFactories,
//! and manual synchronization. This is necessary in Java because:
//! - Thread creation is expensive
//! - Manual lifecycle management is required
//! - Synchronization primitives are heavy
//!
//! Rust + Rayon gives us:
//! - **Work-stealing**: Automatic load balancing across CPU cores
//! - **Scoped threads**: Perfect synchronization without barriers
//! - **Zero cost**: Compiled to same code as manual threading
//! - **Type safety**: Compiler prevents data races
//!
//! ## What We Provide
//!
//! Instead of mimicking Java's complexity, we provide clean abstractions for:
//! - **Executor**: High-level parallel execution with termination support
//! - **Scope**: Synchronization boundary (supersteps in Pregel)
//! - **WorkerContext**: Per-thread state management
//!
//! ## Example: Pregel-Style Iteration
//!
//! ```rust
//! use rust_gds::concurrency::virtual_threads::{Executor, Scope};
//! use rust_gds::concurrency::Concurrency;
//! use rust_gds::termination::TerminationFlag;
//!
//! let executor = Executor::new(Concurrency::available_cores());
//! let termination = TerminationFlag::running_true();
//!
//! for iteration in 0..max_iterations {
//!     // Each iteration is a synchronization boundary (superstep)
//!     executor.scope(&termination, |scope| {
//!         // All workers execute in parallel
//!         scope.spawn_many(node_count, |node_id| {
//!             // Process node
//!             compute_vertex(node_id);
//!         });
//!         // Implicit barrier here - all spawned work completes before scope ends
//!     })?;
//!     
//!     // Synchronization point between iterations
//!     if converged() {
//!         break;
//!     }
//! }
//! ```

mod executor;
mod run_with_concurrency;
mod scope;
mod worker_context;

pub use executor::*;
pub use run_with_concurrency::*;
pub use scope::*;
pub use worker_context::*;
