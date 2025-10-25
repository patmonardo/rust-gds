# Collections Architectural Vision - The Complete Picture

**Date**: October 2024  
**Status**: Active Development - Foundation of Everything  
**Philosophy**: Collections First, Applications Second

---

## The Core Insight

**Collections remains front and center.**

Everything else is a **client** of Collections - PropertyStores, GraphStores, ML Pipelines, etc.

---

## The Three-Tier Architecture

### Tier 0: Collections Package (THE FOUNDATION)

**The Kernel of Kernels** - Everything builds on this!

```
collections/
├── traits/              # Core abstraction
│   └── Collections<T>   # Universal interface
│
├── backends/            # Storage implementations
│   ├── vec/            # RAM (small data)
│   ├── huge/           # RAM (paged, billions)
│   └── arrow/          # DISK (persistent, mmap) [future]
│
├── extensions/          # Level 0 Algorithms
│   ├── paging.rs       # From GDS utils
│   ├── compression.rs  # From GDS utils
│   ├── queue.rs        # From GDS utils
│   ├── stack.rs        # From GDS utils
│   ├── metrics.rs      # From GDS utils
│   ├── random.rs       # From GDS utils
│   └── partitioning.rs # From GDS utils
│
└── macros/             # Code generation
    ├── core/           # collections! macro
    ├── backends/       # vec_collections!, huge_collections!
    └── extensions/     # Extension generators
```

