# ADR 0003 — Node Property Value Contract Split (Traits vs Impl)

Status: accepted
Date: 2025-10-02

## Context

- Node property handling was previously implemented in a single module that mixed trait contracts with default concrete structs, making it hard to reason about the public surface while the implementation evolved.
- Upcoming work (HugeSparseArray / compression drivers) will introduce multiple backing stores that must plug into a stable trait layer without refactoring default value logic each time.
- The node value space (scalars, float/double arrays, long arrays) is essentially fixed, whereas the storage backends must remain flexible.
- We need a compact, explicit API surface that mirrors GDS semantics while keeping implementation details swappable.

## Decision

1. Split the node property module into three layers:
   - `traits/`: owns `NodePropertyValues` and the specialized array/scalar traits, plus the boxed `PropertyValues` blanket impl.
   - `impls/`: houses the `Default*NodePropertyValues` structs and their tests.
   - `node_property_values.rs`: becomes a façade that re-exports both the traits and defaults, preserving the existing public API.
2. Update `node/mod.rs` to publish `traits` and `impls`, while continuing to re-export the same type aliases and constructors that downstream code already consumes.
3. Keep the default implementations lightweight and value-focused, leaving future HugeSparseArray/Compression adapters to implement the traits without touching the default module.

## Rationale

- Separating contracts from implementations keeps the trait layer stable as we experiment with new storage backends.
- Re-exporting through the façade avoids churn for existing callers and maintains TS parity (one import path for consumers).
- Tests stay colocated with the concrete defaults, making it easy to validate both the old and any new implementations against the shared trait contract.
- The structure mirrors the `values/abstract` vs `values/primitive` pattern already called out in ADR 0002, giving us a consistent architectural story.

## Consequences

- New backends (HugeSparseArray, compressed adapters, Arrow2/Polars bridges) can implement `traits::*` in isolation and live alongside or instead of `impls::*` without modifying caller code.
- Documentation and IDE navigation now clearly distinguish between API shape and default behavior.
- Slightly more module boilerplate (additional `mod` declarations), but the node property surface is clearer and more maintainable.
- Macro-based expansions for additional `ValueType` combinations can target the `impls/` module directly, keeping trait definitions untouched.

## Follow-up

- Apply the same split pattern to relationship property values before introducing adjacency compression work.
- Design HugeSparseArray/HugeCSR adapters as independent crates or feature-gated modules that implement the new trait layer.
- Document adapter expectations (zero-copy, fallback semantics) once HugeSparseArray prototypes exist.
