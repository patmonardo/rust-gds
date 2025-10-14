# Tensor System Redesign Analysis

**Date**: October 14, 2025  
**Issue**: Current Rust translation doesn't preserve Java's inheritance structure

## Java Design (Correct)

```java
// Abstract base class with concrete implementations
abstract class Tensor<SELF extends Tensor<SELF>> {
    protected final double[] data;
    protected final int[] dimensions;

    // CONCRETE methods (inherited by all):
    public double dataAt(int idx) { return data[idx]; }
    public void setDataAt(int idx, double v) { data[idx] = v; }
    public void addDataAt(int idx, double v) { data[idx] += v; }
    public SELF map(DoubleUnaryOperator f) { ... }
    public void addInPlace(Tensor<?> other) { ... }
    public double aggregateSum() { ... }
    // etc.

    // ABSTRACT methods (must implement):
    public abstract SELF createWithSameDimensions();
    public abstract SELF copy();
    public abstract SELF add(SELF b);
    protected abstract String shortDescription();
}

// Concrete implementations
class Matrix extends Tensor<Matrix> {
    private final int rows;
    private final int columns;

    // Inherits: dataAt(idx), setDataAt(idx, v), map(), aggregateSum(), etc.

    // Adds Matrix-specific methods:
    public double dataAt(int row, int col) { return dataAt(row * columns + col); }
    public void setDataAt(int row, int col, double v) { ... }
    public Matrix multiply(Matrix other) { ... }
    // etc.

    // Implements abstract methods:
    public Matrix createWithSameDimensions() { return new Matrix(rows, cols); }
    public Matrix copy() { return new Matrix(data.clone(), rows, cols); }
    public Matrix add(Matrix b) { ... }
    protected String shortDescription() { return "Matrix(" + rows + ", " + cols + ")"; }
}

class Vector extends Tensor<Vector> {
    // Inherits ALL base Tensor methods automatically
    // Only implements abstract methods
}

class Scalar extends Tensor<Scalar> {
    // Inherits ALL base Tensor methods automatically
    // Only implements abstract methods
}
```

## Current Rust Design (WRONG)

```rust
// Trait (interface-like, no implementation storage)
pub trait Tensor {
    fn data(&self) -> &[f64];           // ❌ Must be reimplemented by each type
    fn set_data_at(&mut self, idx: usize, value: f64);  // ❌ Reimplemented
    fn map(&self, f: fn(f64) -> f64) -> Box<dyn Tensor>;  // ❌ Reimplemented
    fn aggregate_sum(&self) -> f64;     // ❌ Reimplemented
    // etc. - everything must be reimplemented!
}

// Each type reimplements EVERYTHING independently
pub struct Matrix {
    data: Vec<f64>,  // ❌ Duplicate storage definition
    rows: usize,
    cols: usize,
    dimensions: Vec<usize>,
}

impl Tensor for Matrix {
    fn data(&self) -> &[f64] { &self.data }  // ❌ Had to write this
    fn set_data_at(&mut self, idx: usize, value: f64) { self.data[idx] = value; }  // ❌ Had to write this
    fn aggregate_sum(&self) -> f64 { self.data.iter().sum() }  // ❌ Had to write this
    // etc. - copied and pasted for every type!
}

pub struct Vector {
    data: Vec<f64>,  // ❌ Duplicate storage definition AGAIN
    dimensions: Vec<usize>,
}

impl Tensor for Vector {
    fn data(&self) -> &[f64] { &self.data }  // ❌ Copy-pasted from Matrix
    fn set_data_at(&mut self, idx: usize, value: f64) { self.data[idx] = value; }  // ❌ Copy-pasted
    fn aggregate_sum(&self) -> f64 { self.data.iter().sum() }  // ❌ Copy-pasted
    // etc. - everything duplicated!
}
```

**Problems:**

1. No code reuse - every method copied 3 times (Matrix, Vector, Scalar)
2. No shared storage - `data` and `dimensions` duplicated in each struct
3. Maintenance nightmare - bug fix must be applied 3 times
4. Lost Java's elegant inheritance structure

## Correct Rust Design (Using Enum + Delegation)

Rust doesn't have classical inheritance, but we can achieve the same structure using:

1. **Shared base struct** for common data
2. **Enum for type discrimination**
3. **Methods delegate to base for shared behavior**

