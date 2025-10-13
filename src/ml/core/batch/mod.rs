//! Batch processing module for ML in GDS.
//!
//! This module contains interfaces and utilities for batch processing
//! in machine learning operations.

pub mod batch;
pub mod batch_queue;
pub mod batch_transformer;
pub mod list_batch;
pub mod mapped_batch;
pub mod range_batch;
pub mod singleton_batch;

pub use batch::Batch;
pub use batch_queue::{
    compute_batch_size, consecutive, consecutive_with_batch_size, consecutive_with_concurrency,
    BatchQueue, ConsecutiveBatchQueue, SimpleBatch, DEFAULT_BATCH_SIZE,
};
pub use batch_transformer::{BatchTransformer, IdentityBatchTransformer};
pub use list_batch::ListBatch;
pub use mapped_batch::MappedBatch;
pub use range_batch::RangeBatch;
pub use singleton_batch::SingletonBatch;
