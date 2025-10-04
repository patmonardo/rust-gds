# Rust Graph Data Science (Rust-GDS)

Rust-GDS is a modular graph data science toolkit written in Rust with first-class bindings for Node.js/TypeScript. It provides the core primitives needed to describe, store, and traverse property graphs while keeping the runtime fast, deterministic, and easy to embed in larger data platforms.

> **Why Rust-GDS?**
>
> - Strongly-typed graph schema, projection, and property APIs inspired by Neo4j GDS
> - Efficient storage backends that embrace Arrow2, memory-mapped files, and CPU-friendly layouts
> - Deterministic random graph generators for rapid benchmarking and reproducible testing
> - Shared TypeScript surface (`ts-gds/`) generated through N-API bindings for seamless JavaScript integration

---

## Feature highlights

- **GraphStore orchestration** &mdash; A trait-first design that wires schema, id-maps, adjacency, and property stores into a consistent interface.
- **Projection layer** &mdash; Type-safe `NodeLabel` and `RelationshipType` identifiers with interning and wildcard support that mirror the TypeScript API.
- **Property stores** &mdash; Pluggable graph, node, and relationship property containers with `Randomizable` generators for synthetic data.
- **Random graph tooling** &mdash; `types::random_graph_store` produces seeded `DefaultGraphStore` instances ideal for tests and demos.
- **Interoperable bindings** &mdash; `napi` exports the Rust core to Node.js while `ts-gds/` offers idiomatic TypeScript wrappers and helpers.
- **Feature flags** &mdash; Opt-in capabilities (`cursor`, `core`, `arrow`, `io`, `pipeline`) keep builds lean while enabling advanced modules.

---

## Repository layout

| Path           | Purpose                                                                   |
| -------------- | ------------------------------------------------------------------------- |
| `src/`         | Rust crate sources (`projection`, `types`, storage adapters, etc.).       |
| `tests/`       | Integration tests, including random graph API coverage.                   |
| `examples/`    | Minimal Rust entry points demonstrating key APIs.                         |
| `ts-gds/`      | TypeScript client, adapters, and end-to-end tests for the Node ecosystem. |
| `doc/`         | Architectural decision records, module deep dives, and design notes.      |
| `package.json` | Node helpers for building/testing the Rust library via `npm` scripts.     |
| `build.rs`     | N-API build integration glue.                                             |
| `Cargo.toml`   | Crate manifest with feature flags and dependency graph.                   |

---

## Getting started

### Prerequisites

- Rust toolchain (`rustup`, Rust 1.74+ recommended)
- Node.js 18+ (for N-API bindings and TypeScript client)
- `npm` or `pnpm`

### Clone and bootstrap

```bash
git clone <repository-url>
cd rust-gds
npm install              # installs N-API build helpers
cargo build              # debug build of the Rust crate
```

### Run the full Rust test suite

```bash
cargo test
```

### Run TypeScript tests (optional)

```bash
cd ts-gds
npm install
npm test
```

---

## Using the Rust API

Add the crate to a `Cargo.toml` (when published) or use a git dependency. The example below shows how to spin up a random graph store and inspect its topology:

```rust
use rust_gds::types::{random_graph_store, RandomGraphConfig};

fn main() -> anyhow::Result<()> {
	// Seeded config for deterministic graphs
	let config = RandomGraphConfig::seeded(42)
		.with_node_count(10)
		.with_relationship_count(20);

	let graph_store = random_graph_store(&config)?;
	println!("nodes: {}", graph_store.node_count());
	println!("relationships: {}", graph_store.relationship_count());

	Ok(())
}
```

More advanced usage lives in `examples/` and `tests/`:

- `examples/random_graph.rs` &mdash; builds a seeded graph and dumps schema metadata.
- `tests/random_api.rs` &mdash; integration coverage for `random_graph_store`.

Recommended import surface

For downstream code that wants a stable, small surface, prefer importing the curated prelude:

```rust
use rust_gds::types::prelude::*;
```

---

## Feature flags

The crate exposes several optional features. Enable them in your downstream `Cargo.toml` as needed:

| Feature    | Enables                                                         |
| ---------- | --------------------------------------------------------------- |
| `cursor`   | Cursor-first adjacency iterators for high-throughput traversal. |
| `core`     | Core graph infrastructure (implies `cursor`).                   |
| `arrow`    | Arrow2-backed property columns for analytics workloads.         |
| `io`       | I/O utilities for loading/saving graph snapshots.               |
| `pipeline` | Higher-level orchestration pipelines (experiments).             |

Activate features via Cargo:

```bash
cargo build --features core,arrow
```

---

## TypeScript bindings

The `ts-gds/` package mirrors the Rust API surface for consumers who prefer TypeScript. It includes:

- `api/GraphStore.ts` and friends for runtime graph work in Node.js.
- `core/` utilities for building algorithm pipelines.
- Ready-to-run tests that exercise the N-API bridge backed by the Rust implementation.

To rebuild the native addon in watch mode, use:

```bash
npm run build -- --release
```

The generated `.node` binary can then be required/imported by applications or test harnesses.

---

## Documentation

Deep dives and ADRs are located in the `doc/` directory:

- `adr0001_property_graph_store_design.md` &mdash; project philosophy and initial decisions.
- `graphstore_implementation.md` &mdash; trait architecture and design goals.
- `projection_module.md` &mdash; type interners for labels and relationship types.
- `projection_philosophy.md` &mdash; conceptual alignment with the GDS TypeScript project.

New design notes are welcome; follow the existing ADR format when adding significant decisions.

---

## Contributing

1. Fork the repository and create a feature branch.
2. Follow Rust formatting and run `cargo fmt && cargo clippy` before opening a PR.
3. Ensure both Rust and TypeScript tests are green.
4. Document architectural changes in `doc/` or the README.

Bug reports and feature proposals can be filed via GitHub Issues. Please include reproduction steps or sample datasets when relevant.

---

## License

This project is released under the MIT License. See `LICENSE` for details.
