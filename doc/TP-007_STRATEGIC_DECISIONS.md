# TP-007: Strategic Decisions Document

**Date**: October 16, 2025  
**Status**: Awaiting User Confirmation  
**Phase**: Planning → Execution

---

## Executive Summary

User wants to do a **"True Gamma"** translation of PageRank - not just the algorithm kernel, but the ENTIRE ecosystem:

1. **Core Algorithm** (algo/pagerank/) - The computation
2. **Configuration System** (configs/) - The parameters
3. **Algorithm Infrastructure** (algorithms/) - The result handling

This comprehensive approach will **teach us the GDS architecture** and establish **patterns for all future algorithms**.

---

## Key Strategic Questions

### 1. Pregel Framework Approach

**Question**: Translate full Pregel framework (~2000 lines) or inline BSP logic in PageRank?

**Option A: Full Framework Translation**

- Files: beta/pregel/\*.java (~2000 lines)
- Timeline: 1-2 weeks before seeing PageRank results
- Benefit: Generic framework ready for all Pregel algorithms
- Risk: Over-engineering before understanding real needs

**Option B: Inline Pregel Logic** ✅ RECOMMENDED

- Implementation: BSP loop directly in PageRankAlgorithm (~300 lines)
- Timeline: See PageRank working in 2-3 days
- Benefit: Fast feedback, learn what abstraction really needs
- Strategy: Extract framework AFTER we have 2-3 Pregel algorithms ("Rule of Three")

**RECOMMENDATION**: **Option B - Inline First**

**Rationale**:

- Follows "rule of three" refactoring principle
- Get working algorithm FAST
- Learn Pregel requirements through actual use
- Extract abstraction when we have 3 concrete cases
- Avoids premature optimization

**USER DECISION NEEDED**: ✅ Confirm inline approach or request full framework?

---

### 2. Implementation Order

**Question**: Start with configuration system or jump to algorithm kernel?

**Option A: Config-First Approach** ✅ RECOMMENDED

- Phase 1: Build RankConfig + PageRankConfig (~200 lines, 2-4 hours)
- Phase 2: Build algorithm kernel that uses config
- Benefit: Type-safe parameters from day 1
- Benefit: Validates our config system design early

**Option B: Algorithm-First Approach**

- Phase 1: Build PageRankAlgorithm with hardcoded params
- Phase 2: Retrofit configuration system
- Risk: May discover config needs that require algorithm changes

**RECOMMENDATION**: **Option A - Config-First**

**Rationale**:

- Configuration is foundational
- Easier to build algorithm with config than retrofit config into algorithm
- Validates our src/config/ system design
- Matches our existing pattern (see ML pipeline configs)

**USER DECISION NEEDED**: ✅ Confirm config-first or prefer algorithm-first?

---

### 3. Scope of Translation

**Question**: Translate just PageRank variant, or all 3 variants (PageRank, ArticleRank, Eigenvector)?

**Option A: PageRank Only** ✅ RECOMMENDED (MVP)

- Scope: PageRankComputation + PageRankConfig only
- Timeline: 2-3 days to working algorithm
- Benefit: Fastest path to validation
- Strategy: Add variants AFTER core is proven

**Option B: All Three Variants**

- Scope: PageRank + ArticleRank + Eigenvector
- Timeline: 5-7 days to all variants working
- Benefit: Complete PageRank family immediately
- Risk: More complexity before validation

**RECOMMENDATION**: **Option A - PageRank Only (MVP)**

**Rationale**:

- Validate core architecture first
- ArticleRank and Eigenvector are small deltas from PageRank
- Can add variants in 1-2 hours each after core is proven
- Follows "make it work, make it right, make it fast" principle

**USER DECISION NEEDED**: ✅ Start with PageRank only, or all 3 variants?

---

### 4. Meta-Macro Timing

**Question**: Implement meta-macro code generation NOW or defer to future?

**Current Situation**:

- We have identified the pattern (AlgorithmSpec + Config + Facade unification)
- We have documented the design (see TP-007_ARCHITECTURE_DISCOVERY.md)
- Implementing macro = ~400 lines + learning curve

**Option A: Implement Meta-Macro NOW**

- Timeline: 1-2 weeks for macro implementation
- Benefit: All future algorithms get 78% code reduction immediately
- Risk: Complex macro work BEFORE we have 1 working algorithm

**Option B: Defer Meta-Macro to Future** ✅ RECOMMENDED

