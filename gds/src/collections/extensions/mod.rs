//! Collections Extensions: Advanced Features for Collections First Approach
//!
//! This module provides extension implementations for Collections,
//! repackaging GDS utilities as Collections Extensions for the Collections First approach.

pub mod ndarray;
pub mod gpu;
pub mod distributed;
pub mod compression;
pub mod encryption;
pub mod ml;
pub mod paging;
pub mod memory_estimation;
pub mod queue;
pub mod stack;
pub mod metrics;
pub mod random;
pub mod partitioning;

pub use compression::*;
pub use paging::*;
pub use memory_estimation::*;
pub use queue::*;
pub use stack::*;
pub use metrics::*;
pub use random::*;
pub use partitioning::*;