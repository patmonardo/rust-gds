# Phase 6: Arrow Property Mapping

**Status**: In Progress  
**Created**: 2025-01-XX  
**Strategy**: GAMMA (Simple accumulation, defer incremental builders)

## Overview

Extend Phase 5's NodeAccumulator and EdgeAccumulator to handle property columns from Arrow tables. Convert Arrow column types to rust-gds PropertyValues and build PropertyStore-compatible structures.

## Java GDS Reference Architecture

### NativeNodePropertyImporter Pattern

**Key Components**:

1. **BuildersByLabel**: Maps NodeLabel → PropertyMapping → NodePropertiesFromStoreBuilder
2. **BuildersByLabelIdAndPropertyId**: Optimized lookup structure (LabelId → PropertyId → Builders)
3. **NodePropertiesFromStoreBuilder**: Accumulates property values with default handling
4. **PropertyMapping**: Configuration (property key, default value, aggregation)

**Flow**:

```java
// Setup (once):
propertyMappings.forEach(mapping -> {
    var builder = NodePropertiesFromStoreBuilder.of(mapping.defaultValue(), concurrency);
    buildersByLabel.put(label, mapping, builder);
});

// Import (per node):
for each property in cursor {
    var builder = lookup(labelId, propertyId);
    var gdsValue = GdsNeo4jValueConverter.toValue(neoValue);
    builder.set(neoNodeId, gdsValue);
}

// Build (once):
buildersByPropertyKey.forEach((key, builder) -> {
    propertyValues = builder.build(idMap);
});
```

### NativeRelationshipPropertyReadHelper Pattern

**Key Points**:

1. **Aggregation Support**: NONE, SUM, MIN, MAX, COUNT
2. **Default Values**: Used when property missing or NO_VALUE
3. **Type Coercion**: NumberValue → double, fail on unsupported types
4. **Array Properties**: Support for int[], long[], double[], float[]

**Flow**:

```java
// Read properties into array:
double[] properties = new double[propertyCount];
Arrays.setAll(properties, i -> aggregations[i].emptyValue(defaultValues[i]));

while (cursor.next()) {
    for (int i = 0; i < propertyIds.length; i++) {
        if (cursor.propertyKey() == propertyIds[i]) {
            double value = extractValue(cursor.propertyValue(), defaultValues[i]);
            properties[i] = aggregations[i].normalizePropertyValue(value);
        }
    }
}
```

## Rust GDS Property System

### Existing PropertyValues Trait

**File**: `src/types/properties/property_values.rs`

```rust
pub trait PropertyValues: Send + Sync {
    fn value_type(&self) -> ValueType;
    fn get(&self, index: usize) -> Option<Value>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
}
```

**Concrete Implementations**:

- `LongPropertyValues`: Vec<Option<i64>>
- `DoublePropertyValues`: Vec<Option<f64>>
- `LongArrayPropertyValues`: Vec<Option<Vec<i64>>>
- `DoubleArrayPropertyValues`: Vec<Option<Vec<f64>>>
- `FloatArrayPropertyValues`: Vec<Option<Vec<f32>>>

### PropertyStore Pattern

**From existing code**:

```rust
// DefaultGraphStore stores properties:
node_properties: HashMap<PropertyKey, Box<dyn PropertyValues>>,
relationship_properties: HashMap<PropertyKey, Box<dyn PropertyValues>>,
```

## Phase 6 Design (GAMMA Strategy)

### 1. PropertyConfig Structure

**Purpose**: Configuration for property loading from Arrow columns

```rust
#[derive(Debug, Clone)]
pub struct PropertyConfig {
    /// Property key/name (e.g., "age", "weight")
    pub key: String,

    /// Arrow column index in the table
    pub column_index: usize,

    /// Default value when property is missing or null
    pub default_value: DefaultValue,

    /// Aggregation mode (for relationships)
    pub aggregation: Aggregation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aggregation {
    None,    // No aggregation (use first/last)
    Sum,     // Sum all values
    Min,     // Minimum value
    Max,     // Maximum value
    Count,   // Count occurrences
}
```

### 2. PropertyAccumulator Structure

**Purpose**: Accumulates property values during parallel import

```rust
pub struct PropertyAccumulator {
    /// Property configuration
    config: PropertyConfig,

    /// Accumulated values (sparse storage)
    /// Key: original node/edge ID
    /// Value: accumulated property value
    values: HashMap<i64, Value>,
}

impl PropertyAccumulator {
    pub fn new(config: PropertyConfig) -> Self;

    /// Add/update property value for entity
    pub fn add(&mut self, entity_id: i64, value: Value);

    /// Build final PropertyValues with IdMap for dense storage
    pub fn build(self, id_map: &SimpleIdMap) -> Result<Box<dyn PropertyValues>>;
}
```

### 3. Extended NodeAccumulator

**Add property handling**:

