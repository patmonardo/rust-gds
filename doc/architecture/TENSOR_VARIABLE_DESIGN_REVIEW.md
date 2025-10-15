# Tensor & Variable Design Review - Pre-Rewrite Analysis

**Date**: October 14, 2025  
**Purpose**: Ensure Tensor and Variable designs work together before rewriting

## Part 1: Java Design Analysis

### Java Tensor Hierarchy

```java
abstract class Tensor<SELF extends Tensor<SELF>> {
    protected final double[] data;
    protected final int[] dimensions;

    // Concrete shared methods
    public double dataAt(int idx) { return data[idx]; }
    public void setDataAt(int idx, double v) { data[idx] = v; }
    public double aggregateSum() { ... }
    public SELF map(DoubleUnaryOperator f) { ... }

    // Abstract methods (subclass implements)
    public abstract SELF createWithSameDimensions();
    public abstract SELF copy();
    public abstract SELF add(SELF b);
}

class Matrix extends Tensor<Matrix> { ... }
class Vector extends Tensor<Vector> { ... }
class Scalar extends Tensor<Scalar> { ... }
```

**Key Points:**

- ✅ Tensor is **abstract base class** with shared storage + methods
- ✅ Subclasses inherit data, dimensions, and all concrete methods
- ✅ Type parameter `SELF` ensures type-safe return values
- ✅ Each subclass only implements abstract methods + type-specific methods

### Java Variable Hierarchy

```java
interface Variable<T extends Tensor<T>> {
    T apply(ComputationContext ctx);
    Tensor<?> gradient(Variable<?> parent, ComputationContext ctx);
    boolean requireGradient();
    Iterable<? extends Variable<?>> parents();
    int[] dimensions();
}

abstract class AbstractVariable<T extends Tensor<T>> implements Variable<T> {
    private final int[] dimensions;
    private final boolean requireGradient;
    private final List<? extends Variable<?>> parents;

    // Concrete shared logic
    protected AbstractVariable(List<? extends Variable<?>> parents, int[] dimensions) {
        this.dimensions = dimensions;
        this.parents = parents;
        this.requireGradient = anyParentRequiresGradient();
    }

    private boolean anyParentRequiresGradient() { ... }
}

class Constant<T extends Tensor<T>> extends AbstractVariable<T> { ... }
class Weights<T extends Tensor<T>> extends AbstractVariable<T> { ... }
// etc.
```

**Key Points:**

- ✅ Variable is **interface** (contract only)
- ✅ AbstractVariable is **base class** with shared storage + methods
- ✅ Type parameter `T extends Tensor<T>` links Variable to Tensor type
- ✅ Functions extend AbstractVariable to inherit dimension/parent tracking
- ⚠️ Java uses **wildcards** (`Variable<?>`) for heterogeneous graphs

### Java's Type System Key Insight

```java
// In Java:
Variable<Matrix> matrixVar = new Weights<>(matrix);
Variable<Vector> vectorVar = new Weights<>(vector);

// But computation graph needs mixed types:
List<Variable<?>> parents = List.of(matrixVar, vectorVar);  // ✅ Wildcard

// Methods use wildcard to accept any tensor type:
Tensor<?> gradient(Variable<?> parent, ComputationContext ctx);
```

**The wildcard `?` is CRITICAL** - it allows:

1. Heterogeneous variable graphs (Matrix + Vector + Scalar parents)
2. Runtime polymorphism
3. Type safety at declaration, flexibility at composition

## Part 2: Current Rust Translation Issues

### Issue 1: Tensor Trait vs Base Struct

**Current (WRONG):**

```rust
pub trait Tensor {
    fn data(&self) -> &[f64];           // ❌ Must reimplement
    fn aggregate_sum(&self) -> f64;     // ❌ Must reimplement
    // ... everything duplicated 3 times
}

struct Matrix { data: Vec<f64>, dimensions: Vec<usize> }  // ❌ Duplicate storage
struct Vector { data: Vec<f64>, dimensions: Vec<usize> }  // ❌ Duplicate storage
struct Scalar { data: Vec<f64>, dimensions: Vec<usize> }  // ❌ Duplicate storage
```

**Problem:** No code sharing, storage duplicated, maintenance nightmare.

### Issue 2: Variable Type Erasure

