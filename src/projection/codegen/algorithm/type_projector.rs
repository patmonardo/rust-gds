//! Type Projector - Maya as Dialectical Absolute
//!
//! The Type Projector is the Unity of Revealing (Vidyā) and Concealing (Avidyā),
//! projecting PropertyDescriptor (Form/Svarūpa) to both Storage and Computation extremes.
//!
//! This is distinct from Value Projection (Primitive ↔ Property) which transforms values.
//! Type Projection maps between MODES OF MANIFESTATION: Storage ↔ Computation.
//!
//! # Philosophical Foundation
//!
//! ```text
//!                     ĀTMAN-BRAHMAN (The Absolute)
//!                            |
//!               ┌────────────┴────────────┐
//!               |                         |
//!             MAYA                     ĪŚVARA
//!       (Vidyā ↔ Avidyā)        (Sṛṣṭi-Sthiti-Saṃhāra)
//!    (Revealing ↔ Concealing)  (Create-Preserve-Destroy)
//!               |                         |
//!        TYPE PROJECTOR            OBJECT LIFECYCLE
//!     (Storage ↔ Computation)      (Build-Use-Drop)
//! ```
//!
//! # The Dialectical Object System
//!
//! - **Thesis**: Storage (data-at-rest, Gross/Sthūla)
//! - **Antithesis**: Computation (data-in-motion, Subtle/Sūkṣma)
//! - **Synthesis**: Type Projector (dialectical unity, Form/Svarūpa)
//!
//! # Core Trait
//!
//! ```rust,ignore
//! pub trait TypeProjector {
//!     // Vidyā: Revealing Form as Storage extreme
//!     fn project_to_storage(&self, form: &PropertyDescriptor)
//!         -> Result<StorageDescriptor, ProjectionError>;
//!     
//!     // Vidyā: Revealing Form as Computation extreme
//!     fn project_to_computation(&self, form: &PropertyDescriptor)
//!         -> Result<ComputationDescriptor, ProjectionError>;
//!     
//!     // Avidyā: Recognizing Form from manifestation
//!     fn recognize_from_storage(&self, storage: &StorageDescriptor)
//!         -> Result<PropertyDescriptor, ProjectionError>;
//!     
//!     // The dialectical validation
//!     fn validate_projection(&self,
//!         form: &PropertyDescriptor,
//!         storage: &StorageDescriptor,
//!         computation: &ComputationDescriptor
//!     ) -> Result<(), ProjectionError>;
//! }
//! ```
//!
//! See `doc/TYPE_PROJECTOR_AS_MAYA.md` for complete philosophical architecture.

use std::error::Error;
use std::fmt;

use crate::types::ValueType;

// Import descriptors from the descriptors module
use crate::projection::codegen::descriptors::storage::StorageDescriptor;
use crate::projection::codegen::descriptors::{
    ComputationDescriptor, ComputationPattern, ComputationSpecies, PropertyDescriptor, StorageHint,
};

#[cfg(test)]
use crate::projection::codegen::descriptors::storage::{
    AccessPattern, BackendTechnology, Mutability, StorageLayout,
};

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during type projection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    /// Incompatible type mapping
    IncompatibleTypes {
        source: String,
        target: String,
        reason: String,
    },
    /// Missing required metadata
    MissingMetadata {
        descriptor_type: String,
        field: String,
    },
    /// Unsupported projection
    UnsupportedProjection { from: String, to: String },
    /// Validation failed
    ValidationFailed { descriptor: String, reason: String },
    /// Custom error
    Custom(String),
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleTypes {
                source,
                target,
                reason,
            } => {
                write!(
                    f,
                    "Incompatible type mapping from {} to {}: {}",
                    source, target, reason
                )
            }
            Self::MissingMetadata {
                descriptor_type,
                field,
            } => {
                write!(f, "Missing metadata in {}: {}", descriptor_type, field)
            }
            Self::UnsupportedProjection { from, to } => {
                write!(f, "Unsupported projection from {} to {}", from, to)
            }
            Self::ValidationFailed { descriptor, reason } => {
                write!(f, "Validation failed for {}: {}", descriptor, reason)
            }
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ProjectionError {}

// ============================================================================
// Type Projector Trait - Maya as Dialectical Unity
// ============================================================================

