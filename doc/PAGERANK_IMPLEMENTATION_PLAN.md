# PageRank Implementation Plan for rust-gds

**Status**: Analysis Complete  
**Complexity**: HIGH (but manageable with Pregel foundation)  
**Timeline**: Multi-session work  
**Location**: `src/procedure/algo/pagerank/`

---

## What Java GDS PageRank Actually Does

### 1. The Core Algorithm (PregelComputation)

```java
// PageRankComputation.java - The iterative algorithm
init(InitContext):
  - Set initial rank = (1 - dampingFactor) for all nodes
  - Or 0 if source nodes specified

compute(ComputeContext, Messages):
  - Receive messages from neighbors (summed by Reducer)
  - Update: rank = rank + dampingFactor * sumOfMessages
  - Send: rank / degree to all neighbors
  - Vote to halt when delta < tolerance

// Three variants implemented as separate classes:
PageRankComputation      (standard PR: rank / degree)
ArticleRankComputation   (PR variant: rank / (degree + avgDegree))
EigenvectorComputation   (eigenvector: master-compute scaling)
```

### 2. The Infrastructure (PageRankAlgorithm)

```java
// PageRankAlgorithm.java - The runner
compute():
  - Run PregelJob (Pregel orchestrator)
  - Extract scores from Pregel result
  - Apply scaling (L2Norm, etc.)
  - Return PageRankResult

// Dependencies:
- Pregel<C>: Iterative computation framework
- PregelComputation<C>: The algorithm logic
- HugeDoubleArray: Storage for scores
- ExecutorService: Parallelization
```

### 3. Supporting Infrastructure

```java
// DegreeFunctions.java - Helper utilities
pageRankDegreeFunction():
  - Compute degrees (weighted or unweighted)
  - Return function: nodeId → degree

averageDegree():
  - Parallel iteration over all nodes
  - Calculate average degree for normalization

// PageRankResult.java - Output container
record PageRankResult(
  HugeDoubleArray scores,
  int iterations,
  boolean didConverge
)
```

---

## What We Need to Build (Rust)

### TIER 1: Essential (Pregel Foundation Assumed)

```
src/procedure/algo/pagerank/
├── mod.rs                           (module hub)
├── spec.rs                          (PageRankAlgorithmSpec - implements AlgorithmSpec)
├── variants.rs                      (PageRankVariant enum - STANDARD, ARTICLE, EIGENVECTOR)
├── computation.rs                   (PregelComputation implementation)
├── result.rs                        (PageRankResult container)
└── utilities/
    ├── degree_functions.rs          (degree calculation helpers)
    └── scaling.rs                   (L2Norm, None scalers)
```

### TIER 2: Configuration

```rust
// PageRankConfig trait requirements (from Java)
pub struct PageRankConfig {
    pub max_iterations: usize,           // max supersteps
    pub damping_factor: f64,             // (1-alpha), typically 0.85
    pub tolerance: f64,                  // convergence threshold
    pub relationship_weight_property: Option<String>,
    pub source_nodes: Option<Vec<u32>>,  // for personalized PR
    pub scaler: ScalerType,              // L2NORM, NONE, etc.
}
```

### TIER 3: Pregel Integration Points

```rust
// What we need from Pregel infrastructure:
pub trait PregelComputation<C: Config> {
    // Schema definition
    fn schema(&self, config: &C) -> PregelSchema;

    // Initialization (superstep 0)
    fn init(&self, context: &mut InitContext<C>);

    // Main computation loop
    fn compute(&self, context: &mut ComputeContext<C>, messages: Messages);

    // Optional: master-compute (synchronization point)
    fn master_compute(&self, context: &mut MasterComputeContext<C>) -> bool;

    // Message reduction (how to combine messages)
    fn reducer(&self) -> Option<Reducer>;

    // Relationship weight handling
    fn apply_relationship_weight(&self, value: f64, weight: f64) -> f64;
}

// What Pregel orchestrator provides:
pub struct Pregel<C: Config> {
    pub fn run(&mut self) -> PregelResult;
}

pub struct PregelResult {
    pub node_values: NodeValues,        // Final state of all nodes
    pub iterations: usize,
    pub did_converge: bool,
}
```

---

## Implementation Strategy

### Phase 1: Structure (Session 10?)

