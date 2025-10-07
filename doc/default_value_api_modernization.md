# DefaultValue API Modernization

## Summary

The `DefaultValue` type has evolved from an enum-based API to a modern builder pattern with ergonomic constructors.

## Old Pattern (deprecated)

```rust
// ❌ Ancient enum-style constructors (no longer available)
let value = DefaultValue::Long(42);
let value = DefaultValue::Double(3.14);
let value = DefaultValue::String("hello".to_string());
```

## Modern Pattern (current)

```rust
// ✅ Modern ergonomic constructors
let value = DefaultValue::long(42);
let value = DefaultValue::double(3.14);
let value = DefaultValue::string("hello");  // accepts impl Into<String>
let value = DefaultValue::int(100);
let value = DefaultValue::float(2.5);
let value = DefaultValue::boolean(true);
```

## Key Differences

### 1. Lowercase method names

- Old: `DefaultValue::Long(42)` — enum variant style
- New: `DefaultValue::long(42)` — method call style

### 2. Automatic user-defined flag

Modern constructors automatically mark values as `user_defined`:

```rust
let val = DefaultValue::long(42);
assert!(val.is_user_defined());
```

### 3. System defaults (for type inference)

Use `DefaultValue::of(ValueType)` or specific `for_*()` methods:

```rust
let system_default = DefaultValue::of(ValueType::Long);    // system default for Long
let long_default = DefaultValue::for_long();               // system default: 0
let double_default = DefaultValue::for_double();           // system default: NaN
```

### 4. Ergonomic string handling

```rust
// Old: required .to_string()
let s = DefaultValue::String("hello".to_string());

// New: accepts impl Into<String>
let s = DefaultValue::string("hello");  // ✅ No .to_string() needed
```

## Complete Modern API

### User-defined values

```rust
DefaultValue::long(i64)
DefaultValue::int(i32)
DefaultValue::double(f64)
DefaultValue::float(f32)
DefaultValue::string(impl Into<String>)
DefaultValue::boolean(bool)
DefaultValue::long_array(Vec<i64>)
DefaultValue::double_array(Vec<f64>)
DefaultValue::float_array(Vec<f32>)
```

### System defaults

```rust
DefaultValue::of(ValueType)         // Auto-generate for any type
DefaultValue::for_long()            // 0
DefaultValue::for_double()          // NaN
DefaultValue::for_float()           // NaN
DefaultValue::for_int()             // -2147483648
DefaultValue::for_long_array()      // empty/null
DefaultValue::for_double_array()    // empty/null
DefaultValue::for_float_array()     // empty/null
```

### Advanced constructors

```rust
DefaultValue::create(Option<JsonValue>, is_user_defined: bool)
DefaultValue::user_defined(Option<JsonValue>)
DefaultValue::system_default(Option<JsonValue>)
```

## Migration Checklist

- [x] `src/types/properties/node/impls/default_node_property.rs` — updated to `DefaultValue::long(0)`
- [x] `src/types/properties/relationship/impls/default_relationship_property.rs` — already using `DefaultValue::double(1.0)`
- [x] All other usages verified as modern

## References

- Implementation: `src/types/default_value.rs` (lines 340-380)
- Test examples: `src/types/default_value.rs` (mod tests)
- Usage in schemas: `src/types/schema/property_schema.rs`
