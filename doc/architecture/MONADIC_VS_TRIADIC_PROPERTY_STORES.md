# Monadic vs Triadic PropertyStores

**Status**: Experimental  
**Date**: October 2024  
**Context**: Collections First Architecture

## Executive Summary

This document compares two Collections First property store patterns:

1. **MonadicPropertyStore**: Universal single-level storage
2. **TriadicPropertyStore**: Three-level composed storage (Meta/Node/Link)

Both are **experimental proof-of-concepts** demonstrating how Collections can serve as the universal backend for property storage, independent of GraphStore complexity.

---

## MonadicPropertyStore: The Universal Building Block

### What It Is

A standalone property store that:
- Works directly with **Collections trait**
- Maintains a single flat key space
- Is independent of graph/node/relationship concepts
- Is simple, testable, and composable

### Structure

```rust
MonadicPropertyStore {
    properties: HashMap<String, MonadicProperty>
}

MonadicProperty {
    values: Arc<dyn PropertyValues>,  // Backed by Collections
    schema: PropertySchema
}
```

### Key Characteristics

| Aspect | Description |
|--------|-------------|
| **Key Space** | Flat - all keys in one namespace |
| **Backend** | Any Collections implementation (Vec, HugeArray, Arrow) |
| **Indexing** | Keys only (no element indexing concept) |
| **Use Cases** | Generic property storage for ANY domain |
| **Complexity** | Minimal - just HashMap + Collections |

### Example

```rust
let vec_long = VecLong::from(vec![1, 2, 3]);
let values = MonadicLongPropertyValues::new(vec_long, 0);
let property = MonadicProperty::of("age", Arc::new(values));

let store = MonadicPropertyStore::builder()
    .put("age", property)
    .build();

// Access
let age_values = store.get_property_values("age").unwrap();
```

### Strengths

✅ **Simple**: Single level, clear semantics  
✅ **Universal**: Works for any domain  
✅ **Testable**: Easy to verify behavior  
✅ **Composable**: Can be building block for complex stores  
✅ **Collections First**: Direct Collections integration

### Limitations

⚠️ **Single Level**: No built-in concept of multiple storage contexts  
⚠️ **Flat Keys**: Can't have "age" at multiple levels  
⚠️ **No Structure**: Doesn't encode graph semantics  

---

## TriadicPropertyStore: The Composition Pattern

### What It Is

A HyperPropertyStore that:
- Composes **THREE** MonadicPropertyStores
- Maintains **separate key spaces** for each level
- Allows **different backends** per level
- Encodes a universal Meta/Node/Link pattern

### Structure

```rust
TriadicPropertyStore {
    meta_properties: MonadicPropertyStore,   // Level 0
    node_properties: MonadicPropertyStore,   // Level 1
    link_properties: MonadicPropertyStore,   // Level 2
}
```

### Key Characteristics

| Aspect | Description |
|--------|-------------|
| **Key Spaces** | Three independent namespaces |
| **Backends** | Each level can use different Collections backend |
| **Indexing** | Meta (scalar), Node (indexed), Link (indexed²) |
| **Use Cases** | Systems with three-level structure (graphs, filesystems, etc.) |
| **Complexity** | Moderate - composition + delegation |

### Example

```rust
// Meta: Small Vec for graph metadata
let graph_id = MonadicProperty::of(
    "graph_id",
    Arc::new(MonadicLongPropertyValues::new(
        VecLong::from(vec![42]),
        0
    ))
);

// Node: Huge for billions of nodes
let age = MonadicProperty::of(
    "age",
    Arc::new(MonadicLongPropertyValues::new(
        HugeLongArray::new(1_000_000),
        0
    ))
);

// Link: Huge for trillions of edges
let weight = MonadicProperty::of(
    "weight",
    Arc::new(MonadicDoublePropertyValues::new(
        HugeDoubleArray::new(5_000_000),
        1.0
    ))
);

let store = TriadicPropertyStore::builder()
    .put_meta("graph_id", graph_id)
    .put_node("age", age)
    .put_link("weight", weight)
    .build();

// Access - separate namespaces!
let meta_id = store.get_meta_property_values("graph_id");
let node_ages = store.get_node_property_values("age");
let link_weights = store.get_link_property_values("weight");
```

