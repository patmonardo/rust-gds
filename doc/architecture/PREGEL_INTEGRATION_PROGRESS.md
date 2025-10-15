# Pregel PropertyStore Integration - Progress Report

## ✅ Completed Steps

### Step 1: Add property_source field to Element ✅

**File**: `src/pregel/schema.rs`

```rust
pub struct Element {
    pub property_key: String,
    pub property_type: ValueType,
    pub visibility: Visibility,
    pub default_value: Option<DefaultValue>,
    pub property_source: Option<String>,  // ← NEW!
}
```

### Step 2: Add .with_property_source() builder method ✅

**File**: `src/pregel/schema.rs`

```rust
impl PregelSchemaBuilder {
    pub fn with_property_source(
        mut self,
        property_key: impl Into<String>,
        source_key: impl Into<String>,
    ) -> Self {
        // Links Pregel property to PropertyStore property
        // Enables automatic initialization
    }
}
```

**Tests**: All 7 tests passing including new property_source tests!

---

## Usage Example (Now Possible!)

```rust
let schema = PregelSchema::builder()
    .add("rank", ValueType::Double, Visibility::Public)
    .with_property_source("rank", "seed_rank")  // ← Link to PropertyStore!
    .build();

// Schema now knows: "rank" should initialize from "seed_rank" property
```

---

## 🚧 Remaining Steps

### Step 3: Update Pregel::new() to accept GraphStore

**Current Issue**: `Pregel::new()` takes `Arc<dyn Graph>` which hides the concrete type.
**Need**: Access to GraphStore to call `get_node_property_values()`.

**Options**:

1. **Change signature** to accept `&GraphStore` or `Arc<GraphStore>`
2. **Add trait method** to Graph trait: `fn get_node_property_values(&self, key: &str) -> Result<...>`
3. **Pass properties separately** as an optional parameter

**Recommendation**: Option 2 (add trait method) is cleanest - preserves abstraction.

### Step 4: Wire PropertyProjection in initialization

**Location**: `src/pregel/executor.rs` in `Pregel::new()`

**Pseudocode**:

```rust
impl Pregel {
    pub fn new(graph: Arc<dyn Graph>, schema: PregelSchema, ...) -> Self {
        let node_values = NodeValue::of(&schema, ...);

        // NEW: Load PropertyStore values based on schema mappings
        for element in schema.elements() {
            if let Some(source_key) = &element.property_source {
                if let Ok(props) = graph.get_node_property_values(source_key) {
                    for node_id in 0..graph.node_count() {
                        if let Some(value) = DefaultValue::from_property(&*props, node_id) {
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

### Step 5: Add PregelResult::materialize_to() convenience method

**Location**: `src/pregel/result.rs`

**API**:

```rust
impl PregelResult {
    pub fn materialize_to_property_store(
        &self,
        graph: &mut GraphStore,
        property_key: &str,
        target_key: Option<&str>,
    ) -> Result<(), FormProcessorError> {
        // Use existing materialize_pregel_values()
    }
}
```

---

## Next Decision Point

**Which approach for Step 3?**

### Option A: Add trait method to Graph

```rust
pub trait Graph {
    // ... existing methods

    fn get_node_property_values(&self, key: &str)
        -> Result<Arc<dyn NodePropertyValues>, PropertyError>;
}
```

**Pros**: Clean, preserves abstraction, no breaking changes to Pregel
**Cons**: Adds method to core Graph trait (but that's fine - it's useful!)

### Option B: Change Pregel to accept GraphStore directly

```rust
impl Pregel {
    pub fn new(
        graph: Arc<GraphStore>,  // ← Was Arc<dyn Graph>
        ...
    ) -> Self
}
```

**Pros**: Direct access, no trait changes
**Cons**: Ties Pregel to concrete GraphStore (less flexible)

### Option C: Pass properties as parameter

```rust
impl Pregel {
    pub fn new(
        graph: Arc<dyn Graph>,
        schema: PregelSchema,
        initial_properties: Option<HashMap<String, Arc<dyn NodePropertyValues>>>,
        ...
    ) -> Self
}
```

**Pros**: No trait changes, explicit
**Cons**: More complex API, user has to manually collect properties

---

## Recommendation: Option A

Add `get_node_property_values()` to the Graph trait. It's a natural extension and useful beyond just Pregel.

**Implementation**:

1. Add method to `Graph` trait in `src/types/graph/mod.rs`
2. Implement for `GraphStore` in `src/types/default_graph_store.rs`
3. Update `Pregel::new()` to use it

**Should I proceed with Option A?**
