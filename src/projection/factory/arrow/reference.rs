// Arrow2 Reference System
//
// Provides type-safe wrappers around arrow2 data structures for graph construction.
//
// Design principles:
// - Type safety: arrow2::chunk::Chunk → typed TableReference
// - Schema inference: Convention-based column discovery (id, nodeId, source, target)
// - Validation: Fail-fast with clear error messages
// - Zero-copy: References wrap existing arrow2 structures
//
// Reference hierarchy:
// - ArrowReference: Base trait for table-like arrow2 structures
// - NodeTableReference: Node table wrapper (Chunk<Box<dyn Array>>)
// - EdgeTableReference: Edge table wrapper (Chunk<Box<dyn Array>>)
// - ArrowBatchReference: Single batch wrapper for iteration

use arrow2::array::{Array, PrimitiveArray, Utf8Array};
use arrow2::chunk::Chunk;
use arrow2::datatypes::{DataType, Field, Schema};
use std::sync::Arc;
use thiserror::Error;

// ================================================================================================
// Error Types
// ================================================================================================

#[derive(Debug, Error)]
pub enum ArrowReferenceError {
    #[error("Missing required column: {column_name}")]
    MissingColumn { column_name: String },

    #[error("Invalid column type: expected {expected}, got {actual}")]
    InvalidColumnType { expected: String, actual: String },

    #[error("Schema mismatch: {message}")]
    SchemaMismatch { message: String },

    #[error("Empty table: {table_name}")]
    EmptyTable { table_name: String },

    #[error("Column not found: {column_name} in table {table_name}")]
    ColumnNotFound {
        table_name: String,
        column_name: String,
    },
}

// ================================================================================================
// ArrowReference Trait - Base abstraction for arrow2 table-like structures
// ================================================================================================

/// Base trait for arrow2 table references.
/// Provides common operations for schema access, validation, and column discovery.
pub trait ArrowReference {
    /// Returns the table name for error reporting
    fn table_name(&self) -> &str;

    /// Returns the arrow2 schema
    fn schema(&self) -> &Schema;

    /// Validates that the schema contains required columns
    fn validate_schema(&self) -> Result<(), ArrowReferenceError>;

    /// Returns the number of rows in the table
    fn row_count(&self) -> usize;

    /// Finds a column index by name (case-insensitive)
    fn find_column(&self, name: &str) -> Option<usize> {
        self.schema()
            .fields
            .iter()
            .position(|f| f.name.eq_ignore_ascii_case(name))
    }

    /// Gets a field by name (case-insensitive)
    fn get_field(&self, name: &str) -> Result<&Field, ArrowReferenceError> {
        self.find_column(name)
            .and_then(|idx| self.schema().fields.get(idx))
            .ok_or_else(|| ArrowReferenceError::ColumnNotFound {
                table_name: self.table_name().to_string(),
                column_name: name.to_string(),
            })
    }

    /// Validates that a column exists and has the expected type
    fn validate_column(
        &self,
        name: &str,
        expected_type: &DataType,
    ) -> Result<usize, ArrowReferenceError> {
        let field = self.get_field(name)?;
        let col_idx = self.find_column(name).unwrap(); // Safe: get_field succeeded

        if field.data_type != *expected_type {
            return Err(ArrowReferenceError::InvalidColumnType {
                expected: format!("{:?}", expected_type),
                actual: format!("{:?}", field.data_type),
            });
        }

        Ok(col_idx)
    }
}

// ================================================================================================
// NodeTableReference - Wrapper for node tables (arrow2 Chunk)
// ================================================================================================

/// Reference to a node table stored as arrow2 Chunk.
///
/// Expected schema conventions:
/// - ID column: "id" or "nodeId" (Int64)
/// - Label column: "label" or "labels" (Utf8)
/// - Property columns: any additional columns
///
/// Example:
/// ```rust,ignore
/// let node_table = NodeTableReference::new("nodes", chunk, schema)?;
/// let id_col = node_table.id_column_index();
/// let label_col = node_table.label_column_index();
/// ```
pub struct NodeTableReference {
    table_name: String,
    chunk: Chunk<Box<dyn Array>>,
    schema: Arc<Schema>,
    id_column_idx: usize,
    label_column_idx: Option<usize>,
}

