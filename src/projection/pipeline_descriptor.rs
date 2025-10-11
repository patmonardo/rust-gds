use crate::types::ValueType;
use serde::{Deserialize, Serialize};

pub type PropertyId = u32;
pub type StructId = u32;

/// Field descriptor inside a Struct/UDT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub name: String,
    pub value_type: ValueType,
    pub offset: u16,
}

/// Descriptor for a user-defined struct (UDT)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructDescriptor {
    pub id: StructId,
    pub name: String,
    pub fields: Vec<FieldDescriptor>,
}

/// Storage hint for property backends
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StorageHint {
    FixedWidth,
    VariableLength,
    ListAsOffsets,
    ColumnarStruct,
    SerializedRow,
}

/// Individual property descriptor (leaf level).
///
/// Describes a single property with its type, nullability, and storage characteristics.
/// This is the leaf-level metadata for one property in the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
}

impl PropertyDescriptor {
    pub fn new(id: PropertyId, name: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            id,
            name: name.into(),
            value_type,
            nullable: true,
            storage_hint: StorageHint::VariableLength,
        }
    }

    pub fn with_storage_hint(mut self, hint: StorageHint) -> Self {
        self.storage_hint = hint;
        self
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
}

/// Pipeline descriptor - The Dharma (Unity) that projects into extremes.
///
/// A pipeline has BOTH Storage and Computation poles - it's the most accurate CS term.
/// This is the CENTER of the Five-Fold Brahmachakra - the unity that projects
/// into Computation (flow/process) and Storage (flow/persistence).
///
/// # The Dharma (धर्म)
///
/// In Sanskrit philosophy, Dharma is the governing law, the principle that upholds.
/// A PipelineDescriptor is the Dharma of a computation - the collection of properties
/// and their flows that define what the pipeline IS.
///
/// # Pipeline in Computer Science
///
/// Pipelines are fundamental in CS:
/// - **Unix pipes**: `cat | grep | sort` (computation flow)
/// - **CPU pipelines**: fetch → decode → execute (hardware flow)
/// - **ML pipelines**: data → train → evaluate (computation + storage flow)
/// - **VFS**: application → filesystem → device (storage flow)
///
/// **Our pipeline has BOTH flows simultaneously:**
/// - **Computation pipeline**: How data transforms (algorithm)
/// - **Storage pipeline**: How data persists (data structure)
///
/// # Five-Fold Structure
///
/// ```text
///         PipelineDescriptor (Dharma/Unity)
///                   ॐ
///                   |
///          +--------+--------+
///          |                 |
///     Computation        Storage
///     (Flow/Process)   (Flow/Persistence)
///          |                 |
///     +----+----+       +----+----+
///     |         |       |         |
/// Descriptor Runtime  Descriptor Runtime
///  (WHAT)    (HOW)     (WHAT)    (HOW)
/// ```
///
/// The pipeline projects into both extremes simultaneously:
/// - **Computation**: HOW data flows through computation
/// - **Storage**: HOW data flows through storage
///
/// # Example
///
/// ```rust,ignore
/// use rust_gds::projection::{PipelineDescriptor, PropertyDescriptor};
/// use rust_gds::types::ValueType;
///
/// let property = PropertyDescriptor::new(0, "pagerank", ValueType::Double);
///
/// let pipeline = PipelineDescriptor::new("PageRank")
///     .with_property(property)
///     .with_computation_flow("pagerank")
///     .with_storage_flow("columnar");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineDescriptor {
    /// Name of the pipeline
    pub name: String,

    /// Collection of properties flowing through this pipeline.
    /// A pipeline is NOT one property but a COLLECTION - the relation between them.
    pub properties: Vec<PropertyDescriptor>,

    /// Hint about the computation flow (e.g., "pagerank", "louvain", "wcc").
    /// This is the COMPUTATION pole - how data transforms.
    pub computation_flow: Option<String>,

    /// Hint about the storage flow (e.g., "columnar", "sparse", "dense").
    /// This is the STORAGE pole - how data persists.
    pub storage_flow: Option<String>,
}

