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
- Avoid using fully-qualified std:: paths inside function bodies. Prefer module-level imports (use std::fmt::Debug;) so trait methods and types are in scope and uses are consistent.
- Avoid using long module paths directly inside expressions in function bodies (e.g. `crate::types::graph::RelationshipType::of(...)`). Add module-level `use` statements instead so expressions stay concise and imports are explicit.
- Avoid dropping Trait or Struct implementations you need in newly generated code. Always consult the types in src/types and preserve (or explicitly port) required impls when generating or refactoring core code so library behaviour and public APIs remain consistent.
- **Translation policy**: When asked to translate code from TypeScript/Java to Rust, translate EXACTLY what is in the source file. Do not add "helpful" extensions, "simple standalone versions", convenience implementations, or any other additions unless explicitly requested. A translation request means a literal 1:1 mapping of the source material to idiomatic Rust.

## Naming conventions (recommended)

Add the following project-wide naming conventions. Keep changes small and verify with tests.

- Avoid names that end with or use the word `Trait`. Use descriptive domain terms instead:
  - XTrait → XBehavior (behavioral contract)
  - XTrait → XProvider (factory/service)
  - XTrait → XBuilder (construction)
  - XTrait → XSchema (schema-related)
  - Bare `Trait` identifier should be replaced with a descriptive name (e.g., `IdMapBehavior`, `ElementSchema`).
- Interned/boxed factories:
  - Use `::of()` as the public factory for interned/abstracted types (NodeLabel, RelationshipType).
  - Keep small private constructors (`fn new(...)`) for creation inside the module; prefer `of()` for external use.
- Avoid ambiguous `new` for public APIs. Reserve `::new()` only for non-interned constructors or concrete container types (e.g., `Vec::new()`).
- Trait naming:
  - Prefer terms: Behavior, Provider, Builder, Policy, Store, Schema.
  - Avoid generic names like `Handler`, `Thing`, or `Trait` without domain context.
- Error handling and panic policy:
  - No `unwrap()` / `expect()` in library code. Allowed only in tests & small examples with clear messages.
- Use-path and import discipline:
  - Import from top-level module exports only. Never import implementation-file paths.
- std paths and formatting:
  - Avoid fully-qualified `std::` usages inside function bodies; add `use std::fmt::Debug;` at module level.
- Document any deliberate deviations in code comments or an ADR (see `doc/adrXXXX_naming_conventions.md`).

9. How to locate things quickly

- To find the GraphStore implementation or usages, search for `DefaultGraphStore`, `GraphStore`, `random_graph_store`, and `RandomGraphConfig`.
- Examples and tests are the most reliable ground truth for expected behavior.
- Configuration system: Look in `src/config/` for type-safe builder-based configs (see `examples/config_showcase.rs` for usage patterns).

10. Configuration system (src/config/)

- **Purpose**: Type-safe, validated configuration for algorithms, graph construction, and I/O operations.
- **Pattern**: Builder pattern with `ConfigType::builder()...build()` API.
- **Validation**: All configs validate at construction time via `build()` method.
- **Defaults**: Use `ConfigType::default()` for sensible starting values.
- **AI-friendly**: Designed for automated code generation with clear error messages.
- **Key types**:
  - Algorithms: `PageRankConfig`, `LouvainConfig`, `NodeSimilarityConfig`, `BetweennessCentralityConfig`
  - Graph: `GraphCreateConfig`, `PropertyConfig`, `RandomGraphGeneratorConfig`, `RelationshipsBuilderConfig`
  - I/O: `FileExporterConfig`, `FileImporterConfig`, `DatabaseExporterConfig`, `DatabaseImporterConfig`
- **See**: `doc/config_system_implementation.md` and `src/config/README.md` for complete documentation.

11. Quick-check list before a PR

- `cargo fmt && cargo clippy` (run locally).
- Run `cargo test` and, if you touched bindings, `cd ts-gds && npm test`.
- Update or add an example in `examples/` demonstrating API changes.

If anything in this file is unclear or you need deeper guidance on one area (storage drivers, projection interners, N-API bridging, or config system), tell me which part to expand and I will iterate.
