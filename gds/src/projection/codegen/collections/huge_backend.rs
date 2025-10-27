//! Huge Backend Collections Macro
//!
//! Lightweight macro to implement Collections trait for HugeArray-backed types.
//! This is the "Level 0" Collections infrastructure macro - it implements
//! the Collections trait for existing HugeArray types.
//!
//! **Philosophy**: Similar to vec_collections!, but for paged arrays supporting
//! billions of elements. Minimal by design - just trait implementation.
//!
//! **Moved from**: `collections/macros/backends/huge.rs`
//! **Purpose**: Consolidation - all Collections codegen now lives under projection/codegen

/// Huge Collections macro that generates HugeArray-based Collections implementations
#[macro_export]
macro_rules! huge_collections {
    // Float-safe variant (uses partial_cmp; no Ord bounds)
    (
        $type_name:ident,
        $element_type:ty,
        $value_type:expr,
        $default_value:expr,
        to_f64 = $to_f64:expr,
        kind: Float,
        $features:expr,
        $extensions:expr,
        $doc_desc:expr
    ) => {
        impl $crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> {
                if index < self.size() { Some(self.get(index)) } else { None }
            }

            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.size() { self.set(index, value); }
            }

            fn len(&self) -> usize { self.size() }

            fn as_slice(&self) -> &[$element_type] { &[] }
            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> $crate::config::CollectionsBackend { $crate::config::CollectionsBackend::Huge }
            fn features(&self) -> &[$crate::config::Extension] { &[] }
            fn extensions(&self) -> &[$crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self::new(capacity) }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized {
                let mut h = Self::new(count);
                h.fill(default_value);
                h
            }

            // Aggregations
            fn sum(&self) -> Option<$element_type> where $element_type: std::iter::Sum { Some(self.iter().sum()) }
            fn mean(&self) -> Option<f64> {
                if self.size() == 0 { None } else { Some(self.iter().map(|x| ($to_f64)(x)).sum::<f64>() / self.size() as f64) }
            }
            fn std_dev(&self) -> Option<f64> {
                if self.size() < 2 { None } else { let m = self.mean()?; let v = self.iter().map(|x| { let d = ($to_f64)(x) - m; d*d }).sum::<f64>() / (self.size()-1) as f64; Some(v.sqrt()) }
            }
            fn variance(&self) -> Option<f64> {
                if self.size() < 2 { None } else { let m = self.mean()?; Some(self.iter().map(|x| { let d = ($to_f64)(x) - m; d*d }).sum::<f64>() / (self.size()-1) as f64) }
            }
            fn min(&self) -> Option<$element_type> {
                if self.size() == 0 { None } else { let mut it = self.iter(); let mut m = it.next().unwrap(); for v in it { if v < m { m = v; } } Some(m) }
            }
            fn max(&self) -> Option<$element_type> {
                if self.size() == 0 { None } else { let mut it = self.iter(); let mut m = it.next().unwrap(); for v in it { if v > m { m = v; } } Some(m) }
            }
            fn median(&self) -> Option<$element_type> {
                if self.size() == 0 { None } else { let mut v: Vec<$element_type> = self.iter().collect(); v.sort_by(|a,b| a.partial_cmp(b).unwrap()); let mid = v.len()/2; if v.len()%2==0 { Some((v[mid-1] + v[mid]) / 2.0) } else { Some(v[mid]) } }
            }
            fn percentile(&self, p: f64) -> Option<$element_type> {
                if self.size()==0 || p<0.0 || p>100.0 { None } else { let mut v: Vec<$element_type> = self.iter().collect(); v.sort_by(|a,b| a.partial_cmp(b).unwrap()); let idx = ((p/100.0)*(v.len()-1) as f64).round() as usize; Some(v[idx]) }
            }

            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> {
                let values: Vec<$element_type> = self.iter().collect();
                values.binary_search_by(|a| a.partial_cmp(key).unwrap())
            }

            fn sort(&mut self) {
                // Not supported in-place for Huge arrays
            }

            fn to_vec(self) -> Vec<$element_type> { self.iter().collect() }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self::new(0) }
            fn with_capacity(capacity: usize) -> Self { Self::new(capacity) }
            fn from_vec(values: Vec<$element_type>) -> Self {
                let mut huge = Self::new(values.len());
                for (i, value) in values.into_iter().enumerate() { huge.set(i, value); }
                huge
            }
            fn from_slice(slice: &[$element_type]) -> Self {
                let mut huge = Self::new(slice.len());
                for (i, &value) in slice.iter().enumerate() { huge.set(i, value); }
                huge
            }
            fn with_defaults(count: usize, default_value: $element_type) -> Self {
                let mut huge = Self::new(count);
                for i in 0..count { huge.set(i, default_value); }
                huge
            }
        }
    };

    // Ord-based variant (integers, etc.)
    (
        $type_name:ident,
        $element_type:ty,
        $value_type:expr,
        $default_value:expr,
        to_f64 = $to_f64:expr,
        kind: Ord,
        $features:expr,
        $extensions:expr,
        $doc_desc:expr
    ) => {
        impl crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> {
                if index < self.size() { Some(self.get(index)) } else { None }
            }
            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.size() { self.set(index, value); }
            }
            fn len(&self) -> usize { self.size() }
            fn as_slice(&self) -> &[$element_type] { &[] }
            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> crate::config::CollectionsBackend { crate::config::CollectionsBackend::Huge }
            fn features(&self) -> &[crate::config::Extension] { &[] }
            fn extensions(&self) -> &[crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self::new(capacity) }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized { let mut h = Self::new(count); h.fill(default_value); h }

            fn sum(&self) -> Option<$element_type> where $element_type: std::iter::Sum { Some(self.iter().sum()) }
            fn mean(&self) -> Option<f64> { if self.size() == 0 { None } else { Some(self.iter().map(|x| ($to_f64)(x)).sum::<f64>() / self.size() as f64) } }
            fn std_dev(&self) -> Option<f64> { if self.size() < 2 { None } else { let m = self.mean()?; let v = self.iter().map(|x| { let d = ($to_f64)(x) - m; d*d }).sum::<f64>() / (self.size() - 1) as f64; Some(v.sqrt()) } }
            fn variance(&self) -> Option<f64> { if self.size() < 2 { None } else { let m = self.mean()?; Some(self.iter().map(|x| { let d = ($to_f64)(x) - m; d*d }).sum::<f64>() / (self.size() - 1) as f64) } }
            fn min(&self) -> Option<$element_type> where $element_type: Ord { if self.size() == 0 { None } else { self.iter().min() } }
            fn max(&self) -> Option<$element_type> where $element_type: Ord { if self.size() == 0 { None } else { self.iter().max() } }
            fn median(&self) -> Option<$element_type> where $element_type: Ord { if self.size() == 0 { None } else { let mut v: Vec<$element_type> = self.iter().collect(); v.sort(); let mid = v.len()/2; if v.len()%2==0 { Some((v[mid-1] + v[mid]) / 2) } else { Some(v[mid]) } } }
            fn percentile(&self, p: f64) -> Option<$element_type> where $element_type: Ord { if self.size()==0 || p<0.0 || p>100.0 { None } else { let mut v: Vec<$element_type> = self.iter().collect(); v.sort(); let idx = ((p/100.0)*(v.len()-1) as f64).round() as usize; Some(v[idx]) } }

            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> where $element_type: Ord {
                let v: Vec<$element_type> = self.iter().collect();
                v.binary_search(key)
            }
            fn sort(&mut self) where $element_type: Ord { /* no-op for Huge arrays */ }
            fn to_vec(self) -> Vec<$element_type> { self.iter().collect() }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self::new(0) }
            fn with_capacity(capacity: usize) -> Self { Self::new(capacity) }
            fn from_vec(values: Vec<$element_type>) -> Self { let mut h = Self::new(values.len()); for (i,v) in values.into_iter().enumerate(){ h.set(i,v);} h }
            fn from_slice(slice: &[$element_type]) -> Self { let mut h = Self::new(slice.len()); for (i,&v) in slice.iter().enumerate(){ h.set(i,v);} h }
            fn with_defaults(count: usize, default_value: $element_type) -> Self { let mut h = Self::new(count); for i in 0..count { h.set(i, default_value); } h }
        }
    };

    // Ord but no numeric aggregations (bool, char)
    (
        $type_name:ident,
        $element_type:ty,
        $value_type:expr,
        $default_value:expr,
        kind: OrdNoAgg,
        $features:expr,
        $extensions:expr,
        $doc_desc:expr
    ) => {
        impl crate::collections::traits::Collections<$element_type> for $type_name {
            fn get(&self, index: usize) -> Option<$element_type> {
                if index < self.size() { Some(self.get(index)) } else { None }
            }
            fn set(&mut self, index: usize, value: $element_type) {
                if index < self.size() { self.set(index, value); }
            }
            fn len(&self) -> usize { self.size() }
            fn as_slice(&self) -> &[$element_type] { &[] }
            fn is_null(&self, _index: usize) -> bool { false }
            fn null_count(&self) -> usize { 0 }
            fn default_value(&self) -> $element_type { $default_value }
            fn backend(&self) -> crate::config::CollectionsBackend { crate::config::CollectionsBackend::Huge }
            fn features(&self) -> &[crate::config::Extension] { &[] }
            fn extensions(&self) -> &[crate::config::Extension] { &[] }
            fn value_type(&self) -> crate::types::ValueType { $value_type }
            fn with_capacity(capacity: usize) -> Self where Self: Sized { Self::new(capacity) }
            fn with_defaults(count: usize, default_value: $element_type) -> Self where Self: Sized { let mut h = Self::new(count); h.fill(default_value); h }

            fn sum(&self) -> Option<$element_type> { None }
            fn mean(&self) -> Option<f64> { None }
            fn std_dev(&self) -> Option<f64> { None }
            fn variance(&self) -> Option<f64> { None }
            fn min(&self) -> Option<$element_type> where $element_type: Ord { if self.size() == 0 { None } else { self.iter().min() } }
            fn max(&self) -> Option<$element_type> where $element_type: Ord { if self.size() == 0 { None } else { self.iter().max() } }
            fn median(&self) -> Option<$element_type> where $element_type: Ord { None }
            fn percentile(&self, _p: f64) -> Option<$element_type> where $element_type: Ord { None }

            fn binary_search(&self, key: &$element_type) -> Result<usize, usize> where $element_type: Ord {
                let v: Vec<$element_type> = self.iter().collect();
                v.binary_search(key)
            }
            fn sort(&mut self) where $element_type: Ord { /* no-op for Huge arrays */ }
            fn to_vec(self) -> Vec<$element_type> { self.iter().collect() }
        }

        impl crate::collections::traits::CollectionsFactory<$element_type> for $type_name {
            fn new() -> Self { Self::new(0) }
            fn with_capacity(capacity: usize) -> Self { Self::new(capacity) }
            fn from_vec(values: Vec<$element_type>) -> Self { let mut h = Self::new(values.len()); for (i,v) in values.into_iter().enumerate(){ h.set(i,v);} h }
            fn from_slice(slice: &[$element_type]) -> Self { let mut h = Self::new(slice.len()); for (i,&v) in slice.iter().enumerate(){ h.set(i,v);} h }
            fn with_defaults(count: usize, default_value: $element_type) -> Self { let mut h = Self::new(count); for i in 0..count { h.set(i, default_value); } h }
        }
    };
}

