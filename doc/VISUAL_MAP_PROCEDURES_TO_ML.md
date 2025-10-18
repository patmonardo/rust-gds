# Visual Map: Procedures → Pipelines → ML

This is your north star for the entire next phase of work.

---

## The Three Phases (Next 3-4 Weeks)

```
PHASE 1: PROCEDURES (Weeks 1-2)
═════════════════════════════════════════════════════════════

  Session 9 (Complete) ✅
  ├─ Sum algorithm (single-pass)
  ├─ AlgorithmSpec trait proven
  └─ 10 integration tests passing

  Weekend (This) 📖
  ├─ Deep comprehension
  ├─ Identify speculative code
  └─ Prepare for PageRank

  Week 1 (Session 10) 🏗️
  ├─ Implement PageRank (iterative)
  ├─ Verify Computation/Storage patterns
  ├─ Integrate with Pregel
  └─ 10+ integration tests for PageRank

  Result: Procedures system proven for both:
          ✅ Single-pass (Sum)
          ✅ Iterative (PageRank)


PHASE 2: PIPELINES (Weeks 2-3)
═════════════════════════════════════════════════════════════

  Prerequisite: PageRank working ✅

  Week 2 (Session 11) 🏗️
  ├─ Create PipelineSpec trait
  ├─ Create PipelineExecutor
  ├─ Test: Pipeline of Sum + PageRank
  └─ Extend ExecutionContext for ML metadata

  Result: Can compose algorithms into pipelines
          Example: Sum → PageRank → Output


PHASE 3: ML INTEGRATION (Weeks 3-4)
═════════════════════════════════════════════════════════════

  Prerequisite: Pipelines working ✅

  Week 3 (Session 12) 🏗️
  ├─ Feature engineering framework
  ├─ Model trait and spec
  ├─ ML executor
  └─ Integration with Pipeline

  Week 4 (Sessions 13+) 🚀
  ├─ Specific model implementations
  ├─ Feature transformations
  ├─ Training/inference flows
  └─ Performance optimization

  Result: Full ML pipeline system
          Example: Features + Model + Pipelines
```

---

## Code Architecture (Current State)

```
src/
├─ types/                         ✅ Foundation
│  ├─ graph/
│  ├─ properties/
│  ├─ random_graph_store/
│  └─ prelude/
│
├─ projection/                    ✅ Mature
│  ├─ eval/
│  │  └─ procedure/               ✅ Session 9 + Now
│  │     ├─ algorithm_spec.rs     ✅ Core contract
│  │     ├─ executor.rs           ✅ Orchestrator
│  │     ├─ computation.rs        ⏳ Verify Pregel
│  │     ├─ storage.rs            ⏳ Verify Pregel
│  │     ├─ validation.rs         ⚠️  Review
│  │     ├─ context.rs            ⚠️  Review for ML
│  │     └─ ... (error types, etc)
│  └─ ...
│
├─ procedure/                     🏗️ Building
│  ├─ algo/
│  │  ├─ sum/                     ✅ Complete
│  │  │  ├─ mod.rs
│  │  │  ├─ spec.rs              ✅ AlgorithmSpec impl
│  │  │  ├─ storage.rs           ✅ StorageRuntime
│  │  │  └─ computation.rs       ✅ ComputationRuntime
│  │  └─ pagerank/               📋 Session 10 (ready to build)
│  │     ├─ mod.rs
│  │     ├─ spec.rs              📋 PageRankAlgorithmSpec
│  │     ├─ standard.rs          📋 StandardPageRankComputation
│  │     ├─ article.rs           📋 ArticleRankComputation
│  │     └─ eigenvector.rs       📋 EigenvectorComputation
│  └─ ... (other algos later)
│
├─ pregel/                        ✅ Ready to use
│  ├─ messages.rs                ✅ Message passing
│  ├─ computation.rs             ✅ Computation trait
│  ├─ computer.rs                ✅ Runner
│  ├─ executor.rs                ✅ Orchestrator
│  ├─ context/
│  │  └─ pregel_context.rs       ✅ Execution context
│  ├─ reducers.rs                ✅ Message reduction
│  ├─ schema.rs                  ✅ Value schema
│  ├─ result.rs                  ✅ Results
│  ├─ queues.rs                  ✅ Message queues
│  └─ ...
│
├─ pipeline/                      📋 Week 2 (to build)
│  ├─ mod.rs
│  ├─ spec.rs                    📋 PipelineSpec trait
│  ├─ executor.rs                📋 PipelineExecutor
│  └─ ... (result types, config, etc)
│
├─ ml/                            📋 Week 3+ (to build)
│  ├─ features/                  📋 Feature engineering
│  ├─ models/                    📋 Model definitions
│  ├─ executor/                  📋 ML executor
│  └─ ...
│
└─ ...
```

