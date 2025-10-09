# Pregel Implementation Roadmap

## Current Status: ComputeStep Complete ✅

ComputeStep is **95% functional** - all core logic works, just needs context wiring.

## Phase 1: Utils (The Fun Stuff) 🎯 **← YOU ARE HERE**

These are concrete, straightforward implementations. No generics gymnastics!

### Priority 1: Core Concurrency Utils

1. **Concurrency struct** (`src/concurrency/mod.rs`)

   ```rust
   pub struct Concurrency(usize);

   impl Concurrency {
       pub fn of(threads: usize) -> Self
       pub fn available() -> Self  // num_cpus
       pub fn value(&self) -> usize
   }
   ```

2. **Partition Creation** (already exists, verify complete)
   - `Partition::new(start, count)`
   - `Partition::from(range)`
   - Need partition splitter for node count?

### Priority 2: Message System Utils

3. **Message Buffer** (if not exists)

   - Double-buffered queues
   - Current/next iteration separation
   - Thread-safe message accumulation

4. **Message Iterator Impl** (concrete type)
   - Iterates messages for a node
   - Reset functionality
   - Empty check

### Priority 3: Graph Integration

5. **Graph trait/struct** (if not exists)
   - `for_each_neighbor(node_id, callback)`
   - Degree queries
   - Thread-safe access

### Priority 4: Nice-to-Have Wrappers

6. **MutableInt** (atomic i32 wrapper)

   ```rust
   pub struct MutableInt(AtomicI32);
   impl MutableInt {
       pub fn get(&self) -> i32
       pub fn set(&mut self, value: i32)
       pub fn increment(&mut self)
   }
   ```

