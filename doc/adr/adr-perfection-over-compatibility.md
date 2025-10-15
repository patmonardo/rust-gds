# ADR: Prefer Perfection over Compatibility

Status: Proposed

## Context

The `rust-gds` repository contains a large, modular architecture with several
areas where backward compatibility has been prioritized in the past (root-level
shims, duplicated modules across `projection` and `projection::codegen`, etc.).
While compatibility helps migration and incremental refactors, it also creates
noise, ambiguous re-exports, and a fragmented mental model for contributors.

## Decision

Adopt a clear guideline for new refactors and the codegen migration:

- Prefer Perfection over Compatibility for internal module layout and codegen
  placement. That is, centralize macro definitions and generated artifacts in
  `projection::codegen` and avoid root-level forwarding shims unless absolutely
  necessary.

- Prioritize a single canonical location for macros and generated code. Use
  `#[macro_export]` on macros that must be available crate-wide, rather than
  re-exporting modules at the crate root.

- When public API changes are required, prefer an explicit deprecation path
  (deprecated shims with clear TODOs) instead of duplicated, ambiguous
  re-exports.

## Consequences

- Pros:

  - Clearer module ownership and fewer ambiguous re-exports.
  - Easier long-term maintenance and less accidental API surface.
  - Encourages focused codegen and macro placement.

- Cons:
  - Short-term breakage for callers that import old shims. We'll mitigate with
    small deprecation shims and coordinated migration PRs.

## Implementation Notes

- Keep `projection::codegen` as the canonical home for macros like
  `value_type_table!` and for generated artifacts.
- Use `#[macro_export]` for macros that must be referenced across the crate.
- Remove root-level forwarding shims and ambiguous glob re-exports in favor of
  explicit `pub use` lists.

## See Also

- `doc/next_codegen_quick_start.md`
- `src/projection/codegen/eval_macro.rs`
