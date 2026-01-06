#[test]
#[expect(deprecated)]
fn test_f64_is_positive_negative() {
    assert!((1.0f64).is_positive());
    assert!((-1.0f64).is_negative());
}
