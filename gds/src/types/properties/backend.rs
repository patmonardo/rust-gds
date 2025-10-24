//! Backend abstraction for PropertyValues storage
//!
//! This trait abstracts storage backends used by PropertyValues implementations.
//! Default implementation is provided for `Vec<T>`; future adapters (HugeArray,
//! Arrow) can implement this trait behind feature gates.

use std::fmt::Debug;

/// Generic backend trait for columnar property storage.
pub trait PropertyBackend<T>: Send + Sync + Debug {
    /// Returns a cloned value at the given index, or None if out of bounds.
    fn get(&self, index: usize) -> Option<T>;

    /// Number of logical elements.
    fn len(&self) -> usize;

    /// True if there are no elements.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterator over cloned values.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = T> + 'a>;
}

impl<T: Clone + Debug + Send + Sync + 'static> PropertyBackend<T> for Vec<T> {
    fn get(&self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self[index].clone())
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = T> + 'a> {
        Box::new(self.as_slice().iter().cloned())
    }
}


