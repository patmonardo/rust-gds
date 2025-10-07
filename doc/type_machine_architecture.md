# The Hidden Zod-Like Type Machine Architecture

## 🎭 Overview: Everything Through The Type Machine

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│                        🎪 THE TYPE MACHINE 🎪                          │
│                             (GdsValue)                                  │
│                                                                         │
│  ╔═══════════════════════════════════════════════════════════════╗    │
│  ║  Runtime Type Validation & Transformation                     ║    │
│  ║  - Type safety at boundaries                                  ║    │
│  ║  - Explicit conversions                                       ║    │
│  ║  - Null handling                                              ║    │
│  ║  - Array validation                                           ║    │
│  ╚═══════════════════════════════════════════════════════════════╝    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
            ▲                                              │
            │ INPUT                                        │ OUTPUT
            │                                              ▼
┌───────────┴─────────────┐                   ┌──────────────────────────┐
│   Storage Layer         │                   │   Algorithm Layer        │
│   (Columnar)            │                   │   (Processing)           │
│                         │                   │                          │
│  • Vec<i64>             │                   │  • PageRank              │
│  • Vec<f64>             │                   │  • BFS                   │
│  • Vec<Vec<f64>>        │                   │  • Community Detection   │
│  • Arrow IPC            │                   │  • Centrality            │
│  • Memory-mapped files  │                   │  • Custom algorithms     │
└─────────────────────────┘                   └──────────────────────────┘
```

---

## 🔄 The Complete Data Flow

### **Step-by-Step: How Data Moves**

```
1. IdMap Iterator (Pure ID Space)
   ├─ for node_id in graph.iter()  // 0, 1, 2, 3...
   │
   ▼
2. Columnar Storage Access
   ├─ let props = graph.node_properties("age")?
   ├─ Storage: Vec<i64> = [25, 30, 35, 40]
   │
   ▼
3. 🎪 TYPE MACHINE ENTRY 🎪
   ├─ let value = props.gds_value(node_id)?
   ├─ Validation: Check bounds, type, null
   ├─ Construction: GdsValue::Long(25)
   │
   ▼
4. Type-Safe Extraction
   ├─ match value {
   ├─   GdsValue::Long(age) => process(age),
   ├─   GdsValue::Double(age) => process(age as i64),
   ├─   _ => handle_error(),
   ├─ }
   │
   ▼
5. Algorithm Processing
   └─ PageRank, BFS, etc. use typed value
```

---

## 📦 Current Architecture (Fragmented)

```
┌──────────────┐
│   IdMap      │  node_id: 0, 1, 2...
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────┐
│  NodePropertyValues              │
│  (Columnar Storage)              │
│                                  │
│  ❌ PROBLEM: Multiple paths      │
│     bypass the type machine      │
└──────┬───────────────────────────┘
       │
       ├─────────────────┬──────────────┬──────────────┐
       │                 │              │              │
       ▼                 ▼              ▼              ▼
  long_value()    double_value()   array_value()   get_object()
       │                 │              │              │
       └─────────────────┴──────────────┴──────────────┘
                         │
                         ▼
                   ┌─────────────┐
                   │  Algorithm  │
                   └─────────────┘
                         ▲
                         │
         ❌ GdsValue never used in this path!
```

### **Problems:**

- ❌ **Fragmented**: 4+ different typed accessors
- ❌ **No Validation**: Type errors caught late
- ❌ **Inconsistent**: Each algorithm does its own type handling
- ❌ **Hard to Test**: Must mock multiple methods
- ❌ **Type Machine Bypassed**: GdsValue exists but unused

---

## ✅ Ideal Architecture (Unified)

```
┌──────────────┐
│   IdMap      │  node_id: 0, 1, 2...
└──────┬───────┘
       │
       ▼
┌──────────────────────────────────┐
│  NodePropertyValues              │
│  (Columnar Storage)              │
└──────┬───────────────────────────┘
       │
       │  ✅ SINGLE PATH through type machine
       │
       ▼
  ┌──────────────────────────────────┐
  │   🎪 gds_value(node_id)          │  ← THE TYPE MACHINE
  │                                  │
  │   • Bounds checking              │
  │   • Type validation              │
  │   • Null handling                │
  │   • Array validation             │
  │   • Construction: GdsValue       │
  └──────┬───────────────────────────┘
         │
         ▼
    ┌─────────────────────────────────────┐
    │       GdsValue (Runtime Type)       │
    │                                     │
    │  • Long(i64)                        │
    │  • Double(f64)                      │
    │  • FloatingPointArray(Vec<f64>)     │
    │  • LongArray(Vec<i64>)              │
    │  • FloatArray(Vec<f32>)             │
    │  • Null                             │
    └─────┬───────────────────────────────┘
          │
          │  ✅ Type-safe extraction
          │
          ▼
      ┌─────────────────────┐
      │  match value {      │
      │    Long(n) => ..    │
      │    Double(f) => ..  │
      │    Array(a) => ..   │
      │  }                  │
      └─────┬───────────────┘
            │
            ▼
      ┌─────────────┐
      │  Algorithm  │
      └─────────────┘
