# Unified Collections Architecture: The Three-Tier System

**Date**: October 15, 2025  
**Status**: 🏗️ **Architecture Design** - Unified Collections System  
**Context**: Level 1 Upgrade - Collections Mastery

---

## 🎯 Executive Summary

Design a unified Collections architecture with three tiers:
1. **Collections Layer**: Vec, HugeArray, Arrow Collections (unified API)
2. **Adapter Layer**: Single PropertyValues adapter to Collections
3. **Application Layer**: Clean separation of concerns

---

## Part 1: The Three Collections Systems

### **Current State Analysis**

```rust
// Current: Three separate systems
pub struct LongPropertyValues {
    values: Vec<i64>,           // System 1: Vec-based
}

pub struct HugeLongPropertyValues {
    values: HugeLongArray,      // System 2: HugeArray-based
}

// System 3: Arrow-based (future)
pub struct ArrowLongPropertyValues {
    values: Int64Array,         // System 3: Arrow-based
}
```

### **Problem: No Unified API**

Each system has different methods:
- **Vec**: `vec.iter().sum()`
- **HugeArray**: `huge_array.sum()` (if we add it)
- **Arrow**: `sum(&arrow_array)?`

**Result**: PropertyValues must implement different logic for each backend.

---

## Part 2: Unified Collections API Design

### **The Collections Trait**

```rust
/// Unified Collections API - The "best of all worlds" approach
pub trait Collections<T> {
    // Core operations
    fn get(&self, index: usize) -> Option<T>;
    fn set(&mut self, index: usize, value: T);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    
    // Iteration
    fn iter(&self) -> Box<dyn Iterator<Item = T> + '_>;
    fn into_iter(self) -> Box<dyn Iterator<Item = T>>;
    
    // Aggregation (elevated from Vec)
    fn sum(&self) -> Option<T> where T: Sum;
    fn min(&self) -> Option<T> where T: Ord;
    fn max(&self) -> Option<T> where T: Ord;
    fn mean(&self) -> Option<f64> where T: Into<f64>;
    
    // Search & Sort
    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord;
    fn sort(&mut self) where T: Ord;
    
    // Conversion
    fn to_vec(self) -> Vec<T>;
    fn as_slice(&self) -> &[T];
    
    // Nullability (for Arrow compatibility)
    fn is_null(&self, index: usize) -> bool;
    fn null_count(&self) -> usize;
}
```

### **Implementation for Each System**

```rust
// System 1: Vec Collections (enhanced)
impl<T: Clone + Ord + Sum> Collections<T> for Vec<T> {
    fn get(&self, index: usize) -> Option<T> {
        self.get(index).cloned()
    }
    
    fn set(&mut self, index: usize, value: T) {
        if index < self.len() {
            self[index] = value;
        } else {
            self.resize(index + 1, Default::default());
            self[index] = value;
        }
    }
    
    fn sum(&self) -> Option<T> where T: Sum {
        Some(self.iter().cloned().sum())
    }
    
    fn mean(&self) -> Option<f64> where T: Into<f64> {
        if self.is_empty() { None } else {
            Some(self.iter().map(|&x| x.into()).sum::<f64>() / self.len() as f64)
        }
    }
    
    fn is_null(&self, _index: usize) -> bool { false } // Vec has no nulls
    fn null_count(&self) -> usize { 0 }
}

// System 2: HugeArray Collections
impl<T: Clone + Ord + Sum + Default> Collections<T> for HugeArray<T> {
    fn get(&self, index: usize) -> Option<T> {
        if index < self.size() { Some(self.get(index)) } else { None }
    }
    
    fn set(&mut self, index: usize, value: T) {
        if index < self.size() {
            self.set(index, value);
        }
    }
    
    fn sum(&self) -> Option<T> where T: Sum {
        Some(self.iter().sum())
    }
    
    fn mean(&self) -> Option<f64> where T: Into<f64> {
        if self.size() == 0 { None } else {
            Some(self.iter().map(|x| x.into()).sum::<f64>() / self.size() as f64)
        }
    }
    
    fn is_null(&self, _index: usize) -> bool { false } // HugeArray has no nulls
    fn null_count(&self) -> usize { 0 }
}

// System 3: Arrow Collections
impl Collections<i64> for Int64Array {
    fn get(&self, index: usize) -> Option<i64> {
        if index < self.len() && !self.is_null(index) {
            Some(self.value(index))
        } else {
            None
        }
    }
    
    fn set(&mut self, index: usize, value: i64) {
        // Arrow arrays are immutable, would need to create new array
        // This is a limitation we'd need to handle
    }
    
    fn sum(&self) -> Option<i64> {
        arrow::compute::sum(self).ok()
    }
    
    fn mean(&self) -> Option<f64> {
        arrow::compute::mean(self).ok()
    }
    
    fn is_null(&self, index: usize) -> bool {
        self.is_null(index)
    }
    
    fn null_count(&self) -> usize {
        self.null_count()
    }
}
```