---

## The Trait Hierarchy (What You're Building)

```
┌─────────────────────────────────────────────────────────┐
│  GraphStore (Foundation)                                │
│  ├─ graph_name() → String                              │
│  ├─ node_count() → usize                               │
│  ├─ relationships() → Iterator                         │
│  └─ ...                                                 │
└─────────────────────────────────────────────────────────┘

          ↓ (uses)

┌─────────────────────────────────────────────────────────┐
│  AlgorithmSpec (Session 9+)                             │
│  ├─ name() → &str                                       │
│  ├─ parse_config() → JsonValue                          │
│  ├─ execute<G: GraphStore>() → ComputationResult       │
│  └─ consume_result() → Output                           │
│  ✅ Implementations: Sum, [PageRank pending]            │
└─────────────────────────────────────────────────────────┘

          ↓ (uses)

┌─────────────────────────────────────────────────────────┐
│  PipelineSpec (Week 2)                                  │
│  ├─ stages() → Vec<&dyn AlgorithmSpec>                 │
│  ├─ features() → Vec<&dyn FeatureSpec>                 │
│  ├─ models() → Vec<&dyn ModelSpec>                     │
│  └─ execute<G: GraphStore>() → PipelineResult          │
│  📋 Implementations: TBD                                │
└─────────────────────────────────────────────────────────┘

          ↓ (uses)

┌─────────────────────────────────────────────────────────┐
│  ModelSpec (Week 3)                                     │
│  ├─ name() → &str                                       │
│  ├─ train() → TrainingResult                           │
│  ├─ predict() → PredictionResult                       │
│  └─ ...                                                 │
│  📋 Implementations: TBD                                │
└─────────────────────────────────────────────────────────┘
```

Each trait builds on the previous. Each is a **plugin point** for the executor.

---

## Session Roadmap (Actual Tasks)

### Weekend (Now)

```
Task 1: Read WEEKEND_COMPREHENSION_GUIDE.md
        Read algorithm_spec.rs
        Read executor.rs
        Understand the contract ✅

Task 2: Read sum/ implementation
        Trace execution path
        See how contract is satisfied ✅

Task 3: Check speculative code (SPECULATIVE_CODE_MAP.md)
        Computation/Storage/Validation/Context
        Document observations ✅

Task 4: Review PAGERANK_SESSION_10_READY.md
        Understand what we're building next
        Prepare questions ✅

Output: Deep understanding + readiness checklist
```

### Week 1: Session 10 (PageRank)

```
Part A: Pregel Integration (1.5 hours)
  - Read pregel/computer.rs API
  - Understand message passing
  - Map Pregel traits to AlgorithmSpec

Part B: Implement PageRank (2 hours)
  - Create pagerank/ directory
  - Implement PageRankAlgorithmSpec
  - Implement StandardPageRankComputation
  - Wire to Pregel

Part C: Testing & Verification (1 hour)
  - Write 10+ integration tests
  - Verify convergence
  - Test different graph sizes
  - Confirm Computation/Storage patterns hold

Output: PageRank working, Pregel proven, system ready for pipelines
```

