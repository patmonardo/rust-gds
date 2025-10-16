use rust_gds::projection::eval::form_processor::{
    checked_u64_to_usize, widen_f32_to_f64, widen_i32_to_i64, FormProcessorError,
};

#[test]
fn form_processor_checks() {
    // small id
    assert_eq!(checked_u64_to_usize(0u64).unwrap(), 0usize);
    assert_eq!(checked_u64_to_usize(12345u64).unwrap(), 12345usize);

    // widening
    assert_eq!(widen_i32_to_i64(-10), -10i64);
    assert_eq!(widen_f32_to_f64(1.25f32), 1.25f64);

    // overflow behavior is environment-dependent; we at least ensure the error
    // variant reports the value when conversion fails (can't easily force on 64-bit)
    let large = u64::MAX;
    let res = checked_u64_to_usize(large);
    if res.is_err() {
        assert_eq!(res.unwrap_err(), FormProcessorError::IndexOverflow(large));
    }
}
