# The Real Issues We're Trying to Solve

## TL;DR

**The Gap**: We have PregelSchema and NodeValue (storage), but **no clear bridge** between:

1. PropertyStore (persistent, schema-constrained) → Pregel (ephemeral, algorithm-specific)
2. Algorithm definition (PregelComputation trait) → Runtime execution (how schema gets used)
3. User writes: `context.set_node_value("rank", value)` but where does "rank" come from? Who validates it?

**The Real Issues**:

1. ❌ **Schema disconnect**: PregelSchema exists but isn't tied to PropertyStore schema
2. ❌ **No validation**: Nothing prevents `context.set_node_value("typo_rank", value)` at runtime
3. ❌ **Manual wiring**: User must manually sync schema definition with compute function property access
4. ❌ **No PropertyStore integration**: Can't easily initialize from or persist to PropertyStore

---

## Issue #1: Schema Disconnect

### Current State (What Exists)

```rust
// User defines PregelSchema manually
impl PregelComputation for PageRank {
    fn schema(&self, _config: &Self::Config) -> PregelSchema {
        PregelSchema::builder()
            .add_public("rank", ValueType::Double)  // ← Manual string
            .build()
    }

    fn compute<I: MessageIterator>(...) {
        let rank = context.double_node_value("rank");  // ← Manual string (again!)
        context.set_node_value("rank", new_rank);      // ← Must match above
    }
}
```

### The Problem

**No compile-time safety!** Typos fail at runtime:

```rust
// Schema says "rank"
fn schema(&self) -> PregelSchema {
    PregelSchema::builder().add_public("rank", ValueType::Double).build()
}

// But I type "rnk" (typo!)
fn compute(...) {
    let rank = context.double_node_value("rnk");  // ← Runtime panic!
}
```

### What We Need

**Type-safe property accessors OR validation**:

```rust
// Option A: Typed accessors (compile-time)
context.rank().get()
context.rank().set(new_value)

// Option B: Validated at schema creation (runtime but early)
let schema = PregelSchema::builder()
    .add_public("rank", ValueType::Double)
    .validate_against_computation::<PageRank>()?;  // ← Check compute() uses "rank"
```

---

## Issue #2: PropertyStore Integration (The Real Pain Point)

### Current State

**NO INTEGRATION**. To initialize Pregel from PropertyStore, user must:

```rust
// 1. Define Pregel schema (manual)
let pregel_schema = PregelSchema::builder()
    .add_public("rank", ValueType::Double)
    .build();

// 2. Define PropertyStore schema (manual, separate)
graph.register_property(
    "seed_rank",
    PropertyDescriptor::new(1, "seed_rank", ValueType::Double)
)?;

// 3. Write values to PropertyStore (manual)
for node_id in 0..graph.node_count() {
    graph.set_node_property(node_id, "seed_rank", create_double(0.5))?;
}

// 4. Read from PropertyStore in init function (manual)
fn init(&mut self, context: &mut InitContext<Self::Config>) {
    // How do I get PropertyStore values here? I CAN'T!
    // InitContext has no access to PropertyStore!
}
```

### The Problem

**InitContext has no PropertyStore access!** Look at the signature:

```rust
pub struct InitContext<'a, C: PregelConfig> {
    node_id: u64,
    config: &'a C,
    node_values: &'a mut NodeValue,  // ← Only Pregel storage!
    // No graph, no PropertyStore!
}
```

### What We Need

**Two-way bridge**:

```rust
// A. Read PropertyStore → Pregel (optional initialization)
impl PregelComputation for PageRank {
    fn schema(&self) -> PregelSchema {
        PregelSchema::builder()
            .add_public("rank", ValueType::Double)
            .with_property_source("rank", "seed_rank")  // ← Link to PropertyStore
            .build()
    }

    // Init now has PropertyStore access via projection
    fn init(&mut self, context: &mut InitContext<Self::Config>) {
        // Auto-populated from "seed_rank" property if available
        let initial = context.double_node_value("rank")  // ← Already initialized!
            .unwrap_or(1.0);
    }
}

// B. Write Pregel → PropertyStore (persist results)
let result = pregel.run()?;
result.materialize_to_property_store(
    graph,
    "rank" → "computed_rank"  // ← Write back as "computed_rank"
)?;
```

---

## Issue #3: The Form Processor Connection

### Current State

**Two separate schema systems**:

```
PropertyStore Schema (FormProcessor):
├── PropertyDescriptor
├── Form validation
└── GdsValue wrapping

Pregel Schema (Separate):
├── PregelSchema
├── Element with Visibility
└── DefaultValue (not GdsValue!)
```

### The Problem

**No reuse!** PropertyDescriptor and PregelSchema.Element are parallel implementations:

```rust
// PropertyStore side
struct PropertyDescriptor {
    id: u64,
    key: String,
    value_type: ValueType,
    default_value: Option<Arc<dyn GdsValue>>,  // ← GdsValue
}

// Pregel side
struct Element {
    property_key: String,
    property_type: ValueType,
    visibility: Visibility,
    default_value: Option<DefaultValue>,  // ← DefaultValue (not GdsValue!)
}
```

### What We Need

**Unified descriptor OR clear projection**:

