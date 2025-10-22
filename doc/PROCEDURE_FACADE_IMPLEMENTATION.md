# Procedure Facade Implementation Guide - Getting to Gamma

**Date**: October 22, 2025  
**Goal**: Translate 3 prototype facades to validate pattern  
**Target**: Gamma (Pre-Prim 0.0.x with working user API)

---

## 🎯 Today's Mission

Build **3 prototype facades** that prove the pattern works:

1. **DegreeCentrality** (trivial - no config needed)
2. **PageRank** (standard - has configuration)
3. **Louvain** (iterative - has convergence logic)

Each facade will support **4 execution modes**: stream, stats, mutate, write.

---

## 📁 File Structure

```
gds/src/procedures/
├── facades/                          ← NEW: User-facing API
│   ├── mod.rs                       (re-exports + trait definitions)
│   ├── traits.rs                    (Runner, StreamResults, StatsResults, etc.)
│   ├── builder_base.rs              (shared builder logic)
│   ├── centrality/
│   │   ├── mod.rs
│   │   ├── degree_centrality.rs     (simplest)
│   │   └── pagerank.rs              (standard)
│   └── community/
│       ├── mod.rs
│       └── louvain.rs               (iterative)
│
├── [31 algorithm specs - unchanged]
└── mod.rs                           (re-exports facades)
```

---

## 🏗️ Step 1: Create Trait System

**File: `gds/src/procedures/facades/traits.rs`**

```rust
/// Common trait for all algorithm runners
pub trait AlgorithmRunner {
    type Config;
    type Result;
    
    fn validate_config(&self) -> Result<(), String>;
    fn execute(&self) -> Result<Self::Result, String>;
}

/// Support for streaming results
pub trait StreamResults<T> {
    fn stream(self) -> Result<Box<dyn Iterator<Item = T>>>;
}

/// Support for statistical aggregation
pub trait StatsResults {
    type Stats;
    fn stats(self) -> Result<Self::Stats>;
}

/// Support for in-place mutations
pub trait MutateResults {
    type MutationResult;
    fn mutate(self, property_name: &str) -> Result<Self::MutationResult>;
}

/// Support for persistent writes
pub trait WriteResults {
    type WriteResult;
    fn write(self, property_name: &str) -> Result<Self::WriteResult>;
}
```

---

## 🛠️ Step 2: Create Base Builder

**File: `gds/src/procedures/facades/builder_base.rs`**

```rust
/// Shared builder logic for all algorithms
pub struct ExecutionContext {
    pub graph: Arc<Graph>,
    pub concurrency: usize,
    pub termination_flag: TerminationFlag,
}

impl ExecutionContext {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self {
            graph,
            concurrency: num_cpus::get(),
            termination_flag: TerminationFlag::RUNNING_TRUE,
        }
    }
    
    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }
}

/// Base result types
#[derive(Debug)]
pub struct MutationResult {
    pub property_name: String,
    pub nodes_modified: u64,
    pub time_ms: u64,
}

#[derive(Debug)]
pub struct WriteResult {
    pub property_name: String,
    pub backend: String,
    pub nodes_written: u64,
    pub time_ms: u64,
}
```

---

## 📝 Step 3: Implement Facade 1 - DegreeCentrality (Trivial)

**File: `gds/src/procedures/facades/centrality/degree_centrality.rs`**

```rust
use crate::procedures::degree_centrality::DegreeCentralityAlgorithmSpec;
use crate::api::Graph;

/// Result type for DegreeCentrality
#[derive(Debug)]
pub struct DegreeCentralityResult {
    pub scores: Vec<(u64, f64)>,
    pub computation_time_ms: u64,
}

/// High-level facade for DegreeCentrality
pub struct DegreeCentralityFacade<'a> {
    graph: &'a Graph,
}

impl<'a> DegreeCentralityFacade<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }
    
    /// Stream all degree scores
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = (u64, f64)>>> {
        let start = std::time::Instant::now();
        
        let result = DegreeCentralityAlgorithmSpec::execute(
            self.graph,
            &Default::default(),
        )?;
        
        let scores: Vec<_> = result
            .into_iter()
            .enumerate()
            .map(|(i, score)| (i as u64, score))
            .collect();
        
        Ok(Box::new(scores.into_iter()))
    }
    
    /// Get statistics about degree distribution
    pub fn stats(self) -> Result<DegreeCentralityStats> {
        let result = DegreeCentralityAlgorithmSpec::execute(
            self.graph,
            &Default::default(),
        )?;
        
        let scores: Vec<f64> = result.collect();
        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        Ok(DegreeCentralityStats {
            min_degree: sorted[0],
            max_degree: sorted[sorted.len() - 1],
            mean_degree: scores.iter().sum::<f64>() / scores.len() as f64,
            median_degree: sorted[sorted.len() / 2],
            node_count: scores.len(),
        })
    }
    
    /// Store degrees as node property
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        let start = std::time::Instant::now();
        let result = DegreeCentralityAlgorithmSpec::execute(
            self.graph,
            &Default::default(),
        )?;
        
        self.graph.set_node_property(property_name, result)?;
        
        Ok(MutationResult {
            property_name: property_name.to_string(),
            nodes_modified: self.graph.node_count(),
            time_ms: start.elapsed().as_millis() as u64,
        })
    }
}

#[derive(Debug)]
pub struct DegreeCentralityStats {
    pub min_degree: f64,
    pub max_degree: f64,
    pub mean_degree: f64,
    pub median_degree: f64,
    pub node_count: usize,
}
```

