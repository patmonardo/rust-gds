## 02 — Structures (GraphStore)

Configure and assemble the modern GraphStore with adaptive backends.

**Examples** (in `archive/examples/02-structures/`):
- config_showcase.rs — ✅ Builder configs (compute/memory/cache/properties)
- graphstore_walkthrough.rs — ✅ End-to-end store assembly
- property_showcase.rs — ✅ Property system (graph/node/relationship)
- node_property_store_basics.rs — ✅ Node properties
- relationship_property_store_basics.rs — ✅ Relationship properties

**Coming soon**:
- arrow_config_showcase.rs — 📋 Arrow-backed properties configuration

**Key Concepts**:
- Configuration-driven architecture
- Triadic property system (graph/node/relationship)
- Adaptive backend selection (Vec → Huge → Arrow)
