# Rust Trait Inheritance vs Java Class Inheritance

**Date**: October 14, 2025  
**Issue**: Understanding how to properly translate Java's Tensor inheritance to Rust

## The Key Difference

### Java Class Inheritance = CODE SHARING

```java
abstract class Tensor<SELF> {
    protected final double[] data;    // ✅ SHARED STORAGE
    protected final int[] dimensions; // ✅ SHARED STORAGE

    // ✅ SHARED IMPLEMENTATION - written once, inherited by all
    public double dataAt(int idx) {
        return data[idx];  // Uses parent's data field
    }

    public void addDataAt(int idx, double v) {
        data[idx] += v;  // Mutates parent's data field
    }

    public double aggregateSum() {
        double sum = 0;
        for (double d : data) sum += d;  // Iterates parent's data
        return sum;
    }
}

class Matrix extends Tensor<Matrix> {
    // ✅ INHERITS data, dimensions fields
    // ✅ INHERITS all method implementations
    // Only adds Matrix-specific behavior
}
```

### Rust Trait Inheritance = CONTRACT SHARING (NOT CODE)

```rust
// ❌ WRONG - Traits don't share implementations or storage
trait Tensor {
    fn data(&self) -> &[f64];     // Contract only - no implementation
    fn aggregate_sum(&self) -> f64; // Each type must implement
}

struct Matrix {
    data: Vec<f64>,  // ❌ Separate storage definition
}

impl Tensor for Matrix {
    fn data(&self) -> &[f64] { &self.data }

    fn aggregate_sum(&self) -> f64 {  // ❌ Must write implementation
        self.data.iter().sum()         // ❌ Duplicated for each type
    }
}

struct Vector {
    data: Vec<f64>,  // ❌ Duplicate storage definition
}

impl Tensor for Vector {
    fn data(&self) -> &[f64] { &self.data }

    fn aggregate_sum(&self) -> f64 {  // ❌ Copy-pasted from Matrix
        self.data.iter().sum()         // ❌ Duplicated AGAIN
    }
}
```

## What Rust Trait Inheritance Actually Does

```rust
// Trait inheritance = trait bounds, NOT code sharing
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

trait Tensor: AsAny {  // Tensor "extends" AsAny
    fn data(&self) -> &[f64];
}

// This means: "Any type that implements Tensor MUST ALSO implement AsAny"
// It does NOT mean: "Tensor inherits AsAny's implementation"
```

**Trait inheritance in Rust = "must also implement" requirement**  
**NOT = "inherits implementation"**

## The Correct Rust Solution: Composition + Delegation

To get Java's inheritance benefits in Rust, we use **composition**:

```rust
// 1. Shared data structure (replaces Java's base class fields)
#[derive(Clone, Debug)]
pub struct TensorData {
    data: Vec<f64>,
    dimensions: Vec<usize>,
}

impl TensorData {
    // 2. Shared implementations (replaces Java's concrete methods)
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [f64] {
        &mut self.data
    }

    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    pub fn data_at(&self, idx: usize) -> f64 {
        self.data[idx]
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

    pub fn total_size(&self) -> usize {
        self.dimensions.iter().product()
    }

    pub fn map(&self, f: fn(f64) -> f64) -> Vec<f64> {
        self.data.iter().map(|&x| f(x)).collect()
    }

    pub fn map_inplace(&mut self, f: fn(f64) -> f64) {
        for x in &mut self.data {
            *x = f(*x);
        }
    }

    // etc. - ALL shared behavior lives here
}

// 3. Concrete types wrap TensorData (replaces Java's extends)
#[derive(Clone, Debug)]
pub struct Matrix {
    tensor: TensorData,  // Composition instead of inheritance
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self {
            tensor: TensorData {
                data,
                dimensions: vec![rows, cols],
            },
            rows,
            cols,
        }
    }

    // Delegate to TensorData for shared behavior
    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.tensor.aggregate_sum()
    }

    pub fn dimensions(&self) -> &[usize] {
        self.tensor.dimensions()
    }

    // Matrix-specific methods
    pub fn data_at(&self, row: usize, col: usize) -> f64 {
        self.tensor.data_at(row * self.cols + col)
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        // Matrix-specific logic
    }
}

#[derive(Clone, Debug)]
pub struct Vector {
    tensor: TensorData,  // Same composition
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Self {
        let len = data.len();
        Self {
            tensor: TensorData {
                data,
                dimensions: vec![len],
            },
        }
    }

    // Delegate to TensorData - shares implementation!
    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.tensor.aggregate_sum()  // Same implementation!
    }

    pub fn length(&self) -> usize {
        self.tensor.dimensions()[0]
    }
}

#[derive(Clone, Debug)]
pub struct Scalar {
    tensor: TensorData,  // Same composition
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        Self {
            tensor: TensorData {
                data: vec![value],
                dimensions: vec![1],
            },
        }
    }

    pub fn value(&self) -> f64 {
        self.tensor.data()[0]
    }

    pub fn data(&self) -> &[f64] {
        self.tensor.data()
    }

    pub fn aggregate_sum(&self) -> f64 {
        self.tensor.aggregate_sum()  // Same implementation!
    }
}
```

