use core::cmp::Ordering;
use core::ops::ControlFlow;

#[test]
fn ordering_equality() {
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
fn control_flow_is_methods() {
    let c = ControlFlow::<i32, i32>::Continue(0);
    assert!(c.is_continue());
    assert!(!c.is_break());

    let b = ControlFlow::<i32, i32>::Break(1);
    assert!(!b.is_continue());
    assert!(b.is_break());
}

#[test]
fn control_flow_into_value() {
    let c = ControlFlow::<i32, i32>::Continue(0);
    assert_eq!(c.into_value(), 0);

    let b = ControlFlow::<i32, i32>::Break(1);
    assert_eq!(b.into_value(), 1);
}
