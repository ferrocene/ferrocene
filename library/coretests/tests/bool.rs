use core::cmp::Ordering::{Equal, Greater, Less};
use core::ops::{BitAnd, BitOr, BitXor};

#[test]
fn test_bool() {
    assert_eq!(false.eq(&true), false);
    assert_eq!(false == false, true);
    assert_eq!(false != true, true);
    assert_eq!(false.ne(&false), false);

    assert_eq!(false.bitand(false), false);
    assert_eq!(true.bitand(false), false);
    assert_eq!(false.bitand(true), false);
    assert_eq!(true.bitand(true), true);

    assert_eq!(false & false, false);
    assert_eq!(true & false, false);
    assert_eq!(false & true, false);
    assert_eq!(true & true, true);

    assert_eq!(false.bitor(false), false);
    assert_eq!(true.bitor(false), true);
    assert_eq!(false.bitor(true), true);
    assert_eq!(true.bitor(true), true);

    assert_eq!(false | false, false);
    assert_eq!(true | false, true);
    assert_eq!(false | true, true);
    assert_eq!(true | true, true);

    assert_eq!(false.bitxor(false), false);
    assert_eq!(true.bitxor(false), true);
    assert_eq!(false.bitxor(true), true);
    assert_eq!(true.bitxor(true), false);

    assert_eq!(false ^ false, false);
    assert_eq!(true ^ false, true);
    assert_eq!(false ^ true, true);
    assert_eq!(true ^ true, false);

    assert_eq!(!true, false);
    assert_eq!(!false, true);

    let s = false.to_string();
    assert_eq!(s, "false");
    let s = true.to_string();
    assert_eq!(s, "true");

    assert!(true > false);
    assert!(!(false > true));

    assert!(false < true);
    assert!(!(true < false));

    assert!(false <= false);
    assert!(false >= false);
    assert!(true <= true);
    assert!(true >= true);

    assert!(false <= true);
    assert!(!(false >= true));
    assert!(true >= false);
    assert!(!(true <= false));

    assert_eq!(true.cmp(&true), Equal);
    assert_eq!(false.cmp(&false), Equal);
    assert_eq!(true.cmp(&false), Greater);
    assert_eq!(false.cmp(&true), Less);

    assert_eq!(true.partial_cmp(&true), Some(Equal));
    assert_eq!(false.partial_cmp(&false), Some(Equal));
    assert_eq!(true.partial_cmp(&false), Some(Greater));
    assert_eq!(false.partial_cmp(&true), Some(Less));

    assert!(true.min(true));
    assert!(!(false.min(false)));
    assert!(!(true.min(false)));
    assert!(!(false.min(true)));

    assert!(true.max(true));
    assert!(!(false.max(false)));
    assert!(true.max(false));
    assert!(false.max(true));

    assert!(true.clamp(true, true));
    assert!(!(false.clamp(false, false)));
    assert!(true.clamp(false, true));
    assert!(false.clamp(true, true));
    assert!(!(true.clamp(false, false)));
    assert!(!(false.clamp(false, true)));
}

#[test]
#[should_panic]
fn test_invalid_clamp() {
    assert!(true.clamp(true, false));
}

#[test]
pub fn test_bool_not() {
    if !false {
        assert!(true);
    } else {
        assert!(false);
    }
    if !true {
        assert!(false);
    } else {
        assert!(true);
    }
}

const fn zero() -> i32 {
    0
}

#[test]
fn test_bool_to_option() {
    assert_eq!(false.then_some(0), None);
    assert_eq!(true.then_some(0), Some(0));
    assert_eq!(false.then(|| 0), None);
    assert_eq!(true.then(|| 0), Some(0));

    const A: Option<i32> = false.then_some(0);
    const B: Option<i32> = true.then_some(0);
    const C: Option<i32> = false.then(zero);
    const D: Option<i32> = true.then(zero);

    assert_eq!(A, None);
    assert_eq!(B, Some(0));
    assert_eq!(C, None);
    assert_eq!(D, Some(0));
}

#[test]
fn test_bool_to_result() {
    assert_eq!(false.ok_or(0), Err(0));
    assert_eq!(true.ok_or(0), Ok(()));
    assert_eq!(false.ok_or_else(|| 0), Err(0));
    assert_eq!(true.ok_or_else(|| 0), Ok(()));

    const A: Result<(), i32> = false.ok_or(0);
    const B: Result<(), i32> = true.ok_or(0);
    const C: Result<(), i32> = false.ok_or_else(zero);
    const D: Result<(), i32> = true.ok_or_else(zero);

    assert_eq!(A, Err(0));
    assert_eq!(B, Ok(()));
    assert_eq!(C, Err(0));
    assert_eq!(D, Ok(()));
}
