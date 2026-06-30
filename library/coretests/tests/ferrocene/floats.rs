// Cover `core::fmt::float::float_to_general_debug`
#[test]
fn float_to_general_debug_minus_plus() {
    assert_eq!(format!("{:+?}", -10.0), "-10.0");
    assert_eq!(format!("{:+?}", 10.0), "+10.0");
}
