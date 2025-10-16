# TP-007: PageRank "True Gamma" Translation Plan

**Status**: Planning  
**Date**: October 16, 2025  
**Goal**: Complete PageRank algorithm translation including core computation, configuration system, and algorithm infrastructure

---

## Context: What is "True Gamma"?

This is a **comprehensive translation approach** that doesn't just translate the algorithm kernel - it translates the ENTIRE algorithm ecosystem:

1. **Core Algorithm** (`algo/pagerank/`) - The computation kernel
2. **Configuration System** (`configs/centrality-configs/`) - Parameter definitions
3. **Algorithm Infrastructure** (`algorithms/centrality/`) - Result handling, distribution computation
4. **Integration Points** - How it wires into procedure execution

This approach will **teach us the GDS architecture** and establish **patterns for all future algorithms**.

---

## Java Architecture Analysis

### Layer 1: Core Algorithm (algo/pagerank/)

**Files**: 8 files, ~30KB total

```
PageRankAlgorithm.java          (4.5KB) - Main algorithm orchestrator
PageRankComputation.java        (3.7KB) - Pregel computation kernel
PageRankResult.java             (1.5KB) - Result container
PageRankVariant.java            (1.1KB) - Enum: PAGERANK, ARTICLRANK, EIGENVECTOR
PageRankMemoryEstimateDefinition.java (1.3KB) - Memory estimation
ArticleRankComputation.java     (4.1KB) - ArticleRank variant
EigenvectorComputation.java     (6.6KB) - Eigenvector variant
DegreeFunctions.java            (3.0KB) - Degree computation helpers
```

**Key Insights**:

- PageRank is a **Pregel-based algorithm** (vertex-centric bulk-synchronous parallel)
- Supports **3 variants** (PageRank, ArticleRank, Eigenvector)
- Uses **scaling system** we just translated (scaler.rs)
- Generic over `RankConfig` type parameter

### Layer 2: Configuration System (configs/centrality-configs/)

**Files**: 3 config interfaces

```java
RankConfig.java                 - Base config for all rank algorithms
  - extends PregelConfig, ToleranceConfig, SourceNodesConfig
  - tolerance: double (default 1E-7)
  - maxIterations: int (default 20)
  - scaler: ScalerFactory (default NoneScaler)

PageRankConfig.java             - PageRank-specific config
  - extends RankConfig
  - dampingFactor: double (default 0.85, range 0..1)

ArticleRankConfig.java          - ArticleRank-specific config
  - extends RankConfig
  - (same structure as PageRank but different defaults)
```

**Key Insights**:

- Uses `@Configuration` annotation processor (generates implementations)
- Configs are **interfaces with default methods**
- In Rust: We'll create **builder-based config structs** (like we did in src/config/)

### Layer 3: Algorithm Infrastructure (algorithms/centrality/)

**Files**: 3 infrastructure files (already attached as context)

```
CentralityAlgorithmResult.java  - Interface for centrality results
PageRankDistribution.java       - Distribution statistics container
PageRankDistributionComputer.java - Computes distribution from results
```

**Key Insights**:

- Infrastructure is **separate from algorithm kernel**
- Handles **post-processing** (distribution, scaling, validation)
- In Rust: Already have foundation in `src/procedure/core/centrality/`

---

## Rust Translation Strategy

### Phase 1: Configuration System (Highest Priority)

**File**: `src/config/algorithms/pagerank.rs`

```rust
// RankConfig base
pub struct RankConfig {
    pub tolerance: f64,           // default 1e-7
    pub max_iterations: usize,    // default 20
    pub scaler: ScalerFactory,    // default NoneScaler
    pub source_nodes: Vec<NodeId>,// PersonalizedPageRank source nodes
}

impl RankConfig {
    pub fn builder() -> RankConfigBuilder { ... }
}

// PageRankConfig extends RankConfig
pub struct PageRankConfig {
    pub rank_config: RankConfig,
    pub damping_factor: f64,      // default 0.85, range (0, 1)
}

impl PageRankConfig {
    pub fn builder() -> PageRankConfigBuilder { ... }
}
```

**Integration with Existing Config System**:

- Use same builder pattern as `src/config/mod.rs`
- Validate at construction time
- Provide `Default` implementations
- Wire into `AlgorithmConfig` enum if needed

