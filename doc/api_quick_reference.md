# Pure GraphStore API Quick Reference

**Version**: 0.1.0  
**Date**: October 4, 2025

Quick reference for the most common Pure GraphStore operations.

---

## 🚀 Quick Start

### Create a GraphStore

```rust
use rust_gds::types::graph_store::{
    DefaultGraphStore, GraphName, DatabaseId,
    DatabaseInfo, DatabaseLocation, Capabilities
};
use rust_gds::types::graph::{SimpleIdMap, RelationshipTopology};
use rust_gds::types::schema::GraphSchema;
use rust_gds::projection::RelationshipType;

let store = DefaultGraphStore::new(
    GraphName::new("my_graph"),
    DatabaseInfo::new(
        DatabaseId::new("db"),
        DatabaseLocation::local()
    ),
    GraphSchema::empty(),
    Capabilities::default(),
    SimpleIdMap::from_original_ids([0, 1, 2, 3]),
    HashMap::from([
        (RelationshipType::of("KNOWS"),
         RelationshipTopology::new(vec![vec![1], vec![2], vec![], vec![]], None))
    ])
);
```

---

## 📊 Common GraphStore Operations

### Query Counts

```rust
// Node counts
let total_nodes = store.node_count();
let person_count = store.node_count_for_label(&NodeLabel::of("Person"));

// Relationship counts
let total_rels = store.relationship_count();
let knows_count = store.relationship_count_for_type(&RelationshipType::of("KNOWS"));
```

### Check Existence

```rust
// Labels and types
if store.has_node_label(&NodeLabel::of("Person")) { /* ... */ }
if store.has_relationship_type(&RelationshipType::of("KNOWS")) { /* ... */ }

// Properties
if store.has_node_property("age") { /* ... */ }
if store.has_relationship_property(&RelationshipType::of("KNOWS"), "weight") { /* ... */ }
```

### List Keys

```rust
// Get all available keys
let node_labels: HashSet<NodeLabel> = store.node_labels();
let rel_types: HashSet<RelationshipType> = store.relationship_types();
let node_props: HashSet<String> = store.node_property_keys();
let rel_props: HashSet<String> = store.relationship_property_keys();

// Get keys for specific label/type
let person_props = store.node_property_keys_for_label(&NodeLabel::of("Person"));
let knows_props = store.relationship_property_keys_for_type(&RelationshipType::of("KNOWS"));
```

---

## 🏗️ Property Management

### Add Node Property

```rust
use rust_gds::types::properties::node::DefaultLongNodePropertyValues;

let values = Arc::new(DefaultLongNodePropertyValues::new(
    vec![25, 30, 35, 40],  // property values
    4                       // node count
));

store.add_node_property(
    HashSet::from([NodeLabel::of("Person")]),
    "age",
    values
)?;
```

### Add Relationship Property

```rust
use rust_gds::types::properties::relationship::DefaultRelationshipPropertyValues;

let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
    vec![1.0, 2.5, 3.7],    // property values
    3                        // relationship count
));

store.add_relationship_property(
    RelationshipType::of("KNOWS"),
    "weight",
    values
)?;
```

### Remove Properties

```rust
// Remove node property
store.remove_node_property("age")?;

// Remove relationship property
store.remove_relationship_property(
    &RelationshipType::of("KNOWS"),
    "weight"
)?;
```

### Access Property Values

```rust
// Get node property values
let age_values = store.node_property_values("age")?;
if let Some(age) = age_values.long_value(0) {
    println!("Node 0 age: {}", age);
}

// Get relationship property values
let weight_values = store.relationship_property_values(
    &RelationshipType::of("KNOWS"),
    "weight"
)?;
if let Some(weight) = weight_values.double_value(0) {
    println!("Relationship 0 weight: {}", weight);
}
```

---

## 🔍 Graph Views & Queries

### Create Graph View

```rust
let graph = store.graph();

// Basic info
let node_count = graph.node_count();
let rel_count = graph.relationship_count();
let is_multi = graph.is_multi_graph();
```

### Traverse Relationships

```rust
// Iterate outgoing relationships from node 0
for cursor in graph.stream_relationships(0, 0.0) {
    println!("Source: {}, Target: {}, Property: {}",
        cursor.source_id(),
        cursor.target_id(),
        cursor.property()
    );
}

// Iterate incoming relationships (if inverse-indexed)
for cursor in graph.stream_inverse_relationships(2, 0.0) {
    println!("Source: {}, Target: {}",
        cursor.source_id(),
        cursor.target_id()
    );
}
```

### Check Degree

```rust
use rust_gds::types::graph::degrees::Degrees;

let degree = graph.degree(0);  // outgoing degree
if let Some(in_degree) = graph.degree_inverse(0) {
    println!("In-degree: {}", in_degree);
}
let unique_degree = graph.degree_without_parallel_relationships(0);
```

### Check Relationship Existence

```rust
use rust_gds::types::properties::relationship::RelationshipPredicate;

if graph.exists(0, 1) {
    println!("Relationship exists from 0 to 1");
}
```

### Get Specific Target

```rust
// Get the nth target from node 0
if let Some(target) = graph.nth_target(0, 0) {
    println!("First target: {}", target);
}
```

### Access Node Properties via Graph

```rust
use rust_gds::types::properties::node::NodePropertyContainer;

if let Some(age_values) = graph.node_properties("age") {
    for node_id in 0..graph.node_count() {
        if let Some(age) = age_values.long_value(node_id as u64) {
            println!("Node {} age: {}", node_id, age);
        }
    }
}
```

### Filter by Relationship Type

