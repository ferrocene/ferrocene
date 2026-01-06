// covers `core::cmp::Ord::clamp`.
#[test]
fn default_clamp_impl() {
    fn wrapped_clamp<T: Ord>(this: T, min: T, max: T) -> T {
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct Wrapper<T>(T);

        Wrapper(this).clamp(Wrapper(min), Wrapper(max)).0
    }

    assert_eq!(wrapped_clamp(0, 1, 10), 1);
    assert_eq!(wrapped_clamp(11, 1, 10), 10);
    assert_eq!(wrapped_clamp(5, 1, 10), 5);
}