```rust
// Option A: PregelSchema wraps PropertyDescriptor
impl PregelSchema {
    pub fn from_property_descriptor(
        desc: &PropertyDescriptor,
        visibility: Visibility,
    ) -> Element {
        Element {
            property_key: desc.key.clone(),
            property_type: desc.value_type,
            visibility,
            default_value: desc.default_value
                .as_ref()
                .map(|gds| DefaultValue::from_gds(gds)),
        }
    }
}

// Option B: PropertyProjection functor (what we implemented)
impl PropertyProjection for DefaultValue {
    fn from_property(props: &dyn NodePropertyValues, id: u64) -> Option<Self> {
        // Convert NodePropertyValues → DefaultValue
    }
}
```

**Current status**: We implemented Option B (PropertyProjection), but it's **not wired into InitContext!**

---

## Issue #4: Example Shows The Gap

### What User Wants to Write

```rust
use rust_gds::prelude::*;

fn main() {
    let graph = load_graph("karate.gml")?;

    // Simple case: No PropertyStore, pure Pregel
    let ranks = graph.pregel()
        .pagerank()
        .max_iterations(20)
        .run()?;

    println!("Ranks: {:?}", ranks);
}
```

### What User Actually Has to Write (Current Reality)

```rust
use rust_gds::pregel::*;
use rust_gds::types::ValueType;

// 1. Define config struct
#[derive(Clone)]
struct PageRankConfig { max_iterations: usize }
impl PregelConfig for PageRankConfig { /* boilerplate */ }

// 2. Define computation struct
struct PageRank;
impl PregelComputation for PageRank {
    type Config = PageRankConfig;

    // 3. Define schema (manual strings)
    fn schema(&self, _config: &Self::Config) -> PregelSchema {
        PregelSchema::builder()
            .add_public("rank", ValueType::Double)
            .build()
    }

    // 4. Define init (manual strings)
    fn init(&mut self, context: &mut InitContext<Self::Config>) {
        context.set_node_value("rank", 1.0);
    }

    // 5. Define compute (manual strings)
    fn compute<I: MessageIterator>(...) {
        let rank = context.double_node_value("rank");
        let sum: f64 = messages.iter().sum();
        context.set_node_value("rank", 0.15 + 0.85 * sum);
        context.send_to_neighbors(rank / degree);
    }

    fn master_compute(...) -> bool { true }
}

fn main() {
    // 6. Create all the pieces manually
    let graph = Arc::new(load_graph("karate.gml")?);
    let config = PageRankConfig { max_iterations: 20 };
    let mut computation = PageRank;
    let schema = computation.schema(&config);

    // 7. Create messenger (complex!)
    let messenger = create_double_messenger(&graph, &config);

    // 8. Create init and compute closures (bridge trait to Pregel API)
    let init_fn = Box::new(|context: &mut InitContext<_>| {
        computation.init(context);
    });
    let compute_fn = Box::new(|context, messages| {
        computation.compute(context, messages);
    });

    // 9. Create progress tracker
    let tracker = Arc::new(ProgressTracker::new());

    // 10. Create Pregel executor
    let pregel = Pregel::new(
        graph,
        config,
        schema,
        init_fn,
        compute_fn,
        messenger,
        tracker,
    );

    // 11. Run
    let result = pregel.run();

    // 12. Extract results (manual property name again!)
    let ranks = result.node_values()
        .double_property("rank")?  // ← Manual string (4th time!)
        .to_vec();
}
```

**That's 100+ lines for PageRank!** Compare to Neo4j GDS:

```python
# Neo4j GDS (Python)
result = gds.pageRank.stream(graph, maxIterations=20)
print(result)
```

---

## The Real Questions

### Q1: Do we agree these are the issues?

1. ✅ **Schema disconnect**: Manual string keys, no type safety
2. ✅ **PropertyStore integration**: No automatic initialization from persistent properties
3. ✅ **Form Processor connection**: Two parallel schema systems (PropertyDescriptor vs Element)
4. ✅ **Ergonomics**: Way too much boilerplate for simple algorithms

### Q2: What's the priority?

**My proposal**:

1. **High priority**: PropertyStore integration (Issue #2)

   - Wire PropertyProjection into InitContext
   - Add `.with_property_source()` to schema builder
   - Add `.materialize_to_property_store()` to results

2. **Medium priority**: Ergonomics (Issue #4)

   - Add `.pagerank()` helper methods to GraphStore
   - Pre-defined algorithms as library functions
   - Builder pattern for common cases

3. **Lower priority**: Type safety (Issue #1)

   - Would require proc macros or significant refactoring
   - Runtime validation is "good enough" if we have good errors

4. **Optional**: Form Processor unification (Issue #3)
   - Keep separate for now (PropertyProjection bridges them)
   - Revisit if we see duplication problems

### Q3: Concrete next step?

**I propose**: Create a working PageRank example that shows PropertyStore integration:

```rust
// examples/pregel_pagerank_with_propertystore.rs

fn main() {
    // 1. Create graph and seed with PropertyStore values
    let graph = create_test_graph();
    graph.register_property("seed_rank", ...)?;
    // ... set initial ranks in PropertyStore

    // 2. Run Pregel with PropertyStore initialization
    let result = run_pagerank_with_propertystore(&graph, 20)?;

    // 3. Materialize results back to PropertyStore
    result.materialize_to_property_store(&graph, "computed_rank")?;

    // 4. Verify
    let final_ranks = graph.get_property_values("computed_rank")?;
    println!("Final ranks: {:?}", final_ranks);
}
```

This would demonstrate the **complete loop**: PropertyStore → Pregel → PropertyStore.

---

## Do You Agree?

**Let's align on**:

1. ✅ Are these the real issues?
2. ✅ What's the priority order?
3. ✅ Should we build the PropertyStore integration example next?

Once we agree, I'll implement the concrete solution (not just documentation!).
