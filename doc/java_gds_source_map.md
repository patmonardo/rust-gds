# Java GDS Source File Mapping

**Purpose:** Detailed mapping of Java GDS source files to Rust GDS implementation targets  
**Repository:** https://github.com/neo4j/graph-data-science  
**Version:** Latest (as of Oct 2025)

---

## 1. Core Execution Framework

### 1.1 Executor Package

| Java File                                        | Rust Target                    | Priority | Notes                                       |
| ------------------------------------------------ | ------------------------------ | -------- | ------------------------------------------- |
| `org.neo4j.gds.executor.AlgorithmSpec`           | `src/procedures/descriptor.rs` | HIGH     | Core abstraction for algorithm registration |
| `org.neo4j.gds.executor.ExecutionContext`        | `src/procedures/execution.rs`  | HIGH     | Context pattern with graph + config         |
| `org.neo4j.gds.executor.ExecutionMode`           | `src/procedures/execution.rs`  | HIGH     | Stream/Write/Mutate/Stats enum              |
| `org.neo4j.gds.executor.ProcedureExecutor`       | `src/procedures/executor.rs`   | HIGH     | Main execution coordinator                  |
| `org.neo4j.gds.executor.AlgorithmExecutor`       | `src/procedures/executor.rs`   | MEDIUM   | Algorithm-specific execution                |
| `org.neo4j.gds.executor.ValidationConfiguration` | `src/procedures/validation.rs` | MEDIUM   | Config validation patterns                  |

**Key Path:** `core/src/main/java/org/neo4j/gds/executor/`

**Translation Strategy:**

- `AlgorithmSpec` → Rust trait `ProcedureDescriptor`
- `ExecutionContext` → Rust struct with lifetime management
- Execution modes map directly to Rust enum
- Use `Result<T, ProcedureError>` for error handling

---

### 1.2 Procedures Package

| Java File                                           | Rust Target                         | Priority | Notes                                |
| --------------------------------------------------- | ----------------------------------- | -------- | ------------------------------------ |
| `GraphDataScience`                                  | `src/procedures/mod.rs`             | HIGH     | Main entry point, facade coordinator |
| `ProcedureFacade`                                   | `src/procedures/facade.rs`          | HIGH     | Base facade pattern                  |
| `ProcedureCallContext`                              | `src/procedures/context.rs`         | HIGH     | Call-site context                    |
| `algorithms/centrality/CentralityProcedureFacade`   | `src/procedures/centrality/mod.rs`  | HIGH     | Centrality algorithms facade         |
| `algorithms/community/CommunityProcedureFacade`     | `src/procedures/community/mod.rs`   | HIGH     | Community detection facade           |
| `algorithms/pathfinding/PathFindingProcedureFacade` | `src/procedures/pathfinding/mod.rs` | MEDIUM   | Path algorithms facade               |
| `algorithms/similarity/SimilarityProcedureFacade`   | `src/procedures/similarity/mod.rs`  | MEDIUM   | Similarity algorithms facade         |

**Key Path:** `proc/facade/src/main/java/org/neo4j/gds/procedures/`

**Translation Strategy:**

- Facades become Rust modules with `pub fn` methods
- Each facade method maps to procedure execution
- Use trait objects for polymorphism where needed
- Builder pattern for complex configurations

---

## 2. Algorithm Implementations

### 2.1 Centrality Algorithms

#### PageRank

| Java File                                   | Rust Target                             | Priority | Status                |
| ------------------------------------------- | --------------------------------------- | -------- | --------------------- |
| `algorithms/centrality/PageRank.java`       | `src/procedures/centrality/pagerank.rs` | HIGH     | ✅ Pregel impl exists |
| `algorithms/centrality/PageRankConfig.java` | `src/config/algo_config.rs`             | HIGH     | ✅ Config exists      |

**Reuse:** Existing `src/pregel/` implementation  
**New:** Procedure wrapper with all four modes

#### Betweenness Centrality