## Reducing Boilerplate with Macros

The delegation methods can be generated with a macro:

```rust
macro_rules! impl_tensor_base_methods {
    ($type:ty) => {
        impl $type {
            pub fn data(&self) -> &[f64] {
                self.tensor.data()
            }

            pub fn data_mut(&mut self) -> &mut [f64] {
                self.tensor.data_mut()
            }

            pub fn dimensions(&self) -> &[usize] {
                self.tensor.dimensions()
            }

            pub fn aggregate_sum(&self) -> f64 {
                self.tensor.aggregate_sum()
            }

            pub fn total_size(&self) -> usize {
                self.tensor.total_size()
            }

            pub fn set_data_at(&mut self, idx: usize, value: f64) {
                self.tensor.set_data_at(idx, value);
            }

            pub fn add_data_at(&mut self, idx: usize, value: f64) {
                self.tensor.add_data_at(idx, value);
            }

            pub fn map_inplace(&mut self, f: fn(f64) -> f64) {
                self.tensor.map_inplace(f);
            }
        }
    };
}

// Apply to all types
impl_tensor_base_methods!(Matrix);
impl_tensor_base_methods!(Vector);
impl_tensor_base_methods!(Scalar);
```

## When Do We Need the Trait?

We only need a `Tensor` trait for **polymorphism** - when we need to store different types together:

```rust
// Minimal trait for polymorphism (like Variable's use case)
pub trait Tensor: Debug + Display {
    fn data(&self) -> &[f64];
    fn dimensions(&self) -> &[usize];
    fn clone_box(&self) -> Box<dyn Tensor>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    // Provide default implementations using data()
    fn aggregate_sum(&self) -> f64 {
        self.data().iter().sum()
    }

    fn total_size(&self) -> usize {
        self.dimensions().iter().product()
    }
}

impl Tensor for Matrix {
    fn data(&self) -> &[f64] { self.tensor.data() }
    fn dimensions(&self) -> &[usize] { self.tensor.dimensions() }
    fn clone_box(&self) -> Box<dyn Tensor> { Box::new(self.clone()) }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    // aggregate_sum, total_size use default implementations
}
```

## Direct Method Access vs Trait Methods

```rust
// When you have concrete type - use direct methods (no boxing)
let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
let sum = m.aggregate_sum();  // Direct call to TensorData method

// When you have trait object - use trait methods (dynamic dispatch)
let t: Box<dyn Tensor> = Box::new(m);
let sum = t.aggregate_sum();  // Trait method (dynamic dispatch)
```

## Comparison Table

| Feature                 | Java extends   | Rust trait bounds | Rust composition        |
| ----------------------- | -------------- | ----------------- | ----------------------- |
| Share storage           | ✅ Yes         | ❌ No             | ✅ Yes (via field)      |
| Share implementation    | ✅ Yes         | ❌ No             | ✅ Yes (via delegation) |
| Code written once       | ✅ Yes         | ❌ No             | ✅ Yes                  |
| Type-specific methods   | ✅ Yes         | ✅ Yes            | ✅ Yes                  |
| Runtime polymorphism    | ✅ Yes         | ✅ Yes            | ✅ Yes (with trait)     |
| Zero-cost when concrete | ❌ No (vtable) | ✅ Yes            | ✅ Yes                  |

## The Rewrite Plan

1. **Create `TensorData` struct** with all shared data and methods
2. **Rewrite Matrix, Vector, Scalar** to wrap `TensorData`
3. **Use macro** to generate delegation methods
4. **Keep minimal Tensor trait** for Variable use case (Box<dyn Tensor>)
5. **Update all functions** to use new structure (most should just work)

## Key Insight

> **Java's `extends` = Rust's composition + delegation**
>
> To translate Java inheritance to Rust:
>
> 1. Base class fields → Shared struct
> 2. Base class methods → Methods on shared struct
> 3. Subclass → Struct wrapping shared struct
> 4. Method inheritance → Delegation methods
> 5. Polymorphism → Optional trait for `Box<dyn>`

This is the idiomatic Rust way to achieve the same goals as Java inheritance!
