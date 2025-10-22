# Pathfinding Facade Translation Plan

## Goals
- Provide idiomatic Rust facades for pathfinding algorithms with four modes: stream, stats, mutate, write.
- Expose entrypoints on `Graph` (e.g., `graph.dijkstra()`, `graph.bfs()`), backed by builders.
- Strongly-typed inputs/outputs; iterator-based streaming; config validation; optional memory estimation.
- **Avoid Java ceremony**: No massive interfaces, no Neo4j-specific types, no inheritance hierarchies.

## Scope (initial)
- Dijkstra, BFS, DFS, A*, YenKShortest, AllPairs/APSP (where present), DeltaStepping (if available).

## API Shape (per algorithm)
- `impl Graph { fn dijkstra(&self) -> DijkstraBuilder; /* etc. */ }`
- Builder config examples:
  - Dijkstra: `source`, `target`(optional), `relationship_weight`, `direction`, `cutoff`(optional)
  - BFS/DFS: `source`, `target`(optional), `direction`, `cutoff`
  - A*: `source`, `target`, `heuristic` (predefined or custom), `direction`, `cutoff`
- Modes:
  - `stream() -> impl Iterator<Item = PathResult>`
  - `stats() -> PathStats`
  - `mutate(property: &str) -> MutationResult`
  - `write(property: &str) -> WriteResult`
  - `estimate() -> MemoryEstimate` (optional)

## Types
- Reuse `PathResult`, `PathStats`, `MutationResult`, `WriteResult` from `facades::traits` and `builder_base`.
- Add pathfinding-specific stats fields (e.g., `expanded_nodes`, `visited_edges`, `heuristic_estimates_used`).

## Validation
- `ConfigValidator` checks: presence of `source`, compatibility of `direction`, positive weights for Dijkstra/A*.
- Graph invariants: existence of source/target ids, weight property type.

## Wiring to Specs
- Call algorithm Specs under `gds/src/procedures/pathfinding/*/spec.rs` via executor:
  - Use executor in `projection::eval::procedure` with `ExecutionMode::{Stream, Stats, MutateNodeProperty, Write}`.
  - Map engine results to facade types.

## Result Mapping
- Engine path → `PathResult { nodes: Vec<NodeId>, total_weight: f64, length: usize }` (adjust per algo).
- Stats → `PathStats { runs: u64, expanded_nodes: u64, time_ms: u64, converged: bool }`.

## Estimation
- If spec exposes estimation, surface as `estimate()` returning `MemoryEstimate { bytes, details }`.

## Test Plan
- Unit tests per builder: validation, defaults, edge cases (missing source, negative weight property).
- Integration tests (`gds/tests/facade_pathfind_integration.rs`):
  - Small graphs with known shortest paths; verify stream path and stats.
  - Property write/mutate: check property persistence.

## Milestones

### ✅ M1: Dijkstra Facade Complete
- **Status**: ✅ IMPLEMENTED AND WIRED
- **Features**: Full builder pattern with 4 execution modes (stream, stats, mutate, write)
- **Configuration**: source, targets, weight_property, direction, track_relationships, concurrency
- **Results**: Proper `PathResult` iterator with node paths and costs
- **Tests**: 12 comprehensive unit tests covering validation and edge cases
- **Code**: ~470 lines of clean, documented Rust code

### ✅ M2: BFS Facade Complete
- **Status**: ✅ IMPLEMENTED AND WIRED
- **Features**: Full builder pattern with 4 execution modes (stream, stats, mutate, write)
- **Configuration**: source, targets, max_depth, track_paths, concurrency, delta
- **Results**: Proper `PathResult` iterator with BFS distances (unweighted paths)
- **Tests**: 14 comprehensive unit tests covering validation and edge cases
- **Code**: ~474 lines of clean, documented Rust code
- **Key Differences**: Unweighted traversal, max_depth control, BFS-specific stats (nodes_visited, branching_factor)

### ✅ M3: DFS Facade Complete
- **Status**: ✅ IMPLEMENTED AND WIRED
- **Features**: Full builder pattern with 4 execution modes (stream, stats, mutate, write)
- **Configuration**: source, targets, max_depth, track_paths, concurrency
- **Results**: Proper `PathResult` iterator with DFS discovery order and depths
- **Tests**: 14 comprehensive unit tests covering validation and edge cases
- **Code**: ~481 lines of clean, documented Rust code
- **Key Differences**: Depth-first traversal, backtracking stats, discovery order tracking

### ✅ M4: A* Facade Complete
- **Status**: ✅ IMPLEMENTED AND WIRED
- **Features**: Full builder pattern with 4 execution modes (stream, stats, mutate, write)
- **Configuration**: source, targets, weight_property, heuristic (Manhattan, Euclidean, Haversine, Custom), concurrency
- **Results**: Proper `PathResult` iterator with optimal paths and A* cost estimates
- **Tests**: 15 comprehensive unit tests covering validation, heuristics, and edge cases
- **Code**: ~573 lines of clean, documented Rust code
- **Key Features**: Multiple heuristic types, heuristic accuracy metrics, priority queue simulation

### M5: Bellman-Ford Facade (Next)
- Handle negative weight cycles and shortest paths
- Add cycle detection and negative cycle reporting
- Support for relaxation-based shortest paths

### M3: A* Facade with Heuristics
- Add heuristic enum (Manhattan, Euclidean, custom closure)
- Implement A* builder with heuristic configuration
- Support for geographic and custom distance functions

### M4: Advanced Pathfinding
- K-Shortest Paths (Yen) facade if applicable
- All-Pairs Shortest Paths facade
- Delta-Stepping facade for large graphs

## Java vs Rust: Why We're Not Translating Ceremony

### What Java Facade-API Forces (Ceremony We Skip)
- **Massive Interface**: `PathFindingProcedureFacade` has 100+ methods (one per algorithm×mode)
- **Neo4j-Specific Types**: `PathFindingStreamResult` uses Neo4j's `Path` class and `GraphDatabaseService`
- **Inheritance Hierarchy**: `PathFindingMutateResult extends StandardMutateResult extends AbstractResult`
- **Builder Pattern Overkill**: `AbstractResultBuilder<PathFindingMutateResult>` with `build()` ceremony
- **Configuration Ceremony**: Complex `Map<String, Object>` configs with validation scattered everywhere

### What We Build Instead (Rust-First)
- **Simple Structs**: `DijkstraBuilder` with 4 methods: `stream()`, `stats()`, `mutate()`, `write()`
- **Rust Iterators**: `stream() -> impl Iterator<Item = PathResult>` - memory efficient, composable
- **Strong Types**: `PathResult { nodes: Vec<NodeId>, total_weight: f64, length: usize }`
- **Fluent Config**: `.source(42).target(99).weight_property("cost").stream()`
- **Validation**: `ConfigValidator` checks upfront, returns `Result<T, AlgorithmError>`

### Why This Works Better For Us
- **Less Code**: ~50 lines per facade vs 100+ Java methods per algorithm category
- **Type Safety**: Rust compiler catches config errors at build time
- **Performance**: Iterators avoid materializing large result collections
- **Composability**: Chain operations naturally (`.filter().map().collect()`)
- **No Neo4j Coupling**: Pure Rust, works with any graph storage

## Implementation Notes
- Keep proc-layer ceremony out; facades call specs directly.
- Add convenience on `Graph` after Dijkstra lands.
- Focus on algorithm logic, not facade infrastructure.
