//! Vec Backend Collections Macro
//!
//! Lightweight macro to implement Collections trait for Vec-backed types.
//! This is the "Level 0" Collections infrastructure macro - it implements
//! the Collections trait for structs that have a `data: Vec<T>` field.
//!
//! **Philosophy**: This macro is the foundation that enables Vec backends
//! to participate in the Collections ecosystem. It's minimal by design -
//! just trait implementation, no generation of new types.
//!
//! **Moved from**: `collections/macros/backends/vec.rs`
//! **Purpose**: Consolidation - all Collections codegen now lives under projection/codegen

/// Implement Collections for an existing Vec-backed type with a `data: Vec<T>` field
#[macro_export]
macro_rules! vec_collections {
    // Float-aware arm (partial_cmp for ordering)
    (
        $type_name:ident,
        $element_type:ty,
        $value_type:expr,
        $default_value:expr,
        kind = Float
    ) => {
        impl crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> { self.data.get(index).cloned() }
            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.data.len() { self.data[index] = value; }
                else { self.data.resize(index + 1, $default_value); self.data[index] = value; }
            }
            fn len(&self) -> usize { self.data.len() }
            fn sum(&self) -> Option<$element_type> where $element_type: std::iter::Sum { Some(self.data.iter().cloned().sum()) }
            fn mean(&self) -> Option<f64> { if self.data.is_empty() { None } else { Some(self.data.iter().cloned().map(|x| x as f64).sum::<f64>() / self.data.len() as f64) } }
            fn std_dev(&self) -> Option<f64> { if self.data.len() < 2 { None } else { let m = self.mean()?; let v = self.data.iter().cloned().map(|x| { let d = x as f64 - m; d*d }).sum::<f64>() / (self.data.len() - 1) as f64; Some(v.sqrt()) } }
            fn variance(&self) -> Option<f64> { if self.data.len() < 2 { None } else { let m = self.mean()?; Some(self.data.iter().cloned().map(|x| { let d = x as f64 - m; d*d }).sum::<f64>() / (self.data.len() - 1) as f64) } }
            fn min(&self) -> Option<$element_type> { self.data.iter().cloned().min_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) }
            fn max(&self) -> Option<$element_type> { self.data.iter().cloned().max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) }
            fn median(&self) -> Option<$element_type> {
                if self.data.is_empty() { None } else {
                    let mut v = self.data.clone();
                    v.sort_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let mid = v.len()/2;
                    if v.len()%2==0 { Some((v[mid-1] + v[mid]) / 2.0) } else { Some(v[mid]) }
                }
            }
            fn percentile(&self, p: f64) -> Option<$element_type> {
                if self.data.is_empty() || !(0.0..=100.0).contains(&p) { None } else {
                    let mut v = self.data.clone();
                    v.sort_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let idx = ((p/100.0)*(v.len()-1) as f64).round() as usize;
                    Some(v[idx])
                }
            }
            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> { self.data.binary_search_by(|x| x.partial_cmp(key).unwrap()) }
            fn sort(&mut self) { self.data.sort_by(|a,b| a.partial_cmp(b).unwrap()); }
            fn to_vec(self) -> Vec<$element_type> { self.data }
            fn as_slice(&self) -> &[$element_type] { &self.data }
            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> crate::config::CollectionsBackend { crate::config::CollectionsBackend::Vec }
            fn features(&self) -> &[crate::config::Extension] { &[] }
            fn extensions(&self) -> &[crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self { data: Vec::with_capacity(capacity) } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized { Self { data: vec![default_value; count] } }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self { data: Vec::new() } }
            fn with_capacity(capacity: usize) -> Self { Self { data: Vec::with_capacity(capacity) } }
            fn from_vec(values: Vec<$element_type>) -> Self { Self { data: values } }
            fn from_slice(slice: &[$element_type]) -> Self { Self { data: slice.to_vec() } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self { Self { data: vec![default_value; count] } }
        }
    };
    // Ord-only, no numeric aggregations (e.g., bool, char)
    (
        $type_name:ident,
        $element_type:ty,
        $value_type:expr,
        $default_value:expr,
        kind = OrdNoAgg
    ) => {
        impl crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> { self.data.get(index).cloned() }
            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.data.len() { self.data[index] = value; }
                else { self.data.resize(index + 1, $default_value); self.data[index] = value; }
            }
            fn len(&self) -> usize { self.data.len() }
            fn sum(&self) -> Option<$element_type> { None }
            fn mean(&self) -> Option<f64> { None }
            fn std_dev(&self) -> Option<f64> { None }
            fn variance(&self) -> Option<f64> { None }
            fn min(&self) -> Option<$element_type> where $element_type: Ord { self.data.iter().cloned().min() }
            fn max(&self) -> Option<$element_type> where $element_type: Ord { self.data.iter().cloned().max() }
            fn median(&self) -> Option<$element_type> where $element_type: Ord { None }
            fn percentile(&self, _p: f64) -> Option<$element_type> where $element_type: Ord { None }
            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> where $element_type: Ord { self.data.binary_search(key) }
            fn sort(&mut self) where $element_type: Ord { self.data.sort(); }
            fn to_vec(self) -> Vec<$element_type> { self.data }
            fn as_slice(&self) -> &[$element_type] { &self.data }
            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> crate::config::CollectionsBackend { crate::config::CollectionsBackend::Vec }
            fn features(&self) -> &[crate::config::Extension] { &[] }
            fn extensions(&self) -> &[crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self { data: Vec::with_capacity(capacity) } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized { Self { data: vec![default_value; count] } }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self { data: Vec::new() } }
            fn with_capacity(capacity: usize) -> Self { Self { data: Vec::with_capacity(capacity) } }
            fn from_vec(values: Vec<$element_type>) -> Self { Self { data: values } }
            fn from_slice(slice: &[$element_type]) -> Self { Self { data: slice.to_vec() } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self { Self { data: vec![default_value; count] } }
        }
    };

    (
        $type_name:ident,            // e.g., VecInt
        $element_type:ty,            // e.g., i32
        $value_type:expr,            // e.g., ValueType::Int
        $default_value:expr,         // e.g., 0i32
        to_f64 = $to_f64:expr,       // e.g., |x: i32| x as f64
        kind = Ord                   // comparison kind (currently Ord only)
    ) => {
        impl crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> {
                self.data.get(index).cloned()
            }

            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.data.len() {
                    self.data[index] = value;
                } else {
                    self.data.resize(index + 1, $default_value);
                    self.data[index] = value;
                }
            }

            fn len(&self) -> usize { self.data.len() }

            fn sum(&self) -> Option<$element_type> where $element_type: std::iter::Sum {
                Some(self.data.iter().cloned().sum())
            }

            fn mean(&self) -> Option<f64> {
                if self.data.is_empty() { None } else {
                    let sum: f64 = self.data.iter().map(|&x| ($to_f64)(x)).sum();
                    Some(sum / self.data.len() as f64)
                }
            }

            fn min(&self) -> Option<$element_type> where $element_type: Ord {
                self.data.iter().cloned().min()
            }

            fn max(&self) -> Option<$element_type> where $element_type: Ord {
                self.data.iter().cloned().max()
            }

            fn std_dev(&self) -> Option<f64> {
                if self.data.len() < 2 { None } else {
                    let mean = self.mean()?;
                    let var: f64 = self.data.iter().map(|&x| { let d = ($to_f64)(x) - mean; d*d }).sum::<f64>() / (self.data.len() - 1) as f64;
                    Some(var.sqrt())
                }
            }

            fn variance(&self) -> Option<f64> {
                if self.data.len() < 2 { None } else {
                    let mean = self.mean()?;
                    Some(self.data.iter().map(|&x| { let d = ($to_f64)(x) - mean; d*d }).sum::<f64>() / (self.data.len() - 1) as f64)
                }
            }

            fn median(&self) -> Option<$element_type> where $element_type: Ord {
                if self.data.is_empty() { None } else {
                    let mut v = self.data.clone();
                    v.sort();
                    let mid = v.len() / 2;
                    if v.len() % 2 == 0 { Some((v[mid - 1] + v[mid]) / 2) } else { Some(v[mid]) }
                }
            }

            fn percentile(&self, p: f64) -> Option<$element_type> where $element_type: Ord {
                if self.data.is_empty() || !(0.0..=100.0).contains(&p) { None } else {
                    let mut v = self.data.clone();
                    v.sort();
                    let idx = ((p / 100.0) * (v.len() as f64 - 1.0)).round() as usize;
                    Some(v[idx])
                }
            }

            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> where $element_type: Ord {
                self.data.binary_search(key)
            }

            fn sort(&mut self) where $element_type: Ord { self.data.sort(); }

            fn to_vec(self) -> Vec<$element_type> { self.data }

            fn as_slice(&self) -> &[$element_type] { &self.data }

            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> crate::config::CollectionsBackend { crate::config::CollectionsBackend::Vec }
            fn features(&self) -> &[crate::config::Extension] { &[] }
            fn extensions(&self) -> &[crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self { data: Vec::with_capacity(capacity) } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized { Self { data: vec![default_value; count] } }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self { data: Vec::new() } }
            fn with_capacity(capacity: usize) -> Self { Self { data: Vec::with_capacity(capacity) } }
            fn from_vec(values: Vec<$element_type>) -> Self { Self { data: values } }
            fn from_slice(slice: &[$element_type]) -> Self { Self { data: slice.to_vec() } }
            fn with_defaults(count: usize, default_value: $element_type) -> Self { Self { data: vec![default_value; count] } }
        }
    };
}