impl NodeTableReference {
    /// Creates a new node table reference with schema inference
    pub fn new(
        table_name: impl Into<String>,
        chunk: Chunk<Box<dyn Array>>,
        schema: Arc<Schema>,
    ) -> Result<Self, ArrowReferenceError> {
        let table_name = table_name.into();

        // Validate non-empty
        if chunk.is_empty() {
            return Err(ArrowReferenceError::EmptyTable {
                table_name: table_name.clone(),
            });
        }

        // Find ID column (required)
        let id_column_idx = schema
            .fields
            .iter()
            .position(|f| {
                f.name.eq_ignore_ascii_case("id") || f.name.eq_ignore_ascii_case("nodeId")
            })
            .ok_or_else(|| ArrowReferenceError::MissingColumn {
                column_name: "id or nodeId".to_string(),
            })?;

        // Validate ID column is Int64
        let id_field = &schema.fields[id_column_idx];
        if id_field.data_type != DataType::Int64 {
            return Err(ArrowReferenceError::InvalidColumnType {
                expected: "Int64".to_string(),
                actual: format!("{:?}", id_field.data_type),
            });
        }

        // Find label column (optional)
        let label_column_idx = schema.fields.iter().position(|f| {
            f.name.eq_ignore_ascii_case("label") || f.name.eq_ignore_ascii_case("labels")
        });

        // Validate label column is Utf8 if present
        if let Some(idx) = label_column_idx {
            let label_field = &schema.fields[idx];
            if label_field.data_type != DataType::Utf8 {
                return Err(ArrowReferenceError::InvalidColumnType {
                    expected: "Utf8".to_string(),
                    actual: format!("{:?}", label_field.data_type),
                });
            }
        }

        Ok(Self {
            table_name,
            chunk,
            schema,
            id_column_idx,
            label_column_idx,
        })
    }

    /// Returns the ID column index
    pub fn id_column_index(&self) -> usize {
        self.id_column_idx
    }

    /// Returns the label column index (if present)
    pub fn label_column_index(&self) -> Option<usize> {
        self.label_column_idx
    }

    /// Returns a reference to the ID column as Int64Array
    pub fn id_column(&self) -> &PrimitiveArray<i64> {
        self.chunk.arrays()[self.id_column_idx]
            .as_any()
            .downcast_ref::<PrimitiveArray<i64>>()
            .expect("ID column should be Int64")
    }

    /// Returns a reference to the label column as Utf8Array (if present)
    pub fn label_column(&self) -> Option<&Utf8Array<i32>> {
        self.label_column_idx.map(|idx| {
            self.chunk.arrays()[idx]
                .as_any()
                .downcast_ref::<Utf8Array<i32>>()
                .expect("Label column should be Utf8")
        })
    }

    /// Returns property column indices (all columns except ID and label)
    pub fn property_column_indices(&self) -> Vec<usize> {
        (0..self.chunk.arrays().len())
            .filter(|&idx| idx != self.id_column_idx && Some(idx) != self.label_column_idx)
            .collect()
    }

    /// Returns the underlying chunk
    pub fn chunk(&self) -> &Chunk<Box<dyn Array>> {
        &self.chunk
    }
}

