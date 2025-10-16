// Phase 5-6: Direct Import Tasks with Property Support (GAMMA Strategy)
//
// This module implements concrete ImportTask instances that accumulate graph data
// from Arrow tables and build GraphStore structures. This is a simplified "Gamma"
// approach that defers the full Core/Loading incremental builder infrastructure.
//
// Key components:
// - NodeAccumulator: Accumulates node data (IDs, labels, properties)
// - EdgeAccumulator: Accumulates edge data (source, target, type, properties)
// - PropertyAccumulator: Accumulates property values per entity
// - NodeImportTask: ImportTask for parallel node scanning
// - EdgeImportTask: ImportTask for parallel edge scanning
//
// Phase 6 Extensions:
// - Property column extraction from Arrow
// - Arrow type → PropertyValues conversion
// - Default value handling
// - Null value support
//
// Limitations:
// - Requires all data in memory (no streaming)
// - Two-pass required (nodes first, then edges)
// - Lock contention on shared Mutex<Accumulator>
// - Aggregation logic deferred to Phase 7

use super::{
    task::{ImportTask, TaskError, TaskFactory},
    ArrowBatchReference, ScanCursor,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{
    id_map::{IdMap, SimpleIdMap},
    MappedNodeId, OriginalNodeId, RelationshipTopology,
};
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultFloatArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
    DefaultLongNodePropertyValues,
};
use crate::types::properties::PropertyValues;
use crate::types::{DefaultValue, ValueType};
use arrow2::array::{Array, Float32Array, Float64Array, Int64Array, ListArray};
use arrow2::datatypes::DataType;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// ================================================================================================
// Property Configuration & Accumulation (Phase 6)
// ================================================================================================

/// Configuration for a single property to import from Arrow columns.
///
/// Specifies which column to read, what the property key is, and how to handle
/// missing/null values.
#[derive(Debug, Clone)]
pub struct PropertyConfig {
    /// Property key/name (e.g., "age", "weight", "embedding")
    pub key: String,

    /// Arrow column index in the table (e.g., column 2 for first property after id/labels)
    pub column_index: usize,

    /// Default value when property is missing or null
    pub default_value: DefaultValue,

    /// Expected value type (helps with validation)
    pub value_type: ValueType,
}

impl PropertyConfig {
    /// Creates a new property configuration.
    pub fn new(
        key: impl Into<String>,
        column_index: usize,
        default_value: DefaultValue,
        value_type: ValueType,
    ) -> Self {
        Self {
            key: key.into(),
            column_index,
            default_value,
            value_type,
        }
    }
}
/// Accumulates property values for nodes or relationships during parallel import.
///
/// Stores property values in a sparse HashMap keyed by original entity ID.
/// After all tasks complete, this is converted to dense PropertyValues using the IdMap.
#[derive(Debug)]
pub struct PropertyAccumulator {
    /// Property configuration
    config: PropertyConfig,

    /// Sparse storage: original_id → property value
    /// We store extracted primitive values (i64, f64, Vec<i64>, Vec<f64>, Vec<f32>)
    values: HashMap<OriginalNodeId, PropertyValue>,
}

/// Internal representation of a property value during accumulation.
/// This is converted to PropertyValues after import completes.
#[derive(Debug, Clone)]
enum PropertyValue {
    Long(i64),
    Double(f64),
    LongArray(Vec<i64>),
    DoubleArray(Vec<f64>),
    FloatArray(Vec<f32>),
}

impl PropertyAccumulator {
    /// Creates a new property accumulator with the given configuration.
    pub fn new(config: PropertyConfig) -> Self {
        Self {
            config,
            values: HashMap::new(),
        }
    }

    /// Adds or updates a property value for an entity.
    pub fn set(&mut self, entity_id: OriginalNodeId, value: PropertyValue) {
        self.values.insert(entity_id, value);
    }

    /// Returns the number of accumulated property values.
    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    /// Builds final PropertyValues from accumulated sparse data.
    ///
    /// Uses the IdMap to convert sparse HashMap storage to dense Vec storage,
    /// filling in default values for entities without properties.
    ///
    /// # Arguments
    /// * `id_map` - IdMap for original → mapped ID conversion
    ///
    /// # Returns
    /// Box<dyn PropertyValues> suitable for DefaultGraphStore
    pub fn build(self, id_map: &SimpleIdMap) -> Result<Box<dyn PropertyValues>, ImporterError> {
        let node_count = id_map.node_count();

        match self.config.value_type {
            ValueType::Long => {
                let default = self.config.default_value.long_value().unwrap_or(0);
                let mut dense = vec![default; node_count];

                for (original_id, value) in self.values {
                    if let Some(mapped_id) = id_map.safe_to_mapped_node_id(original_id) {
                        if let PropertyValue::Long(v) = value {
                            dense[mapped_id as usize] = v;
                        }
                    }
                }

                Ok(Box::new(DefaultLongNodePropertyValues::new(
                    dense, node_count,
                )))
            }
            ValueType::Double => {
                let default = self.config.default_value.double_value().unwrap_or(f64::NAN);
                let mut dense = vec![default; node_count];

                for (original_id, value) in self.values {
                    if let Some(mapped_id) = id_map.safe_to_mapped_node_id(original_id) {
                        if let PropertyValue::Double(v) = value {
                            dense[mapped_id as usize] = v;
                        }
                    }
                }

                Ok(Box::new(DefaultDoubleNodePropertyValues::new(
                    dense, node_count,
                )))
            }
            ValueType::LongArray => {
                let mut dense = vec![None; node_count];

                for (original_id, value) in self.values {
                    if let Some(mapped_id) = id_map.safe_to_mapped_node_id(original_id) {
                        if let PropertyValue::LongArray(v) = value {
                            dense[mapped_id as usize] = Some(v);
                        }
                    }
                }

                Ok(Box::new(DefaultLongArrayNodePropertyValues::new(
                    dense, node_count,
                )))
            }
            ValueType::DoubleArray => {
                let mut dense = vec![None; node_count];

                for (original_id, value) in self.values {
                    if let Some(mapped_id) = id_map.safe_to_mapped_node_id(original_id) {
                        if let PropertyValue::DoubleArray(v) = value {
                            dense[mapped_id as usize] = Some(v);
                        }
                    }
                }

                Ok(Box::new(DefaultDoubleArrayNodePropertyValues::new(
                    dense, node_count,
                )))
            }
            ValueType::FloatArray => {
                let mut dense = vec![None; node_count];

                for (original_id, value) in self.values {
                    if let Some(mapped_id) = id_map.safe_to_mapped_node_id(original_id) {
                        if let PropertyValue::FloatArray(v) = value {
                            dense[mapped_id as usize] = Some(v);
                        }
                    }
                }

                Ok(Box::new(DefaultFloatArrayNodePropertyValues::new(
                    dense, node_count,
                )))
            }
            _ => Err(ImporterError::UnsupportedPropertyType {
                property_key: self.config.key.clone(),
                value_type: self.config.value_type,
            }),
        }
    }
}

