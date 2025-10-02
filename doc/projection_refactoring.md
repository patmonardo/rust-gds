# Projection Module Refactoring

## Date: October 1, 2025

## Motivation

Refactored the projection module structure to follow idiomatic Rust conventions and avoid keyword conflicts.

## Changes

### Before

```
src/projection/
├── mod.rs
├── node_label.rs
├── relationship_type.rs
└── r#abstract/              # Used raw identifier for 'abstract' keyword
    ├── property_mapping.rs
    ├── element_projection.rs
    └── abstract_projections.rs
```

### After

```
src/projection/
├── mod.rs
├── node_label.rs            # NodeLabel with interning
├── relationship_type.rs     # RelationshipType with interning
├── traits/                  # Abstract trait definitions
│   ├── mod.rs
│   ├── property_mapping.rs         # PropertyMapping, Aggregation
│   ├── element_projection.rs       # ElementProjection, PropertyMappings
│   └── abstract_projections.rs     # AbstractProjections
└── impls/                   # Concrete implementations
    └── mod.rs               # Ready for implementations
```

## Rationale

1. **Idiomatic Rust**: `traits/` + `impls/` is a standard convention for separating interface from implementation
2. **No keyword conflicts**: Avoids `r#abstract` raw identifier syntax
3. **Clear architecture**: Mirrors TypeScript structure (`abstract/` → `traits/`, `primitive/` → `impls/`)
4. **Barrel imports**: Implementation details hidden behind clean public API in `mod.rs`

## Module Organization Philosophy

- **traits/**: Define behavior contracts (like TypeScript interfaces/abstract classes)
- **impls/**: Provide concrete implementations (like TypeScript concrete classes)
- **Root level**: Lightweight types used across the module (NodeLabel, RelationshipType)

## Status

- ✅ Refactoring complete
- ✅ All 29 tests passing
- ✅ Ready for concrete implementations

## Next Steps

Implement concrete projection types in `impls/`:

1. `property_mappings.rs` - PropertyMappings collection
2. `node_projection.rs` - NodeProjection
3. `relationship_projection.rs` - RelationshipProjection
4. `node_projections.rs` - NodeProjections collection
5. `relationship_projections.rs` - RelationshipProjections collection
6. `immutable_node_projections.rs` - Immutable variant
7. `immutable_relationship_projections.rs` - Immutable variant

## Testing

All projection tests continue to pass after refactoring:

```bash
cargo test projection::
# Result: ok. 29 passed; 0 failed; 0 ignored
```
