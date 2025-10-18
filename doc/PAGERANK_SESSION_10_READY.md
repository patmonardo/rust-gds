# PageRank Implementation Plan - UPDATED WITH PREGEL ANALYSIS

**Status**: ✅ PREGEL INFRASTRUCTURE CONFIRMED  
**Location**: `src/procedure/algo/pagerank/`  
**Complexity**: MEDIUM (Pregel handles the hard parts)  
**Timeline**: Single concentrated session  
**Confidence**: HIGH

---

## Pregel Infrastructure Exists and Ready

### What We Have

```
src/pregel/
├── mod.rs                  (main module hub)
├── messages.rs             (message representation)
├── computation.rs          (computation trait)
├── computer.rs             (runner/executor)
├── executor.rs             (orchestration)
├── context/
│   └── pregel_context.rs   (execution context)
├── node_value.rs           (node state management)
├── schema.rs               (value schema definition)
├── reducers.rs             (message reducers - SUM, MAX, etc.)
├── messengers.rs           (message distribution)
├── queues.rs               (message queues)
├── compute_step.rs         (superstep handling)
├── projection.rs           (projection integration)
└── result.rs               (computation results)
```

This is **complete**. We don't need to build Pregel—just use it.

### Key Components

```rust
// Computation trait (what we implement)
pub trait Computation {
    fn init(&mut self, context: &mut InitContext);
    fn compute(&mut self, context: &mut ComputeContext, messages: Messages);
    fn master_compute(&mut self, context: &mut MasterComputeContext) -> bool;
}

// Schema (what properties nodes have)
pub struct PregelSchema { ... }

// Reducers (how to combine messages)
pub enum Reducer {
    Sum,
    Max,
    Min,
    // ... etc
}

// Computer (what runs everything)
pub struct Computer { ... }
pub fn run<C: Computation>(config, computation) -> Result<PregelResult>
```

---

## What We Build

### Directory Structure

```
src/procedure/algo/pagerank/
├── mod.rs                    (module hub)
├── spec.rs                   (PageRankAlgorithmSpec)
├── variants.rs               (enum: STANDARD, ARTICLE, EIGENVECTOR)
├── standard.rs               (StandardPageRankComputation - impl Computation)
├── article.rs                (ArticleRankComputation - impl Computation)
├── eigenvector.rs            (EigenvectorComputation - impl Computation)
├── result.rs                 (PageRankResult container)
├── config.rs                 (PageRankConfig - parser/validation)
└── utilities/
    ├── mod.rs
    ├── degree_functions.rs   (degree calculation)
    └── scaling.rs            (L2-Norm application)
```

### File 1: `src/procedure/algo/pagerank/mod.rs` (Module Hub)

```rust
//! PageRank Algorithm Implementation
//!
//! Three variants via Pregel:
//! - StandardPageRank: rank / degree
//! - ArticleRank: rank / (degree + avg_degree)
//! - Eigenvector: power iteration with L2-norm

pub mod variants;
pub mod spec;
pub mod config;
pub mod result;
pub mod standard;
pub mod article;
pub mod eigenvector;
pub mod utilities;

pub use spec::PageRankAlgorithmSpec;
pub use variants::PageRankVariant;
pub use result::PageRankResult;
pub use config::PageRankConfig;
```

### File 2: `src/procedure/algo/pagerank/variants.rs`

```rust
//! PageRank algorithm variants

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageRankVariant {
    /// Standard PageRank: P(u) / out_degree(u)
    Standard,

    /// ArticleRank variant: uses (degree + avgDegree) normalization
    ArticleRank,

    /// Eigenvector centrality variant: power iteration with L2-norm
    Eigenvector,
}

impl PageRankVariant {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Standard => "pagerank",
            Self::ArticleRank => "articlerank",
            Self::Eigenvector => "eigenvector",
        }
    }
}
```

### File 3: `src/procedure/algo/pagerank/config.rs`

```rust
//! PageRank Configuration

use serde_json::Value as JsonValue;
use crate::projection::eval::procedure::ConfigError;

#[derive(Debug, Clone)]
pub struct PageRankConfig {
    pub max_iterations: usize,
    pub damping_factor: f64,
    pub tolerance: f64,
    pub relationship_weight_property: Option<String>,
    pub source_nodes: Option<Vec<u32>>,
    pub scale_property: Option<String>,
}

impl PageRankConfig {
    /// Parse from JSON
    pub fn parse(input: &JsonValue) -> Result<Self, ConfigError> {
        let max_iterations = input
            .get("max_iterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(20) as usize;

        let damping_factor = input
            .get("damping_factor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.85);

        let tolerance = input
            .get("tolerance")
            .and_then(|v| v.as_f64())
            .unwrap_or(1e-4);

        // Validate
        if damping_factor <= 0.0 || damping_factor >= 1.0 {
            return Err(ConfigError::InvalidValue {
                param: "damping_factor".to_string(),
                message: "must be between 0 and 1".to_string(),
            });
        }

        if tolerance <= 0.0 {
            return Err(ConfigError::InvalidValue {
                param: "tolerance".to_string(),
                message: "must be positive".to_string(),
            });
        }

        Ok(Self {
            max_iterations,
            damping_factor,
            tolerance,
            relationship_weight_property: None,
            source_nodes: None,
            scale_property: None,
        })
    }
}
```

