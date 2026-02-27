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
