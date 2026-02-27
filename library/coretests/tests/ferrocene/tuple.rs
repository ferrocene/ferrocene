#[test]
fn default_chaining_impl() {
    assert!((1, 2) <= (1, 2));
    assert!((3, 4) >= (1, 2));
    assert!((1, 2) < (3, 4));
    assert!((3, 4) > (1, 2));

    assert!((1, 2) <= (1, 2));
    assert!((1, 2) >= (1, 2));
    assert_eq!((1, 2) <= (2, 2), true);
    assert_eq!((2, 2) >= (1, 2), true);
}

#[test]
fn tuple_comparison() {
    let data = [
        ("core::iter::adapters::Chain", 123_usize),
        ("core::iter::adapters::Clone", 456_usize),
        ("core::iter::adapters::Copie", 789_usize),
        ("core::iter::adapters::Cycle", 123_usize),
        ("core::iter::adapters::Flatt", 456_usize),
        ("core::iter::adapters::TakeN", 789_usize),
    ];

    for val in data.windows(2) {
        let x = val[0];
        let y = val[1];
        assert_eq!([x < y, x <= y, x > y, x >= y], [true, true, false, false]);
    }

    assert!(("1", "2", "3") < ("1", "2", "4"));
    assert!(("1", "2", "3") < ("1", "2", "4"));
    #[derive(PartialOrd, PartialEq)]
    struct Float(f32);
    assert!(!((Float(f32::NAN), Float(f32::NAN), "3") < (Float(1.0), Float(f32::NAN), "4")));
}
