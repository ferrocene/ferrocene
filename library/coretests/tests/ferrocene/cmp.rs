#[test]
fn ordering_equality() {
    use core::cmp::Ordering;

    let os = [Ordering::Less, Ordering::Equal, Ordering::Greater];
    for ordering in os {
        let is_eq = ordering.is_eq();
        let is_ne = ordering.is_ne();
        if let Ordering::Equal = ordering {
            assert!(is_eq);
            assert!(!is_ne);
        } else {
            assert!(!is_eq);
            assert!(is_ne);
        }
    }
}

#[test]
fn unit_comparisons() {
    assert!(!(() != ()));
    assert!(!(&() != &mut ()));
    assert!(!(&mut () != &()));
    assert!(!(() < ()));
}

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
