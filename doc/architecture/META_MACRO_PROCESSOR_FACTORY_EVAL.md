# The Meta Macro Processor: Factory & Eval Code Generation

**Date**: October 15, 2025  
**Insight**: The codegen system generates BOTH factory and eval code  
**Concept**: "Native" = GDSL Runtime = One's Native Language

---

## The Core Insight

### What "Native" Really Means

**Original confusion**: "native" from Java GDS's "Native Projection" (Neo4j-specific)

**Actual meaning**: **GDSL Runtime = Your Native Language for Graph Computation**

Like speaking your native language vs. a foreign language:

- **Foreign Language**: Writing raw Rust graph algorithms from scratch
- **Native Language**: Using GDSL (Graph Domain Specific Language) expressions

**The GDSL Runtime (`projection/`) IS your native language for graphs!**

---

## The Meta Macro Processor

### What It Is

**The `codegen/` system is a Meta Macro Processor**:

- Not just generating code
- Generating code that generates structures
- Meta-level: It reasons about code generation itself

### What It Generates

#### 1. Factory Information

**Factory descriptors** (how to build GraphStores):

```rust
// Meta Macro Processor generates:
pub struct ArrowFactoryDescriptor {
    pub node_schema: SchemaDescriptor,
    pub edge_schema: SchemaDescriptor,
    pub property_mappings: Vec<PropertyMapping>,
}

// Which drives:
impl ArrowNativeFactory {
    pub fn build_from_descriptor(desc: ArrowFactoryDescriptor) -> Self {
        // Factory construction guided by descriptor
    }
}
```

**Example**: `eval!` could generate factory configurations!

```rust
eval! {
    factory: arrow {
        node_table: "nodes.parquet",
        edge_table: "edges.parquet",
        properties: {
            "pageRank" -> Float64,
            "community" -> Int64,
        }
    }
}

// Expands to:
// ArrowNativeFactory::new(...)
//     .with_properties(...)
//     .build()
```

#### 2. Eval Information

**Eval descriptors** (how to execute computations):

```rust
// Meta Macro Processor generates:
pub struct PipelineDescriptor {
    pub steps: Vec<StepDescriptor>,
    pub computations: Vec<ComputationDescriptor>,
    pub storage: Vec<StorageDescriptor>,
}

// Which drives:
impl PipelineExecutor {
    pub fn execute_from_descriptor(desc: PipelineDescriptor) -> Result {
        // Pipeline execution guided by descriptor
    }
}
```

**Example**: `eval!` generates ML pipeline!

```rust
eval! {
    pipeline: ml {
        step: pagerank {
            max_iterations: 20,
            damping_factor: 0.85,
        },
        step: louvain {
            tolerance: 0.0001,
        }
    }
}

// Expands to:
// PipelineExecutor::new()
//     .add_step(PageRank::new(...))
//     .add_step(Louvain::new(...))
//     .execute()
```

---

## The Unified Architecture

### GDSL Runtime = Factory + Eval

```
┌─────────────────────────────────────────────────────────┐
│                    GDSL Runtime                         │
│                 (projection/ module)                    │
│                                                         │
│  ┌──────────────────────┐  ┌──────────────────────┐   │
│  │   factory/           │  │   eval/              │   │
│  │   (CAR - given)      │  │   (CDR - derived)    │   │
│  │                      │  │                      │   │
│  │ - Arrow ingestion    │  │ - ML pipelines       │   │
│  │ - Neo4j connector    │  │ - Form evaluators    │   │
│  │ - Polars connector   │  │ - Procedures         │   │
│  └──────────────────────┘  └──────────────────────┘   │
│              ▲                        ▲                │
│              │                        │                │
│              └────────┬───────────────┘                │
│                       │                                │
│              ┌────────▼─────────┐                      │
│              │   codegen/       │                      │
│              │   (Meta Macro)   │                      │
│              │                  │                      │
│              │ Generates BOTH:  │                      │
│              │ - Factory info   │                      │
│              │ - Eval info      │                      │
│              └──────────────────┘                      │
└─────────────────────────────────────────────────────────┘
```

### The Meta Macro Processor Knows About Everything

**Why?** Because it needs to coordinate between factory and eval!

**Example scenario**:

```rust
eval! {
    // Factory info: How to load data
    load: arrow("nodes.parquet", "edges.parquet") {
        properties: {
            age -> Int64,
            name -> String,
        }
    },

    // Eval info: What to compute
    compute: pipeline {
        step: validate {
            check: age > 0,
            check: name.is_some(),
        },
        step: pagerank {
            max_iterations: 20,
        },
        step: export {
            property: pagerank,
            format: parquet,
        }
    }
}
```

