//! Storage runtime traits and execution surface.
//!
//! StorageRuntime is the DIFFERENCE pole of the Storage extreme, parallel to
//! ComputationRuntime being the DIFFERENCE pole of the Computation extreme.
//!
//! The Five-Fold Brahmachakra:
//! 1. PipelineDescriptor (Unity/Dharma) - The governing relation
//! 2. ComputationDescriptor (Computation Identity/Science)
//! 3. ComputationRuntime (Computation Difference/Manifestation)
//! 4. StorageDescriptor (Storage Identity/Science)
//! 5. StorageRuntime (Storage Difference/Manifestation) ← THIS MODULE
//!
//! Keep small and safe so generated code can implement these traits with minimal imports.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

// Import descriptors from the descriptors module
use crate::projection::codegen::descriptors::{PipelineDescriptor, StorageDescriptor};
use crate::types::graph::Graph;

/// Errors produced by storage runtime execution
#[derive(Debug)]
pub enum StorageError {
    InitFailed(String),
    ReadFailed(String),
    WriteFailed(String),
    FlushFailed(String),
    FinalizeFailed(String),
    AllocationFailed(String),
    ConcurrencyFailed(String),
    PersistenceFailed(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::InitFailed(msg) => write!(f, "Storage init failed: {}", msg),
            StorageError::ReadFailed(msg) => write!(f, "Storage read failed: {}", msg),
            StorageError::WriteFailed(msg) => write!(f, "Storage write failed: {}", msg),
            StorageError::FlushFailed(msg) => write!(f, "Storage flush failed: {}", msg),
            StorageError::FinalizeFailed(msg) => write!(f, "Storage finalize failed: {}", msg),
            StorageError::AllocationFailed(msg) => write!(f, "Storage allocation failed: {}", msg),
            StorageError::ConcurrencyFailed(msg) => {
                write!(f, "Storage concurrency failed: {}", msg)
            }
            StorageError::PersistenceFailed(msg) => {
                write!(f, "Storage persistence failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for StorageError {}

/// Minimal storage execution context passed to StorageRuntime.
///
/// This context carries the essential runtime information:
/// - The graph being operated on
/// - The pipeline descriptor (the Dharma - properties flowing through computation + storage)
/// - The storage descriptor (how this specific storage executes)
/// - Node count for sizing
///
/// Future additions: memory_tracker, metrics, cache_manager, progress_tracker
pub struct StorageContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub pipeline: &'a PipelineDescriptor,
    pub storage: &'a StorageDescriptor,
    pub node_count: usize,
}

impl<'a> StorageContext<'a> {
    pub fn new(
        graph: &'a Arc<dyn Graph>,
        pipeline: &'a PipelineDescriptor,
        storage: &'a StorageDescriptor,
    ) -> Self {
        let node_count = graph.node_count();
        Self {
            graph,
            pipeline,
            storage,
            node_count,
        }
    }
}

/// Storage value wrapper (placeholder for typed storage values)
#[derive(Debug, Clone)]
pub enum StorageValue {
    Long(i64),
    Double(f64),
    LongArray(Vec<i64>),
    DoubleArray(Vec<f64>),
    // Future: other types
}

/// Storage runtime lifecycle trait (parallel to Computer).
///
/// This is the DIFFERENCE pole: HOW storage EXECUTES, not WHAT storage IS.
/// StorageDescriptor declares WHAT (Identity/Science).
/// StorageRuntime enacts HOW (Difference/Manifestation).
///
/// Lifecycle: init → read/write* → flush → finalize
pub trait StorageRuntime: Send + Sync {
    /// Initialize storage (allocate memory, setup concurrency, open files).
    /// This enacts the StorageDescriptor's specifications.
    fn init(&mut self, ctx: &mut StorageContext) -> Result<(), StorageError>;

    /// Read a value from storage (enacts ConcurrencyModel, AccessPattern).
    fn read(&self, ctx: &StorageContext, id: u64) -> Result<StorageValue, StorageError>;

    /// Write a value to storage (enacts ConcurrencyModel, Mutability).
    fn write(
        &mut self,
        ctx: &mut StorageContext,
        id: u64,
        value: StorageValue,
    ) -> Result<(), StorageError>;

    /// Flush storage (enacts PersistenceConfig, SyncPolicy).
    fn flush(&mut self, ctx: &mut StorageContext) -> Result<(), StorageError>;

    /// Finalize storage (sync, checkpoint, cleanup).
    fn finalize(&mut self, ctx: &mut StorageContext) -> Result<(), StorageError>;
}

/// Storage accessor trait for single operations (parallel to ComputeStep).
pub trait StorageAccessor: Send + Sync {
    /// Single access operation (read or write).
    fn access(
        &self,
        ctx: &StorageContext,
        id: u64,
        mode: AccessMode,
    ) -> Result<StorageValue, StorageError>;
}

/// Access mode for storage operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Read,
    Write,
    ReadWrite,
}

/// Factory function type for creating StorageRuntime instances.
pub type StorageRuntimeFactory = fn(u32) -> Result<Box<dyn StorageRuntime>, StorageError>;

lazy_static::lazy_static! {
    static ref STORAGE_RUNTIME_FACTORIES: std::sync::RwLock<HashMap<u32, StorageRuntimeFactory>> =
        std::sync::RwLock::new(HashMap::new());
}

/// Register a factory for instantiating StorageRuntime from a descriptor ID.
/// Returns true if newly inserted, false if ID already registered.
pub fn register_storage_runtime_factory(id: u32, factory: StorageRuntimeFactory) -> bool {
    use std::collections::hash_map::Entry;
    let mut reg = STORAGE_RUNTIME_FACTORIES.write().unwrap();
    match reg.entry(id) {
        Entry::Vacant(e) => {
            e.insert(factory);
            true
        }
        Entry::Occupied(_) => false,
    }
}

/// Instantiate a StorageRuntime from a registered factory.
pub fn instantiate_storage_runtime_from_descriptor(
    id: u32,
) -> Result<Box<dyn StorageRuntime>, StorageError> {
    let reg = STORAGE_RUNTIME_FACTORIES.read().unwrap();
    let factory = reg
        .get(&id)
        .ok_or_else(|| StorageError::InitFailed(format!("No factory registered for id {}", id)))?;
    factory(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::codegen::descriptors::storage::{BackendTechnology, StorageDescriptor};
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    /// Dummy StorageRuntime for testing
    struct DummyStorageRuntime {
        data: Vec<i64>,
        inited: bool,
        flushed: bool,
        finalized: bool,
    }

    impl DummyStorageRuntime {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                inited: false,
                flushed: false,
                finalized: false,
            }
        }
    }

    impl StorageRuntime for DummyStorageRuntime {
        fn init(&mut self, ctx: &mut StorageContext) -> Result<(), StorageError> {
            self.data = vec![0; ctx.node_count];
            self.inited = true;
            Ok(())
        }

        fn read(&self, _ctx: &StorageContext, id: u64) -> Result<StorageValue, StorageError> {
            self.data
                .get(id as usize)
                .copied()
                .map(StorageValue::Long)
                .ok_or_else(|| StorageError::ReadFailed(format!("Invalid id {}", id)))
        }

        fn write(
            &mut self,
            _ctx: &mut StorageContext,
            id: u64,
            value: StorageValue,
        ) -> Result<(), StorageError> {
            if let StorageValue::Long(v) = value {
                if let Some(slot) = self.data.get_mut(id as usize) {
                    *slot = v;
                    return Ok(());
                }
            }
            Err(StorageError::WriteFailed(format!("Invalid write {}", id)))
        }

        fn flush(&mut self, _ctx: &mut StorageContext) -> Result<(), StorageError> {
            self.flushed = true;
            Ok(())
        }

        fn finalize(&mut self, _ctx: &mut StorageContext) -> Result<(), StorageError> {
            self.finalized = true;
            Ok(())
        }
    }

    #[test]
    fn dummy_storage_runtime_lifecycle() {
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = DefaultGraphStore::random(&config).expect("graph creation failed");
        let graph = graph_store.graph();
        let graph_arc: Arc<dyn Graph> = graph.clone();

        let storage_desc = StorageDescriptor::new(
            1,
            "test_storage",
            BackendTechnology::HugeArray { page_size: 4096 },
        );
        let pipeline_desc = PipelineDescriptor::test_pipeline("TestPipeline");

        let mut ctx = StorageContext::new(&graph_arc, &pipeline_desc, &storage_desc);
        let mut runtime = DummyStorageRuntime::new();

        // Init
        assert!(runtime.init(&mut ctx).is_ok());
        assert!(runtime.inited);
        assert_eq!(runtime.data.len(), graph_arc.node_count());

        // Write
        assert!(runtime.write(&mut ctx, 0, StorageValue::Long(42)).is_ok());
        assert!(runtime.write(&mut ctx, 1, StorageValue::Long(99)).is_ok());

        // Read
        match runtime.read(&ctx, 0).unwrap() {
            StorageValue::Long(v) => assert_eq!(v, 42),
            _ => panic!("wrong type"),
        }
        match runtime.read(&ctx, 1).unwrap() {
            StorageValue::Long(v) => assert_eq!(v, 99),
            _ => panic!("wrong type"),
        }

        // Flush
        assert!(runtime.flush(&mut ctx).is_ok());
        assert!(runtime.flushed);

        // Finalize
        assert!(runtime.finalize(&mut ctx).is_ok());
        assert!(runtime.finalized);
    }

    #[test]
    fn register_and_instantiate_factory() {
        // Clear any previous registrations (test isolation)
        STORAGE_RUNTIME_FACTORIES.write().unwrap().clear();

        let factory: StorageRuntimeFactory = |_id| Ok(Box::new(DummyStorageRuntime::new()));
        assert!(register_storage_runtime_factory(99, factory));
        assert!(!register_storage_runtime_factory(99, factory)); // already present

        let runtime = instantiate_storage_runtime_from_descriptor(99).expect("instantiate ok");

        // Verify it's a valid runtime by running lifecycle
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = DefaultGraphStore::random(&config).expect("graph creation failed");
        let graph = graph_store.graph();
        let graph_arc: Arc<dyn Graph> = graph.clone();

        let storage_desc =
            StorageDescriptor::new(1, "test", BackendTechnology::HugeArray { page_size: 4096 });
        let pipeline_desc = PipelineDescriptor::test_pipeline("FactoryTest");

        let mut ctx = StorageContext::new(&graph_arc, &pipeline_desc, &storage_desc);
        let mut runtime = runtime;
        assert!(runtime.init(&mut ctx).is_ok());
        assert!(runtime.read(&ctx, 0).is_ok());
        assert!(runtime.finalize(&mut ctx).is_ok());
    }

    #[test]
    fn instantiate_missing_factory_fails() {
        STORAGE_RUNTIME_FACTORIES.write().unwrap().clear();
        let result = instantiate_storage_runtime_from_descriptor(999);
        assert!(result.is_err());
    }
}
