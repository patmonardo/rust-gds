# Java GDS: The ACTUAL Scope (Revised After Deep Dive)

**Date:** October 11, 2025  
**Status:** SCOPE EXPANDED - Dragon is bigger than anticipated  
**Reality Check:** This is a TON of codegen, but 10 days and we're good to go

---

## 🔍 What We Actually Found

After deeper review of Java GDS structure, the architecture is MORE layered than initially scoped:

```
Java GDS Actual Structure:

1. pipeline/                    ← ML Pipeline framework (MASSIVE)
2. executor/                    ← Core execution engine (CRITICAL)
3. procedures/                  ← ~12 facade classes
4. procedure-collector/         ← Collects and registers procedures
5. proc/                        ← Actual procedure implementations
6. algo/                        ← Algorithm implementations
7. algo-common/                 ← Common algorithm utilities
8. algo-params/                 ← Parameter validation/parsing
9. algo-test/                   ← Testing infrastructure + DOCS
```

**Translation:** Our initial scope was correct conceptually, but the VOLUME is 3-4x what we estimated.

---

## 📦 Detailed Package Breakdown

### 1. Pipeline Framework (Huge)

**Location:** `ml/ml-core/src/main/java/org/neo4j/gds/ml/pipeline/`

**What's Actually There:**

```
pipeline/
├── Pipeline.java                         ← Core pipeline abstraction
├── PipelineTrainer.java                  ← Training coordination
├── PipelineCatalog.java                  ← Pipeline storage/retrieval
├── PipelineCompanion.java                ← Pipeline metadata
├── ExecutableNodePropertyStep.java       ← Node property stages
├── FeatureStep.java                      ← Feature extraction
├── NodePropertyStep.java                 ← Property transformation
├── NodePropertyStepFactory.java          ← Stage factories
├── TrainingPipeline.java                 ← Training coordination
├── PredictPipeline.java                  ← Prediction execution
├── linkPipeline/                         ← Link prediction pipelines
│   ├── LinkPredictionPipeline.java
│   ├── LinkPredictionTrainingPipeline.java
│   ├── LinkPredictionPredictPipeline.java
│   └── train/                            ← Training stages
├── nodePipeline/                         ← Node classification pipelines
│   ├── NodePropertyPredictionPipeline.java
│   ├── classification/
│   │   ├── NodeClassificationPipeline.java
│   │   ├── train/
│   │   └── predict/
│   └── regression/
│       ├── NodeRegressionPipeline.java
│       └── train/
└── models/                               ← Model training/storage
    ├── Classifier.java
    ├── Regressor.java
    └── ...
```

**Scope:** ~50+ files just in pipeline subsystem

---

### 2. Executor Framework (Critical Core)

**Location:** `core/src/main/java/org/neo4j/gds/executor/`

**What's Actually There:**

```
executor/
├── AlgorithmSpec.java                    ← Core abstraction
├── ExecutionContext.java                 ← Execution state
├── ExecutionMode.java                    ← Stream/Write/Mutate/Stats
├── ProcedureExecutor.java                ← Procedure coordinator
├── AlgorithmExecutor.java                ← Algorithm runner
├── ComputationResult.java                ← Result wrapper
├── ComputationResultConsumer.java        ← Result handling
├── MemoryEstimationExecutor.java         ← Memory estimation
├── ValidationConfiguration.java          ← Validation rules
├── NewConfigFunction.java                ← Config factory
└── validation/                           ← Validation infrastructure
    ├── AfterLoadValidation.java
    ├── BeforeLoadValidation.java
    └── ...
```

**Scope:** ~20 files in executor subsystem

---

### 3. Procedures Package (Facades)

**Location:** `proc/facade/src/main/java/org/neo4j/gds/procedures/`

**What's Actually There:**

