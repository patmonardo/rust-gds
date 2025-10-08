# Configuration System

Type-safe, builder-based configuration system for rust-gds graph operations.

## Overview

The configuration system provides a clean, idiomatic Rust API for configuring:

- **Algorithms** (PageRank, Louvain, Node Similarity, Betweenness Centrality)
- **Graph Construction** (Random graphs, property mappings, relationships)
- **I/O Operations** (File import/export, database operations)

## Design Principles

1. **Type Safety** - Compile-time validation via Rust's type system
2. **Sensible Defaults** - All configs implement `Default` with reasonable values
3. **Builder Pattern** - Fluent API for complex configurations
4. **Runtime Validation** - Additional checks with clear error messages
5. **Zero-Cost Abstractions** - Configs are simple structs with no runtime overhead
6. **AI-Friendly** - Designed for automated code generation and agent workflows

## Architecture

```
config/
├── mod.rs              - Module organization and re-exports
├── base_types.rs       - Core configuration traits and base types
├── algo_config.rs      - Algorithm configurations (PageRank, Louvain, etc.)
├── graph_config.rs     - Graph construction configurations
├── io_config.rs        - Import/export configurations
└── validation.rs       - Validation utilities and error types
```

## Usage Examples

### Algorithm Configuration

```rust
use rust_gds::config::*;

// Simple PageRank with defaults
let pagerank = PageRankConfig::default();

// Custom configuration
let custom = PageRankConfig::builder()
    .max_iterations(50)
    .damping_factor(0.9)
    .tolerance(0.00001)
    .concurrency(8)
    .build()?;
```

### Graph Creation

```rust
// Random graph
let random = RandomGraphGeneratorConfig::builder()
    .node_count(10000)
    .average_degree(15.0)
    .seed(42)
    .build()?;

// Named graph with properties
let graph = GraphCreateConfig::builder(String::from("my_graph"))
    .node_projection(vec![String::from("Person")])
    .node_properties(vec![String::from("age")])
    .build()?;
```

### I/O Configuration

```rust
// File export
let export = FileExporterConfig::builder()
    .export_path(String::from("/tmp/graph"))
    .compression_enabled(true)
    .compression_level(9)
    .build()?;

// Database import
let import = DatabaseImporterConfig::builder()
    .database_name(String::from("neo4j"))
    .node_query(String::from("MATCH (n) RETURN n"))
    .build()?;
```

### Validation

All configurations validate at construction time:

```rust
let result = PageRankConfig::builder()
    .damping_factor(1.5) // Invalid: must be 0-1
    .build();

match result {
    Ok(config) => { /* use config */ }
    Err(ConfigError::OutOfRange { name, min, max, value }) => {
        println!("Error: {} must be between {} and {}, got {}",
                 name, min, max, value);
    }
    _ => {}
}
```

## Configuration Types

### Base Traits

- `Config` - Marker trait for all configurations
- `ConcurrencyConfig` - Concurrency settings
- `WriteConfig` - Write operation settings
- `IterationsConfig` - Iterative algorithm settings
- `RelationshipWeightConfig` - Weighted relationship settings

### Algorithm Configs

- `PageRankConfig` - PageRank centrality
- `LouvainConfig` - Louvain community detection
- `NodeSimilarityConfig` - Node similarity computation
- `BetweennessCentralityConfig` - Betweenness centrality

### Graph Configs

- `GraphCreateConfig` - Graph creation from projections
- `PropertyConfig` - Property definitions and aggregation
- `RandomGraphGeneratorConfig` - Random graph generation
- `RelationshipsBuilderConfig` - Relationship topology construction

### I/O Configs

- `FileExporterConfig` - Export to files (CSV, binary)
- `FileImporterConfig` - Import from files
- `DatabaseExporterConfig` - Export to Neo4j/other databases
- `DatabaseImporterConfig` - Import from databases

## Validation Rules

### Numeric Ranges

- **Concurrency**: 1-100
- **Damping Factor**: 0.0-1.0
- **Gamma**: 0.0-10.0
- **Theta**: 0.0-1.0
- **Similarity Cutoff**: 0.0-1.0
- **Compression Level**: 1-9

### String Validation

- **Database Name**: Must start with letter, alphanumeric + underscore only
- **Paths**: Cannot be empty
- **Property Keys**: Cannot be empty

## Feature Flags

Optional features (not yet enabled):

- `serde` - Serialize/deserialize configs to YAML/JSON
- `config-files` - Load configurations from files

To enable:

```toml
[dependencies]
rust-gds = { version = "*", features = ["serde", "config-files"] }
```

## For AI Agents

This configuration system is designed for automated code generation:

1. **Predictable API** - All configs follow the same builder pattern
2. **Clear Defaults** - Call `::default()` for sensible starting point
3. **Validation Feedback** - Errors include parameter name and constraints
4. **Type Safety** - IDE/LSP provides autocomplete and type hints
5. **Composable** - Configs can be nested and combined

Example AI workflow:

```rust
// 1. Start with defaults
let mut config = PageRankConfig::default();

// 2. Apply user parameters
config.max_iterations = 100; // Direct field access works

// 3. Or use builder for validation
let validated = PageRankConfig::builder()
    .max_iterations(100)
    .build()?; // Validates all constraints
```

## Testing

Run configuration tests:

```bash
cargo test --lib config
```

Run example:

```bash
cargo run --example config_showcase
```

## Future Extensions

Planned features:

- [ ] YAML/JSON file loading
- [ ] Environment variable overrides
- [ ] Profile support (dev/prod configs)
- [ ] Config diffing and merging
- [ ] Macro-based config DSL
- [ ] Neo4j Cypher map interop

## Comparison to Other GDS Implementations

### Java GDS

- ✓ Simpler - No inheritance hierarchy
- ✓ Type-safe - Compile-time checking
- ✓ Ergonomic - Builder pattern with defaults

### TypeScript GDS

- ✓ More structured - Explicit builder types
- ✓ Validated - Runtime checks at construction
- ≈ Similar - Three-tier merge strategy available

### Rust Advantages

- Zero-cost abstractions
- Ownership prevents invalid sharing
- Pattern matching on errors
- Trait-based composition
