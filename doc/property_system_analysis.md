# Property System Analysis & Standardization Plan

## Current State Analysis

### Property Type Definitions (Type Aliases)

All three domains use **identical type alias pattern**:

```rust
// Node
pub type NodeProperty = Property<Arc<dyn NodePropertyValues>>;

// Graph
pub type GraphProperty = Property<Arc<dyn GraphPropertyValues>>;

// Relationship
pub type RelationshipProperty = Property<Arc<dyn RelationshipPropertyValues>>;
```

✅ **Status**: Perfectly aligned! All use `Property<Arc<dyn ...Values>>` pattern.

### Generic Property Struct

The base `Property<V>` struct in `src/types/properties/property.rs`:

```rust
pub struct Property<V: PropertyValues> {
    values: V,
    schema: PropertySchema,
}
```

✅ **Fields**:

- `values: V` - The actual data (generic over PropertyValues trait)
- `schema: PropertySchema` - Metadata (key, value_type, default_value, state)

This is the "header" you mentioned - schema is metadata, values is the column data.

### Default Property Implementations

This is where we have **inconsistency**:

| Domain           | Default Implementation    | File Size      | Status                   |
| ---------------- | ------------------------- | -------------- | ------------------------ |
| **Node**         | ✅ `DefaultNodeProperty`  | 3,004 bytes    | Complete                 |
| **Graph**        | ✅ `DefaultGraphProperty` | 3,623 bytes    | Complete                 |
| **Relationship** | ❌ **MISSING**            | 1 byte (empty) | **NEEDS IMPLEMENTATION** |

### Why Have Default Implementations?

Looking at Node and Graph, they both have concrete `Default*Property` structs that:

1. **Wrap the generic Property** with domain-specific ergonomics
2. **Provide convenience constructors** (`of()`, `with_state()`, `with_default()`, `with_schema()`)
3. **Add domain-specific accessors** (like `values_arc()` for Graph)
4. **Simplify the API** for common use cases

**Node Implementation** (`DefaultNodeProperty`):

```rust
pub struct DefaultNodeProperty {
    values: Box<dyn NodePropertyValues>,  // ← Box, not Arc
    schema: PropertySchema,
}
```

**Graph Implementation** (`DefaultGraphProperty`):

```rust
pub struct DefaultGraphProperty {
    values: Arc<dyn GraphPropertyValues>,  // ← Arc (sharable)
    schema: PropertySchema,
}
```

### Key Observation: Box vs Arc

- **Node**: Uses `Box<dyn NodePropertyValues>` - single ownership
- **Graph**: Uses `Arc<dyn GraphPropertyValues>` - shared ownership
- **Relationship**: Should likely use `Arc<dyn RelationshipPropertyValues>` (relationships often shared)

## Issues Identified

### 1. Missing DefaultRelationshipProperty ❌

The file exists but is empty (1 byte). This breaks symmetry with Node and Graph.

### 2. Box vs Arc Inconsistency ⚠️

- Node uses `Box` (exclusive ownership)
- Graph uses `Arc` (shared ownership)
- Type aliases ALL use `Arc`

This creates confusion:

- `NodeProperty = Property<Arc<...>>` but `DefaultNodeProperty` uses `Box`
- Should we standardize on `Arc` for all three?

### 3. PropertyTrait Implementation Differences ⚠️

**Node's DefaultNodeProperty**:

```rust
impl PropertyTrait for DefaultNodeProperty {
    fn key(&self) -> &str { ... }
    fn property_state(&self) -> PropertyState { ... }
    fn property_schema(&self) -> &PropertySchema { ... }
    // Missing values() method!
}
```

**Graph's DefaultGraphProperty**:

```rust
impl PropertyTrait for DefaultGraphProperty {
    type Values = Arc<dyn GraphPropertyValues>;

    fn values(&self) -> &Self::Values { ... }
    fn property_schema(&self) -> &PropertySchema { ... }
}
```

Node's impl is **incomplete** - missing the `Values` associated type and `values()` method!

## Recommended Changes

### Goal: Perfect Triadic Symmetry

All three domains should have:

1. **Type alias** using `Arc<dyn ...PropertyValues>` ✅ (already done)
2. **Default implementation** struct with same structure
3. **Same convenience constructors** (`of()`, `with_state()`, `with_default()`, `with_schema()`)
4. **Complete PropertyTrait implementation** with Values associated type
5. **Same inherent methods** for ergonomic access

### Proposed Structure

