# EVAL_MACRO_SPEC

Date: 2025-10-10  
Scope: Specification for the `eval!` macro (AK‑Macro) — initial implementation in @gds with extraction plan to @reality.

## Purpose (concise)

The `eval!` macro is the canonical code generator that projects a single source-of-truth PropertyDescriptor (Svarūpa) into:

- storage adapters (Rūpa),
- runtime/primitive types (Nāma),
- and optional computation species (e.g., BSP/Pregel).

Primary goals:

- Single source of truth (PropertyDescriptor).
- Two-way projection: PropertyStore ↔ Runtime (FormProcessor / Functors).
- Backend-agnostic generation (HugeArray, Arrow, Sparse via trait hooks).
- Safety-first: NO unchecked `as usize` conversions — use `checked_u64_to_usize`.
- Incremental rollout: implement in `@gds`, perfect, then extract to `@reality`.

## High-level DSL (informal)

eval! {
schema: {
name: "string",
elements: [
{ id: "name", type: <long|double|bool|string|long_array|double_array>, default: <lit>, property_source?: "col_name" },
...
]
},
computation?: BSP { pattern: VertexCentric, init: "init_fn", compute: "compute_fn", aggregator?: "agg", partitioner?: Auto|Name },
runtime?: { backend?: Auto|HugeArray|Arrow|Sparse, check_indexing?: true|false }
}

Notes:

- `schema` required. `computation` and `runtime` optional.
- `init` / `compute` may be references to existing Rust functions (preferred) or strings for later-phase codegen.

## Concrete responsibilities of generated code

1. Schema & registry

   - Generate a static schema struct and register it in a runtime registry for dynamic lookup.
   - Provide accessors for defaults: use lowercase DefaultValue constructors (e.g., `DefaultValue::double(1.0)`).

2. FormProcessor helpers (safety core)

   - `checked_u64_to_usize(id: u64) -> Result<usize, ProjectionError>`
   - Widen/narrow functors for supported types.
   - Unit tests for overflow and 32-bit semantics.

3. PropertyProjection / Materializers

   - `materialize_from_property_store(graph: &Arc<dyn Graph>, schema: &Schema, node_values: &mut NodeValueStore) -> Result<(), ProjectionError>`
   - `materialize_to_property_store(...)`
   - Use `graph.node_properties()` via `NodePropertyContainer` trait; import required trait bounds in generated code.

4. Backend hooks

   - Emit trait-based adapters (e.g., `trait BackendColumn { fn get_double(&self, id: u64) -> Option<f64>; }`).
   - Generated code calls `backend::current()` or `graph.backend()` to select adapter at runtime.

5. Computation species (optional)

   - Generate skeletons only in Phase 0: struct + trait impl with calls to user-supplied `init`/`compute`.
   - Phase progression: skeleton → partial codegen → full-body generation with string bodies only after review.

6. Diagnostics & compile-time checks
   - Assert unique element ids at macro expansion.
   - Emit clear compile-time errors for unsupported types or missing property mappings.

## Safety rules (mandatory)

- No unchecked `as usize` conversions. All index conversions call `checked_u64_to_usize`.
- Generated library code must return `Result` for recoverable errors; avoid `unwrap()` / `expect()`.
- Use `use rust_gds::types::prelude::*;` in generated modules.
- Ensure trait imports that are part of `Graph` composition (e.g., `IdMap`) are imported or qualified in generated files.
- Keep generated imports minimal and from top-level module exports only.

## Expansion model & examples

### Validation (macro-time)

- Ensure `schema.name` is a valid identifier.
- Ensure element ids are unique.
- Ensure `default` is compatible with `type`.
- If `property_source` provided, emit runtime check that column exists (materializer returns descriptive error if absent).

### Example input

```rust
eval! {
  schema: {
    name: "page_rank",
    elements: [
      { id: "rank", type: double, default: 1.0, property_source: "seed_rank" },
      { id: "active", type: bool, default: true }
    ]
  },
  computation: BSP {
    pattern: VertexCentric,
    init: "init_rank",
    compute: "compute_rank",
    partitioner: Auto
  },
  runtime: { backend: Auto, check_indexing: true }
}
```

### Sketch of generated fragments

- `pub struct PageRankSchema;` + `PROPERTY_REGISTRY` entry.
- `pub fn checked_u64_to_usize(id: u64) -> Result<usize, ProjectionError> { ... }`
- `pub fn materialize_from_property_store(...) -> Result<(), ProjectionError> { ... }`  
  iterates 0..graph.node_count(); uses `checked_u64_to_usize(id as u64)?` for node->index conversions.
- If computation provided: `pub struct GeneratedPageRank { schema: Schema, config: PregelConfig }` and `impl Computation for GeneratedPageRank { fn init(&self, ctx: &mut InitContext) { init_rank(ctx) } ... }`

## Testing & CI

- Unit tests:
  - `checked_u64_to_usize` edge cases.
  - Functor round-trip conversions (DefaultValue ↔ PropertyValue).
  - Materializer round-trip: materialize_from_property_store -> materialize_to_property_store.
- Integration example:
  - `examples/pregel_propertystore_eval_demo.rs` — default graph, seed property, run generated Pregel skeleton, assert loaded values.
- CI:
  - Add job that compiles `eval!` examples with `--features core`.
  - Run clippy and fmt on generated outputs (macro emits tidy imports).

## Phase plan (practical)

- Phase 0 (now, @gds): implement safe, conservative generation: FormProcessor + Projection + schema registry + tests. No full body computation generation.
- Phase 1 (weeks 1–2): Backend hooks and index-safety sweep; add example and CI integration.
- Phase 2 (weeks 3–6): Computation skeleton generation + user fn wiring; optimize codegen patterns.
- Phase 3 (weeks 7–10): Performance tuning; extract stable API to `@reality` package; provide cross-language bindings for GDSL/TS.

## Migration checklist (short)

- [ ] `doc/EVAL_MACRO_SPEC.md` checked-in (this file).
- [ ] Implement `checked_u64_to_usize` and unit tests in `src/projection/form_processor.rs`.
- [ ] Create `examples/pregel_propertystore_eval_demo.rs`.
- [ ] Add macro examples to repository and CI job to compile them with `--features core`.
- [ ] Run `cargo fmt && cargo clippy` and fix issues.

## Implementation notes (developer guidance)

- Follow repo import discipline: import from top-level exports only.
- Generated code must prefer `crate::types::prelude::*`.
- Do not enable broad features by default in generated code.
- Keep generated modules small; emit `#[allow(dead_code)]` for staged outputs where necessary.
- Document any deviations in an ADR.

---

This spec is intentionally pragmatic and minimal for Phase 0. It captures the required safety rules, the DSL surface, generated artifacts, and the extraction path to `@reality`. Implement `FormProcessor` (checked_u64_to_usize) first, add the runnable example next, then iterate on computation species generation.
