//! Vector operations - translated from FloatVectorOperations.java

/// Add rhs to lhs in place.
pub fn add_in_place(lhs: &mut [f64], rhs: &[f64]) {
    let length = lhs.len().min(rhs.len());
    
    for i in 0..length {
        lhs[i] += rhs[i];
    }
}

/// Add weighted rhs to lhs in place: lhs[i] += weight * rhs[i]
pub fn add_weighted_in_place(lhs: &mut [f64], rhs: &[f64], weight: f64) {
    let length = lhs.len().min(rhs.len());
    
    for i in 0..length {
        lhs[i] += weight * rhs[i];
    }
}

/// Scale vector by scalar (in-place).
pub fn scale(lhs: &mut [f64], scalar: f64) {
    for value in lhs.iter_mut() {
        *value *= scalar;
    }
}

/// Scale vector by scalar and write to output.
pub fn scale_to(lhs: &[f64], scalar: f64, out: &mut [f64]) {
    assert_eq!(out.len(), lhs.len());
    
    for (i, &value) in lhs.iter().enumerate() {
        out[i] = value * scalar;
    }
}

/// Calculate L2 norm (Euclidean length) of vector.
pub fn l2_norm(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().map(|&v| v * v).sum();
    sum.sqrt()
}

/// Normalize vector to unit length (L2 normalization).
pub fn l2_normalize(array: &mut [f64]) {
    let euclidean_length = l2_norm(array);
    if euclidean_length > 0.0 {
        scale(array, 1.0 / euclidean_length);
    }
}

/// Check if any element in vector matches predicate.
pub fn any_match<F>(vector: &[f64], predicate: F) -> bool
where
    F: Fn(f64) -> bool,
{
    vector.iter().any(|&v| predicate(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_in_place() {
        let mut lhs = vec![1.0, 2.0, 3.0];
        let rhs = vec![4.0, 5.0, 6.0];
        add_in_place(&mut lhs, &rhs);
        assert_eq!(lhs, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_weighted_in_place() {
        let mut lhs = vec![1.0, 2.0, 3.0];
        let rhs = vec![4.0, 5.0, 6.0];
        add_weighted_in_place(&mut lhs, &rhs, 2.0);
        assert_eq!(lhs, vec![9.0, 12.0, 15.0]);
    }

    #[test]
    fn test_scale() {
        let mut data = vec![1.0, 2.0, 3.0];
        scale(&mut data, 2.0);
        assert_eq!(data, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_l2_norm() {
        let data = vec![3.0, 4.0];
        assert_eq!(l2_norm(&data), 5.0);
    }

    #[test]
    fn test_l2_normalize() {
        let mut data = vec![3.0, 4.0];
        l2_normalize(&mut data);
        assert!((data[0] - 0.6).abs() < 1e-10);
        assert!((data[1] - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_any_match() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(any_match(&data, |v| v > 2.0));
        assert!(!any_match(&data, |v| v > 10.0));
    }
}