/// The Absolute Type - Maya as dialectical projector of Storage ↔ Computation
///
/// This trait embodies the Unity of Revealing (Vidyā) and Concealing (Avidyā).
/// It projects PropertyDescriptor (Form) to both Storage and Computation extremes,
/// and recognizes the Form from either manifestation.
///
/// # Philosophical Interpretation
///
/// - **Vidyā (Revealing)**: `project_to_storage()` and `project_to_computation()`
///   reveal the Form as manifesting in two modes
/// - **Avidyā (Concealing)**: `recognize_from_*()` methods conceal the duality
///   by recognizing the singular Form
/// - **Brahman Realization**: The ability to validate consistency across all
///   three descriptors (`validate_projection()`)
///
/// # Example
///
/// ```rust,ignore
/// use crate::projection::codegen::type_projector::*;
///
/// let projector = HugeArrayProjector::default();
/// let form = PropertyDescriptor::new(...);
///
/// // Vidyā: Reveal the duality
/// let storage = projector.project_to_storage(&form)?;
/// let computation = projector.project_to_computation(&form)?;
///
/// // Avidyā: Recognize the unity
/// let recognized = projector.recognize_from_storage(&storage)?;
/// assert_eq!(form.id, recognized.id);
///
/// // Brahman: Know the absolute consistency
/// projector.validate_projection(&form, &storage, &computation)?;
/// ```
pub trait TypeProjector: Send + Sync {
    /// Project Form to Storage extreme (Vidyā: Revealing as Gross/Sthūla)
    ///
    /// Takes a PropertyDescriptor (essential nature) and projects it to
    /// StorageDescriptor (physical manifestation as data-at-rest).
    fn project_to_storage(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<StorageDescriptor, ProjectionError>;

    /// Project Form to Computation extreme (Vidyā: Revealing as Subtle/Sūkṣma)
    ///
    /// Takes a PropertyDescriptor (essential nature) and projects it to
    /// ComputationDescriptor (computational manifestation as data-in-motion).
    fn project_to_computation(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<ComputationDescriptor, ProjectionError>;

    /// Recognize Form from Storage manifestation (Avidyā: Unity from Gross)
    ///
    /// Given a StorageDescriptor, infer the PropertyDescriptor that must
    /// have given rise to it. This is the inverse of `project_to_storage()`.
    fn recognize_from_storage(
        &self,
        storage: &StorageDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError>;

    /// Recognize Form from Computation manifestation (Avidyā: Unity from Subtle)
    ///
    /// Given a ComputationDescriptor, infer the PropertyDescriptor that must
    /// have given rise to it. This is the inverse of `project_to_computation()`.
    fn recognize_from_computation(
        &self,
        computation: &ComputationDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError>;

    /// Validate dialectical consistency (Brahman: Knowing Maya and Īśvara)
    ///
    /// Ensures that the Form, Storage, and Computation descriptors are
    /// mutually consistent - that they represent a coherent projection.
    fn validate_projection(
        &self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor,
    ) -> Result<(), ProjectionError>;

    /// Helper: Project to both extremes at once
    fn project_to_extremes(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<(StorageDescriptor, ComputationDescriptor), ProjectionError> {
        let storage = self.project_to_storage(form)?;
        let computation = self.project_to_computation(form)?;
        self.validate_projection(form, &storage, &computation)?;
        Ok((storage, computation))
    }

    /// Helper: Get projector name for debugging
    fn projector_name(&self) -> &'static str;
}

// ============================================================================
// HugeArray Projector - Chunked, Dense, Sequential
// ============================================================================

/// Type projector optimized for HugeArray backend
///
/// Projects to chunked, dense, sequential storage with fixed-width values.
/// Optimal for: bulk scans, dense graphs, cursor-based iteration.
#[derive(Debug, Clone, Default)]
pub struct HugeArrayProjector {
    /// Optional override for chunk size
    pub chunk_size: Option<usize>,
}

impl HugeArrayProjector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self {
            chunk_size: Some(chunk_size),
        }
    }
}

impl TypeProjector for HugeArrayProjector {
    fn project_to_storage(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<StorageDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, ConcurrencyModel, Density, PersistenceConfig,
            StorageLayout,
        };

        // Vidyā: Reveal Form as Storage (Gross/Sthūla)
        // HugeArray = chunked, dense, sequential, immutable

        let page_size = self.chunk_size.unwrap_or(4096); // Default to 4KB pages

        // Infer density from value type
        let density = match form.value_type {
            ValueType::Long | ValueType::Double | ValueType::Boolean => Density::Dense,
            ValueType::LongArray | ValueType::DoubleArray => Density::Mixed,
            ValueType::String => Density::Mixed,
            _ => Density::Dense,
        };

        let descriptor = StorageDescriptor::new(
            form.id,
            format!("{}_storage", form.name),
            BackendTechnology::HugeArray { page_size },
        )
        .with_layout(StorageLayout::Chunked)
        .with_density(density)
        .with_access_pattern(AccessPattern::Sequential)
        .with_concurrency(ConcurrencyModel::ReadOnly)
        .with_persistence(PersistenceConfig::ephemeral())
        .with_page_size(page_size);

        Ok(descriptor)
    }

    fn project_to_computation(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<ComputationDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };

        // Vidyā: Reveal Form as Computation (Subtle/Sūkṣma)
        // HugeArray storage → BSP computation with VertexCentric pattern

        // Infer computation pattern from property context
        // For now, assume node properties → VertexCentric
        // In future, could inspect property name or metadata
        let pattern = ComputationPattern::VertexCentric;

        let descriptor = ComputationDescriptor::new(
            form.id,
            format!("{}_computation", form.name),
            ComputationSpecies::Bsp, // HugeArray pairs well with BSP/Pregel
            pattern,
        )
        .with_description(format!("BSP computation for property '{}'", form.name));

        Ok(descriptor)
    }

    fn recognize_from_storage(
        &self,
        storage: &StorageDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::BackendTechnology;

        // Avidyā: Recognize Form from Storage manifestation
        // Inverse projection: StorageDescriptor → PropertyDescriptor

        // Validate this is a HugeArray backend
        if !matches!(storage.backend, BackendTechnology::HugeArray { .. }) {
            return Err(ProjectionError::IncompatibleTypes {
                source: "StorageDescriptor".to_string(),
                target: "PropertyDescriptor".to_string(),
                reason: format!(
                    "HugeArrayProjector can only recognize HugeArray backends, got {:?}",
                    storage.backend
                ),
            });
        }

        // Infer value type from compatible types (pick first if multiple)
        let value_type = storage.compatible_types.first().copied().ok_or_else(|| {
            ProjectionError::MissingMetadata {
                descriptor_type: "StorageDescriptor".to_string(),
                field: "compatible_types".to_string(),
            }
        })?;

        // Extract property name (strip "_storage" suffix if present)
        let name = storage
            .name
            .strip_suffix("_storage")
            .unwrap_or(&storage.name)
            .to_string();

        let descriptor = PropertyDescriptor::new(storage.id, name, value_type)
            .with_storage_hint(
                crate::projection::codegen::descriptors::property::StorageHint::FixedWidth,
            )
            .with_nullable(true);

        Ok(descriptor)
    }

    fn recognize_from_computation(
        &self,
        computation: &ComputationDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::computation::ComputationSpecies;

        // Avidyā: Recognize Form from Computation manifestation
        // Inverse projection: ComputationDescriptor → PropertyDescriptor

        // Validate this is a BSP computation (what HugeArray expects)
        if !matches!(computation.species, ComputationSpecies::Bsp) {
            return Err(ProjectionError::IncompatibleTypes {
                source: "ComputationDescriptor".to_string(),
                target: "PropertyDescriptor".to_string(),
                reason: format!(
                    "HugeArrayProjector expects BSP species, got {:?}",
                    computation.species
                ),
            });
        }

        // Extract property name (strip "_computation" suffix if present)
        let name = computation
            .name
            .strip_suffix("_computation")
            .unwrap_or(&computation.name)
            .to_string();

        // Default to Long type (most common for graph properties)
        // In a real system, we'd need more metadata to infer the exact type
        let descriptor = PropertyDescriptor::new(computation.id, name, ValueType::Long)
            .with_storage_hint(
                crate::projection::codegen::descriptors::property::StorageHint::FixedWidth,
            )
            .with_nullable(true);

        Ok(descriptor)
    }

    fn validate_projection(
        &self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor,
    ) -> Result<(), ProjectionError> {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, StorageLayout,
        };

        // Brahman: Validate dialectical consistency
        // All three descriptors must be mutually consistent

        // 1. Validate IDs match
        if form.id != storage.id || form.id != computation.id {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ID mismatch".to_string(),
                reason: format!(
                    "PropertyDescriptor id={}, StorageDescriptor id={}, ComputationDescriptor id={}",
                    form.id, storage.id, computation.id
                ),
            });
        }

        // 2. Validate storage backend is HugeArray
        if !matches!(storage.backend, BackendTechnology::HugeArray { .. }) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "HugeArrayProjector requires HugeArray backend, got {:?}",
                    storage.backend
                ),
            });
        }

        // 3. Validate storage layout is Chunked
        if storage.layout != StorageLayout::Chunked {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "HugeArray requires Chunked layout, got {:?}",
                    storage.layout
                ),
            });
        }

        // 4. Validate computation species is BSP
        if !matches!(computation.species, ComputationSpecies::Bsp) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ComputationDescriptor".to_string(),
                reason: format!(
                    "HugeArray pairs with BSP computation, got {:?}",
                    computation.species
                ),
            });
        }

        // 5. Validate access pattern compatibility
        // Sequential storage → VertexCentric computation is optimal
        match (storage.memory_profile.access_pattern, &computation.pattern) {
            (AccessPattern::Sequential, ComputationPattern::VertexCentric) => {
                // Perfect match
            }
            (AccessPattern::Sequential, ComputationPattern::Global) => {
                // Acceptable - global computations work with sequential
            }
            (pattern, comp_pattern) => {
                // Warning: potentially suboptimal but not invalid
                eprintln!(
                    "Warning: Storage pattern {:?} may not be optimal for computation pattern {:?}",
                    pattern, comp_pattern
                );
            }
        }

        // 6. Validate value type is in compatible list
        if !storage.compatible_types.contains(&form.value_type) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "value_type compatibility".to_string(),
                reason: format!(
                    "Value type {:?} not in storage compatible types: {:?}",
                    form.value_type, storage.compatible_types
                ),
            });
        }

        // All validations passed - the projection is consistent
        Ok(())
    }

    fn projector_name(&self) -> &'static str {
        "HugeArrayProjector"
    }
}

// ============================================================================
// Arrow Projector - Columnar, Zero-Copy, Batch
// ============================================================================

/// Type projector optimized for Arrow backend
///
/// Projects to columnar, zero-copy storage with batch processing.
/// Optimal for: OLAP queries, bulk exports, mmap-friendly workloads.
#[derive(Debug, Clone, Default)]
pub struct ArrowProjector {
    /// Optional override for batch size
    pub batch_size: Option<usize>,
}

