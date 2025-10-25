# Storage and Persistence Vision

**Date**: October 2024  
**Status**: 🔮 VISION - Not Yet Implemented  
**Context**: Collections Package as Foundation

---

## The Big Question: Where Does It Write to Disk?

**Short Answer**: **IT DOESN'T!** (Yet)

**Current Reality**:
- ✅ **Vec**: RAM only, no persistence
- ✅ **HugeArray**: RAM only (paged for scale, but still in-memory)
- ⚠️ **Arrow**: STUB - just Vec<T> with null bitmap, NO disk I/O

**Your Insight**: "Maybe Huge and Vec are both just RAM only? and only Arrow touches disk?"

**YES!** That's the vision!

---

## Current State: Everything is Ephemeral

### Vec Backend = Pure RAM

```rust
VecLong { data: Vec<i64> }  // std::vec::Vec - heap allocated
```

**Persistence**: NONE  
**When dropped**: Data disappears  
**Storage**: Process memory only

### HugeArray Backend = Paged RAM

```rust
PagedHugeL

ongArray {
    pages: Vec<Vec<i64>>,  // Multiple Vec arrays
}
```

**Persistence**: NONE  
**When dropped**: Data disappears  
**Storage**: Process memory (paged for scale)  
**Purpose**: Handle billions of elements without array size limits

### Arrow Backend = **STUB** (Currently Just Vec!)

```rust
ArrowIntArray {
    data: Vec<i32>,           // Just a Vec!
    null_bitmap: Option<Vec<u8>>,  // Null support
}
```

**Reality**: No Apache Arrow integration yet!  
**No**: Arrow IPC, Arrow Parquet, Arrow Flight  
**No**: Disk I/O, memory mapping, zero-copy  
**It's a placeholder!**

---

## The Vision: Arrow as Native Storage

### What Arrow SHOULD Be

Apache Arrow provides:

1. **Columnar Format**: Efficient analytics
2. **Zero-Copy**: Share data without serialization
3. **IPC/Parquet**: Disk persistence
4. **Memory Mapping**: Virtual memory, lazy loading
5. **SIMD Kernels**: Hardware acceleration

### Your Vision: Arrow as Disk Backend

```
Collections Backends:
├── Vec        → RAM only (small, fast)
├── HugeArray  → RAM only (scaled, paged)
└── Arrow      → DISK backed (persistent, mmap)
```

**This makes PERFECT sense!**

---

## Storage Architecture Vision

### Level 0: Collections Backends (What We're Building)

```
Collections<T> Trait
├── VecLong          (RAM, ephemeral)
├── HugeLongArray    (RAM, paged)
└── ArrowLongArray   (DISK, persistent) ← NEED TO BUILD!
```

### Level 1: PropertyStore (Using Collections)

```
MonadicPropertyStore {
    properties: HashMap<String, MonadicProperty>
}

MonadicProperty {
    values: Arc<dyn Collections<T>>  // Could be ANY backend!
}
```

### Level 2: GraphStore (Triadic Composition)

```
TriadicPropertyStore {
    meta_properties: MonadicPropertyStore,   // Vec (small)
    node_properties: MonadicPropertyStore,   // Huge or Arrow
    link_properties: MonadicPropertyStore,   // Huge or Arrow
}
```

### Level 3: Disk Layout (GraphStore Folders)

```
/data/graphs/
└── my-social-network/
    ├── catalog.json               # GraphStore metadata
    ├── level-0-meta/              # Graph metadata
    │   ├── graph_id.arrow
    │   ├── node_count.arrow
    │   └── created_at.arrow
    ├── level-1-nodes/             # Node properties
    │   ├── age.arrow
    │   ├── name.arrow
    │   └── pagerank.arrow
    └── level-2-links/             # Relationship properties
        ├── weight.arrow
        ├── timestamp.arrow
        └── similarity.arrow
```

**Each property = Arrow file!**

---

## The Storage Descriptor System

You already have this vision designed in `reality/src/descriptors/storage.rs`!

### StorageDescriptor

```rust
pub struct StorageDescriptor {
    pub persistence: PersistenceConfig,
    pub backend: BackendTechnology,
    // ...
}

pub enum Persistence {
    Ephemeral,    // Vec, HugeArray (RAM)
    Durable,      // Arrow (Disk)
    Distributed,  // Future: Remote storage
    Hybrid,       // Memory + Disk tiers
}

pub enum BackendTechnology {
    HugeArray { page_size: usize },
    Arrow,  // ← Arrow as native storage!
    Sparse,
    Custom(String),
}
```

**You already designed this!** Now we need to implement it!

---

## Arrow as Native Storage: Implementation Path

