# Root Folder Cleanup - Speculative Stub Removal

**Date**: October 22, 2025  
**Action**: Remove 34 speculative algorithm modules from root  
**Status**: lib.rs cleaned ✅, now delete directories

---

## What Was Removed from lib.rs

```rust
// DELETED - These were exploration artifacts from learning applications/facades
pub mod all_pairs_shortest_path;
pub mod approx_max_k_cut;
pub mod betweenness;
pub mod bridges;
pub mod closeness;
pub mod degree;
pub mod edge_splitter;
pub mod fast_rp;
pub mod graph_coloring;
pub mod graph_sage;
pub mod harmonic;
pub mod hits;
pub mod indirect_exposure;
pub mod influence_maximization;
pub mod k1_coloring;
pub mod k_core;
pub mod k_means;
pub mod kge;
pub mod label_propagation;
pub mod leiden;
pub mod lcc;
pub mod link_prediction;
pub mod louvain;
pub mod modularity_optimization;
pub mod node2vec;
pub mod node_similarity;
pub mod pagerank;
pub mod random_walk;
pub mod scc;
pub mod shortest_path;
pub mod single_source_shortest_path;
pub mod speaker_listener_lpa;
pub mod triangle_count;
pub mod wcc;
pub mod yens_k_shortest_paths;
```

---

## Directories to Delete

These directories in `gds/src/` should be deleted:

```bash
# Run from gds/src/
rm -rf all_pairs_shortest_path/
rm -rf approx_max_k_cut/
rm -rf betweenness/
rm -rf bridges/
rm -rf closeness/
rm -rf degree.rs          # if still exists
rm -rf edge_splitter/
rm -rf fast_rp/
rm -rf graph_coloring/
rm -rf graph_sage/
rm -rf harmonic/
rm -rf hits/
rm -rf indirect_exposure/
rm -rf influence_maximization/
rm -rf k1_coloring/
rm -rf k_core/
rm -rf k_means/
rm -rf kge/
rm -rf label_propagation/
rm -rf leiden/
rm -rf lcc/
rm -rf link_prediction/
rm -rf louvain/
rm -rf modularity_optimization/
rm -rf node2vec/
rm -rf node_similarity/
rm -rf pagerank/
rm -rf random_walk/
rm -rf scc/
rm -rf shortest_path/
rm -rf single_source_shortest_path/
rm -rf speaker_listener_lpa/
rm -rf triangle_count/
rm -rf wcc/
rm -rf yens_k_shortest_paths/
```

---

## After Cleanup

Your `gds/src/` will have **only** conceptual/domain modules:

```
gds/src/
├── procedures/           ← ALL real algorithms (31+)
│   ├── pagerank/
│   ├── degree_centrality/
│   ├── betweenness/
│   ├── louvain/
│   ├── label_propagation/
│   ├── dijkstra/
│   └── [28 more]
│
├── concurrency/          ← Parallel execution
├── core/                 ← Infrastructure
├── pregel/               ← Message passing
├── projection/           ← Evaluation
├── config/
├── types/
├── mem/
├── ml/
├── form/
├── values/
├── collections/
├── errors/
├── logging/
└── termination/          ← Will move here later
```

**14 folders. Zero noise.**

---

## Why This Cleanup Matters

**Before**: 50 folders to scan, 34 are dead stubs
**After**: 14 folders, all meaningful

Every time you open the IDE, you're not drowning in noise. Clean architecture lets your brain focus on the actual work, not context-switching through 34 speculative modules.

---

## Next Steps

1. ✅ Remove declarations from lib.rs (DONE)
2. ⏭️ Delete the 34 directories from gds/src/
3. ⏭️ Run `cargo build` to verify no broken references
4. ⏭️ Commit: "refactor: remove speculative algorithm stubs, consolidate to procedures/"

That's it. Clean architecture achieved.
