# Pure GraphStore Integration Test Plan

**Version**: 0.1.0  
**Date**: October 4, 2025  
**Status**: Planning

## Overview

This document outlines the integration tests needed to validate the Pure GraphStore system as a complete, cohesive unit. Unlike unit tests that verify individual components, these tests validate **end-to-end workflows** and **cross-component interactions**.

---

## 🎯 Test Goals

1. **API Contract Validation**: Ensure all public APIs work as documented
2. **Workflow Coverage**: Test common user scenarios end-to-end
3. **Edge Case Handling**: Validate behavior at boundaries
4. **Error Path Coverage**: Verify error handling is correct
5. **Performance Baseline**: Establish performance expectations

---

## ✅ Current Test Coverage

### Unit Tests (Existing)

**Graph Module**:

- ✅ `default_graph::tests::computes_degrees_and_relationship_counts`
- ✅ `default_graph::tests::filters_relationship_types`
- ✅ `characteristics::tests::*`
- ✅ `topology::tests::*`

**GraphStore Module**:

- ✅ `default_graph_store::tests::graph_view_reflects_store_data`
- ✅ `default_graph_store::tests::manages_relationship_properties`

**Property System**:

- ✅ Node property values (long, double, arrays)
- ✅ Relationship cursors and iteration
- ✅ Property stores (builder pattern)
- ✅ Property values trait implementations

**Schema System**:

- ✅ Schema creation and union
- ✅ Default values
- ✅ Property schema

### Coverage Gaps (Need Integration Tests)

1. ❌ Full workflow: create store → add properties → query → modify → query again
2. ❌ Multi-type graphs with filtering
3. ❌ Complex property queries across multiple types
4. ❌ Large graph traversal patterns
5. ❌ Concurrent access patterns
6. ❌ Error recovery scenarios
7. ❌ Property lifecycle (add → access → remove → verify gone)

---

## 📋 Integration Test Suites

### Suite 1: Core Workflow Tests

**Purpose**: Validate common end-to-end workflows.

#### Test 1.1: Complete GraphStore Lifecycle

```rust
#[test]
fn test_complete_graphstore_lifecycle() {
    // 1. Create empty store
    let mut store = create_test_store();

    // 2. Add node labels
    store.add_node_label(NodeLabel::of("Person")).unwrap();
    store.add_node_label(NodeLabel::of("Company")).unwrap();

    // 3. Add node properties
    let age_values = create_age_property();
    store.add_node_property(
        HashSet::from([NodeLabel::of("Person")]),
        "age",
        age_values
    ).unwrap();

    // 4. Add relationship properties
    let weight_values = create_weight_property();
    store.add_relationship_property(
        RelationshipType::of("KNOWS"),
        "weight",
        weight_values
    ).unwrap();

    // 5. Query via graph view
    let graph = store.graph();
    assert_eq!(graph.node_count(), EXPECTED_NODES);
    assert!(graph.has_relationship_property());

    // 6. Verify property access
    let age = graph.node_properties("age").unwrap();
    assert_eq!(age.long_value(0), Some(25));

    // 7. Modify properties
    store.remove_node_property("age").unwrap();

    // 8. Verify removal
    let graph2 = store.graph();
    assert!(graph2.node_properties("age").is_none());
}
```

#### Test 1.2: Property Addition and Retrieval

```rust
#[test]
fn test_add_and_retrieve_all_property_types() {
    let mut store = create_test_store();

    // Add different property types
    store.add_node_property(labels, "age", long_values).unwrap();
    store.add_node_property(labels, "score", double_values).unwrap();
    store.add_node_property(labels, "tags", array_values).unwrap();

    let graph = store.graph();

    // Verify all accessible
    assert!(graph.node_properties("age").is_some());
    assert!(graph.node_properties("score").is_some());
    assert!(graph.node_properties("tags").is_some());

    // Verify correct values
    let age = graph.node_properties("age").unwrap();
    assert_eq!(age.long_value(0), Some(25));

    let score = graph.node_properties("score").unwrap();
    assert_eq!(score.double_value(0), Some(0.95));
}
```