```rust
// src/procedure/algo/pagerank/spec.rs
pub struct PageRankAlgorithmSpec {
    graph_name: String,
    config: PageRankConfig,
    variant: PageRankVariant,  // STANDARD, ARTICLE, EIGENVECTOR
}

impl AlgorithmSpec for PageRankAlgorithmSpec {
    type Output = PageRankResult;

    fn name(&self) -> &str { "pagerank" }
    fn execute<G: GraphStore>(...) -> Result<ComputationResult<Self::Output>, _> {
        // 1. Create appropriate PregelComputation variant
        let computation = match self.variant {
            PageRankVariant::Standard => StandardPageRankComputation::new(...),
            PageRankVariant::ArticleRank => ArticleRankComputation::new(...),
            PageRankVariant::Eigenvector => EigenvectorComputation::new(...),
        };

        // 2. Run Pregel orchestrator
        let pregel = Pregel::new(graph_store, &self.config, computation);
        let result = pregel.run()?;

        // 3. Apply scaling
        let scores = self.scale_scores(result.node_values)?;

        // 4. Return result
        Ok(ComputationResult::new(
            PageRankResult { scores, iterations: result.iterations, did_converge: result.did_converge },
            elapsed,
        ))
    }
}
```

### Phase 2: Algorithm Variants

```rust
// src/procedure/algo/pagerank/computation.rs
pub struct StandardPageRankComputation { ... }
impl PregelComputation for StandardPageRankComputation {
    fn init(&self, ctx: &mut InitContext) {
        ctx.set_node_value("pagerank", 1.0 - self.config.damping_factor);
    }

    fn compute(&self, ctx: &mut ComputeContext, messages: Messages) {
        let rank = ctx.double_value("pagerank");
        let sum: f64 = messages.iter().sum();
        let new_rank = rank + self.config.damping_factor * sum;
        ctx.set_node_value("pagerank", new_rank);

        let degree = self.degree_function(ctx.node_id());
        if degree > 0.0 {
            ctx.send_to_neighbors(new_rank / degree);
        }
    }
}

pub struct ArticleRankComputation { ... }
// Same as above, but: delta / (degree + avg_degree) instead of delta / degree

pub struct EigenvectorComputation { ... }
// More complex: L2-norm normalization in master_compute()
```

### Phase 3: Supporting Infrastructure

```rust
// src/procedure/algo/pagerank/utilities/degree_functions.rs
pub fn compute_degrees<G: GraphStore>(
    graph: &G,
    has_weights: bool,
) -> Vec<f64> {
    // Call DegreeCentrality internally OR compute directly
    (0..graph.node_count())
        .map(|node_id| {
            if has_weights {
                graph.degree_with_weights(node_id)
            } else {
                graph.degree(node_id) as f64
            }
        })
        .collect()
}

pub fn average_degree<G: GraphStore>(graph: &G) -> f64 {
    let total: f64 = (0..graph.node_count())
        .map(|n| graph.degree(n) as f64)
        .sum();
    total / graph.node_count() as f64
}

// src/procedure/algo/pagerank/utilities/scaling.rs
pub fn scale_scores_l2norm(scores: &mut [f64]) {
    let sum_sq: f64 = scores.iter().map(|s| s * s).sum();
    let norm = sum_sq.sqrt();
    if norm > 0.0 {
        for score in scores.iter_mut() {
            *score /= norm;
        }
    }
}
```

---

## Complexity Breakdown

### What's Simple

```
✓ parse_config() - similar to Sum
✓ validation_config() - similar to Sum
✓ projection_hint() - return appropriate hint
✓ consume_result() - return scores
```

### What's New

```
? PregelComputation trait implementation (need Pregel infra)
  - init() phase
  - compute() phase with message passing
  - master_compute() for synchronization (Eigenvector only)
  - reducer() to sum incoming messages

? Message passing abstraction
  - How do we represent Messages?
  - How do we send to neighbors?
  - How do we accumulate?

? Iterative convergence
  - Loop until convergence or max iterations
  - Track delta changes
  - Implement voting to halt

? Scaling infrastructure
  - L2-Norm normalization
  - Parallel application of scaling
```

---

## Questions to Answer Before Implementation

### 1. Do we have Pregel infrastructure?

Current state:

```
src/projection/eval/procedure/
├── algorithm_spec.rs       ← Generic trait (works for ANY algorithm)
├── executor.rs             ← Generic orchestrator
└── ...
```

**Need to check**: Does `src/projection/` have Pregel? Or do we build it?

### 2. How do we represent Message Passing?

Java uses:

```java
Messages messages;  // Iterable<Double>
context.sendToNeighbors(value);
```

