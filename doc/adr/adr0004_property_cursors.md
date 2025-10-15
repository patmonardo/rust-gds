# ADR 0004 — Property Cursor Contracts Across the Triadic Stores

Status: proposed  
Date: 2025-10-04

## Context

- The property layer now mirrors the triadic store model (graph, node, relationship) established in ADR 0002.
- Relationship traversal already depends on cursor-style iteration (`PropertyCursor`, `RelationshipCursor`, `RelationshipIterator`) that allow adjacency walks to reuse allocation-free snapshots.
- Node properties expose a different surface: `NodePropertyContainer` hands out `Arc<dyn NodePropertyValues>` keyed by property name, primarily for random access by algorithms and projections.
- As we prepare CORE features (IO drivers, compression, external adapters) we need a clear statement about why cursors exist, when a container is appropriate, and how extension modules should plug in without bypassing the object model.
- The name `property_cursor` appeared both at the root and inside `traits/`. We recently removed the legacy module, but we still need to document the conceptual contract so future pruning/renames stay grounded.

## Decision

1. Treat _cursor-style iteration_ as the canonical abstraction whenever property access is coupled to traversal order (primarily relationships):
   - `PropertyCursor` remains the low-level, allocation-free iterator over relationship property sequences.
   - `RelationshipIterator` returns boxed cursor snapshots to support composable stream APIs.
   - Implementations must guarantee that cursors reuse internal buffers and honour fallback values.
2. Treat _container-style access_ as the canonical abstraction whenever properties are keyed and order-independent (node & graph level):
   - `NodePropertyContainer` exposes lookups by property key and returns shared handles (`Arc<dyn NodePropertyValues>`).
   - Graph-level metadata continues to live behind `GraphPropertyStore` (a keyed container) because no traversal cursor exists for global scalars.
3. Extension points follow the pattern:
   - Traits live in `traits/` and own the contract (cursor, iterator, container).
   - Default implementations live in `impls/` and may carry helper methods/tests.
   - Driver or CORE modules implement the trait directly and _must not_ rely on concrete default structs unless they intend to reuse the exact semantics.
4. Naming guidance moving forward:
   - Use singular module names for concept façades (`relationship_property_values`, `node_property_values`).
   - Reserve plural names only when the module exposes helper collections (e.g., `relationship_properties` wrapping cursor helpers). Future clean-up can rename this once API parity concerns are addressed.

## Rationale

- Relationship traversal is adjacency-oriented; cursors let us stream edge properties without materialising vectors or cloning values.
- Node and graph properties are accessed by key, not by adjacency index; containers are a more ergonomic fit and align with TypeScript/Neo4j APIs.
- Separating trait contracts from default implementations keeps the object model pure and ready for specialised drivers (HugeSparseArray, Arrow mmap, on-disk IO) without breaking algorithms.
- The documented pattern gives CORE contributors a consistent guide when introducing new cursor/container hybrids (e.g., manifest IO cursors that feed directly into Arrow readers).

## Consequences

- Algorithm authors know when to expect a cursor (`RelationshipIterator`) versus a keyed container (`NodePropertyContainer`) and can code accordingly.
- Future drivers implementing custom cursors (e.g., compressed adjacency) have a single trait to satisfy, reducing surface area for integration bugs.
- Documentation debt is reduced: the removal of redundant `property_cursor.rs` at the module root is justified by this ADR, preventing regression when renaming traits later.
- We accept temporary naming asymmetry (`relationship_properties`) but treat it as cosmetic; any rename must preserve the cursor/helper pattern explained here.

## Follow-up

- Add API docs (rustdoc examples) illustrating cursor vs container usage inside `DefaultGraph` once CORE projections land.
- Evaluate introducing a lightweight `GraphPropertyContainer` façade for parity once more graph-level metadata exists.
- When migrating CORE IO features, reference this ADR to ensure new adapters (e.g., Arrow2 mmap cursors) implement `PropertyCursor` semantics without leaking driver-specific types.
