# TP-007: Architecture Discovery - The "True Gamma" Insight

**Date**: October 16, 2025  
**Discovery**: GDS Algorithm Architecture is a Multi-Layer System

---

## The Big Revelation

You said: **"bite the bullet and translate that whole thing"** - and this is EXACTLY right.

PageRank isn't just an algorithm - it's an **entire ecosystem** with multiple architectural layers that work together. To truly understand GDS, we need to translate ALL the layers, not just cherry-pick the computation kernel.

---

## The Three-Layer Architecture

### Layer 1: Core Algorithm Implementation (`algo/`)

**Location**: `/home/pat/GitHub/graph-data-science/algo/src/main/java/org/neo4j/gds/`

**Structure**:

```
algo/
├── pagerank/                    # Algorithm-specific implementations
│   ├── PageRankAlgorithm.java   # Orchestrator
│   ├── PageRankComputation.java # Pregel kernel
│   ├── PageRankResult.java      # Result container
│   ├── PageRankVariant.java     # Enum: PAGERANK, ARTICLERANK, EIGENVECTOR
│   └── ...
├── louvain/                     # Another algorithm
├── labelpropagation/            # Another algorithm
└── ...                          # 40+ algorithm modules
```

**Purpose**: The "HOW" - actual computation logic

**Key Characteristics**:

- Algorithm-specific code
- Pregel/BSP frameworks
- Performance-critical inner loops
- Memory estimation
- Result construction

### Layer 2: Configuration Definitions (`algo-params/` and `configs/`)

**Location**: `/home/pat/GitHub/graph-data-science/algo-params/centrality-params/`  
**Location**: `/home/pat/GitHub/graph-data-science/procedures/facade-api/configs/centrality-configs/`

**Structure**:

```
configs/centrality-configs/
└── src/main/java/org/neo4j/gds/pagerank/
    ├── RankConfig.java          # Base config for rank algorithms
    ├── PageRankConfig.java      # PageRank-specific params
    └── ArticleRankConfig.java   # ArticleRank-specific params
```

**Purpose**: The "WHAT" - parameter definitions and validation

**Key Characteristics**:

- `@Configuration` annotation-driven
- Interface with default methods (Java)
- Validation rules (ranges, constraints)
- Immutable value objects
- Builder patterns (via annotation processor)

**Example** (Java):

```java
@Configuration("PageRankConfigImpl")
public interface PageRankConfig extends RankConfig {
    @Configuration.DoubleRange(min = 0, max = 1, maxInclusive = false)
    default double dampingFactor() { return 0.85; }
}
```

**Rust Translation** (our approach):

```rust
pub struct PageRankConfig {
    pub rank_config: RankConfig,
    pub damping_factor: f64,  // validated in builder
}

impl PageRankConfig {
    pub fn builder() -> PageRankConfigBuilder { ... }
}
```

### Layer 3: Algorithm Infrastructure (`algorithms/`)

**Location**: `/home/pat/GitHub/graph-data-science/algo/src/main/java/org/neo4j/gds/algorithms/`

**Structure**:

```
algorithms/
├── centrality/
│   ├── CentralityAlgorithmResult.java        # Result interface
│   ├── PageRankDistribution.java             # Statistics container
│   └── PageRankDistributionComputer.java     # Post-processing
├── community/
│   ├── CommunityCompanion.java               # Result helpers
│   └── ConsecutiveLongNodePropertyValues.java# Property transformations
├── similarity/
│   ├── SimilaritySummaryBuilder.java         # Statistics builder
│   └── WriteRelationshipService.java         # Write-back service
└── ...
```

**Purpose**: The "INFRASTRUCTURE" - result handling, post-processing, write-back

**Key Characteristics**:

- Result transformations
- Distribution computation
- Histogram generation
- Write-back to Neo4j
- Property value adapters

**Rust Translation**: This maps to `src/procedure/core/` we just built in TP-006!

---

## The Parameter Fixation Insight

You said: **"in a ML Data Science world there will be a Param fixation!"**

This is PROFOUNDLY correct. Let me break down why:

### The Java GDS Parameter System

**Problem**: Every algorithm has dozens of parameters

- PageRank: dampingFactor, tolerance, maxIterations, scaler, sourceNodes, ...
- Louvain: tolerance, maxLevels, maxIterations, seedProperty, includeIntermediateCommunities, ...
- NodeSimilarity: topK, topN, degreeCutoff, similarityCutoff, normalizeL2, ...

**Java Solution**: `@Configuration` Annotation Processor

```java
@Configuration("PageRankConfigImpl")
public interface PageRankConfig extends RankConfig {
    @Configuration.DoubleRange(min = 0, max = 1)
    default double dampingFactor() { return 0.85; }
}
```

This generates:

1. `PageRankConfigImpl.java` - Immutable implementation class
2. Builder pattern code
3. Validation logic
4. JSON serialization/deserialization
5. Documentation strings

**Result**: ~50 lines of interface code → ~500 lines of generated code

### The Rust Opportunity

**Instead of annotation processor, use DECLARATIVE MACROS**:

