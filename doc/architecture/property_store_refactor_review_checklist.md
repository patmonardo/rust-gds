# PropertyStore Trait Defaults Refactor - Review Checklist

**Date**: October 6, 2025  
**Reviewer**: Pat  
**Status**: Ready for Review

---

## Overview

This refactor eliminates ~480 lines of duplicate code by using Rust trait default methods. The goal: **triadic symmetry** - identical patterns across Node, Graph, and Relationship property stores.

**Remember**: At the end of the day, we're just mounting Arrow2 files (fancy CSV files 😉). This structure makes that simple and symmetric.

---

## 1. Base PropertyStore Trait ✓/✗

**File**: `src/types/properties/property_store.rs`

### Check:

- [ ] Trait has clean `type Property: Property` associated type (no complex generic bounds)
- [ ] Trait requires only ONE method: `fn properties(&self) -> &HashMap<String, Self::Property>`
- [ ] Trait provides 5 default implementations:
  - [ ] `get(property_key)` - returns `Option<&Self::Property>`
  - [ ] `is_empty()` - returns `bool`
  - [ ] `len()` - returns `usize`
  - [ ] `key_set()` - returns `Vec<&str>`
  - [ ] `contains_key(property_key)` - returns `bool`
- [ ] All defaults are implemented using only `self.properties()`
- [ ] No unused imports (PropertyValues import was removed)

### Expected Pattern:

```rust
pub trait PropertyStore: Send + Sync {
    type Property: Property;

    fn properties(&self) -> &HashMap<String, Self::Property>;

    fn get(&self, property_key: &str) -> Option<&Self::Property> {
        self.properties().get(property_key)
    }
    // ... 4 more defaults
}
```

**Notes**:

- This is the foundation - all domain stores build on this
- Default methods mean concrete stores only implement `properties()`
- No runtime cost - defaults inline at compile time

---

## 2. Domain Trait Extensions ✓/✗

### 2.A Node Property Store

**File**: `src/types/properties/node/node_property_store.rs`

- [ ] Trait declaration: `pub trait NodePropertyStore: PropertyStore`
- [ ] Imports PropertyStore: `use crate::types::properties::property_store::PropertyStore;`
- [ ] No `type Property` declaration (inherited from PropertyStore)
- [ ] Has `type Builder` associated type
- [ ] Has factory methods: `empty()`, `new()`, `builder()`
- [ ] Domain-specific methods ONLY:
  - [ ] `get_all_properties()` - returns `Vec<&Self::Property>`
  - [ ] `get_property_values()` - returns `Option<&dyn NodePropertyValues>`
  - [ ] `to_builder()` - returns `Self::Builder`
- [ ] Removed methods (now in base trait):
  - [ ] ~~`has_property()`~~ → use `contains_key()` from base
  - [ ] ~~`property_key_set()`~~ → use `key_set()` from base
  - [ ] ~~`get_property()`~~ → use `get()` from base
  - [ ] ~~`size()`~~ → use `len()` from base
  - [ ] ~~`is_empty()`~~ → inherited from base

### 2.B Graph Property Store

**File**: `src/types/properties/graph/graph_property_store.rs`

- [ ] Trait declaration: `pub trait GraphPropertyStore: PropertyStore`
- [ ] Imports PropertyStore
- [ ] Same pattern as Node (factory methods + 3 domain methods)
- [ ] Removed duplicate methods (same list as Node)

### 2.C Relationship Property Store

**File**: `src/types/properties/relationship/relationship_property_store.rs`

- [ ] Trait declaration: `pub trait RelationshipPropertyStore: PropertyStore`
- [ ] Imports PropertyStore
- [ ] Same pattern as Node/Graph
- [ ] **Verify**: All three domain traits look nearly identical (triadic symmetry)

**Notes**:

- Each domain adds ~3 domain-specific methods
- All map-like operations inherited from PropertyStore
- Callers can use both base trait methods and domain methods

---

## 3. Concrete Store Implementations ✓/✗

### 3.A DefaultNodePropertyStore

**File**: `src/types/properties/node/impls/default_node_property_store.rs`

#### Check Imports:

- [ ] Imports PropertyStore: `use crate::types::properties::property_store::PropertyStore;`
- [ ] Imports NodePropertyStore and Builder traits
- [ ] NO import of `Property` trait (was removed)