```rust
let filtered_graph = graph.relationship_type_filtered_graph(
    &HashSet::from([
        RelationshipType::of("KNOWS"),
        RelationshipType::of("LIKES")
    ])
)?;

println!("Filtered relationship count: {}", filtered_graph.relationship_count());
```

---

## 🗺️ ID Mapping

### Convert IDs

```rust
use rust_gds::types::graph::id_map::{IdMap, MappedNodeId, OriginalNodeId};

// Original → Mapped
if let Some(mapped) = graph.to_mapped_node_id(42) {
    println!("Original 42 maps to {}", mapped);
}

// Mapped → Original
if let Some(original) = graph.to_original_node_id(0) {
    println!("Mapped 0 is original {}", original);
}
```

### Query Node Labels

```rust
// Get labels for a specific node
let labels = graph.node_labels(0);
for label in labels {
    println!("Node 0 has label: {}", label.name());
}

// Check if node has specific label
if graph.has_label(0, &NodeLabel::of("Person")) {
    println!("Node 0 is a Person");
}

// Get all available labels
let all_labels = graph.available_node_labels();
```

### Iterate Nodes

```rust
use rust_gds::types::graph::id_map::NodeIterator;

// Iterate all nodes
for node_id in graph.iter() {
    println!("Node: {}", node_id);
}

// Iterate nodes with specific labels
let person_label = NodeLabel::of("Person");
for node_id in graph.iter_with_labels(&HashSet::from([person_label])) {
    println!("Person node: {}", node_id);
}
```

---

## 🎨 Advanced Patterns

### Batch Node Iteration

```rust
use rust_gds::types::graph::id_map::BatchNodeIterable;

let batches = graph.batch_iterables(1000);  // batch size
for batch in batches {
    for node_id in batch.iter() {
        // Process node
    }
}
```

### Concurrent Graph Access

```rust
use std::sync::Arc;
use std::thread;

let graph = store.graph();
let handles: Vec<_> = (0..4).map(|thread_id| {
    let graph_copy = Arc::clone(&graph);
    thread::spawn(move || {
        // Each thread has its own graph view
        for node in graph_copy.iter().take(100) {
            let degree = graph_copy.degree(node);
            // Process...
        }
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

### Property with Custom Default

```rust
use rust_gds::types::properties::relationship::RelationshipProperties;

let source = 0;
let target = 1;
let fallback = 1.0;

let weight = graph.relationship_property(source, target, fallback);
println!("Weight (or default): {}", weight);
```

### Get Characteristics

```rust
let characteristics = graph.characteristics();

if characteristics.is_directed() {
    println!("Graph is directed");
}
if characteristics.is_undirected() {
    println!("Graph is undirected");
}
if characteristics.is_inverse_indexed() {
    println!("Graph has inverse index");
}
```

---

## 🔧 Builder Pattern for Property Stores

### Relationship Property Store

```rust
use rust_gds::types::properties::relationship::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder
};

// Build from scratch
let store = DefaultRelationshipPropertyStore::builder()
    .put("weight", property1)
    .put("confidence", property2)
    .build();

// Modify existing
let updated = store.to_builder()
    .remove_property("weight")
    .put("score", property3)
    .build();
```

### Node Property Store

```rust
use rust_gds::types::properties::node::{
    NodePropertyStore, NodePropertyStoreBuilder
};

let store = DefaultNodePropertyStore::builder()
    .put("age", age_property)
    .put("name", name_property)
    .build();
```

---

## ⚠️ Error Handling

```rust
use rust_gds::types::graph_store::{GraphStoreError, GraphStoreResult};

match store.node_property_values("age") {
    Ok(values) => {
        // Use values
    }
    Err(GraphStoreError::PropertyNotFound(key)) => {
        eprintln!("Property not found: {}", key);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

---

## 📝 Type Reference

### Common Types

```rust
use rust_gds::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use rust_gds::projection::{NodeLabel, RelationshipType};
use rust_gds::types::properties::relationship::PropertyValue;

type MappedNodeId = u64;        // Internal compact node ID
type OriginalNodeId = i64;      // External/database node ID
type PropertyValue = f64;       // Current property value type (will expand)
const NOT_FOUND: i64 = -1;      // Sentinel for missing mappings
```

### Result Types

```rust
use rust_gds::types::graph::{GraphResult};
use rust_gds::types::graph_store::{GraphStoreResult, GraphStoreError};

type GraphResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
type GraphStoreResult<T> = Result<T, GraphStoreError>;
```

---

## 🎯 Common Pitfalls

### ❌ DON'T: Try to modify Graph directly

```rust
// Graph is immutable!
// This won't compile:
// graph.add_node_property(...);
```

### ✅ DO: Modify via GraphStore

```rust
store.add_node_property(...)?;
let graph = store.graph();  // Get fresh view
```

### ❌ DON'T: Mutate property stores directly

```rust
// Stores are immutable!
// store.put_property_entry(...);  // Doesn't exist
```

### ✅ DO: Use builder pattern

```rust
store.add_relationship_property(...)?;  // GraphStore handles builder internally
```

### ❌ DON'T: Assume properties exist

```rust
// May panic if property doesn't exist:
// let values = store.node_property_values("age").unwrap();
```

### ✅ DO: Handle errors properly

```rust
match store.node_property_values("age") {
    Ok(values) => { /* use values */ }
    Err(e) => { /* handle error */ }
}
```

---

## 📚 See Also

- [Full API Contract](./api_contract_pure_graphstore.md)
- [ADR 0001: Property Graph Store Design](./adr0001_property_graph_store_design.md)
- [ADR 0002: Triadic GraphStore Architecture](./adr0002_triadic_graphstore_architecture.md)

---

**Last Updated**: October 4, 2025