---

## 📝 Step 4: Implement Facade 2 - PageRank (Standard)

**File: `gds/src/procedures/facades/centrality/pagerank.rs`**

```rust
use crate::procedures::pagerank::{PageRankConfig, PageRankAlgorithmSpec};
use crate::api::Graph;

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
    
    // Fluent configuration
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
    
    // Execute modes
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = (u64, f64)>>> {
        self.config.validate()?;
        let result = PageRankAlgorithmSpec::execute(self.graph, &self.config)?;
        
        Ok(Box::new(
            result.node_scores
                .iter()
                .enumerate()
                .map(|(i, score)| (i as u64, *score))
        ))
    }
    
    pub fn stats(self) -> Result<PageRankStats> {
        self.config.validate()?;
        let result = PageRankAlgorithmSpec::execute(self.graph, &self.config)?;
        
        let scores = &result.node_scores;
        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        Ok(PageRankStats {
            min_score: sorted[0],
            max_score: sorted[sorted.len() - 1],
            mean_score: scores.iter().sum::<f64>() / scores.len() as f64,
            iterations_run: result.iterations,
            converged: result.converged,
        })
    }
    
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        let start = std::time::Instant::now();
        self.config.validate()?;
        let result = PageRankAlgorithmSpec::execute(self.graph, &self.config)?;
        
        self.graph.set_node_property(property_name, result.node_scores)?;
        
        Ok(MutationResult {
            property_name: property_name.to_string(),
            nodes_modified: self.graph.node_count(),
            time_ms: start.elapsed().as_millis() as u64,
        })
    }
}

#[derive(Debug)]
pub struct PageRankStats {
    pub min_score: f64,
    pub max_score: f64,
    pub mean_score: f64,
    pub iterations_run: u32,
    pub converged: bool,
}
```

---

## 📝 Step 5: Implement Facade 3 - Louvain (Iterative)

**File: `gds/src/procedures/facades/community/louvain.rs`**

```rust
use crate::procedures::louvain::{LouvainConfig, LouvainAlgorithmSpec};
use crate::api::Graph;

pub struct LouvainBuilder<'a> {
    graph: &'a Graph,
    config: LouvainConfig,
}

impl<'a> LouvainBuilder<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            config: LouvainConfig::default(),
        }
    }
    
    // Configuration
    pub fn max_iterations(mut self, n: u32) -> Self {
        self.config.max_iterations = n;
        self
    }
    
    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }
    
    // Execution modes
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = (u64, u64)>>> {
        self.config.validate()?;
        let result = LouvainAlgorithmSpec::execute(self.graph, &self.config)?;
        
        Ok(Box::new(
            result.community_ids
                .iter()
                .enumerate()
                .map(|(i, community_id)| (i as u64, *community_id))
        ))
    }
    
    pub fn stats(self) -> Result<LouvainStats> {
        self.config.validate()?;
        let result = LouvainAlgorithmSpec::execute(self.graph, &self.config)?;
        
        let community_count = result.community_ids.iter().max().unwrap_or(&0) + 1;
        
        Ok(LouvainStats {
            community_count,
            modularity: result.modularity,
            iterations_run: result.iterations,
            nodes_processed: self.graph.node_count(),
        })
    }
    
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        let start = std::time::Instant::now();
        self.config.validate()?;
        let result = LouvainAlgorithmSpec::execute(self.graph, &self.config)?;
        
        self.graph.set_node_property(property_name, result.community_ids)?;
        
        Ok(MutationResult {
            property_name: property_name.to_string(),
            nodes_modified: self.graph.node_count(),
            time_ms: start.elapsed().as_millis() as u64,
        })
    }
}

#[derive(Debug)]
pub struct LouvainStats {
    pub community_count: u64,
    pub modularity: f64,
    pub iterations_run: u32,
    pub nodes_processed: u64,
}
```

---

## 🚀 Today's Execution Plan

1. **Create facades/ directory structure**
   ```bash
   mkdir -p gds/src/procedures/facades/{centrality,community}
   touch gds/src/procedures/facades/{mod.rs,traits.rs,builder_base.rs}
   ```

2. **Create trait system** (30 min)
   - Write traits.rs with Runner, StreamResults, etc.
   - Write builder_base.rs with ExecutionContext

3. **Implement DegreeCentrality facade** (20 min)
   - Simplest - validates pattern works
   - Prove stream/stats/mutate modes work

4. **Implement PageRank facade** (30 min)
   - Builder pattern with configuration
   - Test fluent API feels good

5. **Implement Louvain facade** (30 min)
   - Iterative with convergence
   - Validate pattern scales

6. **Test all three** (20 min)
   ```bash
   cargo build
   cargo test procedures::facades::
   ```

7. **Validate it compiles and tests pass**

---

## 🎯 Success Criteria

✅ All three facades compile without warnings  
✅ Each facade supports stream/stats/mutate modes  
✅ Builder pattern feels ergonomic (chaining works)  
✅ Tests validate results are correct  
✅ Pattern is clear enough to replicate for other 28 algorithms

---

## 📊 After Today

You'll have proven:
- ✅ Facade pattern works for simple, standard, and iterative algorithms
- ✅ User API feels good (ergonomic, fluent, Rust-idiomatic)
- ✅ Ready to roll out to all 31 in Week 2

Then you'll have the foundation for everything else (infrastructure, similarity, ML, etc.).

---

**This is the work that turns "we have algorithms" into "users can use algorithms."** 

Let's build it. 🚀