#### Check PropertyStore Implementation:

```rust
impl PropertyStore for DefaultNodePropertyStore {
    type Property = NodeProperty;

    fn properties(&self) -> &HashMap<String, Self::Property> {
        &self.properties
    }
}
```

- [ ] Only implements `properties()` method
- [ ] Returns `&self.properties` directly
- [ ] Clean, simple, ~7 lines total

#### Check NodePropertyStore Implementation:

- [ ] `type Builder = DefaultNodePropertyStoreBuilder;`
- [ ] Factory methods: `empty()`, `new()`, `builder()`
- [ ] Domain methods only (3 methods):
  - [ ] `get_all_properties()` → `self.properties.values().collect()`
  - [ ] `get_property_values()` → with unsafe transmute (see below)
  - [ ] `to_builder()` → clones properties to builder
- [ ] NO implementations of base trait methods

#### Check get_property_values() Implementation:

```rust
fn get_property_values(&self, property_key: &str) -> Option<&dyn NodePropertyValues> {
    self.properties.get(property_key).map(|p| {
        let trait_obj: &dyn crate::types::properties::property_values::PropertyValues = &*p.values;
        // SAFETY: By construction, NodeProperty only stores NodePropertyValues
        unsafe {
            std::mem::transmute::<&dyn PropertyValues, &dyn NodePropertyValues>(trait_obj)
        }
    })
}
```

- [ ] Accesses field directly: `&*p.values`
- [ ] Has SAFETY comment explaining why transmute is safe
- [ ] Transmutes from `&dyn PropertyValues` to `&dyn NodePropertyValues`

#### Check Tests:

- [ ] Tests use base trait methods: `store.contains_key()`, `store.len()`, `store.key_set()`
- [ ] No tests calling old method names like `has_property()`, `size()`

#### Line Count Check:

- [ ] Implementation section ~80 lines (was ~240 before)
- [ ] ~67% code reduction achieved

### 3.B DefaultGraphPropertyStore

**File**: `src/types/properties/graph/impls/default_graph_property_store.rs`

- [ ] Same structure as NodePropertyStore (PropertyStore impl + GraphPropertyStore impl)
- [ ] PropertyStore impl: only `properties()` method
- [ ] GraphPropertyStore impl: 3 domain methods
- [ ] `get_property_values()` has unsafe transmute with SAFETY comment
- [ ] Builder impl uses `DefaultProperty::of()` (not `Property::of()`)
- [ ] ~70 lines for implementations

### 3.C DefaultRelationshipPropertyStore

**File**: `src/types/properties/relationship/impls/default_relationship_property_store.rs`

- [ ] Same structure as Node/Graph
- [ ] User manually edited this file - check it matches pattern
- [ ] PropertyStore impl: only `properties()` method
- [ ] RelationshipPropertyStore impl: 3 domain methods
- [ ] `get_property_values()` has unsafe transmute with SAFETY comment
- [ ] Builder impl uses `DefaultProperty::of()` (not `Property::of()`)
- [ ] ~70 lines for implementations

**Notes**:

- All three concrete stores should look nearly identical
- Total code reduction: ~480 lines across all three
- The unsafe transmute is safe because we only insert domain-specific values

---

## 4. Property Type Aliases ✓/✗

### 4.A NodeProperty

**File**: `src/types/properties/node/node_property.rs`

- [ ] Type alias: `pub type NodeProperty = DefaultProperty;`
- [ ] Factory function: `pub fn node_property_of(...) -> NodeProperty`
- [ ] Uses DefaultProperty::of() internally
- [ ] NO unused Property trait import

### 4.B GraphProperty

**File**: `src/types/properties/graph/graph_property.rs`

- [ ] Type alias: `pub type GraphProperty = DefaultProperty;`
- [ ] Factory function: `pub fn graph_property_of(...) -> GraphProperty`
- [ ] NOT using old generic: ~~`Property<Arc<dyn GraphPropertyValues>>`~~

### 4.C RelationshipProperty

**File**: `src/types/properties/relationship/relationship_property.rs`

- [ ] Type alias: `pub type RelationshipProperty = DefaultProperty;`
- [ ] Factory function: `pub fn relationship_property_of(...) -> RelationshipProperty`
- [ ] NOT using old generic: ~~`Property<Arc<dyn RelationshipPropertyValues>>`~~

