#[test]
fn type_name() {
    assert_eq!(core::any::type_name::<u8>(), core::any::type_name_of_val(&0u8));
}