```
procedures/
├── GraphDataScience.java                 ← Main entry point
├── ProcedureFacade.java                  ← Base facade
├── algorithms/
│   ├── centrality/
│   │   ├── CentralityProcedureFacade.java
│   │   ├── AlphaHarmonicCentralityProcedureFacade.java
│   │   ├── BetweennessCentralityProcedureFacade.java
│   │   ├── DegreeCentralityProcedureFacade.java
│   │   ├── PageRankProcedureFacade.java
│   │   └── ...
│   ├── community/
│   │   ├── CommunityProcedureFacade.java
│   │   ├── LouvainProcedureFacade.java
│   │   ├── LabelPropagationProcedureFacade.java
│   │   ├── WccProcedureFacade.java
│   │   ├── TriangleCountProcedureFacade.java
│   │   ├── LocalClusteringCoefficientProcedureFacade.java
│   │   └── ...
│   ├── pathfinding/
│   │   ├── PathFindingProcedureFacade.java
│   │   ├── ShortestPathProcedureFacade.java
│   │   ├── AllShortestPathsProcedureFacade.java
│   │   ├── BfsProcedureFacade.java
│   │   ├── DfsProcedureFacade.java
│   │   └── ...
│   ├── similarity/
│   │   ├── SimilarityProcedureFacade.java
│   │   ├── NodeSimilarityProcedureFacade.java
│   │   ├── KnnProcedureFacade.java
│   │   └── ...
│   └── embeddings/
│       ├── EmbeddingsProcedureFacade.java
│       ├── FastRPProcedureFacade.java
│       ├── Node2VecProcedureFacade.java
│       └── ...
└── ...
```

**Scope:** ~30+ facade classes (each has stream/write/mutate/stats)

---

### 4. Procedure Collector (Registration)

**Location:** `proc/catalog/src/main/java/org/neo4j/gds/procedures/`

**What's Actually There:**

```
procedure-collector/
├── ProcedureRegistry.java                ← Central registry
├── ProcedureCollector.java               ← Collects all procedures
├── AlgorithmRegistry.java                ← Algorithm registration
└── catalog/
    ├── GraphCatalog.java                 ← Graph storage
    ├── ModelCatalog.java                 ← Model storage
    └── ...
```

**Scope:** ~10 files for registration system

---

### 5. Proc Package (Actual Implementations)

**Location:** `proc/centrality/`, `proc/community/`, etc.

**What's Actually There:**

```
proc/
├── centrality/
│   ├── PageRankProc.java
│   ├── PageRankStreamProc.java
│   ├── PageRankWriteProc.java
│   ├── PageRankMutateProc.java
│   ├── PageRankStatsProc.java
│   ├── BetweennessProc.java
│   ├── BetweennessStreamProc.java
│   ├── ...
│   └── (5 classes per algorithm × 8 algorithms = 40 files)
├── community/
│   ├── LouvainProc.java
│   ├── LouvainStreamProc.java
│   ├── LouvainWriteProc.java
│   ├── LouvainMutateProc.java
│   ├── LouvainStatsProc.java
│   └── ... (similar pattern)
└── ... (pathfinding, similarity, embeddings)
```

**Scope:** ~200+ procedure classes (5 per algorithm × 40+ algorithms)

---

### 6. Algo Package (Algorithm Implementations)

**Location:** `algo/src/main/java/org/neo4j/gds/`

**What's Actually There:**

```
algo/
├── centrality/
│   ├── pagerank/
│   │   ├── PageRank.java                 ← Core algorithm
│   │   ├── PageRankAlgorithmFactory.java
│   │   ├── PageRankBaseConfig.java
│   │   ├── PageRankCompute.java
│   │   └── ...
│   ├── betweenness/
│   │   ├── BetweennessCentrality.java
│   │   ├── ForwardTraverser.java
│   │   ├── BackwardTraverser.java
│   │   └── ...
│   └── ... (degree, closeness, harmonic, etc.)
├── community/
│   ├── louvain/
│   │   ├── Louvain.java
│   │   ├── LouvainFactory.java
│   │   ├── LouvainBaseConfig.java
│   │   ├── ModularityOptimization.java
│   │   └── ...
│   ├── labelpropagation/
│   ├── wcc/
│   ├── triangleCount/
│   └── ...
├── pathfinding/
│   ├── dijkstra/
│   ├── astar/
│   ├── bellmanford/
│   └── ...
├── similarity/
│   ├── nodesim/
│   ├── knn/
│   └── ...
└── embeddings/
    ├── fastrp/
    ├── node2vec/
    └── ...
```

