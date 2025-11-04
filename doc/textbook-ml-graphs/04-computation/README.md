## 04 — Computation (Pregel & Integrations)

Algorithm structure and property store integration. **This is where the course begins!**

- **PageRank** ← **Lecture 1**: Power Iteration algorithm
  - Implementation: `gds/src/procedures/pagerank/pregel_computation.rs`
  - Uses classic Power Iteration: `PR(v) = (1-d) + d × Σ(PR(u) / out_degree(u))`
- pregel_connected_components.rs — ✅ Pregel skeleton
- pregel_propertystore_integration.rs — ✅ Property-backed runs