impl ArrowProjector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_batch_size(batch_size: usize) -> Self {
        Self {
            batch_size: Some(batch_size),
        }
    }
}

impl TypeProjector for ArrowProjector {
    fn project_to_storage(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<StorageDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, ConcurrencyModel, Density, Mutability,
            PersistenceConfig, StorageLayout,
        };

        // Vidyā: Reveal Form as Arrow Storage (Columnar/Zero-Copy)
        // Arrow = columnar, batch-oriented, mmap-friendly

        // Infer density - Arrow handles sparse well with null bitmaps
        let density = match form.value_type {
            ValueType::Long | ValueType::Double | ValueType::Boolean => Density::Dense,
            ValueType::String | ValueType::LongArray | ValueType::DoubleArray => Density::Mixed,
            _ => Density::Mixed,
        };

        let descriptor = StorageDescriptor::new(
            form.id,
            format!("{}_storage", form.name),
            BackendTechnology::Arrow {},
        )
        .with_layout(StorageLayout::Columnar)
        .with_density(density)
        .with_access_pattern(AccessPattern::Batch) // Arrow excels at batch ops
        .with_concurrency(ConcurrencyModel::ReadOnly)
        .with_persistence(PersistenceConfig::ephemeral());

        // Arrow supports immutable zero-copy access
        let mut desc = descriptor;
        desc.memory_profile.mutability = Mutability::Immutable;

        Ok(desc)
    }

    fn project_to_computation(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<ComputationDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };

        // Vidyā: Reveal Form as Dataflow Computation (Batch/Columnar)
        // Arrow storage → Dataflow computation with batch pattern

        // Arrow's columnar nature pairs well with dataflow/MapReduce
        let descriptor = ComputationDescriptor::new(
            form.id,
            format!("{}_computation", form.name),
            ComputationSpecies::Dataflow, // Arrow → Dataflow processing
            ComputationPattern::Global,   // Batch operations are typically global
        )
        .with_description(format!(
            "Dataflow computation for columnar property '{}'",
            form.name
        ));

        Ok(descriptor)
    }

    fn recognize_from_storage(
        &self,
        storage: &StorageDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::BackendTechnology;

        // Avidyā: Recognize Form from Arrow Storage manifestation

        // Validate this is an Arrow backend
        if !matches!(storage.backend, BackendTechnology::Arrow { .. }) {
            return Err(ProjectionError::IncompatibleTypes {
                source: "StorageDescriptor".to_string(),
                target: "PropertyDescriptor".to_string(),
                reason: format!(
                    "ArrowProjector can only recognize Arrow backends, got {:?}",
                    storage.backend
                ),
            });
        }

        // Infer value type from compatible types
        let value_type = storage.compatible_types.first().copied().ok_or_else(|| {
            ProjectionError::MissingMetadata {
                descriptor_type: "StorageDescriptor".to_string(),
                field: "compatible_types".to_string(),
            }
        })?;

        // Extract property name (strip "_storage" suffix if present)
        let name = storage
            .name
            .strip_suffix("_storage")
            .unwrap_or(&storage.name)
            .to_string();

        let descriptor = PropertyDescriptor::new(storage.id, name, value_type)
            .with_storage_hint(
                crate::projection::codegen::descriptors::property::StorageHint::VariableLength,
            )
            .with_nullable(true);

        Ok(descriptor)
    }

    fn recognize_from_computation(
        &self,
        computation: &ComputationDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::computation::ComputationSpecies;

        // Avidyā: Recognize Form from Dataflow Computation manifestation

        // Validate this is a Dataflow computation (what Arrow expects)
        if !matches!(
            computation.species,
            ComputationSpecies::Dataflow | ComputationSpecies::MapReduce
        ) {
            return Err(ProjectionError::IncompatibleTypes {
                source: "ComputationDescriptor".to_string(),
                target: "PropertyDescriptor".to_string(),
                reason: format!(
                    "ArrowProjector expects Dataflow/MapReduce species, got {:?}",
                    computation.species
                ),
            });
        }

        // Extract property name (strip "_computation" suffix if present)
        let name = computation
            .name
            .strip_suffix("_computation")
            .unwrap_or(&computation.name)
            .to_string();

        // Default to Double type (common for analytical workloads)
        let descriptor = PropertyDescriptor::new(computation.id, name, ValueType::Double)
            .with_storage_hint(
                crate::projection::codegen::descriptors::property::StorageHint::VariableLength,
            )
            .with_nullable(true);

        Ok(descriptor)
    }

    fn validate_projection(
        &self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor,
    ) -> Result<(), ProjectionError> {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, StorageLayout,
        };

        // Brahman: Validate dialectical consistency for Arrow

        // 1. Validate IDs match
        if form.id != storage.id || form.id != computation.id {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ID mismatch".to_string(),
                reason: format!(
                    "PropertyDescriptor id={}, StorageDescriptor id={}, ComputationDescriptor id={}",
                    form.id, storage.id, computation.id
                ),
            });
        }

        // 2. Validate storage backend is Arrow
        if !matches!(storage.backend, BackendTechnology::Arrow { .. }) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "ArrowProjector requires Arrow backend, got {:?}",
                    storage.backend
                ),
            });
        }

        // 3. Validate storage layout is Columnar
        if storage.layout != StorageLayout::Columnar {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!("Arrow requires Columnar layout, got {:?}", storage.layout),
            });
        }

        // 4. Validate computation species is Dataflow or MapReduce
        if !matches!(
            computation.species,
            ComputationSpecies::Dataflow | ComputationSpecies::MapReduce
        ) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ComputationDescriptor".to_string(),
                reason: format!(
                    "Arrow pairs with Dataflow/MapReduce computation, got {:?}",
                    computation.species
                ),
            });
        }

        // 5. Validate access pattern compatibility
        // Batch storage → Global computation is optimal
        match (storage.memory_profile.access_pattern, &computation.pattern) {
            (AccessPattern::Batch, ComputationPattern::Global) => {
                // Perfect match
            }
            (AccessPattern::Batch, _) => {
                // Acceptable - batch can support various patterns
            }
            (pattern, comp_pattern) => {
                // Warning: potentially suboptimal
                eprintln!(
                    "Warning: Storage pattern {:?} may not be optimal for computation pattern {:?}",
                    pattern, comp_pattern
                );
            }
        }

        // 6. Validate value type is in compatible list
        if !storage.compatible_types.contains(&form.value_type) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "value_type compatibility".to_string(),
                reason: format!(
                    "Value type {:?} not in storage compatible types: {:?}",
                    form.value_type, storage.compatible_types
                ),
            });
        }

        // All validations passed
        Ok(())
    }

    fn projector_name(&self) -> &'static str {
        "ArrowProjector"
    }
}

// ============================================================================
// Pregel Projector - Vertex-Centric, Message-Passing
// ============================================================================

/// Type projector optimized for Pregel/BSP computation
///
/// Projects to vertex-centric computation with message-passing semantics.
/// Optimal for: graph algorithms, iterative computations, distributed processing.
#[derive(Debug, Clone, Default)]
pub struct PregelProjector {
    /// Whether to optimize for distributed execution
    pub distributed: bool,
}

impl PregelProjector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn distributed() -> Self {
        Self { distributed: true }
    }
}

impl TypeProjector for PregelProjector {
    fn project_to_storage(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<StorageDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, ConcurrencyModel, Mutability, PersistenceConfig,
            StorageLayout,
        };

        // Vidyā: Reveal Form as Storage for Pregel vertex-centric computation

        // Choose backend based on distributed flag
        let backend = if self.distributed {
            BackendTechnology::Arrow {} // Columnar, distributed-friendly
        } else {
            BackendTechnology::HugeArray { page_size: 4096 }
        };