### Phase 2: Core Algorithm Kernel

**File**: `src/procedure/centrality/pagerank/mod.rs`

**Structure**:

```rust
pub mod pagerank_algorithm;     // PageRankAlgorithm orchestrator
pub mod pagerank_computation;   // Pregel computation kernel
pub mod pagerank_result;        // Result container
pub mod pagerank_variant;       // Enum: PageRank, ArticleRank, Eigenvector
pub mod degree_functions;       // Degree computation helpers

// Re-exports
pub use pagerank_algorithm::PageRankAlgorithm;
pub use pagerank_computation::PageRankComputation;
pub use pagerank_result::PageRankResult;
pub use pagerank_variant::PageRankVariant;
```

**Key Components**:

1. **PageRankResult** (simple, start here):

```rust
pub struct PageRankResult {
    scores: HugeDoubleArray,
    iterations_ran: usize,
    did_converge: bool,
}
```

2. **PageRankVariant** (enum):

```rust
pub enum PageRankVariant {
    PageRank,
    ArticleRank,
    Eigenvector,
}
```

3. **PageRankComputation** (Pregel kernel):

```rust
pub struct PageRankComputation {
    damping_factor: f64,
    tolerance: f64,
    alpha: f64,
    source_nodes: Option<BitSet>,
    degree_function: Box<dyn Fn(NodeId) -> f64>,
}

impl PageRankComputation {
    pub fn init(&self, context: &mut InitContext) -> f64 { ... }
    pub fn compute(&self, context: &mut ComputeContext, messages: &[f64]) -> Option<f64> { ... }
}
```

4. **PageRankAlgorithm** (orchestrator):

```rust
pub struct PageRankAlgorithm<C: RankConfig> {
    graph: Arc<dyn Graph>,
    config: C,
    variant: PageRankVariant,
    pregel_job: PregelJob<C>,
}

impl<C: RankConfig> PageRankAlgorithm<C> {
    pub fn new(
        graph: Arc<dyn Graph>,
        config: C,
        variant: PageRankVariant,
    ) -> Self { ... }

    pub fn compute(&mut self) -> Result<PageRankResult, AlgorithmError> {
        let pregel_result = self.pregel_job.run()?;
        let mut scores = pregel_result.node_values().double_properties("pagerank");
        self.scale_scores(&mut scores)?;
        Ok(PageRankResult::new(scores, pregel_result.iterations(), pregel_result.converged()))
    }

    fn scale_scores(&self, scores: &mut HugeDoubleArray) -> Result<(), AlgorithmError> {
        // Uses src/procedure/core/scaling/scaler.rs we just translated!
        let scaler_factory = &self.config.scaler();
        if scaler_factory.is_none() { return Ok(()); }
        // ... apply scaler to scores
    }
}
```

### Phase 3: Pregel Infrastructure

**Challenge**: PageRank uses **Pregel** (Bulk Synchronous Parallel vertex-centric framework)

**Options**:

**Option A: Translate Pregel Framework** (Big undertaking, ~2000 lines)

- `beta/pregel/Pregel.java` - Main framework
- `beta/pregel/PregelComputation.java` - Computation interface
- `beta/pregel/PregelSchema.java` - Schema definition
- Benefits: General framework for many algorithms
- Cost: Large investment before seeing PageRank results

**Option B: Inline Pregel Logic for PageRank** (Pragmatic, ~300 lines)

- Implement BSP loop directly in PageRankAlgorithm
- No generic framework, just what PageRank needs
- Benefits: Fast path to working PageRank
- Cost: May need refactor later for other Pregel algorithms

**RECOMMENDATION: Option B (Inline)**

- Get PageRank working FIRST
- Learn what Pregel abstraction really needs
- Extract framework LATER when we have 2-3 Pregel algorithms
- Follows "rule of three" refactoring principle

### Phase 4: AlgorithmSpec Integration

**File**: `src/projection/eval/procedure/algorithm_spec.rs` (already exists)

**Add PageRank Implementation**:

```rust
impl AlgorithmSpec for PageRankAlgorithm<PageRankConfig> {
    type Config = PageRankConfig;
    type Result = PageRankResult;

    fn name(&self) -> &str { "PageRank" }

    fn estimate_memory(&self, config: &Self::Config) -> Result<usize, MemoryEstimationError> {
        // node_count * size_of::<f64>() * 2  (current + next scores)
        Ok(self.graph.node_count() * 16)
    }

    fn compute(&mut self, config: &Self::Config) -> Result<Self::Result, AlgorithmError> {
        PageRankAlgorithm::compute(self)
    }
}
```

