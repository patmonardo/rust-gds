# Procedure Facade Design Strategy - Idiomatic Rust SDK

**Date**: October 22, 2025  
**Philosophy**: Design for FormDB users first, then generalize  
**Approach**: Rust idioms, not Java translations

---

## 🎯 Core Insight

You are **both the designer AND the user**.

This changes everything:

```
Traditional approach:
  Design abstract API → Hope users figure it out → Fix when complaints come

Your approach (BETTER):
  You use the API daily → You know exactly what hurts → Design for it
  
Result: API that's actually ergonomic
```

---

## 🏗️ Design Philosophy: The Three Layers

Your system should have **three distinct layers**, not a direct translation:

```
┌─────────────────────────────────────────┐
│  LAYER 3: User Application (FormDB)     │
│  "Run PageRank on my graph"             │
│  Rust idioms: `.stream()`, `.stats()`,  │
│  Result types: iterators, futures       │
└─────────────────────────────────────────┘
            ↑
     (What feels natural?)
            │
┌─────────────────────────────────────────┐
│  LAYER 2: Procedure Facade (PUBLIC API) │
│  Builder pattern, fluent interfaces     │
│  Execution modes as Rust enums/traits   │
│  Configuration as strongly-typed structs│
└─────────────────────────────────────────┘
            ↑
     (How do we orchestrate?)
            │
┌─────────────────────────────────────────┐
│  LAYER 1: Algorithm Specs (Implementation)│
│ (Your 31 algorithms - stays as is)      │
│ Internal machinery, not exposed to users│
└─────────────────────────────────────────┘
```

---

## 🤔 Ask Yourself: What Does FormDB Actually Want?

**Not "what does Java GDS do?" but "what would I write?"**

### **Scenario 1: Running PageRank**

```rust
// ❌ JAVA-STYLE (verbose, not idiomatic)
let config = PageRankConfig::builder()
    .iterations(20)
    .damping_factor(0.85)
    .tolerance(0.0001)
    .build()?;
    
let procedure = PageRankProcedure::new(graph);
let result = procedure.execute(&config, &context)?;

// ✅ RUST-STYLE (fluent, ergonomic)
let result = graph
    .pagerank()
    .iterations(20)
    .damping_factor(0.85)
    .tolerance(0.0001)
    .run()?;

// OR, if you prefer explicit stages:
let result = PageRank::new(graph)
    .iterations(20)
    .run()?;

// Even better - traits that handle modes:
let scores: Vec<f64> = graph.pagerank().stream()?;
let stats = graph.pagerank().stats()?;
graph.pagerank().mutate()?;  // Modifies graph in place
```

**Why the second is better:**
- Builder pattern is Rust standard (not Java-like)
- Method chaining is intuitive
- Modes become method names, not enum variants
- Returns are idiomatic types (Vec, stats structs)

---

## 📋 Facade Pattern for Your Use Case

### **Option 1: Graph Extension Methods (RECOMMENDED)**

```rust
// Add methods directly to Graph type
impl Graph {
    pub fn pagerank(&self) -> PageRankBuilder {
        PageRankBuilder::new(self)
    }
    
    pub fn louvain(&self) -> LouvainBuilder {
        LouvainBuilder::new(self)
    }
    
    // etc for all 31 algorithms
}

// Usage from FormDB:
let scores = my_graph.pagerank().stream()?;
let communities = my_graph.louvain().mutate()?;
```

**Pros:**
- Feels like the graph object is "aware" of algorithms
- Natural, fluent interface
- Easy to discover (IDE autocomplete)
- Idiomatic Rust trait extension

**Cons:**
- Graph struct becomes "aware" of many algorithms
- Potential namespace pollution

### **Option 2: Builder Module (Flexible)**

```rust
// Separate namespace
pub mod run {
    pub struct PageRankBuilder { /* ... */ }
    pub struct LouvainBuilder { /* ... */ }
}

// Usage from FormDB:
let scores = run::pagerank(graph)
    .iterations(20)
    .stream()?;
```

**Pros:**
- Clear separation of concerns
- Easy to add/remove algorithms
- Can organize by category

**Cons:**
- Less fluent feeling
- More explicit