```rust
algorithm_config! {
    pub struct PageRankConfig extends RankConfig {
        #[range(0.0..1.0)]
        #[default(0.85)]
        pub damping_factor: f64,
    }
}
```

This macro generates:

1. Struct definition
2. Builder with validation
3. `Default` implementation
4. Serde serialization
5. Documentation comments

**Benefit**: Single source of truth, compile-time validation, zero runtime overhead

---

## The AlgorithmSpec Unification Strategy

### Current State (Separated Concerns)

**Runtime Execution** (`src/projection/eval/procedure/`):

```rust
pub trait AlgorithmSpec {
    type Config;
    type Result;

    fn name(&self) -> &str;
    fn estimate_memory(&self, config: &Self::Config) -> Result<usize>;
    fn compute(&mut self, config: &Self::Config) -> Result<Self::Result>;
}
```

**Metadata/Facade** (`src/ml/pipeline/`):

```rust
pub struct PipelineDescriptor {
    pub name: String,
    pub steps: Vec<PipelineStep>,
    pub training_config: TrainingConfig,
    // ...
}
```

**Configuration** (`src/config/`):

```rust
pub struct PageRankConfig {
    pub damping_factor: f64,
    // ...
}
```

### Future State (Unified via Meta-Macro)

**Single Declaration**:

```rust
define_algorithm! {
    name: PageRank,
    category: Centrality,
    description: "Computes PageRank scores via power iteration",

    config: PageRankConfig {
        damping_factor: f64 = 0.85 in (0.0, 1.0),
        tolerance: f64 = 1e-7 min 0.0,
        max_iterations: usize = 20 min 1,
    },

    algorithm: PageRankAlgorithm,

    modes: [mutate, write, stats, stream],
}
```

**This generates**:

1. `PageRankConfig` struct + builder + validation
2. `PageRankAlgorithm` wrapper
3. `AlgorithmSpec` implementation
4. `ProcedureFacade` metadata
5. All 4 execution modes:
   - `MutatePageRank` - Write results to graph store
   - `WritePageRank` - Write results to Neo4j database
   - `StatsPageRank` - Return statistics only
   - `StreamPageRank` - Stream results one by one

**Code Reduction**:

- Java: ~680 lines (algorithm + 4 procs + config + generated code)
- Rust (manual): ~450 lines (algorithm + AlgorithmSpec impl + config + builders)
- Rust (macro): ~150 lines (algorithm kernel + 10-line macro invocation)

**Reduction**: 78% fewer lines vs Java, 67% fewer lines vs manual Rust

---

## The Pregel Framework Challenge

### What is Pregel?

**Pregel** = Bulk Synchronous Parallel (BSP) vertex-centric computation framework

**Origin**: Google's Pregel paper (2010)  
**Used by**: PageRank, LabelPropagation, K-Means, SSSP, Connected Components, ...

**Key Concepts**:

1. **Vertex-centric**: Each node runs same computation
2. **Message passing**: Nodes send messages to neighbors
3. **Supersteps**: Computation proceeds in synchronized rounds
4. **Vote to halt**: Nodes can become inactive if converged

**Java GDS Implementation**: `beta/pregel/` package (~2000 lines)

### The Translation Dilemma

**Option A: Translate Full Pregel Framework** (Ambitious)

- Files: Pregel.java, PregelComputation.java, PregelSchema.java, Messages.java, Reducer.java, ...
- Lines: ~2000 lines of framework code
- Benefit: Generic framework for ALL Pregel algorithms
- Cost: Large upfront investment, no immediate algorithm results

**Option B: Inline Pregel for PageRank** (Pragmatic) ✅

- Implement BSP loop directly in PageRankAlgorithm
- No generic abstractions, just what PageRank needs
- Lines: ~300 lines
- Benefit: See PageRank working in days, not weeks
- Cost: May need refactoring later when we have more Pregel algorithms

**DECISION**: Option B - "Rule of Three"

- Inline for PageRank (1st algorithm)
- Inline for LabelPropagation (2nd algorithm)
- Extract abstraction when we have 3 Pregel algorithms
- By then, we KNOW what the abstraction should be

---

## The Module Organization Pattern

Following our established Rust module calculus:

### Configuration Module

```
src/config/algorithms/
├── mod.rs                  # Exports only (module interface)
├── rank_config.rs          # RankConfig base implementation
├── pagerank_config.rs      # PageRankConfig implementation
├── louvain_config.rs       # LouvainConfig implementation (future)
└── ...
```

### Algorithm Module

```
src/procedure/centrality/pagerank/
├── mod.rs                      # Exports only (module interface)
├── pagerank_algorithm.rs       # PageRankAlgorithm orchestrator
├── pagerank_computation.rs     # PageRankComputation kernel (inline Pregel)
├── pagerank_result.rs          # PageRankResult container
├── pagerank_variant.rs         # PageRankVariant enum
└── degree_functions.rs         # DegreeFunctions helpers
```

### Infrastructure Module (Already Built!)