**Current (QUESTIONABLE):**

```rust
pub trait Variable {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;  // Type-erased
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor>;
    // ...
}
```

**Questions:**

1. Is `Box<dyn Tensor>` the right choice? (Heap allocation on every operation)
2. Does this match Java's runtime behavior?
3. How does this interact with concrete Tensor types?

### Issue 3: AbstractVariable Translation

**Current:**

```rust
pub struct AbstractVariable {
    dimensions: Vec<usize>,
    require_gradient: bool,
    parents: Vec<Box<dyn Variable>>,
}

impl Variable for AbstractVariable {
    // Implements Variable trait
}
```

**Questions:**

1. Should AbstractVariable be a struct or trait?
2. How do functions extend it?
3. Does composition or trait approach work better?

## Part 3: Design Decision Matrix

### Decision 1: Tensor Storage Strategy

**Option A: Composition (RECOMMENDED)**

```rust
struct TensorData {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

struct Matrix {
    tensor: TensorData,  // Wraps shared data
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn data(&self) -> &[f64] {
        self.tensor.data()  // Delegate
    }
}
```

✅ Code sharing  
✅ Single source of truth  
✅ Matches Java inheritance intent  
❌ Slight delegation boilerplate (fixable with macro)

**Option B: Enum (ALTERNATIVE)**

```rust
enum Tensor {
    Matrix { base: TensorData, rows: usize, cols: usize },
    Vector { base: TensorData },
    Scalar { base: TensorData },
}
```

✅ Single type, easy matching  
✅ Code sharing via base field  
❌ Can't add Matrix-specific methods easily  
❌ Loses type safety for functions

**Option C: Current Trait (WRONG)**
❌ No code sharing  
❌ Duplicate storage  
❌ Doesn't match Java design

**VERDICT: Option A (Composition) is the clear winner**

### Decision 2: Tensor Trait Purpose

**Question:** Do we even need a Tensor trait?

**Answer:** YES, but MINIMAL and ONLY for polymorphism

```rust
// Minimal trait for when we need Box<dyn Tensor>
pub trait Tensor: Debug + Display {
    fn data(&self) -> &[f64];
    fn dimensions(&self) -> &[usize];
    fn clone_box(&self) -> Box<dyn Tensor>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    // Default implementations for convenience
    fn aggregate_sum(&self) -> f64 {
        self.data().iter().sum()
    }

    fn total_size(&self) -> usize {
        self.dimensions().iter().product()
    }
}
```

**Usage:**

- Variable returns `Box<dyn Tensor>` for heterogeneous graphs
- Functions use concrete types (Matrix, Vector) when possible
- Only box when storing in ComputationContext

### Decision 3: Variable Type Strategy

**Question:** Should Variable be generic or type-erased?

**Java's approach:**

```java
Variable<T extends Tensor<T>>  // Generic at declaration
Variable<?>                     // Wildcard for composition
```

**Rust options:**

**Option A: Full Type Erasure (CURRENT)**

```rust
trait Variable {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;
}
```

✅ Simple  
✅ Heterogeneous graphs work  
❌ Heap allocation on every op  
❌ Loses type information

**Option B: Generic + Type Erasure**

```rust
trait Variable {
    type Output: Tensor;  // Associated type
    fn apply(&self, ctx: &ComputationContext) -> Self::Output;
}

// But for storage:
trait VariableErased {
    fn apply_erased(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;
}

impl<V: Variable> VariableErased for V {
    fn apply_erased(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        Box::new(self.apply(ctx))
    }
}
```

✅ Type-safe when concrete  
✅ Erases only when needed  
❌ Complex  
❌ Two trait hierarchy

**Option C: Keep Current Type Erasure**
✅ Simple  
✅ Already works  
✅ Matches Java's wildcard usage at runtime  
✅ Box cost is acceptable (not hot path)

**VERDICT: Option C (Keep type erasure) is pragmatic**

Java's `Variable<?>` IS effectively type-erased at runtime. Our `Box<dyn Tensor>` matches this.

### Decision 4: AbstractVariable Pattern

**Question:** How should functions inherit AbstractVariable behavior?

**Option A: Composition (RECOMMENDED)**