- Timeline: Complete PageRank manually first
- Then extract pattern into macro when we have 2-3 algorithms
- Benefit: Know EXACTLY what the macro needs to generate
- Benefit: Avoid premature abstraction

**RECOMMENDATION**: **Option B - Defer to Future**

**Rationale**:

- "Make it work, make it right, make it fast"
- Get 1-2 algorithms working manually
- THEN extract the repetitive patterns into macro
- By then, we'll know exactly what we need
- Macros are hard to debug - get the pattern right first

**USER DECISION NEEDED**: ✅ Manual first, or macro-driven from start?

---

## Recommended Execution Plan

### Phase 1: Configuration System (2-4 hours)

**Goal**: Type-safe parameter handling

1. Create `src/config/algorithms/mod.rs` (module interface)
2. Create `src/config/algorithms/rank_config.rs`:

   - RankConfig struct
   - RankConfigBuilder
   - Validation (tolerance > 0, max_iterations > 0)
   - Default implementation
   - Tests

3. Create `src/config/algorithms/pagerank_config.rs`:
   - PageRankConfig struct (extends RankConfig)
   - PageRankConfigBuilder
   - Validation (damping_factor in (0, 1))
   - Default implementation
   - Tests

**Success Criteria**:

```rust
let config = PageRankConfig::builder()
    .damping_factor(0.85)?
    .tolerance(1e-7)?
    .max_iterations(20)?
    .build()?;

assert_eq!(config.damping_factor, 0.85);
```

### Phase 2: Core Algorithm Structures (2-3 hours)

**Goal**: Data structures and simple logic

1. Create `src/procedure/centrality/pagerank/mod.rs` (module interface)
2. Create `pagerank_result.rs`:

   - PageRankResult struct (scores, iterations, converged)
   - Simple getters
   - Tests

3. Create `pagerank_variant.rs`:

   - PageRankVariant enum
   - Tests

4. Create `degree_functions.rs`:
   - Degree computation helpers
   - Tests

**Success Criteria**:

```rust
let result = PageRankResult::new(scores, 10, true);
assert_eq!(result.iterations_ran(), 10);
assert!(result.did_converge());
```

### Phase 3: Computation Kernel (1 day)

**Goal**: Working PageRank algorithm (inline Pregel BSP)

1. Create `pagerank_computation.rs`:
   - PageRankComputation struct
   - init() - Initialize node scores
   - compute() - Single BSP superstep
   - Message aggregation logic
   - Convergence check
   - Tests

**Success Criteria**:

```rust
let computation = PageRankComputation::new(config, source_nodes, degree_fn);
let mut scores = vec![0.0; node_count];
computation.init(&mut scores);

// Run supersteps
loop {
    let converged = computation.compute(&graph, &mut scores)?;
    if converged { break; }
}
```

### Phase 4: Algorithm Orchestrator (1 day)

**Goal**: Complete PageRankAlgorithm with scaling

1. Create `pagerank_algorithm.rs`:
   - PageRankAlgorithm struct
   - new() constructor
   - compute() method:
     - Run PageRankComputation BSP loop
     - Apply scaling (uses src/procedure/core/scaling/scaler.rs)
     - Construct PageRankResult
   - Tests

**Success Criteria**:

```rust
let mut algorithm = PageRankAlgorithm::new(graph, config, PageRankVariant::PageRank);
let result = algorithm.compute()?;

// Scores should sum to approximately node_count (unscaled PageRank)
let sum: f64 = result.scores().iter().sum();
assert!((sum - node_count as f64).abs() < 1e-6);
```

### Phase 5: Integration & Testing (4-6 hours)

**Goal**: AlgorithmSpec integration + comprehensive tests

1. Add AlgorithmSpec impl:

   - Implement for `PageRankAlgorithm<PageRankConfig>`
   - name(), estimate_memory(), compute()

2. Create `tests/algorithms/pagerank_test.rs`:

   - Test on random graphs (seeded)
   - Test PersonalizedPageRank (source nodes)
   - Test all 7 scalers (validates TP-006)
   - Test convergence behavior
   - Test tolerance sensitivity

3. Create `examples/pagerank_showcase.rs`:
   - Basic PageRank
   - PersonalizedPageRank
   - Different scalers
   - Result interpretation

**Success Criteria**:

```rust
// Integration test
let graph = random_graph_store(RandomGraphConfig::seeded(42));
let config = PageRankConfig::default();
let mut algo = PageRankAlgorithm::new(graph, config, PageRankVariant::PageRank);
let result = algo.compute()?;

// Top 3 nodes should have highest scores
let mut scores: Vec<_> = result.scores().iter().enumerate().collect();
scores.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
// Assert something about score distribution
```