```rust
// Standardized pattern for all three domains
pub struct Default{Node|Graph|Relationship}Property {
    values: Arc<dyn {Node|Graph|Relationship}PropertyValues>,  // ← Arc for all!
    schema: PropertySchema,
}

impl Default{...}Property {
    // Standard constructors (all three have these)
    pub fn of(key: impl Into<String>, values: Arc<dyn ...Values>) -> Self;
    pub fn with_state(key: impl Into<String>, state: PropertyState, values: Arc<dyn ...Values>) -> Self;
    pub fn with_default(key: impl Into<String>, state: PropertyState, values: Arc<dyn ...Values>, default: DefaultValue) -> Self;
    pub fn with_schema(schema: PropertySchema, values: Arc<dyn ...Values>) -> Self;

    // Standard accessors (all three have these)
    pub fn values(&self) -> &dyn ...PropertyValues;
    pub fn values_arc(&self) -> Arc<dyn ...PropertyValues>;
    pub fn property_schema(&self) -> &PropertySchema;
    pub fn key(&self) -> &str;
}

impl PropertyTrait for Default{...}Property {
    type Values = Arc<dyn ...PropertyValues>;

    fn values(&self) -> &Self::Values { &self.values }
    fn property_schema(&self) -> &PropertySchema { &self.schema }
}
```

### Change List

#### 1. Create DefaultRelationshipProperty ✨ (NEW)

**File**: `src/types/properties/relationship/impls/default_relationship_property.rs`

Clone the structure from `DefaultGraphProperty` but for relationships.

#### 2. Fix DefaultNodeProperty 🔧 (UPDATE)

**Changes needed**:

- Change `Box<dyn NodePropertyValues>` → `Arc<dyn NodePropertyValues>` for consistency
- Add `Values` associated type to PropertyTrait impl
- Fix `values()` method in PropertyTrait impl
- Add `values_arc()` method like Graph has
- Remove `values_box()` (no longer needed with Arc)

#### 3. Standardize Constructors ✅ (VERIFY)

Ensure all three have identical constructor signatures:

- `of(key, values)` - default state
- `with_state(key, state, values)` - explicit state
- `with_default(key, state, values, default)` - explicit default value
- `with_schema(schema, values)` - from existing schema

#### 4. Standardize Accessors ✅ (VERIFY)

All three should have:

- `values() -> &dyn ...PropertyValues`
- `values_arc() -> Arc<dyn ...PropertyValues>`
- `property_schema() -> &PropertySchema`
- `key() -> &str`

## Why Arc for All Three?

### Relationships Are Often Shared

In a graph database, relationship properties are frequently:

- Accessed by multiple relationship iterators
- Shared across partitions
- Read by concurrent queries
- Cached in multiple data structures

`Arc` enables cheap cloning and shared ownership, which is essential for:

- Property stores that don't consume ownership
- Cursors that hold references to properties
- Graph algorithms that traverse relationships

### Consistency Reduces Cognitive Load

If all three domains use `Arc`:

- Same mental model across Node/Graph/Relationship
- Same cloning behavior
- Same thread-safety guarantees
- Same API patterns

### Type Alias Alignment

The type aliases already use `Arc`:

```rust
pub type NodeProperty = Property<Arc<dyn NodePropertyValues>>;
pub type GraphProperty = Property<Arc<dyn GraphPropertyValues>>;
pub type RelationshipProperty = Property<Arc<dyn RelationshipPropertyValues>>;
```

Having `DefaultNodeProperty` use `Box` while `NodeProperty` uses `Arc` creates confusion.

## Implementation Strategy

### Phase 1: Create DefaultRelationshipProperty

1. Create file with complete implementation
2. Add comprehensive tests
3. Export from `relationship/impls/mod.rs`

### Phase 2: Fix DefaultNodeProperty

1. Change `Box` → `Arc` in struct definition
2. Fix PropertyTrait implementation
3. Update `values_box()` → `values_arc()`
4. Update all tests
5. Fix any dependent code

### Phase 3: Verification

1. Ensure all three have identical structure
2. Run full test suite
3. Update documentation

### Phase 4: Documentation

Create `doc/triadic_property_architecture.md` explaining:

- Property as "header" over column data
- Schema (metadata) vs Values (data)
- Triadic symmetry across Node/Graph/Relationship
- Arc-based shared ownership model
- Convenience constructors pattern

## Summary Table

| Aspect                 | Node    | Graph | Relationship | Target     |
| ---------------------- | ------- | ----- | ------------ | ---------- |
| Type alias             | Arc     | Arc   | Arc          | ✅ All Arc |
| Default impl exists    | Yes     | Yes   | **No**       | ✅ All Yes |
| Default impl storage   | **Box** | Arc   | -            | ✅ All Arc |
| Constructor count      | 2       | 4     | 0            | ✅ All 4   |
| PropertyTrait complete | **No**  | Yes   | -            | ✅ All Yes |
| values_arc() method    | **No**  | Yes   | -            | ✅ All Yes |

## Benefits of Standardization

1. **Predictable API** - Same pattern everywhere
2. **Easy Learning** - Know one, know all three
3. **Maintainability** - Changes apply symmetrically
4. **Extensibility** - Clear template for new storage backends
5. **Thread Safety** - Arc enables safe sharing
6. **Performance** - Arc cloning is O(1) pointer copy

## Next Steps

1. ✅ Get approval for Arc-based standardization
2. 🔨 Implement DefaultRelationshipProperty
3. 🔨 Fix DefaultNodeProperty to use Arc
4. ✅ Verify all three are symmetric
5. 📝 Document the triadic property architecture