```rust
struct VariableBase {
    dimensions: Vec<usize>,
    require_gradient: bool,
    parents: Vec<Box<dyn Variable>>,
}

impl VariableBase {
    fn new(parents: Vec<Box<dyn Variable>>, dimensions: Vec<usize>) -> Self {
        let require_gradient = Self::any_parent_requires_gradient(&parents);
        Self { dimensions, require_gradient, parents }
    }

    fn any_parent_requires_gradient(parents: &[Box<dyn Variable>]) -> bool {
        parents.iter().any(|p| p.require_gradient())
    }
}

struct Constant {
    base: VariableBase,  // Composition
    data: Box<dyn Tensor>,
}

impl Variable for Constant {
    fn dimensions(&self) -> &[usize] {
        &self.base.dimensions  // Delegate
    }

    fn parents(&self) -> &[Box<dyn Variable>] {
        &self.base.parents  // Delegate
    }

    fn require_gradient(&self) -> bool {
        self.base.require_gradient  // Delegate
    }
}
```

✅ Code sharing  
✅ Matches Tensor pattern  
✅ Clear structure  
❌ Delegation boilerplate (fixable with macro)

**Option B: Keep AbstractVariable as Struct**

```rust
struct AbstractVariable {
    // Current implementation
}

// Functions can't "extend" it in Rust
```

❌ Can't inherit in Rust  
❌ Functions must duplicate logic

**VERDICT: Option A (Composition with VariableBase)**

## Part 4: Interaction Analysis

### How Tensor and Variable Work Together

**In Variable methods:**

```rust
trait Variable {
    // Returns boxed trait object (type-erased)
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor>;

    // Receives boxed trait object
    fn gradient(&self, parent: &dyn Variable, ctx: &ComputationContext) -> Box<dyn Tensor>;
}
```

**In ComputationContext:**

```rust
pub struct ComputationContext {
    // Stores type-erased tensors
    data: HashMap<*const dyn Any, Box<dyn Tensor>>,
    gradients: HashMap<*const dyn Any, Box<dyn Tensor>>,
}

impl ComputationContext {
    pub fn data(&self, variable: &dyn Variable) -> Option<Box<dyn Tensor>> {
        // Returns boxed trait object
    }
}
```

**In ML Functions:**

```rust
impl Variable for MatrixMultiply {
    fn apply(&self, ctx: &ComputationContext) -> Box<dyn Tensor> {
        // Get parent data (boxed)
        let a_tensor = ctx.data(self.a.as_ref()).unwrap();

        // Downcast to concrete type
        let a_matrix = a_tensor.as_any().downcast_ref::<Matrix>().unwrap();

        // Use concrete Matrix methods
        let result: Matrix = a_matrix.multiply(&b_matrix);

        // Box and return
        Box::new(result)
    }
}
```

**Key Pattern:**

1. Variable methods use `Box<dyn Tensor>` (type-erased)
2. ComputationContext stores `Box<dyn Tensor>`
3. Functions downcast to concrete types when needed
4. Concrete types (Matrix/Vector) have efficient methods
5. Boxing happens only at boundaries

### Performance Considerations

**Where we box:**

- ✅ Variable.apply() return (once per forward pass)
- ✅ ComputationContext storage (once per variable)
- ✅ Variable.gradient() return (once per backward pass)

**Where we DON'T box:**

- ✅ Matrix.multiply() internals (concrete types)
- ✅ Tensor data operations (no allocation)
- ✅ Most function logic (concrete types)

**Verdict:** Boxing cost is acceptable - only happens at graph traversal boundaries.

## Part 5: Compatibility Checklist

### Will New Tensor Design Work with Variable?

| Requirement                               | Old Design | New Design | Status        |
| ----------------------------------------- | ---------- | ---------- | ------------- |
| Variable.apply() returns Box<dyn Tensor>  | ✅ Yes     | ✅ Yes     | ✅ Compatible |
| Tensor trait has clone_box()              | ✅ Yes     | ✅ Yes     | ✅ Compatible |
| Tensor trait has as_any()                 | ✅ Yes     | ✅ Yes     | ✅ Compatible |
| Can downcast Box<dyn Tensor> to Matrix    | ✅ Yes     | ✅ Yes     | ✅ Compatible |
| Matrix/Vector/Scalar impl Tensor trait    | ✅ Yes     | ✅ Yes     | ✅ Compatible |
| Functions can use concrete Matrix methods | ❌ No      | ✅ YES     | ✅ IMPROVED   |
| Tensor operations don't duplicate code    | ❌ NO      | ✅ YES     | ✅ IMPROVED   |