impl ArrowReference for NodeTableReference {
    fn table_name(&self) -> &str {
        &self.table_name
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn validate_schema(&self) -> Result<(), ArrowReferenceError> {
        // ID column validated in constructor
        // Label column validated in constructor if present
        Ok(())
    }

    fn row_count(&self) -> usize {
        self.chunk.len()
    }
}

// ================================================================================================
// EdgeTableReference - Wrapper for edge tables (arrow2 Chunk)
// ================================================================================================

/// Reference to an edge table stored as arrow2 Chunk.
///
/// Expected schema conventions:
/// - Source column: "source" or "sourceId" (Int64)
/// - Target column: "target" or "targetId" (Int64)
/// - Type column: "type" or "relationshipType" (Utf8, optional)
/// - Property columns: any additional columns
///
/// Example:
/// ```rust,ignore
/// let edge_table = EdgeTableReference::new("edges", chunk, schema)?;
/// let source_col = edge_table.source_column_index();
/// let target_col = edge_table.target_column_index();
/// ```
pub struct EdgeTableReference {
    table_name: String,
    chunk: Chunk<Box<dyn Array>>,
    schema: Arc<Schema>,
    source_column_idx: usize,
    target_column_idx: usize,
    type_column_idx: Option<usize>,
}

impl EdgeTableReference {
    /// Creates a new edge table reference with schema inference
    pub fn new(
        table_name: impl Into<String>,
        chunk: Chunk<Box<dyn Array>>,
        schema: Arc<Schema>,
    ) -> Result<Self, ArrowReferenceError> {
        let table_name = table_name.into();

        // Validate non-empty
        if chunk.is_empty() {
            return Err(ArrowReferenceError::EmptyTable {
                table_name: table_name.clone(),
            });
        }

        // Find source column (required)
        let source_column_idx = schema
            .fields
            .iter()
            .position(|f| {
                f.name.eq_ignore_ascii_case("source") || f.name.eq_ignore_ascii_case("sourceId")
            })
            .ok_or_else(|| ArrowReferenceError::MissingColumn {
                column_name: "source or sourceId".to_string(),
            })?;

        // Find target column (required)
        let target_column_idx = schema
            .fields
            .iter()
            .position(|f| {
                f.name.eq_ignore_ascii_case("target") || f.name.eq_ignore_ascii_case("targetId")
            })
            .ok_or_else(|| ArrowReferenceError::MissingColumn {
                column_name: "target or targetId".to_string(),
            })?;

        // Validate source/target columns are Int64
        let source_field = &schema.fields[source_column_idx];
        if source_field.data_type != DataType::Int64 {
            return Err(ArrowReferenceError::InvalidColumnType {
                expected: "Int64".to_string(),
                actual: format!("{:?}", source_field.data_type),
            });
        }

        let target_field = &schema.fields[target_column_idx];
        if target_field.data_type != DataType::Int64 {
            return Err(ArrowReferenceError::InvalidColumnType {
                expected: "Int64".to_string(),
                actual: format!("{:?}", target_field.data_type),
            });
        }

        // Find type column (optional)
        let type_column_idx = schema.fields.iter().position(|f| {
            f.name.eq_ignore_ascii_case("type") || f.name.eq_ignore_ascii_case("relationshipType")
        });

        // Validate type column is Utf8 if present
        if let Some(idx) = type_column_idx {
            let type_field = &schema.fields[idx];
            if type_field.data_type != DataType::Utf8 {
                return Err(ArrowReferenceError::InvalidColumnType {
                    expected: "Utf8".to_string(),
                    actual: format!("{:?}", type_field.data_type),
                });
            }
        }

        Ok(Self {
            table_name,
            chunk,
            schema,
            source_column_idx,
            target_column_idx,
            type_column_idx,
        })
    }

    /// Returns the source column index
    pub fn source_column_index(&self) -> usize {
        self.source_column_idx
    }

    /// Returns the target column index
    pub fn target_column_index(&self) -> usize {
        self.target_column_idx
    }

    /// Returns the type column index (if present)
    pub fn type_column_index(&self) -> Option<usize> {
        self.type_column_idx
    }

    /// Returns a reference to the source column as Int64Array
    pub fn source_column(&self) -> &PrimitiveArray<i64> {
        self.chunk.arrays()[self.source_column_idx]
            .as_any()
            .downcast_ref::<PrimitiveArray<i64>>()
            .expect("Source column should be Int64")
    }

    /// Returns a reference to the target column as Int64Array
    pub fn target_column(&self) -> &PrimitiveArray<i64> {
        self.chunk.arrays()[self.target_column_idx]
            .as_any()
            .downcast_ref::<PrimitiveArray<i64>>()
            .expect("Target column should be Int64")
    }

