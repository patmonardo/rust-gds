# The Two Extremes: Storage (Factory) and Computation (Eval)

**Date**: October 15, 2025  
**Insight**: Factory and Eval are the two fundamental extremes, targets of codegen  
**Principle**: Macros control BOTH storage and computation

---

## The Fundamental Duality

### Storage vs Computation (The Two Extremes)

**In computer science, everything reduces to**:

1. **Storage** - Where data lives (memory, disk, structure)
2. **Computation** - What you do with data (algorithms, transformations)

**In rust-gds projection/**:

1. **Factory** (Storage extreme) - How graphs are STORED
2. **Eval** (Computation extreme) - How graphs are COMPUTED ON

**Codegen sits in the middle** - generating code that controls BOTH!

---

## The Architecture

```
                    ┌──────────────────┐
                    │   projection/    │
                    │  GDSL Runtime    │
                    │                  │
    Storage ◄───────┤   codegen/       │───────► Computation
    Extreme         │  (Meta Macro)    │         Extreme
                    │                  │
         │          └────────┬─────────┘          │
         │                   │                    │
         │                   │                    │
         ▼                   │                    ▼
    ┌─────────┐             │              ┌─────────┐
    │factory/ │◄────────────┴──────────────►│  eval/  │
    │         │  Codegen generates           │         │
    │STORAGE  │  code for both               │COMPUTE  │
    └─────────┘                              └─────────┘

    How graphs      Macros control           What you do
    are STORED      BOTH extremes            WITH graphs
```

### The Extremes Explained

#### Storage Extreme (projection/factory/)

**Concerns**:

- Data layout (CSR, adjacency lists, Arrow columns)
- Memory representation (Vec, HugeArray, mmap)
- Compression (delta encoding, offset packing)
- Ingestion (Arrow → GraphStore)
- Schema (node labels, edge types, properties)

**Questions**:

- Where does the data live?
- How is it structured?
- How do you access it?
- How much space does it take?

**Examples**:

```rust
// Storage decisions
ArrowNativeFactory::new(node_table, edge_table)
    .with_backend(Backend::CSR)           // Storage layout
    .with_compression(Compression::Delta)  // Storage optimization
    .build()                              // Materialize storage
```

#### Computation Extreme (projection/eval/)

**Concerns**:

- Algorithms (PageRank, Louvain, BFS)
- Transformations (filtering, aggregation)
- Pipelines (chained computations)
- Execution (sequential, parallel, distributed)

**Questions**:

- What computation runs?
- In what order?
- How does data flow?
- What are the results?

**Examples**:

```rust
// Computation decisions
PipelineExecutor::new()
    .add_step(PageRank::new(config))     // Computation algorithm
    .add_step(Louvain::new(config))      // Computation algorithm
    .execute(graph)                      // Run computation
```

---

## Codegen Controls Both Extremes

### The Meta Macro Processor

**`projection/codegen/` generates code for BOTH**:

```rust
eval! {
    // Storage directive (Factory target)
    @storage {
        layout: csr,
        compression: delta,
        backend: arrow,
    }

    // Computation directive (Eval target)
    @compute {
        algorithm: pagerank,
        iterations: 20,
        parallelism: rayon,
    }
}
```

**The macro expands to**:

```rust
// Generated Factory code (Storage)
let factory = ArrowNativeFactory::new(...)
    .with_layout(Layout::CSR)
    .with_compression(Compression::Delta);
let graph = factory.build()?;

// Generated Eval code (Computation)
let executor = PipelineExecutor::new()
    .add_step(PageRank::new(...));
let result = executor.execute(&graph)?;
```

---

## Why This Matters

### 1. Separation of Concerns

**Storage and Computation are orthogonal**:

| Storage (Factory) | Computation (Eval)  |
| ----------------- | ------------------- |
| CSR topology      | PageRank algorithm  |
| Vec-based lists   | Louvain algorithm   |
| Arrow columns     | BFS traversal       |
| HugeArrays        | Node2Vec embeddings |

**You can mix and match**:

- CSR storage + PageRank computation ✅
- Vec storage + PageRank computation ✅
- Arrow storage + PageRank computation ✅
- CSR storage + Louvain computation ✅
- etc.

### 2. Optimization Opportunities

**Codegen can optimize across the boundary**:

```rust
eval! {
    // Storage: Arrow with Float64 column
    @storage {
        property: score -> Float64 (arrow_column: 5)
    }

    // Computation: Filter by score
    @compute {
        filter: score > 0.5
    }
}
```

**Optimization**: Codegen sees BOTH sides!

- **Storage side**: score is Arrow Float64 column 5
- **Computation side**: Filter condition `score > 0.5`
- **Optimization**: Use Arrow compute kernel (SIMD!) instead of Rust loop
- **Zero-copy**: No PropertyValues allocation needed

```rust
// Generated optimized code
let filtered_mask = arrow2::compute::comparison::gt_scalar(
    arrow_table.column(5),  // Direct Arrow access
    0.5                     // Scalar comparison (SIMD!)
)?;
// No Vec allocation, no PropertyValues, SIMD vectorized!
```

### 3. Complete Control

**Users control BOTH extremes through macros**:

```rust
eval! {
    // Experiment with storage
    @storage { backend: vec }      // Simple Vec
    @compute { algorithm: pagerank }
    // Fast to test, but memory-inefficient
}

eval! {
    // Switch to optimized storage
    @storage { backend: csr_compressed }  // CSR + compression
    @compute { algorithm: pagerank }
    // Same computation, different storage!
}

eval! {
    // Zero-copy Arrow
    @storage { backend: arrow_zerocopy }
    @compute { algorithm: pagerank }
    // Same computation, zero-copy!
}
```

**Same computation logic, different storage strategies!**

---

## The Codegen Targets

### Target 1: projection/factory/ (Storage)

**What codegen generates**:

1. **Factory construction**

   ```rust
   let factory = ArrowNativeFactory::new(node_table, edge_table);
   ```

2. **Storage configuration**

   ```rust
   factory.with_backend(Backend::CSR)
          .with_id_map(IdMapBackend::Huge)
          .with_properties(PropertyBackend::Arrow);
   ```

3. **Schema mapping**

   ```rust
   factory.with_node_labels(vec![NodeLabel::of("Person")])
          .with_edge_types(vec![RelationshipType::of("KNOWS")]);
   ```

4. **Property mapping**

   ```rust
   factory.with_property_mapping("age", PropertyMapping {
       source_column: 2,
       target_type: ValueType::Long,
       default: DefaultValue::long(0),
   });
   ```

5. **Build invocation**
   ```rust
   let graph_store = factory.build()?;
   ```

### Target 2: projection/eval/ (Computation)

**What codegen generates**:

1. **Pipeline construction**

   ```rust
   let executor = PipelineExecutor::new();
   ```

2. **Step configuration**

   ```rust
   executor.add_step(PageRank::new(PageRankConfig {
       max_iterations: 20,
       damping_factor: 0.85,
       tolerance: 1e-4,
   }));
   ```

3. **Computation chaining**

   ```rust
   executor.add_step(Louvain::new(LouvainConfig { ... }))
           .add_step(NodeSimilarity::new(config));
   ```

4. **Execution invocation**

   ```rust
   let results = executor.execute(&graph_store)?;
   ```

5. **Result handling**
   ```rust
   let pagerank_scores = results.get_property("pagerank")?;
   let communities = results.get_property("community")?;
   ```

---

## Example: Complete Macro Controlling Both

```rust
eval! {
    // ========================================
    // STORAGE EXTREME (Factory target)
    // ========================================
    @storage {
        source: arrow {
            nodes: "data/users.parquet",
            edges: "data/friendships.parquet",
        },

        backend: {
            id_map: simple,        // HashMap + Vec
            topology: csr,         // Compressed Sparse Row
            properties: columnar,  // Vec-based columns
        },

        schema: {
            node_labels: ["User"],
            edge_types: ["FRIENDS_WITH"],
        },

        properties: {
            age: nodes.age as Int64,
            name: nodes.name as String,
            weight: edges.weight as Float64,
        },

        optimizations: {
            compression: delta,
            zero_copy: where_possible,
        }
    }

    // ========================================
    // COMPUTATION EXTREME (Eval target)
    // ========================================
    @compute {
        pipeline: [
            // Step 1: Validate data
            validate {
                assert: age > 0,
                assert: weight > 0.0,
            },

            // Step 2: Compute centrality
            pagerank {
                max_iterations: 20,
                damping_factor: 0.85,
                source_property: weight,  // Weighted PageRank
            },

            // Step 3: Find communities
            louvain {
                tolerance: 0.0001,
                source_property: weight,  // Weighted Louvain
            },

            // Step 4: Filter results
            filter {
                condition: pagerank > 0.001,
            }
        ],

        execution: {
            parallelism: rayon,
            batch_size: 10000,
        }
    }

    // ========================================
    // Codegen coordinates across boundary
    // ========================================
    @optimize {
        // Zero-copy: weight property
        zero_copy_properties: [weight],

        // Pushdown: validation to Arrow
        pushdown_filters: [age > 0, weight > 0.0],

        // Fusion: PageRank + Louvain share weight access
        fuse_property_access: [pagerank, louvain],
    }
}
```

**This single macro controls**:

- ✅ Storage layout (CSR)
- ✅ Property representation (columnar)
- ✅ Schema definition (labels, types)
- ✅ Computation pipeline (PageRank → Louvain → Filter)
- ✅ Execution strategy (parallel with Rayon)
- ✅ Cross-boundary optimization (zero-copy, pushdown, fusion)

---

## The Power of Dual Targets

### Before: Separate Systems

**Storage code** (manual):

```rust
let factory = ArrowNativeFactory::new(node_table, edge_table);
factory.with_backend(Backend::CSR);
let graph = factory.build()?;
```

**Computation code** (manual):

```rust
let executor = PipelineExecutor::new();
executor.add_step(PageRank::new(config));
let result = executor.execute(&graph)?;
```

**Problem**: No coordination! Compiler can't optimize across the boundary.

### After: Unified Codegen

**Macro controls both**:

```rust
eval! {
    @storage { backend: csr }
    @compute { pagerank { iterations: 20 } }
}
```

**Benefits**:

1. **Type safety** - Codegen ensures property types match across boundary
2. **Optimization** - Can fuse storage access with computation
3. **Clarity** - One declaration controls both storage and computation
4. **Flexibility** - Change storage without touching computation code

---

## Analogy: SQL

### SQL Does This Too!

**Storage (Table definition)**:

```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    age INT,
    name VARCHAR(100)
) WITH (
    storage = columnar,
    compression = zstd
);
```

**Computation (Query)**:

```sql
SELECT name, age
FROM users
WHERE age > 18
ORDER BY age DESC;
```

**SQL Engine coordinates**:

- Storage: Columnar format, compressed
- Computation: Filter, sort, project
- Optimization: Pushdown filter to storage, use indices

**rust-gds eval! macro is like SQL for graphs!**

---

## Implementation Strategy

### Phase 1: Separate Targets (TODAY)

**Factory and Eval work independently**:

```rust
// Manual factory
let graph = ArrowNativeFactory::new(...).build()?;

// Manual eval
let result = PipelineExecutor::new()
    .add_step(PageRank::new(...))
    .execute(&graph)?;
```

### Phase 2: Codegen Generates Both (TOMORROW)

**Macro generates factory AND eval code**:

```rust
eval! {
    @storage { ... }
    @compute { ... }
}

// Expands to factory + eval code
```

### Phase 3: Cross-Boundary Optimization (FUTURE)

**Codegen optimizes across storage/computation boundary**:

```rust
eval! {
    @storage { property: score -> Float64 (column: 5) }
    @compute { filter: score > 0.5 }
    @optimize { pushdown: true }
}

// Generates:
// - Arrow compute kernel for filter (storage side)
// - No PropertyValues allocation (zero-copy)
// - SIMD vectorization (hardware optimization)
```

---

## The Vision

### GDSL = Control Over Both Extremes

**One language to rule them all**:

```rust
gdsl! {
    // Storage extreme
    graph: "social_network" {
        storage: {
            backend: adaptive {
                small: vec,         // < 1M edges
                medium: csr,        // < 100M edges
                large: csr_huge,    // > 100M edges
            },
            properties: hybrid {
                hot: arrow_zerocopy,   // Frequently accessed
                cold: compressed,      // Rarely accessed
            }
        }
    }

    // Computation extreme
    pipeline: "community_detection" {
        compute: {
            louvain { parallel: true },
            pagerank { parallel: true },
            node2vec { dimensions: 128 },
        }
    }

    // Optimization across both
    optimize: {
        zero_copy: where_possible,
        pushdown: filters_to_storage,
        fusion: property_access,
        parallel: rayon,
    }
}
```

**One macro, complete control over storage AND computation!**

---

## Summary

### The Two Extremes

| Aspect        | Storage (Factory)           | Computation (Eval)               |
| ------------- | --------------------------- | -------------------------------- |
| **Purpose**   | Where data lives            | What you do with data            |
| **Concerns**  | Layout, memory, compression | Algorithms, pipelines, execution |
| **Location**  | `projection/factory/`       | `projection/eval/`               |
| **Questions** | How is it stored?           | What computation runs?           |
| **Examples**  | CSR, Arrow, HugeArrays      | PageRank, Louvain, BFS           |

### Codegen as Bridge

**`projection/codegen/` sits between**:

- Generates code for factory (storage)
- Generates code for eval (computation)
- Optimizes across the boundary

### The Power

**Macros control BOTH extremes**:

```rust
eval! {
    @storage { ... }   // Controls factory/
    @compute { ... }   // Controls eval/
}
```

**One unified language for complete graph programming!**

---

**Storage and Computation: The two extremes. Codegen: The bridge. GDSL: The unifying language!** 🎯
