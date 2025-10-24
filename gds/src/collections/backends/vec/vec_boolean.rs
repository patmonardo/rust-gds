//! VecBoolean: Vec-based bool Collections implementation
pub struct VecBoolean { pub data: Vec<bool> }
impl VecBoolean { pub fn new() -> Self { Self { data: Vec::new() } } }
impl crate::collections::traits::Collections<bool> for VecBoolean {
    fn get(&self, index: usize) -> Option<bool> { self.data.get(index).map(|x| *x) }
    fn set(&mut self, index: usize, value: bool) { 
        if index < self.data.len() { self.data[index] = value; } 
        else { self.data.resize(index + 1, false); self.data[index] = value; }
    }
    fn len(&self) -> usize { self.data.len() }
    fn sum(&self) -> Option<bool> {
        // For bool, sum means "any true"
        Some(self.data.iter().any(|&x| x))
    }
    fn mean(&self) -> Option<f64> where bool: Into<f64> { 
        if self.data.is_empty() { None } 
        else { Some(self.data.iter().map(|&x| if x { 1.0 } else { 0.0 }).sum::<f64>() / self.data.len() as f64) }
    }
    fn min(&self) -> Option<bool> where bool: Ord { 
        // For bool, min means "all false"
        Some(self.data.iter().all(|&x| !x))
    }
    fn max(&self) -> Option<bool> where bool: Ord { 
        // For bool, max means "any true"
        Some(self.data.iter().any(|&x| x))
    }
    fn std_dev(&self) -> Option<f64> where bool: Into<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            let variance = self.data.iter().map(|&x| { let diff = if x { 1.0 } else { 0.0 } - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64;
            Some(variance.sqrt())
        }
    }
    fn variance(&self) -> Option<f64> where bool: Into<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            Some(self.data.iter().map(|&x| { let diff = if x { 1.0 } else { 0.0 } - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64)
        }
    }
    fn median(&self) -> Option<bool> where bool: Ord {
        if self.data.is_empty() { None } else {
            let true_count = self.data.iter().filter(|&&x| x).count();
            Some(true_count > self.data.len() / 2)
        }
    }
    fn percentile(&self, p: f64) -> Option<bool> where bool: Ord {
        if self.data.is_empty() || p < 0.0 || p > 100.0 { None } else {
            let true_count = self.data.iter().filter(|&&x| x).count();
            let threshold = (p / 100.0 * self.data.len() as f64).round() as usize;
            Some(true_count > threshold)
        }
    }
    fn binary_search(&self, key: &bool) -> Result<usize, usize> where bool: Ord { 
        self.data.binary_search(key) 
    }
    fn sort(&mut self) where bool: Ord { 
        // Sort bools: false first, then true
        self.data.sort();
    }
    fn to_vec(self) -> Vec<bool> { self.data }
    fn as_slice(&self) -> &[bool] { &self.data }
    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> bool { false }
    fn backend(&self) -> crate::collections::config::CollectionsBackend { 
        crate::collections::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::collections::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::collections::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Boolean }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: bool) -> Self {
        Self { data: vec![default_value; count] }
    }
}
