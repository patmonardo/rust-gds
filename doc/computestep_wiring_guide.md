# ComputeStep Wiring - Exact Changes Needed

## Files to Edit

1. `src/pregel/compute_step.rs` - Add graph field, update constructors
2. `src/pregel/computer.rs` - Pass graph to ComputeStep::new(), wrap NodeValue in RwLock

## Edit 1: Add graph field to ComputeStep struct

**Location**: Line ~66 in compute_step.rs

**Change**:

```rust
// BEFORE:
    /// Node value storage
    node_value: Arc<NodeValue>,

    /// Messenger for sending/receiving messages
    messenger: Arc<dyn Messenger<I>>,

// AFTER:
    /// Node value storage (wrapped in RwLock for contexts to write)
    node_value: Arc<parking_lot::RwLock<NodeValue>>,

    /// Graph topology (for contexts to query)
    graph: Arc<dyn crate::types::graph::Graph>,

    /// Messenger for sending/receiving messages
    messenger: Arc<dyn Messenger<I>>,
```

## Edit 2: Update ComputeStep::new() signature

**Location**: Line ~108 in compute_step.rs

**Change**:

```rust
// BEFORE:
    pub fn new(
        init_fn: InitFn<C>,
        compute_fn: ComputeFn<C, I>,
        config: C,
        node_batch: Partition,
        node_value: Arc<NodeValue>,
        messenger: Arc<dyn Messenger<I>>,
        vote_bits: Arc<HugeAtomicBitSet>,
        iteration: usize,
        has_sent_message: Arc<AtomicBool>,
        progress_tracker: Arc<ProgressTracker>,
    ) -> Self

// AFTER:
    pub fn new(
        init_fn: InitFn<C>,
        compute_fn: ComputeFn<C, I>,
        config: C,
        graph: Arc<dyn crate::types::graph::Graph>,  // ADD THIS
        node_batch: Partition,
        node_value: Arc<parking_lot::RwLock<NodeValue>>,  // CHANGE TYPE
        messenger: Arc<dyn Messenger<I>>,
        vote_bits: Arc<HugeAtomicBitSet>,
        iteration: usize,
        has_sent_message: Arc<AtomicBool>,
        progress_tracker: Arc<ProgressTracker>,
    ) -> Self
```

## Edit 3: Update compute_context creation in new()

**Location**: Line ~124 in compute_step.rs

**Change**:

```rust
// BEFORE:
        let compute_context = ComputeContext::new(config.clone(), iteration);

        Self {
            init_fn,
            compute_fn,
            node_batch,
            node_value,
            messenger,
            // ... rest

// AFTER:
        let compute_context = ComputeContext::new(
            Arc::clone(&graph),
            config.clone(),
            Arc::clone(&node_value),
            iteration,
        );

        Self {
            init_fn,
            compute_fn,
            node_batch,
            node_value,
            graph,  // ADD THIS
            messenger,
            // ... rest
```

## Edit 4: Update split() method - left_step creation

**Location**: Line ~182 in compute_step.rs

**Change**:

```rust
// BEFORE:
            let left_step = ComputeStep {
                init_fn: Arc::clone(&self.init_fn),
                compute_fn: Arc::clone(&self.compute_fn),
                node_batch: left_partition,
                node_value: Arc::clone(&self.node_value),
                messenger: Arc::clone(&self.messenger),
                vote_bits: Arc::clone(&self.vote_bits),
                iteration: self.iteration,
                current_node_id: 0,
                has_sent_message: Arc::clone(&self.has_sent_message),
                progress_tracker: Arc::clone(&self.progress_tracker),
                compute_context: ComputeContext::new(self.config.clone(), self.iteration),
                config: self.config.clone(),
            };

// AFTER:
            let left_step = ComputeStep {
                init_fn: Arc::clone(&self.init_fn),
                compute_fn: Arc::clone(&self.compute_fn),
                node_batch: left_partition,
                node_value: Arc::clone(&self.node_value),
                graph: Arc::clone(&self.graph),  // ADD THIS
                messenger: Arc::clone(&self.messenger),
                vote_bits: Arc::clone(&self.vote_bits),
                iteration: self.iteration,
                current_node_id: 0,
                has_sent_message: Arc::clone(&self.has_sent_message),
                progress_tracker: Arc::clone(&self.progress_tracker),
                compute_context: ComputeContext::new(
                    Arc::clone(&self.graph),
                    self.config.clone(),
                    Arc::clone(&self.node_value),
                    self.iteration,
                ),  // CHANGE THIS
                config: self.config.clone(),
            };
```

## Edit 5: Update compute_batch() - InitContext creation

**Location**: Line ~248 in compute_step.rs

**Change**:

```rust
// BEFORE:
                let mut init_ctx = InitContext::new(self.config.clone());
                init_ctx.set_node_id(node_id);

// AFTER:
                let mut init_ctx = InitContext::new(
                    Arc::clone(&self.graph),
                    self.config.clone(),
                    Arc::clone(&self.node_value),
                );
                init_ctx.set_node_id(node_id);
```

## Edit 6: Update node_value() getter (if exists)

**Location**: Line ~160 in compute_step.rs

**Change**:

```rust
// BEFORE:
    pub fn node_value(&self) -> &NodeValue {
        &self.node_value
    }

// AFTER:
    pub fn node_value(&self) -> parking_lot::RwLockReadGuard<NodeValue> {
        self.node_value.read()
    }
```

## Edit 7: Update Computer to pass graph and wrap NodeValue

**File**: `src/pregel/computer.rs`
**Location**: Line ~136

**Change**:

```rust
// BEFORE:
        self.root_task = Some(ComputeStep::new(
            Arc::clone(&self.init_fn),
            Arc::clone(&self.compute_fn),
            self.config.clone(),
            partition,
            Arc::clone(&self.node_values),
            Arc::clone(&self.messenger),
            Arc::clone(&self.vote_bits),
            iteration,
            Arc::clone(&self.sent_message),
            Arc::clone(&self.progress_tracker),
        ));

// AFTER:
        self.root_task = Some(ComputeStep::new(
            Arc::clone(&self.init_fn),
            Arc::clone(&self.compute_fn),
            self.config.clone(),
            Arc::clone(&self.graph),  // ADD THIS
            partition,
            Arc::clone(&self.node_values),  // Assuming Computer already has RwLock
            Arc::clone(&self.messenger),
            Arc::clone(&self.vote_bits),
            iteration,
            Arc::clone(&self.sent_message),
            Arc::clone(&self.progress_tracker),
        ));
```

## Edit 8: Update Computer's node_values field type (if needed)

**File**: `src/pregel/computer.rs`
**Location**: Line ~60

**Check if this needs changing**:

```rust
// If it's currently:
    node_values: Arc<NodeValue>,

// Change to:
    node_values: Arc<parking_lot::RwLock<NodeValue>>,
```

Then update Computer::new() to wrap it:

```rust
// When creating node_values:
node_values: Arc::new(parking_lot::RwLock::new(NodeValue::of(...)))
```

## Compilation Check Commands

After each edit, run:

```bash
cargo build --lib --features core 2>&1 | grep -E "error\[" | head -20
```

Expected outcome: All errors related to context creation should be resolved.

## Next Steps After Completion

1. Verify compilation succeeds
2. Add NodeValue read methods to ComputeContext (double_value, long_value)
3. Test with simple example
4. Wire messenger for message sending
5. Wire vote_bits for voting
