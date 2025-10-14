# ML Functions Tensor API Fix Plan

**Date**: October 14, 2025  
**Status**: 94 compilation errors to fix  
**Root Cause**: Functions translated expecting Matrix methods that don't exist in our Tensor API

## Problem Analysis

The 27 ML functions were translated 1:1 from Java GDS, which has a richer Matrix API with methods like:

- `multiply()` - matrix multiplication
- `multiply_trans_a()` - multiply with first operand transposed
- `multiply_trans_b()` - multiply with second operand transposed
- `sum_per_column()` - reduce matrix to vector by summing each column
- `sum_broadcast_column_wise()` - add vector to each row of matrix
- `set_row()` - copy row data from another matrix
- `as_any_mut()` - mutable downcasting for Tensor trait objects

Our Rust Tensor API (`src/ml/core/tensor/`) only has basic operations:

- ✅ `add()`, `add_inplace()`
- ✅ `scalar_multiply()`, `scalar_multiply_mutate()`
- ✅ `elementwise_product()`, `elementwise_product_mutate()`
- ✅ `map()`, `map_inplace()`
- ✅ `clone_box()`, `aggregate_sum()`
- ❌ **No matrix multiplication variants**
- ❌ **No column/row reduction operations**
- ❌ **No broadcasting operations**
- ❌ **No mutable trait downcasting**

## Error Categories

### Category 1: Missing Matrix Methods (70+ errors)

**Files affected**:

- `matrix_multiply_with_transposed_second_operand.rs` - needs `multiply()`, `multiply_trans_a()`, `multiply_trans_b()`
- `matrix_vector_sum.rs` - needs `sum_per_column()`, `sum_broadcast_column_wise()`
- `slice.rs` - needs `set_row()`

### Category 2: Trait Issues (15+ errors)

**Files affected**:

- `element_sum.rs` - `map()` closure vs function pointer
- `slice.rs` - needs `as_any_mut()` for mutable downcast
- Multiple files - `clone_box()` on `Box<dyn Variable>` (design issue)

### Category 3: API Mismatches (9 errors)

**Files affected**:

- `matrix_multiply_with_transposed_second_operand.rs` - `Matrix::size_in_bytes()` signature

## Fix Strategy

### Phase 1: Add Core Matrix Methods to Tensor API ⚡ PRIORITY

**Target**: `src/ml/core/tensor/matrix.rs`, `tensor.rs`

#### 1.1 Matrix Multiplication Methods

```rust
impl Matrix {
    /// Standard matrix multiplication: C = A × B
    pub fn multiply(&self, other: &Matrix) -> Box<dyn Tensor> { ... }

    /// Multiply with first operand transposed: C = A^T × B
    pub fn multiply_trans_a(&self, other: &Matrix) -> Box<dyn Tensor> { ... }

    /// Multiply with second operand transposed: C = A × B^T
    pub fn multiply_trans_b(&self, other: &Matrix) -> Box<dyn Tensor> { ... }
}
```

#### 1.2 Column/Row Reduction Methods

```rust
impl Matrix {
    /// Sum each column to create a vector [rows -> 1 per column]
    pub fn sum_per_column(&self) -> Box<dyn Tensor> { ... }

    /// Add vector to each row of matrix (broadcast)
    pub fn sum_broadcast_column_wise(&self, vector: &Vector) -> Box<dyn Tensor> { ... }

    /// Copy row from source matrix at source_row_idx to self at target_row
    pub fn set_row(&mut self, target_row: usize, source: &Matrix, source_row_idx: usize) { ... }
}
```

#### 1.3 Trait Mutable Downcasting

```rust
// Add to Tensor trait
pub trait Tensor: ... {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// Implement for Matrix, Vector, Scalar
impl Tensor for Matrix {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
```

### Phase 2: Fix Function-Level Issues

**Target**: Individual function files in `src/ml/core/functions/`

#### 2.1 Fix element_sum.rs map() closure

Change:

```rust
parent_data.map(|_| self_gradient)  // ❌ Closure
```

To:

```rust
parent_data.map({
    fn constant_fn(x: f64, value: f64) -> f64 { value }
    |x| self_gradient  // Still need workaround
})
```

Or better: add `map_constant()` helper to Tensor trait.

#### 2.2 Fix Variable clone_box() calls

The pattern `a.clone_box()` on `Box<dyn Variable>` is wrong. Need to:

- Store parents as `Vec<Box<dyn Variable>>` directly (already done)
- Don't try to clone Variables during construction
- Remove those problematic calls

#### 2.3 Fix Matrix::size_in_bytes() signature

Change from:

```rust
Matrix::size_in_bytes(rows, cols)  // ❌ Takes 2 args
```

To:

```rust
crate::ml::core::tensor::size_in_bytes(&[rows, cols])  // ✅ Uses existing function
```

### Phase 3: Add Comprehensive Tests

For each fixed function:

1. Test basic forward pass
2. Test gradient computation
3. Test edge cases (empty, single element)
4. Test dimension mismatches

## Implementation Order

### Round 1: Tensor API Extensions (1-2 hours)

1. ✅ Add `as_any_mut()` to Tensor trait + impls
2. ✅ Add `multiply()` to Matrix
3. ✅ Add `multiply_trans_a()` to Matrix
4. ✅ Add `multiply_trans_b()` to Matrix
5. ✅ Add `sum_per_column()` to Matrix
6. ✅ Add `sum_broadcast_column_wise()` to Matrix
7. ✅ Add `set_row()` to Matrix

### Round 2: Function Fixes (30-60 min)

1. ✅ Fix `element_sum.rs` map closure
2. ✅ Fix `matrix_multiply_with_transposed_second_operand.rs` Variable clones
3. ✅ Fix `matrix_vector_sum.rs` Variable clones
4. ✅ Fix `slice.rs` as_any_mut usage
5. ✅ Fix size_in_bytes() calls

### Round 3: Testing (1-2 hours)

1. ✅ Test all matrix multiplication variants
2. ✅ Test column reduction operations
3. ✅ Test broadcasting operations
4. ✅ Test all fixed functions end-to-end

## Success Criteria

- ✅ All 94 compilation errors resolved
- ✅ `cargo build --lib` succeeds
- ✅ All new Matrix methods have tests
- ✅ All fixed functions have tests
- ✅ Ready for ComputationContext implementation

## Next Steps After This Fix

1. **ComputationContext** - Implement tensor storage and gradient tracking
2. **BatchNeighbors** - Wrap UniformSampler for GNN aggregation
3. **Pipeline** - End-to-end ML pipeline execution

---

**Current Status**: Planning complete, ready to start Phase 1