### File 4: `src/procedure/algo/pagerank/result.rs`

```rust
//! PageRank Result Container

use crate::types::prelude::*;

#[derive(Debug, Clone)]
pub struct PageRankResult {
    pub scores: Vec<f64>,
    pub iterations: usize,
    pub did_converge: bool,
}

impl PageRankResult {
    pub fn new(scores: Vec<f64>, iterations: usize, did_converge: bool) -> Self {
        Self {
            scores,
            iterations,
            did_converge,
        }
    }

    pub fn score(&self, node_id: u32) -> f64 {
        self.scores.get(node_id as usize).copied().unwrap_or(0.0)
    }
}
```

### File 5: `src/procedure/algo/pagerank/standard.rs`

```rust
//! Standard PageRank Computation
//!
//! Implements the classic PageRank formula:
//! PR(u) = (1-d) + d * sum(PR(v)/out_degree(v)) for v in predecessors(u)

use crate::pregel::{
    Computation, ComputeContext, InitContext, MasterComputeContext, Messages,
};
use std::collections::HashMap;
use super::config::PageRankConfig;

pub struct StandardPageRankComputation {
    config: PageRankConfig,
    degree_function: HashMap<u32, f64>,
}

impl StandardPageRankComputation {
    pub fn new(config: PageRankConfig, degree_function: HashMap<u32, f64>) -> Self {
        Self {
            config,
            degree_function,
        }
    }
}

impl Computation for StandardPageRankComputation {
    fn init(&mut self, context: &mut InitContext) {
        let alpha = 1.0 - self.config.damping_factor;
        context.set_node_value("pagerank", alpha);
    }

    fn compute(&mut self, context: &mut ComputeContext, messages: Messages) {
        let rank = context.double_value("pagerank");
        let mut delta = rank;

        if !context.is_initial_superstep() {
            let sum: f64 = messages.sum();
            delta = self.config.damping_factor * sum;
            context.set_node_value("pagerank", rank + delta);
        }

        if delta > self.config.tolerance || context.is_initial_superstep() {
            let degree = self.degree_function
                .get(&context.node_id())
                .copied()
                .unwrap_or(0.0);

            if degree > 0.0 {
                context.send_to_neighbors(delta / degree);
            }
        } else {
            context.vote_to_halt();
        }
    }

    fn master_compute(&mut self, context: &mut MasterComputeContext) -> bool {
        // Standard PageRank doesn't need global synchronization
        context.iteration() < self.config.max_iterations
    }
}
```

### File 6: `src/procedure/algo/pagerank/spec.rs`

```rust
//! PageRankAlgorithmSpec - Main Algorithm Specification

use crate::projection::eval::procedure::{
    AlgorithmSpec, AlgorithmError, ComputationResult, ConfigError, ConsumerError,
    ExecutionContext, ExecutionMode, ProjectionHint, ValidationConfiguration,
};
use crate::types::prelude::GraphStore;
use serde_json::{json, Value as JsonValue};
use std::time::Instant;
use std::collections::HashMap;

use super::{
    config::PageRankConfig,
    result::PageRankResult,
    variants::PageRankVariant,
    standard::StandardPageRankComputation,
};

pub struct PageRankAlgorithmSpec {
    graph_name: String,
    variant: PageRankVariant,
    config: PageRankConfig,
}

impl PageRankAlgorithmSpec {
    pub fn new(
        graph_name: String,
        variant: PageRankVariant,
        config: PageRankConfig,
    ) -> Self {
        Self {
            graph_name,
            variant,
            config,
        }
    }
}

impl AlgorithmSpec for PageRankAlgorithmSpec {
    type Output = PageRankResult;

    fn name(&self) -> &str {
        self.variant.name()
    }

    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn projection_hint(&self) -> ProjectionHint {
        // PageRank sends messages to all neighbors
        ProjectionHint::VertexCentric
    }

    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        let _config = PageRankConfig::parse(input)?;
        Ok(input.clone())
    }

    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }

    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        let timer = Instant::now();

        // Parse config
        let config = PageRankConfig::parse(config)
            .map_err(|e| AlgorithmError::Execution(format!("{:?}", e)))?;

        context.log(
            crate::projection::eval::procedure::LogLevel::Info,
            &format!("Computing {}: {} iterations, tolerance {}",
                self.name(),
                config.max_iterations,
                config.tolerance
            ),
        );

        // Compute degrees
        let mut degree_function = HashMap::new();
        for node_id in 0..graph_store.node_count() as u32 {
            let degree = graph_store.degree(node_id as usize);
            degree_function.insert(node_id, degree as f64);
        }

        // Create computation
        let mut computation = StandardPageRankComputation::new(
            config.clone(),
            degree_function,
        );

        // Run Pregel
        // TODO: Integrate with actual Pregel::Computer
        // For now, placeholder that returns dummy result
        let scores = vec![0.15; graph_store.node_count()];

        let elapsed = timer.elapsed();

        Ok(ComputationResult::new(
            PageRankResult::new(scores, 20, true),
            elapsed,
        ))
    }

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match mode {
            ExecutionMode::Stream => Ok(result.into_result()),
            ExecutionMode::Stats => Ok(result.into_result()),
            _ => Err(ConsumerError::UnsupportedMode(mode.clone())),
        }
    }
}
```

