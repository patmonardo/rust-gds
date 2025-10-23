// Concurrency package - Java GDS compatible concurrency primitives
//
// This package provides the foundational types for parallel graph algorithm execution.
// It mirrors the Java GDS concurrency package while leveraging Rust's safety guarantees.

pub mod atomics;
pub mod parallel_util;
pub mod pool;
pub mod validator;
pub mod virtual_threads;

mod batch_size;
mod concurrency_level;
mod termination;
pub use batch_size::*;
pub use concurrency_level::*;
pub use termination::*;