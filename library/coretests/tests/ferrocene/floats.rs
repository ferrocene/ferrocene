#[test]
#[expect(deprecated)]
fn test_f64_is_positive_negative() {
    assert!((1.0f64).is_positive());
    assert!((-1.0f64).is_negative());
}

// Cover `core::fmt::float::float_to_general_debug`
#[test]
fn float_to_general_debug_minus_plus() {
    assert_eq!(format!("{:+?}", -10.0), "-10.0");
    assert_eq!(format!("{:+?}", 10.0), "+10.0");
}