// ================================================================================================
// NodeAccumulator - Thread-safe accumulator for node data
// ================================================================================================

/// Accumulates node data during parallel import.
///
/// Stores original node IDs, their associated labels, and property values.
/// After all tasks complete, this can be converted into a `SimpleIdMap` and PropertyValues.
///
/// Thread-safety: Wrapped in `Arc<Mutex<_>>` for parallel writes.
#[derive(Debug)]
pub struct NodeAccumulator {
    /// Original node IDs in insertion order
    original_ids: Vec<OriginalNodeId>,
    /// Label sets per node (indexed by insertion order)
    labels_by_node: HashMap<usize, HashSet<NodeLabel>>,
    /// Property accumulators (one per configured property)
    property_accumulators: Vec<PropertyAccumulator>,
}

impl NodeAccumulator {
    /// Creates a new empty accumulator without properties.
    pub fn new() -> Self {
        Self {
            original_ids: Vec::new(),
            labels_by_node: HashMap::new(),
            property_accumulators: Vec::new(),
        }
    }

    /// Creates a new accumulator with property configurations.
    pub fn new_with_properties(property_configs: Vec<PropertyConfig>) -> Self {
        let property_accumulators = property_configs
            .into_iter()
            .map(PropertyAccumulator::new)
            .collect();

        Self {
            original_ids: Vec::new(),
            labels_by_node: HashMap::new(),
            property_accumulators,
        }
    }

    /// Adds a node with its original ID and labels (no properties).
    ///
    /// # Arguments
    /// * `original_id` - Original node ID from source data
    /// * `labels` - Set of labels assigned to this node
    ///
    /// # Returns
    /// The insertion index (will become mapped node ID)
    pub fn add_node(&mut self, original_id: OriginalNodeId, labels: Vec<NodeLabel>) -> usize {
        let index = self.original_ids.len();
        self.original_ids.push(original_id);
        if !labels.is_empty() {
            self.labels_by_node
                .insert(index, labels.into_iter().collect());
        }
        index
    }

    /// Adds a node with properties.
    ///
    /// # Arguments
    /// * `original_id` - Original node ID from source data
    /// * `labels` - Set of labels assigned to this node
    /// * `properties` - Property values in order matching property_accumulators
    ///
    /// # Returns
    /// The insertion index (will become mapped node ID)
    pub fn add_node_with_properties(
        &mut self,
        original_id: OriginalNodeId,
        labels: Vec<NodeLabel>,
        properties: Vec<PropertyValue>,
    ) -> usize {
        let index = self.add_node(original_id, labels);

        // Set properties in each accumulator
        for (accumulator, value) in self.property_accumulators.iter_mut().zip(properties) {
            accumulator.set(original_id, value);
        }

        index
    }

    /// Returns the number of accumulated nodes.
    pub fn node_count(&self) -> usize {
        self.original_ids.len()
    }

    /// Builds a `SimpleIdMap` from accumulated node data.
    ///
    /// Consumes the accumulator and creates the final ID mapping.
    /// Note: If properties are present, call build_properties() before calling this.
    pub fn build_id_map(self) -> SimpleIdMap {
        let mut id_map = SimpleIdMap::from_original_ids(self.original_ids);

        // Add labels to id_map
        for (node_index, label_set) in self.labels_by_node {
            for label in label_set {
                id_map.add_node_label(label.clone());
                id_map.add_node_id_to_label(node_index as MappedNodeId, label);
            }
        }

        id_map
    }

    /// Builds property values from accumulated property data.
    ///
    /// Requires an IdMap to convert sparse property storage to dense PropertyValues.
    /// Returns a HashMap of property key → PropertyValues.
    ///
    /// # Arguments
    /// * `id_map` - IdMap for original → mapped ID conversion
    ///
    /// # Returns
    /// Map of property keys to their PropertyValues
    pub fn build_properties(
        mut self,
        id_map: &SimpleIdMap,
    ) -> Result<HashMap<String, Box<dyn PropertyValues>>, ImporterError> {
        let mut properties = HashMap::new();

        for accumulator in self.property_accumulators.drain(..) {
            let key = accumulator.config.key.clone();
            let values = accumulator.build(id_map)?;
            properties.insert(key, values);
        }

        Ok(properties)
    }
}