#### Test 1.3: Graph View Consistency

```rust
#[test]
fn test_graph_view_reflects_store_changes() {
    let mut store = create_test_store();

    let graph1 = store.graph();
    let count1 = graph1.relationship_count();

    // Modify store
    store.add_relationship_property(...).unwrap();

    // Old view unchanged
    assert_eq!(graph1.relationship_count(), count1);
    assert!(!graph1.has_relationship_property());

    // New view reflects change
    let graph2 = store.graph();
    assert!(graph2.has_relationship_property());
}
```

---

### Suite 2: Multi-Type Graph Tests

**Purpose**: Test graphs with multiple relationship types and node labels.

#### Test 2.1: Multi-Relationship Graph

```rust
#[test]
fn test_multiple_relationship_types() {
    let store = create_multi_type_graph();
    // Graph with: KNOWS, LIKES, FOLLOWS

    let graph = store.graph();

    // Verify all types present
    let types = store.relationship_types();
    assert_eq!(types.len(), 3);
    assert!(types.contains(&RelationshipType::of("KNOWS")));

    // Verify counts
    assert_eq!(
        store.relationship_count_for_type(&RelationshipType::of("KNOWS")),
        5
    );

    // Filter to single type
    let filtered = graph.relationship_type_filtered_graph(
        &HashSet::from([RelationshipType::of("KNOWS")])
    ).unwrap();

    assert_eq!(filtered.relationship_count(), 5);
}
```

#### Test 2.2: Multi-Label Node Properties

```rust
#[test]
fn test_properties_across_multiple_labels() {
    let mut store = create_test_store();

    // Age property for Person and Employee
    store.add_node_property(
        HashSet::from([
            NodeLabel::of("Person"),
            NodeLabel::of("Employee")
        ]),
        "age",
        age_values
    ).unwrap();

    // Salary only for Employee
    store.add_node_property(
        HashSet::from([NodeLabel::of("Employee")]),
        "salary",
        salary_values
    ).unwrap();

    // Verify property associations
    let person_props = store.node_property_keys_for_label(
        &NodeLabel::of("Person")
    );
    assert!(person_props.contains("age"));
    assert!(!person_props.contains("salary"));

    let employee_props = store.node_property_keys_for_label(
        &NodeLabel::of("Employee")
    );
    assert!(employee_props.contains("age"));
    assert!(employee_props.contains("salary"));
}
```

---

### Suite 3: Traversal and Query Tests

**Purpose**: Test graph traversal patterns and queries.

#### Test 3.1: Full Graph Traversal

```rust
#[test]
fn test_traverse_entire_graph() {
    let graph = create_test_graph();

    let mut visited_nodes = HashSet::new();
    let mut edge_count = 0;

    for node_id in graph.iter() {
        visited_nodes.insert(node_id);

        for cursor in graph.stream_relationships(node_id, 0.0) {
            edge_count += 1;
            assert_eq!(cursor.source_id(), node_id);
        }
    }

    assert_eq!(visited_nodes.len(), graph.node_count());
    assert_eq!(edge_count, graph.relationship_count());
}
```

#### Test 3.2: Bidirectional Traversal

```rust
#[test]
fn test_forward_and_inverse_traversal() {
    let graph = create_inverse_indexed_graph();

    // Forward: node 0 → targets
    let mut forward_targets = HashSet::new();
    for cursor in graph.stream_relationships(0, 0.0) {
        forward_targets.insert(cursor.target_id());
    }

    // Inverse: find sources → node 0
    let mut inverse_sources = HashSet::new();
    for cursor in graph.stream_inverse_relationships(0, 0.0) {
        inverse_sources.insert(cursor.source_id());
    }

    // Verify consistency
    assert!(!forward_targets.is_empty());
    assert!(!inverse_sources.is_empty());

    // Each forward edge should have inverse
    for target in forward_targets {
        let mut found = false;
        for cursor in graph.stream_inverse_relationships(target, 0.0) {
            if cursor.source_id() == 0 {
                found = true;
                break;
            }
        }
        assert!(found, "Missing inverse edge");
    }
}
```