### Week 2: Session 11 (Pipelines)

```
Task 1: Create PipelineSpec trait
        Define what pipelines need

Task 2: Implement PipelineExecutor
        Compose algorithms in sequence

Task 3: Extend ExecutionContext
        Add user/session/project metadata

Task 4: Test pipeline (Sum → PageRank)
        Prove composition works

Output: Pipeline framework ready
```

### Week 3+: Session 12+ (ML)

```
Task 1: Feature engineering traits
Task 2: Model trainer interface
Task 3: ML executor
Task 4: Specific implementations

Output: Full ML pipeline system
```

---

## The Three Axes of Sophistication

```
AXIS 1: Computation Complexity
────────────────────────────────
    Single-Pass      Iterative         Distributed
    (Sum)     →      (PageRank)   →    (Future)
    ✅                ⏳               📋

AXIS 2: Composition Complexity
────────────────────────────────
    Standalone      Chained           ML Ensemble
    Algos      →    Pipelines    →    Systems
    ✅                📋               📋

AXIS 3: Integration Complexity
────────────────────────────────
    Algorithms      Features          Models
    Only       →    + Features   →    + Learning
    ✅                📋               📋
```

You're progressing systematically on all three axes.

---

## What Gets Easier As You Go

```
Session 9 (Sum):       400 lines for spec + 200 lines for runtime
Session 10 (PageRank): ~400 lines for spec (same structure, different computation)
Session 11 (Pipeline): ~300 lines for orchestrator (same pattern as executor)
Session 12 (Features): ~200 lines per feature type (pattern library grows)
```

**Each new system is easier than the last** because the patterns are established.

---

## The Confidence Curve

```
Day 1 (Now):     "This is sophisticated and complex"
                  🤔 High uncertainty

Day 7 (Next):    "I understand procedures system"
                  ✓ Clear roadmap

Day 14 (Wk2):    "I can implement new algorithms"
                  ✓ Patterns proven

Day 21 (Wk3):    "I can build ML systems"
                  ✓ Confidence high

```

By Sunday, you'll move to the second level: Understanding.
By next Friday (Week 1), you'll move to the third level: Mastery.
By Week 3, you'll be designing new systems with confidence.

---

## Your North Star

> **"The Procedures system is production-ready for any graph algorithm that fits the AlgorithmSpec contract. PageRank will prove it works for iterative computation. Pipelines will prove composition works. Then ML becomes a natural extension."**

This is not speculation. This is the actual roadmap based on working code.

---

## Bookmark These Files

For your weekend and beyond:

- **Understanding phase**:

  - WEEKEND_COMPREHENSION_GUIDE.md
  - PRODUCTION_READINESS_SCORECARD.md
  - SPECULATIVE_CODE_MAP.md

- **Implementation phase**:

  - PAGERANK_SESSION_10_READY.md
  - QUICK_REFERENCE_EXECUTOR.md
  - PROCEDURE_EXECUTOR_TRANSLATION.md

- **Code reference**:
  - src/projection/eval/procedure/algorithm_spec.rs
  - src/projection/eval/procedure/executor.rs
  - src/procedure/algo/sum/ (whole directory)

---

## The Moment You're At

You've just completed the most sophisticated codegen in the project. You have:

- ✅ Working code (Sum with 10 tests)
- ✅ Proven pattern (AlgorithmSpec + Executor)
- ✅ Clear roadmap (PageRank → Pipelines → ML)
- ✅ Infrastructure ready (Pregel with 14 files)

Now you need **understanding**. Not coding. Not building. **Understanding.**

This weekend is about comprehension. By Monday, you'll have the confidence to implement PageRank. By end of week, you'll be ready for Pipelines. By end of month, you'll have ML.

Breathe. Take your time. Read carefully. Ask questions. Take notes.

The work will flow once you understand the system. 🙏

Good luck this weekend. You're in a very strong position.