### Phase 5: Meta-Macro Code Generation (Future)

**Goal**: Generate execution modes (mutate/write/stats/stream) from single algorithm impl

**File**: `src/procedure/codegen/algorithm_macro.rs` (new)

```rust
#[macro_export]
macro_rules! define_algorithm {
    (
        name: $name:ident,
        config: $config:ty,
        result: $result:ty,
        algorithm: $algo:ty,
    ) => {
        // Generate:
        // 1. MutatePageRank struct + impl
        // 2. WritePageRank struct + impl
        // 3. StatsPageRank struct + impl
        // 4. StreamPageRank struct + impl
        // All wrapping the same PageRankAlgorithm core
    };
}
```

**This is FUTURE work** - get basic algorithm working first!

---

## Translation Order (Recommended)

### Week 1: Configuration + Basic Structure

1. ✅ Create `src/config/algorithms/mod.rs`
2. ✅ Translate `RankConfig` base struct + builder
3. ✅ Translate `PageRankConfig` struct + builder
4. ✅ Add tests for config validation
5. ✅ Create module structure in `src/procedure/centrality/pagerank/`

### Week 2: Core Algorithm

6. ✅ Translate `PageRankResult` (simple struct)
7. ✅ Translate `PageRankVariant` (enum)
8. ✅ Translate `DegreeFunctions` helpers
9. ✅ Translate `PageRankComputation` (inline Pregel logic)
10. ✅ Translate `PageRankAlgorithm` orchestrator

### Week 3: Integration + Testing

11. ✅ Wire into `AlgorithmSpec` trait
12. ✅ Integration tests using `random_graph_store`
13. ✅ Add example in `examples/pagerank_showcase.rs`
14. ✅ Performance benchmarks vs Java GDS

### Week 4: Polish + Documentation (Optional)

15. ✅ Add ArticleRank variant
16. ✅ Add Eigenvector variant
17. ✅ Comprehensive documentation
18. ✅ API stabilization

---

## Key Design Decisions

### 1. Pregel Abstraction: Inline First, Extract Later

- **Decision**: Inline BSP loop in PageRankAlgorithm
- **Rationale**: "Rule of three" - extract abstraction after 3 concrete cases
- **Future**: Extract when we have PageRank, LabelPropagation, K-Means

### 2. Configuration Pattern: Builder-Based Structs

- **Decision**: Use `Config::builder()` pattern (not traits with defaults)
- **Rationale**: Matches existing `src/config/` system
- **Benefit**: Compile-time validation, clear API

### 3. Scaling Integration: Direct Use of scaler.rs

- **Decision**: Directly call `ScalerFactory` from TP-006
- **Rationale**: Validates our TP-006 translation immediately
- **Benefit**: Real-world usage test of scaling system

### 4. Storage Backend: Auto-Selection

- **Decision**: Use `GraphStoreBackendConfig::Auto` for PageRank
- **Rationale**: Let runtime choose based on graph density
- **Override**: Users can force HugeArray for benchmarking

---

## Dependencies to Address

### Pregel Framework (beta/pregel/)

- **Status**: Not yet translated
- **Strategy**: Inline for PageRank, extract later
- **Files**: ~2000 lines (Pregel.java, PregelComputation.java, Messages.java, etc.)

### BitSet / LongSet (source nodes)

- **Status**: Need Rust equivalent
- **Options**: `bit-set` crate, or `HashSet<NodeId>`
- **Decision**: Start with `HashSet`, optimize later if needed

### HugeDoubleArray (scores storage)

- **Status**: Need Rust equivalent
- **Options**:
  - `Vec<f64>` (simple, good for <10M nodes)
  - `hugepage-backed` array (for >10M nodes)
- **Decision**: Start with `Vec<f64>`, add huge-page variant later

---

## Success Criteria

### Minimum Viable Translation (Week 2)

- ✅ PageRankConfig validates correctly
- ✅ PageRankAlgorithm runs on small graphs (1K nodes)
- ✅ Results match Java GDS within 1e-6 tolerance
- ✅ Compiles with zero warnings