| Java File                                                | Rust Target                                | Priority | Status           |
| -------------------------------------------------------- | ------------------------------------------ | -------- | ---------------- |
| `algorithms/centrality/BetweennessCentrality.java`       | `src/procedures/centrality/betweenness.rs` | HIGH     | ❌ New           |
| `algorithms/centrality/BetweennessCentralityConfig.java` | `src/config/algo_config.rs`                | HIGH     | ✅ Config exists |

**Algorithm:** Brandes' algorithm with parallel execution  
**Key Files to Review:**

- `core/src/main/java/org/neo4j/gds/betweenness/BetweennessCentrality.java`
- `core/src/main/java/org/neo4j/gds/betweenness/ForwardTraverser.java`

#### Degree Centrality

| Java File                                     | Rust Target                           | Priority | Status |
| --------------------------------------------- | ------------------------------------- | -------- | ------ |
| `algorithms/centrality/DegreeCentrality.java` | `src/procedures/centrality/degree.rs` | HIGH     | ❌ New |

**Algorithm:** Simple degree counting (trivial implementation)  
**Strategy:** Can reuse graph topology methods

---

### 2.2 Community Detection Algorithms

#### Louvain

| Java File                                               | Rust Target                           | Priority | Status    |
| ------------------------------------------------------- | ------------------------------------- | -------- | --------- |
| `algorithms/community/Louvain.java`                     | `src/procedures/community/louvain.rs` | HIGH     | ❌ New    |
| `core/src/main/java/org/neo4j/gds/louvain/Louvain.java` | Same                                  | HIGH     | Translate |

**Algorithm:** Modularity-based community detection  
**Key Components:**

- Modularity computation
- Community merging
- Iterative refinement
- Multi-level hierarchy

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/louvain/
  Louvain.java                    ← Main algorithm
  LouvainBaseConfig.java          ← Configuration
  ModularityOptimization.java     ← Core optimization
```

#### Label Propagation

| Java File                                    | Rust Target                                     | Priority | Status |
| -------------------------------------------- | ----------------------------------------------- | -------- | ------ |
| `algorithms/community/LabelPropagation.java` | `src/procedures/community/label_propagation.rs` | HIGH     | ❌ New |

**Algorithm:** Iterative label spreading (good Pregel candidate)  
**Strategy:** Implement as Pregel computation

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/labelpropagation/
  LabelPropagation.java           ← Main algorithm
  LabelPropagationBaseConfig.java ← Configuration
```

#### Weakly Connected Components (WCC)

| Java File                       | Rust Target                       | Priority | Status |
| ------------------------------- | --------------------------------- | -------- | ------ |
| `algorithms/community/WCC.java` | `src/procedures/community/wcc.rs` | HIGH     | ❌ New |

**Algorithm:** Union-find with path compression  
**Strategy:** Can implement as Pregel or parallel union-find

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/wcc/
  Wcc.java                        ← Main algorithm
  WccBaseConfig.java              ← Configuration
```

---

### 2.3 Path Finding Algorithms

#### Breadth-First Search (BFS)

| Java File                         | Rust Target                         | Priority | Status |
| --------------------------------- | ----------------------------------- | -------- | ------ |
| `algorithms/pathfinding/BFS.java` | `src/procedures/pathfinding/bfs.rs` | MEDIUM   | ❌ New |

**Algorithm:** Standard BFS with queue  
**Strategy:** Use existing graph traversal patterns

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/traversal/
  BFS.java                        ← Implementation
```

#### Dijkstra Shortest Path

| Java File                                  | Rust Target                              | Priority | Status |
| ------------------------------------------ | ---------------------------------------- | -------- | ------ |
| `algorithms/pathfinding/ShortestPath.java` | `src/procedures/pathfinding/dijkstra.rs` | MEDIUM   | ❌ New |

**Algorithm:** Dijkstra with priority queue  
**Strategy:** Use binary heap for priority queue

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/paths/dijkstra/
  Dijkstra.java                   ← Main algorithm
  DijkstraConfig.java             ← Configuration
