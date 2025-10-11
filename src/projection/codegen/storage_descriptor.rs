//! Storage Descriptor - The Manifestation of Form in Matter
//!
//! StorageDescriptor describes how PropertyDescriptor (Form/Svarūpa) manifests
//! in physical storage (Matter/Rūpa). Together with PropertyDescriptor and
//! ComputationDescriptor, it forms the triadic absolute:
//!
//!     ॐ = <Storage (Gross), Property (Form), Computation (Subtle)>
//!
//! This is the "struggled for Kantian Absolute" realized as Hegelian
//! Concrete Universal - the CENTER OF ALL EXTREMES.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::types::ValueType;

/// Physical layout strategy for storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageLayout {
    /// Column-oriented (Arrow-style)
    Columnar,
    /// Row-oriented (traditional)
    RowOriented,
    /// Chunked/paged (HugeArray-style)
    Chunked,
    /// Sparse representation
    Sparse,
    /// Mixed/hybrid strategies
    Hybrid,
}

impl StorageLayout {
    pub fn default_for_backend(backend: &BackendTechnology) -> Self {
        match backend {
            BackendTechnology::HugeArray { .. } => StorageLayout::Chunked,
            BackendTechnology::Arrow { .. } => StorageLayout::Columnar,
            BackendTechnology::Sparse { .. } => StorageLayout::Sparse,
            BackendTechnology::Custom(_) => StorageLayout::Hybrid,
        }
    }
}

/// Data density characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density {
    /// Most values present (>90%)
    Dense,
    /// Many missing/default values (<50%)
    Sparse,
    /// Variable density
    Mixed,
}

/// Access pattern hints for optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessPattern {
    /// Linear scans
    Sequential,
    /// Point lookups
    Random,
    /// Vertex-centric (Pregel-style)
    VertexCentric,
    /// Edge-centric (graph traversal)
    EdgeCentric,
    /// Bulk operations
    Batch,
    /// Mixed patterns
    Mixed,
}

/// Mutability characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutability {
    /// Cannot be modified after creation
    Immutable,
    /// Can be modified in-place
    Mutable,
    /// Copy-on-write semantics
    CopyOnWrite,
}

/// Data locality characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locality {
    /// Optimized for cache
    CacheFriendly,
    /// NUMA-aware placement
    NumaAware,
    /// Distributed across machines
    Distributed,
    /// No specific locality
    Unspecified,
}

/// Memory profile describing storage characteristics
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryProfile {
    pub density: Density,
    pub access_pattern: AccessPattern,
    pub mutability: Mutability,
    pub locality: Locality,
}

impl Default for MemoryProfile {
    fn default() -> Self {
        Self {
            density: Density::Dense,
            access_pattern: AccessPattern::Sequential,
            mutability: Mutability::Immutable,
            locality: Locality::Unspecified,
        }
    }
}

/// Persistence strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Persistence {
    /// In-memory only
    Ephemeral,
    /// Disk-backed
    Durable,
    /// Network-replicated
    Distributed,
    /// Tiered (memory + disk)
    Hybrid,
}

/// Synchronization policy for durable storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncPolicy {
    /// Immediate flush
    Immediate,
    /// Periodic flush
    Periodic,
    /// Flush on demand
    OnDemand,
}

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compression {
    None,
    LZ4,
    Zstd,
    Snappy,
    Custom,
}

/// Persistence configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PersistenceConfig {
    pub strategy: Persistence,
    pub sync_policy: SyncPolicy,
    pub compression: Compression,
}

impl PersistenceConfig {
    pub fn ephemeral() -> Self {
        Self {
            strategy: Persistence::Ephemeral,
            sync_policy: SyncPolicy::OnDemand,
            compression: Compression::None,
        }
    }

    pub fn durable() -> Self {
        Self {
            strategy: Persistence::Durable,
            sync_policy: SyncPolicy::Periodic,
            compression: Compression::LZ4,
        }
    }
}

/// Concurrency model for storage access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConcurrencyModel {
    /// No concurrent access
    SingleThreaded,
    /// Read-only concurrent access
    ReadOnly,
    /// Copy-on-write semantics
    CopyOnWrite,
    /// Lock-based (Mutex/RwLock)
    LockBased,
    /// Lock-free atomic operations
    LockFree,
    /// Multi-version concurrency control
    MVCC,
}

/// Growth policy for dynamic storage
#[derive(Debug, Clone, PartialEq)]
pub enum GrowthPolicy {
    /// Fixed size, cannot grow
    Fixed,
    /// Grow by fixed amount
    Linear(usize),
    /// Grow exponentially
    Exponential(f64),
    /// Auto-tune based on usage
    Adaptive,
}

/// Physical geometry of storage
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalGeometry {
    /// Memory alignment in bytes
    pub alignment: usize,
    /// Page/chunk size in bytes
    pub page_size: usize,
    /// Initial number of pages
    pub initial_pages: usize,
    /// Growth policy
    pub growth: GrowthPolicy,
}