### Phase 1: Arrow Backend (Collections Level)

```rust
// Real Arrow implementation
pub struct ArrowLongArray {
    array: Arc<arrow::array::Int64Array>,  // Real Arrow!
    file_path: Option<PathBuf>,            // Disk location
}

impl Collections<i64> for ArrowLongArray {
    fn get(&self, index: usize) -> Option<i64> {
        self.array.value(index)  // Arrow API
    }
    
    fn flush(&mut self) -> Result<()> {
        // Write to Parquet file!
        arrow::ipc::write_parquet(self.file_path, &self.array)
    }
}
```

### Phase 2: PropertyStore with Persistence

```rust
impl MonadicPropertyStore {
    pub fn save(&self, path: &Path) -> Result<()> {
        for (key, property) in &self.properties {
            let file_path = path.join(format!("{}.arrow", key));
            property.values().flush()?;  // Writes to disk!
        }
        Ok(())
    }
    
    pub fn load(path: &Path) -> Result<Self> {
        // Scan directory for .arrow files
        // Load each into ArrowLongArray
        // Reconstruct PropertyStore
    }
}
```

### Phase 3: GraphStore with Folders

```rust
impl TriadicPropertyStore {
    pub fn save(&self, root: &Path) -> Result<()> {
        self.meta_properties.save(&root.join("level-0-meta"))?;
        self.node_properties.save(&root.join("level-1-nodes"))?;
        self.link_properties.save(&root.join("level-2-links"))?;
        
        // Write catalog.json
        self.write_catalog(root)?;
        Ok(())
    }
}
```

### Phase 4: GraphCatalog

```rust
pub struct GraphCatalog {
    graphs: HashMap<String, GraphHandle>,
    root_dir: PathBuf,
}

impl GraphCatalog {
    pub fn list_graphs(&self) -> Vec<String> {
        // Scan directory for graph folders
    }
    
    pub fn load_graph(&mut self, name: &str) -> Result<TriadicPropertyStore> {
        let graph_dir = self.root_dir.join(name);
        TriadicPropertyStore::load(&graph_dir)
    }
    
    pub fn save_graph(&mut self, name: &str, store: &TriadicPropertyStore) -> Result<()> {
        let graph_dir = self.root_dir.join(name);
        store.save(&graph_dir)
    }
}
```

---

## Disk Structure Vision

### Single Graph

```
/data/graphs/social-network/
├── catalog.json                    # Metadata
│   {
│     "name": "social-network",
│     "created_at": "2024-10-25",
│     "node_count": 1000000,
│     "edge_count": 5000000,
│     "properties": {
│       "meta": ["graph_id", "version"],
│       "nodes": ["age", "name", "pagerank"],
│       "links": ["weight", "timestamp"]
│     }
│   }
│
├── level-0-meta/
│   ├── _metadata.json              # Level metadata
│   ├── graph_id.arrow              # Arrow IPC or Parquet
│   └── version.arrow
│
├── level-1-nodes/
│   ├── _metadata.json
│   ├── age.arrow                   # 1M i64 values
│   ├── name.arrow                  # 1M string values
│   └── pagerank.arrow              # 1M f64 values
│
└── level-2-links/
    ├── _metadata.json
    ├── weight.arrow                # 5M f64 values
    └── timestamp.arrow             # 5M i64 values
```

### Multi-Graph Catalog

```
/data/graphs/
├── catalog.json                    # Global catalog
│   {
│     "graphs": [
│       {"name": "social-network", "path": "social-network"},
│       {"name": "knowledge-graph", "path": "knowledge-graph"},
│       {"name": "protein-network", "path": "protein-network"}
│     ]
│   }
│
├── social-network/
│   └── [structure above]
│
├── knowledge-graph/
│   └── [structure above]
│
└── protein-network/
    └── [structure above]
```

---

## Memory Mapping Vision

### Why Arrow + Memory Mapping = Magic

```rust
// Don't load entire file into RAM!
let mmap = unsafe { Mmap::map(&file)? };
let arrow_array = arrow::ipc::read_from_mmap(&mmap)?;

// Data stays on disk, accessed via page faults
// OS handles caching automatically!
```