        let persistence = if self.distributed {
            PersistenceConfig::durable() // Distributed needs durable storage
        } else {
            PersistenceConfig::ephemeral()
        };

        let mut descriptor =
            StorageDescriptor::new(form.id, format!("{}_pregel_storage", form.name), backend)
                .with_layout(StorageLayout::Hybrid) // Vertices distributed + partitioned
                .with_access_pattern(AccessPattern::VertexCentric) // Key: vertex-centric access
                .with_concurrency(ConcurrencyModel::LockBased) // Supports concurrent updates
                .with_persistence(persistence);

        // Set mutability on memory profile
        descriptor.memory_profile.mutability = Mutability::Mutable;

        Ok(descriptor)
    }

    fn project_to_computation(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<ComputationDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };

        // Vidyā: Reveal Form as Computation (Subtle/Sūkṣma)
        // Pregel = BSP, vertex-centric message-passing

        let descriptor = ComputationDescriptor::new(
            form.id,
            format!("{}_pregel_computation", form.name),
            ComputationSpecies::Bsp,           // Bulk Synchronous Parallel
            ComputationPattern::VertexCentric, // Message-passing between vertices
        )
        .with_description(format!(
            "Pregel/BSP vertex-centric computation for property '{}' (distributed={})",
            form.name, self.distributed
        ));

        Ok(descriptor)
    }

    fn recognize_from_storage(
        &self,
        storage: &StorageDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, StorageLayout,
        };

        // Avidyā: Recognize Form from Storage manifestation

        // Validate this looks like Pregel storage
        match (
            &storage.backend,
            storage.layout,
            storage.memory_profile.access_pattern,
        ) {
            (
                BackendTechnology::HugeArray { .. },
                StorageLayout::Hybrid,
                AccessPattern::VertexCentric,
            ) => {}
            (BackendTechnology::Arrow {}, StorageLayout::Hybrid, AccessPattern::VertexCentric) => {}
            _ => {
                return Err(ProjectionError::IncompatibleTypes {
                    source: format!(
                        "StorageDescriptor(backend={:?}, layout={:?}, access={:?})",
                        storage.backend, storage.layout, storage.memory_profile.access_pattern
                    ),
                    target: "PregelProjector".to_string(),
                    reason: "Expected Partitioned layout with VertexCentric access".to_string(),
                });
            }
        }

        // Extract property descriptor
        Ok(PropertyDescriptor::new(
            storage.id,
            storage.name.clone().replace("_pregel_storage", ""),
            ValueType::Long, // Default, can be refined
        )
        .with_storage_hint(StorageHint::FixedWidth) // Pregel typically uses fixed-width
        .with_nullable(false)) // Vertex properties typically non-null
    }

    fn recognize_from_computation(
        &self,
        computation: &ComputationDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        use crate::projection::codegen::descriptors::property::StorageHint;

        // Avidyā: Recognize Form from Computation manifestation

        // Validate this is BSP/Pregel computation
        if computation.species != ComputationSpecies::Bsp {
            return Err(ProjectionError::IncompatibleTypes {
                source: format!("ComputationDescriptor(species={:?})", computation.species),
                target: "PregelProjector".to_string(),
                reason: "Expected BSP computation species".to_string(),
            });
        }

        if computation.pattern != ComputationPattern::VertexCentric {
            return Err(ProjectionError::IncompatibleTypes {
                source: format!("ComputationDescriptor(pattern={:?})", computation.pattern),
                target: "PregelProjector".to_string(),
                reason: "Expected VertexCentric computation pattern".to_string(),
            });
        }

        // Extract property descriptor
        Ok(PropertyDescriptor::new(
            computation.id,
            computation.name.clone().replace("_pregel_computation", ""),
            ValueType::Long, // Default for vertex properties
        )
        .with_storage_hint(StorageHint::FixedWidth)
        .with_nullable(false))
    }

    fn validate_projection(
        &self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor,
    ) -> Result<(), ProjectionError> {
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, StorageLayout,
        };

        // Brahman: Validate dialectical consistency for Pregel

        // 1. Validate IDs match
        if form.id != storage.id || form.id != computation.id {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ID mismatch".to_string(),
                reason: format!(
                    "PropertyDescriptor id={}, StorageDescriptor id={}, ComputationDescriptor id={}",
                    form.id, storage.id, computation.id
                ),
            });
        }

        // 2. Validate storage layout is Hybrid (partitioned + distributed)
        if storage.layout != StorageLayout::Hybrid {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "PregelProjector requires Hybrid layout, got {:?}",
                    storage.layout
                ),
            });
        }

        // 3. Validate access pattern is VertexCentric
        if storage.memory_profile.access_pattern != AccessPattern::VertexCentric {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "Pregel requires VertexCentric access pattern, got {:?}",
                    storage.memory_profile.access_pattern
                ),
            });
        }

        // 4. Validate computation species is BSP
        if computation.species != ComputationSpecies::Bsp {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ComputationDescriptor".to_string(),
                reason: format!(
                    "Pregel requires BSP computation species, got {:?}",
                    computation.species
                ),
            });
        }

        // 5. Validate computation pattern is VertexCentric
        if computation.pattern != ComputationPattern::VertexCentric {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "ComputationDescriptor".to_string(),
                reason: format!(
                    "Pregel requires VertexCentric computation pattern, got {:?}",
                    computation.pattern
                ),
            });
        }

        // 6. Validate backend technology is compatible
        if !matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. } | BackendTechnology::Arrow {}
        ) {
            return Err(ProjectionError::ValidationFailed {
                descriptor: "StorageDescriptor".to_string(),
                reason: format!(
                    "Pregel requires HugeArray or Arrow backend, got {:?}",
                    storage.backend
                ),
            });
        }

        // All validations passed
        Ok(())
    }

    fn projector_name(&self) -> &'static str {
        "PregelProjector"
    }
}

// ============================================================================
// Adaptive Projector - Runtime Profiling
// ============================================================================

/// Adaptive type projector that learns optimal projections from runtime profiling
///
/// Uses runtime data to adaptively choose projection strategies.
/// This is Maya in its most dynamic form - revealing and concealing based
/// on actual behavior rather than static declarations.
///
/// **Philosophical Position**: Being-and-NotBeing itself - the dialectical superior
/// to all fixed projectors. It IS HugeArray when workload is sequential, IS Arrow
/// when workload is batch-oriented, IS Pregel when workload is vertex-centric.
/// It negates itself continuously by becoming what the workload demands.
#[derive(Debug, Clone)]
pub struct AdaptiveProjector {
    /// Learned workload characteristics
    metrics: Option<WorkloadMetrics>,
    /// Strategy bias (0.0 = pure learning, 1.0 = always use default)
    conservatism: f64,
}

impl Default for AdaptiveProjector {
    fn default() -> Self {
        Self {
            metrics: None,
            conservatism: 0.3, // 30% conservative by default
        }
    }
}