    /// Returns a reference to the type column as Utf8Array (if present)
    pub fn type_column(&self) -> Option<&Utf8Array<i32>> {
        self.type_column_idx.map(|idx| {
            self.chunk.arrays()[idx]
                .as_any()
                .downcast_ref::<Utf8Array<i32>>()
                .expect("Type column should be Utf8")
        })
    }

    /// Returns property column indices (all columns except source, target, type)
    pub fn property_column_indices(&self) -> Vec<usize> {
        (0..self.chunk.arrays().len())
            .filter(|&idx| {
                idx != self.source_column_idx
                    && idx != self.target_column_idx
                    && Some(idx) != self.type_column_idx
            })
            .collect()
    }

    /// Returns the underlying chunk
    pub fn chunk(&self) -> &Chunk<Box<dyn Array>> {
        &self.chunk
    }
}

impl ArrowReference for EdgeTableReference {
    fn table_name(&self) -> &str {
        &self.table_name
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn validate_schema(&self) -> Result<(), ArrowReferenceError> {
        // Source/target columns validated in constructor
        // Type column validated in constructor if present
        Ok(())
    }

    fn row_count(&self) -> usize {
        self.chunk.len()
    }
}

// ================================================================================================
// ArrowBatchReference - Single batch wrapper for iteration
// ================================================================================================

/// Reference to a single batch of arrow2 data during parallel iteration.
/// Used by Scanner system to pass batches to consumers.
///
/// **Range Semantics**: Includes `start_offset` and `end_offset` to define
/// a logical sub-range within the chunk without physically slicing arrays.
pub struct ArrowBatchReference<'a> {
    chunk: &'a Chunk<Box<dyn Array>>,
    schema: &'a Schema,
    batch_index: usize,
    start_offset: usize,
    end_offset: usize,
}

impl<'a> ArrowBatchReference<'a> {
    /// Creates a new batch reference with a logical sub-range
    pub fn new(
        chunk: &'a Chunk<Box<dyn Array>>,
        schema: &'a Schema,
        batch_index: usize,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            chunk,
            schema,
            batch_index,
            start_offset,
            end_offset,
        }
    }

    /// Returns the batch index
    pub fn batch_index(&self) -> usize {
        self.batch_index
    }

    /// Returns the chunk
    pub fn chunk(&self) -> &Chunk<Box<dyn Array>> {
        self.chunk
    }

    /// Returns the schema
    pub fn schema(&self) -> &Schema {
        self.schema
    }

    /// Returns the number of rows in this batch (range length, not chunk length)
    pub fn len(&self) -> usize {
        self.end_offset - self.start_offset
    }

