# ML Core Features System Review

**Date**: October 13, 2025  
**Status**: ⚠️ **TRANSLATION MISMATCH - Needs Correction**

## Critical Issues Found

### Issue 1: Missing FeatureExtraction Utility Class

**Java Has**: `FeatureExtraction` - A utility class with static methods for:
- `extract(nodeId, nodeOffset, extractors, consumer)` - Extract single node features
- `extract(batch, extractors)` - Extract batch features → Constant<Matrix>
- `extract(graph, extractors, features)` - Extract all graph features → HugeObjectArray
- `featureCount(extractors)` - Calculate total feature dimensions
- `propertyExtractors(graph, properties)` - Create extractors from property names

**Rust Currently Has**: Trait-based design that doesn't match Java's utility pattern

### Issue 2: FeatureExtractor Interface Mismatch

**Java FeatureExtractor.java**:
```java
public interface FeatureExtractor {
    int dimension();
}
```
- Simple marker interface with only `dimension()` method
- No `extract()` method at this level
- Subtypes: `ScalarFeatureExtractor`, `ArrayFeatureExtractor`

**Current Rust Translation**:
```rust
pub trait FeatureExtractor {
    fn extract(&self, graph: &Graph, config: &FeatureExtractionConfig) 
        -> Result<FeatureExtractionResult, FeatureExtractionError>;
    fn dimension(&self) -> usize;
}
```
- Added methods not in Java interface
- Over-engineered for what should be a simple trait

### Issue 3: FeatureConsumer Missing NOOP

**Java FeatureConsumer.java**:
```java
public interface FeatureConsumer {
    void acceptScalar(long nodeOffset, int offset, double value);
    void acceptArray(long nodeOffset, int offset, double[] values);

    FeatureConsumer NOOP = new FeatureConsumer() {
        @Override public void acceptScalar(...) {}
        @Override public void acceptArray(...) {}
    };
}
```

**Current Rust Translation**:
```rust
pub trait FeatureConsumer {
    fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64);
    fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]);
}
```
- Missing NOOP implementation
- Should have a `NoopConsumer` struct

### Issue 4: Incorrect Separation of Concerns

**Java Design**:
1. `FeatureExtractor` - Simple marker interface (dimension only)
2. `ScalarFeatureExtractor` - Subinterface with `double extract(long)`
3. `ArrayFeatureExtractor` - Subinterface with `double[] extract(long)`
4. `FeatureExtraction` - Utility class with static orchestration methods
5. Implementations: `ScalarPropertyExtractor`, `ArrayPropertyExtractor`, etc.

**Current Rust Design**:
1. `FeatureExtractor` - Over-engineered trait with graph/config/result
2. `ScalarFeatureExtractor`, `ArrayFeatureExtractor` - Correct traits
3. Missing: `FeatureExtraction` utility module/functions
4. Implementations exist but attached to wrong abstractions

## Java Structure Analysis

### Core Abstractions (Correct in Rust)

1. **FeatureExtractor** (marker interface)
   ```java
   public interface FeatureExtractor {
       int dimension();
   }
   ```

2. **ScalarFeatureExtractor** (extends FeatureExtractor)
   ```java
   double extract(long nodeId);
   ```

3. **ArrayFeatureExtractor** (extends FeatureExtractor)
   ```java
   double[] extract(long nodeId);
   ```

### Utility Class (MISSING in Rust)

**FeatureExtraction** - Static utility methods:

```java
// Core extraction method - loops over extractors and feeds consumer
public static void extract(
    long nodeId,
    long nodeOffset,
    List<FeatureExtractor> extractors,
    FeatureConsumer consumer
) {
    int offset = 0;
    for (FeatureExtractor extractor : extractors) {
        if (extractor instanceof ScalarFeatureExtractor) {
            consumer.acceptScalar(nodeOffset, offset, 
                ((ScalarFeatureExtractor) extractor).extract(nodeId));
        } else if (extractor instanceof ArrayFeatureExtractor) {
            consumer.acceptArray(nodeOffset, offset, 
                ((ArrayFeatureExtractor) extractor).extract(nodeId));
        }
        offset += extractor.dimension();
    }
}

// Extract batch to Matrix
public static Constant<Matrix> extract(Batch batch, List<FeatureExtractor> extractors);

// Extract graph to HugeObjectArray
public static HugeObjectArray<double[]> extract(
    Graph graph, 
    List<FeatureExtractor> extractors, 
    HugeObjectArray<double[]> features
);

// Calculate feature count
public static int featureCount(Collection<FeatureExtractor> extractors);

// Create extractors from property names
public static List<FeatureExtractor> propertyExtractors(
    Graph graph, 
    Collection<String> featureProperties
);
```

### Concrete Implementations (Mostly Correct in Rust)

1. **ScalarPropertyExtractor** - Extracts scalar node properties
2. **ArrayPropertyExtractor** - Extracts double[] node properties  
3. **LongArrayPropertyExtractor** - Extracts long[] as double[]
4. **BiasFeature** - Always returns 1.0 (bias term)
5. **HugeObjectArrayFeatureConsumer** - Writes to HugeObjectArray

## Correct Rust Design

### What Should Exist

1. **Traits** (marker + specialized):
   ```rust
   pub trait FeatureExtractor {
       fn dimension(&self) -> usize;
   }
   
   pub trait ScalarFeatureExtractor: FeatureExtractor {
       fn extract(&self, node_id: u64) -> f64;
   }
   
   pub trait ArrayFeatureExtractor: FeatureExtractor {
       fn extract(&self, node_id: u64) -> Vec<f64>;
   }
   ```