impl AdaptiveProjector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom conservatism (0.0 = aggressive learning, 1.0 = conservative)
    pub fn with_conservatism(conservatism: f64) -> Self {
        Self {
            metrics: None,
            conservatism: conservatism.clamp(0.0, 1.0),
        }
    }

    /// Observe runtime behavior and adapt projection strategy
    ///
    /// This IS Maya learning itself - the projector observing actual workload
    /// and dynamically choosing the optimal projection strategy.
    pub fn observe_workload(&mut self, metrics: &WorkloadMetrics) {
        self.metrics = Some(metrics.clone());
    }

    /// Choose optimal projector based on observed metrics
    fn choose_delegate(&self) -> Box<dyn TypeProjector> {
        match &self.metrics {
            None => {
                // No metrics yet - default to HugeArray (safe choice)
                Box::new(HugeArrayProjector::new())
            }
            Some(m) => {
                // Decide based on workload characteristics

                // High write ratio + vertex-centric → Pregel
                if m.write_ratio > 0.3 && m.sequential_access_ratio < 0.5 {
                    return Box::new(PregelProjector::new());
                }

                // Large batches + high read ratio → Arrow
                if m.average_batch_size > 1000 && m.read_ratio > 0.8 {
                    return Box::new(ArrowProjector::new());
                }

                // Sequential access + dense → HugeArray
                if m.sequential_access_ratio > 0.7 {
                    return Box::new(HugeArrayProjector::new());
                }

                // Mixed workload - use conservatism to decide
                if self.conservatism > 0.5 {
                    Box::new(HugeArrayProjector::new()) // Safe default
                } else {
                    // Aggressive - choose based on strongest signal
                    if m.write_ratio > m.read_ratio {
                        Box::new(PregelProjector::new())
                    } else if m.average_batch_size > 100 {
                        Box::new(ArrowProjector::new())
                    } else {
                        Box::new(HugeArrayProjector::new())
                    }
                }
            }
        }
    }
}

/// Workload metrics for adaptive projection
#[derive(Debug, Clone)]
pub struct WorkloadMetrics {
    pub read_ratio: f64,
    pub write_ratio: f64,
    pub sequential_access_ratio: f64,
    pub cache_hit_ratio: f64,
    pub average_batch_size: usize,
}

