# TP-006: Procedure Core Translation Plan

**Status**: Phase 1 COMPLETE ✅  
**Date**: October 16, 2025  
**Translation Level**: Gamma (Best-effort architectural translation)

## Overview

Translation of Java GDS `algo-common` package to `rust-gds/src/procedure/core/`.

**Source**: `/home/pat/GitHub/graph-data-science/algo-common/`  
**Target**: `src/procedure/core/`  
**Files**: ~30 Java files → ~15-20 Rust modules  
**Lines**: ~2000-2500 Java → ~1500-2000 Rust (estimated)

---

## Architectural Insight

The `procedure/` module mirrors the `ml/` module structure:

```
ml/                          procedure/
├── core/                    ├── core/              ← TP-006 (THIS TRANSLATION)
│   ├── abstract_variable    │   ├── result/       (statistics)
│   ├── computation_context  │   ├── scaling/      (feature scaling)
│   ├── features/            │   └── prelude.rs
│   └── ...                  │
└── algo/                    └── algo/              (future)
    └── decision_tree/           ├── centrality/
                                 ├── community/
                                 └── ...
```

**Philosophical Note**: Procedures are to GDSL what procedures are to Pascal -
fundamental building blocks, not microservices! The GDSL provides "big Macro Services"
for graph computation.

---

## Phase 1: Module Skeleton ✅ COMPLETE

**Created**:

- `src/procedure/mod.rs` (updated with core module)
- `src/procedure/core/mod.rs` (main module with architecture doc)
- `src/procedure/core/result/mod.rs` (result processing skeleton)
- `src/procedure/core/scaling/mod.rs` (feature scaling skeleton)
- `src/procedure/core/prelude.rs` (common re-exports)

**Build Status**: ✅ Clean compilation

---

## What We're NOT Translating

The following Java GDS components are **intentionally skipped**:

1. ❌ **Algorithm.java** (base algorithm class)

   - **Why**: Use `AlgorithmSpec` trait from executor instead
   - **Rust equivalent**: `projection::eval::procedure::AlgorithmSpec`

2. ❌ **AlgorithmFactory.java** (factory pattern with visitor)

   - **Why**: Rust uses direct construction + builder pattern

3. ❌ **GraphAlgorithmFactory.java** / **GraphStoreAlgorithmFactory.java**

   - **Why**: Unnecessary ceremony in Rust

4. ❌ **Converters.java** (Long/Int conversion utilities)

   - **Why**: Rust uses u64/usize directly, no conversions needed

5. ❌ **MemoryEstimationNotImplementedException.java**
   - **Why**: Use `AlgorithmError` from executor

**Rationale**: These are Java ceremony that Rust solves differently through traits,
direct construction, and type safety.

---

## Translation Phases

### Phase 1: Module Skeleton ✅ COMPLETE

- Module structure created
- Documentation in place
- Builds cleanly

### Phase 2: Result Processing ✅ COMPLETE

**Status**: All 3 modules translated, 19 tests passing

**Files translated**:

1. ✅ `CentralityStatistics.java` → `result/centrality.rs` (292 lines, 5 tests)
2. ✅ `CommunityStatistics.java` → `result/community.rs` (491 lines, 7 tests)
3. ✅ `SimilarityStatistics.java` → `result/similarity.rs` (289 lines, 7 tests)

**Total**: 1,072 lines of Rust code with comprehensive tests

**Key achievements**:

- Parallel histogram building (centrality, community)
- Sparse array integration (community sizes)
- Iterator abstraction (similarity)
- f64→u64 scaling for HDR histograms
- Dynamic histogram range handling
- Error recovery and bounds checking

**Documentation**:

- `doc/TP-006_PHASE_2_CENTRALITY_COMPLETE.md`
- `doc/TP-006_PHASE_2_COMMUNITY_COMPLETE.md`
- `doc/TP-006_PHASE_2_SIMILARITY_COMPLETE.md`

### Phase 3: Scaling System (NEXT - 2-3 hours)

**Priority**: HIGH - Essential for ML

**Status**: Ready to begin

**Files to translate**:

1. `Scaler.java` + `ScalarScaler.java` → `scaling/scaler.rs` (trait)
2. `ScalerFactory.java` → Factory pattern in scaler.rs
3. `MinMax.java` → `scaling/minmax.rs`
4. `StdScore.java` → `scaling/stdscore.rs`
5. `Mean.java` → `scaling/mean.rs`
6. `Max.java` → `scaling/max.rs`
7. `Center.java` → `scaling/center.rs`
8. `LogScaler.java` → `scaling/log.rs`
9. `NoneScaler.java` → `scaling/none.rs`

**Key simplifications**:

- Use trait objects for dynamic dispatch
- Leverage Rust's type system for safety
- Use `rayon` for parallel statistics computation

---

## Success Criteria

- ✅ Module structure complete (Phase 1)
- ✅ Result builders working with tests (Phase 2 - 19 tests passing)
- ⏳ All scalers working with tests (Phase 3)
- ⏳ Integration with `AlgorithmSpec` trait
- ⏳ Examples demonstrating usage
- ✅ Documentation explaining simplifications (3 Phase 2 docs)

---

## Next Steps

**Phase 2 Complete!** ✅

All result processing modules translated with 1,072 lines of Rust and 19 passing tests.

**Start Phase 3**: Scaling System Translation

1. **Analyze scaler trait hierarchy** in Java GDS
2. **Design Rust trait**: `Scaler` trait + implementations
3. **Translate scalers**: MinMax, StdScore, Mean, Max, Center, Log, None
4. **Add tests**: Each scaler needs comprehensive tests
5. **Integration**: Wire into algorithm pipeline

See Phase 3 section above for file list and dependencies.

---

## Notes

- This translation provides the **foundation** for all algorithm implementations
- Result builders compute statistics (histograms, percentiles, distributions)
- Scalers normalize features for ML pipelines
- Both are essential infrastructure used by every algorithm

**The symmetry with ml/core is intentional and beautiful!**