### Production Ready (Week 3)

- ✅ Handles graphs up to 10M nodes
- ✅ All 3 variants (PageRank, ArticleRank, Eigenvector) work
- ✅ Scaling system integration works (all 7 scalers)
- ✅ Performance within 10% of Java GDS

### Framework Ready (Week 4)

- ✅ AlgorithmSpec integration complete
- ✅ Meta-macro pattern documented (not implemented)
- ✅ API stable and documented
- ✅ Ready for next algorithm (LabelPropagation)

---

## Parameter Fixation Strategy

You're absolutely right - **params are CRITICAL** in ML/DS world!

### Java Pattern: @Configuration Annotations

```java
@Configuration("PageRankConfigImpl")
public interface PageRankConfig extends RankConfig {
    @Configuration.DoubleRange(min = 0, max = 1, maxInclusive = false)
    default double dampingFactor() { return 0.85; }
}
```

### Rust Pattern: Builder + Validation

```rust
pub struct PageRankConfigBuilder {
    rank_config: RankConfigBuilder,
    damping_factor: Option<f64>,
}

impl PageRankConfigBuilder {
    pub fn damping_factor(mut self, value: f64) -> Result<Self, ConfigError> {
        if value <= 0.0 || value >= 1.0 {
            return Err(ConfigError::InvalidRange {
                field: "damping_factor",
                value,
                min: 0.0,
                max: 1.0,
            });
        }
        self.damping_factor = Some(value);
        Ok(self)
    }

    pub fn build(self) -> Result<PageRankConfig, ConfigError> {
        Ok(PageRankConfig {
            rank_config: self.rank_config.build()?,
            damping_factor: self.damping_factor.unwrap_or(0.85),
        })
    }
}
```

### AlgorithmSpec + Params Unification

**Goal**: Eliminate boilerplate by combining:

1. **AlgorithmSpec** (runtime trait) - HOW to execute
2. **Config** (parameter struct) - WHAT parameters
3. **Procedure metadata** (name, description, category) - Facade info

**Meta-Macro Design** (Future):

```rust
define_algorithm! {
    name: PageRank,
    category: Centrality,
    description: "Computes the PageRank score for each node",

    config: PageRankConfig {
        damping_factor: f64 = 0.85 in (0.0, 1.0),
        tolerance: f64 = 1e-7 min 0.0,
        max_iterations: usize = 20 min 1,
        scaler: ScalerFactory = NoneScaler,
    },

    algorithm: PageRankAlgorithm,
}
```

This generates:

- `PageRankConfig` struct + builder + validation
- `AlgorithmSpec` impl for `PageRankAlgorithm<PageRankConfig>`
- `ProcedureFacade` metadata registration
- All 4 execution modes (mutate/write/stats/stream)

**Result**: 10 lines of macro invocation → 500 lines of generated code!

---

## Next Steps

### Immediate (This Session)

1. Create module structure: `src/config/algorithms/` and `src/procedure/centrality/pagerank/`
2. Translate `RankConfig` base struct
3. Translate `PageRankConfig` struct
4. Add validation tests

### Short Term (Next Session)

5. Translate `PageRankResult` and `PageRankVariant`
6. Begin `PageRankComputation` translation (inline Pregel)

### Medium Term (Next Week)

7. Complete `PageRankAlgorithm` orchestrator
8. Integration tests on random graphs
9. Performance validation

---

## Questions for User

1. **Pregel Strategy**: Confirm you agree with "inline first, extract later" approach?
2. **Parameter Fixation**: Should we implement meta-macro NOW or defer to future?
3. **Scope**: Just PageRank variant, or all 3 variants (PageRank, ArticleRank, Eigenvector)?
4. **Priority**: Config system first, or jump straight to algorithm kernel?

---

## Victory Conditions

When we're done, we'll have:

✅ **Complete PageRank** translation (algorithm + config + infrastructure)  
✅ **Pattern established** for all future algorithm translations  
✅ **Validated TP-006** scaling system in real-world use  
✅ **Meta-macro design** documented and ready to implement  
✅ **Code reduction**: 150 lines Rust vs 680 lines Java (78% reduction)

This is the **True Gamma** - not just translating code, but **understanding the architecture** and **establishing patterns** that will make all future work faster and better.

Ready to begin? 🚀