impl Default for PhysicalGeometry {
    fn default() -> Self {
        Self {
            alignment: 64,   // Cache line
            page_size: 4096, // Standard page
            initial_pages: 1,
            growth: GrowthPolicy::Exponential(2.0),
        }
    }
}

/// Backend technology choices
#[derive(Debug, Clone, PartialEq)]
pub enum BackendTechnology {
    HugeArray {
        page_size: usize,
    },
    Arrow {
        // Future: Arrow-specific config
    },
    Sparse {
        initial_capacity: usize,
        load_factor: f64,
    },
    Custom(String), // Named custom backend
}

/// Storage Descriptor - Complete description of storage manifestation
#[derive(Debug, Clone, PartialEq)]
pub struct StorageDescriptor {
    /// Unique identifier
    pub id: u32,

    /// Human-readable name
    pub name: String,

    /// Physical layout strategy
    pub layout: StorageLayout,

    /// Memory characteristics
    pub memory_profile: MemoryProfile,

    /// Persistence configuration
    pub persistence: PersistenceConfig,

    /// Concurrency model
    pub concurrency: ConcurrencyModel,

    /// Physical geometry
    pub geometry: PhysicalGeometry,

    /// Backend technology
    pub backend: BackendTechnology,

    /// Compatible value types
    pub compatible_types: Vec<ValueType>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl StorageDescriptor {
    /// Create new storage descriptor with defaults
    pub fn new(id: u32, name: impl Into<String>, backend: BackendTechnology) -> Self {
        let layout = StorageLayout::default_for_backend(&backend);
        Self {
            id,
            name: name.into(),
            layout,
            memory_profile: MemoryProfile::default(),
            persistence: PersistenceConfig::ephemeral(),
            concurrency: ConcurrencyModel::ReadOnly,
            geometry: PhysicalGeometry::default(),
            backend,
            compatible_types: vec![
                ValueType::Long,
                ValueType::Double,
                ValueType::Boolean,
                ValueType::String,
                ValueType::LongArray,
                ValueType::DoubleArray,
            ],
            metadata: HashMap::new(),
        }
    }

    /// Builder: Set layout
    pub fn with_layout(mut self, layout: StorageLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Builder: Set density
    pub fn with_density(mut self, density: Density) -> Self {
        self.memory_profile.density = density;
        self
    }

    /// Builder: Set access pattern
    pub fn with_access_pattern(mut self, pattern: AccessPattern) -> Self {
        self.memory_profile.access_pattern = pattern;
        self
    }

    /// Builder: Set concurrency model
    pub fn with_concurrency(mut self, model: ConcurrencyModel) -> Self {
        self.concurrency = model;
        self
    }

    /// Builder: Set persistence
    pub fn with_persistence(mut self, config: PersistenceConfig) -> Self {
        self.persistence = config;
        self
    }

    /// Builder: Set page size
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.geometry.page_size = size;
        self
    }
}

lazy_static::lazy_static! {
    static ref STORAGE_REGISTRY: RwLock<HashMap<u32, StorageDescriptor>> =
        RwLock::new(HashMap::new());
}

/// Register a storage descriptor
pub fn register_storage_descriptor(desc: StorageDescriptor) -> bool {
    let mut reg = STORAGE_REGISTRY.write().expect("storage registry poisoned");
    reg.insert(desc.id, desc).is_none()
}

/// Get storage descriptor by id
pub fn get_storage_descriptor(id: u32) -> Option<StorageDescriptor> {
    let reg = STORAGE_REGISTRY.read().expect("storage registry poisoned");
    reg.get(&id).cloned()
}

/// Clear registry (for tests)
#[cfg(test)]
pub fn clear_storage_registry() {
    let mut reg = STORAGE_REGISTRY.write().expect("storage registry poisoned");
    reg.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_register_storage_descriptor() {
        clear_storage_registry();

        let desc = StorageDescriptor::new(
            1,
            "test_storage",
            BackendTechnology::HugeArray { page_size: 4096 },
        )
        .with_density(Density::Dense)
        .with_access_pattern(AccessPattern::VertexCentric)
        .with_concurrency(ConcurrencyModel::ReadOnly);

        assert!(register_storage_descriptor(desc.clone()));

        let retrieved = get_storage_descriptor(1).expect("found");
        assert_eq!(retrieved.name, "test_storage");
        assert_eq!(retrieved.layout, StorageLayout::Chunked);
        assert_eq!(retrieved.memory_profile.density, Density::Dense);
    }

    #[test]
    fn default_layout_for_backends() {
        assert_eq!(
            StorageLayout::default_for_backend(&BackendTechnology::HugeArray { page_size: 4096 }),
            StorageLayout::Chunked
        );
        assert_eq!(
            StorageLayout::default_for_backend(&BackendTechnology::Arrow {}),
            StorageLayout::Columnar
        );
        assert_eq!(
            StorageLayout::default_for_backend(&BackendTechnology::Sparse {
                initial_capacity: 100,
                load_factor: 0.75
            }),
            StorageLayout::Sparse
        );
    }
}
