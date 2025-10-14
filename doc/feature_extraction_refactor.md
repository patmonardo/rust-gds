# Feature Extraction Module Refactoring

**Date**: October 14, 2025  
**Status**: ✅ Complete

## Summary

Refactored the feature extraction module to follow proper Rust idioms and composition patterns learned during the ML-Core tensor/variable refactoring.

## Key Changes

### 1. Removed Incomplete Downcasting Stubs

**Before** (Placeholder pattern):

```rust
fn downcast_to_scalar(_extractor: &dyn FeatureExtractor) -> Option<&dyn ScalarFeatureExtractor> {
    // This will be implemented properly once we have Any trait on FeatureExtractor
    // For now, this is a placeholder for the instanceof pattern
    None
}
```

**Problem**: Would always return `None`, causing panics at runtime.

### 2. Introduced Enum-Based Dispatch

**Java Pattern** (instanceof):

```java
if (extractor instanceof ScalarFeatureExtractor) {
    consumer.acceptScalar(..., ((ScalarFeatureExtractor) extractor).extract(nodeId));
} else if (extractor instanceof ArrayFeatureExtractor) {
    consumer.acceptArray(..., ((ArrayFeatureExtractor) extractor).extract(nodeId));
}
```

**Rust Pattern** (enum dispatch):

```rust
pub enum AnyFeatureExtractor {
    Scalar(Box<dyn ScalarFeatureExtractor>),
    Array(Box<dyn ArrayFeatureExtractor>),
}

impl AnyFeatureExtractor {
    pub fn extract_into(&self, ...) {
        match self {
            AnyFeatureExtractor::Scalar(extractor) => {
                consumer.accept_scalar(..., extractor.extract(node_id));
            }
            AnyFeatureExtractor::Array(extractor) => {
                consumer.accept_array(..., extractor.extract(node_id));
            }
        }
    }
}
```

### 3. Benefits of Enum Approach

**Type Safety**:

- ✅ Compile-time guarantee that all cases are handled
- ✅ No runtime downcasting failures
- ✅ Pattern matching exhaustiveness checking

**Performance**:

- ✅ Single indirect call (match on enum tag)
- ✅ No dynamic type checking (`Any` trait overhead)
- ✅ Better branch prediction

**Idiomatic Rust**:

- ✅ Follows Rust's "make illegal states unrepresentable" philosophy
- ✅ Aligns with sum types (enum) over inheritance
- ✅ Matches the composition pattern we established

## Architecture

### Trait Hierarchy

```
FeatureExtractor (marker trait)
    ├─ ScalarFeatureExtractor (dimension = 1)
    └─ ArrayFeatureExtractor (dimension = N)

AnyFeatureExtractor (enum wrapper)
    ├─ Scalar(Box<dyn ScalarFeatureExtractor>)
    └─ Array(Box<dyn ArrayFeatureExtractor>)
```

### Usage Pattern

```rust
// Create extractors
let extractors = vec![
    AnyFeatureExtractor::Scalar(Box::new(ScalarPropertyExtractor::new(...))),
    AnyFeatureExtractor::Array(Box::new(ArrayPropertyExtractor::new(...))),
];

// Extract features
extract(node_id, node_offset, &extractors, &mut consumer);
```

## Translation Philosophy

### Java → Rust Patterns Discovered

1. **Java's instanceof** → **Rust's enum dispatch**

   - More type-safe
   - Better performance
   - Exhaustiveness checking

2. **Java's interface hierarchy** → **Rust's trait + enum**

   - Traits define behavior
   - Enums provide closed set of implementations
   - Composition over inheritance

3. **Java's dynamic dispatch** → **Rust's static dispatch where possible**
   - Use enums for closed sets
   - Use `dyn Trait` only when truly open

## Test Coverage

**6 Feature Tests**:

- ✅ `test_feature_count` - Scalar-only extraction
- ✅ `test_feature_count_mixed` - Mixed scalar/array extraction
- ✅ `test_extract` - End-to-end extraction with consumer
- ✅ `test_bias_feature_dimension` - Bias feature always dimension 1
- ✅ `test_bias_feature_always_returns_one` - Bias value always 1.0
- ✅ `test_placeholder` - Placeholder test for property extractors

**Total ML-Core**: 151 tests passing

## Lessons for Future Translations

### ✅ Do This

1. **Use enums for closed type hierarchies**

   - When Java has a fixed set of subclasses
   - When you need instanceof-style dispatch

2. **Keep traits simple**

   - Marker traits for shared behavior
   - Specialized traits for specific functionality

3. **Avoid premature trait objects**
   - Start with concrete types
   - Add `dyn Trait` only when needed

### ❌ Avoid This

1. **Don't stub out downcasting**

   - `as_any()` / `Any` trait adds overhead
   - Enums are better for closed sets

2. **Don't fight the type system**

   - Java's flexibility → Rust's explicitness
   - Embrace sum types (enums)

3. **Don't translate patterns literally**
   - Think about the _intent_ in Java
   - Express it idiomatically in Rust

## Dialectical Interpretation

**Java's Approach (Being as Flexibility)**:

- Open inheritance hierarchies
- Dynamic type checking (instanceof)
- Runtime flexibility

**Rust's Approach (Being as Explicitness)**:

- Closed enum variants
- Compile-time type checking (match)
- Static guarantees

**Synthesis (TypeDef Wisdom)**:

- Use the right tool for the job
- Closed sets → enums
- Open extensibility → trait objects
- Performance-critical → static dispatch

## Future Work

When translating the remaining property extractors (when Graph API is available):

1. Wrap each concrete extractor in `AnyFeatureExtractor`
2. Return `Vec<AnyFeatureExtractor>` from factory functions
3. No downcasting or `Any` trait needed

## References

- Java source: `org.neo4j.gds.ml.core.features.FeatureExtraction`
- Rust module: `src/ml/core/features/`
- Related: Composition pattern from tensor/variable refactoring