2. **FeatureConsumer trait**:
   ```rust
   pub trait FeatureConsumer {
       fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64);
       fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]);
   }
   
   pub struct NoopConsumer;
   impl FeatureConsumer for NoopConsumer { /* empty impls */ }
   ```

3. **FeatureExtraction module** (utility functions):
   ```rust
   pub mod feature_extraction {
       /// Extract features for single node
       pub fn extract(
           node_id: u64,
           node_offset: u64,
           extractors: &[Box<dyn FeatureExtractor>],
           consumer: &mut dyn FeatureConsumer,
       );
       
       /// Extract batch to Matrix constant
       pub fn extract_batch(
           batch: &Batch,
           extractors: &[Box<dyn FeatureExtractor>],
       ) -> Constant<Matrix>;
       
       /// Extract graph to vector array
       pub fn extract_graph(
           graph: &Graph,
           extractors: &[Box<dyn FeatureExtractor>],
       ) -> Vec<Vec<f64>>;
       
       /// Calculate total feature dimensions
       pub fn feature_count(extractors: &[Box<dyn FeatureExtractor>]) -> usize;
       
       /// Create extractors from property names
       pub fn property_extractors(
           graph: &Graph,
           properties: &[String],
       ) -> Vec<Box<dyn FeatureExtractor>>;
   }
   ```

## Config System Integration

### FeatureExtractionConfig - NOT Part of Java ml-core

The `FeatureExtractionConfig` in Rust appears to be a **custom addition** not present in Java GDS ml-core.

**Java Approach**: 
- Configuration happens at algorithm level (e.g., NodeClassificationTrainConfig)
- Feature properties passed as `List<String>` directly
- No separate FeatureExtractionConfig class

**Recommendation**:
- Remove `FeatureExtractionConfig` from ml/core/features
- If needed for Rust API design, move to `config/` module
- Keep ml/core/features as pure translation

### FeatureExtractionError & Result - Also Custom

**Java Approach**:
- Methods throw runtime exceptions
- No FeatureExtractionError enum
- No FeatureExtractionResult wrapper

**Rust Idiomatic**:
- Having Result types is good Rust practice
- But not a 1:1 translation
- Should be clearly documented as Rust enhancement

## Action Plan

### Step 1: Fix FeatureExtractor Trait (10 min)
```rust
// Remove over-engineered methods
pub trait FeatureExtractor {
    fn dimension(&self) -> usize;
}

// Keep specialized traits
pub trait ScalarFeatureExtractor: FeatureExtractor {
    fn extract(&self, node_id: u64) -> f64;
}

pub trait ArrayFeatureExtractor: FeatureExtractor {
    fn extract(&self, node_id: u64) -> Vec<f64>;
}
```

### Step 2: Add NoopConsumer (5 min)
```rust
pub struct NoopConsumer;

impl FeatureConsumer for NoopConsumer {
    fn accept_scalar(&mut self, _: u64, _: usize, _: f64) {}
    fn accept_array(&mut self, _: u64, _: usize, _: &[f64]) {}
}
```

### Step 3: Create feature_extraction.rs Module (30 min)
Translate all static methods from FeatureExtraction.java

### Step 4: Review Config Integration (10 min)
- Document FeatureExtractionConfig as Rust enhancement
- Consider moving to config/ module
- Or remove entirely and match Java approach

### Step 5: Update mod.rs Exports (5 min)
```rust
pub mod feature_extraction;
pub use feature_extraction::*;
```

## Compatibility Matrix

| Java Component | Current Rust | Correct? | Action |
|----------------|--------------|----------|--------|
| FeatureExtractor (interface) | Trait with extra methods | ❌ | Simplify |
| ScalarFeatureExtractor | Correct trait | ✅ | Keep |
| ArrayFeatureExtractor | Correct trait | ✅ | Keep |
| FeatureConsumer | Missing NOOP | ⚠️ | Add NoopConsumer |
| FeatureExtraction (utility) | **MISSING** | ❌ | **Create module** |
| ScalarPropertyExtractor | Exists | ✅ | Keep |
| ArrayPropertyExtractor | Exists | ✅ | Keep |
| BiasFeature | Exists | ✅ | Keep |
| HugeObjectArrayFeatureConsumer | Exists | ✅ | Keep |
| FeatureExtractionConfig | Added (not in Java) | ⚠️ | Document/Move |
| FeatureExtractionError | Added (not in Java) | ⚠️ | Keep but document |
| FeatureExtractionResult | Added (not in Java) | ⚠️ | Keep but document |

## Key Insights

1. **Java uses utility class pattern** - Rust should use module with functions
2. **FeatureExtractor is a marker** - Just `dimension()`, not full extraction API
3. **Type dispatch via instanceof** - Rust uses trait objects + downcasting
4. **FeatureConsumer pattern is correct** - Just missing NOOP
5. **Config additions are Rust enhancements** - Not 1:1 translations

## Next Steps

**Priority 1**: Create `feature_extraction.rs` module with utility functions  
**Priority 2**: Simplify `FeatureExtractor` trait to match Java  
**Priority 3**: Add `NoopConsumer` implementation  
**Priority 4**: Decide on config integration approach  

---
**Translation Quality**: Currently 60% - needs utility module to reach 100%
