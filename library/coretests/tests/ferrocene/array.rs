#[test]
fn array_iter_mut() {
    let mut arr = [0u8; 10];

    for x in &mut arr {
        *x = 1;
    }

    let sum = arr.into_iter().sum::<u8>();

    assert_eq!(sum as usize, arr.len());
}

#[test]
fn test_drain_call_once() {
    core::array::ferrocene_test::test_drain_call_once();
}

// Tests:
// - `core::array::ascii::<impl [u8; N]>::as_ascii`
// - `core::array::ascii::<impl [u8; N]>::as_ascii_unchecked`
#[test]
fn test_as_ascii() {
    assert!(b"hello".as_ascii().is_some());
    assert!(b"ping\xfcino".as_ascii().is_none());
}