---

## Timeline Estimates

### Optimistic (Assume no blockers, user highly engaged)

- **Phase 1 (Config)**: 2 hours
- **Phase 2 (Structures)**: 2 hours
- **Phase 3 (Kernel)**: 6 hours
- **Phase 4 (Orchestrator)**: 6 hours
- **Phase 5 (Integration)**: 4 hours
- **Total**: 20 hours = 2.5 days

### Realistic (Normal development pace, some iteration)

- **Phase 1 (Config)**: 4 hours
- **Phase 2 (Structures)**: 3 hours
- **Phase 3 (Kernel)**: 8 hours (debugging BSP logic)
- **Phase 4 (Orchestrator)**: 8 hours (scaling integration challenges)
- **Phase 5 (Integration)**: 6 hours (test coverage)
- **Total**: 29 hours = 3.5 days

### Conservative (Learning curve, unexpected issues)

- **Phase 1 (Config)**: 6 hours
- **Phase 2 (Structures)**: 4 hours
- **Phase 3 (Kernel)**: 12 hours (Pregel BSP learning curve)
- **Phase 4 (Orchestrator)**: 10 hours (scaling + convergence issues)
- **Phase 5 (Integration)**: 8 hours (comprehensive testing)
- **Total**: 40 hours = 5 days

**ESTIMATE**: Plan for **4 days** (realistic + buffer)

---

## Decision Matrix

| Decision                 | Recommended             | Alternative                  | User Choice            |
| ------------------------ | ----------------------- | ---------------------------- | ---------------------- |
| **Pregel Approach**      | Inline BSP (~300 lines) | Full framework (~2000 lines) | ⬜ Inline ⬜ Framework |
| **Implementation Order** | Config-first            | Algorithm-first              | ⬜ Config ⬜ Algorithm |
| **Scope**                | PageRank only           | All 3 variants               | ⬜ PageRank ⬜ All 3   |
| **Meta-Macro**           | Defer to future         | Implement now                | ⬜ Defer ⬜ Now        |

---

## Risk Assessment

### Low Risk ✅