### Strengths

✅ **Separate Key Spaces**: Can have "name" at all three levels  
✅ **Independent Backends**: Vec for meta, Huge for nodes/links  
✅ **Composition**: Each level is just a MonadicPropertyStore  
✅ **Universal Pattern**: Meta/Node/Link transcends graphs  
✅ **Optimization**: Different strategies per level  

### Limitations

⚠️ **More Complex**: Three stores to manage  
⚠️ **More Methods**: Separate access for each level  
⚠️ **Fixed Structure**: Always three levels (not configurable)  

---

## Comparison Matrix

| Feature | Monadic | Triadic |
|---------|---------|---------|
| **Key Spaces** | Single flat namespace | Three independent namespaces |
| **Backends** | One per store | Three (one per level) |
| **Complexity** | Low | Moderate |
| **Composability** | Building block | Composition of monads |
| **Graph Semantics** | None | Meta/Node/Link encoded |
| **Use Cases** | Generic storage | Three-level systems |
| **Test Complexity** | Simple | Moderate |
| **API Surface** | Small | Medium (3x methods) |
| **Collections First** | ✅ Yes | ✅ Yes (3 independent) |

---

## The Universal Three-Level Pattern

### Meta/Node/Link Transcends Graphs

This pattern appears across many domains:

| Domain | Level 0 (Meta) | Level 1 (Node) | Level 2 (Link) |
|--------|---------------|----------------|----------------|
| **Graphs** | Graph metadata | Node properties | Edge properties |
| **File Systems** | Volume metadata | File metadata | Directory links |
| **Databases** | DB metadata | Table metadata | Foreign keys |
| **Networks** | System config | Host properties | Connections |
| **ML Pipelines** | Model metadata | Sample features | Batch metadata |

### Why This Pattern Works

1. **Scalar vs Indexed**: Meta is typically scalar, Node/Link are indexed
2. **Different Scales**: Meta is small, Node is large, Link is huge
3. **Different Access**: Meta is global, Node is by-id, Link is by-id²
4. **Independent Backends**: Vec for meta, Huge for node/link

---

## Relationship to GraphStore

### Current State

The existing `GraphStore` implementation:
- Intermingles node, relationship, and graph property access
- Uses factory patterns and backend abstractions
- Has complex initialization and configuration
- Doesn't leverage Collections First

### How Triadic Could Help

#### Option 1: GraphStore Uses Triadic Internally

```rust
struct GraphStore {
    properties: TriadicPropertyStore,
    topology: CSRGraph,
    // ...
}

impl GraphStore {
    fn node_property(&self, key: &str, node_id: NodeId) {
        self.properties.get_node_property_values(key)
            // ... access by node_id
    }
}
```

#### Option 2: GraphStore Delegates to Three Monads

```rust
struct GraphStore {
    graph_properties: MonadicPropertyStore,
    node_properties: MonadicPropertyStore,
    rel_properties: MonadicPropertyStore,
    topology: CSRGraph,
}
```

#### Option 3: GraphStore as Smart Facade

```rust
// GraphStore remains as-is but internally uses TriadicPropertyStore
// Maintains backward compatibility while leveraging new architecture
```

---

## Trade-offs and Decisions

### When to Use Monadic

✅ **Use Monadic When:**
- You need simple, flat property storage
- Domain doesn't have inherent levels
- You're building a composable component
- You want minimal complexity
- You're experimenting with Collections First

### When to Use Triadic

✅ **Use Triadic When:**
- You have a natural three-level structure
- You need separate key spaces
- You want different backends per level
- You're modeling graphs, filesystems, or similar
- You want to exploit Meta/Node/Link optimizations

### When to Use Neither

Consider traditional approaches when:
- Collections First isn't needed
- You have domain-specific requirements
- Performance requires specialized implementations
- The pattern doesn't fit your use case

---

## Performance Considerations

