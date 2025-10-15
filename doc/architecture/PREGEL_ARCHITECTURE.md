# Pregel Component Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER ALGORITHM                            │
│  impl PregelComputation { compute(), init(), master_compute() }  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ↓
┌─────────────────────────────────────────────────────────────────┐
│                    PREGEL EXECUTOR (TODO)                        │
│  • Superstep loop orchestration                                  │
│  • Vote-to-halt tracking (HugeAtomicBitSet)                     │
│  • Barrier synchronization                                       │
│  • Parallel execution                                            │
└──────┬─────────────────────┬──────────────────────┬─────────────┘
       │                     │                      │
       ↓                     ↓                      ↓
┌────────────────┐  ┌──────────────────┐  ┌──────────────────────┐
│   CONTEXTS     │  │   MESSENGERS     │  │    NODE VALUES       │
│   (PARTIAL)    │  │   (COMPLETE)     │  │    (COMPLETE)        │
├────────────────┤  ├──────────────────┤  ├──────────────────────┤
│ • InitContext  │  │ • SyncQueue      │  │ • PropertyArray enum │
│ • ComputeCtx   │  │ • AsyncQueue     │  │   - Double           │
│ • MasterCtx    │  │ • Reducing       │  │   - Long             │
│ • NodeCentric  │  │                  │  │   - DoubleArray      │
│                │  │ Each with:       │  │   - LongArray        │
│ Needs wiring:  │  │ • Iterator       │  │                      │
│ • Graph access │  │ • send_to()      │  │ • Schema validation  │
│ • NodeValue    │  │ • init_iteration │  │ • Type-safe access   │
│ • Messenger    │  │                  │  │                      │
└────────────────┘  └──────────────────┘  └──────────────────────┘
       │                     │                      │
       │                     ↓                      ↓
       │            ┌──────────────────┐  ┌──────────────────────┐
       │            │  MESSAGE QUEUES  │  │   HUGE ARRAYS        │
       │            │   (COMPLETE)     │  │   (COMPLETE)         │
       │            ├──────────────────┤  ├──────────────────────┤
       │            │ • SyncDouble     │  │ • HugeDoubleArray    │
       │            │   Queues         │  │ • HugeLongArray      │
       │            │ • AsyncDouble    │  │ • HugeObjectArray<T> │
       │            │   Queues         │  │ • HugeAtomic*Array   │
       │            │                  │  │                      │
       │            │ Uses:            │  │ Features:            │
       │            │ • HugeObject     │  │ • Billions of elems  │
       │            │   Array<Vec<f64>>│  │ • Single/Paged       │
       │            │ • swap()         │  │ • Cursor iteration   │
       │            │ • compact()      │  │ • Lock-free atomic   │
       │            └──────────────────┘  └──────────────────────┘
       │                     │
       │                     ↓
       │            ┌──────────────────┐
       │            │    REDUCERS      │
       │            │   (COMPLETE)     │
       │            ├──────────────────┤
       │            │ • Sum (id: 0.0)  │
       │            │ • Min (id: MAX)  │
       │            │ • Max (id: MIN)  │
       │            │ • Count (id: 0)  │
       │            │                  │
       │            │ • Trait object   │
       │            │   conversion     │
       │            │ • String parsing │
       │            └──────────────────┘
       │
       ↓
┌─────────────────────────────────────────────────────────────────┐
│                        GRAPH API                                 │
│  • Graph trait (topology queries)                                │
│  • NodeLabel, RelationshipType                                   │
│  • Degree, neighbors, existence checks                           │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                   CONFIGURATION & SCHEMA                         │
│  • PregelConfig (max_iterations, concurrency, partitioning)     │
│  • PregelSchema (property definitions, visibility)              │
│  • PregelResult (final node values, iteration count)            │
└─────────────────────────────────────────────────────────────────┘
```

## Component Status Legend

✅ **COMPLETE** - Fully implemented, tested, production-ready
⚠️ **PARTIAL** - Structure exists but needs integration/wiring
🚧 **TODO** - Not yet implemented

## Data Flow: Typical Pregel Execution

```
1. INITIALIZATION
   ┌─────────────┐
   │ User calls  │
   │ execute()   │
   └──────┬──────┘
          │
          ↓
   ┌─────────────────────┐
   │ PregelExecutor      │
   │ • Build NodeValue   │
   │ • Build Messenger   │
   │ • Call init()       │
   └──────┬──────────────┘
          │
          ↓
   ┌─────────────────────┐
   │ InitContext         │
   │ • Per-node init     │
   │ • Set initial vals  │
   └─────────────────────┘

2. SUPERSTEP LOOP (while not converged)

   ┌─────────────────────────────────────────┐
   │ Superstep N                             │
   ├─────────────────────────────────────────┤
   │                                         │
   │ A. Master Compute (optional)            │
   │    ┌──────────────────────┐            │
   │    │ MasterComputeContext │            │
   │    │ • Global aggregation │            │
   │    │ • Broadcast values   │            │
   │    └──────────────────────┘            │
   │                                         │
   │ B. Init Iteration                       │
   │    ┌──────────────────────┐            │
   │    │ Messenger            │            │
   │    │ • swap() buffers     │  (Sync)    │
   │    │ • compact() queues   │  (Async)   │
   │    │ • clear send array   │  (Reducing)│
   │    └──────────────────────┘            │
   │                                         │
   │ C. Node Compute (parallel)              │
   │    ┌──────────────────────┐            │
   │    │ For each node:       │            │
   │    │  ComputeContext      │            │
   │    │  • Get messages      │            │
   │    │  • compute() call    │            │
   │    │  • send_to(neighbors)│            │
   │    │  • vote_to_halt()    │            │
   │    └──────────────────────┘            │
   │                                         │
   │ D. Convergence Check                    │
   │    ┌──────────────────────┐            │
   │    │ • All voted to halt? │            │
   │    │ • Max iterations?    │            │
   │    │ • No messages sent?  │            │
   │    └──────────────────────┘            │
   │                                         │
   └─────────────────────────────────────────┘
          │
          │ [converged]
          ↓
   ┌─────────────────────┐
   │ PregelResult        │
   │ • Final NodeValue   │
   │ • Iteration count   │
   │ • Did converge      │
   └─────────────────────┘

