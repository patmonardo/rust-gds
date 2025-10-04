# ADR 0002  Barrel and Prelude Export Strategy

Status: proposed

Context

The crate currently re-exports many symbols from `types` at the top-level. During early development this "export everything" approach is fine, but as the codebase matures we need a curated, stable public surface that:

- communicates intent to downstream users,
- reduces accidental coupling to internal symbols,
- minimizes churn from refactors (by keeping a consistent prelude),
- provides a clear migration path from "everything exported" to a smaller, curated API.

Decision

Introduce a two-layer export strategy for the `types` module:

1. Barrels (module-level): each logical submodule (for example `types::graph`, `types::properties`, `types::graph_store`) will continue to have a `mod.rs` barrel that exposes the module's user-facing traits and a small set of stable types. Barrels should be conservative — prefer traits and small adapter types over large concrete implementations, unless the concrete type is the canonical default (for example `DefaultGraphStore`).

2. Prelude (crate-level curated surface): a new `types::prelude` module will re-export the commonly-used, stable API chosen from the barrels. The prelude is the recommended import surface for downstream consumers ("use types::prelude::\*;"). The prelude intentionally omits experimental helpers, test utilities, and internal-only modules.

Guidelines

- Traits first: Prefer exposing traits in barrels and re-export them via the prelude. Implementations live under `impls/` and are only re-exported when they are canonical defaults.
- Minimal concrete re-exports: Only export concrete types (e.g., `DefaultGraphStore`) if they are the supported default or widely-used convenience types.
- Stable naming: Changing the prelude is a breaking change; treat prelude changes as released API changes and document them in the changelog.
- Linting: `clippy::module_inception` can be used sparingly on module barrels during migration; the long-term goal is a curated prelude, not blanket suppression.

Consequences

- Consumers get a clear, stable import surface.
- Internal code can be refactored with fewer downstream churns when the prelude remains stable.
- Some initial work to craft the prelude and adjust barrels will be required.

Implementation plan

- Create `src/types/prelude.rs` containing re-exports for the most-used traits and canonical types (Graph, GraphStore, Property traits, DefaultGraphStore, Random utilities, etc.).
- Update `src/types/mod.rs` to publish the `prelude` module.
- Iterate: start small and expand the prelude deliberately when real consumer demand arises.

Notes

This ADR is intentionally conservative: start small, be explicit, and expand deliberately.
