# ID Property Values Modules - Current Status

## Summary

`MappedIdNodePropertyValues` and `OriginalIdNodePropertyValues` have been completed but are **not currently used** in the codebase. They are anticipatory infrastructure for future IO/loading scenarios.

## Modules Completed

1. **MappedIdNodePropertyValues** (~260 lines, 16 tests ✅)

   - Property value = mapped (internal) node ID
   - Zero-overhead identity mapping
   - Useful for debugging and internal tracking

2. **OriginalIdNodePropertyValues** (~370 lines, 20 tests ✅)
   - Property value = original (source graph) node ID
   - Captures ID mapping at creation to avoid lifetime issues
   - Critical for export scenarios

## Current Usage: NONE

```bash
# Grep shows no actual usage outside tests:
$ grep -r "MappedIdNodePropertyValues\|OriginalIdNodePropertyValues" src/ --include="*.rs" | grep -v "src/core/utils"
# (no results)
```

These modules only exist in:

- `src/core/utils/mapped_id_node_property_values.rs` (implementation)
- `src/core/utils/original_id_node_property_values.rs` (implementation)
- `src/core/utils/mod.rs` (re-exports)

## Where They WILL Be Used

### 1. Database Import (Future)

When loading from Neo4j or other databases:

```rust
// Future DatabaseImporter pseudocode:
let id_map = /* build mapping from DB IDs → internal IDs */;
let original_ids = OriginalIdNodePropertyValues::from_id_map(&id_map);

// Export results back with original IDs:
for node_id in 0..graph.node_count() {
    let internal_score = pagerank_scores.long_value(node_id);
    let original_id = original_ids.long_value(node_id)?;
    // Write (original_id, score) back to database
}
```

### 2. File Import/Export (Future)

CSV or other file formats where node IDs must be preserved:

```rust
// Import from CSV with non-sequential IDs:
// CSV: node_id,label
//      1001,A
//      2003,B
//      3005,C
let id_map = /* map file IDs → 0,1,2 */;
let original_ids = OriginalIdNodePropertyValues::from_id_map(&id_map);

// Export results preserving original IDs:
write_csv_with_original_ids(&original_ids, &results);
```

### 3. Algorithm Results Export (Future)

Any algorithm that needs to output results in terms of source graph IDs:

```rust
// After running PageRank:
let scores = pagerank(&graph, &config);
let original_ids = OriginalIdNodePropertyValues::from_id_map(&graph.id_map());

// Stream results with original IDs:
for node in 0..graph.node_count() {
    emit_result(
        original_ids.long_value(node)?,  // Original ID from source
        scores.double_value(node)?        // PageRank score
    );
}
```

## Existing IO Infrastructure (Config Only)

The config system exists but **no implementations yet**:

```rust
// src/config/io_config.rs - EXISTS
pub struct FileImporterConfig { ... }       // ✅ Config only
pub struct FileExporterConfig { ... }       // ✅ Config only
pub struct DatabaseImporterConfig { ... }   // ✅ Config only
pub struct DatabaseExporterConfig { ... }   // ✅ Config only

// Actual implementations - NOT YET BUILT:
// src/io/file_importer.rs      ❌ Does not exist
// src/io/database_importer.rs  ❌ Does not exist
// src/io/csv_loader.rs         ❌ Does not exist
// src/io/neo4j_connector.rs    ❌ Does not exist
```

## Why These Modules Matter

1. **IdMap Transparency**: Internal graphs use 0-based sequential IDs for performance
2. **Source Fidelity**: Original IDs from Neo4j, CSV, etc. may be non-sequential (1001, 2003, 5009)
3. **Round-Trip Export**: Algorithm results must be exportable with original IDs intact
4. **Form Processor Integration**: These will move to MetaMacro Form Processor when built

## MetaMacro Form Processor (Future Home)

You mentioned these should eventually live in the **Form Processor**, not core/utils:

> "we are really addressing this in the Form Processor and hopefully what these last few modules do will move out of core/utils and into our MetaMacro Form Processor"

The Form Processor will handle:

- **Input forms**: Parse/validate/transform incoming data (CSV, JSON, Neo4j results)
- **ID mapping coordination**: Track Original ↔ Mapped ID relationships
- **Output forms**: Transform algorithm results back to source ID space
- **Schema validation**: Ensure imported data matches expected graph schema

## Decision Point: Keep or Remove?

### Option 1: Keep Modules (Current State) ✅

- **Pro**: Ready when IO layer is built
- **Pro**: Tests ensure correctness
- **Pro**: No work needed later
- **Con**: Dead code until IO exists
- **Con**: Confuses "what's actually used?"

### Option 2: Remove Until Needed

- **Pro**: No dead code in codebase
- **Pro**: Clear signal of current capabilities
- **Con**: Will need to recreate when IO is built
- **Con**: Lose test coverage and design thinking

### Option 3: Keep but Mark as "Stub" ⚠️

- Add `#[cfg(feature = "io")]` feature gate
- Document in ADR as "future infrastructure"
- Keep code but don't compile unless `io` feature enabled

## Recommendation

**Keep the modules** (Option 1) because:

1. They're complete and tested (36/36 tests passing)
2. They follow correct Rust patterns (no unsafe, proper error handling)
3. IO layer will need them immediately when built
4. Moving to Form Processor is a refactor, not deletion
5. Documentation now clarifies their "future use" status

## Related Work Still Missing

To actually use these modules, we need:

1. ✅ **IdMap trait** - exists in `src/types/graph/id_map/`
2. ✅ **ID property modules** - just completed
3. ❌ **File loaders** - CSV, JSON, Parquet parsers
4. ❌ **Database connectors** - Neo4j Bolt protocol, JDBC bridges
5. ❌ **Graph builders from external data** - Transform loaded data → GraphStore
6. ❌ **Result exporters** - Write algorithm outputs back to source format
7. ❌ **Form Processor architecture** - MetaMacro input/output transformation pipeline

## File Locations

```
src/core/utils/
├── mapped_id_node_property_values.rs        ✅ Complete (16 tests)
├── original_id_node_property_values.rs      ✅ Complete (20 tests)
└── mod.rs                                    ✅ Exports both

src/config/io_config.rs                       ✅ Config only (no implementations)

src/io/                                        ❌ Directory does not exist
```

## Next Steps (When IO is Ready)

1. Create `src/io/` module structure
2. Implement `FileImporter` using `FileImporterConfig`
3. Use `OriginalIdNodePropertyValues::from_id_map()` in import pipeline
4. Build `ResultExporter` that consumes ID property values
5. Test round-trip: Import → Process → Export with ID preservation
6. Migrate to Form Processor when MetaMacro architecture is ready

## Conclusion

These modules are **complete but dormant**. They represent thoughtful anticipation of IO needs but aren't blocking current work. When the IO layer is built, they'll be immediately useful. Until then, they serve as documentation of the ID mapping problem and its solution.

---

**Status**: ✅ Complete but unused  
**Tests**: 36/36 passing  
**Future Home**: MetaMacro Form Processor  
**Blocks**: Nothing - IO can be built independently  
**Blocked By**: IO layer implementation (FileImporter, DatabaseImporter, etc.)