#### Test 3.3: Property-Weighted Traversal

```rust
#[test]
fn test_relationship_property_during_traversal() {
    let mut store = create_test_store();

    // Add weight property
    store.add_relationship_property(
        RelationshipType::of("KNOWS"),
        "weight",
        weight_values
    ).unwrap();

    let graph = store.graph();

    // Traverse and collect weights
    let mut weights = Vec::new();
    for cursor in graph.stream_relationships(0, 1.0) {
        weights.push(cursor.property());
    }

    // Verify weights were retrieved
    assert!(!weights.is_empty());
    assert!(weights.iter().any(|&w| w != 1.0));  // Not all fallback
}
```

---

### Suite 4: Edge Cases and Boundaries

**Purpose**: Test behavior at edge cases and boundaries.

#### Test 4.1: Empty Graph

```rust
#[test]
fn test_empty_graph_behavior() {
    let store = create_empty_graph_store();
    let graph = store.graph();

    assert!(graph.is_empty());
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.relationship_count(), 0);

    // Traversal should be safe
    for _ in graph.iter() {
        panic!("Should not iterate");
    }
}
```

#### Test 4.2: Single Node Graph

```rust
#[test]
fn test_single_node_no_edges() {
    let store = create_single_node_graph();
    let graph = store.graph();

    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.relationship_count(), 0);
    assert_eq!(graph.degree(0), 0);

    // Properties should work
    let graph = store.graph();
    if let Some(props) = graph.node_properties("label") {
        assert!(props.value_count() >= 1);
    }
}
```

#### Test 4.3: Self-Loop Graph

```rust
#[test]
fn test_self_loops() {
    let graph = create_self_loop_graph();
    // Node 0 → Node 0

    assert!(graph.exists(0, 0));
    assert_eq!(graph.degree(0), 1);

    let mut self_loops = 0;
    for cursor in graph.stream_relationships(0, 0.0) {
        if cursor.target_id() == 0 {
            self_loops += 1;
        }
    }
    assert_eq!(self_loops, 1);
}
```

#### Test 4.4: Disconnected Components

```rust
#[test]
fn test_disconnected_graph_components() {
    let graph = create_disconnected_graph();
    // Component 1: nodes 0, 1, 2
    // Component 2: nodes 3, 4
    // No edges between components

    // Component 1 should be traversable
    let component1_reachable = bfs_from_node(&graph, 0);
    assert!(component1_reachable.contains(&0));
    assert!(component1_reachable.contains(&1));
    assert!(!component1_reachable.contains(&3));  // Different component
}
```

---

### Suite 5: Error Handling Tests

**Purpose**: Verify proper error handling and recovery.

#### Test 5.1: Property Not Found

```rust
#[test]
fn test_access_nonexistent_property() {
    let store = create_test_store();

    match store.node_property_values("nonexistent") {
        Err(GraphStoreError::PropertyNotFound(key)) => {
            assert_eq!(key, "nonexistent");
        }
        _ => panic!("Expected PropertyNotFound error"),
    }
}
```

#### Test 5.2: Remove Nonexistent Property

```rust
#[test]
fn test_remove_nonexistent_property() {
    let mut store = create_test_store();

    let result = store.remove_node_property("nonexistent");
    assert!(result.is_err());

    match result {
        Err(GraphStoreError::PropertyNotFound(_)) => { /* expected */ }
        _ => panic!("Wrong error type"),
    }
}
```

#### Test 5.3: Double Remove

```rust
#[test]
fn test_double_remove_property() {
    let mut store = create_test_store();

    // Add property
    store.add_node_property(labels, "age", values).unwrap();

    // First remove succeeds
    assert!(store.remove_node_property("age").is_ok());

    // Second remove fails
    assert!(store.remove_node_property("age").is_err());
}
```

---

### Suite 6: Concurrent Access Tests

**Purpose**: Verify thread safety and concurrent access patterns.

#### Test 6.1: Concurrent Reads

