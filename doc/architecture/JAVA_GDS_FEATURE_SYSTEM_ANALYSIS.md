# Java GDS Feature System Analysis

**Purpose**: Understand Java GDS feature architecture to guide Rust implementation.  
**Date**: October 2025  
**Context**: FormDB ML Platform - Feature Assembly (Task 6)

---

## Java GDS Feature Architecture

### Core Interfaces

#### 1. `FeatureStep` Interface

```java
public interface FeatureStep extends ToMapConvertible {
    List<String> inputNodeProperties();  // Source properties
    String name();                         // Feature name
    Map<String, Object> configuration();   // Transform config
}
```

**Purpose**: Declarative feature specification (what to compute).

#### 2. `FeatureStepUtil` Class

```java
public final class FeatureStepUtil {
    // Get property dimension (1 for scalar, N for array)
    public static int propertyDimension(Graph graph, String nodeProperty);

    // Validate computed features (check for NaN)
    public static void validateComputedFeatures(
        double[] linkFeatures,
        int startOffset,
        int endOffset,
        Runnable throwError
    );
}
```

**Purpose**: Utility functions for feature validation and dimensionality.

### Key Patterns

#### Pattern 1: Property → Feature Transformation

**Java Approach**:

1. **Property Dimension**: Use `propertyDimension()` to determine if property is scalar (1) or array (N)
2. **Feature Extraction**: Read property values and transform them
3. **Validation**: Check for NaN values (invalid features)
4. **Result**: double[] array ready for ML consumption

**Rust Translation**:

```rust
// Property dimension detection
fn property_dimension(property: &Arc<dyn PropertyValues>) -> usize {
    match property.value_type() {
        ValueType::Long | ValueType::Double => 1,  // Scalar
        ValueType::LongArray | ValueType::DoubleArray | ValueType::FloatArray => {
            // Array - get dimension from first element
            property.dimension()
        }
    }
}

// Feature validation
fn validate_features(features: &[f64]) -> Result<(), ComputeError> {
    if features.iter().any(|f| f.is_nan()) {
        Err(ComputeError::InvalidFeature("NaN values detected"))
    } else {
        Ok(())
    }
}
```

#### Pattern 2: Feature Step Types

**From Java GDS codebase (implied from usage)**:

1. **Identity**: Copy property as-is

   ```java
   // Just read property, no transformation
   double[] features = nodeProperties.doubleArrayValue(nodeId);
   ```

2. **Normalize**: Scale to [0,1] or z-score

   ```java
   // Min-max normalization
   normalized = (value - min) / (max - min);

   // Z-score normalization
   normalized = (value - mean) / stddev;
   ```

3. **Combine**: Concatenate multiple properties

   ```java
   // Concatenate property arrays
   double[] combined = new double[totalDimension];
   int offset = 0;
   for (String prop : sourceProperties) {
       double[] values = graph.nodeProperties(prop).doubleArrayValue(nodeId);
       System.arraycopy(values, 0, combined, offset, values.length);
       offset += values.length;
   }
   ```

4. **Project**: Dimensionality reduction
   ```java
   // PCA-style projection to lower dimension
   double[] projected = projectToLowerDimension(values, targetDim);
   ```

#### Pattern 3: Feature Assembly Pipeline

**Java GDS Flow**:

```
Graph + FeatureSteps
    ↓
For each node:
    ↓
For each FeatureStep:
    ↓
1. Read source properties
    ↓
2. Apply transformation
    ↓
3. Validate (check NaN)
    ↓
4. Concatenate to feature vector
    ↓
Result: double[] feature vector per node
```

**Rust Translation**:

```
PipelineState + FeatureStepDescriptors
    ↓
For each FeatureStep:
    ↓
1. Lookup source properties in state.properties
    ↓
2. Apply Transformation trait
    ↓
3. Validate result
    ↓
4. Store in state.features
    ↓
Result: HashMap<String, Arc<dyn PropertyValues>>
```

---

## Rust Feature System Design

### Module Structure

```
src/projection/native/ml/features/
├── mod.rs                    # Module exports
├── assembler.rs              # FeatureAssembler trait
├── transformation.rs         # Transformation trait + implementations
├── identity.rs               # IdentityTransformation
├── normalize.rs              # NormalizeTransformation
├── combine.rs                # CombineTransformation
└── validation.rs             # Feature validation utilities
```

### Core Traits

#### 1. `FeatureAssembler` Trait

```rust
/// Assembles features from properties for a single step.
pub trait FeatureAssembler: Send + Sync {
    /// Assemble feature from source properties.
    fn assemble(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
        step: &FeatureStepDescriptor,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError>;

    /// Get expected feature dimension.
    fn feature_dimension(&self, step: &FeatureStepDescriptor) -> usize;
}
```

#### 2. `Transformation` Trait

```rust
/// Transforms property values into feature values.
pub trait Transformation: Send + Sync {
    /// Transform property values to feature values.
    fn transform(
        &self,
        values: &Arc<dyn PropertyValues>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError>;

    /// Get transformation name (for debugging).
    fn name(&self) -> &str;
}
```

### Concrete Implementations