**The Meta Macro Processor must**:

1. Generate factory code (Arrow ingestion)
2. Generate eval code (pipeline execution)
3. **Coordinate**: pagerank property from eval → factory export descriptor
4. **Type check**: age is Int64 in factory, used as numeric in eval
5. **Optimize**: Can Arrow columns be wrapped zero-copy?

---

## "One's Native Language"

### What This Means

**Programming graphs in Rust** = Speaking a foreign language

```rust
// Raw Rust (foreign language)
let mut ranks = vec![1.0; graph.node_count()];
for _ in 0..20 {
    let mut new_ranks = vec![0.0; graph.node_count()];
    for node in graph.nodes() {
        let out_degree = graph.out_degree(node);
        let contribution = ranks[node] / out_degree as f64;
        for neighbor in graph.neighbors(node) {
            new_ranks[neighbor] += contribution;
        }
    }
    for i in 0..ranks.len() {
        ranks[i] = 0.15 + 0.85 * new_ranks[i];
    }
}
```

**Programming graphs in GDSL** = Speaking your native language

```rust
// GDSL (native language)
eval! {
    pagerank {
        max_iterations: 20,
        damping_factor: 0.85,
    }
}
```

**The difference**:

- Foreign language: You think in terms of loops, vectors, indices
- Native language: You think in terms of graph concepts (PageRank, communities, centrality)

### Why "Native" Makes Sense

**GDSL is native to graph problems**, like:

- SQL is native to relational data
- Regular expressions are native to text patterns
- HTML is native to document structure

**The GDSL Runtime (`projection/`) provides the native environment!**

---

## The Codegen System as Meta-Level Processor

### Level 0: Raw Rust

```rust
// Direct implementation
struct Graph { ... }
impl Graph {
    fn pagerank(&self) -> Vec<f64> { ... }
}
```

### Level 1: GDSL Runtime

```rust
// Using the runtime
use projection::eval::ml::PageRank;
let result = PageRank::new(config).execute(graph)?;
```

### Level 2: Meta Macro Processor

```rust
// Generating the runtime usage
eval! {
    pagerank { max_iterations: 20 }
}

// Expands to Level 1 code
// Which uses Level 0 implementations
```

**The codegen system operates at Level 2** - it generates code that uses the runtime!

---

## Code Generation Patterns

### Pattern 1: Factory Descriptor Generation

```rust
// User writes:
eval! {
    factory: arrow {
        nodes: "data/nodes.parquet",
        edges: "data/edges.parquet",
    }
}

// Meta Macro generates:
{
    let node_table = arrow2::read_parquet("data/nodes.parquet")?;
    let edge_table = arrow2::read_parquet("data/edges.parquet")?;
    let factory = ArrowNativeFactory::new(node_table, edge_table);
    factory.build_graph_store()?
}
```

### Pattern 2: Eval Descriptor Generation

```rust
// User writes:
eval! {
    pipeline {
        pagerank { iterations: 20 },
        louvain { tolerance: 0.0001 },
    }
}

// Meta Macro generates:
{
    let mut executor = PipelineExecutor::new();
    executor.add_step(Box::new(PageRank::new(PageRankConfig {
        max_iterations: 20,
        ..Default::default()
    })));
    executor.add_step(Box::new(Louvain::new(LouvainConfig {
        tolerance: 0.0001,
        ..Default::default()
    })));
    executor.execute(graph)?
}
```

### Pattern 3: Combined Factory + Eval

```rust
// User writes:
eval! {
    load: arrow("data/graph.parquet"),
    compute: pagerank { iterations: 20 },
    export: parquet("output/ranks.parquet"),
}

// Meta Macro generates:
{
    // Factory code
    let graph_store = ArrowNativeFactory::from_file("data/graph.parquet")?.build()?;

    // Eval code
    let ranks = PageRank::new(config).execute(&graph_store)?;

    // Factory code (export)
    let export_factory = ArrowExportFactory::new();
    export_factory.export_properties(&graph_store, "output/ranks.parquet")?;
}
```

---

## Why This Architecture Is Powerful

### 1. Separation of Concerns

**Factory** (projection/factory/):

- How to get data IN
- Arrow, Neo4j, Polars, etc.
- Schema inference
- Zero-copy optimization

**Eval** (projection/eval/):

- What to compute
- ML pipelines, procedures
- Algorithm execution
- Result aggregation

**Codegen** (projection/codegen/):

- How to coordinate them
- Generate descriptors for both
- Type safety across boundary
- Optimization opportunities