```rust
pub struct NodeAccumulator {
    // Existing Phase 5 fields:
    original_ids: Vec<i64>,
    labels_by_node: HashMap<i64, Vec<NodeLabel>>,

    // NEW: Property accumulators
    property_accumulators: Vec<PropertyAccumulator>,
}

impl NodeAccumulator {
    pub fn new_with_properties(property_configs: Vec<PropertyConfig>) -> Self;

    pub fn add_node_with_properties(
        &mut self,
        original_id: i64,
        labels: Vec<NodeLabel>,
        properties: Vec<Value>,
    );

    pub fn build_properties(
        self,
        id_map: &SimpleIdMap,
    ) -> Result<HashMap<PropertyKey, Box<dyn PropertyValues>>>;
}
```

### 4. Extended EdgeAccumulator

**Add property handling**:

```rust
pub struct EdgeAccumulator {
    // Existing Phase 5 fields:
    edges: Vec<(i64, i64, RelationshipType)>,

    // NEW: Property accumulators per relationship type
    property_accumulators: HashMap<RelationshipType, Vec<PropertyAccumulator>>,
}

impl EdgeAccumulator {
    pub fn new_with_properties(
        property_configs: HashMap<RelationshipType, Vec<PropertyConfig>>
    ) -> Self;

    pub fn add_edge_with_properties(
        &mut self,
        source: i64,
        target: i64,
        rel_type: RelationshipType,
        properties: Vec<Value>,
    );

    pub fn build_properties(
        &self,
        id_map: &SimpleIdMap,
        topologies: &HashMap<RelationshipType, RelationshipTopology>,
    ) -> Result<HashMap<PropertyKey, Box<dyn PropertyValues>>>;
}
```

### 5. Arrow Column → Value Conversion

**Purpose**: Convert Arrow column types to rust-gds Value

```rust
pub fn extract_property_value(
    batch: &ArrowBatchReference,
    column_index: usize,
    row_index: usize,
    default_value: &DefaultValue,
) -> Result<Value> {
    let column = batch.column(column_index)?;

    match column.data_type() {
        DataType::Int64 => {
            let array = column.as_any().downcast_ref::<Int64Array>()
                .ok_or(PropertyError::TypeMismatch)?;

            if array.is_null(row_index) {
                Ok(default_value.to_value())
            } else {
                Ok(Value::Long(array.value(row_index)))
            }
        }
        DataType::Float64 => {
            let array = column.as_any().downcast_ref::<Float64Array>()?;
            if array.is_null(row_index) {
                Ok(default_value.to_value())
            } else {
                Ok(Value::Double(array.value(row_index)))
            }
        }
        DataType::List(_) => {
            // Handle arrays: int[], long[], double[], float[]
            extract_array_property(column, row_index, default_value)
        }
        _ => Err(PropertyError::UnsupportedType(column.data_type().clone())),
    }
}
```

### 6. Updated process_node_batch

**Extend to handle properties**:

```rust
fn process_node_batch(
    batch_ref: &ArrowBatchReference,
    property_configs: &[PropertyConfig],
) -> Result<(Vec<i64>, Vec<Vec<NodeLabel>>, Vec<Vec<Value>>)> {
    let batch = batch_ref.batch();
    let row_count = batch_ref.row_count();

    // Existing ID + label extraction
    let ids = extract_id_column(batch, 0)?;
    let labels = extract_label_column(batch, 1)?;

    // NEW: Extract property columns
    let mut properties_by_node = Vec::with_capacity(row_count);
    for row_idx in 0..row_count {
        let mut props = Vec::with_capacity(property_configs.len());
        for config in property_configs {
            let value = extract_property_value(
                batch_ref,
                config.column_index,
                row_idx,
                &config.default_value,
            )?;
            props.push(value);
        }
        properties_by_node.push(props);
    }

    Ok((ids, labels, properties_by_node))
}
```

## Arrow Schema Expectations

### Node Table Schema

```
Column 0: id (Int64) - required
Column 1: labels (List<Utf8>) - required
Column 2+: property_1 (Int64|Float64|List<...>) - optional
Column 3+: property_2 (Int64|Float64|List<...>) - optional
...
```

### Edge Table Schema

```
Column 0: source (Int64) - required
Column 1: target (Int64) - required
Column 2: type (Utf8) - required
Column 3+: property_1 (Int64|Float64|List<...>) - optional
Column 4+: property_2 (Int64|Float64|List<...>) - optional
...
```

## Implementation Steps

### Step 1: Property Types & Config (1 hour)

- [ ] Create `PropertyConfig` struct
- [ ] Add `Aggregation` enum (defer aggregation logic to Phase 7)
- [ ] Add property config to `ArrowFactoryConfig`
- [ ] Update `ArrowFactoryConfig::builder()` API

### Step 2: PropertyAccumulator (1.5 hours)