    /// Returns the start offset within the chunk
    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    /// Returns the end offset within the chunk
    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    /// Returns true if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.chunk.is_empty()
    }

    /// Gets a column by index
    pub fn column(&self, idx: usize) -> Option<&Box<dyn Array>> {
        self.chunk.arrays().get(idx)
    }

    /// Gets a typed Int64 column
    pub fn int64_column(&self, idx: usize) -> Option<&PrimitiveArray<i64>> {
        self.column(idx)
            .and_then(|arr| arr.as_any().downcast_ref::<PrimitiveArray<i64>>())
    }

    /// Gets a typed Utf8 column
    pub fn utf8_column(&self, idx: usize) -> Option<&Utf8Array<i32>> {
        self.column(idx)
            .and_then(|arr| arr.as_any().downcast_ref::<Utf8Array<i32>>())
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use arrow2::array::Int64Array;

    fn create_test_node_chunk() -> (Chunk<Box<dyn Array>>, Arc<Schema>) {
        let id_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3]));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "Person",
            "Person",
            "Organization",
        ]));

        let chunk = Chunk::new(vec![id_array, label_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
        ]));

        (chunk, schema)
    }

    fn create_test_edge_chunk() -> (Chunk<Box<dyn Array>>, Arc<Schema>) {
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([2, 3, 1]));
        let type_array: Box<dyn Array> =
            Box::new(Utf8Array::<i32>::from_slice(["KNOWS", "WORKS_AT", "KNOWS"]));

        let chunk = Chunk::new(vec![source_array, target_array, type_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Int64, false),
            Field::new("target", DataType::Int64, false),
            Field::new("type", DataType::Utf8, false),
        ]));

        (chunk, schema)
    }

    #[test]
    fn test_node_table_reference_creation() {
        let (chunk, schema) = create_test_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        assert_eq!(node_table.table_name(), "nodes");
        assert_eq!(node_table.row_count(), 3);
        assert_eq!(node_table.id_column_index(), 0);
        assert_eq!(node_table.label_column_index(), Some(1));
    }

    #[test]
    fn test_node_table_reference_id_column() {
        let (chunk, schema) = create_test_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        let id_col = node_table.id_column();
        assert_eq!(id_col.len(), 3);
        assert_eq!(id_col.value(0), 1);
        assert_eq!(id_col.value(1), 2);
        assert_eq!(id_col.value(2), 3);
    }

    #[test]
    fn test_node_table_reference_label_column() {
        let (chunk, schema) = create_test_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        let label_col = node_table.label_column().unwrap();
        assert_eq!(label_col.len(), 3);
        assert_eq!(label_col.value(0), "Person");
        assert_eq!(label_col.value(1), "Person");
        assert_eq!(label_col.value(2), "Organization");
    }

    #[test]
    fn test_edge_table_reference_creation() {
        let (chunk, schema) = create_test_edge_chunk();
        let edge_table = EdgeTableReference::new("edges", chunk, schema).unwrap();

        assert_eq!(edge_table.table_name(), "edges");
        assert_eq!(edge_table.row_count(), 3);
        assert_eq!(edge_table.source_column_index(), 0);
        assert_eq!(edge_table.target_column_index(), 1);
        assert_eq!(edge_table.type_column_index(), Some(2));
    }

    #[test]
    fn test_edge_table_reference_columns() {
        let (chunk, schema) = create_test_edge_chunk();
        let edge_table = EdgeTableReference::new("edges", chunk, schema).unwrap();

        let source_col = edge_table.source_column();
        assert_eq!(source_col.value(0), 1);
        assert_eq!(source_col.value(1), 2);

        let target_col = edge_table.target_column();
        assert_eq!(target_col.value(0), 2);
        assert_eq!(target_col.value(1), 3);

        let type_col = edge_table.type_column().unwrap();
        assert_eq!(type_col.value(0), "KNOWS");
        assert_eq!(type_col.value(1), "WORKS_AT");
    }

    #[test]
    fn test_arrow_batch_reference() {
        let (chunk, schema) = create_test_node_chunk();
        let batch = ArrowBatchReference::new(&chunk, &schema, 0, 0, 3);

        assert_eq!(batch.batch_index(), 0);
        assert_eq!(batch.len(), 3);
        assert_eq!(batch.start_offset(), 0);
        assert_eq!(batch.end_offset(), 3);
        assert!(!batch.is_empty());

        let id_col = batch.int64_column(0).unwrap();
        assert_eq!(id_col.len(), 3);
    }

    #[test]
    fn test_missing_id_column_error() {
        let label_array: Box<dyn Array> =
            Box::new(Utf8Array::<i32>::from_slice(["Person", "Person"]));
        let chunk = Chunk::new(vec![label_array]);
        let schema = Arc::new(Schema::from(vec![Field::new(
            "label",
            DataType::Utf8,
            false,
        )]));

        let result = NodeTableReference::new("nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::MissingColumn { .. })
        ));
    }

    #[test]
    fn test_invalid_id_type_error() {
        let id_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice(["1", "2"]));
        let chunk = Chunk::new(vec![id_array]);
        let schema = Arc::new(Schema::from(vec![Field::new("id", DataType::Utf8, false)]));

        let result = NodeTableReference::new("nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::InvalidColumnType { .. })
        ));
    }

    #[test]
    fn test_empty_chunk_error() {
        let chunk = Chunk::<Box<dyn Array>>::new(vec![]);
        let schema = Arc::new(Schema::from(vec![Field::new("id", DataType::Int64, false)]));

        let result = NodeTableReference::new("nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::EmptyTable { .. })
        ));
    }
}