impl Default for NodeAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================================================
// EdgeAccumulator - Thread-safe accumulator for edge data
// ================================================================================================

/// Accumulates edge data during parallel import.
///
/// Stores source/target pairs with their relationship types. After all tasks complete,
/// this can be converted into `RelationshipTopology` structures.
///
/// Thread-safety: Wrapped in `Arc<Mutex<_>>` for parallel writes.
#[derive(Debug)]
pub struct EdgeAccumulator {
    /// Edges as (source_original, target_original, relationship_type)
    edges: Vec<(OriginalNodeId, OriginalNodeId, RelationshipType)>,
}

impl EdgeAccumulator {
    /// Creates a new empty accumulator.
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    /// Adds an edge with source, target, and relationship type.
    ///
    /// # Arguments
    /// * `source_original` - Original source node ID
    /// * `target_original` - Original target node ID
    /// * `rel_type` - Relationship type
    pub fn add_edge(
        &mut self,
        source_original: OriginalNodeId,
        target_original: OriginalNodeId,
        rel_type: RelationshipType,
    ) {
        self.edges
            .push((source_original, target_original, rel_type));
    }

    /// Returns the number of accumulated edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Builds relationship topologies from accumulated edge data.
    ///
    /// Consumes the accumulator and creates topologies grouped by relationship type.
    /// Requires a pre-built IdMap to convert original IDs to mapped IDs.
    ///
    /// # Arguments
    /// * `id_map` - IdMap for original → mapped ID conversion
    ///
    /// # Returns
    /// Map of relationship types to their topologies
    pub fn build_topology(
        self,
        id_map: &SimpleIdMap,
    ) -> Result<HashMap<RelationshipType, RelationshipTopology>, ImporterError> {
        let node_count = id_map.node_count();

        // Group edges by relationship type
        let mut edges_by_type: HashMap<RelationshipType, Vec<(MappedNodeId, MappedNodeId)>> =
            HashMap::new();

        for (source_orig, target_orig, rel_type) in self.edges {
            // Map original IDs to mapped IDs
            let source_mapped = id_map.safe_to_mapped_node_id(source_orig).ok_or_else(|| {
                ImporterError::InvalidNodeId {
                    original_id: source_orig,
                }
            })?;

            let target_mapped = id_map.safe_to_mapped_node_id(target_orig).ok_or_else(|| {
                ImporterError::InvalidNodeId {
                    original_id: target_orig,
                }
            })?;

            edges_by_type
                .entry(rel_type)
                .or_default()
                .push((source_mapped, target_mapped));
        }

        // Build topologies
        let mut topologies = HashMap::new();

        for (rel_type, edges) in edges_by_type {
            // Build outgoing adjacency lists
            let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

            for (source, target) in edges {
                outgoing[source as usize].push(target);
            }

            let topology = RelationshipTopology::new(outgoing, None);
            topologies.insert(rel_type, topology);
        }

        Ok(topologies)
    }
}

impl Default for EdgeAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================================================
// NodeImportTask - ImportTask implementation for nodes
// ================================================================================================

/// Import task for scanning and accumulating node data.
///
/// Reads batches from a scanner cursor, extracts node IDs and labels from Arrow columns,
/// and writes them to a shared accumulator.
pub struct NodeImportTask {
    task_index: usize,
    accumulator: Arc<Mutex<NodeAccumulator>>,
}

impl NodeImportTask {
    /// Creates a new node import task.
    pub fn new(task_index: usize, accumulator: Arc<Mutex<NodeAccumulator>>) -> Self {
        Self {
            task_index,
            accumulator,
        }
    }
}

impl ImportTask for NodeImportTask {
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError> {
        let mut records_imported = 0u64;

        while cursor.reserve_batch() {
            // Process batch
            let mut batch_result: Result<u64, TaskError> = Ok(0);

            cursor.consume_batch(&mut |batch: &ArrowBatchReference| {
                match process_node_batch(batch, &self.accumulator) {
                    Ok(count) => {
                        batch_result = Ok(count);
                        true // Continue
                    }
                    Err(e) => {
                        batch_result = Err(TaskError::ExecutionFailed {
                            message: e.to_string(),
                        });
                        false // Stop
                    }
                }
            });

            records_imported += batch_result?;
        }

        Ok((records_imported, 0)) // No properties yet
    }

    fn task_name(&self) -> String {
        format!("node-import-task-{}", self.task_index)
    }

    fn task_index(&self) -> usize {
        self.task_index
    }
}

// ================================================================================================
// Property Extraction Helpers (Phase 6)
// ================================================================================================