impl PipelineDescriptor {
    /// Create a new pipeline descriptor.
    ///
    /// # Parameters
    /// - `name`: Name of the pipeline (e.g., "PageRank", "Louvain")
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            properties: Vec::new(),
            computation_flow: None,
            storage_flow: None,
        }
    }

    /// Add a property to the pipeline.
    ///
    /// A pipeline is a COLLECTION of properties, not a single property.
    /// This builds up the collection of properties flowing through.
    pub fn with_property(mut self, property: PropertyDescriptor) -> Self {
        self.properties.push(property);
        self
    }

    /// Add multiple properties to the pipeline.
    pub fn with_properties(mut self, properties: Vec<PropertyDescriptor>) -> Self {
        self.properties.extend(properties);
        self
    }

    /// Set computation flow hint (the COMPUTATION pole).
    ///
    /// Examples: "pagerank", "louvain", "wcc", "betweenness"
    pub fn with_computation_flow(mut self, flow: impl Into<String>) -> Self {
        self.computation_flow = Some(flow.into());
        self
    }

    /// Set storage flow hint (the STORAGE pole).
    ///
    /// Examples: "columnar", "sparse", "dense", "csr"
    pub fn with_storage_flow(mut self, flow: impl Into<String>) -> Self {
        self.storage_flow = Some(flow.into());
        self
    }

    // Backwards compatibility aliases
    /// Alias for with_computation_flow (backwards compatibility)
    #[doc(hidden)]
    pub fn with_algorithm(self, algorithm: impl Into<String>) -> Self {
        self.with_computation_flow(algorithm)
    }

    /// Alias for with_storage_flow (backwards compatibility)
    #[doc(hidden)]
    pub fn with_structure(self, structure: impl Into<String>) -> Self {
        self.with_storage_flow(structure)
    }

    /// Get a property by name.
    pub fn get_property(&self, name: &str) -> Option<&PropertyDescriptor> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get a property by ID.
    pub fn get_property_by_id(&self, id: PropertyId) -> Option<&PropertyDescriptor> {
        self.properties.iter().find(|p| p.id == id)
    }

    /// Get the primary property (first in collection).
    ///
    /// Many pipelines have one main property (e.g., "pagerank" for PageRank).
    /// This is a convenience method for that common case.
    pub fn primary_property(&self) -> Option<&PropertyDescriptor> {
        self.properties.first()
    }

    /// Get all property IDs.
    pub fn property_ids(&self) -> Vec<PropertyId> {
        self.properties.iter().map(|p| p.id).collect()
    }

    /// Check if pipeline has a property with given name.
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.iter().any(|p| p.name == name)
    }
}

// ===== Backwards Compatibility Aliases =====

/// Type alias for migration from ProgramDescriptor
#[doc(hidden)]
pub type ProgramDescriptor = PipelineDescriptor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_descriptor() {
        let prop = PropertyDescriptor::new(0, "test", ValueType::Double)
            .with_nullable(false)
            .with_storage_hint(StorageHint::FixedWidth);

        assert_eq!(prop.id, 0);
        assert_eq!(prop.name, "test");
        assert!(!prop.nullable);
    }

    #[test]
    fn test_pipeline_descriptor_single_property() {
        let property = PropertyDescriptor::new(0, "pagerank", ValueType::Double);
        let pipeline = PipelineDescriptor::new("PageRank")
            .with_property(property)
            .with_computation_flow("pagerank")
            .with_storage_flow("columnar");

        assert_eq!(pipeline.name, "PageRank");
        assert_eq!(pipeline.properties.len(), 1);
        assert_eq!(pipeline.computation_flow, Some("pagerank".to_string()));
        assert_eq!(pipeline.storage_flow, Some("columnar".to_string()));
    }

    #[test]
    fn test_pipeline_descriptor_multiple_properties() {
        let property1 = PropertyDescriptor::new(0, "pagerank", ValueType::Double);
        let property2 = PropertyDescriptor::new(1, "iterations", ValueType::Long);

        let pipeline = PipelineDescriptor::new("PageRank")
            .with_property(property1)
            .with_property(property2);

        assert_eq!(pipeline.properties.len(), 2);
        assert!(pipeline.has_property("pagerank"));
        assert!(pipeline.has_property("iterations"));
        assert!(!pipeline.has_property("nonexistent"));
    }

    #[test]
    fn test_pipeline_descriptor_queries() {
        let property = PropertyDescriptor::new(42, "test", ValueType::Long);
        let pipeline = PipelineDescriptor::new("Test").with_property(property);

        assert!(pipeline.get_property("test").is_some());
        assert!(pipeline.get_property_by_id(42).is_some());
        assert!(pipeline.primary_property().is_some());

        assert_eq!(pipeline.property_ids(), vec![42]);
    }

    #[test]
    fn test_pipeline_descriptor_dharma_concept() {
        // A pipeline has BOTH poles - computation flow and storage flow
        let property = PropertyDescriptor::new(0, "score", ValueType::Double);

        let pipeline = PipelineDescriptor::new("Louvain")
            .with_property(property)
            .with_computation_flow("community_detection") // Computation pipeline
            .with_storage_flow("sparse_graph"); // Storage pipeline

        // The pipeline IS the dharma - the governing relation with BOTH flows
        assert_eq!(pipeline.name, "Louvain");
        assert!(pipeline.computation_flow.is_some());
        assert!(pipeline.storage_flow.is_some());

        // It's a collection of properties flowing through
        assert_eq!(pipeline.properties.len(), 1);
    }

    #[test]
    fn test_backwards_compatibility_aliases() {
        // Old method names should still work
        let property = PropertyDescriptor::new(0, "test", ValueType::Double);
        let pipeline = PipelineDescriptor::new("Test")
            .with_property(property)
            .with_algorithm("test_algo") // Old name
            .with_structure("test_struct"); // Old name

        assert_eq!(pipeline.computation_flow, Some("test_algo".to_string()));
        assert_eq!(pipeline.storage_flow, Some("test_struct".to_string()));
    }
}
