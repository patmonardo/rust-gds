# Pipeline Integration Roadmap

**Date**: October 14, 2025  
**Phase**: Graph API Integration & Pipeline Design  
**Prerequisites**: ML-Core foundation complete (230 tests passing)

---

## Current State Assessment

### ✅ What We Have (ML-Core)

1. **Tensor Operations** - Matrix, Vector, Scalar with full gradient support
2. **Variable System** - 19 ML functions (Sigmoid, Softmax, Loss functions, etc.)
3. **Computation Context** - Forward/backward propagation with caching
4. **Batch Processing** - Lazy iteration, parallel consumption foundation
5. **Samplers** - Uniform, Weighted, Long samplers with adaptive strategies
6. **Feature Extraction** - Scalar and Array extractors with enum dispatch
7. **Subgraph Foundation** - LocalIdMap, BatchNeighbors, SubGraph basics
8. **Abstractions** - NeighborhoodFunction, RelationshipWeights traits
9. **DecisionTree** - Ready to integrate (needs review)

### ⚠️ What Needs Integration

1. **Graph API** - Connect ML-Core to GraphStore/Graph types
2. **NeighborhoodSampler** - Full implementation with Graph streaming
3. **SubGraph Builders** - buildSubGraph(), buildSubGraphs() methods
4. **Pipeline System** - End-to-end ML workflows
5. **Training Loops** - Using SubGraph + BatchQueue + Variables

---

## Phase 1: Graph API Integration Review

### Goals

1. **Understand Current State**

   - What GraphStore/Graph APIs exist?
   - What's working vs what's problematic?
   - How does RandomGraph fit in?

2. **Identify Integration Points**

   - Where does ML-Core need Graph types?
   - How do NeighborhoodFunction/RelationshipWeights connect?
   - What test utilities are available?

3. **Plan Minimal Integration**
   - What's the smallest working connection?
   - Can we test incrementally?
   - What's the critical path?

### Tasks (Session 1)

#### 1.1 GraphStore API Survey

```rust
// Questions:
// - What graph types exist? (Graph, GraphStore, etc.)
// - How do we access nodes/relationships?
// - How do we stream neighbors?
// - How do we get relationship properties/weights?
// - What's the difference between GraphStore and Graph?
```

**Files to Review**:

- `src/types/graph/*.rs` (or wherever Graph API lives)
- `src/projection/graph_store/*.rs` (GraphStore implementation)
- Any existing Graph trait definitions

**Output**: Document of Graph API capabilities and limitations

#### 1.2 RandomGraph Review

```rust
// Questions:
// - How do we create test graphs?
// - What's the RandomGraphConfig API?
// - Can we create deterministic test graphs?
// - How do we add properties/weights?
```

**Files to Review**:

- Wherever RandomGraph utilities live
- Test files using random graphs
- Configuration builders

**Output**: Test utility checklist and examples

#### 1.3 Integration Point Design

```rust
// Design questions:
// - Should Graph implement NeighborhoodFunction?
// - Should Graph implement RelationshipWeights?
// - Or do we create adapter types?
// - How do we handle lifetimes?
```

**Approaches to Consider**:

**Option A: Direct Implementation**

```rust
impl NeighborhoodFunction for Graph {
    fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(self.neighbors(node_id))
    }
}
```

**Option B: Adapter Pattern**

```rust
struct GraphNeighborhoodSampler<'a> {
    graph: &'a Graph,
    num_samples: usize,
}

impl NeighborhoodFunction for GraphNeighborhoodSampler<'_> {
    fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
        // Use NeighborhoodSampler with graph
    }
}
```

**Output**: Design document with pros/cons of each approach

---

## Phase 2: Pipeline System Survey

### Goals

1. **Understand Java GDS Pipeline**

   - What is Pipeline? (abstraction for ML workflows)
   - What components does it have?
   - How does it orchestrate training?

2. **Identify Our Needs**

   - What do we need for basic GNN training?
   - What do we need for DecisionTree integration?
   - Can we simplify?

3. **Design Rust-Idiomatic Pipeline**
   - Use composition, not inheritance
   - Use trait objects where needed
   - Leverage existing batch/sampler infrastructure

### Java GDS Pipeline Components (To Survey)

#### 2.1 Core Pipeline

**Location**: `ml/ml-algo/src/main/java/org/neo4j/gds/ml/pipeline/`

**Questions**:

- What's the Pipeline trait/interface?
- How does it define stages?
- How does it manage state?
- How does it handle errors?

#### 2.2 Node Classification Pipeline

**Location**: Wherever node classification lives

**Questions**:

