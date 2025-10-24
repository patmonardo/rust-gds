//! VecShort: Vec-based i16 Collections implementation
pub struct VecShort { pub data: Vec<i16> }
impl VecShort { pub fn new() -> Self { Self { data: Vec::new() } } }
impl crate::collections::traits::Collections<i16> for VecShort {
    fn get(&self, index: usize) -> Option<i16> { self.data.get(index).map(|x| *x) }
    fn set(&mut self, index: usize, value: i16) { 
        if index < self.data.len() { self.data[index] = value; } 
        else { self.data.resize(index + 1, 0i16); self.data[index] = value; }
    }
    fn len(&self) -> usize { self.data.len() }
    fn sum(&self) -> Option<i16> where i16: std::iter::Sum { Some(self.data.iter().cloned().sum()) }
    fn mean(&self) -> Option<f64> where i16: Into<f64> { 
        if self.data.is_empty() { None } 
        else { Some(self.data.iter().map(|&x| x as f64).sum::<f64>() / self.data.len() as f64) }
    }
    fn min(&self) -> Option<i16> where i16: Ord { self.data.iter().min().cloned() }
    fn max(&self) -> Option<i16> where i16: Ord { self.data.iter().max().cloned() }
    fn std_dev(&self) -> Option<f64> where i16: Into<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            let variance = self.data.iter().map(|&x| { let diff = x as f64 - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64;
            Some(variance.sqrt())
        }
    }
    fn variance(&self) -> Option<f64> where i16: Into<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            Some(self.data.iter().map(|&x| { let diff = x as f64 - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64)
        }
    }
    fn median(&self) -> Option<i16> where i16: Ord {
        if self.data.is_empty() { None } else {
            let mut sorted = self.data.clone(); sorted.sort();
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 { Some((sorted[mid - 1] + sorted[mid]) / 2) } else { Some(sorted[mid]) }
        }
    }
    fn percentile(&self, p: f64) -> Option<i16> where i16: Ord {
        if self.data.is_empty() || p < 0.0 || p > 100.0 { None } else {
            let mut sorted = self.data.clone(); sorted.sort();
            let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
            Some(sorted[index])
        }
    }
    fn binary_search(&self, key: &i16) -> Result<usize, usize> where i16: Ord { self.data.binary_search(key) }
    fn sort(&mut self) where i16: Ord { self.data.sort(); }
    fn to_vec(self) -> Vec<i16> { self.data }
    fn as_slice(&self) -> &[i16] { &self.data }
    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> i16 { 0 }
    fn backend(&self) -> crate::collections::config::CollectionsBackend { 
        crate::collections::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::collections::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::collections::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Short }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: i16) -> Self {
        Self { data: vec![default_value; count] }
    }
}