```rust
#[test]
fn test_concurrent_graph_reads() {
    let store = create_large_test_graph();
    let graph = store.graph();

    let handles: Vec<_> = (0..4).map(|thread_id| {
        let graph_clone = Arc::clone(&graph);
        thread::spawn(move || {
            let mut local_count = 0;
            for node in graph_clone.iter().skip(thread_id * 100).take(100) {
                local_count += graph_clone.degree(node);
            }
            local_count
        })
    }).collect();

    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    assert_eq!(results.len(), 4);
    assert!(results.iter().all(|&count| count > 0));
}
```

#### Test 6.2: Read While Write (Store vs Graph)

```rust
#[test]
fn test_read_graph_while_modifying_store() {
    let mut store = create_test_store();
    let graph = store.graph();

    let initial_count = graph.relationship_count();

    // Spawn reader thread with old graph view
    let reader_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        graph.relationship_count()  // Should still see old view
    });

    // Modify store
    store.delete_relationships(&RelationshipType::of("KNOWS")).unwrap();

    // Reader should see original data
    let reader_count = reader_handle.join().unwrap();
    assert_eq!(reader_count, initial_count);

    // New view sees change
    assert_ne!(store.graph().relationship_count(), initial_count);
}
```

---

### Suite 7: Performance Baseline Tests

**Purpose**: Establish performance expectations for future comparisons.

#### Test 7.1: Large Graph Creation

```rust
#[test]
#[ignore]  // Run explicitly for performance testing
fn perf_create_large_graph() {
    let start = Instant::now();

    let store = create_graph_with_n_nodes(100_000);

    let elapsed = start.elapsed();
    println!("Created 100k node graph in {:?}", elapsed);

    assert!(elapsed < Duration::from_secs(5));
}
```

#### Test 7.2: Full Graph Traversal Performance

```rust
#[test]
#[ignore]
fn perf_traverse_large_graph() {
    let graph = create_graph_with_n_nodes(100_000);

    let start = Instant::now();

    let mut total_degree = 0;
    for node in graph.iter() {
        total_degree += graph.degree(node);
    }

    let elapsed = start.elapsed();
    println!("Traversed 100k nodes in {:?}", elapsed);
    println!("Total degree: {}", total_degree);

    assert!(elapsed < Duration::from_secs(1));
}
```

---

## 🏗️ Test Infrastructure

### Helper Functions Needed

```rust
// Create test fixtures
fn create_test_store() -> DefaultGraphStore { /* ... */ }
fn create_multi_type_graph() -> DefaultGraphStore { /* ... */ }
fn create_inverse_indexed_graph() -> DefaultGraph { /* ... */ }
fn create_empty_graph_store() -> DefaultGraphStore { /* ... */ }

// Create test data
fn create_age_property() -> Arc<dyn NodePropertyValues> { /* ... */ }
fn create_weight_property() -> Arc<dyn RelationshipPropertyValues> { /* ... */ }

// Utility functions
fn bfs_from_node(graph: &dyn Graph, start: MappedNodeId) -> HashSet<MappedNodeId> { /* ... */ }
```

---

## 📊 Success Criteria

Integration tests are successful when:

1. ✅ All documented API patterns work as specified
2. ✅ Error handling is predictable and documented
3. ✅ Concurrent access is safe and produces expected results
4. ✅ Edge cases don't panic or produce undefined behavior
5. ✅ Performance baselines are established
6. ✅ Tests are maintainable and well-documented

---

## 🚀 Implementation Plan

### Phase 1: Core Workflows (Do First)

- Suite 1: Core workflow tests
- Suite 4: Edge cases (empty, single node, etc.)

### Phase 2: Multi-Type Support

- Suite 2: Multi-type graph tests

### Phase 3: Advanced Features

- Suite 3: Traversal tests
- Suite 5: Error handling

### Phase 4: Production Ready

- Suite 6: Concurrent access
- Suite 7: Performance baselines

---

## 📝 Notes

- Integration tests go in `tests/integration/` directory
- Use `#[ignore]` for long-running performance tests
- Keep each test focused on one scenario
- Use descriptive test names that explain what's being tested
- Add documentation comments explaining the test scenario

---

**Status**: Ready to implement after code check-in  
**Next Step**: Implement Phase 1 tests
