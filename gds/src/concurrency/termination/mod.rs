// Termination system for graceful cancellation of long-running algorithms
//
// This module provides a lightweight termination system that allows algorithms
// to check for cancellation requests and gracefully terminate execution.
//
// ## Design Philosophy
//
// Java GDS uses:
//   - TerminationMonitor (functional interface)
//   - TerminationFlag (throttled checking with 10s interval)
//   - TerminatedException (thrown on termination)
//
// Rust uses the same pattern but with:
//   - Trait instead of functional interface
//   - AtomicBool + Instant for thread-safe throttling
//   - Standard Error trait for exceptions

mod termination_exception;
mod termination_flag;
mod termination_monitor;

pub use termination_exception::*;
pub use termination_flag::*;
pub use termination_monitor::*;

/// Number of nodes to process before checking termination status.
///
/// This constant is used by algorithms to determine how often to check
/// the termination flag during node iteration.
pub const RUN_CHECK_NODE_COUNT: usize = 10_000;
