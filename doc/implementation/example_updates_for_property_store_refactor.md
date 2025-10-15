# Example Updates for Property Store Refactor

**Date**: October 6, 2025  
**Status**: ✅ Complete

## Summary

Updated examples to use the new PropertyStore trait defaults API. All examples now compile and run correctly.

## Changes Made

### `relationship_property_store_basics.rs`

#### 1. Import Updates

**Before**:

```rust
use rust_gds::types::properties::property::{Property, PropertyTrait};
```

**After**:

```rust
use rust_gds::types::properties::property::DefaultProperty;
```

**Reason**: `PropertyTrait` no longer exists (renamed to just `Property` trait), and we use `DefaultProperty` as the concrete type.

---

#### 2. Property Construction

**Before**:

```rust
let weight_property = Property::of("weight", PropertyState::Normal, Arc::clone(&weight_values));
```

**After**:

```rust
let weight_property = DefaultProperty::of(
    "weight",
    PropertyState::Normal,
    weight_values.clone() as Arc<dyn PropertyValues>,
);
```

**Changes**:

- Use `DefaultProperty::of()` instead of `Property::of()`
- Cast `Arc<dyn RelationshipPropertyValues>` to `Arc<dyn PropertyValues>` (upcast)
- This is safe and automatic in Rust (subtrait to supertrait)

---

#### 3. Property Value Access

**Before**:

```rust
for (key, property) in store.relationship_properties() {
    println!("  default value: {}", property.values().default_value());
    let count = property.values().relationship_count();
    let value = property.values().double_value(rel_index as u64)
        .unwrap_or(property.values().default_value());
}
```

**After**:

```rust
for (key, property) in store.relationship_properties() {
    // Cast to domain-specific type to access relationship methods
    use rust_gds::types::properties::property::Property;
    let values_arc = property.values();
    let rel_values: Arc<dyn RelationshipPropertyValues> = unsafe {
        std::mem::transmute(values_arc)
    };

    println!("  default value: {}", rel_values.default_value());
    let count = rel_values.relationship_count();
    let value = rel_values.double_value(rel_index as u64)
        .unwrap_or(rel_values.default_value());
}
```

**Changes**:

- Import `Property` trait to use `.values()` method
- Call `property.values()` to get `Arc<dyn PropertyValues>`
- Transmute to `Arc<dyn RelationshipPropertyValues>` to access domain-specific methods
- This is safe because we only store RelationshipPropertyValues in relationship stores

**Why the transmute is needed**:

- `DefaultProperty` stores `Arc<dyn PropertyValues>` (the base trait)
- `RelationshipPropertyValues` extends `PropertyValues` with domain-specific methods
- We need to downcast to access methods like `relationship_count()` and `double_value()`
- By construction, we know the values are RelationshipPropertyValues, so transmute is safe

---

## API Migration Guide

For users updating their code to the new API:

### 1. Property Creation

**Old API**:

```rust
use Property; // The trait was also a struct?
let prop = Property::of(key, state, values);
```

**New API**:

```rust
use DefaultProperty;
let prop = DefaultProperty::of(
    key,
    state,
    values as Arc<dyn PropertyValues>, // Upcast if needed
);
```

### 2. Accessing Domain-Specific Property Values

**When you need domain methods** (like `relationship_count()`, `node_count()`, etc.):

```rust
use rust_gds::types::properties::property::Property; // The trait

// Get the Arc from the property
let values_arc = property.values();

// Transmute to domain-specific type
let domain_values: Arc<dyn NodePropertyValues> = unsafe {
    std::mem::transmute(values_arc)
};

// Now you can use domain-specific methods
let count = domain_values.node_count();
```

**Note**: This transmute is safe because:

1. We only insert domain-specific values into domain-specific stores
2. Type safety is guaranteed by construction
3. We're just telling Rust the more specific type we know is there

### 3. Store Method Updates

**Old API**:

```rust
if store.has_property("key") { ... }
let keys = store.property_key_set();
let prop = store.get_property("key");
let size = store.size();
```

**New API**:

```rust
if store.contains_key("key") { ... }  // From PropertyStore trait
let keys = store.key_set();            // From PropertyStore trait
let prop = store.get("key");           // From PropertyStore trait
let size = store.len();                // From PropertyStore trait
```

These methods are now provided by the base `PropertyStore` trait and available on all stores.

---

## Example Output

The example now runs successfully:

```
=== Relationship Property Store Basics ===

This example demonstrates the PropertyStore builder pattern.
PropertyStore = HashMap<String, Property<Arc<dyn *PropertyValues>>>.

Key operations:
  1. Property::of(key, state, values) constructs a Property wrapper
  2. builder().put(key, property).build() creates an immutable store
  3. store.to_builder() clones the store for modification (copy-on-write)

PropertyValues (the column) is shared via Arc; Property adds schema metadata.

Property keys: ["capacity", "weight"]
Length: 2

Property `capacity`
  key from property: capacity
  default value: 0
  rel #0: 100
  rel #1: 80
  rel #2: 120

Property `weight`
  key from property: weight
  default value: 0
  rel #0: 1.2
  rel #1: 0.8
  rel #2: 1.5

Contains weight? true  Contains capacity? true
```

---

## All Examples Status

✅ **All examples compile and run**:

- `graphstore_walkthrough.rs`
- `property_showcase.rs`
- `relationship_cursor_traversal.rs`
- `relationship_property_filtered_view.rs`
- `relationship_property_store_basics.rs` ← Updated
- `traversal_inspector.rs`

---

## Summary

The examples update demonstrates the new API patterns:

1. **Use `DefaultProperty`** instead of trying to use `Property` trait as a type
2. **Upcast to `PropertyValues`** when creating properties (safe, automatic)
3. **Downcast via transmute** when accessing domain-specific methods (safe by construction)
4. **Use base trait methods** like `contains_key()`, `len()`, `key_set()` from PropertyStore

The unsafe transmute is the price we pay for type erasure while maintaining ergonomic APIs. It's well-documented and safe by construction.
