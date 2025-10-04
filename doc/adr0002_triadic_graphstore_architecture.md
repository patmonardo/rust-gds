# ADR 0002 — Triadic GraphStore & NativeFactory (Architecture Overview)

Status: proposed
Date: 2025-10-01

## Context
- We are building a Rust-first Graph Data Store inspired by GDS but adapted to a columnar, driver-based architecture.
- The system must expose a stable logical API for graph algorithms (GraphFactory / CSRHugeGraph) while relying on Polars/Arrow as drivers (not as first-class stores).
- Graph workloads require three distinct property axes: NodeStore (unary properties), RelationshipStore (edge-centric, adjacency & edge-table, hyperedges), and Graph-level metadata.
- Java/TS idioms (abstract/primitive) are preserved conceptually; Rust uses traits for abstract contracts and `Default*` implementations for concrete defaults.
- Zero-copy lifetime guarantees (Arrow2 mmap) and feature-gated drivers ("arrow2", "polars") must be explicit.

## Decision
1. Adopt a triadic store model:
   - NodeStore: columnar unary properties + IdMap.
   - RelationshipStore: edge-centric API that supports CSR fast-paths, edge-table reads, and hyperedge endpoints. Expose both edge-centric and adjacency-centric access.
   - Graph-level store: optional columnar metadata.

2. Canonical logical objects:
   - GraphStore: persistent top-level holder of Schema + stores (singleton manager).
   - NativeFactory (ProjectionMaster / RootAgent): orchestrates SmartProjection creation and calls GraphFactory.
   - GraphFactory: produces heavyweight graphs (CSRHugeGraph) used by algorithms.

3. Values package conventions:
   - `values/abstract`: traits (one trait per file).
   - `values/primitive`: default concrete implementations, explicit `Default` prefix (DefaultLongArray, DefaultLongValue, etc.).
   - GdsNoValue singleton exposed as no_value().

4. Drivers and adapters:
   - Implement drivers as clients/adapters under `storage/` (feature-gated).
   - Drivers implement the property-store traits (NodeStore / RelationshipStore / ColumnarStore).
   - Arrow2 adapters must keep an Arc to mmaped buffers or their owning store; adapters are zero‑copy when possible.
   - Polars adapters provide high-level analytic conversions and may copy when necessary.

5. Naming & ownership conventions:
   - Method naming consistent: `as_object()`, `value_type()`, `contains_key()`, `len()`.
   - Traits use concise names (LongArray, IntegralValue). Concrete types use `Default` prefix.
   - Use Arc for shared, cheaply-cloned array backing in defaults.

6. Feature gating and testing:
   - Cargo features: `"arrow2"`, `"polars"`. CI runs both combinations.
   - Unit tests live with modules; cross-driver parity tests live under `tests/` (integration).

## Rationale
- Separating logical GraphStore and physical drivers keeps the core architecture stable while leveraging Polars/Arrow strengths.
- Triadic stores acknowledge that relationship storage is semantically different (adjacency vs edge-table vs hyperedge).
- Maintaining abstract/primitive mapping preserves the GDS mental model and eases porting of algorithms and SDKs.
- Explicit lifetime rules prevent subtle zero-copy bugs across FFI and mmap-backed buffers.

## Consequences
- Clear contracts for algorithm authors and driver implementors.
- Extra boilerplate for RelationshipStore (but necessary for correctness & performance).
- Need for ADRs documenting lifetime guarantees and conversion costs (Arrow→Polars).
- Feature-gated drivers increase maintenance but reduce dependency surface for consumers.

## Implementation notes / checklist
- Implement core traits:
  - `types::properties::ColumnarStore`, `NodeStore`, `RelationshipStore`
  - `values::abstract` traits and `values::primitive::Default*` types
- Create `storage/arrow2_adapter` and `storage/polars_adapter` with feature flags.
- Wire `DefaultValue` into `ValueType` fallback helpers and PropertySchema builders.
- Add `GraphStore` singleton + `NativeFactory` that calls into `GraphFactory`.
- Document and test Arrow2 mmap lifetime contract; provide examples that show zero‑copy access and safe ownership.
- Write parity tests to assert Polars-backed and Arrow2-backed adapters return identical scalar/array results on sample datasets.

## Next steps
- Draft ADRs covering:
  - Arrow2 lifetime & mmap ownership
  - RelationshipStore shapes (CSR vs edge-table vs hyperedge)
  - Naming / trait conventions (values/abstract vs primitive)
  - Feature gating policy
- Implement minimal Arrow2 NodeStore/RelationshipStore skeleton behind `arrow2` feature.
- Implement PrimitiveValues factory wiring to Default* arrays and values.
- Create small integration test comparing Arrow2 vs Polars adapters.