```rust
// Base structure with shared data and methods (like Java's Tensor base class)
pub struct TensorBase {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl TensorBase {
    // Concrete methods (like Java's concrete methods)
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    pub fn set_data_at(&mut self, idx: usize, value: f64) {
        self.data[idx] = value;
    }

    pub fn add_data_at(&mut self, idx: usize, value: f64) {
        self.data[idx] += value;
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.data.iter().sum()
    }

    pub fn map(&self, f: fn(f64) -> f64) -> Vec<f64> {
        self.data.iter().map(|&x| f(x)).collect()
    }

    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    // etc. - all shared logic lives here ONCE
}

// Matrix wraps base and adds specific behavior
pub struct Matrix {
    base: TensorBase,  // ✅ Composition instead of duplication
    rows: usize,
    cols: usize,
}

impl Matrix {
    // Delegate to base for inherited behavior
    pub fn data(&self) -> &[f64] {
        self.base.data()
    }

    pub fn set_data_at(&mut self, idx: usize, value: f64) {
        self.base.set_data_at(idx, value);
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.base.aggregate_sum()
    }

    // Matrix-specific methods
    pub fn data_at(&self, row: usize, col: usize) -> f64 {
        self.base.data()[row * self.cols + col]
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        // Matrix-specific logic
    }
}

// Vector wraps base (inherits behavior)
pub struct Vector {
    base: TensorBase,  // ✅ Same base, different wrapper
}

impl Vector {
    // Delegate to base - inherits ALL behavior automatically
    pub fn data(&self) -> &[f64] {
        self.base.data()
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.base.aggregate_sum()
    }

    // Vector doesn't need to add data_at() - it's already in base!
}

// Scalar wraps base (inherits behavior)
pub struct Scalar {
    base: TensorBase,  // ✅ Same base, different wrapper
}

impl Scalar {
    pub fn value(&self) -> f64 {
        self.base.data()[0]
    }
}
```

## Alternative: Enum-Based Design

```rust
// Single enum that IS a Tensor
pub enum Tensor {
    Matrix { base: TensorBase, rows: usize, cols: usize },
    Vector { base: TensorBase },
    Scalar { base: TensorBase },
}

impl Tensor {
    // Methods work on any variant
    pub fn data(&self) -> &[f64] {
        match self {
            Tensor::Matrix { base, .. } => base.data(),
            Tensor::Vector { base } => base.data(),
            Tensor::Scalar { base } => base.data(),
        }
    }

    pub fn aggregate_sum(&self) -> f64 {
        match self {
            Tensor::Matrix { base, .. } => base.aggregate_sum(),
            Tensor::Vector { base } => base.aggregate_sum(),
            Tensor::Scalar { base } => base.aggregate_sum(),
        }
    }

    // Matrix-specific methods only available when Matrix variant
    pub fn multiply(&self, other: &Tensor) -> Result<Tensor, Error> {
        match (self, other) {
            (Tensor::Matrix { base: base1, rows, cols }, Tensor::Matrix { base: base2, .. }) => {
                // Matrix multiplication
            }
            _ => Err(Error::TypeMismatch),
        }
    }
}
```

## Recommendation: Hybrid Approach

**Best design for Rust GDS:**

1. **TensorData** struct - holds data and dimensions (replaces Java's Tensor base class data)
2. **Concrete types** (Matrix, Vector, Scalar) wrap TensorData
3. **Minimal trait** for polymorphism where needed
4. **Macro for delegation** to reduce boilerplate

```rust
// Shared data structure
pub struct TensorData {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl TensorData {
    // All shared behavior lives here
}

// Concrete types
pub struct Matrix {
    tensor: TensorData,
    rows: usize,
    cols: usize,
}

// Macro to generate delegation methods
macro_rules! impl_tensor_methods {
    ($type:ty) => {
        impl $type {
            pub fn data(&self) -> &[f64] {
                self.tensor.data()
            }

            pub fn aggregate_sum(&self) -> f64 {
                self.tensor.aggregate_sum()
            }
            // etc.
        }
    };
}

impl_tensor_methods!(Matrix);
impl_tensor_methods!(Vector);
impl_tensor_methods!(Scalar);
```

## Action Plan

1. **Create TensorData/TensorBase** with all shared methods
2. **Refactor Matrix** to wrap TensorBase
3. **Refactor Vector** to wrap TensorBase
4. **Refactor Scalar** to wrap TensorBase
5. **Remove duplicated code** from each type
6. **Update all ML functions** to use new structure
7. **Test everything**

## Benefits of Correct Design

✅ **Single source of truth** - data storage defined once  
✅ **Code reuse** - aggregate_sum() implemented once, used by all  
✅ **Easier maintenance** - fix bug in one place  
✅ **Preserves Java design** - clear mapping to original code  
✅ **Type safety** - Matrix-specific methods only on Matrix  
✅ **Performance** - no trait objects unless needed

## Current Session Impact

The 62 remaining errors are likely because:

- We're calling methods that should be inherited but aren't
- Type mismatches because we lost the base type relationship
- Missing methods that should come from base

**With proper design, many of these errors will disappear automatically.**