/// Extracts a property value from an Arrow column at the given row index.
///
/// Handles type conversion from Arrow types to PropertyValue and applies
/// default values when the cell is null.
fn extract_property_value(
    batch: &ArrowBatchReference,
    config: &PropertyConfig,
    row_index: usize,
) -> Result<PropertyValue, ImporterError> {
    let column = batch.column(config.column_index).ok_or_else(|| {
        ImporterError::PropertyColumnOutOfBounds {
            property_key: config.key.clone(),
            column_index: config.column_index,
            column_count: batch.chunk().arrays().len(),
        }
    })?;

    match column.data_type() {
        DataType::Int64 => {
            let array = column
                .as_any()
                .downcast_ref::<Int64Array>()
                .ok_or_else(|| ImporterError::PropertyTypeMismatch {
                    property_key: config.key.clone(),
                    expected: ValueType::Long,
                    actual: format!("{:?}", column.data_type()),
                })?;

            if array.is_null(row_index) {
                Ok(PropertyValue::Long(
                    config.default_value.long_value().unwrap_or(0),
                ))
            } else {
                Ok(PropertyValue::Long(array.value(row_index)))
            }
        }
        DataType::Float64 => {
            let array = column
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| ImporterError::PropertyTypeMismatch {
                    property_key: config.key.clone(),
                    expected: ValueType::Double,
                    actual: format!("{:?}", column.data_type()),
                })?;

            if array.is_null(row_index) {
                Ok(PropertyValue::Double(
                    config.default_value.double_value().unwrap_or(f64::NAN),
                ))
            } else {
                Ok(PropertyValue::Double(array.value(row_index)))
            }
        }
        DataType::Float32 => {
            let array = column
                .as_any()
                .downcast_ref::<Float32Array>()
                .ok_or_else(|| ImporterError::PropertyTypeMismatch {
                    property_key: config.key.clone(),
                    expected: ValueType::Double,
                    actual: format!("{:?}", column.data_type()),
                })?;

            if array.is_null(row_index) {
                Ok(PropertyValue::Double(
                    config.default_value.double_value().unwrap_or(f64::NAN),
                ))
            } else {
                Ok(PropertyValue::Double(array.value(row_index) as f64))
            }
        }
        DataType::List(_) => extract_list_property(column.as_ref(), config, row_index),
        _ => Err(ImporterError::UnsupportedPropertyType {
            property_key: config.key.clone(),
            value_type: config.value_type,
        }),
    }
}

/// Extracts a list/array property value from an Arrow List column.
fn extract_list_property(
    column: &dyn Array,
    config: &PropertyConfig,
    row_index: usize,
) -> Result<PropertyValue, ImporterError> {
    let list_array = column
        .as_any()
        .downcast_ref::<ListArray<i32>>()
        .ok_or_else(|| ImporterError::PropertyTypeMismatch {
            property_key: config.key.clone(),
            expected: config.value_type,
            actual: format!("{:?}", column.data_type()),
        })?;

    if list_array.is_null(row_index) {
        // Return default based on value type
        return match config.value_type {
            ValueType::LongArray => Ok(PropertyValue::LongArray(vec![])),
            ValueType::DoubleArray => Ok(PropertyValue::DoubleArray(vec![])),
            ValueType::FloatArray => Ok(PropertyValue::FloatArray(vec![])),
            _ => Err(ImporterError::UnsupportedPropertyType {
                property_key: config.key.clone(),
                value_type: config.value_type,
            }),
        };
    }

    let offsets = list_array.offsets().as_slice();
    let start = offsets[row_index] as usize;
    let end = offsets[row_index + 1] as usize;
    let values = list_array.values();

    // Try to downcast to specific array types
    if let Some(int_array) = values.as_any().downcast_ref::<Int64Array>() {
        let vec: Vec<i64> = (start..end).map(|i| int_array.value(i)).collect();
        Ok(PropertyValue::LongArray(vec))
    } else if let Some(double_array) = values.as_any().downcast_ref::<Float64Array>() {
        let vec: Vec<f64> = (start..end).map(|i| double_array.value(i)).collect();
        Ok(PropertyValue::DoubleArray(vec))
    } else if let Some(float_array) = values.as_any().downcast_ref::<Float32Array>() {
        let vec: Vec<f32> = (start..end).map(|i| float_array.value(i)).collect();
        Ok(PropertyValue::FloatArray(vec))
    } else {
        Err(ImporterError::UnsupportedPropertyType {
            property_key: config.key.clone(),
            value_type: config.value_type,
        })
    }
}

/// Processes a single node batch, extracting and accumulating node data.
fn process_node_batch(
    batch: &ArrowBatchReference,
    accumulator: &Arc<Mutex<NodeAccumulator>>,
) -> Result<u64, ImporterError> {
    // Extract ID column (required, index 0)
    let id_column = batch.int64_column(0).ok_or(ImporterError::MissingColumn {
        column_name: "id".to_string(),
    })?;

    // Extract label column (optional, index 1)
    let label_column = batch.utf8_column(1);

    let mut count = 0u64;
    let mut acc = accumulator.lock().map_err(|e| ImporterError::LockError {
        message: e.to_string(),
    })?;

    // Process each row in the batch range
    for i in batch.start_offset()..batch.end_offset() {
        let original_id = id_column.value(i);

        // Extract labels if column exists
        let labels = if let Some(label_col) = label_column {
            let label_str = label_col.value(i);
            vec![NodeLabel::of(label_str)]
        } else {
            vec![]
        };

        acc.add_node(original_id, labels);
        count += 1;
    }

    Ok(count)
}