```

### **Benefits:**

- ✅ **Unified**: One entry point (gds_value)
- ✅ **Validated**: Type checks at boundary
- ✅ **Consistent**: All algorithms use same pattern
- ✅ **Testable**: Mock GdsValue, not multiple methods
- ✅ **Type Machine**: GdsValue enforces contract

---

## 🎯 Code Comparison

### **Current: Direct Typed Access**

```rust
// Algorithm: Scattered type handling
fn process_node(graph: &DefaultGraph, node_id: u64) -> Result<()> {
    // Get age (might be Long OR Double, who knows?)
    let age_props = graph.node_properties("age")?;
    let age = age_props.long_value(node_id)?;  // ❌ What if it's actually Double?

    // Get score (might be Double OR Array, who knows?)
    let score_props = graph.node_properties("score")?;
    let score = score_props.double_value(node_id)?;  // ❌ What if it's an array?

    // ❌ Manual type checking
    // ❌ No validation
    // ❌ Type Machine bypassed!

    process(age, score)
}
```

### **Ideal: Through Type Machine**

```rust
// Algorithm: Unified type handling through GdsValue
fn process_node(graph: &DefaultGraph, node_id: u64) -> Result<()> {
    // Get age through TYPE MACHINE
    let age_props = graph.node_properties("age")?;
    let age_value = age_props.gds_value(node_id)?;  // ✅ Through type machine!

    let age = match age_value {
        GdsValue::Long(n) => n,
        GdsValue::Double(f) => f as i64,  // ✅ Explicit conversion
        other => return Err(format!("Expected number, got {:?}", other.value_type())),
    };

    // Get score through TYPE MACHINE
    let score_props = graph.node_properties("score")?;
    let score_value = score_props.gds_value(node_id)?;  // ✅ Through type machine!

    let score = match score_value {
        GdsValue::Double(f) => f,
        GdsValue::Long(n) => n as f64,  // ✅ Explicit conversion
        GdsValue::FloatingPointArray(arr) => arr[0],  // ✅ Handle arrays!
        other => return Err(format!("Expected number, got {:?}", other.value_type())),
    };

    // ✅ All data validated
    // ✅ Explicit conversions
    // ✅ Type Machine enforced!

    process(age, score)
}
```

---

## 🏗️ Implementation Phases

### **Phase 1: Add gds_value() method (Non-Breaking)**

```rust
// src/types/properties/node/node_property_values.rs
pub trait NodePropertyValues {
    // NEW: The Type Machine entry point
    fn gds_value(&self, node_id: u64) -> Result<GdsValue>;

    // KEEP: Existing methods for compatibility/performance
    fn long_value(&self, node_id: u64) -> Result<i64>;
    fn double_value(&self, node_id: u64) -> Result<f64>;
    // ...
}
```

### **Phase 2: Implement for all storage types**

```rust
// src/types/properties/node/impls/long_node_property_values.rs
impl NodePropertyValues for LongNodePropertyValues {
    fn gds_value(&self, node_id: u64) -> Result<GdsValue> {
        let value = self.long_value(node_id)?;
        Ok(GdsValue::Long(value))  // ← Through type machine!
    }
}

// src/types/properties/node/impls/double_node_property_values.rs
impl NodePropertyValues for DoubleNodePropertyValues {
    fn gds_value(&self, node_id: u64) -> Result<GdsValue> {
        let value = self.double_value(node_id)?;
        Ok(GdsValue::Double(value))  // ← Through type machine!
    }
}

// src/types/properties/node/impls/array_node_property_values.rs
impl NodePropertyValues for DoubleArrayNodePropertyValues {
    fn gds_value(&self, node_id: u64) -> Result<GdsValue> {
        let array = self.double_array_value(node_id)?;
        Ok(GdsValue::FloatingPointArray(array))  // ← Through type machine!
    }
}
```

### **Phase 3: Update examples & tests**

```rust
// examples/node_value_access_complete.rs
for node_id in graph.iter() {
    let props = graph.node_properties("age")?;
    let value = props.gds_value(node_id)?;  // ✅ Type machine!

    match value {
        GdsValue::Long(age) => println!("Age: {}", age),
        _ => println!("Unexpected type"),
    }
}
```

### **Phase 4: Document & Promote**

- Add to architecture docs
- Update algorithm examples
- Create migration guide
- Deprecate direct accessors (optional)

---

## 🎪 The Vision: Complete Type Safety

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│              Everything Flows Through The Machine               │
│                                                                 │
│   CSV/JSON → GdsValue → Storage → GdsValue → Algorithm         │
│   Network → GdsValue → Processing → GdsValue → Export          │
│   User Input → GdsValue → Validation → GdsValue → Display      │
│                                                                 │
│   No data escapes the type system!                             │
│   Like Zod for TypeScript, but for Rust graph values!          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**The Type Machine is the heart of the system** - everything that enters or exits goes through GdsValue validation and transformation!