- [ ] Implement `PropertyAccumulator` struct
- [ ] Implement `add()` method (HashMap insertion)
- [ ] Implement `build()` method (HashMap → dense PropertyValues)
- [ ] Handle type conversion (Arrow → Value → PropertyValues)
- [ ] Unit tests (5 tests)

### Step 3: Extend NodeAccumulator (1.5 hours)

- [ ] Add `property_accumulators` field
- [ ] Update `new()` constructor with property configs
- [ ] Update `add_node()` to accept properties
- [ ] Implement `build_properties()` method
- [ ] Update `process_node_batch()` to extract properties
- [ ] Unit tests (5 tests)

### Step 4: Extend EdgeAccumulator (1.5 hours)

- [ ] Add `property_accumulators` field (per relationship type)
- [ ] Update `new()` constructor with property configs
- [ ] Update `add_edge()` to accept properties
- [ ] Implement `build_properties()` method
- [ ] Update `process_edge_batch()` to extract properties
- [ ] Unit tests (5 tests)

### Step 5: Arrow Column Extraction (1.5 hours)

- [ ] Implement `extract_property_value()` for primitive types
- [ ] Implement `extract_array_property()` for array types
- [ ] Handle nulls with default values
- [ ] Error handling for type mismatches
- [ ] Unit tests (6 tests)

### Step 6: Integration & Testing (1 hour)

- [ ] Update `NodeImportTask` to use property-aware accumulator
- [ ] Update `EdgeImportTask` to use property-aware accumulator
- [ ] Create integration test with properties
- [ ] Update `ArrowNativeFactory::build()` to wire properties
- [ ] Integration tests (3 tests)

**Total Estimated Time**: ~8 hours

## Testing Strategy

### Unit Tests

**PropertyAccumulator**:

1. `test_property_accumulator_new` - Empty accumulator creation
2. `test_property_accumulator_add_long` - Add long property
3. `test_property_accumulator_add_double` - Add double property
4. `test_property_accumulator_add_array` - Add array property
5. `test_property_accumulator_build` - Build PropertyValues with IdMap

**NodeAccumulator Properties**:

1. `test_node_accumulator_with_properties` - Create with property configs
2. `test_node_accumulator_add_node_with_properties` - Add node with properties
3. `test_node_accumulator_build_properties` - Build property store
4. `test_node_accumulator_default_values` - Handle missing properties
5. `test_node_accumulator_null_values` - Handle null values

**EdgeAccumulator Properties**:

1. `test_edge_accumulator_with_properties` - Create with property configs
2. `test_edge_accumulator_add_edge_with_properties` - Add edge with properties
3. `test_edge_accumulator_build_properties` - Build property store
4. `test_edge_accumulator_default_values` - Handle missing properties
5. `test_edge_accumulator_property_per_type` - Different properties per rel type

**Arrow Column Extraction**:

1. `test_extract_long_property` - Extract Int64 column
2. `test_extract_double_property` - Extract Float64 column
3. `test_extract_long_array_property` - Extract List<Int64> column
4. `test_extract_double_array_property` - Extract List<Float64> column
5. `test_extract_null_with_default` - Null value → default value
6. `test_extract_unsupported_type` - Error on unsupported type

### Integration Tests

1. `test_import_nodes_with_properties` - End-to-end node property import
2. `test_import_edges_with_properties` - End-to-end edge property import
3. `test_property_defaults_and_nulls` - Default value handling

**Target**: +24 new tests (total: 92 tests)

## Known Limitations (GAMMA Strategy)

### What Phase 6 DOES Include:

✅ Property column extraction from Arrow  
✅ Type conversion (Arrow → PropertyValues)  
✅ Default value handling  
✅ Null value handling  
✅ Array property support (int[], long[], double[], float[])  
✅ Property storage in DefaultGraphStore

### What Phase 6 DOES NOT Include (defer to future):

⚠️ Aggregation logic (SUM, MIN, MAX) - Phase 7 or future  
⚠️ String properties - Arrow has strings, but need string PropertyValues impl  
⚠️ Nested/complex types - Just primitives and arrays for MVP  
⚠️ Property validation/schema enforcement - Trust Arrow schema  
⚠️ Incremental property builders - Use simple HashMap accumulation

## Success Criteria

- [ ] `PropertyAccumulator` implemented and tested (5 tests)
- [ ] `NodeAccumulator` extended for properties (5 tests)
- [ ] `EdgeAccumulator` extended for properties (5 tests)
- [ ] Arrow column extraction working (6 tests)
- [ ] Integration tests passing (3 tests)
- [ ] Total: 92+ tests passing (68 existing + 24 new)
- [ ] Properties accessible from DefaultGraphStore
- [ ] Documentation updated

## Next Steps After Phase 6

**Phase 7: Consumer System** (optional optimization)

- Buffered property writers
- Aggregation logic implementation
- Batch optimization

**Phase 8: Integration & Testing**

- End-to-end pipeline tests
- Performance benchmarks
- Example usage
- Complete documentation