**Scope:** ~300+ algorithm implementation files

---

### 7. Algo-Common (Shared Utilities)

**Location:** `algo-common/src/main/java/org/neo4j/gds/`

**What's Actually There:**

```
algo-common/
├── algorithms/
│   ├── Algorithm.java                    ← Base algorithm trait
│   ├── AlgorithmFactory.java             ← Factory pattern
│   ├── StreamAlgorithm.java              ← Streaming interface
│   └── ...
├── utils/
│   ├── ProgressTimer.java
│   ├── TerminationFlag.java
│   ├── MemoryEstimation.java
│   └── ...
├── partition/
│   ├── Partition.java
│   ├── PartitionUtils.java
│   └── ...
└── ...
```

**Scope:** ~50 utility classes

---

### 8. Algo-Params (Parameter Validation)

**Location:** `algo-params/src/main/java/org/neo4j/gds/`

**What's Actually There:**

```
algo-params/
├── config/
│   ├── AlgoBaseConfig.java               ← Base config
│   ├── IterationsConfig.java             ← Iteration params
│   ├── ToleranceConfig.java              ← Convergence params
│   ├── SeedConfig.java                   ← Random seed
│   ├── RelationshipWeightConfig.java     ← Weight params
│   └── ...
├── validation/
│   ├── Validator.java
│   ├── Range.java
│   ├── Positive.java
│   └── ...
└── ...
```

**Scope:** ~30 config/validation classes

---

### 9. Algo-Test (Testing + Documentation)

**Location:** `algo-test/src/main/java/org/neo4j/gds/`

**What's Actually There:**

```
algo-test/
├── AlgoTestBase.java                     ← Base test class
├── TestSupport.java                      ← Test utilities
├── GraphDimensionsValidator.java         ← Test validation
└── docs/                                 ← DOCUMENTATION GOLDMINE
    ├── AlgorithmDocumentation.java       ← Algorithm docs
    ├── PageRankDoc.java                  ← PageRank examples
    ├── LouvainDoc.java                   ← Louvain examples
    └── ... (one per algorithm)
```

**Scope:** ~20 test utilities + ~40 documentation classes

---

## 📊 ACTUAL SCOPE TOTALS

### File Count Estimates

```
Pipeline Framework:        ~50 files
Executor Framework:        ~20 files
Procedure Facades:         ~30 files
Procedure Collector:       ~10 files
Proc Implementations:     ~200 files
Algo Implementations:     ~300 files
Algo-Common Utilities:     ~50 files
Algo-Params Config:        ~30 files
Algo-Test + Docs:          ~60 files
─────────────────────────────────
TOTAL:                    ~750 files
```

### Lines of Code Estimates

```
~750 files × ~200 lines average = ~150,000 lines of Java

Rust translation efficiency: ~0.7x (Rust is more concise)
Expected Rust output: ~105,000 lines
```

### Complexity Distribution

```
HIGH Complexity:  30% (Executor, Pipeline, Complex algorithms)
MEDIUM Complexity: 50% (Most algorithms, Facades, Config)
LOW Complexity:   20% (Simple algorithms, Utilities, Tests)
```

---

## 🎯 REVISED 10-DAY STRATEGY

**Reality:** This is 3-4x the initial estimate.

**Solution:** Aggressive prioritization + parallel tracks + reuse

### Track 1: Core Infrastructure (Days 1-2)