/// Processes a single node batch and offers each record to a provided consumer.
///
/// Returns Ok(true) if the entire batch was accepted by the consumer, Ok(false)
/// if the consumer signalled backpressure (rejected a record). This function
/// mirrors `process_node_batch` but routes records to a consumer instead of an
/// accumulator.
fn process_node_batch_with_consumer(
    batch: &ArrowBatchReference,
    consumer: &mut dyn super::consumer::RecordConsumer<super::consumer::NodeRecord>,
) -> Result<bool, ImporterError> {
    // Extract ID column (required, index 0)
    let id_column = batch.int64_column(0).ok_or(ImporterError::MissingColumn {
        column_name: "id".to_string(),
    })?;

    // Extract label column (optional, index 1)
    let label_column = batch.utf8_column(1);

    // Process each row in the batch range
    for i in batch.start_offset()..batch.end_offset() {
        let original_id = id_column.value(i);

        // Extract labels if column exists
        let labels = if let Some(label_col) = label_column {
            let label_str = label_col.value(i);
            vec![NodeLabel::of(label_str)]
        } else {
            vec![]
        };

        let record = super::consumer::NodeRecord {
            node_id: original_id as u64,
            labels,
        };

        // Offer to consumer; if it returns false, signal backpressure
        let offered = consumer.offer(record);
        if !offered {
            return Ok(false);
        }
    }

    Ok(true)
}

// ================================================================================================
// EdgeImportTask - ImportTask implementation for edges
// ================================================================================================

/// Import task for scanning and accumulating edge data.
///
/// Reads batches from a scanner cursor, extracts source/target IDs and relationship types
/// from Arrow columns, and writes them to a shared accumulator.
pub struct EdgeImportTask {
    task_index: usize,
    accumulator: Arc<Mutex<EdgeAccumulator>>,
    #[allow(dead_code)]
    id_map: Arc<SimpleIdMap>, // For validation (future use)
}

impl EdgeImportTask {
    /// Creates a new edge import task.
    pub fn new(
        task_index: usize,
        accumulator: Arc<Mutex<EdgeAccumulator>>,
        id_map: Arc<SimpleIdMap>,
    ) -> Self {
        Self {
            task_index,
            accumulator,
            id_map,
        }
    }
}

impl ImportTask for EdgeImportTask {
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError> {
        let mut records_imported = 0u64;

        while cursor.reserve_batch() {
            // Process batch
            let mut batch_result: Result<u64, TaskError> = Ok(0);

            cursor.consume_batch(&mut |batch: &ArrowBatchReference| {
                match process_edge_batch(batch, &self.accumulator) {
                    Ok(count) => {
                        batch_result = Ok(count);
                        true // Continue
                    }
                    Err(e) => {
                        batch_result = Err(TaskError::ExecutionFailed {
                            message: e.to_string(),
                        });
                        false // Stop
                    }
                }
            });

            records_imported += batch_result?;
        }

        Ok((records_imported, 0)) // No properties yet
    }

    fn task_name(&self) -> String {
        format!("edge-import-task-{}", self.task_index)
    }

    fn task_index(&self) -> usize {
        self.task_index
    }
}

/// Processes a single edge batch, extracting and accumulating edge data.
fn process_edge_batch(
    batch: &ArrowBatchReference,
    accumulator: &Arc<Mutex<EdgeAccumulator>>,
) -> Result<u64, ImporterError> {
    // Extract source column (required, index 0 or named "source")
    let source_column = batch.int64_column(0).ok_or(ImporterError::MissingColumn {
        column_name: "source".to_string(),
    })?;

    // Extract target column (required, index 1 or named "target")
    let target_column = batch.int64_column(1).ok_or(ImporterError::MissingColumn {
        column_name: "target".to_string(),
    })?;

    // Extract type column (optional, index 2)
    let type_column = batch.utf8_column(2);
    let default_rel_type = RelationshipType::of("RELATED");

    let mut count = 0u64;
    let mut acc = accumulator.lock().map_err(|e| ImporterError::LockError {
        message: e.to_string(),
    })?;

    // Process each row in the batch range
    for i in batch.start_offset()..batch.end_offset() {
        let source_id = source_column.value(i);
        let target_id = target_column.value(i);

        // Extract relationship type if column exists
        let rel_type = if let Some(type_col) = type_column {
            let type_str = type_col.value(i);
            RelationshipType::of(type_str)
        } else {
            default_rel_type.clone()
        };

        acc.add_edge(source_id, target_id, rel_type);
        count += 1;
    }

    Ok(count)
}

/// Processes a single edge batch and offers each relationship record to a provided consumer.
///
/// Returns Ok(true) if the entire batch was accepted by the consumer, Ok(false)
/// if the consumer signalled backpressure (rejected a record). This mirrors
/// `process_edge_batch` but routes records to a consumer instead of an accumulator.
fn process_edge_batch_with_consumer(
    batch: &ArrowBatchReference,
    consumer: &mut dyn super::consumer::RecordConsumer<super::consumer::RelationshipRecord>,
) -> Result<bool, ImporterError> {
    // Extract source column (required, index 0 or named "source")
    let source_column = batch.int64_column(0).ok_or(ImporterError::MissingColumn {
        column_name: "source".to_string(),
    })?;

    // Extract target column (required, index 1 or named "target")
    let target_column = batch.int64_column(1).ok_or(ImporterError::MissingColumn {
        column_name: "target".to_string(),
    })?;

    // Extract type column (optional, index 2)
    let type_column = batch.utf8_column(2);
    let default_rel_type = RelationshipType::of("RELATED");

    // Process each row in the batch range
    for i in batch.start_offset()..batch.end_offset() {
        let source_id = source_column.value(i);
        let target_id = target_column.value(i);

        // Extract relationship type if column exists
        let rel_type = if let Some(type_col) = type_column {
            let type_str = type_col.value(i);
            RelationshipType::of(type_str)
        } else {
            default_rel_type.clone()
        };

        let record = super::consumer::RelationshipRecord {
            relationship_id: 0, // relationship id not available in this schema
            source_node_id: source_id as u64,
            target_node_id: target_id as u64,
            relationship_type: rel_type,
        };

        let offered = consumer.offer(record);
        if !offered {
            return Ok(false);
        }
    }

    Ok(true)
}