- Configuration system (we've done this before)
- Data structures (PageRankResult, PageRankVariant - simple)
- Module organization (established pattern)

### Medium Risk ⚠️

- Pregel BSP implementation (new pattern for us)
- Message passing in Graph trait (may need extensions)
- Convergence detection (subtle floating-point issues)

### High Risk 🔴

- Performance compared to Java GDS (unknown until measured)
- Memory usage for large graphs (need profiling)
- Scaling system integration (should work, but untested at scale)

### Mitigation Strategies

- **Pregel BSP**: Study Java implementation carefully, translate literally first
- **Graph trait**: Add minimal extensions needed, defer full redesign
- **Convergence**: Use extensive tests with known-good results
- **Performance**: Accept 10% slower than Java GDS initially, optimize later
- **Memory**: Start with Vec<f64>, add huge-page variant if needed
- **Scaling**: Extensive tests with all 7 scalers on varied graphs

---

## Success Criteria Checklist

### MVP (Minimum Viable PageRank) - End of Phase 4

- ⬜ Compiles with zero warnings
- ⬜ PageRankConfig validates correctly (damping factor, tolerance, etc.)
- ⬜ Runs on small graph (1000 nodes, 5000 relationships)
- ⬜ Converges within 20 iterations
- ⬜ Results are numerically stable (no NaN/Inf)
- ⬜ Scores sum to expected value (unscaled: ≈ node_count)

### Production Ready - End of Phase 5

- ⬜ AlgorithmSpec trait implemented
- ⬜ Handles graphs up to 1M nodes
- ⬜ All 7 scalers work correctly
- ⬜ PersonalizedPageRank works (source nodes)
- ⬜ Results match Java GDS within 1e-6 tolerance
- ⬜ Comprehensive test coverage (>80%)
- ⬜ Example code in examples/pagerank_showcase.rs
- ⬜ Documentation complete

### Framework Ready - Future

- ⬜ ArticleRank variant implemented
- ⬜ Eigenvector variant implemented
- ⬜ Performance within 10% of Java GDS
- ⬜ Memory usage within 10% of Java GDS
- ⬜ Ready for second algorithm (LabelPropagation)
- ⬜ Pregel abstraction extracted (if 3+ algorithms completed)
- ⬜ Meta-macro design implemented (if deferral approved)

---

## Dependencies & Prerequisites

### Must Have (Blockers)

- ✅ Graph trait with node iteration
- ✅ Scaling system (src/procedure/core/scaling/) - DONE in TP-006
- ✅ Config builder pattern (src/config/) - DONE
- ⬜ Storage for scores (Vec<f64> acceptable for MVP)

### Should Have (Workarounds Available)

- ⬜ BitSet for source nodes (can use HashSet<NodeId>)
- ⬜ HugeDoubleArray (can use Vec<f64> for now)
- ⬜ Message passing in Graph trait (can implement ad-hoc for MVP)

### Nice to Have (Future)

- ⬜ Full Pregel framework
- ⬜ Meta-macro code generation
- ⬜ Hugepage-backed arrays
- ⬜ SIMD-optimized computations

---

## Open Questions

1. **Graph trait extensions**: Do we need formal message-passing API, or ad-hoc for PageRank?

   - **Proposal**: Ad-hoc for MVP, formalize when we have 2+ Pregel algorithms

2. **Score storage**: Vec<f64> or introduce HugeDoubleArray abstraction now?

   - **Proposal**: Vec<f64> for MVP, optimize if profiling shows issues

3. **Source nodes representation**: HashSet<NodeId> or BitSet?

   - **Proposal**: HashSet for MVP, optimize later

4. **Convergence detection**: Per-node or global average delta?

   - **Follow Java**: Global average delta with vote-to-halt pattern

5. **Testing strategy**: Unit tests vs integration tests vs property-based tests?
   - **Proposal**: Mix - unit tests for components, integration for full algorithm, golden tests vs Java GDS

---

## Commit Strategy

### Commit 1: Configuration System

```
TP-007 Phase 1: PageRank Configuration System

- Add src/config/algorithms/rank_config.rs (RankConfig base)
- Add src/config/algorithms/pagerank_config.rs (PageRankConfig)
- Builder pattern with validation
- Tests for config validation

Files: 4 new, ~250 lines
```

### Commit 2: Data Structures

```
TP-007 Phase 2: PageRank Data Structures

- Add src/procedure/centrality/pagerank/pagerank_result.rs
- Add src/procedure/centrality/pagerank/pagerank_variant.rs
- Add src/procedure/centrality/pagerank/degree_functions.rs
- Tests for all structures

Files: 4 new, ~200 lines
```

### Commit 3: Computation Kernel

```
TP-007 Phase 3: PageRank Computation Kernel

- Add src/procedure/centrality/pagerank/pagerank_computation.rs
- Inline Pregel BSP implementation
- Tests for init(), compute(), convergence

Files: 1 new, ~400 lines
```

### Commit 4: Algorithm Orchestrator

```
TP-007 Phase 4: PageRank Algorithm Orchestrator

- Add src/procedure/centrality/pagerank/pagerank_algorithm.rs
- Integrate with scaling system
- Tests for complete algorithm

Files: 1 new, ~350 lines
```

### Commit 5: Integration & Tests

```
TP-007 Phase 5: PageRank Integration & Testing

- AlgorithmSpec implementation
- Comprehensive integration tests
- Example code (pagerank_showcase.rs)
- Documentation

Files: 3 new, ~400 lines
```

**Total**: 5 commits, ~1600 lines of production code + tests

---

## User Decisions Required

Please confirm your preferences:

### 1. Pregel Approach

- ⬜ **Inline BSP logic** (recommended) - Fast path to working PageRank
- ⬜ **Full Pregel framework** - More upfront work, generic solution

### 2. Implementation Order

- ⬜ **Config-first** (recommended) - Build type-safe params first
- ⬜ **Algorithm-first** - Jump to computation kernel

### 3. Scope

- ⬜ **PageRank only** (recommended) - Fastest MVP validation
- ⬜ **All 3 variants** - Complete PageRank family immediately

### 4. Meta-Macro

- ⬜ **Defer to future** (recommended) - Get algorithm working first
- ⬜ **Implement now** - Code generation from start

### 5. Ready to Start?

- ⬜ **Yes, let's begin Phase 1** - Start with configuration system
- ⬜ **Need more discussion** - Clarify specific concerns

---

## Next Immediate Actions (Once Approved)

1. Create `src/config/algorithms/mod.rs`
2. Translate `RankConfig` base configuration
3. Translate `PageRankConfig` extended configuration
4. Write validation tests
5. Commit Phase 1

**Estimated time to first commit**: 2-4 hours

---

**Status**: ⏸️ Awaiting user confirmation on strategic decisions

**Ready to execute on user's signal! 🚀**
