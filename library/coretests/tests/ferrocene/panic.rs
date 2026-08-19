#[test]
fn panic_location() {
    let loc = core::panic::Location::caller();

    let _ = loc.line();
    let _ = loc.column();
}

#[test]
fn panic_info() {
    let txt = "hello, world!";
    let args = format_args!("{txt}");
    let info = core::ferrocene_test::create_panic_info(&args);

    let s = info.to_string();
    eprintln!("{s}");
    let (fst, snd) = s.split_once(":\n").unwrap();
    assert!(fst.starts_with("panicked at "));
    assert_eq!(snd, txt);

    let msg = info.message();
    assert_eq!(format!("{msg:?}"), txt);
}
