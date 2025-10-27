// Arrow-Native Factory Module
//
// This module provides Arrow-native GraphStore construction.
// Arrow IS the native format for rust-gds (like Neo4j is native for Java GDS).
//
// Key principles:
// - Zero-copy where possible (Arrow arrays → PropertyValues directly)
// - Parallel import (chunked, concurrent)
// - Schema-driven (Arrow schema → GraphStore schema)
// - Memory-efficient (streaming, bounded buffers)
// - NOT IO (assumes Arrow tables already in memory)

// Public API surface
pub use self::config::{ArrowProjectionConfig, ArrowProjectionError};
pub use self::consumer::{
    BufferedEdgeConsumer, BufferedNodeConsumer, CompositeEdgeConsumer, NodeRecord,
    RecordConsumer as ConsumerTrait, RelationshipRecord,
};
pub use self::consumer_task::{
    ConsumerEdgeImportTask, ConsumerEdgeImportTaskFactory, ConsumerNodeImportTask,
    ConsumerNodeImportTaskFactory,
};
pub use self::factory::ArrowNativeFactory;
pub use self::importer::{
    EdgeAccumulator, EdgeImportTask, EdgeImportTaskFactory, ImporterError, NodeAccumulator,
    NodeImportTask, NodeImportTaskFactory,
};
pub use self::reference::{
    ArrowBatchReference, ArrowReference, ArrowReferenceError, EdgeTableReference,
    NodeTableReference,
};
pub use self::scanner::{
    BatchScanner, EdgeBatchScanner, NodeBatchScanner, RecordConsumer, ScanCursor, ScannerError,
    DEFAULT_BATCH_SIZE, DEFAULT_PREFETCH_SIZE,
};
pub use self::task::{
    AggregatedImportResult, ImportResult, ImportTask, ProgressTracker, TaskError, TaskFactory,
    TaskRunner,
};

// Module organization
mod config;
mod consumer; // Phase 7: BufferedConsumers (IMPLEMENTED)
mod consumer_task; // Phase 8: Consumer-based ImportTask implementations (Option A)
mod factory;
mod importer; // Phase 5: NodeImportTask, EdgeImportTask, accumulators (GAMMA STRATEGY) + Phase 6 properties
mod reference; // Phase 2: TableReference, BatchReference (IMPLEMENTED with arrow2)
mod scanner; // Phase 3: BatchScanner trait + NodeBatchScanner + EdgeBatchScanner
mod task; // Phase 4: ImportTask trait + TaskRunner + parallel orchestration

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Basic smoke test - ensures module structure is valid
        // Real tests will come in each phase
    }
}