---

## Integration Points Needed

### 1. Pregel Computer Integration

Currently stubbed in `spec.rs`. Need to:

```rust
// In execute():
let pregel = Pregel::new(
    graph_store,
    &config,
    computation,
);
let pregel_result = pregel.run()?;
let scores = extract_scores(pregel_result)?;
```

### 2. Degree Calculation

```rust
// Currently: manual iteration
// Could use existing DegreeCentrality or inline

let degrees = (0..graph_store.node_count())
    .map(|n| graph_store.degree(n) as f64)
    .collect::<Vec<_>>();
```

### 3. Optional: Eigenvector Master-Compute

For Eigenvector variant, need L2-Norm scaling:

```rust
fn master_compute(&mut self, context: &mut MasterComputeContext) -> bool {
    // After each superstep, normalize by L2-norm
    // Check convergence
    // Continue or halt
}
```

---

## What We Know Works

| Component             | Status    | Confidence |
| --------------------- | --------- | ---------- |
| AlgorithmSpec trait   | ✅ Proven | 100%       |
| Pregel infrastructure | ✅ Exists | 100%       |
| Message passing       | ✅ Exists | 100%       |
| Computation trait     | ✅ Exists | 100%       |
| Context API           | ✅ Exists | 100%       |
| Reducers              | ✅ Exists | 100%       |

**Missing**:

- Integration glue (spec.rs ↔ Pregel.run())
- Tests showing end-to-end flow
- Eigenvector master-compute logic

---

## Session 10 Tasks (One Session)

### Task 1: Create module structure (15 min)

- Create all files above
- Wire up mod.rs imports
- Verify compilation

### Task 2: Implement StandardPageRankComputation (30 min)

- Parse config
- Implement init()
- Implement compute()
- Implement master_compute()

### Task 3: Wire PageRankAlgorithmSpec to Pregel (30 min)

- Find Pregel::Computer API
- Call it correctly from execute()
- Extract results

### Task 4: Write integration tests (45 min)

- Test name(), parse_config()
- Test execute() with small graph
- Test convergence
- Test different variants

### Task 5: Add ArticleRank variant (30 min)

- Copy standard.rs → article.rs
- Change: `delta / (degree + avg_degree)`
- Test

**Total**: ~3 hours for a working PageRank implementation

---

## Success Criteria

```
✅ PageRankAlgorithmSpec compiles
✅ Implements AlgorithmSpec trait completely
✅ parse_config() accepts standard parameters
✅ execute() runs Pregel and returns scores
✅ consume_result() handles Stream/Stats modes
✅ Integration test: basic PageRank convergence
✅ 10+ tests passing
✅ No warnings from clippy
✅ Documentation complete
```

---

## Next Steps

1. **Verify Pregel::Computer API**

   - Read `src/pregel/computer.rs` to find `pub fn run()`
   - Understand return type
   - Understand how to extract node values

2. **Verify degree access**

   - Check `GraphStore::degree()` API
   - Does it take node_id as u32 or usize?
   - Does it return u32 or usize?

3. **Verify execution context**

   - Check what LogLevel options exist
   - Check what logging methods are available

4. **Get ready for Session 10**
   - These files are ready to create
   - Pregel integration is 1-2 hours of focused work
   - Strong foundation to build on

---

## The Bigger Picture

```
Session 9: ✅ COMPLETE
  Sum algorithm via Executor
  10 integration tests
  Foundation proven

Session 10: READY TO START
  PageRank algorithm via Pregel
  Iterative computation proven
  Multi-superstep patterns learned

Session 11-12: TODO
  ArticleRank + Eigenvector variants
  Scaling infrastructure
  Performance optimization

Session 13+: TODO
  Pipeline infrastructure
  ML Executors
  Models + Features
  Feature engineering
```

**You're in a very strong position.** The architecture holds. PageRank will validate it under load. Then pipelines become straightforward.

Good place to pause and refresh. 🙏