**Notes**:

- All three domains use same concrete type: DefaultProperty
- Factory functions provide ergonomic constructors
- No more confusion between Property trait and Property struct

---

## 5. DefaultProperty Struct ✓/✗

**File**: `src/types/properties/property.rs`

### Check Structure:

```rust
pub struct DefaultProperty {
    pub schema: PropertySchema,
    pub values: Arc<dyn PropertyValues>,
}
```

- [ ] Two public fields: `schema` and `values`
- [ ] Derives: `Clone, Debug`
- [ ] Debug works because PropertyValues now has Debug bound

### Check Constructors:

- [ ] `new(schema, values)` - basic constructor
- [ ] `of(key, state, values)` - with auto default value
- [ ] `with_default(key, state, values, default_value)` - explicit default

### Check Convenience Methods:

- [ ] `key(&self) -> &str` - delegates to `self.schema.key()`
- [ ] `value_type(&self) -> ValueType` - delegates to `self.schema.value_type()`
- [ ] `property_state(&self) -> PropertyState` - delegates to `self.schema.state()`
- [ ] `default_value(&self) -> &DefaultValue` - delegates to `self.schema.default_value()`

### Check Property Trait Implementation:

```rust
impl Property for DefaultProperty {
    fn schema(&self) -> &PropertySchema { &self.schema }
    fn values(&self) -> Arc<dyn PropertyValues> { Arc::clone(&self.values) }
}
```

- [ ] Implements the Property trait
- [ ] `values()` returns cloned Arc (not a reference)
- [ ] This is the method called in default_graph.rs

**Notes**:

- Public fields allow direct access when needed
- Trait method provides Arc cloning when needed for transmute
- Convenience methods make it ergonomic to use

---

## 6. PropertyValues Trait ✓/✗

**File**: `src/types/properties/property_values.rs`

- [ ] Trait signature: `pub trait PropertyValues: Send + Sync + std::fmt::Debug`
- [ ] Debug bound added to enable `#[derive(Debug)]` on DefaultProperty
- [ ] No other changes to the trait

**Notes**:

- Single line change but critical for derivable Debug
- All concrete PropertyValues types already implemented Debug

---

## 7. Usage Sites ✓/✗

### 7.A default_graph.rs

**File**: `src/types/graph/default_graph.rs`

- [ ] Imports Property trait: `use crate::types::properties::property::Property;`
- [ ] Uses trait method: `property.values()` (returns Arc)
- [ ] Transmutes Arc with SAFETY comment:

```rust
let values_arc = property.values();
let rel_values = unsafe {
    std::mem::transmute::<
        Arc<dyn PropertyValues>,
        Arc<dyn RelationshipPropertyValues>
    >(values_arc)
};
```

- [ ] User manually edited this file - verify changes look good

### 7.B default_graph_store.rs

**File**: `src/types/graph_store/default_graph_store.rs`

- [ ] Uses `store.contains_key()` instead of old `store.has_property()`
- [ ] Uses transmute for Arc type conversion with SAFETY comment
- [ ] User manually edited this file - verify changes

### 7.C Other Files

- [ ] Check any other files that use property stores
- [ ] Verify they use base trait methods: `contains_key()`, `len()`, `key_set()`, `get()`
- [ ] NOT old domain methods: ~~`has_property()`~~, ~~`size()`~~, ~~`property_key_set()`~~

---

## 8. Tests ✓/✗

### Run Tests:

```bash
cargo test --lib
```

- [ ] All 174 tests passing
- [ ] No test failures
- [ ] No test warnings

### Check Specific Test Updates:

**File**: `src/types/properties/node/impls/default_node_property_store.rs` (tests module)

- [ ] Uses `store.len()` not `store.size()`
- [ ] Uses `store.contains_key()` not `store.has_property()`
- [ ] Uses `store.key_set()` not `store.property_key_set()`

---

## 9. Code Quality ✓/✗

### Run Clippy:

```bash
cargo clippy --lib
```

- [ ] Zero clippy warnings
- [ ] No unused imports
- [ ] No dead code warnings

### Check Documentation:

