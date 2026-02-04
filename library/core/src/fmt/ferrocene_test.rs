pub fn test_rt_argument_as_u16_none() {
    assert_eq!(super::rt::Argument::new_debug(&10).as_u16(), None);
}