```

#### A\* Search

| Java File                           | Rust Target                           | Priority | Status |
| ----------------------------------- | ------------------------------------- | -------- | ------ |
| `algorithms/pathfinding/AStar.java` | `src/procedures/pathfinding/astar.rs` | LOW      | ❌ New |

**Algorithm:** A\* with heuristic  
**Strategy:** Extend Dijkstra with heuristic function

---

### 2.4 Similarity Algorithms

#### Node Similarity

| Java File                                   | Rust Target                                    | Priority | Status |
| ------------------------------------------- | ---------------------------------------------- | -------- | ------ |
| `algorithms/similarity/NodeSimilarity.java` | `src/procedures/similarity/node_similarity.rs` | MEDIUM   | ❌ New |

**Algorithm:** Jaccard/Overlap/Cosine similarity  
**Strategy:** Parallel pairwise comparison

**Key Files to Review:**

```
core/src/main/java/org/neo4j/gds/similarity/nodesim/
  NodeSimilarity.java             ← Main algorithm
  NodeSimilarityConfig.java       ← Configuration
```

---

## 3. ML Pipeline Framework

### 3.1 Pipeline Core

| Java File                                                                            | Rust Target                | Priority | Status |
| ------------------------------------------------------------------------------------ | -------------------------- | -------- | ------ |
| `ml/ml-core/src/main/java/org/neo4j/gds/ml/pipeline/Pipeline.java`                   | `src/pipeline/mod.rs`      | HIGH     | ❌ New |
| `ml/ml-core/src/main/java/org/neo4j/gds/ml/pipeline/PipelineTrainer.java`            | `src/pipeline/training.rs` | HIGH     | ❌ New |
| `ml/ml-core/src/main/java/org/neo4j/gds/ml/pipeline/ExecutableNodePropertyStep.java` | `src/pipeline/stage.rs`    | HIGH     | ❌ New |

**Key Path:** `ml/ml-core/src/main/java/org/neo4j/gds/ml/pipeline/`

**Translation Strategy:**

- `Pipeline` → Rust struct with `Vec<Box<dyn PipelineStage>>`
- `PipelineTrainer` → Trait for training logic
- `ExecutableNodePropertyStep` → Trait for pipeline stages

**Key Concepts:**

1. **Stage Composition:** Chaining stages in sequence
2. **State Management:** Accumulating features and models
3. **Training vs Prediction:** Different execution paths
4. **Model Persistence:** Saving/loading trained models

---

### 3.2 Node Classification Pipeline

| Java File                                                                                                  | Rust Target                                      | Priority | Status |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------ | -------- | ------ |
| `ml/ml-algo/src/main/java/org/neo4j/gds/ml/nodePropertyPrediction/NodeClassificationPipeline.java`         | `src/pipeline/node_classification/pipeline.rs`   | HIGH     | ❌ New |
| `ml/ml-algo/src/main/java/org/neo4j/gds/ml/nodePropertyPrediction/NodeClassificationTrainingPipeline.java` | `src/pipeline/node_classification/training.rs`   | HIGH     | ❌ New |
| `ml/ml-algo/src/main/java/org/neo4j/gds/ml/nodePropertyPrediction/NodeClassificationPredictPipeline.java`  | `src/pipeline/node_classification/prediction.rs` | HIGH     | ❌ New |

**Key Path:** `ml/ml-algo/src/main/java/org/neo4j/gds/ml/nodePropertyPrediction/`

**Pipeline Stages:**

1. **Feature Extraction:** Node properties → feature vectors
2. **Normalization:** Scale/normalize features
3. **Training:** Train classifier (Logistic Regression, Random Forest, etc.)
4. **Prediction:** Apply trained model to new nodes

**Key Files to Review:**

```
ml/ml-algo/src/main/java/org/neo4j/gds/ml/nodePropertyPrediction/
  NodeClassificationPipeline.java
  NodeClassificationTrainingPipeline.java
  NodeClassificationPredictPipeline.java