// ================================================================================================
// Task Factories
// ================================================================================================

/// Factory for creating NodeImportTask instances.
pub struct NodeImportTaskFactory {
    accumulator: Arc<Mutex<NodeAccumulator>>,
}

impl NodeImportTaskFactory {
    /// Creates a new factory with the given accumulator.
    pub fn new(accumulator: Arc<Mutex<NodeAccumulator>>) -> Self {
        Self { accumulator }
    }
}

impl TaskFactory for NodeImportTaskFactory {
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError> {
        Ok(Box::new(NodeImportTask::new(
            task_index,
            self.accumulator.clone(),
        )))
    }
}

/// Factory for creating EdgeImportTask instances.
pub struct EdgeImportTaskFactory {
    accumulator: Arc<Mutex<EdgeAccumulator>>,
    id_map: Arc<SimpleIdMap>,
}

impl EdgeImportTaskFactory {
    /// Creates a new factory with the given accumulator and ID map.
    pub fn new(accumulator: Arc<Mutex<EdgeAccumulator>>, id_map: Arc<SimpleIdMap>) -> Self {
        Self {
            accumulator,
            id_map,
        }
    }
}

impl TaskFactory for EdgeImportTaskFactory {
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError> {
        Ok(Box::new(EdgeImportTask::new(
            task_index,
            self.accumulator.clone(),
            self.id_map.clone(),
        )))
    }
}

// ================================================================================================
// Error Types
// ================================================================================================

/// Errors that can occur during import operations.
#[derive(Debug, Clone)]
pub enum ImporterError {
    /// Missing required column in Arrow batch
    MissingColumn { column_name: String },
    /// Invalid node ID (not found in IdMap)
    InvalidNodeId { original_id: OriginalNodeId },
    /// Lock acquisition failed
    LockError { message: String },
    /// Unsupported property type
    UnsupportedPropertyType {
        property_key: String,
        value_type: ValueType,
    },
    /// Property type mismatch (Arrow type doesn't match expected type)
    PropertyTypeMismatch {
        property_key: String,
        expected: ValueType,
        actual: String,
    },
    /// Property column index out of bounds
    PropertyColumnOutOfBounds {
        property_key: String,
        column_index: usize,
        column_count: usize,
    },
}

impl std::fmt::Display for ImporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImporterError::MissingColumn { column_name } => {
                write!(f, "Missing required column: {}", column_name)
            }
            ImporterError::InvalidNodeId { original_id } => {
                write!(f, "Invalid node ID: {} not found in IdMap", original_id)
            }
            ImporterError::LockError { message } => {
                write!(f, "Lock error: {}", message)
            }
            ImporterError::UnsupportedPropertyType {
                property_key,
                value_type,
            } => {
                write!(
                    f,
                    "Unsupported property type for '{}': {:?}",
                    property_key, value_type
                )
            }
            ImporterError::PropertyTypeMismatch {
                property_key,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Property '{}' type mismatch: expected {:?}, got {}",
                    property_key, expected, actual
                )
            }
            ImporterError::PropertyColumnOutOfBounds {
                property_key,
                column_index,
                column_count,
            } => {
                write!(
                    f,
                    "Property '{}' column index {} out of bounds (table has {} columns)",
                    property_key, column_index, column_count
                )
            }
        }
    }
}

