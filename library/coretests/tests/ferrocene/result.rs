#[test]
fn result_methods() {
    let s = String::from("hello");
    let mut ok = Ok::<&&String, &&String>(&&s);
    let mut err = Err::<&&String, &&String>(&&s);

    let mut len = None;
    err = err.inspect(|s| {
        len = Some(s.len());
    });
    assert!(len.is_none());
    assert!(err.is_err_and(|&x| x == &s));
    assert!(!err.is_ok_and(|&x| x == &s));

    ok = ok.inspect(|s| {
        len = Some(s.len());
    });
    assert_eq!(len, Some(5));
    assert!(ok.is_ok_and(|&x| x == &s));
    assert!(!ok.is_err_and(|&x| x == &s));

    let mut len = None;
    ok = ok.inspect_err(|s| {
        len = Some(s.len());
    });
    assert!(len.is_none());
    assert!(ok.is_ok_and(|&x| x == &s));
    assert!(!ok.is_err_and(|&x| x == &s));

    err = err.inspect_err(|s| {
        len = Some(s.len());
    });
    assert_eq!(len, Some(5));
    assert!(err.is_err_and(|&x| x == &s));
    assert!(!err.is_ok_and(|&x| x == &s));

    assert_eq!(ok.copied().cloned().map_or_default(|s| s.len()), 5);
    assert_eq!(err.copied().cloned().map_or_default(|s| s.len()), 0);

    assert_eq!(ok.copied().cloned().map_or_else(|_| 0, |s| s.len()), 5);
    assert_eq!(err.copied().cloned().map_or_else(|_| 0, |s| s.len()), 0);

    let mut x = 5i32;

    assert!(Ok::<&mut i32, i32>(&mut x).cloned().is_ok_and(|x| x == 5));
    assert!(Ok::<&mut i32, i32>(&mut x).copied().is_ok_and(|x| x == 5));

    assert!(Err::<&mut i32, i32>(x).cloned().is_err_and(|x| x == 5));
    assert!(Err::<&mut i32, i32>(x).copied().is_err_and(|x| x == 5));

    assert!(ok.err().is_none());
    assert!(err.err().is_some());
}

#[test]
#[should_panic = "reached"]
fn inspect_result() {
    let _ = Ok::<&str, &str>("foo").inspect(|_| panic!("reached"));
}

#[test]
#[should_panic = "reached"]
fn inspect_result_err() {
    let _ = Err::<&str, &str>("foo").inspect_err(|_| panic!("reached"));
}
