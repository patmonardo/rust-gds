# ADR0008: Collections as Universal Backend Architecture

## Status
Proposed

## Context

The current PropertyStore system has several architectural issues:

- **GraphStore** stores properties for nodes, relationships, and graph as `HashMap<String, Arc<dyn PropertyValues>>`
- **PropertyValues** trait abstracts property storage but has multiple implementations
- **Multiple implementations**: `Default*PropertyValues` (Vec-based), `Huge*PropertyValues` (HugeArray-based)
- **No unified backend abstraction**: Each PropertyValues type implements storage differently
- **Redundant code**: Similar logic duplicated across property types
- **Hard to extend**: Adding new backends (Arrow, GPU) requires implementing each PropertyValues type

### Current Architecture
```
GraphStore
  ├─ graph_properties: HashMap<String, Arc<dyn GraphPropertyValues>>
  ├─ node_properties: HashMap<String, Arc<dyn NodePropertyValues>>
  └─ relationship_property_stores: HashMap<RelType, RelationshipPropertyStore>
       └─ properties: HashMap<String, Arc<dyn RelationshipPropertyValues>>

PropertyValues (Trait)
  ├─ NodePropertyValues
  │   ├─ DefaultLongNodePropertyValues (uses Vec<i64>)
  │   └─ HugeLongNodePropertyValues (uses HugeLongArray)
  ├─ RelationshipPropertyValues
  │   └─ DefaultRelationshipPropertyValues (uses Vec<f64>)
  └─ GraphPropertyValues
      └─ DefaultLongGraphPropertyValues (uses Vec<i64>)
```

## Decision

**Collections First Architecture**: All PropertyValues implementations use `Collections<T>` as their universal backend.

### Architecture Layers

**Level 0: Collections (Algorithmic Kernel)**
- Vec, Huge, Arrow backends
- Unified API: `get(i)`, `set(i, v)`, `sum()`, `mean()`, etc.
- Extensions: Paging, Compression, Memory Estimation, etc.

**Level 1: PropertyStore (using Collections)**
- `UniversalPropertyValues<T, C: Collections<T>>` - UniversalAdapter
- `PropertyValues` trait (API contract)
- Factory creates `Collections` → wraps in `UniversalAdapter`

**Level 2: GraphStore (Super Extension)**
- Stores `Arc<dyn PropertyValues>`
- PropertyValues are Collections-backed
- GraphStore = Collections + Graph Semantics

### Proposed Architecture
```
Level 2: GraphStore (Super Extension)
  └─ PropertyStore (collections of PropertyValues)
      └─ PropertyValues (wrapper trait)
          └─ UniversalPropertyValues (UniversalAdapter)
              └─ Collections<T> (Level 0 Backend)
                  ├─ VecInt, VecLong, VecDouble
                  ├─ HugeIntArray, HugeLongArray, HugeDoubleArray
                  └─ ArrowIntArray, ArrowLongArray, ArrowDoubleArray
```

### Data Flow
```
GraphStore
  → stores HashMap<String, Arc<dyn NodePropertyValues>>
  → NodePropertyValues = UniversalPropertyValues<i64, VecLong>
  → VecLong: Collections<i64>
  → Collections API: get(i), set(i, v), sum(), mean()
```

### Implementation Pattern
```rust
// Factory creates Collections backend
let collection = match cfg.backend.primary {
    CollectionsBackend::Vec => VecLong::from(values),
    CollectionsBackend::Huge => HugeLongArray::from(values),
    CollectionsBackend::Arrow => ArrowLongArray::from(values),
};

// Wrap in UniversalAdapter
let universal = UniversalPropertyValues::new(collection, ValueType::Long, 0);

// Wrap in PropertyValues trait
Arc::new(UniversalLongNodePropertyValues::new(universal, node_count))
```

## Consequences

### Positive
- **Single backend abstraction**: All PropertyValues use Collections
- **Code reuse**: No duplication across property types
- **Easy to extend**: Add new backends by implementing Collections<T>
- **Uniform API**: All property operations use same Collections interface
- **Performance**: Collections Extensions (Paging, Compression) available to all properties
- **Type safety**: Generic Collections<T> with compile-time type checking

### Negative
- **Breaking change**: Code using `Default*PropertyValues` directly will break
- **Migration effort**: All PropertyValues implementations need refactoring
- **Complexity**: Additional abstraction layer (UniversalAdapter)
- **Learning curve**: Developers need to understand Collections API

### Migration Strategy
1. **Phase 1**: Create UniversalPropertyValues implementations alongside existing ones
2. **Phase 2**: Update factory to use Collections + UniversalAdapter
3. **Phase 3**: Deprecate legacy implementations with migration guide
4. **Phase 4**: Remove legacy implementations in next major version

## Implementation

### Files to Create
- `gds/src/types/properties/node/impls/universal_node_property_values.rs`
- `gds/src/types/properties/relationship/impls/universal_relationship_property_values.rs`
- `gds/src/types/properties/graph/impls/universal_graph_property_values.rs`

### Files to Modify
- `gds/src/types/properties/factory.rs` - Use Collections + UniversalAdapter
- `gds/src/config/property_store_config.rs` - Remove, use CollectionsConfig
- GraphStore implementations - Use CollectionsConfig

### Success Criteria
- All PropertyValues implementations use UniversalAdapter
- Factory uses Collections + UniversalAdapter pattern
- GraphStore uses CollectionsConfig
- Clear migration path documented
- All tests pass

## References
- ADR0001: Property Graph Store Design
- ADR0003: Node Property Value Contract
- Collections Platform Architecture Document
