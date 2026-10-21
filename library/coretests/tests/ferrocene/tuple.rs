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

// Covers:
// * core::tuple::<impl core::cmp::PartialEq for (T,)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (Z, Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (A, Z, Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (B, A, Z, Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (C, B, A, Z, Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (D, C, B, A, Z, Y, X, W, V, U, T)>::eq
// * core::tuple::<impl core::cmp::PartialEq for (E, D, C, B, A, Z, Y, X, W, V, U, T)>::eq
#[test]
fn tuple_partial_eq() {
    assert!((1,) == (1,));
    assert!((1, 2) == (1, 2));
    assert!((1, 2, 3) == (1, 2, 3));
    assert!((1, 2, 3, 4) == (1, 2, 3, 4));
    assert!((1, 2, 3, 4, 5) == (1, 2, 3, 4, 5));
    assert!((1, 2, 3, 4, 5, 6) == (1, 2, 3, 4, 5, 6));
    assert!((1, 2, 3, 4, 5, 6, 7) == (1, 2, 3, 4, 5, 6, 7));
    assert!((1, 2, 3, 4, 5, 6, 7, 8) == (1, 2, 3, 4, 5, 6, 7, 8));
    assert!((1, 2, 3, 4, 5, 6, 7, 8, 9) == (1, 2, 3, 4, 5, 6, 7, 8, 9));
    assert!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10) == (1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    assert!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) == (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
    assert!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12) == (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
}