- How does it use features?
- How does it create batches?
- How does it train models?
- How does it evaluate?

#### 2.3 Link Prediction Pipeline

**Location**: Wherever link prediction lives

**Questions**:

- How does it differ from node classification?
- How does it use relationship features?
- How does it sample negative edges?

#### 2.4 Special Pipelines (Unknown)

**Note**: User mentioned "two special pipelines we haven't even looked at"

**Tasks**:

- Identify these pipelines in Java GDS
- Understand their purpose
- Assess necessity for Rust port

### Rust Pipeline Design Considerations

#### Key Questions

1. **Trait or Struct?**

   ```rust
   // Option A: Trait-based
   pub trait Pipeline {
       type Input;
       type Output;
       fn train(&mut self, data: Self::Input) -> Result<Self::Output>;
   }

   // Option B: Concrete with generics
   pub struct Pipeline<M, F> {
       model: M,
       features: F,
   }
   ```

2. **Builder Pattern?**

   ```rust
   Pipeline::builder()
       .model(model_config)
       .features(feature_extractors)
       .training_config(config)
       .build()
   ```

3. **State Management?**

   ```rust
   // Owned state
   struct Pipeline { state: PipelineState }

   // vs

   // Shared state
   struct Pipeline<'a> { state: &'a mut PipelineState }
   ```

4. **Error Handling?**

   ```rust
   type Result<T> = std::result::Result<T, PipelineError>;

   enum PipelineError {
       GraphError(GraphError),
       ModelError(ModelError),
       DataError(String),
   }
   ```

---

## Phase 3: Minimal Working Integration

### Goal: SubGraph + Graph API

**Objective**: Get SubGraph.buildSubGraph() working with real Graph

**Steps**:

1. **Implement GraphNeighborhoodFunction**

   ```rust
   struct GraphNeighborhoodSampler<'a> {
       graph: &'a Graph,
       sampler: NeighborhoodSampler,
   }

   impl NeighborhoodFunction for GraphNeighborhoodSampler<'_> {
       fn sample(&self, node_id: u64) -> Box<dyn Iterator<Item = u64> + '_> {
           // Use sampler.sample(graph, node_id, num_samples)
       }
   }
   ```

2. **Implement GraphRelationshipWeights**

   ```rust
   impl RelationshipWeights for Graph {
       fn weight_with_default(&self, src: u64, tgt: u64, def: f64) -> f64 {
           self.relationship_property(src, tgt).unwrap_or(def)
       }
   }
   ```

3. **Implement SubGraph::build_subgraph()**

   ```rust
   impl SubGraph {
       pub fn build_subgraph(
           batch_node_ids: &[u64],
           neighborhood_fn: &dyn NeighborhoodFunction,
           weight_fn: &dyn RelationshipWeights,
       ) -> Self {
           let mut idmap = LocalIdMap::new();

           // Map batch nodes
           for &id in batch_node_ids {
               idmap.to_mapped(id);
           }

           // Sample neighbors
           let mut neighbors = Vec::new();
           for local_id in 0..idmap.size() {
               let original_id = idmap.to_original(local_id);
               let sampled: Vec<u64> = neighborhood_fn
                   .sample(original_id)
                   .map(|id| idmap.to_mapped(id) as u64)
                   .collect();
               neighbors.push(sampled);
           }

           SubGraph::new(
               (0..batch_node_ids.len()).collect(),
               idmap.original_ids_vec(),
               neighbors,
               /* weighted */ true,
           )
       }
   }
   ```

4. **Write Integration Test**
   ```rust
   #[test]
   fn test_subgraph_from_random_graph() {
       let graph = RandomGraph::new(100, 0.1);  // 100 nodes, 10% edge prob
       let sampler = GraphNeighborhoodSampler::new(&graph, 5);  // 5 neighbors
       let weights = &graph as &dyn RelationshipWeights;

       let batch = vec![0, 1, 2, 3, 4];
       let subgraph = SubGraph::build_subgraph(&batch, &sampler, weights);

       assert_eq!(subgraph.batch_size(), 5);
       assert!(subgraph.node_count() >= 5);  // At least batch nodes
   }
   ```

---

## Phase 4: DecisionTree Integration

### Goal: Understand How DecisionTree Fits

**Questions**:

1. What's the current DecisionTree API?
2. How does it consume features?
3. How does it train?
4. How does it integrate with Graph?
5. Does it need SubGraph?

**Tasks**:

1. Review DecisionTree code
2. Identify dependencies
3. Test with simple dataset
4. Document integration points
5. Plan GNN + DecisionTree hybrid (if applicable)

---

