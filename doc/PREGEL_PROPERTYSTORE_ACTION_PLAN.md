# Pregel PropertyStore Integration - Clear Action Plan

## TL;DR

**Problem**: Pregel InitContext has NO access to PropertyStore. Can't initialize algorithm state from persistent properties.

**Solution**: Wire PropertyProjection trait into Pregel initialization so PregelSchema can link to PropertyStore properties.

**Status**:

- ✅ PropertyProjection trait implemented (src/pregel/projection.rs)
- ❌ Not wired into Pregel executor
- ❌ PregelSchema has no property_source field

---

## What We're NOT Doing

**eval_macro / value_type_table!** = Separate system (future/speculative)

- Purpose: Unified PropertyDescriptor generation via macro DSL
- Status: Implemented but NOT needed for Pregel PropertyStore integration
- Don't confuse this with the Pregel issue!

---

## The Concrete Problem (Run the Demo!)

```bash
cargo run --example pregel_propertystore_demo --features core
```

This shows:

1. PropertyStore has "seed_rank" (persistent)
2. Pregel has "rank" (ephemeral)
3. **NO CONNECTION** between them
4. InitContext cannot access PropertyStore to read "seed_rank"

---

## The 5 Concrete Steps to Fix

### Step 1: Add property_source to Element

**File**: `src/pregel/schema.rs`

```rust
pub struct Element {
    pub property_key: String,
    pub property_type: ValueType,
    pub visibility: Visibility,
    pub default_value: Option<DefaultValue>,
    pub property_source: Option<String>,  // ← ADD THIS
}
```

### Step 2: Add .with_property_source() builder

**File**: `src/pregel/schema.rs`

```rust
impl PregelSchemaBuilder {
    pub fn with_property_source(
        mut self,
        property_key: impl Into<String>,
        source_key: impl Into<String>,
    ) -> Self {
        let key = property_key.into();
        // Find element and update property_source
        if let Some(element) = self.elements.iter_mut()
            .find(|e| e.property_key == key)
        {
            element.property_source = Some(source_key.into());
        }
        self
    }
}
```

### Step 3: Update Pregel::new() signature

**File**: `src/pregel/executor.rs`

```rust
// Current (Arc<dyn Graph> hides concrete type)
pub fn new(
    graph: Arc<dyn Graph>,
    ...
) -> Self

// Needed (GraphStore has PropertyStore access)
pub fn new(
    graph: &GraphStore,  // or Arc<GraphStore>
    ...
) -> Self
```

### Step 4: Wire PropertyProjection in initialization

**File**: `src/pregel/executor.rs`

```rust
impl Pregel {
    pub fn new(graph: &GraphStore, config, schema, init_fn, ...) -> Self {
        let node_values = NodeValue::of(&schema, graph.node_count(), concurrency);

        // NEW: Load PropertyStore values based on schema mappings
        for element in schema.elements() {
            if let Some(source_key) = &element.property_source {
                // Get PropertyStore values
                if let Ok(props) = graph.get_node_property_values(source_key) {
                    // Use PropertyProjection to convert
                    for node_id in 0..graph.node_count() {
                        if let Some(value) = DefaultValue::from_property(&*props, node_id) {
                            // Pre-populate NodeValue storage
                            node_values.set_value(node_id, &element.property_key, value);
                        }
                    }
                }
            }
        }

        // Continue with existing initialization...
    }
}
```

### Step 5: Add convenience method to PregelResult

**File**: `src/pregel/result.rs`

```rust
impl PregelResult {
    pub fn materialize_to_property_store(
        &self,
        graph: &mut GraphStore,
        property_key: &str,
        target_key: Option<&str>,
    ) -> Result<(), FormProcessorError> {
        let target = target_key.unwrap_or(property_key);

        // Get values from Pregel storage
        let values = self.node_values
            .read()
            .get_property_values(property_key)?;

        // Use existing materialize_pregel_values
        materialize_pregel_values(graph, target, values.iter_with_ids())
    }
}
```

---

## After Implementation: The New API

```rust
use rust_gds::prelude::*;

fn pagerank_with_propertystore(graph: &mut GraphStore) -> Result<Vec<f64>> {
    // Step 1: Define schema WITH PropertyStore link
    let schema = PregelSchema::builder()
        .add_public("rank", ValueType::Double)
        .with_property_source("rank", "seed_rank")  // ← Links to PropertyStore!
        .build();

    // Step 2: Run Pregel (auto-loads from PropertyStore)
    let result = Pregel::new(graph, config, schema, init, compute, ...)
        .run()?;

    // Step 3: Write results back
    result.materialize_to_property_store(
        graph,
        "rank",
        Some("computed_rank")  // Write as new property
    )?;

    Ok(result.node_values().double_property("rank")?.to_vec())
}
```

---

## Testing Strategy

### Unit Test

```rust
#[test]
fn test_property_source_loading() {
    let mut graph = GraphStore::new();

    // Set up PropertyStore
    graph.register_property("seed", PropertyDescriptor::new(1, "seed", ValueType::Double))?;
    for node in 0..10 {
        graph.set_node_property(node, "seed", create_double(node as f64 * 0.1))?;
    }

    // Create schema with source
    let schema = PregelSchema::builder()
        .add_public("value", ValueType::Double)
        .with_property_source("value", "seed")
        .build();

    // Create Pregel (should auto-load from "seed")
    let pregel = Pregel::new(&graph, config, schema, init, compute, ...);

    // Verify values loaded
    let node_values = pregel.node_values();
    assert_eq!(node_values.double_value(5, "value"), Some(0.5));
}
```

### Integration Test

```rust
#[test]
fn test_pagerank_with_propertystore() {
    let mut graph = create_test_graph();

    // Seed PropertyStore
    graph.register_property("seed_rank", ...)?;
    // ... set initial ranks

    // Run PageRank
    let ranks = pagerank_with_propertystore(&mut graph)?;

    // Verify PropertyStore updated
    let stored_ranks = graph.get_property_values("computed_rank")?;
    assert_eq!(stored_ranks.len(), graph.node_count());
}
```

---

## Timeline Estimate

- Step 1 (Add field): 10 minutes
- Step 2 (Builder method): 15 minutes
- Step 3 (Update signature): 30 minutes (may need refactoring)
- Step 4 (Wire projection): 1 hour (core logic + error handling)
- Step 5 (Convenience method): 20 minutes
- Testing: 1 hour
- **Total: ~3 hours of focused implementation**

---

## Do We Agree?

1. ✅ This is the right problem to solve (PropertyStore ↔ Pregel gap)?
2. ✅ These 5 steps are the right approach?
3. ✅ Ready to implement (not just document)?

**If yes, I'll start with Step 1!**