**Philosophy**:
- Extensions ARE Level 0 Algorithms
- No need for full Genus/Species (that's for higher levels)
- Just call them Extensions - architecturally simple
- They evolved from Java GDS `core/utils` - but now properly integrated!

### Tier 1: Clients (APPLICATIONS OF COLLECTIONS)

**Simple Clients** (Monadic Pattern):
```
MonadicPropertyStore
    → Universal single-level storage
    → Works for ANY domain
    → Uses Collections<T> directly
```

**Hyper Clients** (Triadic Pattern):
```
TriadicPropertyStore
    → Context-aware three-level storage
    → Composes three MonadicPropertyStores
    → Meta/Node/Link pattern
    → Each level uses different Collections backend
```

**The Naming**:
- "Monadic" and "Triadic" work for initial experimentation
- They have philosophical connotations (good for exploration)
- Keep them for now as we learn!

### Tier 2: Graph Applications

**GraphStore** (Future):
```
GraphStore
    → Client of TriadicPropertyStore
    → Adds graph semantics (topology + properties)
    → CSR for topology, Collections for properties
```

---

## The Backend Strategy

### Three Backends, One Persistent

| Backend | Storage | Persistence | Use Case |
|---------|---------|-------------|----------|
| **Vec** | RAM | Ephemeral | Small data, testing |
| **Huge** | RAM (paged) | Ephemeral | Big data, algorithms |
| **Arrow** | Disk (mmap) | **PERSISTENT** | Production graphs |

**This is exactly what we want!** ✅

- Compute in RAM (Vec/Huge)
- Persist to Disk (Arrow)
- Choice per property!

---

## Collections Extensions = Level 0 Algorithms

### What Are Extensions?

They're **algorithmic** enhancements to Collections:

**From Java GDS `core/utils`** (evolved):
- Paging strategies
- Compression algorithms
- Queue/Stack operations
- Partitioning algorithms
- Memory estimation
- Random generation
- Metrics tracking

**These ARE algorithms** - but at the Collections level, not Graph level!

### Why Not Full Genus/Species?

**You're right** - we CAN skimp architecturally here!

**Level 0 (Collections)**: Extensions (simple)  
**Level 1+ (Graphs/ML)**: Genus/Species (full taxonomy)

**Rationale**:
- Extensions are infrastructure, not user-facing algorithms
- They compose with Collections (wrapper pattern)
- Simple naming is fine: `CompressedCollection`, `PagedCollection`
- Users don't need to understand Genus/Species at this level

**But the IDEA is there** - Extensions follow similar patterns:
- Composability
- Orthogonality
- Layering
- Just simpler naming!

---

## Utils: The Legacy Problem

### Current State

```
gds/src/core/utils/
├── partition/          # → Should be Collections Extension
├── paged/             # → Should be Collections Extension
├── intersections.rs   # → Should be Collections Extension
├── array_layout.rs    # → Should be Collections Extension
└── ...                # More stuff that should move
```

**Problem**: "Utils" is a junk drawer - important stuff shoved in a corner!

### The Vision: Dissolve Utils

**Move everything into proper homes**:

```
Before (Utils Mess):
gds/src/core/utils/partition/degree_partition.rs
gds/src/core/utils/paged/huge_atomic_bitset.rs

After (Collections Extensions):
gds/src/collections/extensions/partitioning.rs
gds/src/collections/extensions/paging.rs
```

**Why**:
- Utils = "I don't know where this goes"
- Extensions = "This enhances Collections"
- Clear purpose, clear location
- Not hidden away!

### Migration Strategy

**Phase 1**: Identify what belongs in Collections  
**Phase 2**: Move to extensions/ and refactor  
**Phase 3**: Deprecate old utils/  
**Phase 4**: Delete utils/ entirely  

**Not urgent** - but the direction is clear!

---

## The Experimentation Landscape

### What We've Built So Far

✅ **Collections Core**:
- Collections<T> trait
- Vec backend (8/8 primitives)
- Huge backend (3/9 primitives)
- 8 Extensions from utils

✅ **Simple Client (Monadic)**:
- MonadicPropertyStore
- 18 generated PropertyValues types
- Tests passing

✅ **Hyper Client (Triadic)**:
- TriadicPropertyStore
- Three-level composition
- Separate key spaces
- Tests passing

### How Much Experimentation Remains?

**Collections Package** - JUST BEGUN!

**Need to Explore**:

1. **Complete Backends**
   - 6 more Huge primitive impls
   - HugeObjectArray<T> Collections impl
   - Real Arrow implementation

2. **More Extensions**
   - Move more from utils/
   - Design new ones
   - Test composition

3. **Complex Types**
   - Collections<Vec<T>> for arrays
   - Collections<HashMap<K,V>> for maps
   - Collections<CustomStruct> for objects

4. **Performance**
   - Benchmark different backends
   - Optimize hot paths
   - Memory profiling

5. **Persistence** (Arrow)
   - Disk I/O
   - Memory mapping
   - Compression
   - Transactions?

6. **Client Patterns**
   - More monadic experiments
   - More triadic experiments
   - Other composition patterns?

**We're maybe 20% done with Collections!** 🚀

---

## The Philosophical Stack

### Level 0: Collections + Extensions (Infrastructure)

**Purpose**: Universal data structures and algorithms  
**Audience**: Internal (other packages)  
**Naming**: Simple (Collections, Extensions)  
**Examples**: VecLong, CompressedCollection, PartitionAwareCollection

### Level 1: Property/Graph Clients (Applications)

**Purpose**: Domain-specific storage patterns  
**Audience**: Graph algorithms  
**Naming**: Experimental (Monadic/Triadic for now)  
**Examples**: MonadicPropertyStore, TriadicPropertyStore, GraphStore

### Level 2: Graph Algorithms (Genus/Species)

**Purpose**: User-facing graph algorithms  
**Audience**: End users  
**Naming**: Full taxonomy (Genus/Species)  
**Examples**: PageRank (Centrality), ShortestPath (Pathfinding)

### Level 3: ML Pipelines (Feature Engineering)

**Purpose**: Machine learning workflows  
**Audience**: Data scientists  
**Naming**: ML conventions  
**Examples**: Node embeddings, link prediction

---

## The Collections Vision

### What Collections IS

✅ Universal data structure abstraction  
✅ Multiple backends (Vec, Huge, Arrow)  
✅ Composable extensions (Level 0 algorithms)  
✅ Foundation for everything else  
✅ Simple, focused, powerful  

### What Collections IS NOT

❌ Graph-specific  
❌ ML-specific  
❌ Application logic  
❌ User-facing API  

### Why This Matters

**Separation of Concerns**:
- Collections = data structures
- Clients = domain patterns
- Algorithms = user features

**Composability**:
- Any client can use any backend
- Extensions compose freely
- Mix and match as needed

**Evolution**:
- Collections evolves independently
- Clients evolve independently
- Loose coupling enables experimentation

---

## Current Focus

### What We're Doing NOW

1. **Perfecting Collections** - The foundation must be solid!
   - Complete primitive impls
   - Test thoroughly
   - Document clearly

2. **Experimenting with Clients** - Learn what works!
   - Monadic pattern (working!)
   - Triadic pattern (working!)
   - What other patterns exist?

3. **Building Infrastructure** - Extensions matter!
   - Migrate from utils/
   - Design new extensions
   - Test composition

### What We're NOT Doing

❌ Integrating with GraphStore yet  
❌ Production readiness  
❌ Performance optimization (premature!)  
❌ Arrow implementation (future)  

**Learn, experiment, explore!**

---

## The Path Forward

### Short Term (Weeks)

1. Complete Huge primitive impls (6 remaining)
2. Implement HugeObjectArray<T> Collections
3. Test complex types (arrays, maps, objects)
4. Generate all 46 PropertyValues types
5. More client experiments

### Medium Term (Months)

1. Migrate more from utils/ to extensions/
2. Design additional extensions
3. Benchmark and optimize
4. Real Arrow implementation
5. GraphStore integration experiments

### Long Term (Future)

1. Production-ready Collections
2. Full persistence story
3. Distributed storage (Arrow Flight?)
4. Mature client patterns
5. GraphStore migration

---

## Summary

**Collections** = The foundation of everything  
**Backends** = Three (Vec, Huge, Arrow) - one persistent  
**Extensions** = Level 0 Algorithms (from utils, evolved)  
**Clients** = Simple (Monadic) and Hyper (Triadic) patterns  
**Utils** = Legacy mess to be dissolved into proper homes  
**Experimentation** = JUST BEGUN - 80% remains!  

**The architecture is clear, the vision is solid, the work is exciting!** ✨

---

## Philosophical Notes

### Why "Monadic" and "Triadic"?

These terms have philosophical depth:
- **Monad** = single, self-contained unit
- **Triad** = three-in-one composition

They're not just technical terms - they carry meaning about the architecture!

Keep them for experimentation. If they prove their worth, they stay. If not, we'll evolve.

### Why Collections First?

Because **data structures are foundational**:
- Algorithms need data structures
- Applications need algorithms
- Build from the bottom up
- Get the foundation right

**Collections First = Architecture First** 🎯