**Benefits**:
- Load 100GB graph on 8GB machine
- OS manages memory pressure
- Fast startup (don't load everything)
- Share data between processes

---

## The Beautiful Hierarchy

```
Application Layer
    ↓
GraphCatalog (manages multiple graphs)
    ↓
TriadicPropertyStore (three-level structure)
    ↓
MonadicPropertyStore (property collections)
    ↓
MonadicProperty (single property)
    ↓
Collections<T> (backend abstraction)
    ↓
ArrowLongArray (disk-backed storage)
    ↓
Arrow Parquet File (actual bytes on disk)
```

**Every level is simple!**

---

## What Needs to Happen

### 1. Implement Real Arrow Backend

- Replace stub with `arrow::array::*`
- Support all primitive types
- Add null handling
- Implement flush/load

### 2. Add Persistence to Collections

```rust
pub trait Collections<T> {
    // Existing methods...
    
    fn save(&self, path: &Path) -> Result<()>;
    fn load(path: &Path) -> Result<Self>;
}
```

### 3. Add Persistence to PropertyStore

```rust
impl MonadicPropertyStore {
    pub fn save(&self, dir: &Path) -> Result<()>;
    pub fn load(dir: &Path) -> Result<Self>;
}
```

### 4. Add Persistence to GraphStore

```rust
impl TriadicPropertyStore {
    pub fn save(&self, dir: &Path) -> Result<()>;
    pub fn load(dir: &Path) -> Result<Self>;
}
```

### 5. Implement GraphCatalog

```rust
pub struct GraphCatalog {
    pub fn list_graphs(&self) -> Vec<String>;
    pub fn load_graph(&mut self, name: &str) -> Result<TriadicPropertyStore>;
    pub fn save_graph(&mut self, name: &str, store: &TriadicPropertyStore) -> Result<()>;
}
```

---

## Design Decisions

### Backend Selection Strategy

**Small data (<10M elements)**: Vec (fast, simple)  
**Large data (10M-10B elements)**: HugeArray (paged RAM)  
**Persistent data**: Arrow (disk, mmap)  
**Distributed data**: Future Arrow Flight

### When to Use Each

| Use Case | Backend | Rationale |
|----------|---------|-----------|
| Testing | Vec | Fast, disposable |
| Algorithms (ephemeral) | HugeArray | Scale, speed |
| Analytics (durable) | Arrow | Persistence, zero-copy |
| Production graphs | Arrow | Survive restarts |
| Shared graphs | Arrow | Memory-mapped sharing |

### File Format: Arrow IPC vs Parquet

**Arrow IPC** (Inter-Process Communication):
- Faster (no compression)
- Direct memory mapping
- Better for hot data

**Arrow Parquet**:
- Compressed (smaller files)
- Better for cold storage
- Industry standard

**Recommendation**: Start with IPC, add Parquet later

---

## Open Questions

### 1. Mutability

If Arrow is memory-mapped:
- Can we mutate in-place?
- Or copy-on-write?
- Or append-only logs?

### 2. Transactions

How to ensure consistency:
- Write-ahead log?
- Atomic renames?
- MVCC snapshots?

### 3. Versioning

How to handle graph evolution:
- Schema changes?
- Property additions/removals?
- Backward compatibility?

### 4. Compression

- Compress entire files?
- Per-column compression?
- Adaptive based on data?

### 5. Indexes

- Should we index property values?
- Separate index files?
- Arrow-native indexes?

---

## Next Steps

### Immediate (Learning)

1. ✅ Understand current state (DONE)
2. ⏳ Study Apache Arrow Rust crate
3. ⏳ Experiment with Arrow IPC
4. ⏳ Try memory mapping

### Short Term (Prototyping)

1. Implement ArrowLongArray with real Arrow
2. Add save/load to Collections
3. Add save/load to PropertyStore
4. Test round-trip (save → load)

### Medium Term (Integration)

1. Implement all Arrow primitive types
2. Add TriadicPropertyStore persistence
3. Implement GraphCatalog
4. Create examples

### Long Term (Production)

1. Performance optimization
2. Transaction support
3. Distributed storage (Arrow Flight)
4. Monitoring and diagnostics

---

## Summary

**Your Question**: "Where do things actually get written to disk?"  
**Answer**: **NOWHERE - Yet!**

**Your Vision**: "Arrow is our Native Storage"  
**Response**: **YES! Perfect!**

**The Path**: 
1. Vec/Huge = RAM (ephemeral, fast)
2. Arrow = Disk (persistent, mmap)
3. Triadic = Three folders
4. Catalog = Multiple graphs

**It all fits together beautifully!** 🎨

The Collections package is the foundation, and Arrow will be the persistence layer. We just need to build it! 🚀

---

## References

- Apache Arrow Rust: https://docs.rs/arrow/latest/arrow/
- Arrow IPC Format: https://arrow.apache.org/docs/format/Columnar.html
- Arrow Flight: https://arrow.apache.org/docs/format/Flight.html
- Parquet Format: https://parquet.apache.org/docs/
- `reality/src/descriptors/storage.rs` - Your storage descriptor vision!