3. FINALIZATION
   ┌─────────────────────┐
   │ • Release messengers│
   │ • Return result     │
   └─────────────────────┘
```

## Message Passing Flow

```
SYNCHRONOUS (SyncQueueMessenger):
─────────────────────────────────
Iteration N:
  Write Buffer: [msg1, msg2, ...]  ← send_to() appends here
  Read Buffer:  [old messages]     ← compute() reads from here

  swap() called at iteration boundary

Iteration N+1:
  Write Buffer: []                 ← now empty (was read buffer)
  Read Buffer:  [msg1, msg2, ...]  ← now has N's messages


ASYNCHRONOUS (AsyncQueueMessenger):
────────────────────────────────────
Single Buffer: [msg1, msg2, msg3, ...]
               [head───────────────]

  • push() appends immediately
  • Messages visible same iteration
  • compact() when head > 25% capacity


REDUCING (ReducingMessenger):
──────────────────────────────
Send Array:    [0.0, 0.0, 5.0, 0.0, ...]  ← atomic reduction
Receive Array: [0.0, 8.0, 0.0, 0.0, ...]  ← previous iteration

  send_to(src, target, msg):
    loop {
      current = array[target]
      reduced = reducer.reduce(current, msg)
      if compare_exchange(current, reduced) succeeds:
        break
    }

  swap() at iteration boundary
  Each node gets ≤1 message (reduced value)
```

## Test Coverage Map

```
┌─────────────────────────────────────┐
│ Collections Layer                   │
│ ✅ HugeDoubleArray:      20+ tests │
│ ✅ HugeLongArray:        20+ tests │
│ ✅ HugeObjectArray:      8 tests   │
│ ✅ HugeAtomicDoubleArray:10 tests  │
│ ✅ HugeAtomicLongArray:  10 tests  │
│ ✅ Cursors:              30+ tests │
└─────────────────────────────────────┘
           │
           ↓
┌─────────────────────────────────────┐
│ Pregel Data Structures              │
│ ✅ NodeValue:            8 tests   │
│ ✅ SyncDoubleQueues:     7 tests   │
│ ✅ AsyncDoubleQueues:    7 tests   │
│ ✅ PregelSchema:         15+ tests │
└─────────────────────────────────────┘
           │
           ↓
┌─────────────────────────────────────┐
│ Message Passing                     │
│ ✅ Reducers:             14 tests  │
│ ✅ SyncQueueMessenger:   3 tests   │
│ ✅ AsyncQueueMessenger:  3 tests   │
│ ✅ ReducingMessenger:    6 tests   │
└─────────────────────────────────────┘
           │
           ↓
┌─────────────────────────────────────┐
│ Integration & E2E                   │
│ 🚧 Executor:             TODO      │
│ 🚧 Context integration:  TODO      │
│ 🚧 Algorithm examples:   TODO      │
└─────────────────────────────────────┘

TOTAL: 809 tests passing ✅
```

## File Structure

```
src/pregel/
├── mod.rs                          # Module exports (COMPLETE)
├── computation.rs                  # Computation traits (COMPLETE)
├── config.rs                       # PregelConfig (COMPLETE)
├── schema.rs                       # PregelSchema (COMPLETE)
├── result.rs                       # PregelResult (COMPLETE)
├── messages.rs                     # Message traits (COMPLETE)
├── reducers.rs                     # Reducer impls (COMPLETE) ✨ NEW
├── messengers.rs                   # Messenger impls (COMPLETE) ✨ NEW
├── queues.rs                       # Message queues (COMPLETE)
├── node_value.rs                   # Property storage (COMPLETE)
├── context/
│   ├── mod.rs                      # Context exports
│   ├── init_context.rs             # Init phase (PARTIAL)
│   ├── compute_context.rs          # Compute phase (PARTIAL)
│   ├── master_compute_context.rs   # Master phase (PARTIAL)
│   ├── node_centric_context.rs     # Base context (PARTIAL)
│   └── bidirectional_context.rs    # Bidirectional (PARTIAL)
└── executor.rs                     # BSP orchestration (TODO) 🚧

examples/
└── pregel_*.rs                     # Algorithm examples (TODO) 🚧
```

## Performance Targets (Estimated)

```
Node Count: 1M nodes
Message Count: 10M messages per iteration
Hardware: Modern CPU (8 cores)

┌──────────────────┬──────────────┬──────────────┐
│ Messenger Type   │ Iteration    │ Memory       │
├──────────────────┼──────────────┼──────────────┤
│ SyncQueue        │ ~100-200ms   │ ~80 MB       │
│ AsyncQueue       │ ~100-200ms   │ ~80 MB       │
│ Reducing (Sum)   │ ~50-100ms    │ ~8 MB        │
└──────────────────┴──────────────┴──────────────┘

With parallelism (8 cores):
  Reducing: ~10-20ms/iteration ⚡
```

---

**Key Insight**: The hard part is DONE! We have all the primitives.
Now we just need to orchestrate them in the executor loop.