## Phase 5: End-to-End Pipeline

### Goal: Complete Training Loop

**Components Needed**:

1. Graph data loading
2. Feature extraction from graph
3. Batch creation (BatchQueue)
4. SubGraph sampling
5. Model forward pass (Variables)
6. Loss computation
7. Backward pass (gradients)
8. Optimizer step
9. Evaluation metrics
10. Model checkpointing

**Minimal Pipeline**:

```rust
struct GNNPipeline {
    graph: Graph,
    model: Box<dyn Variable>,
    optimizer: Optimizer,
    batch_size: usize,
    num_samples: Vec<usize>,  // Neighbors per layer
}

impl GNNPipeline {
    fn train(&mut self, train_nodes: &[u64], epochs: usize) -> Result<TrainingHistory> {
        for epoch in 0..epochs {
            let batches = BatchQueue::consecutive(train_nodes.len(), self.batch_size);

            for batch in batches {
                // 1. Sample subgraph
                let subgraph = self.sample_subgraph(&batch);

                // 2. Extract features
                let features = self.extract_features(&subgraph);

                // 3. Forward pass
                let ctx = ComputationContext::new();
                let predictions = ctx.forward(self.model.as_ref());

                // 4. Compute loss
                let loss = self.loss_fn.apply(&predictions, &targets);

                // 5. Backward pass
                ctx.backward(loss.as_ref());

                // 6. Update weights
                self.optimizer.step(&ctx);
            }
        }

        Ok(TrainingHistory::new())
    }
}
```

---

## Success Criteria

### Phase 1: Graph API Integration

- ✅ Graph API documented
- ✅ RandomGraph utilities working
- ✅ NeighborhoodFunction + RelationshipWeights implemented for Graph
- ✅ SubGraph.build_subgraph() working with real graphs
- ✅ Integration tests passing

### Phase 2: Pipeline Design

- ✅ Java Pipeline system surveyed
- ✅ Rust Pipeline design documented
- ✅ Essential vs optional components identified
- ✅ Minimal pipeline prototype

### Phase 3: DecisionTree Integration

- ✅ DecisionTree API understood
- ✅ Integration points identified
- ✅ Simple test working

### Phase 4: End-to-End

- ✅ Complete training loop working
- ✅ Evaluation metrics
- ✅ Example notebooks/scripts
- ✅ Documentation

---

## Open Questions (To Investigate)

1. **Graph API Ownership**

   - Who owns the Graph during training?
   - Do we clone? Borrow? Arc?
   - Lifetimes in pipeline?

2. **Feature Storage**

   - Where do node features live? (Graph properties? Separate?)
   - How do we batch-load them efficiently?
   - Caching strategy?

3. **Model Serialization**

   - How do we save/load trained models?
   - Weight serialization format?
   - Checkpoint strategy?

4. **Distributed Training**

   - Is this needed now?
   - How does Java GDS handle it?
   - Rust concurrency model?

5. **Performance**
   - What are the bottlenecks?
   - Where do we optimize?
   - Profiling strategy?

---

## Resources to Review

### Rust-GDS Codebase

- `src/types/graph/` - Graph API
- `src/projection/` - GraphStore implementation
- `src/ml/` - Current ML implementation
- `tests/` - Integration tests
- `examples/` - Usage examples

### Java GDS Codebase

- `ml/ml-algo/` - ML algorithms
- `ml/ml-core/` - Core ML abstractions (mostly done!)
- `ml/ml-models/` - Model implementations
- `core/` - Graph operations
- `test-utils/` - Test utilities

### Documentation

- ADRs in `doc/`
- Architecture notes
- API documentation
- Examples and tutorials

---

## Timeline (Rough Estimate)

- **Phase 1** (Graph Integration): 1-2 sessions
- **Phase 2** (Pipeline Survey): 1 session
- **Phase 3** (Minimal Integration): 2-3 sessions
- **Phase 4** (DecisionTree): 1 session
- **Phase 5** (End-to-End): 3-4 sessions

**Total**: ~8-12 work sessions to complete pipeline

**Note**: This is exploratory work - timeline may adjust as we learn

---

## Next Session Action Items

1. ☐ List all Graph-related files in codebase
2. ☐ Read Graph trait/struct definitions
3. ☐ Find RandomGraph utilities
4. ☐ Create simple Graph + SubGraph integration test
5. ☐ Document Graph API capabilities
6. ☐ Identify blockers/problems with current Graph API
7. ☐ Plan minimal NeighborhoodFunction implementation

**Starting Point**: Review what's working, identify what's problematic

---

**"The foundation is complete. Now we connect the pieces."** 🔗