### **Option 3: Hybrid (BEST OF BOTH)**

```rust
// Graph extension for high-frequency use:
impl Graph {
    pub fn pagerank(&self) -> algorithms::PageRankBuilder {
        algorithms::PageRankBuilder::new(self)
    }
}

// Also supports direct construction:
algorithms::PageRank::on(graph).run()?;

// Organized by category:
mod algorithms {
    pub mod centrality {
        pub struct PageRankBuilder { /* ... */ }
        pub struct BetweennessBuilder { /* ... */ }
    }
    pub use centrality::*;
}
```

---

## 🎨 What Each Facade Should Look Like

### **The PageRank Facade Example**

```rust
/// High-level PageRank runner
pub struct PageRankBuilder<'a> {
    graph: &'a Graph,
    config: PageRankConfig,
}

impl<'a> PageRankBuilder<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            config: PageRankConfig::default(),
        }
    }
    
    // Fluent configuration methods
    pub fn iterations(mut self, n: u32) -> Self {
        self.config.iterations = n;
        self
    }
    
    pub fn tolerance(mut self, t: f64) -> Self {
        self.config.tolerance = t;
        self
    }
    
    pub fn damping_factor(mut self, d: f64) -> Self {
        self.config.damping_factor = d;
        self
    }
    
    // STREAM mode: return iterator over all results
    pub fn stream(self) -> Result<impl Iterator<Item = (NodeId, f64)>> {
        let result = self.config.validate()
            .and_then(|_| PageRankAlgorithmSpec::execute(
                self.graph,
                &self.config,
                &ExecutionContext::new()
            ))?;
        
        Ok(result.node_scores.iter().enumerate()
            .map(|(i, score)| (NodeId(i as u64), *score)))
    }
    
    // STATS mode: return aggregated statistics
    pub fn stats(self) -> Result<PageRankStats> {
        let result = self.config.validate()
            .and_then(|_| PageRankAlgorithmSpec::execute(
                self.graph,
                &self.config,
                &ExecutionContext::new()
            ))?;
        
        Ok(PageRankStats {
            min_score: result.node_scores.iter().cloned().fold(f64::INFINITY, f64::min),
            max_score: result.node_scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            mean_score: result.node_scores.iter().sum::<f64>() / result.node_scores.len() as f64,
            iterations_run: result.iterations,
            converged: result.converged,
        })
    }
    
    // MUTATE mode: store as node property
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        let result = self.execute()?;
        self.graph.set_node_property(property_name, result.node_scores)?;
        Ok(MutationResult {
            property_name: property_name.to_string(),
            nodes_written: self.graph.node_count() as u64,
        })
    }
    
    // WRITE mode: write to storage/database
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let result = self.execute()?;
        // Write to storage backend
        self.graph.storage().write_property(property_name, result.node_scores)?;
        Ok(WriteResult {
            property_name: property_name.to_string(),
            backend: "storage".to_string(),
        })
    }
    
    // Internal execution (shared)
    fn execute(&self) -> Result<PageRankResult> {
        self.config.validate()?;
        PageRankAlgorithmSpec::execute(
            self.graph,
            &self.config,
            &ExecutionContext::new()
        )
    }
}
```

---

## 📊 Result Types: Think Like Rust

### **Stream Mode**

```rust
// ❌ Java way: return raw list
fn stream(&self) -> Vec<(u64, f64)> { }

// ✅ Rust way: return iterator
fn stream(self) -> Result<impl Iterator<Item = (NodeId, f64)>> { }

// Why: Lazy evaluation, memory efficient, composable
// Usage:
for (node_id, score) in graph.pagerank().stream()? {
    println!("Node {} has score {}", node_id, score);
}

// Can chain with other iterators:
graph.pagerank()
    .stream()?
    .filter(|(_, score)| score > &0.5)
    .collect::<Vec<_>>()
```

### **Stats Mode**

```rust
// ✅ Return a strongly-typed struct (not a generic Map)
#[derive(Debug, Clone)]
pub struct PageRankStats {
    pub min_score: f64,
    pub max_score: f64,
    pub mean_score: f64,
    pub median_score: f64,
    pub std_dev: f64,
    pub iterations_run: u32,
    pub converged: bool,
}

// Usage is clear and type-safe:
let stats = graph.pagerank().stats()?;
println!("Converged in {} iterations", stats.iterations_run);
```