- [ ] PropertyStore trait has doc comments on defaults
- [ ] SAFETY comments on all unsafe transmute operations
- [ ] Domain traits have doc comments explaining purpose

---

## 10. Architecture Verification ✓/✗

### Triadic Symmetry Check:

Compare these three files side-by-side:

- `src/types/properties/node/impls/default_node_property_store.rs`
- `src/types/properties/graph/impls/default_graph_property_store.rs`
- `src/types/properties/relationship/impls/default_relationship_property_store.rs`

- [ ] All three have identical structure:
  - PropertyStore impl with only `properties()` method
  - Domain trait impl with factory methods + 3 domain methods
  - Builder implementation
  - Inherent convenience methods
  - Tests
- [ ] Line counts similar (~70-80 lines for trait impls)
- [ ] No copy-paste drift between the three

### DRY Principle Check:

- [ ] Zero duplication of map-like operations (get, len, contains_key, etc.)
- [ ] All implementations in ONE place (PropertyStore trait defaults)
- [ ] Changes to base trait automatically propagate to all three domains

---

## 11. Safety Review ✓/✗

### Unsafe Transmute Usage:

All transmutes follow this pattern:

```rust
// SAFETY: By construction, {Domain}Property only stores {Domain}PropertyValues
unsafe {
    std::mem::transmute::<&dyn PropertyValues, &dyn {Domain}PropertyValues>(trait_obj)
}
```

For each transmute location:

- [ ] Node: `default_node_property_store.rs` line ~58
- [ ] Graph: `default_graph_property_store.rs` line ~56
- [ ] Relationship: `default_relationship_property_store.rs` line ~54
- [ ] Graph usage: `default_graph.rs` line ~353
- [ ] GraphStore usage: `default_graph_store.rs` line ~456

Check each:

- [ ] Has SAFETY comment explaining why it's safe
- [ ] Only transmuting between PropertyValues supertrait and domain subtrait
- [ ] Type safety guaranteed by construction (we only insert domain values)

---

## 12. Final Verification ✓/✗

### Build and Test:

```bash
# Clean build
cargo clean
cargo build --lib

# Full test suite
cargo test --lib

# Clippy
cargo clippy --lib

# Check examples compile
cargo check --examples
```

- [ ] Clean build succeeds
- [ ] All tests pass (174 tests)
- [ ] Zero clippy warnings
- [ ] Examples compile

### Documentation Check:

- [ ] Review `doc/property_store_trait_defaults_refactor.md`
- [ ] Verify it accurately describes the changes
- [ ] Check all code snippets in doc match actual code

---

## Summary Metrics

**Before Refactor**:

- DefaultNodePropertyStore: ~240 lines
- DefaultGraphPropertyStore: ~167 lines
- DefaultRelationshipPropertyStore: ~168 lines
- **Total**: ~575 lines of implementation code

**After Refactor**:

- DefaultNodePropertyStore: ~80 lines
- DefaultGraphPropertyStore: ~70 lines
- DefaultRelationshipPropertyStore: ~70 lines
- **Total**: ~220 lines of implementation code

**Code Reduction**: ~355 lines eliminated (~62% reduction)

---

## Sign-off

- [ ] All checklist items reviewed
- [ ] Architecture makes sense
- [ ] Unsafe code is justified and documented
- [ ] Tests pass
- [ ] Ready to commit

**Reviewer**: ******\_\_\_\_******  
**Date**: ******\_\_\_\_******  
**Notes**:

---

## Quick Reference: Method Name Mapping

| Old Domain Method    | New Base Trait Method | Where         |
| -------------------- | --------------------- | ------------- |
| `has_property(key)`  | `contains_key(key)`   | PropertyStore |
| `property_key_set()` | `key_set()`           | PropertyStore |
| `get_property(key)`  | `get(key)`            | PropertyStore |
| `size()`             | `len()`               | PropertyStore |
| `is_empty()`         | `is_empty()`          | PropertyStore |

Domain-specific methods (stay in domain traits):

- `get_all_properties()` - returns all property references
- `get_property_values(key)` - returns domain-specific PropertyValues
- `to_builder()` - converts store to builder

---

**Remember**: This is just organizing our Arrow2 file mounts! The complexity is in the type system ensuring we don't mix Node/Graph/Rel values. The actual data is just columnar arrays. 📊
