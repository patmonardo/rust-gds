/// Small helpers for cross-type array equality checks (mirrors TS ArrayEquals lightly).
pub fn byte_and_long(bytes: &[u8], longs: &[i64]) -> bool {
    if bytes.len() != longs.len() {
        return false;
    }
    for i in 0..bytes.len() {
        if bytes[i] as i64 != longs[i] {
            return false;
        }
    }
    true
}

pub fn short_and_long(shorts: &[i16], longs: &[i64]) -> bool {
    if shorts.len() != longs.len() {
        return false;
    }
    for i in 0..shorts.len() {
        if shorts[i] as i64 != longs[i] {
            return false;
        }
    }
    true
}

pub fn int_and_long(ints: &[i32], longs: &[i64]) -> bool {
    if ints.len() != longs.len() {
        return false;
    }
    for i in 0..ints.len() {
        if ints[i] as i64 != longs[i] {
            return false;
        }
    }
    true
}

pub fn long_and_long(a: &[i64], b: &[i64]) -> bool {
    a == b
}

pub fn float_and_double(floats: &[f32], doubles: &[f64]) -> bool {
    if floats.len() != doubles.len() {
        return false;
    }
    for i in 0..floats.len() {
        if (floats[i] as f64) != doubles[i] {
            return false;
        }
    }
    true
}
