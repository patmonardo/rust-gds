# ADR 0005: Values System Architecture and Macro Strategy

Status: Revised Draft
Date: 2025-10-10

## Purpose

Clarify the Values system architecture, record the separation between columnar storage views (PropertyValues) and individual value access (GdsValue / PrimitiveValues), and propose a lightweight "Eval Macro" orchestration strategy to generate and integrate both macro families consistently.

## Context

We evolved from an early mixed implementation where `PropertyValue`-style hacks and ad-hoc Vec-backed code proliferated. Two coherent macro-driven subsystems emerged:

- PropertyValues macros: generate storage-view adapters (columnar, u64-indexed) over backends.
- PrimitiveValues / GdsValue macros: generate runtime value drivers and factories for individual values and arrays.

Both systems must align on the same `ValueType` canonical enum and interoperable trait surfaces so that Projection/Graph layers and Pregel evaluation can be deterministic, efficient, and maintainable.

## Problem Statement

1. The current code has duplicated, hand-written implementations for many primitive/array types. This increases maintenance burden and risk of inconsistent behavior.
2. The boundary between storage (columnar PropertyValues) and runtime (GdsValue/NodeValue) has been fuzzy; a lingering `PropertyValue = f64` hack demonstrates the drift.
3. We need a clear, maintainable way to generate: (a) backends and their adapters (PropertyValues) and (b) runtime value drivers and factories (PrimitiveValues/GdsValue), while keeping them consistent and discoverable.

## Decision

We adopt a three-part strategy:

1. Keep `GdsValue` as the canonical runtime semantic model. Macros in the PrimitiveValues family generate concrete `GdsValue` impls (scalars and arrays) and the `PrimitiveValues` factory.

2. Keep `PropertyValues` as the canonical storage/view model. Macro-generated `TypedPropertyValues<B, T>` adapters wrap `ArrayBackend<T>` backends (Huge, Arrow, Mmap, Sparse) and expose a `PropertyValues` trait that uses `u64` ids at its public boundary and returns `Option<Arc<dyn GdsValue>>` for reads.

3. Introduce a small, lightweight "Eval Macro" orchestrator (a higher-level macro_rules module) whose job is to drive both macro families from a single source of truth (the `ValueType` table). The Eval Macro will:
   - centrally declare the list of value variants and canonical Rust storage types (i64, f64, i32, f32, String, etc.),
   - generate invocations for `gds_value_impl!` (PrimitiveValues) and `generate_property_values!` (PropertyValues adapters), and
   - emit the `PrimitiveValues::of()` factory arms and `PropertyValues` adapter registrations consistently.

## Rationale

- A single source of truth reduces duplication: adding a ValueType variant flows to both runtime and storage code paths.
- PrimitiveValues macros stay the prioritized family (they produce `GdsValue` drivers that are fundamental to projection and cursor code). PropertyValues adapters are thin wrappers that expose storage via the agreed `PropertyValues` trait.
- The Eval Macro is a code-generation convenience: it does not hide complexity but ensures consistent, discoverable, and testable outputs across macro sets.

## Traits and Boundaries

We standardize three trait surfaces (minimal, stable contract):

1. `ArrayBackend<T>` — storage abstraction implemented by Huge/Arrow/Mmap/Sparse backends.

   - `len(&self) -> usize`
   - `get(&self, idx: usize) -> T` (or typed slice access)
   - `set(&mut self, idx: usize, value: T) -> Result<(), BackendError>`
   - `contiguous_slice(&self, offset, len) -> Option<&[T]>`
   - `chunk_iter()` for page/chunk iteration

2. `PropertyValues` — public storage view (u64 boundary):

   - `value_type(&self) -> ValueType`
   - `len(&self) -> usize`
   - `get_u64(&self, id: u64) -> Option<Arc<dyn GdsValue>>`
   - `set_u64(&mut self, id: u64, value: Arc<dyn GdsValue>) -> Result<(), Error>`
   - single checked `u64 -> usize` conversion at the boundary; explicit error on overflow