```
src/procedure/core/
├── centrality/
│   └── mod.rs              # CentralityResult (TP-006)
├── community/
│   └── mod.rs              # CommunityResult (TP-006)
├── similarity/
│   └── mod.rs              # SimilarityResult (TP-006)
└── scaling/
    ├── mod.rs              # ScalerFactory interface (TP-006)
    └── scaler.rs           # 7 scaler implementations (TP-006)
```

**Insight**: TP-006 Procedure Core was building the infrastructure layer!

---

## Dependencies Identified

### 1. Storage Types

- **HugeDoubleArray**: Need Rust equivalent for score storage

  - Option A: `Vec<f64>` (simple, good for <10M nodes)
  - Option B: Hugepage-backed array (for >10M nodes)
  - **Decision**: Start with `Vec<f64>`, optimize later

- **BitSet / LongSet**: Need Rust equivalent for source nodes (PersonalizedPageRank)
  - Option A: `bit-set` crate
  - Option B: `HashSet<NodeId>`
  - **Decision**: Start with `HashSet`, optimize later

### 2. Pregel Framework

- **Status**: Not yet translated (~2000 lines)
- **Strategy**: Inline for PageRank, extract later
- **Timeline**: Future work after 2-3 Pregel algorithms

### 3. Graph Trait Extensions

- **Need**: Message passing support in Graph trait
- **Current**: Basic node/relationship iteration
- **Required**:
  - `send_message(target: NodeId, value: f64)`
  - `receive_messages(node: NodeId) -> Vec<f64>`
- **Strategy**: Add as needed for PageRank

---

## Translation Phases

### Phase 1: Configuration System ✅ Priority

**Goal**: Type-safe parameter handling  
**Files**: `src/config/algorithms/rank_config.rs`, `pagerank_config.rs`  
**Lines**: ~200 lines  
**Duration**: 2-4 hours

**Deliverable**:

```rust
let config = PageRankConfig::builder()
    .damping_factor(0.85)?
    .tolerance(1e-7)?
    .max_iterations(20)?
    .scaler(ScalerFactory::none())?
    .build()?;
```

### Phase 2: Core Algorithm ✅ Priority

**Goal**: Working PageRank computation  
**Files**: `src/procedure/centrality/pagerank/*.rs`  
**Lines**: ~600 lines  
**Duration**: 1-2 days

**Deliverable**:

```rust
let mut algorithm = PageRankAlgorithm::new(graph, config, PageRankVariant::PageRank);
let result = algorithm.compute()?;
```

### Phase 3: Integration ✅ Priority

**Goal**: AlgorithmSpec trait implementation + tests  
**Files**: `src/projection/eval/procedure/algorithm_spec.rs`, `tests/algorithms/pagerank_test.rs`  
**Lines**: ~300 lines  
**Duration**: 4-6 hours

**Deliverable**:

```rust
impl AlgorithmSpec for PageRankAlgorithm<PageRankConfig> { ... }
```

### Phase 4: Meta-Macro (Future)

**Goal**: Code generation for execution modes  
**Files**: `src/procedure/codegen/algorithm_macro.rs`  
**Lines**: ~400 lines (macro) + ~100 lines per algorithm (generated)  
**Duration**: 1-2 weeks

**Deliverable**: 78% code reduction via macro invocation

---

## Success Metrics

### Week 1 (MVP)

- ✅ PageRankConfig validates correctly
- ✅ PageRankAlgorithm runs on 1K node graphs
- ✅ Results match Java GDS within 1e-6
- ✅ Zero compiler warnings

### Week 2 (Production)

- ✅ Handles 10M node graphs
- ✅ All 3 variants work (PageRank, ArticleRank, Eigenvector)
- ✅ All 7 scalers work (validates TP-006)
- ✅ Performance within 10% of Java GDS

### Week 3 (Framework)

- ✅ AlgorithmSpec integration complete
- ✅ Ready for next algorithm (LabelPropagation)
- ✅ Meta-macro design documented
- ✅ API stable

---

## Key Insights Summary

1. **True Gamma = Comprehensive Translation**

   - Not just algorithm kernel
   - Config + Algorithm + Infrastructure
   - Learn the architecture, not just translate code

2. **Parameter Fixation is Real**

   - ML/DS = Parameter-heavy domain
   - Java uses annotation processors
   - Rust can use declarative macros
   - Single source of truth

3. **Infrastructure Already Exists**

   - TP-006 built the foundation
   - Centrality/Community/Similarity result handling
   - Scaling system (7 scalers)
   - PageRank validates it immediately

4. **Pregel: Inline First, Extract Later**

   - Don't build framework before use cases
   - Get 2-3 algorithms working with inline BSP
   - THEN extract common abstraction
   - "Rule of Three" refactoring

5. **Meta-Macro = Ultimate Goal**
   - 10 lines of declaration → 500 lines of code
   - 78% reduction vs Java GDS
   - Eliminates boilerplate
   - Single source of truth

---

## Next Actions

1. ✅ User confirms strategy (inline Pregel, config-first, PageRank only)
2. ✅ Create module structure
3. ✅ Translate RankConfig base
4. ✅ Translate PageRankConfig
5. ✅ Begin algorithm kernel translation

**This is the path to understanding GDS architecture deeply. 🚀**
