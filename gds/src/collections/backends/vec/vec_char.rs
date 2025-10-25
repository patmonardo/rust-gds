//! VecChar: Vec-based char Collections implementation
pub struct VecChar { pub data: Vec<char> }
impl VecChar { pub fn new() -> Self { Self { data: Vec::new() } } }
impl crate::collections::traits::Collections<char> for VecChar {
    fn get(&self, index: usize) -> Option<char> { self.data.get(index).map(|x| *x) }
    fn set(&mut self, index: usize, value: char) { 
        if index < self.data.len() { self.data[index] = value; } 
        else { self.data.resize(index + 1, '\0'); self.data[index] = value; }
    }
    fn len(&self) -> usize { self.data.len() }
    fn sum(&self) -> Option<char> {
        // For char, sum means "first char" (char doesn't have meaningful sum)
        self.data.first().cloned()
    }
    fn mean(&self) -> Option<f64> { 
        if self.data.is_empty() { None } 
        else { Some(self.data.iter().map(|&x| x as u32 as f64).sum::<f64>() / self.data.len() as f64) }
    }
    fn min(&self) -> Option<char> where char: Ord { self.data.iter().min().cloned() }
    fn max(&self) -> Option<char> where char: Ord { self.data.iter().max().cloned() }
    fn std_dev(&self) -> Option<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            let variance = self.data.iter().map(|&x| { let diff = x as u32 as f64 - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64;
            Some(variance.sqrt())
        }
    }
    fn variance(&self) -> Option<f64> {
        if self.data.len() < 2 { None } else {
            let mean = self.mean()?;
            Some(self.data.iter().map(|&x| { let diff = x as u32 as f64 - mean; diff * diff }).sum::<f64>() / (self.data.len() - 1) as f64)
        }
    }
    fn median(&self) -> Option<char> where char: Ord {
        if self.data.is_empty() { None } else {
            let mut sorted = self.data.clone(); sorted.sort();
            let mid = sorted.len() / 2;
            Some(sorted[mid])
        }
    }
    fn percentile(&self, p: f64) -> Option<char> where char: Ord {
        if self.data.is_empty() || p < 0.0 || p > 100.0 { None } else {
            let mut sorted = self.data.clone(); sorted.sort();
            let index = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
            Some(sorted[index])
        }
    }
    fn binary_search(&self, key: &char) -> Result<usize, usize> where char: Ord { 
        self.data.binary_search(key) 
    }
    fn sort(&mut self) where char: Ord { 
        self.data.sort();
    }
    fn to_vec(self) -> Vec<char> { self.data }
    fn as_slice(&self) -> &[char] { &self.data }
    fn is_null(&self, _index: usize) -> bool { false }
    fn null_count(&self) -> usize { 0 }
    fn default_value(&self) -> char { '\0' }
    fn backend(&self) -> crate::config::CollectionsBackend { 
        crate::config::CollectionsBackend::Vec 
    }
    fn features(&self) -> &[crate::config::Extension] { &[] }
    fn extensions(&self) -> &[crate::config::Extension] { &[] }
    fn value_type(&self) -> crate::types::ValueType { crate::types::ValueType::Char }
    fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
    fn with_defaults(count: usize, default_value: char) -> Self {
        Self { data: vec![default_value; count] }
    }
}
