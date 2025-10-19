// Integration tests for Phase 2: Arrow2 Reference System
//
// Tests the arrow2 reference types (NodeTableReference, EdgeTableReference, ArrowBatchReference)
// that wrap arrow2 Chunk structures for type-safe graph construction.

#[cfg(test)]
#[cfg(feature = "arrow")]
mod arrow_reference_tests {
    use arrow2::array::{Array, Int64Array, Utf8Array};
    use arrow2::chunk::Chunk;
    use arrow2::datatypes::{DataType, Field, Schema};
    use gds::projection::factory::arrow::{
        ArrowBatchReference, ArrowReference, ArrowReferenceError, EdgeTableReference,
        NodeTableReference,
    };
    use std::sync::Arc;

    // =============================================================================================
    // Helper Functions
    // =============================================================================================

    fn create_node_chunk() -> (Chunk<Box<dyn Array>>, Arc<Schema>) {
        let id_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3, 4, 5]));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "Person",
            "Person",
            "Organization",
            "Person",
            "Organization",
        ]));
        let age_array: Box<dyn Array> = Box::new(Int64Array::from_slice([25, 30, 0, 35, 0]));

        let chunk = Chunk::new(vec![id_array, label_array, age_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
            Field::new("age", DataType::Int64, true), // Property
        ]));

        (chunk, schema)
    }

    fn create_edge_chunk() -> (Chunk<Box<dyn Array>>, Arc<Schema>) {
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3, 4]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([2, 3, 4, 1]));
        let type_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "KNOWS", "WORKS_AT", "KNOWS", "KNOWS",
        ]));
        let weight_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3, 4]));

        let chunk = Chunk::new(vec![source_array, target_array, type_array, weight_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Int64, false),
            Field::new("target", DataType::Int64, false),
            Field::new("type", DataType::Utf8, false),
            Field::new("weight", DataType::Int64, true), // Property
        ]));

        (chunk, schema)
    }

    // =============================================================================================
    // NodeTableReference Tests
    // =============================================================================================

    #[test]
    fn test_node_table_reference_creation() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("test_nodes", chunk, schema).unwrap();

        assert_eq!(node_table.table_name(), "test_nodes");
        assert_eq!(node_table.row_count(), 5);
        assert_eq!(node_table.id_column_index(), 0);
        assert_eq!(node_table.label_column_index(), Some(1));
    }

    #[test]
    fn test_node_table_reference_id_column() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("test_nodes", chunk, schema).unwrap();

        let id_col = node_table.id_column();
        assert_eq!(id_col.len(), 5);
        assert_eq!(id_col.value(0), 1);
        assert_eq!(id_col.value(4), 5);
    }

    #[test]
    fn test_node_table_reference_label_column() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("test_nodes", chunk, schema).unwrap();

        let label_col = node_table.label_column().unwrap();
        assert_eq!(label_col.len(), 5);
        assert_eq!(label_col.value(0), "Person");
        assert_eq!(label_col.value(2), "Organization");
    }

    #[test]
    fn test_node_table_property_columns() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("test_nodes", chunk, schema).unwrap();

        let property_indices = node_table.property_column_indices();
        assert_eq!(property_indices.len(), 1); // Only "age" is a property
        assert_eq!(property_indices[0], 2); // Age column at index 2
    }

    #[test]
    fn test_node_table_validate_schema() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("test_nodes", chunk, schema).unwrap();

        // Schema validation happens in constructor, this should succeed
        assert!(node_table.validate_schema().is_ok());
    }

    #[test]
    fn test_node_table_missing_id_column() {
        // Create chunk without ID column
        let label_array: Box<dyn Array> =
            Box::new(Utf8Array::<i32>::from_slice(["Person", "Person"]));
        let chunk = Chunk::new(vec![label_array]);
        let schema = Arc::new(Schema::from(vec![Field::new(
            "label",
            DataType::Utf8,
            false,
        )]));

        let result = NodeTableReference::new("bad_nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::MissingColumn { .. })
        ));
    }

    #[test]
    fn test_node_table_invalid_id_type() {
        // ID column with wrong type (Utf8 instead of Int64)
        let id_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice(["1", "2"]));
        let chunk = Chunk::new(vec![id_array]);
        let schema = Arc::new(Schema::from(vec![Field::new("id", DataType::Utf8, false)]));

        let result = NodeTableReference::new("bad_nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::InvalidColumnType { .. })
        ));
    }

    #[test]
    fn test_node_table_empty_chunk() {
        let chunk = Chunk::<Box<dyn Array>>::new(vec![]);
        let schema = Arc::new(Schema::from(vec![Field::new("id", DataType::Int64, false)]));

        let result = NodeTableReference::new("empty_nodes", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::EmptyTable { .. })
        ));
    }

    #[test]
    fn test_node_table_case_insensitive_column_names() {
        // Test with "nodeId" instead of "id"
        let id_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3]));
        let chunk = Chunk::new(vec![id_array]);
        let schema = Arc::new(Schema::from(vec![Field::new(
            "nodeId",
            DataType::Int64,
            false,
        )]));

        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();
        assert_eq!(node_table.id_column_index(), 0);
    }

    // =============================================================================================
    // EdgeTableReference Tests
    // =============================================================================================

    #[test]
    fn test_edge_table_reference_creation() {
        let (chunk, schema) = create_edge_chunk();
        let edge_table = EdgeTableReference::new("test_edges", chunk, schema).unwrap();

        assert_eq!(edge_table.table_name(), "test_edges");
        assert_eq!(edge_table.row_count(), 4);
        assert_eq!(edge_table.source_column_index(), 0);
        assert_eq!(edge_table.target_column_index(), 1);
        assert_eq!(edge_table.type_column_index(), Some(2));
    }

    #[test]
    fn test_edge_table_source_target_columns() {
        let (chunk, schema) = create_edge_chunk();
        let edge_table = EdgeTableReference::new("test_edges", chunk, schema).unwrap();

        let source_col = edge_table.source_column();
        assert_eq!(source_col.len(), 4);
        assert_eq!(source_col.value(0), 1);
        assert_eq!(source_col.value(3), 4);

        let target_col = edge_table.target_column();
        assert_eq!(target_col.len(), 4);
        assert_eq!(target_col.value(0), 2);
        assert_eq!(target_col.value(3), 1);
    }

    #[test]
    fn test_edge_table_type_column() {
        let (chunk, schema) = create_edge_chunk();
        let edge_table = EdgeTableReference::new("test_edges", chunk, schema).unwrap();

        let type_col = edge_table.type_column().unwrap();
        assert_eq!(type_col.len(), 4);
        assert_eq!(type_col.value(0), "KNOWS");
        assert_eq!(type_col.value(1), "WORKS_AT");
    }

    #[test]
    fn test_edge_table_property_columns() {
        let (chunk, schema) = create_edge_chunk();
        let edge_table = EdgeTableReference::new("test_edges", chunk, schema).unwrap();

        let property_indices = edge_table.property_column_indices();
        assert_eq!(property_indices.len(), 1); // Only "weight" is a property
        assert_eq!(property_indices[0], 3); // Weight column at index 3
    }

    #[test]
    fn test_edge_table_missing_source_column() {
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2]));
        let chunk = Chunk::new(vec![target_array]);
        let schema = Arc::new(Schema::from(vec![Field::new(
            "target",
            DataType::Int64,
            false,
        )]));

        let result = EdgeTableReference::new("bad_edges", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::MissingColumn { .. })
        ));
    }

    #[test]
    fn test_edge_table_missing_target_column() {
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2]));
        let chunk = Chunk::new(vec![source_array]);
        let schema = Arc::new(Schema::from(vec![Field::new(
            "source",
            DataType::Int64,
            false,
        )]));

        let result = EdgeTableReference::new("bad_edges", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::MissingColumn { .. })
        ));
    }

    #[test]
    fn test_edge_table_invalid_source_type() {
        let source_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice(["1", "2"]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2]));
        let chunk = Chunk::new(vec![source_array, target_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Utf8, false),
            Field::new("target", DataType::Int64, false),
        ]));

        let result = EdgeTableReference::new("bad_edges", chunk, schema);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::InvalidColumnType { .. })
        ));
    }

    #[test]
    fn test_edge_table_case_insensitive_column_names() {
        // Test with "sourceId" and "targetId" instead of "source"/"target"
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([2, 3, 1]));
        let chunk = Chunk::new(vec![source_array, target_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("sourceId", DataType::Int64, false),
            Field::new("targetId", DataType::Int64, false),
        ]));

        let edge_table = EdgeTableReference::new("edges", chunk, schema).unwrap();
        assert_eq!(edge_table.source_column_index(), 0);
        assert_eq!(edge_table.target_column_index(), 1);
    }

    // =============================================================================================
    // ArrowBatchReference Tests
    // =============================================================================================

    #[test]
    fn test_arrow_batch_reference_creation() {
        let (chunk, schema) = create_node_chunk();
        let batch = ArrowBatchReference::new(&chunk, &schema, 42, 0, 5);

        assert_eq!(batch.batch_index(), 42);
        assert_eq!(batch.len(), 5);
        assert_eq!(batch.start_offset(), 0);
        assert_eq!(batch.end_offset(), 5);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_arrow_batch_reference_column_access() {
        let (chunk, schema) = create_node_chunk();
        let batch = ArrowBatchReference::new(&chunk, &schema, 0, 0, 5);

        // Access Int64 ID column
        let id_col = batch.int64_column(0).unwrap();
        assert_eq!(id_col.len(), 5);
        assert_eq!(id_col.value(0), 1);

        // Access Utf8 label column
        let label_col = batch.utf8_column(1).unwrap();
        assert_eq!(label_col.len(), 5);
        assert_eq!(label_col.value(0), "Person");

        // Access Int64 property column
        let age_col = batch.int64_column(2).unwrap();
        assert_eq!(age_col.len(), 5);
        assert_eq!(age_col.value(0), 25);
    }

    #[test]
    fn test_arrow_batch_reference_schema_access() {
        let (chunk, schema) = create_node_chunk();
        let batch = ArrowBatchReference::new(&chunk, &schema, 0, 0, 5);

        let batch_schema = batch.schema();
        assert_eq!(batch_schema.fields.len(), 3);
        assert_eq!(batch_schema.fields[0].name, "id");
        assert_eq!(batch_schema.fields[1].name, "label");
    }

    #[test]
    fn test_arrow_batch_reference_empty() {
        let empty_chunk = Chunk::<Box<dyn Array>>::new(vec![]);
        let schema = Arc::new(Schema::from(vec![]));
        let batch = ArrowBatchReference::new(&empty_chunk, &schema, 0, 0, 0);

        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }

    // =============================================================================================
    // ArrowReference Trait Tests
    // =============================================================================================

    #[test]
    fn test_arrow_reference_trait_node_table() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        // Test trait methods
        assert_eq!(node_table.table_name(), "nodes");
        assert_eq!(node_table.row_count(), 5);
        assert!(node_table.validate_schema().is_ok());

        // Test find_column
        assert_eq!(node_table.find_column("id"), Some(0));
        assert_eq!(node_table.find_column("label"), Some(1));
        assert_eq!(node_table.find_column("age"), Some(2));
        assert_eq!(node_table.find_column("nonexistent"), None);
    }

    #[test]
    fn test_arrow_reference_trait_edge_table() {
        let (chunk, schema) = create_edge_chunk();
        let edge_table = EdgeTableReference::new("edges", chunk, schema).unwrap();

        // Test trait methods
        assert_eq!(edge_table.table_name(), "edges");
        assert_eq!(edge_table.row_count(), 4);
        assert!(edge_table.validate_schema().is_ok());

        // Test find_column
        assert_eq!(edge_table.find_column("source"), Some(0));
        assert_eq!(edge_table.find_column("target"), Some(1));
        assert_eq!(edge_table.find_column("type"), Some(2));
    }

    #[test]
    fn test_arrow_reference_get_field() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        // Test get_field success
        let id_field = node_table.get_field("id").unwrap();
        assert_eq!(id_field.name, "id");
        assert_eq!(id_field.data_type, DataType::Int64);

        // Test get_field failure
        let result = node_table.get_field("nonexistent");
        assert!(matches!(
            result,
            Err(ArrowReferenceError::ColumnNotFound { .. })
        ));
    }

    #[test]
    fn test_arrow_reference_validate_column() {
        let (chunk, schema) = create_node_chunk();
        let node_table = NodeTableReference::new("nodes", chunk, schema).unwrap();

        // Test valid column
        let id_idx = node_table.validate_column("id", &DataType::Int64).unwrap();
        assert_eq!(id_idx, 0);

        // Test wrong type
        let result = node_table.validate_column("id", &DataType::Utf8);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::InvalidColumnType { .. })
        ));

        // Test missing column
        let result = node_table.validate_column("nonexistent", &DataType::Int64);
        assert!(matches!(
            result,
            Err(ArrowReferenceError::ColumnNotFound { .. })
        ));
    }
}
