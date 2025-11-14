# DefaultGraphStore backend quickstart

The `DefaultGraphStore` is nothing more than "GraphStore + GraphStoreConfig". The
config decides which Collections backend (`Vec`, `Huge`, `Arrow`) powers every
property store, and the helper methods inside `DefaultGraphStore` ask the config
for a `CollectionsConfig` each time you insert property data.

This note walks through the three practical profiles you can use today:

1. **Vec (default)** – lowest friction, great for small/medium graphs.
2. **Huge** – paged arrays for very large scalar properties.
3. **Arrow** – columnar storage ready for zero-copy / analytics pipelines.

## How the pieces fit together

- `GraphStoreConfig` (in `gds/src/config/graph_store_config.rs`) owns a
  `GraphStorePropertiesConfig` struct. That struct exposes:
  - Default backend per tier (node / relationship / graph properties)
  - Adaptive Huge switching (threshold + on/off)
  - Arrow preference toggle
- `DefaultGraphStore::add_*_property_*` calls
  `config.node_collections_config::<T>(count)` (or the relationship/graph
  equivalent). These helpers wire the backend choice into
  `create_*_backend_from_config`, which returns one of `Vec`, `Huge`, or `Arrow`.
- Converting to runtime adapters is now centralized in
  `build_node_*_property_values` (and graph equivalents). Huge payloads are
  converted to Vec adapters for now, while Arrow payloads stay Arrow-backed.

Because of that flow, _every_ property addition already respects whatever
`GraphStoreConfig` you pass to the constructor—no additional wiring is needed.

## Vec backend (default)

```rust
use rust_gds::config::GraphStoreConfig;
use rust_gds::types::graph_store::default_graph_store::DefaultGraphStore;

let config = GraphStoreConfig::default(); // Vec for every tier until the Huge threshold
let mut store = DefaultGraphStore::new(
    config,
    graph_name,
    database_info,
    schema,
    capabilities,
    id_map,
    relationship_topologies,
);

// Properties inherit the Vec backend automatically
store.add_node_property_i64("age".into(), vec![21, 42, 63])?;
store.add_graph_property_f64("density".into(), vec![0.57])?;
```

### Forcing Vec everywhere

When you never want the adaptive switch to pick Huge (e.g., deterministic CI
runs), call the convenience constructor we just added:

```rust
let config = GraphStoreConfig::vec_only();
let mut store = DefaultGraphStore::new(config, ...);
```

Internally this is equivalent to `GraphStoreConfig::default().force_vec_backends()`:
adaptive switching is disabled, Arrow preference is cleared, and every tier’s
primary backend is locked to `CollectionsBackend::Vec`.

## Huge backend

There are two easy paths to Huge arrays:

1. **Adaptive path (recommended):** keep the default config and push more than
   `properties.huge_array_threshold` (10 million elements by default). The
   `node_collections_config` helper notices the size and returns a `Huge`
   backend. This works for nodes and relationships; graph properties are tiny,
   so they stay Vec.
2. **Explicit path:** set the defaults yourself when constructing the config,
   or lower the threshold for testing:

```rust
use rust_gds::config::{CollectionsBackend, GraphStoreConfig, GraphStorePropertiesConfig};

let properties = GraphStorePropertiesConfig::builder()
    .default_node_backend(CollectionsBackend::Huge)
    .default_relationship_backend(CollectionsBackend::Huge)
    .huge_array_threshold(1_000) // optional: force Huge sooner
    .build()?;

let config = GraphStoreConfig::builder().properties(properties).build()?;
let mut store = DefaultGraphStore::new(config, ...);
```

Once the config says "Huge", the factory returns a `HugeLongArray`/
`HugeDoubleArray`. The property-value builders immediately convert those into
Vec adapters (so downstream APIs keep working) while still gaining the Huge
allocation behavior.

## Arrow backend

Arrow is controlled by the `prefer_arrow` flag or by explicitly choosing Arrow as
one of the default backends.

```rust
use rust_gds::config::{CollectionsBackend, GraphStoreConfig, GraphStorePropertiesConfig};

// Global Arrow preference
let config = GraphStoreConfig::builder()
    .properties(GraphStorePropertiesConfig::builder()
        .prefer_arrow(true)
        .build()?)
    .build()?;

// or fine-grained
let config = GraphStoreConfig::builder()
    .properties(GraphStorePropertiesConfig::builder()
        .default_node_backend(CollectionsBackend::Arrow)
        .default_graph_backend(CollectionsBackend::Vec)
        .build()?)
    .build()?;
```

When `prefer_arrow` is true, the selection logic short-circuits before Huge
checks, so **every** call to `*_collections_config` yields an Arrow backend. Use
this when you know Arrow buffers are available and want the adapters fed with
`ArrowLongArray` / `ArrowDoubleArray` instances.

## Mixing backends per property

You can bypass the global config entirely by building collections configs
manually and passing them to the property-store builders. For example, node
property builders expose `put_*_with_config` helpers that accept a
`CollectionsConfig<T>`; `DefaultGraphStore` mirrors that API via
`add_node_property_*` but you can still insert pre-built `Arc<dyn NodePropertyValues>`
that came from any backend.

```rust
use rust_gds::config::{CollectionsBackend, CollectionsConfigBuilder};
use rust_gds::types::properties::node::impls::default_node_property_store::DefaultNodePropertyStore;

let arrow_config = CollectionsConfigBuilder::<i64>::new()
    .with_backend(CollectionsBackend::Arrow)
    .build();

let store = DefaultNodePropertyStore::builder()
    .put_long_with_config(&arrow_config, "pagerank", vec![1, 2, 3])
    .build();
```

That same pattern works for relationships and graph properties. The unified
Collections API means there is one set of config knobs and one factory surface
regardless of where the values end up.

## Checklist when switching profiles

- Decide if you want **global** behavior (set it on `GraphStoreConfig`) or
  **per-property** overrides (build a `CollectionsConfig<T>` by hand).
- For Huge arrays, confirm the adaptive threshold matches your dataset before
  concluding "Huge isn’t being selected".
- For Arrow, ensure the crate is built with Arrow support (`arrow` feature) and
  that you actually supply enough data—Arrow adapters expect contiguous buffers.
- Remember you can always call `GraphStoreConfig::vec_only()` to reset to the
  most conservative profile after experimentation.

With those three scenarios covered (Vec, Huge, Arrow), you can test every backend
combination without touching the `DefaultGraphStore` internals—simply create the
config you want and hand it to the constructor.
