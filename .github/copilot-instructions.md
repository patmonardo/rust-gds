# Copilot instructions for working in rust-gds

This file contains targeted, actionable guidance for AI coding agents to be productive in this repository.

1. Big picture (quick)

- Rust-GDS is a modular property-graph library written in Rust with N-API/TypeScript bindings in `ts-gds/`.
- Core runtime lives in `src/` (notably `projection/`, `types/`, `values/`); `GraphStore` is the top-level runtime holder.
- Design notes and rationale live in `doc/` (see `adr0002_triadic_graphstore_architecture.md` and `projection_philosophy.md`).

2. Key concepts and project-specific patterns

- Trait-first GraphStore: look for `GraphStore`, `DefaultGraphStore`, and `NativeFactory` across `src/` and `doc/` ADRs.
- Properties are column-oriented: "PropertyValues = columns" (examples/property_showcase.rs). Code expects property stores to attach columns, not scalars.
- Use the curated prelude when importing: `use rust_gds::types::prelude::*;` for stable, minimal surface.
- Feature flags control compile-time surface. `core` implies `cursor`. Enable only features you need to avoid long compile times.
- **Module pattern**: Import from top-level modules only (e.g., `crate::types::properties::Property`), never implementation files (e.g., `crate::types::properties::property::Property`). See `doc/module_organization_pattern.md`.

3. Where to read for the "why"

- `doc/adr0002_triadic_graphstore_architecture.md` — triadic GraphStore and factory pattern.
- `doc/graphstore_implementation.md` — trait architecture and responsibilities.
- `examples/property_showcase.rs` and `examples/graphstore_walkthrough.rs` — runnable demonstrations of idiomatic usage.

4. Build / test / debug commands (concrete)

- Build (debug): `cargo build`
- Build with features: `cargo build --features core,arrow`
- Run an example: `cargo run --example graphstore_walkthrough` (also used by `examples/cr.sh`).
- Run Rust tests: `cargo test`
- Enable backtraces / logging for debugging: `RUST_BACKTRACE=1 RUST_LOG=debug cargo test` or `RUST_LOG=info cargo run --example ...`
- Rebuild TypeScript/N-API native addon: `cd ts-gds && npm install && npm run build -- --release` and then run `npm test` in `ts-gds`.

5. Files to inspect first for any change

- `src/lib.rs` — crate re-exports and top-level modules.
- `Cargo.toml` — features and crate-type (`cdylib` implies native-addon usage).
- `build.rs` and `package.json` — N-API build integration and npm scripts.
- `examples/*` and `tests/*` — canonical usages and integration tests (e.g. `tests/random_api.rs`, `examples/property_showcase.rs`).

6. Tests and deterministic data

- Use `types::random_graph_store` and `RandomGraphConfig::seeded(42)` for deterministic graphs in unit/integration tests (see `tests/random_api.rs`).

7. TypeScript & interop notes

- `ts-gds/` mirrors the Rust API and depends on the native `.node` binary produced by the Rust build.
- N-API wiring is in `build.rs` and `napi` / `napi-derive` in `Cargo.toml`.
- When changing exported symbols, update the TypeScript surface in `ts-gds/api/` and re-run the N-API build.

8. Conventions AI should follow when editing

- Preserve ADRs in `doc/` and follow existing module responsibilities.
- Prefer small, focused PRs that change one subsystem (projection, types, storage) and update `examples/` to show intended use.
- Avoid enabling broad feature sets by default; mirror existing feature usage in `Cargo.toml` and tests.
- **Import discipline**: Always import from top-level `mod.rs` exports. Never use implementation file names in paths (e.g., use `crate::types::PropertyState`, not `crate::types::property_state::PropertyState`).
- **Property trait impls**: Use the standard pattern with explicit `as Arc<dyn PropertyValues>` cast. See `doc/property_trait_implementation_pattern.md`.
- **DefaultValue API**: Use modern lowercase constructors (`DefaultValue::long(42)`, not enum-style `DefaultValue::Long(42)`). See `doc/default_value_api_modernization.md`.

9. How to locate things quickly

- To find the GraphStore implementation or usages, search for `DefaultGraphStore`, `GraphStore`, `random_graph_store`, and `RandomGraphConfig`.
- Examples and tests are the most reliable ground truth for expected behavior.

10. Quick-check list before a PR

- `cargo fmt && cargo clippy` (run locally).
- Run `cargo test` and, if you touched bindings, `cd ts-gds && npm test`.
- Update or add an example in `examples/` demonstrating API changes.

If anything in this file is unclear or you need deeper guidance on one area (storage drivers, projection interners, or N-API bridging), tell me which part to expand and I will iterate.
