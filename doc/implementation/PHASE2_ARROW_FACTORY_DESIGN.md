# Phase 2 Design: Reference System

**Date**: October 15, 2025  
**Status**: 📐 Design Complete (Implementation deferred to arrow2 integration)  
**Translation Plan**: TP-004 Phase 2

## Overview

Phase 2 defines the Reference System - type-safe wrappers around Arrow tables and batches that enable schema inference and validation.

## Key Concepts

### ArrowReference Trait

Base trait for all table references:

```rust
pub trait ArrowReference: Send + Sync {
    fn table_name(&self) -> &str;
    fn schema(&self) -> &Schema;
    fn validate_schema(&self) -> Result<(), ArrowProjectionError>;
}
```

### NodeTableReference

Wraps Arrow tables containing nodes:

- **ID column**: Primary key (Int64)
- **Label column**: Optional node labels (String or List<String>)
- **Property columns**: All other columns
- **Schema inference**: Auto-detect column roles from names
- **Validation**: Verify schema matches expected structure

### EdgeTableReference

Wraps Arrow tables containing edges:

- **Source column**: Source node IDs (Int64)
- **Target column**: Target node IDs (Int64)
- **Type column**: Optional edge types (String)
- **Property columns**: All other columns
- **Schema inference**: Auto-detect from conventions
- **Validation**: Verify source/target are Int64

### ArrowBatchReference

Wraps individual Arrow RecordBatches:

- **Batch metadata**: Row count, schema
- **Column extraction**: Type-safe accessors
- **Iterator state**: Current position (for Phase 3 Scanner)

## Translation from Java

**Java GDS Reference System** (7 files):

- `RecordReference.java` → `ArrowReference` trait
- `NodeReference.java` + 3 impls → `NodeTableReference`
- `RelationshipReference.java` + 2 impls → `EdgeTableReference`

**Key differences**:

- Neo4j cursor state → Arrow batch + schema
- Database references → in-memory table metadata
- Label token sets → label column (String/List)
- Property references → column names

## Implementation Status

**Phase 2 is DEFERRED** to full arrow2 integration because:

1. **arrow2 migration needed**: Project uses arrow2, not arrow crate
2. **No immediate consumers**: Factory skeleton doesn't use references yet
3. **Design is clear**: Can implement when needed (Phase 3+ Scanner)
4. **Better with context**: Will implement alongside actual arrow2 table usage

## Design Decisions

### Schema Inference Conventions

**Node tables**:

- ID column: `id`, `nodeId`, `node_id`
- Label column: `label`, `labels`, `node_label`
- Properties: all other columns

**Edge tables**:

- Source: `source`, `src`, `source_id`, `sourceId`
- Target: `target`, `dst`, `target_id`, `targetId`
- Type: `type`, `rel_type`, `relationship_type`
- Properties: all other columns

### Validation Strategy

- **Fail-fast**: Validate at reference creation time
- **Type checking**: Verify ID/source/target are Int64
- **Clear errors**: Descriptive messages with column/table names

## Next Steps

**Phase 3: Scanner System** - Will implement references when building batch scanners.

For now, Phase 2 documents the design and establishes the conceptual foundation. The actual Rust implementation will happen when we integrate with arrow2 tables in subsequent phases.

## Related Documents

- `TP-004_NATIVE_PROJECTION_ARROW_FACTORY.md` - Overall translation plan
- `NATIVE_PROJECTION_ARROW_DESIGN.md` - Architecture design
- `PHASE1_ARROW_FACTORY_COMPLETE.md` - Phase 1 completion record

---

**The documentation IS the Knowledge Graph.** This Phase 2 design record captures the conceptual model and will guide implementation when arrow2 integration happens. ✨