### **Mutate Mode**

```rust
// ✅ Rust way: make changes in-place, return summary
#[derive(Debug)]
pub struct MutationResult {
    pub property_name: String,
    pub nodes_modified: u64,
    pub time_ms: u64,
}

let result = graph.pagerank().mutate("pagerank_score")?;
println!("Modified {} nodes in {}ms", result.nodes_modified, result.time_ms);
```

---

## 🗂️ File Organization

Think about what FormDB (the user) sees:

```rust
// gds/src/procedures/mod.rs - USER-FACING

pub mod centrality;      // PageRank, Betweenness, etc
pub mod community;       // Louvain, LabelProp, etc
pub mod pathfinding;     // Dijkstra, BFS, etc
pub mod algorithms;      // Your 31 algorithm specs (INTERNAL)

// What users import:
use gds::procedures::centrality::*;
use gds::procedures::community::*;

// OR just:
use gds::procedures::*;  // Gets all facades
```

Each module exposes **only** the facade builders, not the internals.

---

## 🎯 Design Questions to Ask Yourself

For each algorithm, decide:

1. **What is this algorithm FOR?**
   - PageRank → "Find important nodes"
   - Louvain → "Find communities"
   - Dijkstra → "Find shortest path"
   
   This determines the default mode and API.

2. **What results matter most?**
   - Stats that matter: PageRank = max/min/mean
   - Stream data: Dijkstra = paths
   - Mutations: Louvain = community IDs
   
3. **How often will FormDB use this?**
   - Frequent → Make it ergonomic (extension methods)
   - Rare → Make it explicit (module functions)

4. **What configuration is essential?**
   - Iterations? Tolerance? Weights?
   - Only expose what users actually tune

---

## 💡 Example: Building 3 Different Facades

### **Facade 1: Simple Algorithm (DegreeCentrality)**

```rust
impl Graph {
    pub fn degree_centrality(&self) -> DegreeCentralityResult {
        // So simple, no builder needed!
        let result = DegreeCentralityAlgorithmSpec::execute(self)?;
        Ok(DegreeCentralityResult {
            scores: result.into_iter().collect(),
        })
    }
}

// Usage:
let degrees = my_graph.degree_centrality()?;
```

### **Facade 2: Configurable (PageRank)**

```rust
impl Graph {
    pub fn pagerank(&self) -> PageRankBuilder {
        PageRankBuilder::new(self)
    }
}

// Usage:
my_graph.pagerank()
    .iterations(50)
    .damping_factor(0.85)
    .stream()?
```

### **Facade 3: Complex (Dijkstra with targets)**

```rust
impl Graph {
    pub fn shortest_paths(&self, source: NodeId) -> ShortestPathBuilder {
        ShortestPathBuilder::new(self, source)
    }
}

impl ShortestPathBuilder {
    pub fn to_node(mut self, target: NodeId) -> Self {
        self.targets.push(target);
        self
    }
    
    pub fn to_nodes(mut self, targets: &[NodeId]) -> Self {
        self.targets.extend_from_slice(targets);
        self
    }
}

// Usage:
my_graph.shortest_paths(source)
    .to_node(target_a)
    .to_node(target_b)
    .stream()?
```

---

## 🚀 Implementation Strategy

### **Phase 1: Foundation (Week 1)**

```
1. Design the trait system:
   ├─ AlgorithmRunner trait (common interface)
   ├─ StreamResults trait (for stream mode)
   ├─ StatsResults trait (for stats mode)
   └─ MutateResults trait (for mutations)

2. Create base builders:
   ├─ AbstractBuilder (shared logic)
   └─ ExecutionContext (how to run algorithms)

3. Test with 3 algorithms:
   ├─ DegreeCentrality (trivial)
   ├─ PageRank (standard)
   └─ Dijkstra (complex)
```

### **Phase 2: Rollout (Week 2-3)**

```
4. Build facades for all 31:
   ├─ 5 centrality algorithms
   ├─ 5 community detection
   ├─ 10 path finding
   ├─ 2 spanning trees
   └─ 8 utility algorithms

5. Integration testing:
   ├─ Each algorithm facade works
   ├─ All four modes (stream/stats/mutate/write)
   └─ Error handling
```