---

## Part 3: Single PropertyValues Adapter

### **The Universal Adapter**

```rust
/// Universal PropertyValues adapter to Collections
pub struct UniversalPropertyValues<T, C>
where
    C: Collections<T>,
{
    collections: C,
    value_type: ValueType,
    default_value: T,
}

impl<T, C> UniversalPropertyValues<T, C>
where
    C: Collections<T>,
{
    pub fn new(collections: C, value_type: ValueType, default_value: T) -> Self {
        Self { collections, value_type, default_value }
    }
}

impl<T, C> PropertyValues for UniversalPropertyValues<T, C>
where
    T: Clone + Debug + Send + Sync + 'static,
    C: Collections<T> + Send + Sync,
{
    fn value_type(&self) -> ValueType {
        self.value_type
    }
    
    fn element_count(&self) -> usize {
        self.collections.len()
    }
}

impl<T, C> TypedPropertyValues for UniversalPropertyValues<T, C>
where
    T: Clone + Debug + Send + Sync + 'static,
    C: Collections<T> + Send + Sync,
{
    fn get<R: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<R> {
        if let Some(value) = self.collections.get(index) {
            let gds_value = self.to_gds_value(value);
            R::from_gds_value(gds_value.as_ref()).map_err(|e| PropertyValuesError::UnsupportedOperation(e))
        } else {
            Err(PropertyValuesError::InvalidNodeId(index as u64))
        }
    }
    
    fn get_slice<R: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<R>> {
        let mut result = Vec::new();
        for &index in indices {
            result.push(self.get::<R>(index)?);
        }
        Ok(result)
    }
    
    fn get_all<R: FromGdsValue>(&self) -> PropertyValuesResult<&[R]> {
        // This would need to be implemented based on the specific collection type
        // For now, we'll return an error indicating it needs collection-specific implementation
        Err(PropertyValuesError::UnsupportedOperation("get_all requires collection-specific implementation".to_string()))
    }
}
```

---

## Part 4: Factory Pattern for Collections

### **Collections Factory**

```rust
/// Factory for creating Collections-based PropertyValues
pub enum CollectionsBackend {
    Vec,        // Standard Vec
    HugeArray,  // Paged arrays
    Arrow,      // Arrow arrays
}

pub struct CollectionsFactory;

impl CollectionsFactory {
    pub fn create_long_property_values(
        values: Vec<i64>,
        backend: CollectionsBackend,
    ) -> Box<dyn PropertyValues> {
        match backend {
            CollectionsBackend::Vec => {
                let collections = values; // Vec<i64> implements Collections<i64>
                Box::new(UniversalPropertyValues::new(
                    collections,
                    ValueType::Long,
                    0i64,
                ))
            }
            CollectionsBackend::HugeArray => {
                let collections = HugeLongArray::from_vec(values);
                Box::new(UniversalPropertyValues::new(
                    collections,
                    ValueType::Long,
                    0i64,
                ))
            }
            CollectionsBackend::Arrow => {
                let collections = Int64Array::from(values);
                Box::new(UniversalPropertyValues::new(
                    collections,
                    ValueType::Long,
                    0i64,
                ))
            }
        }
    }
    
    // Similar methods for other types...
    pub fn create_double_property_values(
        values: Vec<f64>,
        backend: CollectionsBackend,
    ) -> Box<dyn PropertyValues> {
        // Implementation...
    }
}
```

---

## Part 5: Enhanced Vec Implementation

### **Elevating Vec to Collections Standards**

