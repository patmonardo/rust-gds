# Next Session Plan - Facade Integration & Testing

**Date Written**: October 22, 2025  
**Previous Session**: Infrastructure + 3 prototype facades complete  
**Status**: Ready for integration testing and pattern extension

---

## 🎯 Session Goals

Move from **infrastructure** to **real functionality**:

1. Write integration tests connecting facades to actual algorithm specs
2. Verify stream/stats/mutate modes actually work
3. Extend pattern to remaining 28 facades
4. Build confidence in the design

---

## 📋 Immediate Tasks (First 2 Hours)

### Task 1: Create Integration Test Framework

Create `gds/tests/facade_integration_tests.rs`:

```rust
#[cfg(test)]
mod facade_integration_tests {
    use gds::Graph;
    use gds::procedures::facades::centrality::*;
    
    fn create_test_graph() -> Graph {
        // Simple graph: 0--1--2--3
        //              |        |
        //              +--------+
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(0, 3);
        graph
    }
    
    #[test]
    fn test_degree_centrality_on_simple_graph() {
        let graph = create_test_graph();
        let facade = DegreeCentralityFacade::new();
        
        let results: Vec<_> = facade.stream()
            .expect("stream should succeed")
            .collect();
        
        // Verify node 0 has degree 2
        // Verify node 1 has degree 2
        // Verify node 2 has degree 2
        // Verify node 3 has degree 2
        assert!(results.len() > 0);
    }
    
    #[test]
    fn test_pagerank_convergence() {
        let graph = create_test_graph();
        let builder = PageRankBuilder::new()
            .iterations(100)
            .tolerance(1e-6);
        
        let stats = builder.stats()
            .expect("stats should succeed");
        
        // Verify convergence happened
        assert!(stats.converged);
        assert!(stats.iterations_ran < 100); // Should converge early
    }
    
    #[test]
    fn test_betweenness_identifies_bridges() {
        let graph = create_test_graph();
        let builder = BetweennessBuilder::new();
        
        let stats = builder.stats()
            .expect("stats should succeed");
        
        // Node 1 and 2 should have higher betweenness (they're bridges)
        assert!(stats.bridge_nodes > 0);
    }
}
```

**Steps**:
1. Create test file
2. Set up simple test graph
3. Run each facade mode
4. Verify results make sense

---

### Task 2: Integration with Algorithm Specs

Currently, facades return empty results (TODO). Connect them:

```rust
// In degree_centrality.rs
pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
    let spec = DEGREE_CENTRALITYAlgorithmSpec::new("graph".to_string());
    let config = serde_json::json!({});
    let context = ExecutionContext::new(1000, 5000); // TODO: Get from graph
    
    let result = spec.execute(&context, &config)?;
    
    Ok(Box::new(result.scores.into_iter()
        .enumerate()
        .map(|(i, score)| CentralityScore {
            node_id: i as u64,
            score,
        })))
}
```

**Status**: Depends on getting graph reference into facades

---

## 🚀 Medium Tasks (Next 3 Hours)

### Task 3: Extend to Remaining Centrality Algorithms

Apply pattern to:
- Closeness Centrality
- Harmonic Centrality  
- HITS

Each will be similar to our prototypes. Use PageRank as template for iterative ones.

```
gds/src/procedures/facades/centrality/
├── closeness.rs            [~150 lines]
├── harmonic.rs             [~150 lines]
└── hits.rs                 [~200 lines - more complex]
```

---

### Task 4: Module Re-exports

Update `centrality/mod.rs`:

```rust
pub mod degree_centrality;
pub mod pagerank;
pub mod betweenness;
pub mod closeness;
pub mod harmonic;
pub mod hits;

pub use degree_centrality::DegreeCentralityFacade;
pub use pagerank::PageRankBuilder;
pub use betweenness::BetweennessBuilder;
pub use closeness::ClosenessBuilder;
pub use harmonic::HarmonicBuilder;
pub use hits::{HitsBuilder, HitsStats};
```

---

## 📚 Follow-Up Tasks (Optional)

### Community Facades

Create `community/` facades after centrality is solid:

```
community/
├── louvain.rs                [Builder - iterative]
├── label_propagation.rs      [Builder - iterative]
├── wcc.rs                    [Simple facade - no config]
├── local_clustering_coeff.rs [Simple]
└── triangle_count.rs         [Simple aggregation]
```

Each teaches about community detection:
- Louvain: Modularity optimization
- Label Propagation: Message passing
- WCC: Component detection
- LocalClusteringCoeff: Local density
- TriangleCount: Clustering structure

### Pathfinding Facades

Create `pathfinding/` facades:

```
pathfinding/
├── dijkstra.rs           [Single-source shortest]
├── bfs.rs                [Breadth-first]
├── dfs.rs                [Depth-first]
├── astar.rs              [Heuristic search]
├── bellman_ford.rs       [Negative weights]
└── delta_stepping.rs     [Parallel shortest]
```

These teach about path-finding trade-offs.

---

## 🎓 Testing Strategy

### Unit Tests
- Configuration validation (already done for PageRank)
- Default values
- Builder pattern
- Error cases

### Integration Tests
- Real graph execution
- Result correctness
- Performance characteristics
- Multiple modes (stream/stats/mutate)

### Example Integration Test

```rust
#[test]
fn test_pagerank_and_degree_correlation() {
    let graph = create_realistic_graph();
    
    // PageRank should give higher scores to highly connected nodes
    let pr_stats = PageRankBuilder::new().stats().unwrap();
    let degree_stats = DegreeCentralityFacade::new().stats().unwrap();
    
    // They should be similar (not identical - different models)
    assert!((pr_stats.mean - degree_stats.mean).abs() < 0.5);
}
```

---

## ✅ Checklist for Next Session

- [ ] Create integration test framework
- [ ] Test DegreeCentrality with real graph
- [ ] Test PageRank convergence
- [ ] Test Betweenness bridge detection
- [ ] Connect facades to algorithm specs
- [ ] Implement Closeness Centrality facade
- [ ] Implement Harmonic Centrality facade
- [ ] Implement HITS facade
- [ ] Update module re-exports
- [ ] Run all tests (unit + integration)
- [ ] Document any pattern changes

---

## 📊 Success Criteria

✅ **Code Quality**:
- All facades compile without errors
- Integration tests pass
- >80% code coverage

✅ **Functionality**:
- Stream mode returns correct results
- Stats mode computes valid statistics
- Mutate mode stores properties
- Configuration validation works

✅ **Documentation**:
- Every facade has doc examples
- Statistics are explained
- Performance characteristics documented

✅ **Pattern**:
- All 9 centrality facades follow same pattern
- Ready to extend to 22 more algorithms
- Traits are stable (no changes needed)

---

## 🎯 After This Session

Once facades are working:
1. **Week 2**: Implement remaining algorithms
2. **Week 3**: Performance optimization
3. **Week 4**: Full Gamma status

---

## 📝 Notes

- The `Result` type in `traits.rs` should work for all facades
- Each facade's `stats()` method teaches about algorithm properties
- Configuration validation via `ConfigValidator` prevents user errors
- Test both success and failure cases
- Document trade-offs (speed vs accuracy, memory vs time, etc.)

**Key Insight**: Each facade is a learning opportunity - understand what matters most about that algorithm and expose it in the API!

---

**Next Step**: Create integration test file and start connecting facades to specs. Good luck! 🚀

