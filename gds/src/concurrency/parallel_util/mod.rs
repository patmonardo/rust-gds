//! Parallel execution utilities for graph algorithms.
//!
//! This module provides the core parallel execution infrastructure for GDS algorithms.
//! It combines:
//! - **Phase 1**: Batch calculation utilities (pure math functions)
//! - **Phase 2**: Rayon-powered parallel execution (work-stealing parallelism)
//!
//! ## Design Philosophy
//!
//! Java GDS uses ExecutorService/ThreadPoolExecutor with manual task management.
//! Rust GDS uses Rayon's work-stealing scheduler which is:
//!   - Faster (lock-free work stealing)
//!   - Simpler (no manual thread pool management)
//!   - Safer (compile-time data race prevention)
//!
//! We keep the GDS API surface for algorithm compatibility, but underneath
//! it's pure Rayon magic!

mod batch_util;
mod parallel_executor;

pub use batch_util::BatchUtil;
pub use parallel_executor::*;

/// Default batch size for parallel operations.
pub const DEFAULT_BATCH_SIZE: usize = 10_000;