#### 1. **IdentityTransformation** (Phase 2.3 - Current)

```rust
pub struct IdentityTransformation;

impl Transformation for IdentityTransformation {
    fn transform(&self, values: &Arc<dyn PropertyValues>) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        Ok(values.clone())  // Just clone Arc pointer
    }

    fn name(&self) -> &str { "identity" }
}
```

#### 2. **NormalizeTransformation** (Phase 2.5)

```rust
pub struct NormalizeTransformation {
    strategy: NormalizationStrategy,
}

pub enum NormalizationStrategy {
    MinMax { min: f64, max: f64 },
    ZScore { mean: f64, stddev: f64 },
}

impl Transformation for NormalizeTransformation {
    fn transform(&self, values: &Arc<dyn PropertyValues>) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Compute normalization parameters if not provided
        let (min, max, mean, stddev) = compute_statistics(values)?;

        // Apply normalization
        let normalized = match self.strategy {
            NormalizationStrategy::MinMax { min, max } => {
                normalize_minmax(values, min, max)?
            }
            NormalizationStrategy::ZScore { mean, stddev } => {
                normalize_zscore(values, mean, stddev)?
            }
        };

        Ok(Arc::new(normalized))
    }
}
```

#### 3. **CombineTransformation** (Phase 2.5)

```rust
pub struct CombineTransformation {
    source_properties: Vec<String>,
}

impl CombineTransformation {
    /// Combine multiple properties into single feature vector.
    pub fn combine(
        &self,
        properties: &HashMap<String, Arc<dyn PropertyValues>>,
    ) -> Result<Arc<dyn PropertyValues>, ComputeError> {
        // Get all source properties
        let sources: Vec<_> = self.source_properties
            .iter()
            .map(|name| properties.get(name).ok_or_else(||
                ComputeError::MissingProperty(name.clone())
            ))
            .collect::<Result<_, _>>()?;

        // Compute total dimension
        let total_dim: usize = sources.iter()
            .map(|p| property_dimension(p))
            .sum();

        // Concatenate into single array
        let combined = concatenate_properties(&sources, total_dim)?;

        Ok(Arc::new(combined))
    }
}
```

### Feature Validation

```rust
pub mod validation {
    /// Validate feature values (check for NaN, Inf).
    pub fn validate_features(features: &Arc<dyn PropertyValues>) -> Result<(), ComputeError> {
        // Check each node's feature
        for node_id in 0..features.node_count() {
            match features.value_type() {
                ValueType::Double => {
                    let value = features.double_value(node_id);
                    if value.is_nan() || value.is_infinite() {
                        return Err(ComputeError::InvalidFeature {
                            node: node_id,
                            reason: "NaN or Inf value".into(),
                        });
                    }
                }
                ValueType::DoubleArray => {
                    let values = features.double_array_value(node_id);
                    if values.iter().any(|v| v.is_nan() || v.is_infinite()) {
                        return Err(ComputeError::InvalidFeature {
                            node: node_id,
                            reason: "NaN or Inf in array".into(),
                        });
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
```

---

## Integration with Pipeline Executor

### Current Implementation (Phase 2.3)

**File**: `src/projection/native/ml/pipeline_executor.rs`

```rust
fn assemble_features(&mut self) -> Result<(), ComputeError> {
    self.state.set_phase(ExecutionPhase::FeatureSteps);

    let feature_steps: Vec<_> = self.pipeline.steps.iter()
        .filter_map(|step| match step {
            StepDescriptor::Feature(f_step) => Some(f_step),
            _ => None,
        })
        .collect();

    for step in feature_steps {
        // Phase 2.3: Simple identity transformation
        if let Some(source_property) = step.source_properties.first() {
            if let Some(property_values) = self.state.get_property(source_property) {
                self.state.add_feature(step.name.clone(), property_values.clone());
                self.state.increment_step();
            } else {
                return Err(ComputeError::InitFailed(format!(
                    "source property '{}' not found for feature '{}'",
                    source_property, step.name
                )));
            }
        }
    }

    Ok(())
}
```

### Enhanced Implementation (Phase 2.5)

```rust
fn assemble_features(&mut self) -> Result<(), ComputeError> {
    use crate::projection::eval::ml::features::{
        FeatureAssembler, DefaultFeatureAssembler,
    };

    self.state.set_phase(ExecutionPhase::FeatureSteps);

    let assembler = DefaultFeatureAssembler::new();

    let feature_steps: Vec<_> = self.pipeline.steps.iter()
        .filter_map(|step| match step {
            StepDescriptor::Feature(f_step) => Some(f_step),
            _ => None,
        })
        .collect();

    for step in feature_steps {
        // Use assembler to apply transformations
        let feature_values = assembler.assemble(&self.state.properties, step)?;

        // Validate features
        validation::validate_features(&feature_values)?;

        // Store in state
        self.state.add_feature(step.name.clone(), feature_values);
        self.state.increment_step();
    }

    Ok(())
}
```

---

## Implementation Plan (Task 6)

### Phase 1: Core Infrastructure (Day 5 AM)