```

---

### 3.3 Link Prediction Pipeline

| Java File                                                                          | Rust Target                                | Priority | Status |
| ---------------------------------------------------------------------------------- | ------------------------------------------ | -------- | ------ |
| `ml/ml-algo/src/main/java/org/neo4j/gds/ml/linkmodels/LinkPredictionPipeline.java` | `src/pipeline/link_prediction/pipeline.rs` | MEDIUM   | ❌ New |
| Similar training/prediction files                                                  | Similar targets                            | MEDIUM   | ❌ New |

**Key Path:** `ml/ml-algo/src/main/java/org/neo4j/gds/ml/linkmodels/`

**Pipeline Stages:**

1. **Link Features:** Extract features from node pairs
2. **Negative Sampling:** Generate negative examples
3. **Training:** Train link classifier
4. **Prediction:** Predict link probability

---

### 3.4 Embeddings

#### FastRP (Fast Random Projection)

| Java File                                                              | Rust Target                         | Priority | Status |
| ---------------------------------------------------------------------- | ----------------------------------- | -------- | ------ |
| `ml/ml-algo/src/main/java/org/neo4j/gds/embeddings/fastrp/FastRP.java` | `src/pipeline/embeddings/fastrp.rs` | MEDIUM   | ❌ New |

**Algorithm:** Random projection with iterative aggregation  
**Strategy:** Matrix operations + graph aggregation

**Key Files to Review:**

```
ml/ml-algo/src/main/java/org/neo4j/gds/embeddings/fastrp/
  FastRP.java                     ← Main algorithm
  FastRPBaseConfig.java           ← Configuration