7. **AtomicBoolean** (if std doesn't suffice)
   - Already using std::sync::atomic::AtomicBool
   - Probably fine as-is!

## Phase 2: Context Wiring (30 Minutes) 🔌

**Quick plumbing pass to connect everything:**

### 2.1 Update NodeCentricContext

```rust
pub struct NodeCentricContext<C: PregelConfig> {
    node_id: u64,
    graph: Arc<Graph>,           // ADD THIS
    node_value: Arc<NodeValue>,  // ADD THIS
    _config: PhantomData<C>,
}

impl<C: PregelConfig> NodeCentricContext<C> {
    pub fn new(config: C, graph: Arc<Graph>, node_value: Arc<NodeValue>) -> Self {
        Self {
            node_id: 0,
            graph,
            node_value,
            _config: PhantomData,
        }
    }
}
```

### 2.2 Add Property Access to InitContext

```rust
impl<C: PregelConfig> InitContext<C> {
    pub fn set_node_value(&mut self, key: &str, value: f64) {
        self.base.node_value.set_double(self.base.node_id, key, value);
    }

    pub fn node_value(&self, key: &str) -> Option<f64> {
        self.base.node_value.get_double(self.base.node_id, key)
    }
}
```

### 2.3 Add Graph + Messenger to ComputeContext

```rust
pub struct ComputeContext<C: PregelConfig> {
    base: NodeCentricContext<C>,
    iteration: usize,
    messenger: Arc<dyn Messenger>,  // ADD THIS
    sent_message: bool,             // ADD THIS
}

impl<C: PregelConfig> ComputeContext<C> {
    pub fn for_each_neighbor<F>(&self, mut f: F)
    where F: FnMut(u64) {
        self.base.graph.for_each_neighbor(self.base.node_id, f);
    }

    pub fn send_to_neighbors(&mut self, message: f64) {
        self.base.graph.for_each_neighbor(self.base.node_id, |neighbor| {
            self.messenger.send_to(self.base.node_id, neighbor, message);
        });
        self.sent_message = true;
    }

    pub fn send_to(&mut self, target: u64, message: f64) {
        self.messenger.send_to(self.base.node_id, target, message);
        self.sent_message = true;
    }

    pub fn has_sent_message(&self) -> bool {
        self.sent_message
    }
}
```

### 2.4 Update ComputeStep Constructor

Pass graph and node_value when creating contexts:

```rust
let compute_context = ComputeContext::new(
    config.clone(),
    iteration,
    Arc::clone(&graph),
    Arc::clone(&node_value),
    Arc::clone(&messenger),
);
```

## Phase 3: Pregel Executor (The Real Crown) 👑

**This is where it all comes together:**

### 3.1 Pregel Struct

```rust
pub struct Pregel<C: PregelConfig> {
    config: C,
    graph: Arc<Graph>,
    schema: PregelSchema,
}

impl<C: PregelConfig> Pregel<C> {
    pub fn new(graph: Arc<Graph>, config: C, schema: PregelSchema) -> Self

    pub fn run<I, F, G>(
        &self,
        init_fn: F,
        compute_fn: G,
    ) -> PregelResult<NodeValue>
    where
        I: MessageIterator,
        F: Fn(&mut InitContext<C>) + Send + Sync,
        G: Fn(&mut ComputeContext<C>, &mut Messages<I>) + Send + Sync,
    {
        // 1. Create node value storage
        let node_value = Arc::new(NodeValue::of(&self.schema, self.graph.node_count(), ...));

        // 2. Create vote bits
        let vote_bits = Arc::new(HugeAtomicBitSet::new(self.graph.node_count()));

        // 3. Create messenger
        let messenger = Arc::new(SyncMessenger::new(...));

        // 4. Create partitions
        let partitions = self.create_partitions();

        // 5. BSP Loop
        for iteration in 0..self.config.max_iterations() {
            let has_sent_message = Arc::new(AtomicBool::new(false));

            // Execute all partitions in parallel
            partitions.par_iter().for_each(|partition| {
                let step = ComputeStep::new(
                    Arc::new(init_fn),
                    Arc::new(compute_fn),
                    self.config.clone(),
                    partition.clone(),
                    Arc::clone(&node_value),
                    Arc::clone(&messenger),
                    Arc::clone(&vote_bits),
                    iteration,
                    Arc::clone(&has_sent_message),
                    progress_tracker.clone(),
                );

                step.compute();
            });

            // Check convergence
            if !has_sent_message.load(Ordering::Relaxed) && vote_bits.all_set() {
                break;  // Converged!
            }

            // Swap message buffers
            messenger.advance_iteration();
        }

        // 6. Return results
        Ok(node_value)
    }
}
```

### 3.2 Partition Creation

```rust
fn create_partitions(&self) -> Vec<Partition> {
    let node_count = self.graph.node_count();
    let concurrency = self.config.concurrency().value();
    let batch_size = (node_count + concurrency - 1) / concurrency;

    (0..concurrency)
        .map(|i| {
            let start = i * batch_size;
            let end = ((i + 1) * batch_size).min(node_count);
            Partition::new(start, end - start)
        })
        .collect()
}
```

### 3.3 Message Buffer Management

```rust
pub struct SyncMessenger {
    current_messages: Vec<Vec<f64>>,  // Messages for current iteration
    next_messages: Vec<Vec<f64>>,     // Buffer for next iteration
}

impl SyncMessenger {
    pub fn advance_iteration(&mut self) {
        std::mem::swap(&mut self.current_messages, &mut self.next_messages);
        self.next_messages.clear();
    }
}
```

## Phase 4: First Algorithm - PageRank 📊

**Validate the entire system:**

```rust
pub fn page_rank(graph: Arc<Graph>, config: PageRankConfig) -> PregelResult<NodeValue> {
    let schema = PregelSchema::builder()
        .add("rank", ValueType::Double, DefaultValue::double(0.0))
        .build();

    let pregel = Pregel::new(graph, config, schema);

    pregel.run(
        // Init: set initial rank
        |ctx| {
            ctx.set_node_value("rank", 1.0 / ctx.node_count() as f64);
        },
        // Compute: PageRank iteration
        |ctx, messages| {
            let damping = 0.85;

            // Sum incoming messages
            let sum: f64 = messages.iter().sum();
            let new_rank = (1.0 - damping) + damping * sum;

            ctx.set_node_value("rank", new_rank);

            // Send rank to neighbors
            let out_degree = ctx.degree();
            if out_degree > 0 {
                let message = new_rank / out_degree as f64;
                ctx.send_to_neighbors(message);
            } else {
                ctx.vote_to_halt();
            }
        },
    )
}
```

## Timeline Estimate

- **Phase 1 (Utils)**: 2-4 hours (concrete implementations, no abstractions)
- **Phase 2 (Context Wiring)**: 30 minutes (just passing Arc refs)
- **Phase 3 (Executor)**: 3-5 hours (BSP loop, partitioning, convergence)
- **Phase 4 (PageRank)**: 1 hour (validation and debugging)

**Total: ~1 day of focused work**

## Next Immediate Steps

1. ✅ **Assess what utils already exist** (grep through src/)
2. ✅ **Implement missing utils** (Concurrency, message buffers, etc.)
3. ✅ **Wire contexts** (quick pass with Arc refs)
4. ✅ **Build executor** (the real work)
5. ✅ **PageRank** (proof it works!)

## Key Insight

**ComputeStep is done.** It's the hardest part because it required:

- Generic programming
- Parallel execution patterns
- Rust ownership gymnastics
- Trait object design

**Everything else is straightforward:**

- Utils: Concrete structs, no generics
- Context wiring: Just passing references
- Executor: Classic BSP loop
- PageRank: ~30 lines of user code

**You were right to move on!** The crown jewel is polished. Time to build the crown! 👑