Rust options:

```rust
// Option A: Closure-based
context.send_to_neighbors(value);
context.for_each_message(|msg| { ... });

// Option B: Iterator-based
let messages = context.messages();
for msg in messages { ... }

// Option C: Reducer pattern
let sum = context.sum_messages();
```

### 3. How do we handle synchronization?

PageRank needs:

- Barrier between supersteps
- All nodes must complete compute() before moving to next superstep
- Optional master_compute() for global decisions (Eigenvector)

**Question**: Do we have ExecutorService equivalent for parallel work?

### 4. Storage vs. Computation Poles

```
Storage Pole (Gross):
  - HugeDoubleArray for scores
  - One array per variant (or reuse?)

Computation Pole (Subtle):
  - Message accumulation
  - Delta tracking
  - Convergence checking
```

---

## Comparison: Sum vs. PageRank

| Aspect          | Sum                    | PageRank                      |
| --------------- | ---------------------- | ----------------------------- |
| Iterations      | 1                      | 1-100+                        |
| Message passing | None                   | Heavy (every superstep)       |
| Convergence     | Implicit (1 pass)      | Explicit (tolerance checking) |
| Synchronization | Single execute()       | Barrier between supersteps    |
| Storage         | Simple f64 accumulator | HugeDoubleArray of scores     |
| Computation     | Trivial (add)          | Complex (rank + damping)      |
| Lines of code   | ~90                    | ~300-400                      |
| Test complexity | Low                    | High (need convergence tests) |

---

## Realistic Timeline

### IF Pregel infrastructure exists and works:

```
Session 10a: PageRankAlgorithmSpec + StandardPageRankComputation
             - Basic parsing, config, variant selection
             - Implement for STANDARD variant only
             - Integration test: basic convergence
             ~ 2-3 hours

Session 10b: ArticleRank + Eigenvector variants
             - ArticleRankComputation (similar to Standard)
             - EigenvectorComputation (with master_compute)
             - Tests for each variant
             ~ 2-3 hours

Session 10c: Utilities + Scaling
             - Degree functions
             - Scaling infrastructure
             - Performance optimization
             ~ 1-2 hours

Total: 1 session (broken into 3 parts)
```

### IF we need to build Pregel infrastructure:

```
Phase 1: Pregel trait + basic orchestrator
         - PregelComputation trait
         - Message passing abstraction
         - Superstep iteration
         ~ 3-4 hours

Phase 2: Parallel execution
         - ThreadPool integration
         - Barrier synchronization
         - Result aggregation
         ~ 2-3 hours

Phase 3: PageRank implementation
         (as above)
         ~ 3-5 hours

Total: 2-3 sessions
```

---

## My Recommendation

**STOP HERE and assess**:

1. **Check if Pregel exists**

   ```bash
   cd /home/pat/VSCode/rust-gds
   find src -name "*pregel*" -o -name "*vertex*" -o -name "*message*" | head -20
   ```

2. **If YES**: Jump straight to PageRank implementation (Session 10)

   - 1 session to do StandardPageRank
   - Next session to add variants

3. **If NO**: We need to discuss whether to:
   - A) Build minimal Pregel infrastructure (3-4 hour effort)
   - B) Implement PageRank without Pregel (less clean, but works)
   - C) Skip PageRank, do simpler iterative algorithm (e.g., Label Propagation)

---

## The Bigger Picture (What You Noted)

You're absolutely right:

```
Phase 1: COMPLETE ✓
  Executor infrastructure + AlgorithmSpec contract
  Sum algorithm proof of concept
  Generic orchestration

Phase 2: IN PROGRESS
  Iterative algorithms (PageRank via Pregel)
  Message passing patterns
  Convergence handling

Phase 3: TODO (Major work)
  Pipeline infrastructure
  ML Executors
  Models + Features
  Parameter optimization
  Training/Inference separation
```

**PageRank is the bridge between Phase 1 and Phase 3.** It teaches us:

- Iterative computation patterns
- Pregel/BSP message passing
- Multi-step algorithms
- Convergence logic

Then **Pipelines** can compose PageRank + other algos + ML models.

---

## Next Action

**Recommendation**: Take a breath, do the assessment, and ping me when ready. The foundation is solid enough that PageRank should be straightforward if Pregel exists, or we can plan accordingly if it doesn't.

This was indeed a **major win**. You've proven the executor system works. PageRank will prove iterative algorithms work. Then we're in position for ML pipelines.

🙏