impl std::error::Error for ImporterError {}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::id_map::PartialIdMap; // Trait for to_mapped_node_id

    #[test]
    fn test_node_accumulator_empty() {
        let acc = NodeAccumulator::new();
        assert_eq!(acc.node_count(), 0);
    }

    #[test]
    fn test_node_accumulator_add_node() {
        let mut acc = NodeAccumulator::new();
        let idx = acc.add_node(100, vec![NodeLabel::of("Person")]);
        assert_eq!(idx, 0);
        assert_eq!(acc.node_count(), 1);

        let idx2 = acc.add_node(101, vec![NodeLabel::of("Company")]);
        assert_eq!(idx2, 1);
        assert_eq!(acc.node_count(), 2);
    }

    #[test]
    fn test_node_accumulator_build_id_map() {
        let mut acc = NodeAccumulator::new();
        acc.add_node(100, vec![NodeLabel::of("Person")]);
        acc.add_node(101, vec![NodeLabel::of("Person")]);
        acc.add_node(200, vec![NodeLabel::of("Company")]);

        let id_map = acc.build_id_map();
        assert_eq!(id_map.node_count(), 3);
        assert_eq!(id_map.to_mapped_node_id(100), Some(0));
        assert_eq!(id_map.to_mapped_node_id(101), Some(1));
        assert_eq!(id_map.to_mapped_node_id(200), Some(2));

        let person_label = NodeLabel::of("Person");
        assert_eq!(id_map.node_count_for_label(&person_label), 2);
    }

    #[test]
    fn test_edge_accumulator_empty() {
        let acc = EdgeAccumulator::new();
        assert_eq!(acc.edge_count(), 0);
    }

    #[test]
    fn test_edge_accumulator_add_edge() {
        let mut acc = EdgeAccumulator::new();
        acc.add_edge(100, 101, RelationshipType::of("KNOWS"));
        assert_eq!(acc.edge_count(), 1);

        acc.add_edge(101, 200, RelationshipType::of("WORKS_AT"));
        assert_eq!(acc.edge_count(), 2);
    }

    #[test]
    fn test_edge_accumulator_build_topology() {
        // Create IdMap
        let id_map = SimpleIdMap::from_original_ids([100, 101, 200]);

        // Create accumulator with edges
        let mut acc = EdgeAccumulator::new();
        acc.add_edge(100, 101, RelationshipType::of("KNOWS"));
        acc.add_edge(100, 200, RelationshipType::of("KNOWS"));
        acc.add_edge(101, 200, RelationshipType::of("WORKS_AT"));

        // Build topologies
        let topologies = acc.build_topology(&id_map).unwrap();
        assert_eq!(topologies.len(), 2);

        // Check KNOWS topology
        let knows = topologies.get(&RelationshipType::of("KNOWS")).unwrap();
        assert_eq!(knows.relationship_count(), 2);
        assert_eq!(knows.outgoing(0).unwrap(), &[1, 2]);

        // Check WORKS_AT topology
        let works_at = topologies.get(&RelationshipType::of("WORKS_AT")).unwrap();
        assert_eq!(works_at.relationship_count(), 1);
        assert_eq!(works_at.outgoing(1).unwrap(), &[2]);
    }

    #[test]
    fn test_edge_accumulator_invalid_node_id() {
        let id_map = SimpleIdMap::from_original_ids([100, 101]);
        let mut acc = EdgeAccumulator::new();
        acc.add_edge(100, 999, RelationshipType::of("KNOWS")); // 999 not in id_map

        let result = acc.build_topology(&id_map);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ImporterError::InvalidNodeId { original_id: 999 }
        ));
    }

    #[test]
    fn test_node_import_task_factory() {
        let acc = Arc::new(Mutex::new(NodeAccumulator::new()));
        let factory = NodeImportTaskFactory::new(acc);

        let task = factory.create_task(0).unwrap();
        assert_eq!(task.task_index(), 0);
        assert_eq!(task.task_name(), "node-import-task-0");
    }

    #[test]
    fn test_edge_import_task_factory() {
        let acc = Arc::new(Mutex::new(EdgeAccumulator::new()));
        let id_map = Arc::new(SimpleIdMap::from_original_ids([1, 2, 3]));
        let factory = EdgeImportTaskFactory::new(acc, id_map);

        let task = factory.create_task(1).unwrap();
        assert_eq!(task.task_index(), 1);
        assert_eq!(task.task_name(), "edge-import-task-1");
    }

    // ========== Phase 6: Property Tests ==========

    #[test]
    fn test_property_config_creation() {
        let config = PropertyConfig::new("age", 2, DefaultValue::long(42), ValueType::Long);
        assert_eq!(config.key, "age");
        assert_eq!(config.column_index, 2);
        assert_eq!(config.value_type, ValueType::Long);
    }

    #[test]
    fn test_property_accumulator_empty() {
        let config = PropertyConfig::new("age", 2, DefaultValue::long(0), ValueType::Long);
        let acc = PropertyAccumulator::new(config);
        assert_eq!(acc.value_count(), 0);
    }

    #[test]
    fn test_property_accumulator_add_long() {
        let config = PropertyConfig::new("age", 2, DefaultValue::long(0), ValueType::Long);
        let mut acc = PropertyAccumulator::new(config);
        acc.set(100, PropertyValue::Long(42));
        acc.set(101, PropertyValue::Long(35));
        assert_eq!(acc.value_count(), 2);
    }

    #[test]
    fn test_property_accumulator_add_double() {
        let config = PropertyConfig::new("weight", 2, DefaultValue::double(0.0), ValueType::Double);
        let mut acc = PropertyAccumulator::new(config);
        acc.set(100, PropertyValue::Double(1.5));
        acc.set(101, PropertyValue::Double(2.7));
        assert_eq!(acc.value_count(), 2);
    }

    #[test]
    fn test_property_accumulator_add_long_array() {
        let config = PropertyConfig::new("tags", 2, DefaultValue::null(), ValueType::LongArray);
        let mut acc = PropertyAccumulator::new(config);
        acc.set(100, PropertyValue::LongArray(vec![1, 2, 3]));
        acc.set(101, PropertyValue::LongArray(vec![4, 5]));
        assert_eq!(acc.value_count(), 2);
    }

    #[test]
    fn test_property_accumulator_build_long() {
        let config = PropertyConfig::new("age", 2, DefaultValue::long(-1), ValueType::Long);
        let mut acc = PropertyAccumulator::new(config);

        // Add some values
        acc.set(100, PropertyValue::Long(42));
        acc.set(101, PropertyValue::Long(35));

        // Build with IdMap
        let id_map = SimpleIdMap::from_original_ids([100, 101, 102]);
        let property_values = acc.build(&id_map).unwrap();

        assert_eq!(property_values.value_type(), ValueType::Long);
        assert_eq!(property_values.element_count(), 3);
    }

    #[test]
    fn test_property_accumulator_build_double() {
        let config = PropertyConfig::new(
            "weight",
            2,
            DefaultValue::double(f64::NAN),
            ValueType::Double,
        );
        let mut acc = PropertyAccumulator::new(config);

        acc.set(100, PropertyValue::Double(1.5));
        acc.set(102, PropertyValue::Double(2.7));

        let id_map = SimpleIdMap::from_original_ids([100, 101, 102]);
        let property_values = acc.build(&id_map).unwrap();

        assert_eq!(property_values.value_type(), ValueType::Double);
        assert_eq!(property_values.element_count(), 3);
    }

    #[test]
    fn test_property_accumulator_build_long_array() {
        let config =
            PropertyConfig::new("embedding", 2, DefaultValue::null(), ValueType::LongArray);
        let mut acc = PropertyAccumulator::new(config);

        acc.set(100, PropertyValue::LongArray(vec![1, 2, 3]));
        acc.set(101, PropertyValue::LongArray(vec![4, 5, 6]));

        let id_map = SimpleIdMap::from_original_ids([100, 101]);
        let property_values = acc.build(&id_map).unwrap();

        assert_eq!(property_values.value_type(), ValueType::LongArray);
        assert_eq!(property_values.element_count(), 2);
    }

    #[test]
    fn test_property_accumulator_build_double_array() {
        let config =
            PropertyConfig::new("embedding", 2, DefaultValue::null(), ValueType::DoubleArray);
        let mut acc = PropertyAccumulator::new(config);

        acc.set(100, PropertyValue::DoubleArray(vec![1.0, 2.0, 3.0]));

        let id_map = SimpleIdMap::from_original_ids([100, 101]);
        let property_values = acc.build(&id_map).unwrap();

        assert_eq!(property_values.value_type(), ValueType::DoubleArray);
        assert_eq!(property_values.element_count(), 2);
    }

    #[test]
    fn test_property_accumulator_build_float_array() {
        let config =
            PropertyConfig::new("embedding", 2, DefaultValue::null(), ValueType::FloatArray);
        let mut acc = PropertyAccumulator::new(config);

        acc.set(100, PropertyValue::FloatArray(vec![1.0, 2.0, 3.0]));

        let id_map = SimpleIdMap::from_original_ids([100, 101]);
        let property_values = acc.build(&id_map).unwrap();

        assert_eq!(property_values.value_type(), ValueType::FloatArray);
        assert_eq!(property_values.element_count(), 2);
    }

    #[test]
    fn test_node_accumulator_with_properties() {
        let property_configs = vec![
            PropertyConfig::new("age", 2, DefaultValue::long(0), ValueType::Long),
            PropertyConfig::new("weight", 3, DefaultValue::double(0.0), ValueType::Double),
        ];

        let acc = NodeAccumulator::new_with_properties(property_configs);
        assert_eq!(acc.node_count(), 0);
        assert_eq!(acc.property_accumulators.len(), 2);
    }

    #[test]
    fn test_node_accumulator_add_node_with_properties() {
        let property_configs = vec![PropertyConfig::new(
            "age",
            2,
            DefaultValue::long(0),
            ValueType::Long,
        )];

        let mut acc = NodeAccumulator::new_with_properties(property_configs);

        let properties = vec![PropertyValue::Long(42)];
        let idx = acc.add_node_with_properties(100, vec![NodeLabel::of("Person")], properties);

        assert_eq!(idx, 0);
        assert_eq!(acc.node_count(), 1);
        assert_eq!(acc.property_accumulators[0].value_count(), 1);
    }

    #[test]
    fn test_node_accumulator_build_properties() {
        let property_configs = vec![
            PropertyConfig::new("age", 2, DefaultValue::long(-1), ValueType::Long),
            PropertyConfig::new("weight", 3, DefaultValue::double(0.0), ValueType::Double),
        ];

        let mut acc = NodeAccumulator::new_with_properties(property_configs);

        // Add nodes with properties
        let properties1 = vec![PropertyValue::Long(42), PropertyValue::Double(70.5)];
        acc.add_node_with_properties(100, vec![NodeLabel::of("Person")], properties1);

        let properties2 = vec![PropertyValue::Long(35), PropertyValue::Double(65.0)];
        acc.add_node_with_properties(101, vec![NodeLabel::of("Person")], properties2);

        // Build IdMap first
        let id_map = SimpleIdMap::from_original_ids([100, 101]);

        // Build properties
        let property_map = acc.build_properties(&id_map).unwrap();

        assert_eq!(property_map.len(), 2);
        assert!(property_map.contains_key("age"));
        assert!(property_map.contains_key("weight"));

        let age_values = property_map.get("age").unwrap();
        assert_eq!(age_values.value_type(), ValueType::Long);
        assert_eq!(age_values.element_count(), 2);

        let weight_values = property_map.get("weight").unwrap();
        assert_eq!(weight_values.value_type(), ValueType::Double);
        assert_eq!(weight_values.element_count(), 2);
    }

    #[test]
    fn test_node_accumulator_property_defaults() {
        let property_configs = vec![PropertyConfig::new(
            "age",
            2,
            DefaultValue::long(99),
            ValueType::Long,
        )];

        let mut acc = NodeAccumulator::new_with_properties(property_configs);

        // Add node with property
        let properties = vec![PropertyValue::Long(42)];
        acc.add_node_with_properties(100, vec![NodeLabel::of("Person")], properties);

        // Add node without property (will use default)
        acc.add_node(101, vec![NodeLabel::of("Person")]);

        let id_map = SimpleIdMap::from_original_ids([100, 101]);
        let property_map = acc.build_properties(&id_map).unwrap();

        let age_values = property_map.get("age").unwrap();
        assert_eq!(age_values.element_count(), 2);
    }

    #[test]
    fn test_property_accumulator_unsupported_type() {
        let config = PropertyConfig::new(
            "data",
            2,
            DefaultValue::null(),
            ValueType::String, // Unsupported
        );
        let acc = PropertyAccumulator::new(config);

        let id_map = SimpleIdMap::from_original_ids([100]);
        let result = acc.build(&id_map);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ImporterError::UnsupportedPropertyType { .. }
        ));
    }
}