```rust
/// Enhanced Vec with Collections API
pub struct EnhancedVec<T> {
    data: Vec<T>,
    nulls: Option<Vec<bool>>,  // For nullability support
}

impl<T> EnhancedVec<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(), nulls: None }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            data: Vec::with_capacity(capacity), 
            nulls: None 
        }
    }
    
    pub fn with_nulls(capacity: usize) -> Self {
        Self { 
            data: Vec::with_capacity(capacity), 
            nulls: Some(Vec::with_capacity(capacity))
        }
    }
    
    // Enhanced methods
    pub fn sum(&self) -> Option<T> where T: Sum {
        if self.nulls.is_some() {
            // Handle nulls
            self.data.iter()
                .zip(self.nulls.as_ref().unwrap().iter())
                .filter(|(_, &is_null)| !is_null)
                .map(|(value, _)| value.clone())
                .sum::<T>()
                .into()
        } else {
            Some(self.data.iter().cloned().sum())
        }
    }
    
    pub fn mean(&self) -> Option<f64> where T: Into<f64> {
        if self.is_empty() { return None; }
        
        let sum: f64 = if self.nulls.is_some() {
            self.data.iter()
                .zip(self.nulls.as_ref().unwrap().iter())
                .filter(|(_, &is_null)| !is_null)
                .map(|(value, _)| value.clone().into())
                .sum()
        } else {
            self.data.iter().map(|x| x.clone().into()).sum()
        };
        
        let count = if self.nulls.is_some() {
            self.nulls.as_ref().unwrap().iter().filter(|&&is_null| !is_null).count()
        } else {
            self.data.len()
        };
        
        if count == 0 { None } else { Some(sum / count as f64) }
    }
}

impl<T> Collections<T> for EnhancedVec<T>
where
    T: Clone + Default + Ord + Sum,
{
    fn get(&self, index: usize) -> Option<T> {
        if index < self.data.len() {
            if let Some(ref nulls) = self.nulls {
                if nulls[index] { None } else { Some(self.data[index].clone()) }
            } else {
                Some(self.data[index].clone())
            }
        } else {
            None
        }
    }
    
    fn set(&mut self, index: usize, value: T) {
        if index >= self.data.len() {
            self.data.resize(index + 1, Default::default());
            if let Some(ref mut nulls) = self.nulls {
                nulls.resize(index + 1, false);
            }
        }
        self.data[index] = value;
        if let Some(ref mut nulls) = self.nulls {
            nulls[index] = false;
        }
    }
    
    fn len(&self) -> usize { self.data.len() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    
    fn sum(&self) -> Option<T> where T: Sum { self.sum() }
    fn mean(&self) -> Option<f64> where T: Into<f64> { self.mean() }
    
    fn is_null(&self, index: usize) -> bool {
        if let Some(ref nulls) = self.nulls {
            index < nulls.len() && nulls[index]
        } else {
            false
        }
    }
    
    fn null_count(&self) -> usize {
        if let Some(ref nulls) = self.nulls {
            nulls.iter().filter(|&&is_null| is_null).count()
        } else {
            0
        }
    }
}
```

---

## Part 6: Migration Strategy

### **Phase 1: Implement Collections Trait**

1. **Define Collections trait** with unified API
2. **Implement for Vec** (enhanced with aggregation methods)
3. **Implement for HugeArray** (add missing methods)
4. **Implement for Arrow** (future integration)

### **Phase 2: Create Universal Adapter**

1. **Create UniversalPropertyValues** adapter
2. **Implement PropertyValues trait** for adapter
3. **Add factory methods** for different backends
4. **Maintain backward compatibility**

### **Phase 3: Migrate Existing Code**

1. **Update PropertyValues implementations** to use adapter
2. **Remove duplicate code** across different backends
3. **Add new Collections backends** (Arrow, etc.)
4. **Performance testing** and optimization

---

## Part 7: Benefits of Unified Architecture

### **Code Reduction**

```rust
// Before: Three separate implementations
impl PropertyValues for LongPropertyValues { /* 50 lines */ }
impl PropertyValues for HugeLongPropertyValues { /* 50 lines */ }
impl PropertyValues for ArrowLongPropertyValues { /* 50 lines */ }

// After: Single adapter
impl<T, C> PropertyValues for UniversalPropertyValues<T, C> { /* 50 lines */ }
```

**Result**: 150 lines → 50 lines (67% reduction)

### **Consistency**

- **Unified API**: All Collections implement the same interface
- **Predictable behavior**: Same methods work across all backends
- **Easy testing**: Test once, works everywhere

### **Extensibility**

- **New backends**: Just implement Collections trait
- **New operations**: Add to Collections trait, available everywhere
- **Performance optimization**: Optimize in Collections layer

---

## Part 8: Implementation Plan

### **Immediate Actions**

1. **Define Collections trait** in `src/collections/mod.rs`
2. **Implement for Vec** with enhanced methods
3. **Implement for HugeArray** (add missing methods)
4. **Create UniversalPropertyValues** adapter

### **Next Steps**

1. **Update factory.rs** to use Collections factory
2. **Migrate existing PropertyValues** to use adapter
3. **Add Arrow Collections** implementation
4. **Performance testing** and optimization

---

## Part 9: Conclusion

### **Key Benefits**

1. **Unified API**: Single interface for all Collections
2. **Code Reduction**: 67% less duplicate code
3. **Extensibility**: Easy to add new backends
4. **Performance**: Optimize once, benefit everywhere
5. **Maintainability**: Single point of truth

### **Architecture Summary**

```
┌─────────────────────────────────────┐
│        Application Layer            │
│    (PropertyValues, GraphStore)     │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│         Adapter Layer               │
│    (UniversalPropertyValues)        │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│        Collections Layer           │
│  (Vec, HugeArray, Arrow Collections) │
└─────────────────────────────────────┘
```

### **Next Steps**

1. **Implement Collections trait** with unified API
2. **Elevate Vec** to Collections standards
3. **Create universal adapter** for PropertyValues
4. **Migrate existing code** to use unified system

---

**Status**: ✅ **Architecture Design Complete**  
**Next**: Implement Collections trait and universal adapter