### Monadic Performance

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Insert property | O(1) amortized | HashMap insert |
| Get property | O(1) | HashMap lookup |
| Get value by index | O(1) | Delegates to Collections |
| Iterate keys | O(n) | HashMap iteration |
| Memory overhead | Low | One HashMap + properties |

### Triadic Performance

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Insert property | O(1) amortized | Delegates to Monadic |
| Get property | O(1) | Level dispatch + HashMap |
| Get value by index | O(1) | Delegates to Collections |
| Iterate keys | O(n) per level | Three separate iterations |
| Memory overhead | Low | Three HashMaps + properties |

**Key Insight**: Triadic has **no significant overhead** because it's just composition + delegation!

---

## Migration Path

### From Legacy PropertyStore to Monadic

1. **Create Monadic wrapper** around existing implementations
2. **Test compatibility** with existing code
3. **Gradually migrate** to Collections backends
4. **Remove legacy code** once migration complete

### From Monadic to Triadic

1. **Identify levels** in your domain (what's meta vs node vs link?)
2. **Create separate Monadic stores** for each level
3. **Compose into Triadic** structure
4. **Update access patterns** to use level-specific methods
5. **Optimize backends** independently per level

### GraphStore Integration (Future)

1. **Phase 1**: Experiment with Triadic in parallel (CURRENT)
2. **Phase 2**: Implement Collections backends for all property types
3. **Phase 3**: Create GraphStore adapter using Triadic internally
4. **Phase 4**: Migrate existing graphs to new system
5. **Phase 5**: Deprecate legacy property system

---

## Open Questions

### For Monadic

1. Should Monadic support property mutations, or be immutable?
2. How to handle property schema validation?
3. Should we support property versioning?
4. How to handle null/missing values?

### For Triadic

1. Should levels be configurable (more or fewer than 3)?
2. Should we support custom level names?
3. How to handle cross-level queries?
4. Should Triadic support transactions across levels?

### For GraphStore

1. Can we maintain backward compatibility?
2. How to migrate existing graphs?
3. What's the performance impact?
4. Should we expose Triadic API directly or hide it?

---

## Future Work

### Short Term (Experimental)

- ✅ Implement Monadic PropertyStore
- ✅ Implement Triadic PropertyStore
- ✅ Create examples and documentation
- ⏳ Implement more Collections backends
- ⏳ Test with real graph workloads

### Medium Term (Proof of Concept)

- ⏳ Integrate Triadic with GraphStore
- ⏳ Benchmark performance vs legacy
- ⏳ Migrate sample graphs to new system
- ⏳ Document migration patterns

### Long Term (Production)

- ⏳ Full Collections backend support (Vec, Huge, Arrow)
- ⏳ Complete GraphStore migration
- ⏳ Deprecate legacy property system
- ⏳ Production deployment

---

## Conclusion

### Key Takeaways

1. **Monadic** = Universal single-level storage building block
2. **Triadic** = Composition of three Monadic stores
3. **Collections First** works for both patterns
4. **Meta/Node/Link** is a universal pattern beyond graphs
5. **Composition** beats inheritance for property stores

### Recommendations

**For Experimentation:**
- Start with Monadic to understand Collections First
- Evolve to Triadic when you need multiple levels
- Keep both patterns experimental until proven

**For GraphStore:**
- Don't rush integration
- Learn from experiments first
- Plan migration carefully
- Maintain backward compatibility

**For Future Systems:**
- Consider Collections First from the start
- Think about Meta/Node/Link patterns
- Prefer composition over inheritance
- Keep it simple until complexity is needed

---

## References

- `gds/src/types/properties/monadic/` - Monadic implementation
- `gds/src/types/properties/triadic/` - Triadic implementation
- `gds/examples/monadic_property_store_demo.rs` - Monadic example
- `gds/examples/triadic_property_store_demo.rs` - Triadic example
- `doc/architecture/UNIFIED_COLLECTIONS_ARCHITECTURE.md` - Collections First architecture
- `doc/adr/adr0008_collections_universal_backend.md` - Collections as Universal Backend ADR