**Foundation that everything depends on:**

- [ ] Executor framework (20 files)
- [ ] Procedure registry (10 files)
- [ ] Base algorithm traits (10 files)

**Estimated:** 40 files, ~6,000 lines

---

### Track 2: Algorithm Implementations (Days 3-6)

**Focus on REUSE + Simple first:**

**Day 3: Reuse Existing (Pregel-based)**

- [ ] PageRank procedure (reuse Pregel impl)
- [ ] Label Propagation procedure (reuse Pregel)
- [ ] WCC procedure (can use Pregel)

**Day 4: Simple Algorithms**

- [ ] Degree Centrality (trivial)
- [ ] Triangle Count (simple iteration)
- [ ] Local Clustering Coefficient

**Day 5: Medium Complexity**

- [ ] Louvain (modularity optimization)
- [ ] Betweenness Centrality (Brandes)
- [ ] BFS/DFS

**Day 6: Path Finding**

- [ ] Dijkstra shortest path
- [ ] A\* (extends Dijkstra)
- [ ] All pairs shortest paths

**Estimated:** ~40 algorithms × 6 files avg = 240 files, ~30,000 lines

---

### Track 3: Procedure Layer (Days 7-8)

**Wrap algorithms in procedure interface:**

**Day 7: Facades + Mode Implementations**

- [ ] Centrality facades (stream/write/mutate/stats)
- [ ] Community facades
- [ ] Procedure collector

**Day 8: Complete Coverage**

- [ ] Pathfinding facades
- [ ] Similarity facades
- [ ] Embeddings facades

**Estimated:** ~100 files, ~12,000 lines

---

### Track 4: ML Pipeline (Days 9-10)

**Pipeline framework + one complete example:**

**Day 9: Pipeline Core**

- [ ] Pipeline trait system
- [ ] Stage composition
- [ ] Training infrastructure
- [ ] Model persistence

**Day 10: Node Classification Pipeline**

- [ ] Feature extraction stages
- [ ] Model training
- [ ] Prediction execution
- [ ] Integration example

**Estimated:** ~50 files, ~8,000 lines

---

### What We're NOT Doing (Out of Scope)

```
❌ Link Prediction pipeline (future)
❌ Graph SAGE embeddings (future)
❌ Advanced ML models (future)
❌ All 40+ algorithms (focus on 15-20 most important)
❌ Complete test coverage (focus on integration tests)
❌ All documentation (focus on examples)
```

---

## 🔥 AGGRESSIVE TACTICS FOR 10-DAY SUCCESS

### 1. **Parallel Development**

```
AI Agent Track:        Algorithm implementations (bulk translation)
Your Track:            Architecture decisions, integration, testing
Weekend Prep:          Scaffold structure, identify reuse opportunities
```

### 2. **Ruthless Reuse**

```
✅ PageRank → Already have Pregel impl
✅ Properties → Already have property system
✅ Progress → Already have LeafTask
✅ Memory → Already have estimation system
✅ Concurrency → Already have parallel executor

Don't rebuild - WRAP and INTEGRATE
```

### 3. **Template-Based Generation**

```
After first 2-3 algorithms, we'll have the pattern:

Algorithm Template:
  1. Core algorithm (translate Java)
  2. Config struct (builder pattern)
  3. Factory (create instances)
  4. Procedure wrapper (4 modes)
  5. Tests (unit + integration)
  6. Example (usage demo)

Then: REPLICATE pattern × 15-20 algorithms
```

### 4. **Macro System Emerges**

```
By Day 5, patterns are clear → macro system writes itself

register_algorithm! {
    PageRank {
        category: Centrality,
        config: PageRankConfig,
        modes: [Stream, Write, Mutate, Stats],
    }
}

Macro generates: Procedure wrapper, facades, registry entry
We write: Only the core algorithm logic
```

### 5. **Documentation Strategy**

