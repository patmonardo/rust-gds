# ADR 0001 — PropertyGraph (GraphStore) design

Status: proposed

Context

- Need a canonical, logical PropertyGraph that is not a DataFrame but composes schema + property stores.
- Storage/backing may be Arrow2 mmap (zero-copy) and/or Polars Series (analytics).

Decision

- Introduce a GraphStore abstraction that:
  - owns/refs a Schema registry (types/property schemas),
  - attaches one or more PropertyStore implementations (Arrow2MmapPropertyStore, PolarsPropertyStore),
  - exposes projection APIs to create transient Graph projections (subgraphs, node/relationship views) without copying underlying buffers.
- PropertyStore implementations are drivers: provide scalar reads, iterators/cursors, and optional conversion to Polars Series/DataFrame.
- Enforce lifetime rule: Arrow2 adapters require the store (Arc) to remain alive while arrays are referenced.

Consequences

- Clear separation between logical graph and physical drivers.
- Easier to swap/extend stores (e.g., add file-system partitioning).
- Must standardize defaults, coercions, and zero-copy contracts in subsequent ADRs.