1. **Create module structure**

   ```bash
   mkdir -p src/projection/native/ml/features
   touch src/projection/native/ml/features/{mod.rs,assembler.rs,transformation.rs,validation.rs}
   ```

2. **Define traits** (assembler.rs, transformation.rs)

   - `FeatureAssembler` trait
   - `Transformation` trait
   - Error types for feature assembly

3. **Validation utilities** (validation.rs)
   - `validate_features()` - Check for NaN/Inf
   - `property_dimension()` - Get feature dimension
   - Unit tests

### Phase 2: Basic Transformations (Day 5 PM)

4. **IdentityTransformation** (identity.rs)

   - Simple Arc clone (current behavior)
   - Tests: scalar, array, embedding

5. **NormalizeTransformation** (normalize.rs)

   - Min-max normalization
   - Z-score normalization
   - Statistics computation
   - Tests: normalize scalars, normalize arrays

6. **DefaultFeatureAssembler** (assembler.rs)
   - Implements FeatureAssembler trait
   - Routes to appropriate transformation
   - Tests: assemble single property, validate features

### Phase 3: Advanced Transformations (Day 6 AM)

7. **CombineTransformation** (combine.rs)

   - Concatenate multiple properties
   - Handle different dimensions
   - Tests: combine scalars, combine arrays, mixed types

8. **Update FeatureStepDescriptor** (step_descriptor.rs)
   - Add transformation config field
   - Serialization support
   - Builder methods

### Phase 4: Integration & Testing (Day 6 PM)

9. **Update pipeline_executor.rs**

   - Replace identity logic with assembler
   - Add transformation selection
   - Error handling

10. **Integration tests**

    - Test normalize → features
    - Test combine → features
    - Test end-to-end with transformations

11. **Documentation**
    - Update ML_PIPELINE_ARCHITECTURE.md
    - Feature assembly examples
    - Transformation guide

---

## Key Design Decisions

### 1. Transformation as Trait

**Rationale**: Need polymorphic transformation dispatch (runtime selection based on config).

**Benefits**:

- Extensible (add new transformations without modifying executor)
- Testable (mock transformations)
- Composable (chain transformations)

### 2. PropertyValues as Feature Container

**Rationale**: Features are just special properties (ML-ready values).

**Benefits**:

- Reuse existing PropertyValues infrastructure
- No new value types needed
- Cursor support for large datasets

### 3. Validation at Assembly Time

**Rationale**: Catch invalid features early (before training).

**Java GDS Pattern**: `validateComputedFeatures()` called after each transformation.

**Benefits**:

- Clear error messages (which node, which feature)
- Fail-fast (don't waste time training on invalid data)

### 4. HashMap<String, PropertyValues> for State

**Rationale**: Features indexed by name (like properties).

**Benefits**:

- Simple lookup (get_feature("name"))
- Multiple features per pipeline
- Easy serialization

---

## Java GDS References

### Key Classes

1. **FeatureStep.java**

   - Interface: `inputNodeProperties()`, `name()`, `configuration()`
   - Purpose: Declarative feature specification

2. **FeatureStepUtil.java**

   - Utility: `propertyDimension()`, `validateComputedFeatures()`
   - Purpose: Feature dimension detection + validation

3. **Pipeline.java**

   - Method: `featureSteps()`, `featureProperties()`, `validateFeatureProperties()`
   - Purpose: Pipeline-level feature management

4. **PipelineExecutor.java**
   - Enum: `DatasetSplits` (TRAIN, TEST, TEST_COMPLEMENT, FEATURE_INPUT)
   - Purpose: Dataset splitting for training

### Patterns to Preserve

1. **Feature Dimension Detection**: Use property type to determine dimension
2. **NaN Validation**: Check all feature values for NaN/Inf
3. **Property Missing Errors**: Clear error if source property not found
4. **Feature Concatenation**: Combine multiple properties into single vector

---

## Success Criteria

### Quantitative

- ✅ 3 transformation implementations (Identity, Normalize, Combine)
- ✅ 10+ unit tests (each transformation + assembler)
- ✅ 3 integration tests (normalize, combine, end-to-end)
- ✅ Zero compilation warnings
- ✅ All 50+ ML tests passing

### Qualitative

- ✅ Clean trait-based design (extensible)
- ✅ Java GDS patterns preserved (dimension detection, validation)
- ✅ Identity transformation replaced with assembler
- ✅ Ready for Phase 2.5 advanced transformations
- ✅ Documentation complete (examples, API guide)

---

## Next Steps (After Task 6)

**Task 7: Training Executor** (Days 7-8)

- Hyperparameter search (GridSearch/RandomSearch)
- Model selection (validation metric)
- Cross-validation

**Task 8: Model Trait System** (Day 7)

- Model trait (fit, predict, evaluate)
- ModelType enum
- ModelMetadata

**Task 9: Simple Model** (Day 8)

- Logistic regression or simple classifier
- Uses ndarray for computations
- Demonstrates Model trait

---

**Status**: Ready to implement Feature Assembly (Task 6)  
**Timeline**: 2 days (Day 5-6)  
**Dependencies**: Pipeline Executor (Task 5) ✅ Complete  
**Blocker**: None - all infrastructure in place