3. `GdsValue` and its `Array` subtraits — runtime/value drivers produced by PrimitiveValues macros. Additional helpers added:
   - `contiguous_slice` / `chunk_iter` to support zero-copy kernels and paged processing
   - `is_nullable()` to indicate presence of nulls

## Pregel and NodeValue

`NodeValue` remains a compact, Pregel-specific runtime enum optimized for compute and messaging. Pregel reads properties exclusively via `PropertyValues` and converts them via `NodeValue::from_property(&impl PropertyValues, node_id: u64)` using conservative type mapping (no silent coercion; explicit helper casts provided).

## Eval Macro Orchestration

The Eval Macro (a small macro_rules file) will be authoritative for the ValueType table. Note: this is a compile-time code generation macro (not runtime eval). Example usage:

```
value_type_table! {
   Long => { rust: i64, gds_variant: Long, property_macro: generate_long_property_values },
   Double => { rust: f64, gds_variant: Double, property_macro: generate_double_property_values },
   String => { rust: String, gds_variant: String, property_macro: generate_string_property_values },
   // ... other entries ...
}
```

The Eval Macro will expand into invocations of `gds_value_impl!` and `generate_property_values!` for each row. This keeps both macro families in sync and makes it easy to add or remove ValueType variants.

## Consequences

Positive

- Single authoritative ValueType table reduces drift and duplication.
- Macro-generated code will be consistent and maintainable; PrimitiveValues macros remain the primary drivers for runtime behavior.
- Backends can be added without changing projection or pregels — only adapter generation changes.

Risks/Challenges

- Macro complexity increases slightly; we must keep macro expansions readable and well-tested.
- Debugging macro-generated code requires good naming and test coverage.
- Migration work: adapt existing hand-written `Default*` implementations to macro outputs and remove `PropertyValue=f64` compatibility hacks.

## Implementation Plan (practical)

1. Add `src/collections/array_backend.rs` with `ArrayBackend<T>` and `BackendError`.
2. Extend `src/values/traits/gds_value.rs` with contiguous/chunk helpers and `is_nullable`.
3. Implement the Eval Macro table in `src/values/value_type_table.rs` (macro_rules) and wire it into `src/values/macros.rs`.
4. Implement `gds_value_impl!` and `gds_value_factory!` (PrimitiveValues) to consume the table.
5. Implement `generate_property_values!` (PropertyValues adapters) — start with Huge and Arrow backends.
6. Add `NodeValue::from_property` and remove the remaining `PropertyValue=f64` usage (compat shim allowed briefly).
7. Add unit tests for each macro-generated artifact, plus a smoke test that ensures a `ValueType` entry produces both a `GdsValue` impl and a `PropertyValues` adapter.

## Next Steps

- Decide migration policy for existing hand-written implementations (replace in-place vs phased coexistence).
- Implement the Eval Macro table and a single prototype variant (Long) to validate the flow.
- Add ADR note on u64→usize conversion policy (fail explicitly / central helper).

## References

- ADR 0001: Property Graph Store Design
- ADR 0003: Node Property Value Contract (PropertyValue enum removal)
- ADR 0004: Property Cursors (First Macro System)
- ADR 0003 (macro array runtime): macro-based HugeArray runtime ADR
- Arrow2 documentation: Individual value extraction from IPC

## Key insight

Macro generation is a first-class architectural tool in rust-gds. We keep two focused macro families — PrimitiveValues macros (produce `GdsValue` runtime drivers) and PropertyValues macros (produce storage adapters) — and add a small, authoritative Eval Macro to drive both from a single ValueType table. This reduces drift, improves discoverability, and makes adding new types/backends low-friction.

Terminology note: in our triadic framing we use **Noumenal = Pure Form** for the immutable ValueType table and **Transcendental = Pure Nama** for the Form Processor (the policy/orchestration layer). A geometric "triangle" may be thought of as a Pure Form in this sense: a minimal, immutable schema shape. The Eval Macro lives in the Transcendental (Pure Nama) role — it projects the Noumenal table into the two Phenomenal species (Subtle and Gross) and supplies the hooks the Form Processor needs. In practice, the macro-driven Form Processor may also host the `Eval()` hook used by projection/evaluation code.