impl TypeProjector for AdaptiveProjector {
    fn project_to_storage(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<StorageDescriptor, ProjectionError> {
        // Being-and-NotBeing: Become what the workload demands
        let delegate = self.choose_delegate();
        delegate.project_to_storage(form)
    }

    fn project_to_computation(
        &self,
        form: &PropertyDescriptor,
    ) -> Result<ComputationDescriptor, ProjectionError> {
        // Maya revealing through learned patterns
        let delegate = self.choose_delegate();
        delegate.project_to_computation(form)
    }

    fn recognize_from_storage(
        &self,
        storage: &StorageDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        // Try each delegate until one recognizes the pattern
        // This IS Avidyā in its purest form - recognizing through trial

        let delegates: Vec<Box<dyn TypeProjector>> = vec![
            Box::new(HugeArrayProjector::new()),
            Box::new(ArrowProjector::new()),
            Box::new(PregelProjector::new()),
        ];

        for delegate in delegates {
            if let Ok(prop) = delegate.recognize_from_storage(storage) {
                return Ok(prop);
            }
        }

        Err(ProjectionError::IncompatibleTypes {
            source: format!("StorageDescriptor(backend={:?})", storage.backend),
            target: "AdaptiveProjector".to_string(),
            reason: "No delegate could recognize this storage pattern".to_string(),
        })
    }

    fn recognize_from_computation(
        &self,
        computation: &ComputationDescriptor,
    ) -> Result<PropertyDescriptor, ProjectionError> {
        // Try each delegate until one recognizes the pattern

        let delegates: Vec<Box<dyn TypeProjector>> = vec![
            Box::new(HugeArrayProjector::new()),
            Box::new(ArrowProjector::new()),
            Box::new(PregelProjector::new()),
        ];

        for delegate in delegates {
            if let Ok(prop) = delegate.recognize_from_computation(computation) {
                return Ok(prop);
            }
        }

        Err(ProjectionError::IncompatibleTypes {
            source: format!("ComputationDescriptor(species={:?})", computation.species),
            target: "AdaptiveProjector".to_string(),
            reason: "No delegate could recognize this computation pattern".to_string(),
        })
    }

    fn validate_projection(
        &self,
        form: &PropertyDescriptor,
        storage: &StorageDescriptor,
        computation: &ComputationDescriptor,
    ) -> Result<(), ProjectionError> {
        // Adaptive validation: delegate to the projector that would have created this
        let delegate = self.choose_delegate();
        delegate.validate_projection(form, storage, computation)
    }

    fn projector_name(&self) -> &'static str {
        "AdaptiveProjector"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projector_trait_object() {
        let _projector: Box<dyn TypeProjector> = Box::new(HugeArrayProjector::new());
        // Verify trait object works
    }

    #[test]
    fn test_huge_array_projector_creation() {
        let projector = HugeArrayProjector::new();
        assert_eq!(projector.projector_name(), "HugeArrayProjector");
        assert!(projector.chunk_size.is_none());

        let projector_with_size = HugeArrayProjector::with_chunk_size(8192);
        assert_eq!(projector_with_size.chunk_size, Some(8192));
    }

    #[test]
    fn test_arrow_projector_creation() {
        let projector = ArrowProjector::new();
        assert_eq!(projector.projector_name(), "ArrowProjector");
        assert!(projector.batch_size.is_none());

        let projector_with_size = ArrowProjector::with_batch_size(1024);
        assert_eq!(projector_with_size.batch_size, Some(1024));
    }

    #[test]
    fn test_pregel_projector_creation() {
        let projector = PregelProjector::new();
        assert_eq!(projector.projector_name(), "PregelProjector");
        assert!(!projector.distributed);

        let distributed = PregelProjector::distributed();
        assert!(distributed.distributed);
    }

    #[test]
    fn test_adaptive_projector_creation() {
        let projector = AdaptiveProjector::new();
        assert_eq!(projector.projector_name(), "AdaptiveProjector");
    }

    #[test]
    fn test_projection_error_display() {
        let error = ProjectionError::IncompatibleTypes {
            source: "Long".to_string(),
            target: "String".to_string(),
            reason: "Type mismatch".to_string(),
        };
        assert!(error.to_string().contains("Incompatible type mapping"));

        let error = ProjectionError::MissingMetadata {
            descriptor_type: "PropertyDescriptor".to_string(),
            field: "storage_hint".to_string(),
        };
        assert!(error.to_string().contains("Missing metadata"));
    }

    // ========================================================================
    // HugeArrayProjector Tests
    // ========================================================================

    #[test]
    fn test_huge_array_project_long_to_storage() {
        use crate::projection::codegen::descriptors::property::StorageHint;
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, Density, StorageLayout,
        };

        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(42, "pagerank", ValueType::Long)
            .with_storage_hint(StorageHint::FixedWidth);

        let storage = projector.project_to_storage(&form).unwrap();

        // Validate Vidyā: Form revealed as Storage
        assert_eq!(storage.id, 42);
        assert_eq!(storage.name, "pagerank_storage");
        assert!(matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. }
        ));
        assert_eq!(storage.layout, StorageLayout::Chunked);
        assert_eq!(storage.memory_profile.density, Density::Dense);
        assert_eq!(
            storage.memory_profile.access_pattern,
            AccessPattern::Sequential
        );
        assert!(storage.compatible_types.contains(&ValueType::Long));
    }

    #[test]
    fn test_huge_array_project_to_computation() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };

        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(42, "pagerank", ValueType::Double);

        let computation = projector.project_to_computation(&form).unwrap();

        // Validate Vidyā: Form revealed as Computation
        assert_eq!(computation.id, 42);
        assert_eq!(computation.name, "pagerank_computation");
        assert!(matches!(computation.species, ComputationSpecies::Bsp));
        assert!(matches!(
            computation.pattern,
            ComputationPattern::VertexCentric
        ));
    }

    #[test]
    fn test_huge_array_project_to_extremes() {
        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(100, "score", ValueType::Double);

        // Maya: Reveal both extremes at once
        let (storage, computation) = projector.project_to_extremes(&form).unwrap();

        // Both extremes should have matching IDs
        assert_eq!(storage.id, 100);
        assert_eq!(computation.id, 100);
    }

    #[test]
    fn test_huge_array_recognize_from_storage() {
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = HugeArrayProjector::new();

        // Create a storage descriptor
        let storage = StorageDescriptor::new(
            42,
            "test_property_storage",
            BackendTechnology::HugeArray { page_size: 4096 },
        );

        // Avidyā: Recognize Form from Storage
        let form = projector.recognize_from_storage(&storage).unwrap();

        assert_eq!(form.id, 42);
        assert_eq!(form.name, "test_property"); // "_storage" suffix stripped
        assert!(storage.compatible_types.contains(&form.value_type));
    }

    #[test]
    fn test_huge_array_recognize_from_storage_wrong_backend() {
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = HugeArrayProjector::new();

        // Create an Arrow storage descriptor
        let storage =
            StorageDescriptor::new(42, "test_property_storage", BackendTechnology::Arrow {});

        // Should fail - wrong backend type
        let result = projector.recognize_from_storage(&storage);
        assert!(result.is_err());

        match result.unwrap_err() {
            ProjectionError::IncompatibleTypes { .. } => { /* expected */ }
            other => panic!("Expected IncompatibleTypes error, got {:?}", other),
        }
    }

    #[test]
    fn test_huge_array_recognize_from_computation() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };

        let projector = HugeArrayProjector::new();

        // Create a BSP computation descriptor
        let computation = ComputationDescriptor::new(
            42,
            "test_property_computation",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        // Avidyā: Recognize Form from Computation
        let form = projector.recognize_from_computation(&computation).unwrap();

        assert_eq!(form.id, 42);
        assert_eq!(form.name, "test_property"); // "_computation" suffix stripped
    }

    #[test]
    fn test_huge_array_validate_consistent_projection() {
        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(42, "test", ValueType::Long);

        // Project to both extremes
        let storage = projector.project_to_storage(&form).unwrap();
        let computation = projector.project_to_computation(&form).unwrap();

        // Brahman: Validate consistency
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_ok(), "Valid projection should pass validation");
    }

    #[test]
    fn test_huge_array_validate_id_mismatch() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(42, "test", ValueType::Long);

        let storage = StorageDescriptor::new(
            43, // Wrong ID!
            "test_storage",
            BackendTechnology::HugeArray { page_size: 4096 },
        );

        let computation = ComputationDescriptor::new(
            42,
            "test_computation",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        // Should fail - ID mismatch
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_err());

        match result.unwrap_err() {
            ProjectionError::ValidationFailed { descriptor, .. } => {
                assert!(descriptor.contains("ID mismatch"));
            }
            other => panic!("Expected ValidationFailed error, got {:?}", other),
        }
    }

    #[test]
    fn test_huge_array_validate_wrong_backend() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = HugeArrayProjector::new();
        let form = PropertyDescriptor::new(42, "test", ValueType::Long);

        let storage = StorageDescriptor::new(
            42,
            "test_storage",
            BackendTechnology::Arrow {}, // Wrong backend!
        );

        let computation = ComputationDescriptor::new(
            42,
            "test_computation",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        // Should fail - wrong backend
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_err());

        match result.unwrap_err() {
            ProjectionError::ValidationFailed { reason, .. } => {
                assert!(reason.contains("HugeArray"));
            }
            other => panic!("Expected ValidationFailed error, got {:?}", other),
        }
    }

    #[test]
    fn test_huge_array_roundtrip_through_storage() {
        let projector = HugeArrayProjector::new();
        let original = PropertyDescriptor::new(42, "test_prop", ValueType::Long);

        // Forward projection
        let storage = projector.project_to_storage(&original).unwrap();

        // Inverse projection (Avidyā: recognizing unity)
        let recognized = projector.recognize_from_storage(&storage).unwrap();

        // Verify essential properties preserved
        assert_eq!(original.id, recognized.id);
        assert_eq!(
            original.name,
            recognized
                .name
                .strip_suffix("_storage")
                .unwrap_or(&recognized.name)
        );
        // Note: value_type should match (first in compatible_types)
    }

    #[test]
    fn test_huge_array_custom_chunk_size() {
        let projector = HugeArrayProjector::with_chunk_size(8192);
        let form = PropertyDescriptor::new(42, "test", ValueType::Long);

        let storage = projector.project_to_storage(&form).unwrap();

        // Verify custom chunk size is used
        if let crate::projection::codegen::descriptors::storage::BackendTechnology::HugeArray {
            page_size,
        } = storage.backend
        {
            assert_eq!(page_size, 8192);
        } else {
            panic!("Expected HugeArray backend");
        }
    }

    // ========================================================================
    // ArrowProjector Tests (1st Middle Finger of Dakṣiṇāmūrti)
    // ========================================================================

    #[test]
    fn test_arrow_project_to_storage() {
        use crate::projection::codegen::descriptors::property::StorageHint;
        use crate::projection::codegen::descriptors::storage::{
            AccessPattern, BackendTechnology, StorageLayout,
        };

        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(100, "analytics_score", ValueType::Double)
            .with_storage_hint(StorageHint::VariableLength);

        let storage = projector.project_to_storage(&form).unwrap();

        // Validate Vidyā: Form revealed as Arrow Storage (Columnar)
        assert_eq!(storage.id, 100);
        assert_eq!(storage.name, "analytics_score_storage");
        assert!(matches!(storage.backend, BackendTechnology::Arrow { .. }));
        assert_eq!(storage.layout, StorageLayout::Columnar);
        assert_eq!(storage.memory_profile.access_pattern, AccessPattern::Batch);
        assert!(storage.compatible_types.contains(&ValueType::Double));
    }

    #[test]
    fn test_arrow_project_to_computation() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationPattern, ComputationSpecies,
        };

        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(100, "analytics_score", ValueType::Double);

        let computation = projector.project_to_computation(&form).unwrap();

        // Validate Vidyā: Form revealed as Dataflow Computation
        assert_eq!(computation.id, 100);
        assert_eq!(computation.name, "analytics_score_computation");
        assert!(matches!(computation.species, ComputationSpecies::Dataflow));
        assert!(matches!(computation.pattern, ComputationPattern::Global));
    }

    #[test]
    fn test_arrow_project_to_extremes() {
        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(200, "metric", ValueType::Long);

        // Maya: Reveal both extremes at once
        let (storage, computation) = projector.project_to_extremes(&form).unwrap();

        // Both extremes should have matching IDs
        assert_eq!(storage.id, 200);
        assert_eq!(computation.id, 200);
    }

    #[test]
    fn test_arrow_recognize_from_storage() {
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = ArrowProjector::new();

        // Create an Arrow storage descriptor
        let storage =
            StorageDescriptor::new(100, "test_property_storage", BackendTechnology::Arrow {});

        // Avidyā: Recognize Form from Storage
        let form = projector.recognize_from_storage(&storage).unwrap();

        assert_eq!(form.id, 100);
        assert_eq!(form.name, "test_property");
        assert!(storage.compatible_types.contains(&form.value_type));
    }

    #[test]
    fn test_arrow_recognize_from_storage_wrong_backend() {
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = ArrowProjector::new();

        // Create a HugeArray storage descriptor (wrong backend)
        let storage = StorageDescriptor::new(
            100,
            "test_property_storage",
            BackendTechnology::HugeArray { page_size: 4096 },
        );

        // Should fail - wrong backend type
        let result = projector.recognize_from_storage(&storage);
        assert!(result.is_err());

        match result.unwrap_err() {
            ProjectionError::IncompatibleTypes { .. } => { /* expected */ }
            other => panic!("Expected IncompatibleTypes error, got {:?}", other),
        }
    }

    #[test]
    fn test_arrow_recognize_from_computation() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };

        let projector = ArrowProjector::new();

        // Create a Dataflow computation descriptor
        let computation = ComputationDescriptor::new(
            100,
            "test_property_computation",
            ComputationSpecies::Dataflow,
            ComputationPattern::Global,
        );

        // Avidyā: Recognize Form from Computation
        let form = projector.recognize_from_computation(&computation).unwrap();

        assert_eq!(form.id, 100);
        assert_eq!(form.name, "test_property");
    }

    #[test]
    fn test_arrow_recognize_from_computation_wrong_species() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };

        let projector = ArrowProjector::new();

        // Create a BSP computation descriptor (not Dataflow/MapReduce)
        let computation = ComputationDescriptor::new(
            100,
            "test_property_computation",
            ComputationSpecies::Bsp, // Wrong species!
            ComputationPattern::VertexCentric,
        );

        // Should fail - wrong species
        let result = projector.recognize_from_computation(&computation);
        assert!(result.is_err());

        match result.unwrap_err() {
            ProjectionError::IncompatibleTypes { .. } => { /* expected */ }
            other => panic!("Expected IncompatibleTypes error, got {:?}", other),
        }
    }

    #[test]
    fn test_arrow_validate_consistent_projection() {
        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(100, "test", ValueType::Double);

        // Project to both extremes
        let storage = projector.project_to_storage(&form).unwrap();
        let computation = projector.project_to_computation(&form).unwrap();

        // Brahman: Validate consistency
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(
            result.is_ok(),
            "Valid Arrow projection should pass validation"
        );
    }

    #[test]
    fn test_arrow_validate_wrong_backend() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor,
        };

        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(100, "test", ValueType::Double);

        let storage = StorageDescriptor::new(
            100,
            "test_storage",
            BackendTechnology::HugeArray { page_size: 4096 }, // Wrong backend!
        );

        let computation = ComputationDescriptor::new(
            100,
            "test_computation",
            ComputationSpecies::Dataflow,
            ComputationPattern::Global,
        );

        // Should fail - wrong backend
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_err());
    }

    #[test]
    fn test_arrow_validate_wrong_layout() {
        use crate::projection::codegen::descriptors::computation::{
            ComputationDescriptor, ComputationPattern, ComputationSpecies,
        };
        use crate::projection::codegen::descriptors::storage::{
            BackendTechnology, StorageDescriptor, StorageLayout,
        };

        let projector = ArrowProjector::new();
        let form = PropertyDescriptor::new(100, "test", ValueType::Double);

        let mut storage = StorageDescriptor::new(100, "test_storage", BackendTechnology::Arrow {});
        storage.layout = StorageLayout::Chunked; // Wrong layout for Arrow!

        let computation = ComputationDescriptor::new(
            100,
            "test_computation",
            ComputationSpecies::Dataflow,
            ComputationPattern::Global,
        );

        // Should fail - wrong layout
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_err());
    }

    #[test]
    fn test_arrow_roundtrip_through_storage() {
        let projector = ArrowProjector::new();
        let original = PropertyDescriptor::new(100, "test_prop", ValueType::Double);

        // Forward projection
        let storage = projector.project_to_storage(&original).unwrap();

        // Inverse projection (Avidyā: recognizing unity)
        let recognized = projector.recognize_from_storage(&storage).unwrap();

        // Verify essential properties preserved
        assert_eq!(original.id, recognized.id);
        assert_eq!(
            original.name,
            recognized
                .name
                .strip_suffix("_storage")
                .unwrap_or(&recognized.name)
        );
    }

    // ========================================================================
    // Additional Pregel Projector Tests (2nd Middle Finger)
    // ========================================================================

    #[test]
    fn test_pregel_project_to_storage() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(42, "vertex_score", ValueType::Double);

        let storage = projector.project_to_storage(&form).unwrap();

        assert_eq!(storage.id, 42);
        assert!(storage.name.contains("pregel_storage"));
        assert_eq!(storage.layout, StorageLayout::Hybrid);
        assert_eq!(
            storage.memory_profile.access_pattern,
            AccessPattern::VertexCentric
        );
        assert_eq!(storage.memory_profile.mutability, Mutability::Mutable);
        assert!(matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. }
        ));
    }

    #[test]
    fn test_pregel_project_to_storage_distributed() {
        let projector = PregelProjector::distributed();
        let form = PropertyDescriptor::new(43, "community_id", ValueType::Long);

        let storage = projector.project_to_storage(&form).unwrap();

        assert_eq!(storage.id, 43);
        assert_eq!(storage.layout, StorageLayout::Hybrid);
        assert!(matches!(storage.backend, BackendTechnology::Arrow {}));
    }

    #[test]
    fn test_pregel_project_to_computation() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(44, "rank", ValueType::Double);

        let computation = projector.project_to_computation(&form).unwrap();

        assert_eq!(computation.id, 44);
        assert!(computation.name.contains("pregel_computation"));
        assert_eq!(computation.species, ComputationSpecies::Bsp);
        assert_eq!(computation.pattern, ComputationPattern::VertexCentric);
    }

    #[test]
    fn test_pregel_recognize_from_storage() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(46, "component", ValueType::Long);

        let storage = projector.project_to_storage(&form).unwrap();
        let recognized = projector.recognize_from_storage(&storage).unwrap();

        assert_eq!(recognized.id, 46);
        assert_eq!(recognized.value_type, ValueType::Long);
        assert_eq!(recognized.storage_hint, StorageHint::FixedWidth);
    }

    #[test]
    fn test_pregel_recognize_from_storage_wrong_layout() {
        let projector = PregelProjector::new();

        let mut storage = StorageDescriptor::new(
            47,
            "test",
            BackendTechnology::Sparse {
                initial_capacity: 1000,
                load_factor: 0.75,
            },
        );
        storage.layout = StorageLayout::Sparse;

        let result = projector.recognize_from_storage(&storage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ProjectionError::IncompatibleTypes { .. }
        ));
    }

    #[test]
    fn test_pregel_recognize_from_computation() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(48, "distance", ValueType::Double);

        let computation = projector.project_to_computation(&form).unwrap();
        let recognized = projector.recognize_from_computation(&computation).unwrap();

        assert_eq!(recognized.id, 48);
        assert_eq!(recognized.value_type, ValueType::Long);
    }

    #[test]
    fn test_pregel_recognize_from_computation_wrong_species() {
        let projector = PregelProjector::new();

        let computation = ComputationDescriptor::new(
            49,
            "test",
            ComputationSpecies::Dataflow,
            ComputationPattern::Global,
        );

        let result = projector.recognize_from_computation(&computation);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ProjectionError::IncompatibleTypes { .. }
        ));
    }

    #[test]
    fn test_pregel_validate_consistent_projection() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(50, "centrality", ValueType::Double);

        let storage = projector.project_to_storage(&form).unwrap();
        let computation = projector.project_to_computation(&form).unwrap();

        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pregel_validate_wrong_layout() {
        let projector = PregelProjector::new();
        let form = PropertyDescriptor::new(51, "test", ValueType::Long);

        let mut storage = projector.project_to_storage(&form).unwrap();
        storage.layout = StorageLayout::Chunked;

        let computation = projector.project_to_computation(&form).unwrap();

        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ProjectionError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn test_pregel_roundtrip_through_storage() {
        let projector = PregelProjector::distributed();
        let original = PropertyDescriptor::new(52, "pagerank", ValueType::Double);

        let storage = projector.project_to_storage(&original).unwrap();
        let recognized = projector.recognize_from_storage(&storage).unwrap();

        assert_eq!(original.id, recognized.id);
        assert!(original.name.contains("pagerank"));
    }

    // ========================================================================
    // Adaptive Projector Tests (3rd Middle Finger - Being-and-NotBeing)
    // ========================================================================

    #[test]
    fn test_adaptive_default_to_hugearray() {
        // No metrics observed - should default to HugeArray (safe choice)
        let projector = AdaptiveProjector::new();
        let form = PropertyDescriptor::new(60, "default_prop", ValueType::Long);

        let storage = projector.project_to_storage(&form).unwrap();

        // Should choose HugeArray as default
        assert!(matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. }
        ));
        assert_eq!(storage.layout, StorageLayout::Chunked);
    }

    #[test]
    fn test_adaptive_learn_pregel_pattern() {
        // High write ratio + non-sequential → Pregel
        let mut projector = AdaptiveProjector::new();
        let metrics = WorkloadMetrics {
            read_ratio: 0.3,
            write_ratio: 0.7,             // High writes
            sequential_access_ratio: 0.2, // Random access
            cache_hit_ratio: 0.5,
            average_batch_size: 100,
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(61, "mutable_vertex", ValueType::Double);
        let storage = projector.project_to_storage(&form).unwrap();

        // Should choose Pregel for high-write, vertex-centric workload
        assert_eq!(storage.layout, StorageLayout::Hybrid);
        assert_eq!(
            storage.memory_profile.access_pattern,
            AccessPattern::VertexCentric
        );
    }

    #[test]
    fn test_adaptive_learn_arrow_pattern() {
        // Large batches + high read ratio → Arrow
        let mut projector = AdaptiveProjector::new();
        let metrics = WorkloadMetrics {
            read_ratio: 0.9, // Mostly reads
            write_ratio: 0.1,
            sequential_access_ratio: 0.5,
            cache_hit_ratio: 0.7,
            average_batch_size: 2000, // Large batches
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(62, "batch_prop", ValueType::DoubleArray);
        let storage = projector.project_to_storage(&form).unwrap();

        // Should choose Arrow for batch-oriented workload
        assert_eq!(storage.layout, StorageLayout::Columnar);
        assert_eq!(storage.memory_profile.access_pattern, AccessPattern::Batch);
    }

    #[test]
    fn test_adaptive_learn_hugearray_pattern() {
        // Sequential access + dense → HugeArray
        let mut projector = AdaptiveProjector::new();
        let metrics = WorkloadMetrics {
            read_ratio: 0.6,
            write_ratio: 0.4,
            sequential_access_ratio: 0.8, // Highly sequential
            cache_hit_ratio: 0.9,
            average_batch_size: 50,
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(63, "sequential_prop", ValueType::Long);
        let storage = projector.project_to_storage(&form).unwrap();

        // Should choose HugeArray for sequential workload
        assert!(matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. }
        ));
        assert_eq!(
            storage.memory_profile.access_pattern,
            AccessPattern::Sequential
        );
    }

    #[test]
    fn test_adaptive_conservatism() {
        // Test conservative behavior (bias toward safe default)
        let mut projector = AdaptiveProjector::with_conservatism(0.9); // Very conservative
        let metrics = WorkloadMetrics {
            read_ratio: 0.5,
            write_ratio: 0.5, // Mixed workload
            sequential_access_ratio: 0.5,
            cache_hit_ratio: 0.5,
            average_batch_size: 100,
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(64, "mixed_prop", ValueType::Double);
        let storage = projector.project_to_storage(&form).unwrap();

        // Conservative should choose HugeArray (safe default) even for mixed workload
        assert!(matches!(
            storage.backend,
            BackendTechnology::HugeArray { .. }
        ));
    }

    #[test]
    fn test_adaptive_aggressive_learning() {
        // Test aggressive learning (bias toward specialized projectors)
        let mut projector = AdaptiveProjector::with_conservatism(0.1); // Very aggressive
        let metrics = WorkloadMetrics {
            read_ratio: 0.4,
            write_ratio: 0.6, // Slightly more writes
            sequential_access_ratio: 0.3,
            cache_hit_ratio: 0.4,
            average_batch_size: 80,
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(65, "aggressive_prop", ValueType::Long);
        let storage = projector.project_to_storage(&form).unwrap();

        // Aggressive should choose Pregel for write-heavy workload
        assert_eq!(storage.layout, StorageLayout::Hybrid);
    }

    #[test]
    fn test_adaptive_recognize_from_storage_hugearray() {
        let projector = AdaptiveProjector::new();
        let form = PropertyDescriptor::new(66, "test", ValueType::Long);

        // Create HugeArray storage
        let huge = HugeArrayProjector::new();
        let storage = huge.project_to_storage(&form).unwrap();

        // Adaptive should recognize it
        let recognized = projector.recognize_from_storage(&storage).unwrap();
        assert_eq!(recognized.id, 66);
    }

    #[test]
    fn test_adaptive_recognize_from_storage_arrow() {
        let projector = AdaptiveProjector::new();
        let form = PropertyDescriptor::new(67, "test", ValueType::Double);

        // Create Arrow storage
        let arrow = ArrowProjector::new();
        let storage = arrow.project_to_storage(&form).unwrap();

        // Adaptive should recognize it
        let recognized = projector.recognize_from_storage(&storage).unwrap();
        assert_eq!(recognized.id, 67);
    }

    #[test]
    fn test_adaptive_recognize_from_storage_pregel() {
        let projector = AdaptiveProjector::new();
        let form = PropertyDescriptor::new(68, "test", ValueType::Long);

        // Create Pregel storage
        let pregel = PregelProjector::new();
        let storage = pregel.project_to_storage(&form).unwrap();

        // Adaptive should recognize it
        let recognized = projector.recognize_from_storage(&storage).unwrap();
        assert_eq!(recognized.id, 68);
    }

    #[test]
    fn test_adaptive_recognize_from_computation() {
        let projector = AdaptiveProjector::new();

        // Create BSP computation
        let computation = ComputationDescriptor::new(
            69,
            "test_computation",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        // Adaptive should recognize it
        let recognized = projector.recognize_from_computation(&computation).unwrap();
        assert_eq!(recognized.id, 69);
    }

    #[test]
    fn test_adaptive_validate_learned_projection() {
        // Test that validation works with learned patterns
        let mut projector = AdaptiveProjector::new();
        let metrics = WorkloadMetrics {
            read_ratio: 0.8,
            write_ratio: 0.2,
            sequential_access_ratio: 0.7,
            cache_hit_ratio: 0.8,
            average_batch_size: 100,
        };
        projector.observe_workload(&metrics);

        let form = PropertyDescriptor::new(70, "validated_prop", ValueType::Long);
        let storage = projector.project_to_storage(&form).unwrap();
        let computation = projector.project_to_computation(&form).unwrap();

        // Should validate successfully
        let result = projector.validate_projection(&form, &storage, &computation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adaptive_being_and_notbeing() {
        // Philosophy test: Adaptive IS all projectors and IS none of them
        let projector = AdaptiveProjector::new();

        // Without metrics, it IS HugeArray
        let form1 = PropertyDescriptor::new(71, "prop1", ValueType::Long);
        let storage1 = projector.project_to_storage(&form1).unwrap();
        assert!(matches!(
            storage1.backend,
            BackendTechnology::HugeArray { .. }
        ));

        // Now observe Pregel workload
        let mut projector = projector; // Make mutable
        let pregel_metrics = WorkloadMetrics {
            read_ratio: 0.2,
            write_ratio: 0.8,
            sequential_access_ratio: 0.3,
            cache_hit_ratio: 0.5,
            average_batch_size: 100,
        };
        projector.observe_workload(&pregel_metrics);

        // Now it IS Pregel (Being-and-NotBeing in action!)
        let form2 = PropertyDescriptor::new(72, "prop2", ValueType::Double);
        let storage2 = projector.project_to_storage(&form2).unwrap();
        assert_eq!(storage2.layout, StorageLayout::Hybrid); // Pregel layout

        // It negated itself by becoming what the workload demanded!
    }
}