### **Phase 3: Polish (Week 3-4)**

```
6. Performance profiling:
   ├─ Identify hot paths
   ├─ Optimize common operations
   └─ Benchmark vs Java GDS

7. Documentation:
   ├─ Examples for each facade
   ├─ Common patterns
   └─ Performance tips
```

---

## 🎁 Key Rust Idioms to Use

### **1. Builder Pattern (Already shown)**

```rust
graph.pagerank()
    .iterations(20)
    .run()?
```

### **2. Iterators Instead of Collections**

```rust
// ✅ Return iterator
pub fn stream(self) -> Result<impl Iterator<Item = (NodeId, f64)>>

// ❌ Don't return Vec
pub fn stream(self) -> Result<Vec<(NodeId, f64)>>
```

### **3. Strongly-Typed Results**

```rust
// ✅ Specific types
pub struct PageRankStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

// ❌ Generic types
pub struct Stats {
    pub values: HashMap<String, f64>,
}
```

### **4. Trait Objects for Polymorphism**

```rust
// Instead of enum for execution modes:
pub trait Algorithm {
    fn stream(&self) -> Result<Vec<Value>>;
    fn stats(&self) -> Result<Stats>;
}

// Different implementations for different modes
```

### **5. Error Types That Make Sense**

```rust
// ✅ Specific errors
pub enum AlgorithmError {
    ConfigValidation(String),
    GraphInvalid(String),
    ExecutionTimeout,
}

// ❌ Generic strings
Result<T, String>
```

---

## 📝 Your First Facade (Template)

```rust
/// High-level API for [AlgorithmName]
pub struct [AlgorithmName]Builder<'a> {
    graph: &'a Graph,
    config: [AlgorithmName]Config,
}

impl<'a> [AlgorithmName]Builder<'a> {
    /// Create new builder
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            config: [AlgorithmName]Config::default(),
        }
    }
    
    /// Configure: [relevant parameters]
    pub fn param(mut self, value: Type) -> Self {
        self.config.param = value;
        self
    }
    
    /// Execute and stream results
    pub fn stream(self) -> Result<impl Iterator<Item = (NodeId, Value)>> {
        self.config.validate()?;
        // Execute algorithm via spec
        // Return iterator
    }
    
    /// Execute and get statistics
    pub fn stats(self) -> Result<[AlgorithmName]Stats> {
        // Similar to stream, but aggregate
    }
    
    /// Execute and mutate graph
    pub fn mutate(self, property: &str) -> Result<MutationResult> {
        // Similar, but store result as property
    }
    
    /// Execute and write to storage
    pub fn write(self, property: &str) -> Result<WriteResult> {
        // Similar, but write to backend
    }
}
```

---

## 🎯 Decision Tree for Your First Session

```
Should I use extension methods?
    ├─ YES if: Algorithm is fundamental (PageRank, Louvain, Dijkstra)
    ├─ NO if: Algorithm is specialized (CELF, Steiner)
    └─ Try: Start with centrality (most fundamental)

Should I support all four modes?
    ├─ YES if: Algorithm can output both node properties and stats
    ├─ MAYBE if: Algorithm naturally fits one mode better
    └─ Try: Start with stream + stats (most flexible)

Should I use iterators?
    ├─ YES if: Result could be large (stream mode)
    ├─ NO if: Result is small (stats mode)
    └─ Try: Always use iterators (future-proof)

Should I simplify configuration?
    ├─ YES: Only expose parameters FormDB actually changes
    ├─ Keep defaults for everything else
    └─ Try: Start minimal, add parameters as needed
```

---

## 🔑 Remember

> "You are both designer AND user. Design for how you want to USE these algorithms, not how Java GDS exposed them."

This is your competitive advantage. Most library authors design for the theoretical "any user." You're designing for **yourself**—the most critical user.

---

**Next: Start with 3 facades (DegreeCentrality, PageRank, Louvain), perfect them, then roll out to the rest.**

The pattern will become clear, and you'll develop muscle memory for what good Rust facades feel like.