### Will VariableBase Work with Functions?

| Requirement                      | Old Design     | New Design          | Status        |
| -------------------------------- | -------------- | ------------------- | ------------- |
| Functions store dimensions       | ✅ Yes         | ✅ Yes (in base)    | ✅ Compatible |
| Functions track parents          | ✅ Yes         | ✅ Yes (in base)    | ✅ Compatible |
| Functions track require_gradient | ✅ Yes         | ✅ Yes (in base)    | ✅ Compatible |
| Code sharing for common logic    | ❌ NO          | ✅ YES              | ✅ IMPROVED   |
| Easy to add new functions        | ⚠️ Boilerplate | ✅ Less boilerplate | ✅ IMPROVED   |

## Part 6: Migration Strategy

### Phase 1: Create New Tensor System (2-3 hours)

1. Create `TensorData` struct with all shared methods
2. Rewrite `Matrix` to wrap `TensorData`
3. Rewrite `Vector` to wrap `TensorData`
4. Rewrite `Scalar` to wrap `TensorData`
5. Create delegation macro
6. Update minimal `Tensor` trait
7. Test basic operations

### Phase 2: Create VariableBase (1 hour)

1. Create `VariableBase` struct
2. Add common logic (dimension tracking, parent tracking)
3. Create delegation macro for Variable impls
4. Update `AbstractVariable` to use base
5. Test basic variable operations

### Phase 3: Update Functions (2-3 hours)

1. Update Constant to use VariableBase
2. Update Weights to use VariableBase
3. Fix remaining functions one-by-one
4. Fix lifetime issues (should be fewer now)
5. Fix clone_box issues (should be gone)
6. Test compilation

### Phase 4: Test & Validate (1 hour)

1. Run all existing tests
2. Add new Tensor tests
3. Add new Variable tests
4. Verify 62 errors reduced significantly

## Part 7: Risk Analysis

### Low Risk ✅

- TensorData creation (new code, doesn't break existing)
- VariableBase creation (new code, doesn't break existing)
- Macro creation (optional, can add later)

### Medium Risk ⚠️

- Matrix/Vector/Scalar rewrites (many usages in functions)
- Function updates (62 errors to fix)
- Lifetime issues (should improve but need testing)

### High Risk ❌

- None identified! Design is sound and compatible

### Mitigation Strategies

1. **Incremental approach** - rewrite Tensor first, test, then Variable
2. **Keep old code** - comment out instead of deleting initially
3. **Test frequently** - compile after each major change
4. **One file at a time** - don't change everything at once
5. **Git commits** - commit after each working phase

## Part 8: Final Verdict

### Should We Proceed? **YES! ✅**

**Reasons:**

1. ✅ Design is sound and matches Java's intent
2. ✅ Tensor and Variable systems are compatible
3. ✅ Will eliminate code duplication
4. ✅ Will reduce error count
5. ✅ Clear migration path
6. ✅ Low risk with incremental approach
7. ✅ No breaking changes to Variable interface

### Design Summary

**Tensor:**

- TensorData struct (shared storage + methods)
- Matrix/Vector/Scalar wrap TensorData (composition)
- Minimal Tensor trait (only for Box<dyn Tensor>)
- Delegation macro for boilerplate reduction

**Variable:**

- Keep type-erased trait (matches Java wildcards)
- VariableBase struct (shared dimension/parent tracking)
- Functions wrap VariableBase (composition)
- Delegation macro for boilerplate reduction

**Interaction:**

- Variable methods use Box<dyn Tensor>
- Functions downcast to concrete types when needed
- Boxing only at boundaries
- Efficient concrete operations

### Next Steps

1. **Review this document together** - make sure we agree
2. **Start Phase 1** - Create TensorData + rewrite Matrix
3. **Test incrementally** - verify each step
4. **Proceed to Phase 2** - Create VariableBase
5. **Fix functions** - apply new patterns
6. **Celebrate** - watch errors drop from 62 to ~10-20

**Estimated total time: 6-8 hours of focused work**

Ready to proceed? 🚀