### 2. The Meta Macro Sees Everything

**It can optimize across the boundary**:

```rust
eval! {
    // Factory: Load Arrow table
    load: arrow("data.parquet") {
        properties: { score -> Float64 }
    },

    // Eval: Use property directly
    compute: filter { where: score > 0.5 },
}

// Meta Macro can optimize:
// - Zero-copy: Arrow Float64Array → PropertyValues
// - Pushdown: Filter in Arrow compute kernel (SIMD!)
// - Fusion: Combine load + filter into single pass
```

**Normal code can't do this** because factory and eval are separate!

### 3. Native Language for Graphs

**Users think in graph terms**:

```rust
eval! {
    // Natural graph language
    load_graph,
    find_communities,
    rank_nodes,
    export_results,
}
```

**Not in implementation terms**:

```rust
// Unnatural implementation language
let mut adj_lists = Vec::new();
let mut id_map = HashMap::new();
for row in table.rows() { ... }
let mut scores = vec![0.0; ...];
for iteration in 0..max_iter { ... }
```

---

## Future: Full GDSL

### Today: Pieces Exist

- ✅ Factory (Arrow ingestion working!)
- ✅ Eval (ML pipelines working!)
- ✅ Codegen (Eval macro working!)

### Tomorrow: Unified Language

```rust
// Complete GDSL program
gdsl! {
    // Data sources
    nodes: arrow("users.parquet"),
    edges: arrow("friendships.parquet"),

    // Property definitions
    properties: {
        age: nodes.age as Int64,
        name: nodes.name as String,
        weight: edges.weight as Float64,
    },

    // Graph construction
    graph: build {
        node_labels: ["User"],
        relationship_types: ["FRIENDS_WITH"],
    },

    // Computations
    compute: {
        communities: louvain { tolerance: 0.0001 },
        centrality: pagerank { iterations: 20 },
        embeddings: node2vec { dimensions: 128 },
    },

    // Validation
    validate: {
        assert: communities.modularity() > 0.3,
        assert: centrality.sum() ≈ graph.node_count(),
    },

    // Export
    export: {
        graph: neo4j("bolt://localhost:7687"),
        properties: parquet("results/"),
        report: json("results/summary.json"),
    },
}
```

**The Meta Macro Processor makes this possible** because it coordinates all pieces!

---

## The Architecture Today

### projection/ = GDSL Runtime

**Your native language for graphs**:

```
projection/
├── factory/         # How to BUILD graphs (CAR - given data)
│   └── arrow/       # Arrow-native ingestion (Phase 1-7 ✅)
│
├── eval/            # How to USE graphs (CDR - derived computations)
│   ├── ml/          # ML pipelines (PageRank, Louvain, etc.)
│   ├── form/        # Form evaluators
│   └── procedure/   # Stored procedures
│
├── codegen/         # How to GENERATE code for both
│   ├── eval_macro   # The Meta Macro Processor
│   ├── functors     # Type conversions
│   └── descriptors  # Factory + Eval descriptors
│
├── traits/          # Core abstractions
└── impls/           # Concrete implementations
```

**All three work together** to provide the "native language" for graph programming!

---

## Summary

### The Insight

> "The Meta Macro Processor can code up both Factory info and Eval info. So now 'native' means GDSL Runtime, one's Native Language in a sense."

**Translation**:

1. **Meta Macro Processor** = `projection/codegen/`

   - Generates factory descriptors (how to build graphs)
   - Generates eval descriptors (how to compute on graphs)
   - Coordinates between them for optimization

2. **Native Language** = GDSL (Graph Domain Specific Language)

   - Not "native" like "Native Projection" (Java GDS's Neo4j-specific code)
   - "Native" like "one's native language" (natural expression of intent)
   - The way you SHOULD think about graphs (not loops and vectors!)

3. **GDSL Runtime** = `projection/` module
   - Factory: Build graphs from data
   - Eval: Compute on graphs
   - Codegen: Generate code that uses both
   - **Together**: Your native environment for graph problems

### Why This Matters

**Before**: Three separate systems (factory, eval, codegen)

**Now**: One unified GDSL Runtime where:

- Factory and Eval are the runtime
- Codegen is the meta-level that coordinates them
- Users write in their "native language" (GDSL)
- The system handles factory + eval coordination automatically

**The rename from `native/` → `eval/` actually clarifies this!**

- Not "native projection" (Java GDS concept)
- "Eval" clearly part of the GDSL Runtime
- Paired with "factory" for complete picture

---

**The GDSL Runtime: Your native language for graph computation!** 🎯