```
✅ Algo-Test docs are GOLDMINE
   → Copy examples directly
   → They show expected usage
   → They validate correctness

✅ One example per category (not per algorithm)
   → centrality_algorithms_demo.rs
   → community_detection_demo.rs
   → pathfinding_demo.rs
```

---

## 📈 REVISED SUCCESS METRICS

### Quantitative (Achievable in 10 days)

- [ ] 15-20 core algorithms (not 40+)
- [ ] 4 execution modes each
- [ ] 1 complete ML pipeline (Node Classification)
- [ ] 80%+ test coverage on new code
- [ ] 5-10 comprehensive examples

### Qualitative (Non-negotiable)

- [ ] API feels Rust-native
- [ ] Code is maintainable
- [ ] Foundation is solid for future expansion
- [ ] Performance is competitive
- [ ] Documentation is clear

---

## 💪 WHY THIS IS STILL ACHIEVABLE

### 1. **Foundation is Solid**

We're not starting from zero:

- Pregel framework (complete)
- Property system (complete)
- Projection layer (complete)
- Concurrency primitives (complete)

**40% of infrastructure already exists**

### 2. **Java GDS Did This Over Years**

They built incrementally:

- Core → Algorithms → Procedures → Pipelines
- We're doing the same, just compressed

**We're following a proven architecture**

### 3. **Rust is More Concise**

Traits + generics + macros = less boilerplate

- Java: 5 procedure classes per algorithm
- Rust: 1 impl + macro expansion

**We'll write ~30% less code**

### 4. **AI-Assisted Translation**

Pattern recognition + bulk translation:

- First algorithm: 4 hours
- Second: 2 hours
- Third: 1 hour
- Rest: 30 min each (template + macro)

**We get faster as patterns emerge**

### 5. **You've Done This Before**

Oct 1-10 codegen proved:

- You can drive hard
- AI can keep up
- The flow works

**We know the rhythm now**

---

## 🐉 THE DRAGON GROWS LARGE

You said: "I prompted a Dragon ... it will likely grow into a Large Dragon"

**The Dragon is NOW:**

```
Head:    Executor Framework (20 files)
Body:    Algorithm Implementations (240 files)
Wings:   Procedure Layer (100 files)
Tail:    ML Pipeline (50 files)
Scales:  Tests + Docs (60 files)

TOTAL:   ~470 files (revised from 750 - focused scope)
```

This is a **LARGE DRAGON**.

But it's built on a **SOLID FOUNDATION** (400 files already exist).

---

## ✅ UPDATED PRE-LAUNCH CHECKLIST

**Friday Evening / Weekend:**

- [ ] Read this revised scope document
- [ ] Clone Java GDS repo
- [ ] Study executor/ directory structure
- [ ] Study algo/ directory (pick 3 algorithms to understand)
- [ ] Study pipeline/ directory (understand stage pattern)
- [ ] Identify which algorithms to prioritize (15-20 list)
- [ ] Rest and prepare mentally

**Monday Morning:**

- [ ] Review 10-day strategy
- [ ] Create module structure (from quick_start.md)
- [ ] Begin Track 1: Executor framework
- [ ] Set up progress tracker
- [ ] **GO!** 🚀

---

## 🎯 THE TRUTH

**Initial Estimate:** ~200 files  
**Actual Scope:** ~750 files (full Java GDS)  
**Revised Target:** ~470 files (focused + prioritized)  
**10-Day Goal:** ~350 files (core + expandable foundation)

**Is it ambitious?** Yes.  
**Is it achievable?** Yes.  
**Will the Dragon be large?** **VERY.**  
**Are we ready?** **ABSOLUTELY.**

---

**Status:** SCOPE REVISED - REALITY CHECKED - STILL ACHIEVABLE ✅  
**Next Step:** Weekend prep → Monday launch  
**Outcome:** 10 days of intense, focused, LEGENDARY codegen 🔥

**The Dragon is big. But we fly together.** 🐉