```

#### Node2Vec

| Java File                                                                  | Rust Target                           | Priority | Status |
| -------------------------------------------------------------------------- | ------------------------------------- | -------- | ------ |
| `ml/ml-algo/src/main/java/org/neo4j/gds/embeddings/node2vec/Node2Vec.java` | `src/pipeline/embeddings/node2vec.rs` | LOW      | ❌ New |

**Algorithm:** Random walks + Skip-gram  
**Strategy:** Walk generation + word2vec-style training

---

## 4. Configuration System

### 4.1 Base Configuration Classes

| Java File                    | Rust Target                       | Priority | Status            |
| ---------------------------- | --------------------------------- | -------- | ----------------- |
| `config/AlgoBaseConfig.java` | `src/config/algo_config.rs`       | HIGH     | ✅ Pattern exists |
| `config/WriteConfig.java`    | `src/config/io_config.rs`         | HIGH     | ✅ Exists         |
| `config/MutateConfig.java`   | `src/config/graphstore_config.rs` | HIGH     | ✅ Exists         |

**Key Path:** `core/src/main/java/org/neo4j/gds/config/`

**Translation Strategy:**

- Already have builder-based config pattern
- Extend existing configs with new algorithm parameters
- Reuse validation patterns from existing code

---

## 5. Result Types

### 5.1 Common Result Structures

| Java File                      | Rust Target                           | Priority | Notes                       |
| ------------------------------ | ------------------------------------- | -------- | --------------------------- |
| `result/AbstractResultBuilder` | `src/procedures/result.rs`            | HIGH     | Builder pattern for results |
| `result/CentralityStatistics`  | `src/procedures/centrality/result.rs` | HIGH     | Centrality-specific results |
| `result/CommunityStatistics`   | `src/procedures/community/result.rs`  | HIGH     | Community-specific results  |

**Translation Strategy:**

- Create `ProcedureResult` enum for polymorphic results
- Use builder pattern for complex result construction
- Implement `Display` trait for human-readable output

---

## 6. Memory Estimation

### 6.1 Memory Estimation Framework

| Java File                    | Rust Target                    | Priority | Status    |
| ---------------------------- | ------------------------------ | -------- | --------- |
| `mem/MemoryEstimation.java`  | `src/mem/memory_estimation.rs` | MEDIUM   | ✅ Exists |
| `mem/MemoryEstimations.java` | `src/mem/memest/`              | MEDIUM   | ✅ Exists |

**Strategy:** Reuse existing memory estimation system from `src/mem/`

---

## 7. Progress Tracking Integration

### 7.1 Progress Tracking

| Java File                                  | Rust Target                             | Priority | Status      |
| ------------------------------------------ | --------------------------------------- | -------- | ----------- |
| `core/utils/progress/ProgressTracker.java` | `src/core/utils/progress/tasks/`        | HIGH     | ✅ Complete |
| `core/utils/progress/tasks/Task.java`      | `src/core/utils/progress/tasks/task.rs` | HIGH     | ✅ Complete |

**Strategy:** Use existing `LeafTask` for all algorithm progress tracking

---

## 8. Translation Priorities

### Phase 1: Foundation (Days 1-2)

```
HIGH PRIORITY - Must translate first:
✓ AlgorithmSpec → ProcedureDescriptor
✓ ExecutionContext → ProcedureContext
✓ ProcedureExecutor → Executor infrastructure
✓ Registry pattern setup
```

### Phase 2: First Algorithms (Days 3-5)

```
HIGH PRIORITY - Prove the pattern:
✓ PageRank procedure (reuse Pregel)
✓ Degree Centrality (simple)
✓ Louvain (complex)
✓ Label Propagation (Pregel)
✓ BFS (path finding)
```

### Phase 3: ML Pipelines (Days 6-8)

```
MEDIUM PRIORITY - Extend pattern:
✓ Pipeline trait system
✓ Node Classification pipeline
✓ FastRP embeddings
✓ Link Prediction (if time)
```

### Phase 4: Polish (Days 9-10)

```
FINALIZATION:
✓ Form Processor unification
✓ Examples for each algorithm
✓ Integration tests
✓ Documentation
```

---

## 9. Files NOT to Translate (Out of Scope)

### 9.1 Neo4j-Specific Code

```
❌ Transaction management (Rust GDS has no transactions)
❌ Cypher integration (we're not embedding in Neo4j)
❌ Security/authentication (handle at application level)
❌ Logging framework (use Rust's log/tracing crates)
```

### 9.2 Advanced Features (Future Work)

```
⏸️ Graph catalog persistence (future: save/load graphs)
⏸️ Arrow Flight integration (future: remote execution)
⏸️ Distributed execution (future: multi-node)
⏸️ Streaming writes (future: incremental updates)
```

---

## 10. Quick Reference Commands

### Clone Java GDS

```bash
git clone https://github.com/neo4j/graph-data-science.git
cd graph-data-science
```

### Navigate to Key Directories

```bash
# Core algorithms
cd core/src/main/java/org/neo4j/gds

# Procedures
cd proc/facade/src/main/java/org/neo4j/gds/procedures

# ML Pipelines
cd ml/ml-algo/src/main/java/org/neo4j/gds/ml
```

### Search for Specific Algorithms

```bash
# Find all centrality algorithms
find . -name "*Centrality*.java"

# Find all pipeline-related files
find . -path "*/ml/pipeline/*.java"

# Find configuration files
find . -name "*Config.java" | grep -E "(centrality|community|pathfinding)"
```

---

## 11. Review Checklist

Before starting codegen:

- [ ] Java GDS repository cloned
- [ ] Key files identified for each algorithm
- [ ] Algorithm patterns understood (Pregel vs iterative vs one-shot)
- [ ] Configuration patterns mapped
- [ ] Result types mapped
- [ ] Memory estimation approach decided
- [ ] Progress tracking integration planned
- [ ] Test strategy defined

---

**Status:** READY FOR REVIEW  
**Next Step:** Weekend study, Monday/Tuesday codegen launch  
**Companion Doc:** `next_codegen_review.md`
